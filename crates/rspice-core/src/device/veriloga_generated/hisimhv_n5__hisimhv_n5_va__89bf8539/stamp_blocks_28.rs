#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign21610_e16646, assign21610_e16646_d_n0, assign21610_e16646_d_n2, assign21610_e16646_d_n4, assign21610_e16646_d_n5, assign21610_e16646_d_n6, assign21610_e16646_d_n7, assign21610_e16646_d_n8, assign21610_e16646_d_n9, assign21610_e16646_d_n10, assign21610_e16646_d_n11, assign21610_e16646_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 == 0.0)) {
        let assign21610_e16640: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign21610_e16643: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign21610_e16644: f64 = (assign21610_e16640 + assign21610_e16643);
        (assign21610_e16644, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21610_e16646;
        locals.var_rdd_dn0 = assign21610_e16646_d_n0;
        locals.var_rdd_dn2 = assign21610_e16646_d_n2;
        locals.var_rdd_dn4 = assign21610_e16646_d_n4;
        locals.var_rdd_dn5 = assign21610_e16646_d_n5;
        locals.var_rdd_dn6 = assign21610_e16646_d_n6;
        locals.var_rdd_dn7 = assign21610_e16646_d_n7;
        locals.var_rdd_dn8 = assign21610_e16646_d_n8;
        locals.var_rdd_dn9 = assign21610_e16646_d_n9;
        locals.var_rdd_dn10 = assign21610_e16646_d_n10;
        locals.var_rdd_dn11 = assign21610_e16646_d_n11;
        locals.var_rdd_dn14 = assign21610_e16646_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21620_e16659, assign21620_e16659_d_n0, assign21620_e16659_d_n2, assign21620_e16659_d_n4, assign21620_e16659_d_n5, assign21620_e16659_d_n6, assign21620_e16659_d_n7, assign21620_e16659_d_n8, assign21620_e16659_d_n9, assign21620_e16659_d_n10, assign21620_e16659_d_n11, assign21620_e16659_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 == 0.0)) {
        let assign21620_e16653: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21620_e16656: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21620_e16657: f64 = (assign21620_e16653 + assign21620_e16656);
        (assign21620_e16657, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21620_e16659;
        locals.var_rsd_dn0 = assign21620_e16659_d_n0;
        locals.var_rsd_dn2 = assign21620_e16659_d_n2;
        locals.var_rsd_dn4 = assign21620_e16659_d_n4;
        locals.var_rsd_dn5 = assign21620_e16659_d_n5;
        locals.var_rsd_dn6 = assign21620_e16659_d_n6;
        locals.var_rsd_dn7 = assign21620_e16659_d_n7;
        locals.var_rsd_dn8 = assign21620_e16659_d_n8;
        locals.var_rsd_dn9 = assign21620_e16659_d_n9;
        locals.var_rsd_dn10 = assign21620_e16659_d_n10;
        locals.var_rsd_dn11 = assign21620_e16659_d_n11;
        locals.var_rsd_dn14 = assign21620_e16659_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21630_e16665, assign21630_e16665_d_n0, assign21630_e16665_d_n2, assign21630_e16665_d_n4, assign21630_e16665_d_n5, assign21630_e16665_d_n6, assign21630_e16665_d_n7, assign21630_e16665_d_n8, assign21630_e16665_d_n9, assign21630_e16665_d_n10, assign21630_e16665_d_n11, assign21630_e16665_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21630_e16663: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign21630_e16663, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21630_e16665;
        locals.var_rdd_dn0 = assign21630_e16665_d_n0;
        locals.var_rdd_dn2 = assign21630_e16665_d_n2;
        locals.var_rdd_dn4 = assign21630_e16665_d_n4;
        locals.var_rdd_dn5 = assign21630_e16665_d_n5;
        locals.var_rdd_dn6 = assign21630_e16665_d_n6;
        locals.var_rdd_dn7 = assign21630_e16665_d_n7;
        locals.var_rdd_dn8 = assign21630_e16665_d_n8;
        locals.var_rdd_dn9 = assign21630_e16665_d_n9;
        locals.var_rdd_dn10 = assign21630_e16665_d_n10;
        locals.var_rdd_dn11 = assign21630_e16665_d_n11;
        locals.var_rdd_dn14 = assign21630_e16665_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21640_e16671, assign21640_e16671_d_n0, assign21640_e16671_d_n2, assign21640_e16671_d_n4, assign21640_e16671_d_n5, assign21640_e16671_d_n6, assign21640_e16671_d_n7, assign21640_e16671_d_n8, assign21640_e16671_d_n9, assign21640_e16671_d_n10, assign21640_e16671_d_n11, assign21640_e16671_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21640_e16669: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign21640_e16669, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21640_e16671;
        locals.var_rsd_dn0 = assign21640_e16671_d_n0;
        locals.var_rsd_dn2 = assign21640_e16671_d_n2;
        locals.var_rsd_dn4 = assign21640_e16671_d_n4;
        locals.var_rsd_dn5 = assign21640_e16671_d_n5;
        locals.var_rsd_dn6 = assign21640_e16671_d_n6;
        locals.var_rsd_dn7 = assign21640_e16671_d_n7;
        locals.var_rsd_dn8 = assign21640_e16671_d_n8;
        locals.var_rsd_dn9 = assign21640_e16671_d_n9;
        locals.var_rsd_dn10 = assign21640_e16671_d_n10;
        locals.var_rsd_dn11 = assign21640_e16671_d_n11;
        locals.var_rsd_dn14 = assign21640_e16671_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21650_e16683, assign21650_e16683_d_n0, assign21650_e16683_d_n2, assign21650_e16683_d_n4, assign21650_e16683_d_n5, assign21650_e16683_d_n6, assign21650_e16683_d_n7, assign21650_e16683_d_n8, assign21650_e16683_d_n9, assign21650_e16683_d_n10, assign21650_e16683_d_n11, assign21650_e16683_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21650_e16676: f64 = (locals.var_vdsemodenml * locals.var_rd0);
        let assign21650_e16677: f64 = (locals.var_rdd + assign21650_e16676);
        let assign21650_e16680: f64 = (locals.var_vdsemodervs * locals.var_rs0);
        let assign21650_e16681: f64 = (assign21650_e16677 + assign21650_e16680);
        (assign21650_e16681, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21650_e16683;
        locals.var_rdd_dn0 = assign21650_e16683_d_n0;
        locals.var_rdd_dn2 = assign21650_e16683_d_n2;
        locals.var_rdd_dn4 = assign21650_e16683_d_n4;
        locals.var_rdd_dn5 = assign21650_e16683_d_n5;
        locals.var_rdd_dn6 = assign21650_e16683_d_n6;
        locals.var_rdd_dn7 = assign21650_e16683_d_n7;
        locals.var_rdd_dn8 = assign21650_e16683_d_n8;
        locals.var_rdd_dn9 = assign21650_e16683_d_n9;
        locals.var_rdd_dn10 = assign21650_e16683_d_n10;
        locals.var_rdd_dn11 = assign21650_e16683_d_n11;
        locals.var_rdd_dn14 = assign21650_e16683_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21660_e16695, assign21660_e16695_d_n0, assign21660_e16695_d_n2, assign21660_e16695_d_n4, assign21660_e16695_d_n5, assign21660_e16695_d_n6, assign21660_e16695_d_n7, assign21660_e16695_d_n8, assign21660_e16695_d_n9, assign21660_e16695_d_n10, assign21660_e16695_d_n11, assign21660_e16695_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21660_e16688: f64 = (locals.var_vdsemodenml * locals.var_rs0);
        let assign21660_e16689: f64 = (locals.var_rsd + assign21660_e16688);
        let assign21660_e16692: f64 = (locals.var_vdsemodervs * locals.var_rd0);
        let assign21660_e16693: f64 = (assign21660_e16689 + assign21660_e16692);
        (assign21660_e16693, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21660_e16695;
        locals.var_rsd_dn0 = assign21660_e16695_d_n0;
        locals.var_rsd_dn2 = assign21660_e16695_d_n2;
        locals.var_rsd_dn4 = assign21660_e16695_d_n4;
        locals.var_rsd_dn5 = assign21660_e16695_d_n5;
        locals.var_rsd_dn6 = assign21660_e16695_d_n6;
        locals.var_rsd_dn7 = assign21660_e16695_d_n7;
        locals.var_rsd_dn8 = assign21660_e16695_d_n8;
        locals.var_rsd_dn9 = assign21660_e16695_d_n9;
        locals.var_rsd_dn10 = assign21660_e16695_d_n10;
        locals.var_rsd_dn11 = assign21660_e16695_d_n11;
        locals.var_rsd_dn14 = assign21660_e16695_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21670_e16705, assign21670_e16705_d_n0, assign21670_e16705_d_n2, assign21670_e16705_d_n4, assign21670_e16705_d_n5, assign21670_e16705_d_n6, assign21670_e16705_d_n7, assign21670_e16705_d_n8, assign21670_e16705_d_n9, assign21670_e16705_d_n10, assign21670_e16705_d_n11, assign21670_e16705_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21670_e16699: f64 = (locals.var_vdsemodenml * locals.var_rdd);
        let assign21670_e16702: f64 = (locals.var_vdsemodervs * locals.var_rsd);
        let assign21670_e16703: f64 = (assign21670_e16699 + assign21670_e16702);
        (assign21670_e16703, ((locals.var_vdsemodenml * locals.var_rdd_dn0) + (locals.var_vdsemodervs * locals.var_rsd_dn0)), ((locals.var_vdsemodenml * locals.var_rdd_dn2) + (locals.var_vdsemodervs * locals.var_rsd_dn2)), ((locals.var_vdsemodenml * locals.var_rdd_dn4) + (locals.var_vdsemodervs * locals.var_rsd_dn4)), ((locals.var_vdsemodenml * locals.var_rdd_dn5) + (locals.var_vdsemodervs * locals.var_rsd_dn5)), ((locals.var_vdsemodenml * locals.var_rdd_dn6) + (locals.var_vdsemodervs * locals.var_rsd_dn6)), ((locals.var_vdsemodenml * locals.var_rdd_dn7) + (locals.var_vdsemodervs * locals.var_rsd_dn7)), ((locals.var_vdsemodenml * locals.var_rdd_dn8) + (locals.var_vdsemodervs * locals.var_rsd_dn8)), ((locals.var_vdsemodenml * locals.var_rdd_dn9) + (locals.var_vdsemodervs * locals.var_rsd_dn9)), ((locals.var_vdsemodenml * locals.var_rdd_dn10) + (locals.var_vdsemodervs * locals.var_rsd_dn10)), ((locals.var_vdsemodenml * locals.var_rdd_dn11) + (locals.var_vdsemodervs * locals.var_rsd_dn11)), ((locals.var_vdsemodenml * locals.var_rdd_dn14) + (locals.var_vdsemodervs * locals.var_rsd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21670_e16705;
        locals.var_t0_dn0 = assign21670_e16705_d_n0;
        locals.var_t0_dn2 = assign21670_e16705_d_n2;
        locals.var_t0_dn4 = assign21670_e16705_d_n4;
        locals.var_t0_dn5 = assign21670_e16705_d_n5;
        locals.var_t0_dn6 = assign21670_e16705_d_n6;
        locals.var_t0_dn7 = assign21670_e16705_d_n7;
        locals.var_t0_dn8 = assign21670_e16705_d_n8;
        locals.var_t0_dn9 = assign21670_e16705_d_n9;
        locals.var_t0_dn10 = assign21670_e16705_d_n10;
        locals.var_t0_dn11 = assign21670_e16705_d_n11;
        locals.var_t0_dn14 = assign21670_e16705_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21710_e16737, assign21710_e16737_d_n0, assign21710_e16737_d_n2, assign21710_e16737_d_n4, assign21710_e16737_d_n5, assign21710_e16737_d_n6, assign21710_e16737_d_n7, assign21710_e16737_d_n8, assign21710_e16737_d_n9, assign21710_e16737_d_n10, assign21710_e16737_d_n11, assign21710_e16737_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21710_e16731: f64 = (locals.var_vdsemodenml * locals.var_rsd);
        let assign21710_e16734: f64 = (locals.var_vdsemodervs * locals.var_rdd);
        let assign21710_e16735: f64 = (assign21710_e16731 + assign21710_e16734);
        (assign21710_e16735, ((locals.var_vdsemodenml * locals.var_rsd_dn0) + (locals.var_vdsemodervs * locals.var_rdd_dn0)), ((locals.var_vdsemodenml * locals.var_rsd_dn2) + (locals.var_vdsemodervs * locals.var_rdd_dn2)), ((locals.var_vdsemodenml * locals.var_rsd_dn4) + (locals.var_vdsemodervs * locals.var_rdd_dn4)), ((locals.var_vdsemodenml * locals.var_rsd_dn5) + (locals.var_vdsemodervs * locals.var_rdd_dn5)), ((locals.var_vdsemodenml * locals.var_rsd_dn6) + (locals.var_vdsemodervs * locals.var_rdd_dn6)), ((locals.var_vdsemodenml * locals.var_rsd_dn7) + (locals.var_vdsemodervs * locals.var_rdd_dn7)), ((locals.var_vdsemodenml * locals.var_rsd_dn8) + (locals.var_vdsemodervs * locals.var_rdd_dn8)), ((locals.var_vdsemodenml * locals.var_rsd_dn9) + (locals.var_vdsemodervs * locals.var_rdd_dn9)), ((locals.var_vdsemodenml * locals.var_rsd_dn10) + (locals.var_vdsemodervs * locals.var_rdd_dn10)), ((locals.var_vdsemodenml * locals.var_rsd_dn11) + (locals.var_vdsemodervs * locals.var_rdd_dn11)), ((locals.var_vdsemodenml * locals.var_rsd_dn14) + (locals.var_vdsemodervs * locals.var_rdd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21710_e16737;
        locals.var_t0_dn0 = assign21710_e16737_d_n0;
        locals.var_t0_dn2 = assign21710_e16737_d_n2;
        locals.var_t0_dn4 = assign21710_e16737_d_n4;
        locals.var_t0_dn5 = assign21710_e16737_d_n5;
        locals.var_t0_dn6 = assign21710_e16737_d_n6;
        locals.var_t0_dn7 = assign21710_e16737_d_n7;
        locals.var_t0_dn8 = assign21710_e16737_d_n8;
        locals.var_t0_dn9 = assign21710_e16737_d_n9;
        locals.var_t0_dn10 = assign21710_e16737_d_n10;
        locals.var_t0_dn11 = assign21710_e16737_d_n11;
        locals.var_t0_dn14 = assign21710_e16737_d_n14;
        locals.var_t0_rv = 0.0;

        let assign21750_e16762: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard424 = assign21750_e16762;
        locals.var_guard424_rv = 0.0;

        let (assign21760_e16768, assign21760_e16768_d_n0, assign21760_e16768_d_n2, assign21760_e16768_d_n4, assign21760_e16768_d_n5, assign21760_e16768_d_n6, assign21760_e16768_d_n7, assign21760_e16768_d_n8, assign21760_e16768_d_n9, assign21760_e16768_d_n10, assign21760_e16768_d_n11, assign21760_e16768_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21760_e16766: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign21760_e16766, (-locals.var_vbs_bnd_dn0), (-locals.var_vbs_bnd_dn2), (-locals.var_vbs_bnd_dn4), (-locals.var_vbs_bnd_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_dn6), (-locals.var_vbs_bnd_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_dn9), (-locals.var_vbs_bnd_dn10), (-locals.var_vbs_bnd_dn11), (-locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21760_e16768;
        locals.var_t1_dn0 = assign21760_e16768_d_n0;
        locals.var_t1_dn2 = assign21760_e16768_d_n2;
        locals.var_t1_dn4 = assign21760_e16768_d_n4;
        locals.var_t1_dn5 = assign21760_e16768_d_n5;
        locals.var_t1_dn6 = assign21760_e16768_d_n6;
        locals.var_t1_dn7 = assign21760_e16768_d_n7;
        locals.var_t1_dn8 = assign21760_e16768_d_n8;
        locals.var_t1_dn9 = assign21760_e16768_d_n9;
        locals.var_t1_dn10 = assign21760_e16768_d_n10;
        locals.var_t1_dn11 = assign21760_e16768_d_n11;
        locals.var_t1_dn14 = assign21760_e16768_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21770_e16774, assign21770_e16774_d_n0, assign21770_e16774_d_n2, assign21770_e16774_d_n4, assign21770_e16774_d_n5, assign21770_e16774_d_n6, assign21770_e16774_d_n7, assign21770_e16774_d_n8, assign21770_e16774_d_n9, assign21770_e16774_d_n10, assign21770_e16774_d_n11, assign21770_e16774_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21770_e16772: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign21770_e16772, (locals.var_vbs_max_dn0 - locals.var_vbs_bnd_dn0), (locals.var_vbs_max_dn2 - locals.var_vbs_bnd_dn2), (locals.var_vbs_max_dn4 - locals.var_vbs_bnd_dn4), (locals.var_vbs_max_dn5 - locals.var_vbs_bnd_dn5), (locals.var_vbs_max_dn6 - locals.var_vbs_bnd_dn6), (locals.var_vbs_max_dn7 - locals.var_vbs_bnd_dn7), (locals.var_vbs_max_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_max_dn9 - locals.var_vbs_bnd_dn9), (locals.var_vbs_max_dn10 - locals.var_vbs_bnd_dn10), (locals.var_vbs_max_dn11 - locals.var_vbs_bnd_dn11), (locals.var_vbs_max_dn14 - locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21770_e16774;
        locals.var_t2_dn0 = assign21770_e16774_d_n0;
        locals.var_t2_dn2 = assign21770_e16774_d_n2;
        locals.var_t2_dn4 = assign21770_e16774_d_n4;
        locals.var_t2_dn5 = assign21770_e16774_d_n5;
        locals.var_t2_dn6 = assign21770_e16774_d_n6;
        locals.var_t2_dn7 = assign21770_e16774_d_n7;
        locals.var_t2_dn8 = assign21770_e16774_d_n8;
        locals.var_t2_dn9 = assign21770_e16774_d_n9;
        locals.var_t2_dn10 = assign21770_e16774_d_n10;
        locals.var_t2_dn11 = assign21770_e16774_d_n11;
        locals.var_t2_dn14 = assign21770_e16774_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21780_e16780, assign21780_e16780_d_n0, assign21780_e16780_d_n2, assign21780_e16780_d_n4, assign21780_e16780_d_n5, assign21780_e16780_d_n6, assign21780_e16780_d_n7, assign21780_e16780_d_n8, assign21780_e16780_d_n9, assign21780_e16780_d_n10, assign21780_e16780_d_n11, assign21780_e16780_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21780_e16778: f64 = (locals.var_t1 / locals.var_t2);
        (assign21780_e16778, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21780_e16780;
        locals.var_tmf1_dn0 = assign21780_e16780_d_n0;
        locals.var_tmf1_dn2 = assign21780_e16780_d_n2;
        locals.var_tmf1_dn4 = assign21780_e16780_d_n4;
        locals.var_tmf1_dn5 = assign21780_e16780_d_n5;
        locals.var_tmf1_dn6 = assign21780_e16780_d_n6;
        locals.var_tmf1_dn7 = assign21780_e16780_d_n7;
        locals.var_tmf1_dn8 = assign21780_e16780_d_n8;
        locals.var_tmf1_dn9 = assign21780_e16780_d_n9;
        locals.var_tmf1_dn10 = assign21780_e16780_d_n10;
        locals.var_tmf1_dn11 = assign21780_e16780_d_n11;
        locals.var_tmf1_dn14 = assign21780_e16780_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21790_e16786, assign21790_e16786_d_n0, assign21790_e16786_d_n2, assign21790_e16786_d_n4, assign21790_e16786_d_n5, assign21790_e16786_d_n6, assign21790_e16786_d_n7, assign21790_e16786_d_n8, assign21790_e16786_d_n9, assign21790_e16786_d_n10, assign21790_e16786_d_n11, assign21790_e16786_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21790_e16784: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21790_e16784, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21790_e16786;
        locals.var_tmf2_dn0 = assign21790_e16786_d_n0;
        locals.var_tmf2_dn2 = assign21790_e16786_d_n2;
        locals.var_tmf2_dn4 = assign21790_e16786_d_n4;
        locals.var_tmf2_dn5 = assign21790_e16786_d_n5;
        locals.var_tmf2_dn6 = assign21790_e16786_d_n6;
        locals.var_tmf2_dn7 = assign21790_e16786_d_n7;
        locals.var_tmf2_dn8 = assign21790_e16786_d_n8;
        locals.var_tmf2_dn9 = assign21790_e16786_d_n9;
        locals.var_tmf2_dn10 = assign21790_e16786_d_n10;
        locals.var_tmf2_dn11 = assign21790_e16786_d_n11;
        locals.var_tmf2_dn14 = assign21790_e16786_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21800_e16792, assign21800_e16792_d_n0, assign21800_e16792_d_n2, assign21800_e16792_d_n4, assign21800_e16792_d_n5, assign21800_e16792_d_n6, assign21800_e16792_d_n7, assign21800_e16792_d_n8, assign21800_e16792_d_n9, assign21800_e16792_d_n10, assign21800_e16792_d_n11, assign21800_e16792_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21800_e16790: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign21800_e16790, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign21800_e16792;
        locals.var_tmf3_dn0 = assign21800_e16792_d_n0;
        locals.var_tmf3_dn2 = assign21800_e16792_d_n2;
        locals.var_tmf3_dn4 = assign21800_e16792_d_n4;
        locals.var_tmf3_dn5 = assign21800_e16792_d_n5;
        locals.var_tmf3_dn6 = assign21800_e16792_d_n6;
        locals.var_tmf3_dn7 = assign21800_e16792_d_n7;
        locals.var_tmf3_dn8 = assign21800_e16792_d_n8;
        locals.var_tmf3_dn9 = assign21800_e16792_d_n9;
        locals.var_tmf3_dn10 = assign21800_e16792_d_n10;
        locals.var_tmf3_dn11 = assign21800_e16792_d_n11;
        locals.var_tmf3_dn14 = assign21800_e16792_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign21810_e16798, assign21810_e16798_d_n0, assign21810_e16798_d_n2, assign21810_e16798_d_n4, assign21810_e16798_d_n5, assign21810_e16798_d_n6, assign21810_e16798_d_n7, assign21810_e16798_d_n8, assign21810_e16798_d_n9, assign21810_e16798_d_n10, assign21810_e16798_d_n11, assign21810_e16798_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21810_e16796: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign21810_e16796, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign21810_e16798;
        locals.var_tmf4_dn0 = assign21810_e16798_d_n0;
        locals.var_tmf4_dn2 = assign21810_e16798_d_n2;
        locals.var_tmf4_dn4 = assign21810_e16798_d_n4;
        locals.var_tmf4_dn5 = assign21810_e16798_d_n5;
        locals.var_tmf4_dn6 = assign21810_e16798_d_n6;
        locals.var_tmf4_dn7 = assign21810_e16798_d_n7;
        locals.var_tmf4_dn8 = assign21810_e16798_d_n8;
        locals.var_tmf4_dn9 = assign21810_e16798_d_n9;
        locals.var_tmf4_dn10 = assign21810_e16798_d_n10;
        locals.var_tmf4_dn11 = assign21810_e16798_d_n11;
        locals.var_tmf4_dn14 = assign21810_e16798_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign21820_e16812, assign21820_e16812_d_n0, assign21820_e16812_d_n2, assign21820_e16812_d_n4, assign21820_e16812_d_n5, assign21820_e16812_d_n6, assign21820_e16812_d_n7, assign21820_e16812_d_n8, assign21820_e16812_d_n9, assign21820_e16812_d_n10, assign21820_e16812_d_n11, assign21820_e16812_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21820_e16803: f64 = (1.0 + locals.var_tmf1);
        let assign21820_e16805: f64 = (assign21820_e16803 + locals.var_tmf2);
        let assign21820_e16807: f64 = (assign21820_e16805 + locals.var_tmf3);
        let assign21820_e16809: f64 = (assign21820_e16807 + locals.var_tmf4);
        let assign21820_e16810: f64 = (1.0 / assign21820_e16809);
        (assign21820_e16810, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign21820_e16809 * assign21820_e16809))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign21820_e16812;
        locals.var_tmf0_dn0 = assign21820_e16812_d_n0;
        locals.var_tmf0_dn2 = assign21820_e16812_d_n2;
        locals.var_tmf0_dn4 = assign21820_e16812_d_n4;
        locals.var_tmf0_dn5 = assign21820_e16812_d_n5;
        locals.var_tmf0_dn6 = assign21820_e16812_d_n6;
        locals.var_tmf0_dn7 = assign21820_e16812_d_n7;
        locals.var_tmf0_dn8 = assign21820_e16812_d_n8;
        locals.var_tmf0_dn9 = assign21820_e16812_d_n9;
        locals.var_tmf0_dn10 = assign21820_e16812_d_n10;
        locals.var_tmf0_dn11 = assign21820_e16812_d_n11;
        locals.var_tmf0_dn14 = assign21820_e16812_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign21830_e16833, assign21830_e16833_d_n0, assign21830_e16833_d_n2, assign21830_e16833_d_n4, assign21830_e16833_d_n5, assign21830_e16833_d_n6, assign21830_e16833_d_n7, assign21830_e16833_d_n8, assign21830_e16833_d_n9, assign21830_e16833_d_n10, assign21830_e16833_d_n11, assign21830_e16833_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21830_e16817: f64 = (2.0 * locals.var_tmf1);
        let assign21830_e16818: f64 = (1.0 + assign21830_e16817);
        let assign21830_e16821: f64 = (3.0 * locals.var_tmf2);
        let assign21830_e16822: f64 = (assign21830_e16818 + assign21830_e16821);
        let assign21830_e16825: f64 = (4.0 * locals.var_tmf3);
        let assign21830_e16826: f64 = (assign21830_e16822 + assign21830_e16825);
        let assign21830_e16827: f64 = (-assign21830_e16826);
        let assign21830_e16829: f64 = (assign21830_e16827 * locals.var_tmf0);
        let assign21830_e16831: f64 = (assign21830_e16829 * locals.var_tmf0);
        (assign21830_e16831, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21830_e16833;
        locals.var_vbscldvbs_dn0 = assign21830_e16833_d_n0;
        locals.var_vbscldvbs_dn2 = assign21830_e16833_d_n2;
        locals.var_vbscldvbs_dn4 = assign21830_e16833_d_n4;
        locals.var_vbscldvbs_dn5 = assign21830_e16833_d_n5;
        locals.var_vbscldvbs_dn6 = assign21830_e16833_d_n6;
        locals.var_vbscldvbs_dn7 = assign21830_e16833_d_n7;
        locals.var_vbscldvbs_dn8 = assign21830_e16833_d_n8;
        locals.var_vbscldvbs_dn9 = assign21830_e16833_d_n9;
        locals.var_vbscldvbs_dn10 = assign21830_e16833_d_n10;
        locals.var_vbscldvbs_dn11 = assign21830_e16833_d_n11;
        locals.var_vbscldvbs_dn14 = assign21830_e16833_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21840_e16841, assign21840_e16841_d_n0, assign21840_e16841_d_n2, assign21840_e16841_d_n4, assign21840_e16841_d_n5, assign21840_e16841_d_n6, assign21840_e16841_d_n7, assign21840_e16841_d_n8, assign21840_e16841_d_n9, assign21840_e16841_d_n10, assign21840_e16841_d_n11, assign21840_e16841_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21840_e16838: f64 = (1.0 - locals.var_tmf0);
        let assign21840_e16839: f64 = (locals.var_t2 * assign21840_e16838);
        (assign21840_e16839, ((locals.var_t2_dn0 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign21840_e16841;
        locals.var_ty_dn0 = assign21840_e16841_d_n0;
        locals.var_ty_dn2 = assign21840_e16841_d_n2;
        locals.var_ty_dn4 = assign21840_e16841_d_n4;
        locals.var_ty_dn5 = assign21840_e16841_d_n5;
        locals.var_ty_dn6 = assign21840_e16841_d_n6;
        locals.var_ty_dn7 = assign21840_e16841_d_n7;
        locals.var_ty_dn8 = assign21840_e16841_d_n8;
        locals.var_ty_dn9 = assign21840_e16841_d_n9;
        locals.var_ty_dn10 = assign21840_e16841_d_n10;
        locals.var_ty_dn11 = assign21840_e16841_d_n11;
        locals.var_ty_dn14 = assign21840_e16841_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign21850_e16851, assign21850_e16851_d_n0, assign21850_e16851_d_n2, assign21850_e16851_d_n4, assign21850_e16851_d_n5, assign21850_e16851_d_n6, assign21850_e16851_d_n7, assign21850_e16851_d_n8, assign21850_e16851_d_n9, assign21850_e16851_d_n10, assign21850_e16851_d_n11, assign21850_e16851_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21850_e16845: f64 = (1.0 - locals.var_tmf0);
        let assign21850_e16848: f64 = (locals.var_tmf1 * locals.var_vbscldvbs);
        let assign21850_e16849: f64 = (assign21850_e16845 + assign21850_e16848);
        (assign21850_e16849, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21850_e16851;
        locals.var_t0_dn0 = assign21850_e16851_d_n0;
        locals.var_t0_dn2 = assign21850_e16851_d_n2;
        locals.var_t0_dn4 = assign21850_e16851_d_n4;
        locals.var_t0_dn5 = assign21850_e16851_d_n5;
        locals.var_t0_dn6 = assign21850_e16851_d_n6;
        locals.var_t0_dn7 = assign21850_e16851_d_n7;
        locals.var_t0_dn8 = assign21850_e16851_d_n8;
        locals.var_t0_dn9 = assign21850_e16851_d_n9;
        locals.var_t0_dn10 = assign21850_e16851_d_n10;
        locals.var_t0_dn11 = assign21850_e16851_d_n11;
        locals.var_t0_dn14 = assign21850_e16851_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21860_e16856, assign21860_e16856_d_n0, assign21860_e16856_d_n2, assign21860_e16856_d_n4, assign21860_e16856_d_n5, assign21860_e16856_d_n6, assign21860_e16856_d_n7, assign21860_e16856_d_n8, assign21860_e16856_d_n9, assign21860_e16856_d_n10, assign21860_e16856_d_n11, assign21860_e16856_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21860_e16854: f64 = (-locals.var_vbscldvbs);
        (assign21860_e16854, (-locals.var_vbscldvbs_dn0), (-locals.var_vbscldvbs_dn2), (-locals.var_vbscldvbs_dn4), (-locals.var_vbscldvbs_dn5), (-locals.var_vbscldvbs_dn6), (-locals.var_vbscldvbs_dn7), (-locals.var_vbscldvbs_dn8), (-locals.var_vbscldvbs_dn9), (-locals.var_vbscldvbs_dn10), (-locals.var_vbscldvbs_dn11), (-locals.var_vbscldvbs_dn14),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21860_e16856;
        locals.var_vbscldvbs_dn0 = assign21860_e16856_d_n0;
        locals.var_vbscldvbs_dn2 = assign21860_e16856_d_n2;
        locals.var_vbscldvbs_dn4 = assign21860_e16856_d_n4;
        locals.var_vbscldvbs_dn5 = assign21860_e16856_d_n5;
        locals.var_vbscldvbs_dn6 = assign21860_e16856_d_n6;
        locals.var_vbscldvbs_dn7 = assign21860_e16856_d_n7;
        locals.var_vbscldvbs_dn8 = assign21860_e16856_d_n8;
        locals.var_vbscldvbs_dn9 = assign21860_e16856_d_n9;
        locals.var_vbscldvbs_dn10 = assign21860_e16856_d_n10;
        locals.var_vbscldvbs_dn11 = assign21860_e16856_d_n11;
        locals.var_vbscldvbs_dn14 = assign21860_e16856_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21870_e16862, assign21870_e16862_d_n0, assign21870_e16862_d_n2, assign21870_e16862_d_n4, assign21870_e16862_d_n5, assign21870_e16862_d_n6, assign21870_e16862_d_n7, assign21870_e16862_d_n8, assign21870_e16862_d_n9, assign21870_e16862_d_n10, assign21870_e16862_d_n11, assign21870_e16862_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21870_e16860: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign21870_e16860, (locals.var_vbs_bnd_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21870_e16862;
        locals.var_vbscl_dn0 = assign21870_e16862_d_n0;
        locals.var_vbscl_dn2 = assign21870_e16862_d_n2;
        locals.var_vbscl_dn4 = assign21870_e16862_d_n4;
        locals.var_vbscl_dn5 = assign21870_e16862_d_n5;
        locals.var_vbscl_dn6 = assign21870_e16862_d_n6;
        locals.var_vbscl_dn7 = assign21870_e16862_d_n7;
        locals.var_vbscl_dn8 = assign21870_e16862_d_n8;
        locals.var_vbscl_dn9 = assign21870_e16862_d_n9;
        locals.var_vbscl_dn10 = assign21870_e16862_d_n10;
        locals.var_vbscl_dn11 = assign21870_e16862_d_n11;
        locals.var_vbscl_dn14 = assign21870_e16862_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21880_e16868, assign21880_e16868_d_n0, assign21880_e16868_d_n2, assign21880_e16868_d_n4, assign21880_e16868_d_n5, assign21880_e16868_d_n6, assign21880_e16868_d_n7, assign21880_e16868_d_n8, assign21880_e16868_d_n9, assign21880_e16868_d_n10, assign21880_e16868_d_n11, assign21880_e16868_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21880_e16866: f64 = (1.0 / locals.var_t2);
        (assign21880_e16866, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21880_e16868;
        locals.var_t3_dn0 = assign21880_e16868_d_n0;
        locals.var_t3_dn2 = assign21880_e16868_d_n2;
        locals.var_t3_dn4 = assign21880_e16868_d_n4;
        locals.var_t3_dn5 = assign21880_e16868_d_n5;
        locals.var_t3_dn6 = assign21880_e16868_d_n6;
        locals.var_t3_dn7 = assign21880_e16868_d_n7;
        locals.var_t3_dn8 = assign21880_e16868_d_n8;
        locals.var_t3_dn9 = assign21880_e16868_d_n9;
        locals.var_t3_dn10 = assign21880_e16868_d_n10;
        locals.var_t3_dn11 = assign21880_e16868_d_n11;
        locals.var_t3_dn14 = assign21880_e16868_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21890_e16874, assign21890_e16874_d_n0, assign21890_e16874_d_n2, assign21890_e16874_d_n4, assign21890_e16874_d_n5, assign21890_e16874_d_n6, assign21890_e16874_d_n7, assign21890_e16874_d_n8, assign21890_e16874_d_n9, assign21890_e16874_d_n10, assign21890_e16874_d_n11, assign21890_e16874_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21890_e16872: f64 = (locals.var_t1 * locals.var_t3);
        (assign21890_e16872, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21890_e16874;
        locals.var_t4_dn0 = assign21890_e16874_d_n0;
        locals.var_t4_dn2 = assign21890_e16874_d_n2;
        locals.var_t4_dn4 = assign21890_e16874_d_n4;
        locals.var_t4_dn5 = assign21890_e16874_d_n5;
        locals.var_t4_dn6 = assign21890_e16874_d_n6;
        locals.var_t4_dn7 = assign21890_e16874_d_n7;
        locals.var_t4_dn8 = assign21890_e16874_d_n8;
        locals.var_t4_dn9 = assign21890_e16874_d_n9;
        locals.var_t4_dn10 = assign21890_e16874_d_n10;
        locals.var_t4_dn11 = assign21890_e16874_d_n11;
        locals.var_t4_dn14 = assign21890_e16874_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21900_e16880, assign21900_e16880_d_n0, assign21900_e16880_d_n2, assign21900_e16880_d_n4, assign21900_e16880_d_n5, assign21900_e16880_d_n6, assign21900_e16880_d_n7, assign21900_e16880_d_n8, assign21900_e16880_d_n9, assign21900_e16880_d_n10, assign21900_e16880_d_n11, assign21900_e16880_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21900_e16878: f64 = (locals.var_t4 * locals.var_t4);
        (assign21900_e16878, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21900_e16880;
        locals.var_t5_dn0 = assign21900_e16880_d_n0;
        locals.var_t5_dn2 = assign21900_e16880_d_n2;
        locals.var_t5_dn4 = assign21900_e16880_d_n4;
        locals.var_t5_dn5 = assign21900_e16880_d_n5;
        locals.var_t5_dn6 = assign21900_e16880_d_n6;
        locals.var_t5_dn7 = assign21900_e16880_d_n7;
        locals.var_t5_dn8 = assign21900_e16880_d_n8;
        locals.var_t5_dn9 = assign21900_e16880_d_n9;
        locals.var_t5_dn10 = assign21900_e16880_d_n10;
        locals.var_t5_dn11 = assign21900_e16880_d_n11;
        locals.var_t5_dn14 = assign21900_e16880_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21910_e16894, assign21910_e16894_d_n0, assign21910_e16894_d_n2, assign21910_e16894_d_n4, assign21910_e16894_d_n5, assign21910_e16894_d_n6, assign21910_e16894_d_n7, assign21910_e16894_d_n8, assign21910_e16894_d_n9, assign21910_e16894_d_n10, assign21910_e16894_d_n11, assign21910_e16894_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21910_e16884: f64 = (1.0 + locals.var_t4);
        let assign21910_e16888: f64 = (1.0 + locals.var_t4);
        let assign21910_e16890: f64 = (assign21910_e16888 + locals.var_t5);
        let assign21910_e16891: f64 = (locals.var_t5 * assign21910_e16890);
        let assign21910_e16892: f64 = (assign21910_e16884 + assign21910_e16891);
        (assign21910_e16892, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign21910_e16894;
        locals.var_t7_dn0 = assign21910_e16894_d_n0;
        locals.var_t7_dn2 = assign21910_e16894_d_n2;
        locals.var_t7_dn4 = assign21910_e16894_d_n4;
        locals.var_t7_dn5 = assign21910_e16894_d_n5;
        locals.var_t7_dn6 = assign21910_e16894_d_n6;
        locals.var_t7_dn7 = assign21910_e16894_d_n7;
        locals.var_t7_dn8 = assign21910_e16894_d_n8;
        locals.var_t7_dn9 = assign21910_e16894_d_n9;
        locals.var_t7_dn10 = assign21910_e16894_d_n10;
        locals.var_t7_dn11 = assign21910_e16894_d_n11;
        locals.var_t7_dn14 = assign21910_e16894_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign21920_e16916, assign21920_e16916_d_n0, assign21920_e16916_d_n2, assign21920_e16916_d_n4, assign21920_e16916_d_n5, assign21920_e16916_d_n6, assign21920_e16916_d_n7, assign21920_e16916_d_n8, assign21920_e16916_d_n9, assign21920_e16916_d_n10, assign21920_e16916_d_n11, assign21920_e16916_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21920_e16899: f64 = (2.0 * locals.var_t4);
        let assign21920_e16900: f64 = (1.0 + assign21920_e16899);
        let assign21920_e16903: f64 = (3.0 * locals.var_t5);
        let assign21920_e16904: f64 = (assign21920_e16900 + assign21920_e16903);
        let assign21920_e16907: f64 = (4.0 * locals.var_t4);
        let assign21920_e16909: f64 = (assign21920_e16907 * locals.var_t5);
        let assign21920_e16910: f64 = (assign21920_e16904 + assign21920_e16909);
        let assign21920_e16913: f64 = (locals.var_t7 * locals.var_t7);
        let assign21920_e16914: f64 = (assign21920_e16910 / assign21920_e16913);
        (assign21920_e16914, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn0))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn2))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn4))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn5))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn6))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn7))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn8))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn9))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn10))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn11))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn14))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign21920_e16913 * assign21920_e16913)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21920_e16916;
        locals.var_vbscldvbs_dn0 = assign21920_e16916_d_n0;
        locals.var_vbscldvbs_dn2 = assign21920_e16916_d_n2;
        locals.var_vbscldvbs_dn4 = assign21920_e16916_d_n4;
        locals.var_vbscldvbs_dn5 = assign21920_e16916_d_n5;
        locals.var_vbscldvbs_dn6 = assign21920_e16916_d_n6;
        locals.var_vbscldvbs_dn7 = assign21920_e16916_d_n7;
        locals.var_vbscldvbs_dn8 = assign21920_e16916_d_n8;
        locals.var_vbscldvbs_dn9 = assign21920_e16916_d_n9;
        locals.var_vbscldvbs_dn10 = assign21920_e16916_d_n10;
        locals.var_vbscldvbs_dn11 = assign21920_e16916_d_n11;
        locals.var_vbscldvbs_dn14 = assign21920_e16916_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21930_e16921, assign21930_e16921_d_n0, assign21930_e16921_d_n2, assign21930_e16921_d_n4, assign21930_e16921_d_n5, assign21930_e16921_d_n6, assign21930_e16921_d_n7, assign21930_e16921_d_n8, assign21930_e16921_d_n9, assign21930_e16921_d_n10, assign21930_e16921_d_n11, assign21930_e16921_d_n14,) = {
    if (locals.var_guard424 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21930_e16921;
        locals.var_vbscl_dn0 = assign21930_e16921_d_n0;
        locals.var_vbscl_dn2 = assign21930_e16921_d_n2;
        locals.var_vbscl_dn4 = assign21930_e16921_d_n4;
        locals.var_vbscl_dn5 = assign21930_e16921_d_n5;
        locals.var_vbscl_dn6 = assign21930_e16921_d_n6;
        locals.var_vbscl_dn7 = assign21930_e16921_d_n7;
        locals.var_vbscl_dn8 = assign21930_e16921_d_n8;
        locals.var_vbscl_dn9 = assign21930_e16921_d_n9;
        locals.var_vbscl_dn10 = assign21930_e16921_d_n10;
        locals.var_vbscl_dn11 = assign21930_e16921_d_n11;
        locals.var_vbscl_dn14 = assign21930_e16921_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21940_e16926, assign21940_e16926_d_n0, assign21940_e16926_d_n2, assign21940_e16926_d_n4, assign21940_e16926_d_n5, assign21940_e16926_d_n6, assign21940_e16926_d_n7, assign21940_e16926_d_n8, assign21940_e16926_d_n9, assign21940_e16926_d_n10, assign21940_e16926_d_n11, assign21940_e16926_d_n14,) = {
    if (locals.var_guard424 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21940_e16926;
        locals.var_vbscldvbs_dn0 = assign21940_e16926_d_n0;
        locals.var_vbscldvbs_dn2 = assign21940_e16926_d_n2;
        locals.var_vbscldvbs_dn4 = assign21940_e16926_d_n4;
        locals.var_vbscldvbs_dn5 = assign21940_e16926_d_n5;
        locals.var_vbscldvbs_dn6 = assign21940_e16926_d_n6;
        locals.var_vbscldvbs_dn7 = assign21940_e16926_d_n7;
        locals.var_vbscldvbs_dn8 = assign21940_e16926_d_n8;
        locals.var_vbscldvbs_dn9 = assign21940_e16926_d_n9;
        locals.var_vbscldvbs_dn10 = assign21940_e16926_d_n10;
        locals.var_vbscldvbs_dn11 = assign21940_e16926_d_n11;
        locals.var_vbscldvbs_dn14 = assign21940_e16926_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let assign21950_e16929: f64 = (locals.var_vbscldvbs * locals.var_vds);
        let assign21950_e16931: f64 = (assign21950_e16929 / 2.0);
        locals.var_t1 = assign21950_e16931;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs_dn0 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs_dn2 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs_dn4 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs_dn5 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs_dn6 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs_dn7 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs_dn8 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs_dn9 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs_dn10 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs_dn11 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs_dn14 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn14)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign21960_e16934: f64 = (2.0 * locals.var_t1);
        let assign21960_e16936: f64 = (assign21960_e16934 / p.p262);
        locals.var_tmf1 = assign21960_e16936;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p262);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign21970_e16941: f64 = (1.0 / 2.0);
        let assign21970_e16945: f64 = (1.0 / 6.0);
        let assign21970_e16949: f64 = (1.0 / 24.0);
        let assign21970_e16953: f64 = (1.0 / 120.0);
        let assign21970_e16957: f64 = (1.0 / 720.0);
        let assign21970_e16961: f64 = (1.0 / 5040.0);
        let assign21970_e16962: f64 = (locals.var_tmf1 * assign21970_e16961);
        let assign21970_e16963: f64 = (assign21970_e16957 + assign21970_e16962);
        let assign21970_e16964: f64 = (locals.var_tmf1 * assign21970_e16963);
        let assign21970_e16965: f64 = (assign21970_e16953 + assign21970_e16964);
        let assign21970_e16966: f64 = (locals.var_tmf1 * assign21970_e16965);
        let assign21970_e16967: f64 = (assign21970_e16949 + assign21970_e16966);
        let assign21970_e16968: f64 = (locals.var_tmf1 * assign21970_e16967);
        let assign21970_e16969: f64 = (assign21970_e16945 + assign21970_e16968);
        let assign21970_e16970: f64 = (locals.var_tmf1 * assign21970_e16969);
        let assign21970_e16971: f64 = (assign21970_e16941 + assign21970_e16970);
        let assign21970_e16972: f64 = (locals.var_tmf1 * assign21970_e16971);
        let assign21970_e16973: f64 = (1.0 + assign21970_e16972);
        locals.var_tmf2 = assign21970_e16973;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign21970_e16961)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign21980_e16976: f64 = (1.0 / 2.0);
        let assign21980_e16980: f64 = (1.0 / 3.0);
        let assign21980_e16984: f64 = (1.0 / 8.0);
        let assign21980_e16988: f64 = (1.0 / 30.0);
        let assign21980_e16992: f64 = (1.0 / 144.0);
        let assign21980_e16996: f64 = (1.0 / 840.0);
        let assign21980_e16997: f64 = (locals.var_tmf1 * assign21980_e16996);
        let assign21980_e16998: f64 = (assign21980_e16992 + assign21980_e16997);
        let assign21980_e16999: f64 = (locals.var_tmf1 * assign21980_e16998);
        let assign21980_e17000: f64 = (assign21980_e16988 + assign21980_e16999);
        let assign21980_e17001: f64 = (locals.var_tmf1 * assign21980_e17000);
        let assign21980_e17002: f64 = (assign21980_e16984 + assign21980_e17001);
        let assign21980_e17003: f64 = (locals.var_tmf1 * assign21980_e17002);
        let assign21980_e17004: f64 = (assign21980_e16980 + assign21980_e17003);
        let assign21980_e17005: f64 = (locals.var_tmf1 * assign21980_e17004);
        let assign21980_e17006: f64 = (assign21980_e16976 + assign21980_e17005);
        locals.var_tmf3 = assign21980_e17006;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21980_e16996)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21980_e16996)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21980_e16996)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21980_e16996)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21980_e16996)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21980_e16996)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21980_e16996)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21980_e16996)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21980_e16996)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign21980_e16996)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign21980_e16996)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign21990_e17009: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd = assign21990_e17009;
        locals.var_vzadd_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign22000_e17011: f64 = (-2.0);
        let assign22000_e17013: f64 = (assign22000_e17011 * locals.var_tmf3);
        let assign22000_e17016: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign22000_e17017: f64 = (assign22000_e17013 / assign22000_e17016);
        locals.var_t2 = assign22000_e17017;
        locals.var_t2_dn0 = ((((assign22000_e17011 * locals.var_tmf3_dn0) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn2 = ((((assign22000_e17011 * locals.var_tmf3_dn2) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn4 = ((((assign22000_e17011 * locals.var_tmf3_dn4) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn5 = ((((assign22000_e17011 * locals.var_tmf3_dn5) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn6 = ((((assign22000_e17011 * locals.var_tmf3_dn6) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn7 = ((((assign22000_e17011 * locals.var_tmf3_dn7) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn8 = ((((assign22000_e17011 * locals.var_tmf3_dn8) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn9 = ((((assign22000_e17011 * locals.var_tmf3_dn9) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn10 = ((((assign22000_e17011 * locals.var_tmf3_dn10) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn11 = ((((assign22000_e17011 * locals.var_tmf3_dn11) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn14 = ((((assign22000_e17011 * locals.var_tmf3_dn14) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_rv = 0.0;

        let assign22010_e17020: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign22010_e17020;
        locals.var_guard425_rv = 0.0;

        let (assign22020_e17024, assign22020_e17024_d_n0, assign22020_e17024_d_n2, assign22020_e17024_d_n4, assign22020_e17024_d_n5, assign22020_e17024_d_n6, assign22020_e17024_d_n7, assign22020_e17024_d_n8, assign22020_e17024_d_n9, assign22020_e17024_d_n10, assign22020_e17024_d_n11, assign22020_e17024_d_n14,) = {
    if (locals.var_guard425 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign22020_e17024;
        locals.var_vzadd_dn0 = assign22020_e17024_d_n0;
        locals.var_vzadd_dn2 = assign22020_e17024_d_n2;
        locals.var_vzadd_dn4 = assign22020_e17024_d_n4;
        locals.var_vzadd_dn5 = assign22020_e17024_d_n5;
        locals.var_vzadd_dn6 = assign22020_e17024_d_n6;
        locals.var_vzadd_dn7 = assign22020_e17024_d_n7;
        locals.var_vzadd_dn8 = assign22020_e17024_d_n8;
        locals.var_vzadd_dn9 = assign22020_e17024_d_n9;
        locals.var_vzadd_dn10 = assign22020_e17024_d_n10;
        locals.var_vzadd_dn11 = assign22020_e17024_d_n11;
        locals.var_vzadd_dn14 = assign22020_e17024_d_n14;
        locals.var_vzadd_rv = 0.0;

        let assign22030_e17027: f64 = (locals.var_vbscl + locals.var_vzadd);
        locals.var_vbsz = assign22030_e17027;
        locals.var_vbsz_dn0 = (locals.var_vbscl_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbscl_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn4 = (locals.var_vbscl_dn4 + locals.var_vzadd_dn4);
        locals.var_vbsz_dn5 = (locals.var_vbscl_dn5 + locals.var_vzadd_dn5);
        locals.var_vbsz_dn6 = (locals.var_vbscl_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbscl_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn8 = (locals.var_vbscl_dn8 + locals.var_vzadd_dn8);
        locals.var_vbsz_dn9 = (locals.var_vbscl_dn9 + locals.var_vzadd_dn9);
        locals.var_vbsz_dn10 = (locals.var_vbscl_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbscl_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn14 = (locals.var_vbscl_dn14 + locals.var_vzadd_dn14);
        locals.var_vbsz_rv = 0.0;

        let assign22040_e17031: f64 = (2.0 * locals.var_vzadd);
        let assign22040_e17032: f64 = (locals.var_vds + assign22040_e17031);
        locals.var_vdsz = assign22040_e17032;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd_dn4));
        locals.var_vdsz_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd_dn5));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd_dn8));
        locals.var_vdsz_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd_dn9));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd_dn14));
        locals.var_vdsz_rv = 0.0;

        let assign22050_e17035: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign22050_e17035;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn4 = locals.var_vzadd_dn4;
        locals.var_vgsz_dn5 = locals.var_vzadd_dn5;
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd_dn8);
        locals.var_vgsz_dn9 = locals.var_vzadd_dn9;
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = locals.var_vzadd_dn11;
        locals.var_vgsz_dn14 = locals.var_vzadd_dn14;
        locals.var_vgsz_rv = 0.0;

        let assign22060_e17038: f64 = (locals.var_qnsub_esi * locals.var_cox0_inv);
        let assign22060_e17040: f64 = (assign22060_e17038 * locals.var_cox0_inv);
        locals.var_t1 = assign22060_e17040;
        locals.var_t1_dn0 = ((locals.var_qnsub_esi_dn0 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn2 = ((locals.var_qnsub_esi_dn2 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn4 = ((locals.var_qnsub_esi_dn4 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn5 = ((locals.var_qnsub_esi_dn5 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn6 = ((locals.var_qnsub_esi_dn6 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn7 = ((locals.var_qnsub_esi_dn7 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn8 = ((locals.var_qnsub_esi_dn8 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn9 = ((locals.var_qnsub_esi_dn9 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn10 = ((locals.var_qnsub_esi_dn10 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn11 = ((locals.var_qnsub_esi_dn11 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn14 = ((locals.var_qnsub_esi_dn14 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_rv = 0.0;

        let assign22070_e17043: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2 = assign22070_e17043;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = locals.var_vgs_dn6;
        locals.var_t2_dn7 = locals.var_vgs_dn7;
        locals.var_t2_dn8 = locals.var_vgs_dn8;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign22080_e17047: f64 = (2.0 / locals.var_t1);
        let assign22080_e17051: f64 = (1.0 / locals.var_betatnom);
        let assign22080_e17052: f64 = (locals.var_t2 - assign22080_e17051);
        let assign22080_e17054: f64 = (assign22080_e17052 - locals.var_vbscl);
        let assign22080_e17055: f64 = (assign22080_e17047 * assign22080_e17054);
        let assign22080_e17056: f64 = (1.0 + assign22080_e17055);
        locals.var_t3 = assign22080_e17056;
        locals.var_t3_dn0 = (((-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn0 - locals.var_vbscl_dn0)));
        locals.var_t3_dn2 = (((-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn2 - locals.var_vbscl_dn2)));
        locals.var_t3_dn4 = (((-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn4 - locals.var_vbscl_dn4)));
        locals.var_t3_dn5 = (((-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn5 - locals.var_vbscl_dn5)));
        locals.var_t3_dn6 = (((-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn6 - locals.var_vbscl_dn6)));
        locals.var_t3_dn7 = (((-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn7 - locals.var_vbscl_dn7)));
        locals.var_t3_dn8 = (((-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn8 - locals.var_vbscl_dn8)));
        locals.var_t3_dn9 = (((-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn9 - locals.var_vbscl_dn9)));
        locals.var_t3_dn10 = (((-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn10 - locals.var_vbscl_dn10)));
        locals.var_t3_dn11 = (((-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn11 - locals.var_vbscl_dn11)));
        locals.var_t3_dn14 = (((-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn14 - locals.var_vbscl_dn14)));
        locals.var_t3_rv = 0.0;

        let assign22090_e17059: f64 = (locals.var_t3 * locals.var_t3);
        let assign22090_e17062: f64 = (4.0 * 0.001);
        let assign22090_e17064: f64 = (assign22090_e17062 * 0.001);
        let assign22090_e17065: f64 = (assign22090_e17059 + assign22090_e17064);
        let assign22090_e17066: f64 = (assign22090_e17065).sqrt();
        locals.var_tmf2 = assign22090_e17066;
        locals.var_tmf2_dn0 = (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn2 = (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn4 = (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn5 = (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn6 = (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn7 = (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn8 = (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn9 = (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn10 = (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn11 = (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn14 = (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_rv = 0.0;

        let assign22100_e17071: f64 = (locals.var_t3 / locals.var_tmf2);
        let assign22100_e17072: f64 = (1.0 + assign22100_e17071);
        let assign22100_e17073: f64 = (0.5 * assign22100_e17072);
        locals.var_t5 = assign22100_e17073;
        locals.var_t5_dn0 = (0.5 * (((locals.var_t3_dn0 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn2 = (0.5 * (((locals.var_t3_dn2 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn4 = (0.5 * (((locals.var_t3_dn4 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn5 = (0.5 * (((locals.var_t3_dn5 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn6 = (0.5 * (((locals.var_t3_dn6 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn7 = (0.5 * (((locals.var_t3_dn7 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn8 = (0.5 * (((locals.var_t3_dn8 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn9 = (0.5 * (((locals.var_t3_dn9 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn10 = (0.5 * (((locals.var_t3_dn10 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn11 = (0.5 * (((locals.var_t3_dn11 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn14 = (0.5 * (((locals.var_t3_dn14 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_rv = 0.0;

        let assign22110_e17077: f64 = (locals.var_t3 + locals.var_tmf2);
        let assign22110_e17078: f64 = (0.5 * assign22110_e17077);
        locals.var_t4 = assign22110_e17078;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3_dn0 + locals.var_tmf2_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3_dn2 + locals.var_tmf2_dn2));
        locals.var_t4_dn4 = (0.5 * (locals.var_t3_dn4 + locals.var_tmf2_dn4));
        locals.var_t4_dn5 = (0.5 * (locals.var_t3_dn5 + locals.var_tmf2_dn5));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3_dn6 + locals.var_tmf2_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3_dn7 + locals.var_tmf2_dn7));
        locals.var_t4_dn8 = (0.5 * (locals.var_t3_dn8 + locals.var_tmf2_dn8));
        locals.var_t4_dn9 = (0.5 * (locals.var_t3_dn9 + locals.var_tmf2_dn9));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3_dn10 + locals.var_tmf2_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3_dn11 + locals.var_tmf2_dn11));
        locals.var_t4_dn14 = (0.5 * (locals.var_t3_dn14 + locals.var_tmf2_dn14));
        locals.var_t4_rv = 0.0;

        let assign22120_e17081: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign22120_e17081;
        locals.var_guard426_rv = 0.0;

        let (assign22130_e17085, assign22130_e17085_d_n0, assign22130_e17085_d_n2, assign22130_e17085_d_n4, assign22130_e17085_d_n5, assign22130_e17085_d_n6, assign22130_e17085_d_n7, assign22130_e17085_d_n8, assign22130_e17085_d_n9, assign22130_e17085_d_n10, assign22130_e17085_d_n11, assign22130_e17085_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22130_e17085;
        locals.var_t4_dn0 = assign22130_e17085_d_n0;
        locals.var_t4_dn2 = assign22130_e17085_d_n2;
        locals.var_t4_dn4 = assign22130_e17085_d_n4;
        locals.var_t4_dn5 = assign22130_e17085_d_n5;
        locals.var_t4_dn6 = assign22130_e17085_d_n6;
        locals.var_t4_dn7 = assign22130_e17085_d_n7;
        locals.var_t4_dn8 = assign22130_e17085_d_n8;
        locals.var_t4_dn9 = assign22130_e17085_d_n9;
        locals.var_t4_dn10 = assign22130_e17085_d_n10;
        locals.var_t4_dn11 = assign22130_e17085_d_n11;
        locals.var_t4_dn14 = assign22130_e17085_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22140_e17089, assign22140_e17089_d_n0, assign22140_e17089_d_n2, assign22140_e17089_d_n4, assign22140_e17089_d_n5, assign22140_e17089_d_n6, assign22140_e17089_d_n7, assign22140_e17089_d_n8, assign22140_e17089_d_n9, assign22140_e17089_d_n10, assign22140_e17089_d_n11, assign22140_e17089_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22140_e17089;
        locals.var_t5_dn0 = assign22140_e17089_d_n0;
        locals.var_t5_dn2 = assign22140_e17089_d_n2;
        locals.var_t5_dn4 = assign22140_e17089_d_n4;
        locals.var_t5_dn5 = assign22140_e17089_d_n5;
        locals.var_t5_dn6 = assign22140_e17089_d_n6;
        locals.var_t5_dn7 = assign22140_e17089_d_n7;
        locals.var_t5_dn8 = assign22140_e17089_d_n8;
        locals.var_t5_dn9 = assign22140_e17089_d_n9;
        locals.var_t5_dn10 = assign22140_e17089_d_n10;
        locals.var_t5_dn11 = assign22140_e17089_d_n11;
        locals.var_t5_dn14 = assign22140_e17089_d_n14;
        locals.var_t5_rv = 0.0;

        let assign22150_e17092: f64 = (locals.var_t4 + 1e-25);
        locals.var_t4 = assign22150_e17092;
        locals.var_t4_dn0 = locals.var_t4_dn0;
        locals.var_t4_dn2 = locals.var_t4_dn2;
        locals.var_t4_dn4 = locals.var_t4_dn4;
        locals.var_t4_dn5 = locals.var_t4_dn5;
        locals.var_t4_dn6 = locals.var_t4_dn6;
        locals.var_t4_dn7 = locals.var_t4_dn7;
        locals.var_t4_dn8 = locals.var_t4_dn8;
        locals.var_t4_dn9 = locals.var_t4_dn9;
        locals.var_t4_dn10 = locals.var_t4_dn10;
        locals.var_t4_dn11 = locals.var_t4_dn11;
        locals.var_t4_dn14 = locals.var_t4_dn14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let assign22160_e17094: f64 = (locals.var_t4).sqrt();
        locals.var_tx = assign22160_e17094;
        locals.var_tx_dn0 = (locals.var_t4_dn0 / (2.0 * assign22160_e17094));
        locals.var_tx_dn2 = (locals.var_t4_dn2 / (2.0 * assign22160_e17094));
        locals.var_tx_dn4 = (locals.var_t4_dn4 / (2.0 * assign22160_e17094));
        locals.var_tx_dn5 = (locals.var_t4_dn5 / (2.0 * assign22160_e17094));
        locals.var_tx_dn6 = (locals.var_t4_dn6 / (2.0 * assign22160_e17094));
        locals.var_tx_dn7 = (locals.var_t4_dn7 / (2.0 * assign22160_e17094));
        locals.var_tx_dn8 = (locals.var_t4_dn8 / (2.0 * assign22160_e17094));
        locals.var_tx_dn9 = (locals.var_t4_dn9 / (2.0 * assign22160_e17094));
        locals.var_tx_dn10 = (locals.var_t4_dn10 / (2.0 * assign22160_e17094));
        locals.var_tx_dn11 = (locals.var_t4_dn11 / (2.0 * assign22160_e17094));
        locals.var_tx_dn14 = (locals.var_t4_dn14 / (2.0 * assign22160_e17094));
        locals.var_tx_rv = 0.0;

        let assign22170_e17099: f64 = (1.0 - locals.var_tx);
        let assign22170_e17100: f64 = (locals.var_t1 * assign22170_e17099);
        let assign22170_e17101: f64 = (locals.var_t2 + assign22170_e17100);
        locals.var_pslsat = assign22170_e17101;
        locals.var_pslsat_dn0 = (locals.var_t2_dn0 + ((locals.var_t1_dn0 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn0))));
        locals.var_pslsat_dn2 = (locals.var_t2_dn2 + ((locals.var_t1_dn2 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn2))));
        locals.var_pslsat_dn4 = (locals.var_t2_dn4 + ((locals.var_t1_dn4 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn4))));
        locals.var_pslsat_dn5 = (locals.var_t2_dn5 + ((locals.var_t1_dn5 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn5))));
        locals.var_pslsat_dn6 = (locals.var_t2_dn6 + ((locals.var_t1_dn6 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2_dn7 + ((locals.var_t1_dn7 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn7))));
        locals.var_pslsat_dn8 = (locals.var_t2_dn8 + ((locals.var_t1_dn8 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn8))));
        locals.var_pslsat_dn9 = (locals.var_t2_dn9 + ((locals.var_t1_dn9 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn9))));
        locals.var_pslsat_dn10 = (locals.var_t2_dn10 + ((locals.var_t1_dn10 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn10))));
        locals.var_pslsat_dn11 = (locals.var_t2_dn11 + ((locals.var_t1_dn11 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn11))));
        locals.var_pslsat_dn14 = (locals.var_t2_dn14 + ((locals.var_t1_dn14 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn14))));
        locals.var_pslsat_rv = 0.0;

        let assign22180_e17104: f64 = (locals.var_pslsat - locals.var_pb2c);
        locals.var_vdsats = assign22180_e17104;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2c_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2c_dn2);
        locals.var_vdsats_dn4 = (locals.var_pslsat_dn4 - locals.var_pb2c_dn4);
        locals.var_vdsats_dn5 = (locals.var_pslsat_dn5 - locals.var_pb2c_dn5);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2c_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2c_dn7);
        locals.var_vdsats_dn8 = (locals.var_pslsat_dn8 - locals.var_pb2c_dn8);
        locals.var_vdsats_dn9 = (locals.var_pslsat_dn9 - locals.var_pb2c_dn9);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2c_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2c_dn11);
        locals.var_vdsats_dn14 = (locals.var_pslsat_dn14 - locals.var_pb2c_dn14);
        locals.var_vdsats_rv = 0.0;

        let assign22190_e17107: f64 = (locals.var_vdsats - 0.1);
        let assign22190_e17109: f64 = (assign22190_e17107 - 0.05);
        locals.var_tmf1 = assign22190_e17109;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn4 = locals.var_vdsats_dn4;
        locals.var_tmf1_dn5 = locals.var_vdsats_dn5;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn8 = locals.var_vdsats_dn8;
        locals.var_tmf1_dn9 = locals.var_vdsats_dn9;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn14 = locals.var_vdsats_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign22200_e17112: f64 = (4.0 * 0.1);
        let assign22200_e17114: f64 = (assign22200_e17112 * 0.05);
        locals.var_tmf2 = assign22200_e17114;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign22210_e17121, assign22210_e17121_d_n0, assign22210_e17121_d_n2, assign22210_e17121_d_n4, assign22210_e17121_d_n5, assign22210_e17121_d_n6, assign22210_e17121_d_n7, assign22210_e17121_d_n8, assign22210_e17121_d_n9, assign22210_e17121_d_n10, assign22210_e17121_d_n11, assign22210_e17121_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign22210_e17120: f64 = (-locals.var_tmf2);
        (assign22210_e17120, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign22210_e17121;
        locals.var_tmf2_dn0 = assign22210_e17121_d_n0;
        locals.var_tmf2_dn2 = assign22210_e17121_d_n2;
        locals.var_tmf2_dn4 = assign22210_e17121_d_n4;
        locals.var_tmf2_dn5 = assign22210_e17121_d_n5;
        locals.var_tmf2_dn6 = assign22210_e17121_d_n6;
        locals.var_tmf2_dn7 = assign22210_e17121_d_n7;
        locals.var_tmf2_dn8 = assign22210_e17121_d_n8;
        locals.var_tmf2_dn9 = assign22210_e17121_d_n9;
        locals.var_tmf2_dn10 = assign22210_e17121_d_n10;
        locals.var_tmf2_dn11 = assign22210_e17121_d_n11;
        locals.var_tmf2_dn14 = assign22210_e17121_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign22220_e17124: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22220_e17126: f64 = (assign22220_e17124 + locals.var_tmf2);
        let assign22220_e17127: f64 = (assign22220_e17126).sqrt();
        locals.var_tmf2 = assign22220_e17127;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22220_e17127));
        locals.var_tmf2_rv = 0.0;

        let assign22230_e17132: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22230_e17133: f64 = (1.0 + assign22230_e17132);
        let assign22230_e17134: f64 = (0.5 * assign22230_e17133);
        locals.var_t6 = assign22230_e17134;
        locals.var_t6_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_rv = 0.0;

        let assign22240_e17139: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22240_e17140: f64 = (0.5 * assign22240_e17139);
        let assign22240_e17141: f64 = (0.1 + assign22240_e17140);
        locals.var_vdsats = assign22240_e17141;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_vdsats_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_vdsats_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));
        locals.var_vdsats_rv = 0.0;

        let assign22250_e17144: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1 = assign22250_e17144;
        locals.var_t1_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn4 = (((locals.var_vds_dn4 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn4)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn5 = (((locals.var_vds_dn5 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn5)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn8 = (((locals.var_vds_dn8 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn8)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn9 = (((locals.var_vds_dn9 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn9)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn14 = (((locals.var_vds_dn14 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn14)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_rv = 0.0;

        let assign22260_e17147: f64 = locals.var_t1;
        locals.var_tmf1 = assign22260_e17147;
        locals.var_tmf1_dn0 = locals.var_t1_dn0;
        locals.var_tmf1_dn2 = locals.var_t1_dn2;
        locals.var_tmf1_dn4 = locals.var_t1_dn4;
        locals.var_tmf1_dn5 = locals.var_t1_dn5;
        locals.var_tmf1_dn6 = locals.var_t1_dn6;
        locals.var_tmf1_dn7 = locals.var_t1_dn7;
        locals.var_tmf1_dn8 = locals.var_t1_dn8;
        locals.var_tmf1_dn9 = locals.var_t1_dn9;
        locals.var_tmf1_dn10 = locals.var_t1_dn10;
        locals.var_tmf1_dn11 = locals.var_t1_dn11;
        locals.var_tmf1_dn14 = locals.var_t1_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign22270_e17150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign22270_e17150;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14));
        locals.var_tmf2_rv = 0.0;

        let assign22280_e17153: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign22280_e17153;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4));
        locals.var_tmf3_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8));
        locals.var_tmf3_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14));
        locals.var_tmf3_rv = 0.0;

        let assign22290_e17156: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign22290_e17156;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4));
        locals.var_tmf4_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8));
        locals.var_tmf4_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14));
        locals.var_tmf4_rv = 0.0;

        let assign22300_e17160: f64 = (1.0 + locals.var_tmf1);
        let assign22300_e17162: f64 = (assign22300_e17160 + locals.var_tmf2);
        let assign22300_e17164: f64 = (assign22300_e17162 + locals.var_tmf3);
        let assign22300_e17166: f64 = (assign22300_e17164 + locals.var_tmf4);
        let assign22300_e17167: f64 = (1.0 / assign22300_e17166);
        locals.var_tx = assign22300_e17167;
        locals.var_tx_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn4 = (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn5 = (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn8 = (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn9 = (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn14 = (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_rv = 0.0;

        let assign22310_e17171: f64 = (2.0 * locals.var_tmf1);
        let assign22310_e17172: f64 = (1.0 + assign22310_e17171);
        let assign22310_e17175: f64 = (3.0 * locals.var_tmf2);
        let assign22310_e17176: f64 = (assign22310_e17172 + assign22310_e17175);
        let assign22310_e17179: f64 = (4.0 * locals.var_tmf3);
        let assign22310_e17180: f64 = (assign22310_e17176 + assign22310_e17179);
        let assign22310_e17181: f64 = (-assign22310_e17180);
        let assign22310_e17183: f64 = (assign22310_e17181 * locals.var_tx);
        let assign22310_e17185: f64 = (assign22310_e17183 * locals.var_tx);
        locals.var_t0 = assign22310_e17185;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn0)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn2)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn2));
        locals.var_t0_dn4 = (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn4)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn4));
        locals.var_t0_dn5 = (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn5)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn5));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn6)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn7)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn7));
        locals.var_t0_dn8 = (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn8)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn8));
        locals.var_t0_dn9 = (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn9)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn9));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn10)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn11)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn11));
        locals.var_t0_dn14 = (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn14)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn14));
        locals.var_t0_rv = 0.0;

        let assign22320_e17189: f64 = (1.0 - locals.var_tx);
        let assign22320_e17190: f64 = assign22320_e17189;
        locals.var_tx = assign22320_e17190;
        locals.var_tx_dn0 = (-locals.var_tx_dn0);
        locals.var_tx_dn2 = (-locals.var_tx_dn2);
        locals.var_tx_dn4 = (-locals.var_tx_dn4);
        locals.var_tx_dn5 = (-locals.var_tx_dn5);
        locals.var_tx_dn6 = (-locals.var_tx_dn6);
        locals.var_tx_dn7 = (-locals.var_tx_dn7);
        locals.var_tx_dn8 = (-locals.var_tx_dn8);
        locals.var_tx_dn9 = (-locals.var_tx_dn9);
        locals.var_tx_dn10 = (-locals.var_tx_dn10);
        locals.var_tx_dn11 = (-locals.var_tx_dn11);
        locals.var_tx_dn14 = (-locals.var_tx_dn14);
        locals.var_tx_rv = 0.0;

        let assign22330_e17192: f64 = (-locals.var_t0);
        locals.var_t0 = assign22330_e17192;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn4 = (-locals.var_t0_dn4);
        locals.var_t0_dn5 = (-locals.var_t0_dn5);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn8 = (-locals.var_t0_dn8);
        locals.var_t0_dn9 = (-locals.var_t0_dn9);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn14 = (-locals.var_t0_dn14);
        locals.var_t0_rv = 0.0;

        let assign22340_e17195: f64 = (locals.var_tx * locals.var_tx);
        locals.var_fmdvds = assign22340_e17195;
        locals.var_fmdvds_dn0 = ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2));
        locals.var_fmdvds_dn4 = ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4));
        locals.var_fmdvds_dn5 = ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5));
        locals.var_fmdvds_dn6 = ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7));
        locals.var_fmdvds_dn8 = ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8));
        locals.var_fmdvds_dn9 = ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9));
        locals.var_fmdvds_dn10 = ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11));
        locals.var_fmdvds_dn14 = ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14));
        locals.var_fmdvds_rv = 0.0;

        let assign22350_e17198: f64 = if locals.var_flg_qmetemp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign22350_e17198;
        locals.var_guard427_rv = 0.0;

        let (assign22360_e17202,) = {
    if (locals.var_guard427 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22360_e17202;
        locals.var_flg_qme_rv = 0.0;

        let (assign22370_e17207,) = {
    if (locals.var_guard427 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22370_e17207;
        locals.var_flg_qme_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;
        locals.var_t1_rv = 0.0;

        let assign22390_e17211: f64 = (locals.var_t1 * locals.var_pb20);
        let assign22390_e17212: f64 = (assign22390_e17211).sqrt();
        locals.var_t2 = assign22390_e17212;
        locals.var_t2_dn0 = (((locals.var_t1_dn0 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn0)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn2 = (((locals.var_t1_dn2 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn2)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn4 = (((locals.var_t1_dn4 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn4)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn5 = (((locals.var_t1_dn5 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn5)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn6 = (((locals.var_t1_dn6 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn6)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn7 = (((locals.var_t1_dn7 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn7)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn8 = (((locals.var_t1_dn8 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn8)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn9 = (((locals.var_t1_dn9 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn9)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn10 = (((locals.var_t1_dn10 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn10)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn11 = (((locals.var_t1_dn11 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn11)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn14 = (((locals.var_t1_dn14 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn14)) / (2.0 * assign22390_e17212));
        locals.var_t2_rv = 0.0;

        let assign22400_e17215: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22400_e17218: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign22400_e17219: f64 = (assign22400_e17215 + assign22400_e17218);
        locals.var_vthq = assign22400_e17219;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv));
        locals.var_vthq_dn4 = (locals.var_pb20_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv));
        locals.var_vthq_dn5 = (locals.var_pb20_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv));
        locals.var_vthq_dn8 = (locals.var_pb20_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv));
        locals.var_vthq_dn9 = (locals.var_pb20_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv));
        locals.var_vthq_dn11 = (locals.var_pb20_dn11 + (locals.var_t2_dn11 * locals.var_cox0_inv));
        locals.var_vthq_dn14 = (locals.var_pb20_dn14 + (locals.var_t2_dn14 * locals.var_cox0_inv));
        locals.var_vthq_rv = 0.0;

        let assign22410_e17222: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign22410_e17222;
        locals.var_guard428_rv = 0.0;

        let (assign22420_e17226, assign22420_e17226_d_n0, assign22420_e17226_d_n2, assign22420_e17226_d_n4, assign22420_e17226_d_n5, assign22420_e17226_d_n6, assign22420_e17226_d_n7, assign22420_e17226_d_n8, assign22420_e17226_d_n9, assign22420_e17226_d_n10, assign22420_e17226_d_n11, assign22420_e17226_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_tox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22420_e17226;
        locals.var_toxe_dn0 = assign22420_e17226_d_n0;
        locals.var_toxe_dn2 = assign22420_e17226_d_n2;
        locals.var_toxe_dn4 = assign22420_e17226_d_n4;
        locals.var_toxe_dn5 = assign22420_e17226_d_n5;
        locals.var_toxe_dn6 = assign22420_e17226_d_n6;
        locals.var_toxe_dn7 = assign22420_e17226_d_n7;
        locals.var_toxe_dn8 = assign22420_e17226_d_n8;
        locals.var_toxe_dn9 = assign22420_e17226_d_n9;
        locals.var_toxe_dn10 = assign22420_e17226_d_n10;
        locals.var_toxe_dn11 = assign22420_e17226_d_n11;
        locals.var_toxe_dn14 = assign22420_e17226_d_n14;
        locals.var_toxe_rv = 0.0;

        let (assign22430_e17230, assign22430_e17230_d_n0, assign22430_e17230_d_n2, assign22430_e17230_d_n4, assign22430_e17230_d_n5, assign22430_e17230_d_n6, assign22430_e17230_d_n7, assign22430_e17230_d_n8, assign22430_e17230_d_n9, assign22430_e17230_d_n10, assign22430_e17230_d_n11, assign22430_e17230_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_cox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22430_e17230;
        locals.var_cox_dn0 = assign22430_e17230_d_n0;
        locals.var_cox_dn2 = assign22430_e17230_d_n2;
        locals.var_cox_dn4 = assign22430_e17230_d_n4;
        locals.var_cox_dn5 = assign22430_e17230_d_n5;
        locals.var_cox_dn6 = assign22430_e17230_d_n6;
        locals.var_cox_dn7 = assign22430_e17230_d_n7;
        locals.var_cox_dn8 = assign22430_e17230_d_n8;
        locals.var_cox_dn9 = assign22430_e17230_d_n9;
        locals.var_cox_dn10 = assign22430_e17230_d_n10;
        locals.var_cox_dn11 = assign22430_e17230_d_n11;
        locals.var_cox_dn14 = assign22430_e17230_d_n14;
        locals.var_cox_rv = 0.0;

        let (assign22440_e17234, assign22440_e17234_d_n0, assign22440_e17234_d_n2, assign22440_e17234_d_n4, assign22440_e17234_d_n5, assign22440_e17234_d_n6, assign22440_e17234_d_n7, assign22440_e17234_d_n8, assign22440_e17234_d_n9, assign22440_e17234_d_n10, assign22440_e17234_d_n11, assign22440_e17234_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_cox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22440_e17234;
        locals.var_cox_inv_dn0 = assign22440_e17234_d_n0;
        locals.var_cox_inv_dn2 = assign22440_e17234_d_n2;
        locals.var_cox_inv_dn4 = assign22440_e17234_d_n4;
        locals.var_cox_inv_dn5 = assign22440_e17234_d_n5;
        locals.var_cox_inv_dn6 = assign22440_e17234_d_n6;
        locals.var_cox_inv_dn7 = assign22440_e17234_d_n7;
        locals.var_cox_inv_dn8 = assign22440_e17234_d_n8;
        locals.var_cox_inv_dn9 = assign22440_e17234_d_n9;
        locals.var_cox_inv_dn10 = assign22440_e17234_d_n10;
        locals.var_cox_inv_dn11 = assign22440_e17234_d_n11;
        locals.var_cox_inv_dn14 = assign22440_e17234_d_n14;
        locals.var_cox_inv_rv = 0.0;

        let (assign22450_e17242, assign22450_e17242_d_n0, assign22450_e17242_d_n2, assign22450_e17242_d_n4, assign22450_e17242_d_n5, assign22450_e17242_d_n6, assign22450_e17242_d_n7, assign22450_e17242_d_n8, assign22450_e17242_d_n9, assign22450_e17242_d_n10, assign22450_e17242_d_n11, assign22450_e17242_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        let assign22450_e17238: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22450_e17240: f64 = (assign22450_e17238 * locals.var_cox_inv);
        (assign22450_e17240, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22450_e17242;
        locals.var_t0_dn0 = assign22450_e17242_d_n0;
        locals.var_t0_dn2 = assign22450_e17242_d_n2;
        locals.var_t0_dn4 = assign22450_e17242_d_n4;
        locals.var_t0_dn5 = assign22450_e17242_d_n5;
        locals.var_t0_dn6 = assign22450_e17242_d_n6;
        locals.var_t0_dn7 = assign22450_e17242_d_n7;
        locals.var_t0_dn8 = assign22450_e17242_d_n8;
        locals.var_t0_dn9 = assign22450_e17242_d_n9;
        locals.var_t0_dn10 = assign22450_e17242_d_n10;
        locals.var_t0_dn11 = assign22450_e17242_d_n11;
        locals.var_t0_dn14 = assign22450_e17242_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign22460_e17248, assign22460_e17248_d_n0, assign22460_e17248_d_n2, assign22460_e17248_d_n4, assign22460_e17248_d_n5, assign22460_e17248_d_n6, assign22460_e17248_d_n7, assign22460_e17248_d_n8, assign22460_e17248_d_n9, assign22460_e17248_d_n10, assign22460_e17248_d_n11, assign22460_e17248_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        let assign22460_e17246: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22460_e17246, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22460_e17248;
        locals.var_cnstcoxi_dn0 = assign22460_e17248_d_n0;
        locals.var_cnstcoxi_dn2 = assign22460_e17248_d_n2;
        locals.var_cnstcoxi_dn4 = assign22460_e17248_d_n4;
        locals.var_cnstcoxi_dn5 = assign22460_e17248_d_n5;
        locals.var_cnstcoxi_dn6 = assign22460_e17248_d_n6;
        locals.var_cnstcoxi_dn7 = assign22460_e17248_d_n7;
        locals.var_cnstcoxi_dn8 = assign22460_e17248_d_n8;
        locals.var_cnstcoxi_dn9 = assign22460_e17248_d_n9;
        locals.var_cnstcoxi_dn10 = assign22460_e17248_d_n10;
        locals.var_cnstcoxi_dn11 = assign22460_e17248_d_n11;
        locals.var_cnstcoxi_dn14 = assign22460_e17248_d_n14;
        locals.var_cnstcoxi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22470_e17259, assign22470_e17259_d_n0, assign22470_e17259_d_n2, assign22470_e17259_d_n4, assign22470_e17259_d_n5, assign22470_e17259_d_n6, assign22470_e17259_d_n7, assign22470_e17259_d_n8, assign22470_e17259_d_n9, assign22470_e17259_d_n10, assign22470_e17259_d_n11, assign22470_e17259_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22470_e17253: f64 = (locals.var_vgs - locals.var_vbs);
        let assign22470_e17255: f64 = (assign22470_e17253 - locals.var_vthq);
        let assign22470_e17257: f64 = (assign22470_e17255 + p.p236);
        (assign22470_e17257, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn4), (-locals.var_vthq_dn5), ((locals.var_vgs_dn6 - locals.var_vbs_dn6) - locals.var_vthq_dn6), (locals.var_vgs_dn7 - locals.var_vthq_dn7), ((locals.var_vgs_dn8 - locals.var_vbs_dn8) - locals.var_vthq_dn8), ((-locals.var_vbs_dn9) - locals.var_vthq_dn9), (-locals.var_vthq_dn10), (-locals.var_vthq_dn11), (-locals.var_vthq_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22470_e17259;
        locals.var_t5_dn0 = assign22470_e17259_d_n0;
        locals.var_t5_dn2 = assign22470_e17259_d_n2;
        locals.var_t5_dn4 = assign22470_e17259_d_n4;
        locals.var_t5_dn5 = assign22470_e17259_d_n5;
        locals.var_t5_dn6 = assign22470_e17259_d_n6;
        locals.var_t5_dn7 = assign22470_e17259_d_n7;
        locals.var_t5_dn8 = assign22470_e17259_d_n8;
        locals.var_t5_dn9 = assign22470_e17259_d_n9;
        locals.var_t5_dn10 = assign22470_e17259_d_n10;
        locals.var_t5_dn11 = assign22470_e17259_d_n11;
        locals.var_t5_dn14 = assign22470_e17259_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign22480_e17277, assign22480_e17277_d_n0, assign22480_e17277_d_n2, assign22480_e17277_d_n4, assign22480_e17277_d_n5, assign22480_e17277_d_n6, assign22480_e17277_d_n7, assign22480_e17277_d_n8, assign22480_e17277_d_n9, assign22480_e17277_d_n10, assign22480_e17277_d_n11, assign22480_e17277_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22480_e17264: f64 = (locals.var_t5 * locals.var_t5);
        let assign22480_e17268: f64 = (1e-9 * 0.01);
        let assign22480_e17269: f64 = (4.0 * assign22480_e17268);
        let assign22480_e17272: f64 = (1e-9 * 0.01);
        let assign22480_e17273: f64 = (assign22480_e17269 * assign22480_e17272);
        let assign22480_e17274: f64 = (assign22480_e17264 + assign22480_e17273);
        let assign22480_e17275: f64 = (assign22480_e17274).sqrt();
        (assign22480_e17275, (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign22480_e17275)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22480_e17277;
        locals.var_tmf2_dn0 = assign22480_e17277_d_n0;
        locals.var_tmf2_dn2 = assign22480_e17277_d_n2;
        locals.var_tmf2_dn4 = assign22480_e17277_d_n4;
        locals.var_tmf2_dn5 = assign22480_e17277_d_n5;
        locals.var_tmf2_dn6 = assign22480_e17277_d_n6;
        locals.var_tmf2_dn7 = assign22480_e17277_d_n7;
        locals.var_tmf2_dn8 = assign22480_e17277_d_n8;
        locals.var_tmf2_dn9 = assign22480_e17277_d_n9;
        locals.var_tmf2_dn10 = assign22480_e17277_d_n10;
        locals.var_tmf2_dn11 = assign22480_e17277_d_n11;
        locals.var_tmf2_dn14 = assign22480_e17277_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22490_e17288, assign22490_e17288_d_n0, assign22490_e17288_d_n2, assign22490_e17288_d_n4, assign22490_e17288_d_n5, assign22490_e17288_d_n6, assign22490_e17288_d_n7, assign22490_e17288_d_n8, assign22490_e17288_d_n9, assign22490_e17288_d_n10, assign22490_e17288_d_n11, assign22490_e17288_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22490_e17284: f64 = (locals.var_t5 / locals.var_tmf2);
        let assign22490_e17285: f64 = (1.0 + assign22490_e17284);
        let assign22490_e17286: f64 = (0.5 * assign22490_e17285);
        (assign22490_e17286, (0.5 * (((locals.var_t5_dn0 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn2 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn4 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn5 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn6 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn7 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn8 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn9 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn10 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn11 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn14 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22490_e17288;
        locals.var_t3_dn0 = assign22490_e17288_d_n0;
        locals.var_t3_dn2 = assign22490_e17288_d_n2;
        locals.var_t3_dn4 = assign22490_e17288_d_n4;
        locals.var_t3_dn5 = assign22490_e17288_d_n5;
        locals.var_t3_dn6 = assign22490_e17288_d_n6;
        locals.var_t3_dn7 = assign22490_e17288_d_n7;
        locals.var_t3_dn8 = assign22490_e17288_d_n8;
        locals.var_t3_dn9 = assign22490_e17288_d_n9;
        locals.var_t3_dn10 = assign22490_e17288_d_n10;
        locals.var_t3_dn11 = assign22490_e17288_d_n11;
        locals.var_t3_dn14 = assign22490_e17288_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22500_e17297, assign22500_e17297_d_n0, assign22500_e17297_d_n2, assign22500_e17297_d_n4, assign22500_e17297_d_n5, assign22500_e17297_d_n6, assign22500_e17297_d_n7, assign22500_e17297_d_n8, assign22500_e17297_d_n9, assign22500_e17297_d_n10, assign22500_e17297_d_n11, assign22500_e17297_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22500_e17294: f64 = (locals.var_t5 + locals.var_tmf2);
        let assign22500_e17295: f64 = (0.5 * assign22500_e17294);
        (assign22500_e17295, (0.5 * (locals.var_t5_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t5_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t5_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t5_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t5_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t5_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t5_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t5_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t5_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t5_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t5_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22500_e17297;
        locals.var_t2_dn0 = assign22500_e17297_d_n0;
        locals.var_t2_dn2 = assign22500_e17297_d_n2;
        locals.var_t2_dn4 = assign22500_e17297_d_n4;
        locals.var_t2_dn5 = assign22500_e17297_d_n5;
        locals.var_t2_dn6 = assign22500_e17297_d_n6;
        locals.var_t2_dn7 = assign22500_e17297_d_n7;
        locals.var_t2_dn8 = assign22500_e17297_d_n8;
        locals.var_t2_dn9 = assign22500_e17297_d_n9;
        locals.var_t2_dn10 = assign22500_e17297_d_n10;
        locals.var_t2_dn11 = assign22500_e17297_d_n11;
        locals.var_t2_dn14 = assign22500_e17297_d_n14;
        locals.var_t2_rv = 0.0;

        let assign22510_e17300: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign22510_e17300;
        locals.var_guard429_rv = 0.0;

        let (assign22520_e17307, assign22520_e17307_d_n0, assign22520_e17307_d_n2, assign22520_e17307_d_n4, assign22520_e17307_d_n5, assign22520_e17307_d_n6, assign22520_e17307_d_n7, assign22520_e17307_d_n8, assign22520_e17307_d_n9, assign22520_e17307_d_n10, assign22520_e17307_d_n11, assign22520_e17307_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22520_e17307;
        locals.var_t2_dn0 = assign22520_e17307_d_n0;
        locals.var_t2_dn2 = assign22520_e17307_d_n2;
        locals.var_t2_dn4 = assign22520_e17307_d_n4;
        locals.var_t2_dn5 = assign22520_e17307_d_n5;
        locals.var_t2_dn6 = assign22520_e17307_d_n6;
        locals.var_t2_dn7 = assign22520_e17307_d_n7;
        locals.var_t2_dn8 = assign22520_e17307_d_n8;
        locals.var_t2_dn9 = assign22520_e17307_d_n9;
        locals.var_t2_dn10 = assign22520_e17307_d_n10;
        locals.var_t2_dn11 = assign22520_e17307_d_n11;
        locals.var_t2_dn14 = assign22520_e17307_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22530_e17314, assign22530_e17314_d_n0, assign22530_e17314_d_n2, assign22530_e17314_d_n4, assign22530_e17314_d_n5, assign22530_e17314_d_n6, assign22530_e17314_d_n7, assign22530_e17314_d_n8, assign22530_e17314_d_n9, assign22530_e17314_d_n10, assign22530_e17314_d_n11, assign22530_e17314_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22530_e17314;
        locals.var_t3_dn0 = assign22530_e17314_d_n0;
        locals.var_t3_dn2 = assign22530_e17314_d_n2;
        locals.var_t3_dn4 = assign22530_e17314_d_n4;
        locals.var_t3_dn5 = assign22530_e17314_d_n5;
        locals.var_t3_dn6 = assign22530_e17314_d_n6;
        locals.var_t3_dn7 = assign22530_e17314_d_n7;
        locals.var_t3_dn8 = assign22530_e17314_d_n8;
        locals.var_t3_dn9 = assign22530_e17314_d_n9;
        locals.var_t3_dn10 = assign22530_e17314_d_n10;
        locals.var_t3_dn11 = assign22530_e17314_d_n11;
        locals.var_t3_dn14 = assign22530_e17314_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22540_e17321, assign22540_e17321_d_n0, assign22540_e17321_d_n2, assign22540_e17321_d_n4, assign22540_e17321_d_n5, assign22540_e17321_d_n6, assign22540_e17321_d_n7, assign22540_e17321_d_n8, assign22540_e17321_d_n9, assign22540_e17321_d_n10, assign22540_e17321_d_n11, assign22540_e17321_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22540_e17319: f64 = (locals.var_t2 + 1e-25);
        (assign22540_e17319, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22540_e17321;
        locals.var_t2_dn0 = assign22540_e17321_d_n0;
        locals.var_t2_dn2 = assign22540_e17321_d_n2;
        locals.var_t2_dn4 = assign22540_e17321_d_n4;
        locals.var_t2_dn5 = assign22540_e17321_d_n5;
        locals.var_t2_dn6 = assign22540_e17321_d_n6;
        locals.var_t2_dn7 = assign22540_e17321_d_n7;
        locals.var_t2_dn8 = assign22540_e17321_d_n8;
        locals.var_t2_dn9 = assign22540_e17321_d_n9;
        locals.var_t2_dn10 = assign22540_e17321_d_n10;
        locals.var_t2_dn11 = assign22540_e17321_d_n11;
        locals.var_t2_dn14 = assign22540_e17321_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22550_e17328, assign22550_e17328_d_n0, assign22550_e17328_d_n2, assign22550_e17328_d_n4, assign22550_e17328_d_n5, assign22550_e17328_d_n6, assign22550_e17328_d_n7, assign22550_e17328_d_n8, assign22550_e17328_d_n9, assign22550_e17328_d_n10, assign22550_e17328_d_n11, assign22550_e17328_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22550_e17326: f64 = (1.0 / locals.var_t2);
        (assign22550_e17326, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22550_e17328;
        locals.var_t3_dn0 = assign22550_e17328_d_n0;
        locals.var_t3_dn2 = assign22550_e17328_d_n2;
        locals.var_t3_dn4 = assign22550_e17328_d_n4;
        locals.var_t3_dn5 = assign22550_e17328_d_n5;
        locals.var_t3_dn6 = assign22550_e17328_d_n6;
        locals.var_t3_dn7 = assign22550_e17328_d_n7;
        locals.var_t3_dn8 = assign22550_e17328_d_n8;
        locals.var_t3_dn9 = assign22550_e17328_d_n9;
        locals.var_t3_dn10 = assign22550_e17328_d_n10;
        locals.var_t3_dn11 = assign22550_e17328_d_n11;
        locals.var_t3_dn14 = assign22550_e17328_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22560_e17338, assign22560_e17338_d_n0, assign22560_e17338_d_n2, assign22560_e17338_d_n4, assign22560_e17338_d_n5, assign22560_e17338_d_n6, assign22560_e17338_d_n7, assign22560_e17338_d_n8, assign22560_e17338_d_n9, assign22560_e17338_d_n10, assign22560_e17338_d_n11, assign22560_e17338_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22560_e17332: f64 = (-1.0);
        let assign22560_e17335: f64 = (locals.var_t2 * locals.var_t2);
        let assign22560_e17336: f64 = (assign22560_e17332 / assign22560_e17335);
        (assign22560_e17336, (-((assign22560_e17332 * ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (assign22560_e17335 * assign22560_e17335))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22560_e17338;
        locals.var_t7_dn0 = assign22560_e17338_d_n0;
        locals.var_t7_dn2 = assign22560_e17338_d_n2;
        locals.var_t7_dn4 = assign22560_e17338_d_n4;
        locals.var_t7_dn5 = assign22560_e17338_d_n5;
        locals.var_t7_dn6 = assign22560_e17338_d_n6;
        locals.var_t7_dn7 = assign22560_e17338_d_n7;
        locals.var_t7_dn8 = assign22560_e17338_d_n8;
        locals.var_t7_dn9 = assign22560_e17338_d_n9;
        locals.var_t7_dn10 = assign22560_e17338_d_n10;
        locals.var_t7_dn11 = assign22560_e17338_d_n11;
        locals.var_t7_dn14 = assign22560_e17338_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign22570_e17346, assign22570_e17346_d_n0, assign22570_e17346_d_n2, assign22570_e17346_d_n4, assign22570_e17346_d_n5, assign22570_e17346_d_n6, assign22570_e17346_d_n7, assign22570_e17346_d_n8, assign22570_e17346_d_n9, assign22570_e17346_d_n10, assign22570_e17346_d_n11, assign22570_e17346_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22570_e17343: f64 = (locals.var_vthq).abs();
        let assign22570_e17344: f64 = (2.0 * assign22570_e17343);
        (assign22570_e17344, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn4 } else { (-locals.var_vthq_dn4) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn5 } else { (-locals.var_vthq_dn5) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn8 } else { (-locals.var_vthq_dn8) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn9 } else { (-locals.var_vthq_dn9) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn14 } else { (-locals.var_vthq_dn14) }),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22570_e17346;
        locals.var_t4_dn0 = assign22570_e17346_d_n0;
        locals.var_t4_dn2 = assign22570_e17346_d_n2;
        locals.var_t4_dn4 = assign22570_e17346_d_n4;
        locals.var_t4_dn5 = assign22570_e17346_d_n5;
        locals.var_t4_dn6 = assign22570_e17346_d_n6;
        locals.var_t4_dn7 = assign22570_e17346_d_n7;
        locals.var_t4_dn8 = assign22570_e17346_d_n8;
        locals.var_t4_dn9 = assign22570_e17346_d_n9;
        locals.var_t4_dn10 = assign22570_e17346_d_n10;
        locals.var_t4_dn11 = assign22570_e17346_d_n11;
        locals.var_t4_dn14 = assign22570_e17346_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22580_e17355, assign22580_e17355_d_n0, assign22580_e17355_d_n2, assign22580_e17355_d_n4, assign22580_e17355_d_n5, assign22580_e17355_d_n6, assign22580_e17355_d_n7, assign22580_e17355_d_n8, assign22580_e17355_d_n9, assign22580_e17355_d_n10, assign22580_e17355_d_n11, assign22580_e17355_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22580_e17351: f64 = (locals.var_t5 - locals.var_vgs);
        let assign22580_e17353: f64 = (assign22580_e17351 + locals.var_vfb);
        (assign22580_e17353, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, (locals.var_t5_dn6 - locals.var_vgs_dn6), (locals.var_t5_dn7 - locals.var_vgs_dn7), (locals.var_t5_dn8 - locals.var_vgs_dn8), locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22580_e17355;
        locals.var_t6_dn0 = assign22580_e17355_d_n0;
        locals.var_t6_dn2 = assign22580_e17355_d_n2;
        locals.var_t6_dn4 = assign22580_e17355_d_n4;
        locals.var_t6_dn5 = assign22580_e17355_d_n5;
        locals.var_t6_dn6 = assign22580_e17355_d_n6;
        locals.var_t6_dn7 = assign22580_e17355_d_n7;
        locals.var_t6_dn8 = assign22580_e17355_d_n8;
        locals.var_t6_dn9 = assign22580_e17355_d_n9;
        locals.var_t6_dn10 = assign22580_e17355_d_n10;
        locals.var_t6_dn11 = assign22580_e17355_d_n11;
        locals.var_t6_dn14 = assign22580_e17355_d_n14;
        locals.var_t6_rv = 0.0;

        let assign22590_e17358: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign22590_e17358;
        locals.var_guard430_rv = 0.0;

        let (assign22600_e17365, assign22600_e17365_d_n0, assign22600_e17365_d_n2, assign22600_e17365_d_n4, assign22600_e17365_d_n5, assign22600_e17365_d_n6, assign22600_e17365_d_n7, assign22600_e17365_d_n8, assign22600_e17365_d_n9, assign22600_e17365_d_n10, assign22600_e17365_d_n11, assign22600_e17365_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard430 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22600_e17365;
        locals.var_t4_dn0 = assign22600_e17365_d_n0;
        locals.var_t4_dn2 = assign22600_e17365_d_n2;
        locals.var_t4_dn4 = assign22600_e17365_d_n4;
        locals.var_t4_dn5 = assign22600_e17365_d_n5;
        locals.var_t4_dn6 = assign22600_e17365_d_n6;
        locals.var_t4_dn7 = assign22600_e17365_d_n7;
        locals.var_t4_dn8 = assign22600_e17365_d_n8;
        locals.var_t4_dn9 = assign22600_e17365_d_n9;
        locals.var_t4_dn10 = assign22600_e17365_d_n10;
        locals.var_t4_dn11 = assign22600_e17365_d_n11;
        locals.var_t4_dn14 = assign22600_e17365_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22610_e17378, assign22610_e17378_d_n0, assign22610_e17378_d_n2, assign22610_e17378_d_n4, assign22610_e17378_d_n5, assign22610_e17378_d_n6, assign22610_e17378_d_n7, assign22610_e17378_d_n8, assign22610_e17378_d_n9, assign22610_e17378_d_n10, assign22610_e17378_d_n11, assign22610_e17378_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22610_e17370: f64 = (1.0 / locals.var_t4);
        let assign22610_e17372: f64 = (assign22610_e17370 - locals.var_t3);
        let assign22610_e17375: f64 = (1e-9 * 0.01);
        let assign22610_e17376: f64 = (assign22610_e17372 - assign22610_e17375);
        (assign22610_e17376, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn0), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn2), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn4), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn5), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn6), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn7), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn8), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn9), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn10), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn11), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign22610_e17378;
        locals.var_tmf1_dn0 = assign22610_e17378_d_n0;
        locals.var_tmf1_dn2 = assign22610_e17378_d_n2;
        locals.var_tmf1_dn4 = assign22610_e17378_d_n4;
        locals.var_tmf1_dn5 = assign22610_e17378_d_n5;
        locals.var_tmf1_dn6 = assign22610_e17378_d_n6;
        locals.var_tmf1_dn7 = assign22610_e17378_d_n7;
        locals.var_tmf1_dn8 = assign22610_e17378_d_n8;
        locals.var_tmf1_dn9 = assign22610_e17378_d_n9;
        locals.var_tmf1_dn10 = assign22610_e17378_d_n10;
        locals.var_tmf1_dn11 = assign22610_e17378_d_n11;
        locals.var_tmf1_dn14 = assign22610_e17378_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign22620_e17391, assign22620_e17391_d_n0, assign22620_e17391_d_n2, assign22620_e17391_d_n4, assign22620_e17391_d_n5, assign22620_e17391_d_n6, assign22620_e17391_d_n7, assign22620_e17391_d_n8, assign22620_e17391_d_n9, assign22620_e17391_d_n10, assign22620_e17391_d_n11, assign22620_e17391_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22620_e17384: f64 = (1.0 / locals.var_t4);
        let assign22620_e17385: f64 = (4.0 * assign22620_e17384);
        let assign22620_e17388: f64 = (1e-9 * 0.01);
        let assign22620_e17389: f64 = (assign22620_e17385 * assign22620_e17388);
        (assign22620_e17389, ((4.0 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22620_e17391;
        locals.var_tmf2_dn0 = assign22620_e17391_d_n0;
        locals.var_tmf2_dn2 = assign22620_e17391_d_n2;
        locals.var_tmf2_dn4 = assign22620_e17391_d_n4;
        locals.var_tmf2_dn5 = assign22620_e17391_d_n5;
        locals.var_tmf2_dn6 = assign22620_e17391_d_n6;
        locals.var_tmf2_dn7 = assign22620_e17391_d_n7;
        locals.var_tmf2_dn8 = assign22620_e17391_d_n8;
        locals.var_tmf2_dn9 = assign22620_e17391_d_n9;
        locals.var_tmf2_dn10 = assign22620_e17391_d_n10;
        locals.var_tmf2_dn11 = assign22620_e17391_d_n11;
        locals.var_tmf2_dn14 = assign22620_e17391_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22630_e17402, assign22630_e17402_d_n0, assign22630_e17402_d_n2, assign22630_e17402_d_n4, assign22630_e17402_d_n5, assign22630_e17402_d_n6, assign22630_e17402_d_n7, assign22630_e17402_d_n8, assign22630_e17402_d_n9, assign22630_e17402_d_n10, assign22630_e17402_d_n11, assign22630_e17402_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let (assign22630_e17400, assign22630_e17400_d_n0, assign22630_e17400_d_n2, assign22630_e17400_d_n4, assign22630_e17400_d_n5, assign22630_e17400_d_n6, assign22630_e17400_d_n7, assign22630_e17400_d_n8, assign22630_e17400_d_n9, assign22630_e17400_d_n10, assign22630_e17400_d_n11, assign22630_e17400_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign22630_e17399: f64 = (-locals.var_tmf2);
                (assign22630_e17399, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign22630_e17400, assign22630_e17400_d_n0, assign22630_e17400_d_n2, assign22630_e17400_d_n4, assign22630_e17400_d_n5, assign22630_e17400_d_n6, assign22630_e17400_d_n7, assign22630_e17400_d_n8, assign22630_e17400_d_n9, assign22630_e17400_d_n10, assign22630_e17400_d_n11, assign22630_e17400_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22630_e17402;
        locals.var_tmf2_dn0 = assign22630_e17402_d_n0;
        locals.var_tmf2_dn2 = assign22630_e17402_d_n2;
        locals.var_tmf2_dn4 = assign22630_e17402_d_n4;
        locals.var_tmf2_dn5 = assign22630_e17402_d_n5;
        locals.var_tmf2_dn6 = assign22630_e17402_d_n6;
        locals.var_tmf2_dn7 = assign22630_e17402_d_n7;
        locals.var_tmf2_dn8 = assign22630_e17402_d_n8;
        locals.var_tmf2_dn9 = assign22630_e17402_d_n9;
        locals.var_tmf2_dn10 = assign22630_e17402_d_n10;
        locals.var_tmf2_dn11 = assign22630_e17402_d_n11;
        locals.var_tmf2_dn14 = assign22630_e17402_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22640_e17412, assign22640_e17412_d_n0, assign22640_e17412_d_n2, assign22640_e17412_d_n4, assign22640_e17412_d_n5, assign22640_e17412_d_n6, assign22640_e17412_d_n7, assign22640_e17412_d_n8, assign22640_e17412_d_n9, assign22640_e17412_d_n10, assign22640_e17412_d_n11, assign22640_e17412_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22640_e17407: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22640_e17409: f64 = (assign22640_e17407 + locals.var_tmf2);
        let assign22640_e17410: f64 = (assign22640_e17409).sqrt();
        (assign22640_e17410, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22640_e17410)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22640_e17412;
        locals.var_tmf2_dn0 = assign22640_e17412_d_n0;
        locals.var_tmf2_dn2 = assign22640_e17412_d_n2;
        locals.var_tmf2_dn4 = assign22640_e17412_d_n4;
        locals.var_tmf2_dn5 = assign22640_e17412_d_n5;
        locals.var_tmf2_dn6 = assign22640_e17412_d_n6;
        locals.var_tmf2_dn7 = assign22640_e17412_d_n7;
        locals.var_tmf2_dn8 = assign22640_e17412_d_n8;
        locals.var_tmf2_dn9 = assign22640_e17412_d_n9;
        locals.var_tmf2_dn10 = assign22640_e17412_d_n10;
        locals.var_tmf2_dn11 = assign22640_e17412_d_n11;
        locals.var_tmf2_dn14 = assign22640_e17412_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign22650_e17423, assign22650_e17423_d_n0, assign22650_e17423_d_n2, assign22650_e17423_d_n4, assign22650_e17423_d_n5, assign22650_e17423_d_n6, assign22650_e17423_d_n7, assign22650_e17423_d_n8, assign22650_e17423_d_n9, assign22650_e17423_d_n10, assign22650_e17423_d_n11, assign22650_e17423_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22650_e17419: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22650_e17420: f64 = (1.0 + assign22650_e17419);
        let assign22650_e17421: f64 = (0.5 * assign22650_e17420);
        (assign22650_e17421, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22650_e17423;
        locals.var_t6_dn0 = assign22650_e17423_d_n0;
        locals.var_t6_dn2 = assign22650_e17423_d_n2;
        locals.var_t6_dn4 = assign22650_e17423_d_n4;
        locals.var_t6_dn5 = assign22650_e17423_d_n5;
        locals.var_t6_dn6 = assign22650_e17423_d_n6;
        locals.var_t6_dn7 = assign22650_e17423_d_n7;
        locals.var_t6_dn8 = assign22650_e17423_d_n8;
        locals.var_t6_dn9 = assign22650_e17423_d_n9;
        locals.var_t6_dn10 = assign22650_e17423_d_n10;
        locals.var_t6_dn11 = assign22650_e17423_d_n11;
        locals.var_t6_dn14 = assign22650_e17423_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign22660_e17436, assign22660_e17436_d_n0, assign22660_e17436_d_n2, assign22660_e17436_d_n4, assign22660_e17436_d_n5, assign22660_e17436_d_n6, assign22660_e17436_d_n7, assign22660_e17436_d_n8, assign22660_e17436_d_n9, assign22660_e17436_d_n10, assign22660_e17436_d_n11, assign22660_e17436_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22660_e17428: f64 = (1.0 / locals.var_t4);
        let assign22660_e17432: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22660_e17433: f64 = (0.5 * assign22660_e17432);
        let assign22660_e17434: f64 = (assign22660_e17428 - assign22660_e17433);
        (assign22660_e17434, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22660_e17436;
        locals.var_t2_dn0 = assign22660_e17436_d_n0;
        locals.var_t2_dn2 = assign22660_e17436_d_n2;
        locals.var_t2_dn4 = assign22660_e17436_d_n4;
        locals.var_t2_dn5 = assign22660_e17436_d_n5;
        locals.var_t2_dn6 = assign22660_e17436_d_n6;
        locals.var_t2_dn7 = assign22660_e17436_d_n7;
        locals.var_t2_dn8 = assign22660_e17436_d_n8;
        locals.var_t2_dn9 = assign22660_e17436_d_n9;
        locals.var_t2_dn10 = assign22660_e17436_d_n10;
        locals.var_t2_dn11 = assign22660_e17436_d_n11;
        locals.var_t2_dn14 = assign22660_e17436_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22670_e17445, assign22670_e17445_d_n0, assign22670_e17445_d_n2, assign22670_e17445_d_n4, assign22670_e17445_d_n5, assign22670_e17445_d_n6, assign22670_e17445_d_n7, assign22670_e17445_d_n8, assign22670_e17445_d_n9, assign22670_e17445_d_n10, assign22670_e17445_d_n11, assign22670_e17445_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22670_e17441: f64 = (p.p235 * locals.var_t2);
        let assign22670_e17443: f64 = (assign22670_e17441 + p.p237);
        (assign22670_e17443, (p.p235 * locals.var_t2_dn0), (p.p235 * locals.var_t2_dn2), (p.p235 * locals.var_t2_dn4), (p.p235 * locals.var_t2_dn5), (p.p235 * locals.var_t2_dn6), (p.p235 * locals.var_t2_dn7), (p.p235 * locals.var_t2_dn8), (p.p235 * locals.var_t2_dn9), (p.p235 * locals.var_t2_dn10), (p.p235 * locals.var_t2_dn11), (p.p235 * locals.var_t2_dn14),)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22670_e17445;
        locals.var_dtox_dn0 = assign22670_e17445_d_n0;
        locals.var_dtox_dn2 = assign22670_e17445_d_n2;
        locals.var_dtox_dn4 = assign22670_e17445_d_n4;
        locals.var_dtox_dn5 = assign22670_e17445_d_n5;
        locals.var_dtox_dn6 = assign22670_e17445_d_n6;
        locals.var_dtox_dn7 = assign22670_e17445_d_n7;
        locals.var_dtox_dn8 = assign22670_e17445_d_n8;
        locals.var_dtox_dn9 = assign22670_e17445_d_n9;
        locals.var_dtox_dn10 = assign22670_e17445_d_n10;
        locals.var_dtox_dn11 = assign22670_e17445_d_n11;
        locals.var_dtox_dn14 = assign22670_e17445_d_n14;
        locals.var_dtox_rv = 0.0;

        let (assign22680_e17450, assign22680_e17450_d_n0, assign22680_e17450_d_n2, assign22680_e17450_d_n4, assign22680_e17450_d_n5, assign22680_e17450_d_n6, assign22680_e17450_d_n7, assign22680_e17450_d_n8, assign22680_e17450_d_n9, assign22680_e17450_d_n10, assign22680_e17450_d_n11, assign22680_e17450_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22680_e17450;
        locals.var_t7_dn0 = assign22680_e17450_d_n0;
        locals.var_t7_dn2 = assign22680_e17450_d_n2;
        locals.var_t7_dn4 = assign22680_e17450_d_n4;
        locals.var_t7_dn5 = assign22680_e17450_d_n5;
        locals.var_t7_dn6 = assign22680_e17450_d_n6;
        locals.var_t7_dn7 = assign22680_e17450_d_n7;
        locals.var_t7_dn8 = assign22680_e17450_d_n8;
        locals.var_t7_dn9 = assign22680_e17450_d_n9;
        locals.var_t7_dn10 = assign22680_e17450_d_n10;
        locals.var_t7_dn11 = assign22680_e17450_d_n11;
        locals.var_t7_dn14 = assign22680_e17450_d_n14;
        locals.var_t7_rv = 0.0;

        let assign22690_e17453: f64 = (locals.var_dtox * 1000000000000.0);
        let assign22690_e17455: f64 = if assign22690_e17453 < locals.var_tox0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign22690_e17455;
        locals.var_guard431_rv = 0.0;

        let (assign22700_e17462, assign22700_e17462_d_n0, assign22700_e17462_d_n2, assign22700_e17462_d_n4, assign22700_e17462_d_n5, assign22700_e17462_d_n6, assign22700_e17462_d_n7, assign22700_e17462_d_n8, assign22700_e17462_d_n9, assign22700_e17462_d_n10, assign22700_e17462_d_n11, assign22700_e17462_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22700_e17462;
        locals.var_dtox_dn0 = assign22700_e17462_d_n0;
        locals.var_dtox_dn2 = assign22700_e17462_d_n2;
        locals.var_dtox_dn4 = assign22700_e17462_d_n4;
        locals.var_dtox_dn5 = assign22700_e17462_d_n5;
        locals.var_dtox_dn6 = assign22700_e17462_d_n6;
        locals.var_dtox_dn7 = assign22700_e17462_d_n7;
        locals.var_dtox_dn8 = assign22700_e17462_d_n8;
        locals.var_dtox_dn9 = assign22700_e17462_d_n9;
        locals.var_dtox_dn10 = assign22700_e17462_d_n10;
        locals.var_dtox_dn11 = assign22700_e17462_d_n11;
        locals.var_dtox_dn14 = assign22700_e17462_d_n14;
        locals.var_dtox_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22710_e17469,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22710_e17469;
        locals.var_flg_qme_rv = 0.0;

        let (assign22720_e17476, assign22720_e17476_d_n0, assign22720_e17476_d_n2, assign22720_e17476_d_n4, assign22720_e17476_d_n5, assign22720_e17476_d_n6, assign22720_e17476_d_n7, assign22720_e17476_d_n8, assign22720_e17476_d_n9, assign22720_e17476_d_n10, assign22720_e17476_d_n11, assign22720_e17476_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22720_e17474: f64 = (locals.var_tox0 + locals.var_dtox);
        (assign22720_e17474, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22720_e17476;
        locals.var_toxe_dn0 = assign22720_e17476_d_n0;
        locals.var_toxe_dn2 = assign22720_e17476_d_n2;
        locals.var_toxe_dn4 = assign22720_e17476_d_n4;
        locals.var_toxe_dn5 = assign22720_e17476_d_n5;
        locals.var_toxe_dn6 = assign22720_e17476_d_n6;
        locals.var_toxe_dn7 = assign22720_e17476_d_n7;
        locals.var_toxe_dn8 = assign22720_e17476_d_n8;
        locals.var_toxe_dn9 = assign22720_e17476_d_n9;
        locals.var_toxe_dn10 = assign22720_e17476_d_n10;
        locals.var_toxe_dn11 = assign22720_e17476_d_n11;
        locals.var_toxe_dn14 = assign22720_e17476_d_n14;
        locals.var_toxe_rv = 0.0;

        let (assign22730_e17483, assign22730_e17483_d_n0, assign22730_e17483_d_n2, assign22730_e17483_d_n4, assign22730_e17483_d_n5, assign22730_e17483_d_n6, assign22730_e17483_d_n7, assign22730_e17483_d_n8, assign22730_e17483_d_n9, assign22730_e17483_d_n10, assign22730_e17483_d_n11, assign22730_e17483_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22730_e17481: f64 = (locals.var_c_eox / locals.var_toxe);
        (assign22730_e17481, (-((locals.var_c_eox * locals.var_toxe_dn0) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn2) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn4) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn5) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn6) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn7) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn8) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn9) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn10) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn11) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn14) / (locals.var_toxe * locals.var_toxe))),)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22730_e17483;
        locals.var_cox_dn0 = assign22730_e17483_d_n0;
        locals.var_cox_dn2 = assign22730_e17483_d_n2;
        locals.var_cox_dn4 = assign22730_e17483_d_n4;
        locals.var_cox_dn5 = assign22730_e17483_d_n5;
        locals.var_cox_dn6 = assign22730_e17483_d_n6;
        locals.var_cox_dn7 = assign22730_e17483_d_n7;
        locals.var_cox_dn8 = assign22730_e17483_d_n8;
        locals.var_cox_dn9 = assign22730_e17483_d_n9;
        locals.var_cox_dn10 = assign22730_e17483_d_n10;
        locals.var_cox_dn11 = assign22730_e17483_d_n11;
        locals.var_cox_dn14 = assign22730_e17483_d_n14;
        locals.var_cox_rv = 0.0;

        let (assign22740_e17493, assign22740_e17493_d_n0, assign22740_e17493_d_n2, assign22740_e17493_d_n4, assign22740_e17493_d_n5, assign22740_e17493_d_n6, assign22740_e17493_d_n7, assign22740_e17493_d_n8, assign22740_e17493_d_n9, assign22740_e17493_d_n10, assign22740_e17493_d_n11, assign22740_e17493_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22740_e17487: f64 = (-locals.var_c_eox);
        let assign22740_e17490: f64 = (locals.var_toxe * locals.var_toxe);
        let assign22740_e17491: f64 = (assign22740_e17487 / assign22740_e17490);
        (assign22740_e17491, (-((assign22740_e17487 * ((locals.var_toxe_dn0 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn0))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn2 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn2))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn4 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn4))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn5 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn5))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn6 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn6))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn7 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn7))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn8 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn8))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn9 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn9))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn10 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn10))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn11 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn11))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn14 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn14))) / (assign22740_e17490 * assign22740_e17490))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22740_e17493;
        locals.var_t1_dn0 = assign22740_e17493_d_n0;
        locals.var_t1_dn2 = assign22740_e17493_d_n2;
        locals.var_t1_dn4 = assign22740_e17493_d_n4;
        locals.var_t1_dn5 = assign22740_e17493_d_n5;
        locals.var_t1_dn6 = assign22740_e17493_d_n6;
        locals.var_t1_dn7 = assign22740_e17493_d_n7;
        locals.var_t1_dn8 = assign22740_e17493_d_n8;
        locals.var_t1_dn9 = assign22740_e17493_d_n9;
        locals.var_t1_dn10 = assign22740_e17493_d_n10;
        locals.var_t1_dn11 = assign22740_e17493_d_n11;
        locals.var_t1_dn14 = assign22740_e17493_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22750_e17500, assign22750_e17500_d_n0, assign22750_e17500_d_n2, assign22750_e17500_d_n4, assign22750_e17500_d_n5, assign22750_e17500_d_n6, assign22750_e17500_d_n7, assign22750_e17500_d_n8, assign22750_e17500_d_n9, assign22750_e17500_d_n10, assign22750_e17500_d_n11, assign22750_e17500_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22750_e17498: f64 = (locals.var_toxe / locals.var_c_eox);
        (assign22750_e17498, (locals.var_toxe_dn0 / locals.var_c_eox), (locals.var_toxe_dn2 / locals.var_c_eox), (locals.var_toxe_dn4 / locals.var_c_eox), (locals.var_toxe_dn5 / locals.var_c_eox), (locals.var_toxe_dn6 / locals.var_c_eox), (locals.var_toxe_dn7 / locals.var_c_eox), (locals.var_toxe_dn8 / locals.var_c_eox), (locals.var_toxe_dn9 / locals.var_c_eox), (locals.var_toxe_dn10 / locals.var_c_eox), (locals.var_toxe_dn11 / locals.var_c_eox), (locals.var_toxe_dn14 / locals.var_c_eox),)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22750_e17500;
        locals.var_cox_inv_dn0 = assign22750_e17500_d_n0;
        locals.var_cox_inv_dn2 = assign22750_e17500_d_n2;
        locals.var_cox_inv_dn4 = assign22750_e17500_d_n4;
        locals.var_cox_inv_dn5 = assign22750_e17500_d_n5;
        locals.var_cox_inv_dn6 = assign22750_e17500_d_n6;
        locals.var_cox_inv_dn7 = assign22750_e17500_d_n7;
        locals.var_cox_inv_dn8 = assign22750_e17500_d_n8;
        locals.var_cox_inv_dn9 = assign22750_e17500_d_n9;
        locals.var_cox_inv_dn10 = assign22750_e17500_d_n10;
        locals.var_cox_inv_dn11 = assign22750_e17500_d_n11;
        locals.var_cox_inv_dn14 = assign22750_e17500_d_n14;
        locals.var_cox_inv_rv = 0.0;

        let (assign22760_e17507, assign22760_e17507_d_n0, assign22760_e17507_d_n2, assign22760_e17507_d_n4, assign22760_e17507_d_n5, assign22760_e17507_d_n6, assign22760_e17507_d_n7, assign22760_e17507_d_n8, assign22760_e17507_d_n9, assign22760_e17507_d_n10, assign22760_e17507_d_n11, assign22760_e17507_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22760_e17505: f64 = (1.0 / locals.var_c_eox);
        (assign22760_e17505, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22760_e17507;
        locals.var_t1_dn0 = assign22760_e17507_d_n0;
        locals.var_t1_dn2 = assign22760_e17507_d_n2;
        locals.var_t1_dn4 = assign22760_e17507_d_n4;
        locals.var_t1_dn5 = assign22760_e17507_d_n5;
        locals.var_t1_dn6 = assign22760_e17507_d_n6;
        locals.var_t1_dn7 = assign22760_e17507_d_n7;
        locals.var_t1_dn8 = assign22760_e17507_d_n8;
        locals.var_t1_dn9 = assign22760_e17507_d_n9;
        locals.var_t1_dn10 = assign22760_e17507_d_n10;
        locals.var_t1_dn11 = assign22760_e17507_d_n11;
        locals.var_t1_dn14 = assign22760_e17507_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22770_e17516, assign22770_e17516_d_n0, assign22770_e17516_d_n2, assign22770_e17516_d_n4, assign22770_e17516_d_n5, assign22770_e17516_d_n6, assign22770_e17516_d_n7, assign22770_e17516_d_n8, assign22770_e17516_d_n9, assign22770_e17516_d_n10, assign22770_e17516_d_n11, assign22770_e17516_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22770_e17512: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22770_e17514: f64 = (assign22770_e17512 * locals.var_cox_inv);
        (assign22770_e17514, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22770_e17516;
        locals.var_t0_dn0 = assign22770_e17516_d_n0;
        locals.var_t0_dn2 = assign22770_e17516_d_n2;
        locals.var_t0_dn4 = assign22770_e17516_d_n4;
        locals.var_t0_dn5 = assign22770_e17516_d_n5;
        locals.var_t0_dn6 = assign22770_e17516_d_n6;
        locals.var_t0_dn7 = assign22770_e17516_d_n7;
        locals.var_t0_dn8 = assign22770_e17516_d_n8;
        locals.var_t0_dn9 = assign22770_e17516_d_n9;
        locals.var_t0_dn10 = assign22770_e17516_d_n10;
        locals.var_t0_dn11 = assign22770_e17516_d_n11;
        locals.var_t0_dn14 = assign22770_e17516_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign22780_e17523, assign22780_e17523_d_n0, assign22780_e17523_d_n2, assign22780_e17523_d_n4, assign22780_e17523_d_n5, assign22780_e17523_d_n6, assign22780_e17523_d_n7, assign22780_e17523_d_n8, assign22780_e17523_d_n9, assign22780_e17523_d_n10, assign22780_e17523_d_n11, assign22780_e17523_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22780_e17521: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22780_e17521, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22780_e17523;
        locals.var_cnstcoxi_dn0 = assign22780_e17523_d_n0;
        locals.var_cnstcoxi_dn2 = assign22780_e17523_d_n2;
        locals.var_cnstcoxi_dn4 = assign22780_e17523_d_n4;
        locals.var_cnstcoxi_dn5 = assign22780_e17523_d_n5;
        locals.var_cnstcoxi_dn6 = assign22780_e17523_d_n6;
        locals.var_cnstcoxi_dn7 = assign22780_e17523_d_n7;
        locals.var_cnstcoxi_dn8 = assign22780_e17523_d_n8;
        locals.var_cnstcoxi_dn9 = assign22780_e17523_d_n9;
        locals.var_cnstcoxi_dn10 = assign22780_e17523_d_n10;
        locals.var_cnstcoxi_dn11 = assign22780_e17523_d_n11;
        locals.var_cnstcoxi_dn14 = assign22780_e17523_d_n14;
        locals.var_cnstcoxi_rv = 0.0;

        locals.var_vbsz2 = locals.var_vbsz;
        locals.var_vbsz2_dn0 = locals.var_vbsz_dn0;
        locals.var_vbsz2_dn2 = locals.var_vbsz_dn2;
        locals.var_vbsz2_dn4 = locals.var_vbsz_dn4;
        locals.var_vbsz2_dn5 = locals.var_vbsz_dn5;
        locals.var_vbsz2_dn6 = locals.var_vbsz_dn6;
        locals.var_vbsz2_dn7 = locals.var_vbsz_dn7;
        locals.var_vbsz2_dn8 = locals.var_vbsz_dn8;
        locals.var_vbsz2_dn9 = locals.var_vbsz_dn9;
        locals.var_vbsz2_dn10 = locals.var_vbsz_dn10;
        locals.var_vbsz2_dn11 = locals.var_vbsz_dn11;
        locals.var_vbsz2_dn14 = locals.var_vbsz_dn14;
        locals.var_vbsz2_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;
        locals.var_t1_rv = 0.0;

        let assign22810_e17529: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign22810_e17530: f64 = (locals.var_t1 * assign22810_e17529);
        let assign22810_e17531: f64 = (assign22810_e17530).sqrt();
        locals.var_qb0 = assign22810_e17531;
        locals.var_qb0_dn0 = (((locals.var_t1_dn0 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn2 = (((locals.var_t1_dn2 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn4 = (((locals.var_t1_dn4 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn5 = (((locals.var_t1_dn5 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn6 = (((locals.var_t1_dn6 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn7 = (((locals.var_t1_dn7 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn8 = (((locals.var_t1_dn8 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn9 = (((locals.var_t1_dn9 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn10 = (((locals.var_t1_dn10 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn11 = (((locals.var_t1_dn11 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn14 = (((locals.var_t1_dn14 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign22810_e17531));
        locals.var_qb0_rv = 0.0;

        let assign22820_e17534: f64 = (0.5 * locals.var_t1);
        let assign22820_e17536: f64 = (assign22820_e17534 / locals.var_qb0);
        locals.var_t2 = assign22820_e17536;
        locals.var_t2_dn0 = ((((0.5 * locals.var_t1_dn0) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn0)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn2 = ((((0.5 * locals.var_t1_dn2) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn2)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn4 = ((((0.5 * locals.var_t1_dn4) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn5 = ((((0.5 * locals.var_t1_dn5) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn5)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn6 = ((((0.5 * locals.var_t1_dn6) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn6)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn7 = ((((0.5 * locals.var_t1_dn7) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn7)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn8 = ((((0.5 * locals.var_t1_dn8) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn8)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn9 = ((((0.5 * locals.var_t1_dn9) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn9)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn10 = ((((0.5 * locals.var_t1_dn10) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn10)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn11 = ((((0.5 * locals.var_t1_dn11) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn11)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn14 = ((((0.5 * locals.var_t1_dn14) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn14)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_rv = 0.0;

        let assign22830_e17539: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22830_e17542: f64 = (locals.var_qb0 * locals.var_cox_inv);
        let assign22830_e17543: f64 = (assign22830_e17539 + assign22830_e17542);
        let assign22830_e17545: f64 = (assign22830_e17543 + locals.var_ptovr);
        locals.var_vthp = assign22830_e17545;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn4 = ((locals.var_pb20_dn4 + ((locals.var_qb0_dn4 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn4))) + locals.var_ptovr_dn4);
        locals.var_vthp_dn5 = ((locals.var_pb20_dn5 + ((locals.var_qb0_dn5 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn5))) + locals.var_ptovr_dn5);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn8 = ((locals.var_pb20_dn8 + ((locals.var_qb0_dn8 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn8))) + locals.var_ptovr_dn8);
        locals.var_vthp_dn9 = ((locals.var_pb20_dn9 + ((locals.var_qb0_dn9 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn9))) + locals.var_ptovr_dn9);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = ((locals.var_pb20_dn11 + ((locals.var_qb0_dn11 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn14 = ((locals.var_pb20_dn14 + ((locals.var_qb0_dn14 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn14))) + locals.var_ptovr_dn14);
        locals.var_vthp_rv = 0.0;

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn4 = locals.var_pb20_dn4;
        locals.var_pb20b_dn5 = locals.var_pb20_dn5;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn8 = locals.var_pb20_dn8;
        locals.var_pb20b_dn9 = locals.var_pb20_dn9;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn14 = locals.var_pb20_dn14;
        locals.var_pb20b_rv = 0.0;

        locals.var_t0 = 0.95;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let (assign22860_e17553,) = {
    if (locals.var_uc_codep > 1.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_t4 = assign22860_e17553;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn14 = 0.0;
        locals.var_t4_rv = 0.0;

        let assign22870_e17556: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22870_e17559: f64 = (locals.var_t4 * locals.var_vbsz2);
        let assign22870_e17560: f64 = (assign22870_e17556 - assign22870_e17559);
        let assign22870_e17562: f64 = (assign22870_e17560 - 0.001);
        locals.var_t1 = assign22870_e17562;
        locals.var_t1_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - ((locals.var_t4_dn0 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn0)));
        locals.var_t1_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - ((locals.var_t4_dn2 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn2)));
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - ((locals.var_t4_dn4 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn4)));
        locals.var_t1_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - ((locals.var_t4_dn5 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn5)));
        locals.var_t1_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - ((locals.var_t4_dn6 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn6)));
        locals.var_t1_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - ((locals.var_t4_dn7 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn7)));
        locals.var_t1_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - ((locals.var_t4_dn8 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn8)));
        locals.var_t1_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - ((locals.var_t4_dn9 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn9)));
        locals.var_t1_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - ((locals.var_t4_dn10 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn10)));
        locals.var_t1_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - ((locals.var_t4_dn11 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn11)));
        locals.var_t1_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - ((locals.var_t4_dn14 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn14)));
        locals.var_t1_rv = 0.0;

        let assign22880_e17565: f64 = (locals.var_t1 * locals.var_t1);
        let assign22880_e17568: f64 = (4.0 * locals.var_t0);
        let assign22880_e17570: f64 = (assign22880_e17568 * locals.var_pb20b);
        let assign22880_e17572: f64 = (assign22880_e17570 * 0.001);
        let assign22880_e17573: f64 = (assign22880_e17565 + assign22880_e17572);
        let assign22880_e17574: f64 = (assign22880_e17573).sqrt();
        locals.var_t2 = assign22880_e17574;
        locals.var_t2_dn0 = ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((4.0 * locals.var_t0_dn0) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn0)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn2 = ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((4.0 * locals.var_t0_dn2) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn2)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn4 = ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((4.0 * locals.var_t0_dn4) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn4)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn5 = ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((4.0 * locals.var_t0_dn5) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn5)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn6 = ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((4.0 * locals.var_t0_dn6) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn6)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn7 = ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((4.0 * locals.var_t0_dn7) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn7)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn8 = ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((4.0 * locals.var_t0_dn8) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn8)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn9 = ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((4.0 * locals.var_t0_dn9) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn9)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn10 = ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((4.0 * locals.var_t0_dn10) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn10)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn11 = ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((4.0 * locals.var_t0_dn11) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn11)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn14 = ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((4.0 * locals.var_t0_dn14) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn14)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_rv = 0.0;

        let assign22890_e17577: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22890_e17581: f64 = (locals.var_t1 + locals.var_t2);
        let assign22890_e17582: f64 = (0.5 * assign22890_e17581);
        let assign22890_e17583: f64 = (assign22890_e17577 - assign22890_e17582);
        locals.var_t3 = assign22890_e17583;
        locals.var_t3_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0)));
        locals.var_t3_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2)));
        locals.var_t3_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4)));
        locals.var_t3_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5)));
        locals.var_t3_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6)));
        locals.var_t3_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7)));
        locals.var_t3_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8)));
        locals.var_t3_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9)));
        locals.var_t3_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10)));
        locals.var_t3_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11)));
        locals.var_t3_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14)));
        locals.var_t3_rv = 0.0;

        let (assign22900_e17591, assign22900_e17591_d_n0, assign22900_e17591_d_n2, assign22900_e17591_d_n4, assign22900_e17591_d_n5, assign22900_e17591_d_n6, assign22900_e17591_d_n7, assign22900_e17591_d_n8, assign22900_e17591_d_n9, assign22900_e17591_d_n10, assign22900_e17591_d_n11, assign22900_e17591_d_n14,) = {
    if (locals.var_uc_codep == 1.0) {
        let assign22900_e17589: f64 = (p.p366 * locals.var_vdsz);
        (assign22900_e17589, (p.p366 * locals.var_vdsz_dn0), (p.p366 * locals.var_vdsz_dn2), (p.p366 * locals.var_vdsz_dn4), (p.p366 * locals.var_vdsz_dn5), (p.p366 * locals.var_vdsz_dn6), (p.p366 * locals.var_vdsz_dn7), (p.p366 * locals.var_vdsz_dn8), (p.p366 * locals.var_vdsz_dn9), (p.p366 * locals.var_vdsz_dn10), (p.p366 * locals.var_vdsz_dn11), (p.p366 * locals.var_vdsz_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_t5 = assign22900_e17591;
        locals.var_t5_dn0 = assign22900_e17591_d_n0;
        locals.var_t5_dn2 = assign22900_e17591_d_n2;
        locals.var_t5_dn4 = assign22900_e17591_d_n4;
        locals.var_t5_dn5 = assign22900_e17591_d_n5;
        locals.var_t5_dn6 = assign22900_e17591_d_n6;
        locals.var_t5_dn7 = assign22900_e17591_d_n7;
        locals.var_t5_dn8 = assign22900_e17591_d_n8;
        locals.var_t5_dn9 = assign22900_e17591_d_n9;
        locals.var_t5_dn10 = assign22900_e17591_d_n10;
        locals.var_t5_dn11 = assign22900_e17591_d_n11;
        locals.var_t5_dn14 = assign22900_e17591_d_n14;
        locals.var_t5_rv = 0.0;

        let assign22910_e17594: f64 = (locals.var_pb20b - locals.var_t3);
        let assign22910_e17596: f64 = (assign22910_e17594 + locals.var_t5);
        locals.var_pbsum = assign22910_e17596;
        locals.var_pbsum_dn0 = ((locals.var_pb20b_dn0 - locals.var_t3_dn0) + locals.var_t5_dn0);
        locals.var_pbsum_dn2 = ((locals.var_pb20b_dn2 - locals.var_t3_dn2) + locals.var_t5_dn2);
        locals.var_pbsum_dn4 = ((locals.var_pb20b_dn4 - locals.var_t3_dn4) + locals.var_t5_dn4);
        locals.var_pbsum_dn5 = ((locals.var_pb20b_dn5 - locals.var_t3_dn5) + locals.var_t5_dn5);
        locals.var_pbsum_dn6 = ((locals.var_pb20b_dn6 - locals.var_t3_dn6) + locals.var_t5_dn6);
        locals.var_pbsum_dn7 = ((locals.var_pb20b_dn7 - locals.var_t3_dn7) + locals.var_t5_dn7);
        locals.var_pbsum_dn8 = ((locals.var_pb20b_dn8 - locals.var_t3_dn8) + locals.var_t5_dn8);
        locals.var_pbsum_dn9 = ((locals.var_pb20b_dn9 - locals.var_t3_dn9) + locals.var_t5_dn9);
        locals.var_pbsum_dn10 = ((locals.var_pb20b_dn10 - locals.var_t3_dn10) + locals.var_t5_dn10);
        locals.var_pbsum_dn11 = ((locals.var_pb20b_dn11 - locals.var_t3_dn11) + locals.var_t5_dn11);
        locals.var_pbsum_dn14 = ((locals.var_pb20b_dn14 - locals.var_t3_dn14) + locals.var_t5_dn14);
        locals.var_pbsum_rv = 0.0;

        let assign22920_e17598: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign22920_e17598;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn4 = (locals.var_pbsum_dn4 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn5 = (locals.var_pbsum_dn5 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn8 = (locals.var_pbsum_dn8 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn9 = (locals.var_pbsum_dn9 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn14 = (locals.var_pbsum_dn14 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_rv = 0.0;

        let assign22930_e17601: f64 = if p.p140 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign22930_e17601;
        locals.var_guard432_rv = 0.0;

        let (assign22940_e17605, assign22940_e17605_d_n0, assign22940_e17605_d_n2, assign22940_e17605_d_n4, assign22940_e17605_d_n5, assign22940_e17605_d_n6, assign22940_e17605_d_n7, assign22940_e17605_d_n8, assign22940_e17605_d_n9, assign22940_e17605_d_n10, assign22940_e17605_d_n11, assign22940_e17605_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        (locals.var_qnsub_esi2, locals.var_qnsub_esi2_dn0, locals.var_qnsub_esi2_dn2, locals.var_qnsub_esi2_dn4, locals.var_qnsub_esi2_dn5, locals.var_qnsub_esi2_dn6, locals.var_qnsub_esi2_dn7, locals.var_qnsub_esi2_dn8, locals.var_qnsub_esi2_dn9, locals.var_qnsub_esi2_dn10, locals.var_qnsub_esi2_dn11, locals.var_qnsub_esi2_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22940_e17605;
        locals.var_t1_dn0 = assign22940_e17605_d_n0;
        locals.var_t1_dn2 = assign22940_e17605_d_n2;
        locals.var_t1_dn4 = assign22940_e17605_d_n4;
        locals.var_t1_dn5 = assign22940_e17605_d_n5;
        locals.var_t1_dn6 = assign22940_e17605_d_n6;
        locals.var_t1_dn7 = assign22940_e17605_d_n7;
        locals.var_t1_dn8 = assign22940_e17605_d_n8;
        locals.var_t1_dn9 = assign22940_e17605_d_n9;
        locals.var_t1_dn10 = assign22940_e17605_d_n10;
        locals.var_t1_dn11 = assign22940_e17605_d_n11;
        locals.var_t1_dn14 = assign22940_e17605_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign22950_e17611, assign22950_e17611_d_n0, assign22950_e17611_d_n2, assign22950_e17611_d_n4, assign22950_e17611_d_n5, assign22950_e17611_d_n6, assign22950_e17611_d_n7, assign22950_e17611_d_n8, assign22950_e17611_d_n9, assign22950_e17611_d_n10, assign22950_e17611_d_n11, assign22950_e17611_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22950_e17609: f64 = (p.p224 - locals.var_vbsz2);
        (assign22950_e17609, (-locals.var_vbsz2_dn0), (-locals.var_vbsz2_dn2), (-locals.var_vbsz2_dn4), (-locals.var_vbsz2_dn5), (-locals.var_vbsz2_dn6), (-locals.var_vbsz2_dn7), (-locals.var_vbsz2_dn8), (-locals.var_vbsz2_dn9), (-locals.var_vbsz2_dn10), (-locals.var_vbsz2_dn11), (-locals.var_vbsz2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22950_e17611;
        locals.var_t2_dn0 = assign22950_e17611_d_n0;
        locals.var_t2_dn2 = assign22950_e17611_d_n2;
        locals.var_t2_dn4 = assign22950_e17611_d_n4;
        locals.var_t2_dn5 = assign22950_e17611_d_n5;
        locals.var_t2_dn6 = assign22950_e17611_d_n6;
        locals.var_t2_dn7 = assign22950_e17611_d_n7;
        locals.var_t2_dn8 = assign22950_e17611_d_n8;
        locals.var_t2_dn9 = assign22950_e17611_d_n9;
        locals.var_t2_dn10 = assign22950_e17611_d_n10;
        locals.var_t2_dn11 = assign22950_e17611_d_n11;
        locals.var_t2_dn14 = assign22950_e17611_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign22960_e17617, assign22960_e17617_d_n0, assign22960_e17617_d_n2, assign22960_e17617_d_n4, assign22960_e17617_d_n5, assign22960_e17617_d_n6, assign22960_e17617_d_n7, assign22960_e17617_d_n8, assign22960_e17617_d_n9, assign22960_e17617_d_n10, assign22960_e17617_d_n11, assign22960_e17617_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22960_e17615: f64 = (locals.var_t2 + 1e-25);
        (assign22960_e17615, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22960_e17617;
        locals.var_t3_dn0 = assign22960_e17617_d_n0;
        locals.var_t3_dn2 = assign22960_e17617_d_n2;
        locals.var_t3_dn4 = assign22960_e17617_d_n4;
        locals.var_t3_dn5 = assign22960_e17617_d_n5;
        locals.var_t3_dn6 = assign22960_e17617_d_n6;
        locals.var_t3_dn7 = assign22960_e17617_d_n7;
        locals.var_t3_dn8 = assign22960_e17617_d_n8;
        locals.var_t3_dn9 = assign22960_e17617_d_n9;
        locals.var_t3_dn10 = assign22960_e17617_d_n10;
        locals.var_t3_dn11 = assign22960_e17617_d_n11;
        locals.var_t3_dn14 = assign22960_e17617_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign22970_e17628, assign22970_e17628_d_n0, assign22970_e17628_d_n2, assign22970_e17628_d_n4, assign22970_e17628_d_n5, assign22970_e17628_d_n6, assign22970_e17628_d_n7, assign22970_e17628_d_n8, assign22970_e17628_d_n9, assign22970_e17628_d_n10, assign22970_e17628_d_n11, assign22970_e17628_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22970_e17621: f64 = (locals.var_t3 * locals.var_t3);
        let assign22970_e17624: f64 = (4.0 * 0.001);
        let assign22970_e17625: f64 = (assign22970_e17621 + assign22970_e17624);
        let assign22970_e17626: f64 = (assign22970_e17625).sqrt();
        (assign22970_e17626, (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22970_e17626)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22970_e17628;
        locals.var_t4_dn0 = assign22970_e17628_d_n0;
        locals.var_t4_dn2 = assign22970_e17628_d_n2;
        locals.var_t4_dn4 = assign22970_e17628_d_n4;
        locals.var_t4_dn5 = assign22970_e17628_d_n5;
        locals.var_t4_dn6 = assign22970_e17628_d_n6;
        locals.var_t4_dn7 = assign22970_e17628_d_n7;
        locals.var_t4_dn8 = assign22970_e17628_d_n8;
        locals.var_t4_dn9 = assign22970_e17628_d_n9;
        locals.var_t4_dn10 = assign22970_e17628_d_n10;
        locals.var_t4_dn11 = assign22970_e17628_d_n11;
        locals.var_t4_dn14 = assign22970_e17628_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22980_e17636, assign22980_e17636_d_n0, assign22980_e17636_d_n2, assign22980_e17636_d_n4, assign22980_e17636_d_n5, assign22980_e17636_d_n6, assign22980_e17636_d_n7, assign22980_e17636_d_n8, assign22980_e17636_d_n9, assign22980_e17636_d_n10, assign22980_e17636_d_n11, assign22980_e17636_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22980_e17633: f64 = (locals.var_t3 + locals.var_t4);
        let assign22980_e17634: f64 = (0.5 * assign22980_e17633);
        (assign22980_e17634, (0.5 * (locals.var_t3_dn0 + locals.var_t4_dn0)), (0.5 * (locals.var_t3_dn2 + locals.var_t4_dn2)), (0.5 * (locals.var_t3_dn4 + locals.var_t4_dn4)), (0.5 * (locals.var_t3_dn5 + locals.var_t4_dn5)), (0.5 * (locals.var_t3_dn6 + locals.var_t4_dn6)), (0.5 * (locals.var_t3_dn7 + locals.var_t4_dn7)), (0.5 * (locals.var_t3_dn8 + locals.var_t4_dn8)), (0.5 * (locals.var_t3_dn9 + locals.var_t4_dn9)), (0.5 * (locals.var_t3_dn10 + locals.var_t4_dn10)), (0.5 * (locals.var_t3_dn11 + locals.var_t4_dn11)), (0.5 * (locals.var_t3_dn14 + locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22980_e17636;
        locals.var_t5_dn0 = assign22980_e17636_d_n0;
        locals.var_t5_dn2 = assign22980_e17636_d_n2;
        locals.var_t5_dn4 = assign22980_e17636_d_n4;
        locals.var_t5_dn5 = assign22980_e17636_d_n5;
        locals.var_t5_dn6 = assign22980_e17636_d_n6;
        locals.var_t5_dn7 = assign22980_e17636_d_n7;
        locals.var_t5_dn8 = assign22980_e17636_d_n8;
        locals.var_t5_dn9 = assign22980_e17636_d_n9;
        locals.var_t5_dn10 = assign22980_e17636_d_n10;
        locals.var_t5_dn11 = assign22980_e17636_d_n11;
        locals.var_t5_dn14 = assign22980_e17636_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign22990_e17646, assign22990_e17646_d_n0, assign22990_e17646_d_n2, assign22990_e17646_d_n4, assign22990_e17646_d_n5, assign22990_e17646_d_n6, assign22990_e17646_d_n7, assign22990_e17646_d_n8, assign22990_e17646_d_n9, assign22990_e17646_d_n10, assign22990_e17646_d_n11, assign22990_e17646_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22990_e17642: f64 = (locals.var_t3 / locals.var_t4);
        let assign22990_e17643: f64 = (1.0 + assign22990_e17642);
        let assign22990_e17644: f64 = (0.5 * assign22990_e17643);
        (assign22990_e17644, (0.5 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22990_e17646;
        locals.var_t6_dn0 = assign22990_e17646_d_n0;
        locals.var_t6_dn2 = assign22990_e17646_d_n2;
        locals.var_t6_dn4 = assign22990_e17646_d_n4;
        locals.var_t6_dn5 = assign22990_e17646_d_n5;
        locals.var_t6_dn6 = assign22990_e17646_d_n6;
        locals.var_t6_dn7 = assign22990_e17646_d_n7;
        locals.var_t6_dn8 = assign22990_e17646_d_n8;
        locals.var_t6_dn9 = assign22990_e17646_d_n9;
        locals.var_t6_dn10 = assign22990_e17646_d_n10;
        locals.var_t6_dn11 = assign22990_e17646_d_n11;
        locals.var_t6_dn14 = assign22990_e17646_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23000_e17652, assign23000_e17652_d_n0, assign23000_e17652_d_n2, assign23000_e17652_d_n4, assign23000_e17652_d_n5, assign23000_e17652_d_n6, assign23000_e17652_d_n7, assign23000_e17652_d_n8, assign23000_e17652_d_n9, assign23000_e17652_d_n10, assign23000_e17652_d_n11, assign23000_e17652_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23000_e17650: f64 = (1.0 / locals.var_t5);
        (assign23000_e17650, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn14 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23000_e17652;
        locals.var_t7_dn0 = assign23000_e17652_d_n0;
        locals.var_t7_dn2 = assign23000_e17652_d_n2;
        locals.var_t7_dn4 = assign23000_e17652_d_n4;
        locals.var_t7_dn5 = assign23000_e17652_d_n5;
        locals.var_t7_dn6 = assign23000_e17652_d_n6;
        locals.var_t7_dn7 = assign23000_e17652_d_n7;
        locals.var_t7_dn8 = assign23000_e17652_d_n8;
        locals.var_t7_dn9 = assign23000_e17652_d_n9;
        locals.var_t7_dn10 = assign23000_e17652_d_n10;
        locals.var_t7_dn11 = assign23000_e17652_d_n11;
        locals.var_t7_dn14 = assign23000_e17652_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23010_e17658, assign23010_e17658_d_n0, assign23010_e17658_d_n2, assign23010_e17658_d_n4, assign23010_e17658_d_n5, assign23010_e17658_d_n6, assign23010_e17658_d_n7, assign23010_e17658_d_n8, assign23010_e17658_d_n9, assign23010_e17658_d_n10, assign23010_e17658_d_n11, assign23010_e17658_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23010_e17656: f64 = (p.p223 * locals.var_t7);
        (assign23010_e17656, (p.p223 * locals.var_t7_dn0), (p.p223 * locals.var_t7_dn2), (p.p223 * locals.var_t7_dn4), (p.p223 * locals.var_t7_dn5), (p.p223 * locals.var_t7_dn6), (p.p223 * locals.var_t7_dn7), (p.p223 * locals.var_t7_dn8), (p.p223 * locals.var_t7_dn9), (p.p223 * locals.var_t7_dn10), (p.p223 * locals.var_t7_dn11), (p.p223 * locals.var_t7_dn14),)
    } else {
        (locals.var_bs12, locals.var_bs12_dn0, locals.var_bs12_dn2, locals.var_bs12_dn4, locals.var_bs12_dn5, locals.var_bs12_dn6, locals.var_bs12_dn7, locals.var_bs12_dn8, locals.var_bs12_dn9, locals.var_bs12_dn10, locals.var_bs12_dn11, locals.var_bs12_dn14,)
    }
};
        locals.var_bs12 = assign23010_e17658;
        locals.var_bs12_dn0 = assign23010_e17658_d_n0;
        locals.var_bs12_dn2 = assign23010_e17658_d_n2;
        locals.var_bs12_dn4 = assign23010_e17658_d_n4;
        locals.var_bs12_dn5 = assign23010_e17658_d_n5;
        locals.var_bs12_dn6 = assign23010_e17658_d_n6;
        locals.var_bs12_dn7 = assign23010_e17658_d_n7;
        locals.var_bs12_dn8 = assign23010_e17658_d_n8;
        locals.var_bs12_dn9 = assign23010_e17658_d_n9;
        locals.var_bs12_dn10 = assign23010_e17658_d_n10;
        locals.var_bs12_dn11 = assign23010_e17658_d_n11;
        locals.var_bs12_dn14 = assign23010_e17658_d_n14;
        locals.var_bs12_rv = 0.0;

        let (assign23020_e17665, assign23020_e17665_d_n0, assign23020_e17665_d_n2, assign23020_e17665_d_n4, assign23020_e17665_d_n5, assign23020_e17665_d_n6, assign23020_e17665_d_n7, assign23020_e17665_d_n8, assign23020_e17665_d_n9, assign23020_e17665_d_n10, assign23020_e17665_d_n11, assign23020_e17665_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23020_e17661: f64 = (-locals.var_bs12);
        let assign23020_e17663: f64 = (assign23020_e17661 * locals.var_t7);
        (assign23020_e17663, (((-locals.var_bs12_dn0) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn0)), (((-locals.var_bs12_dn2) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn2)), (((-locals.var_bs12_dn4) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn4)), (((-locals.var_bs12_dn5) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn5)), (((-locals.var_bs12_dn6) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn6)), (((-locals.var_bs12_dn7) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn7)), (((-locals.var_bs12_dn8) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn8)), (((-locals.var_bs12_dn9) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn9)), (((-locals.var_bs12_dn10) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn10)), (((-locals.var_bs12_dn11) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn11)), (((-locals.var_bs12_dn14) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23020_e17665;
        locals.var_t8_dn0 = assign23020_e17665_d_n0;
        locals.var_t8_dn2 = assign23020_e17665_d_n2;
        locals.var_t8_dn4 = assign23020_e17665_d_n4;
        locals.var_t8_dn5 = assign23020_e17665_d_n5;
        locals.var_t8_dn6 = assign23020_e17665_d_n6;
        locals.var_t8_dn7 = assign23020_e17665_d_n7;
        locals.var_t8_dn8 = assign23020_e17665_d_n8;
        locals.var_t8_dn9 = assign23020_e17665_d_n9;
        locals.var_t8_dn10 = assign23020_e17665_d_n10;
        locals.var_t8_dn11 = assign23020_e17665_d_n11;
        locals.var_t8_dn14 = assign23020_e17665_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign23030_e17677, assign23030_e17677_d_n0, assign23030_e17677_d_n2, assign23030_e17677_d_n4, assign23030_e17677_d_n5, assign23030_e17677_d_n6, assign23030_e17677_d_n7, assign23030_e17677_d_n8, assign23030_e17677_d_n9, assign23030_e17677_d_n10, assign23030_e17677_d_n11, assign23030_e17677_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23030_e17669: f64 = (0.93 * locals.var_pb20);
        let assign23030_e17672: f64 = (locals.var_vbsz2 + locals.var_bs12);
        let assign23030_e17673: f64 = (assign23030_e17669 - assign23030_e17672);
        let assign23030_e17675: f64 = (assign23030_e17673 - 0.001);
        (assign23030_e17675, ((0.93 * locals.var_pb20_dn0) - (locals.var_vbsz2_dn0 + locals.var_bs12_dn0)), ((0.93 * locals.var_pb20_dn2) - (locals.var_vbsz2_dn2 + locals.var_bs12_dn2)), ((0.93 * locals.var_pb20_dn4) - (locals.var_vbsz2_dn4 + locals.var_bs12_dn4)), ((0.93 * locals.var_pb20_dn5) - (locals.var_vbsz2_dn5 + locals.var_bs12_dn5)), ((0.93 * locals.var_pb20_dn6) - (locals.var_vbsz2_dn6 + locals.var_bs12_dn6)), ((0.93 * locals.var_pb20_dn7) - (locals.var_vbsz2_dn7 + locals.var_bs12_dn7)), ((0.93 * locals.var_pb20_dn8) - (locals.var_vbsz2_dn8 + locals.var_bs12_dn8)), ((0.93 * locals.var_pb20_dn9) - (locals.var_vbsz2_dn9 + locals.var_bs12_dn9)), ((0.93 * locals.var_pb20_dn10) - (locals.var_vbsz2_dn10 + locals.var_bs12_dn10)), ((0.93 * locals.var_pb20_dn11) - (locals.var_vbsz2_dn11 + locals.var_bs12_dn11)), ((0.93 * locals.var_pb20_dn14) - (locals.var_vbsz2_dn14 + locals.var_bs12_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23030_e17677;
        locals.var_tmf1_dn0 = assign23030_e17677_d_n0;
        locals.var_tmf1_dn2 = assign23030_e17677_d_n2;
        locals.var_tmf1_dn4 = assign23030_e17677_d_n4;
        locals.var_tmf1_dn5 = assign23030_e17677_d_n5;
        locals.var_tmf1_dn6 = assign23030_e17677_d_n6;
        locals.var_tmf1_dn7 = assign23030_e17677_d_n7;
        locals.var_tmf1_dn8 = assign23030_e17677_d_n8;
        locals.var_tmf1_dn9 = assign23030_e17677_d_n9;
        locals.var_tmf1_dn10 = assign23030_e17677_d_n10;
        locals.var_tmf1_dn11 = assign23030_e17677_d_n11;
        locals.var_tmf1_dn14 = assign23030_e17677_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23040_e17687, assign23040_e17687_d_n0, assign23040_e17687_d_n2, assign23040_e17687_d_n4, assign23040_e17687_d_n5, assign23040_e17687_d_n6, assign23040_e17687_d_n7, assign23040_e17687_d_n8, assign23040_e17687_d_n9, assign23040_e17687_d_n10, assign23040_e17687_d_n11, assign23040_e17687_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23040_e17682: f64 = (0.93 * locals.var_pb20);
        let assign23040_e17683: f64 = (4.0 * assign23040_e17682);
        let assign23040_e17685: f64 = (assign23040_e17683 * 0.001);
        (assign23040_e17685, ((4.0 * (0.93 * locals.var_pb20_dn0)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn2)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn4)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn5)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn6)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn7)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn8)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn9)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn10)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn11)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn14)) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23040_e17687;
        locals.var_tmf2_dn0 = assign23040_e17687_d_n0;
        locals.var_tmf2_dn2 = assign23040_e17687_d_n2;
        locals.var_tmf2_dn4 = assign23040_e17687_d_n4;
        locals.var_tmf2_dn5 = assign23040_e17687_d_n5;
        locals.var_tmf2_dn6 = assign23040_e17687_d_n6;
        locals.var_tmf2_dn7 = assign23040_e17687_d_n7;
        locals.var_tmf2_dn8 = assign23040_e17687_d_n8;
        locals.var_tmf2_dn9 = assign23040_e17687_d_n9;
        locals.var_tmf2_dn10 = assign23040_e17687_d_n10;
        locals.var_tmf2_dn11 = assign23040_e17687_d_n11;
        locals.var_tmf2_dn14 = assign23040_e17687_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23050_e17697, assign23050_e17697_d_n0, assign23050_e17697_d_n2, assign23050_e17697_d_n4, assign23050_e17697_d_n5, assign23050_e17697_d_n6, assign23050_e17697_d_n7, assign23050_e17697_d_n8, assign23050_e17697_d_n9, assign23050_e17697_d_n10, assign23050_e17697_d_n11, assign23050_e17697_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let (assign23050_e17695, assign23050_e17695_d_n0, assign23050_e17695_d_n2, assign23050_e17695_d_n4, assign23050_e17695_d_n5, assign23050_e17695_d_n6, assign23050_e17695_d_n7, assign23050_e17695_d_n8, assign23050_e17695_d_n9, assign23050_e17695_d_n10, assign23050_e17695_d_n11, assign23050_e17695_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23050_e17694: f64 = (-locals.var_tmf2);
                (assign23050_e17694, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23050_e17695, assign23050_e17695_d_n0, assign23050_e17695_d_n2, assign23050_e17695_d_n4, assign23050_e17695_d_n5, assign23050_e17695_d_n6, assign23050_e17695_d_n7, assign23050_e17695_d_n8, assign23050_e17695_d_n9, assign23050_e17695_d_n10, assign23050_e17695_d_n11, assign23050_e17695_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23050_e17697;
        locals.var_tmf2_dn0 = assign23050_e17697_d_n0;
        locals.var_tmf2_dn2 = assign23050_e17697_d_n2;
        locals.var_tmf2_dn4 = assign23050_e17697_d_n4;
        locals.var_tmf2_dn5 = assign23050_e17697_d_n5;
        locals.var_tmf2_dn6 = assign23050_e17697_d_n6;
        locals.var_tmf2_dn7 = assign23050_e17697_d_n7;
        locals.var_tmf2_dn8 = assign23050_e17697_d_n8;
        locals.var_tmf2_dn9 = assign23050_e17697_d_n9;
        locals.var_tmf2_dn10 = assign23050_e17697_d_n10;
        locals.var_tmf2_dn11 = assign23050_e17697_d_n11;
        locals.var_tmf2_dn14 = assign23050_e17697_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23060_e17706, assign23060_e17706_d_n0, assign23060_e17706_d_n2, assign23060_e17706_d_n4, assign23060_e17706_d_n5, assign23060_e17706_d_n6, assign23060_e17706_d_n7, assign23060_e17706_d_n8, assign23060_e17706_d_n9, assign23060_e17706_d_n10, assign23060_e17706_d_n11, assign23060_e17706_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23060_e17701: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23060_e17703: f64 = (assign23060_e17701 + locals.var_tmf2);
        let assign23060_e17704: f64 = (assign23060_e17703).sqrt();
        (assign23060_e17704, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23060_e17704)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23060_e17706;
        locals.var_tmf2_dn0 = assign23060_e17706_d_n0;
        locals.var_tmf2_dn2 = assign23060_e17706_d_n2;
        locals.var_tmf2_dn4 = assign23060_e17706_d_n4;
        locals.var_tmf2_dn5 = assign23060_e17706_d_n5;
        locals.var_tmf2_dn6 = assign23060_e17706_d_n6;
        locals.var_tmf2_dn7 = assign23060_e17706_d_n7;
        locals.var_tmf2_dn8 = assign23060_e17706_d_n8;
        locals.var_tmf2_dn9 = assign23060_e17706_d_n9;
        locals.var_tmf2_dn10 = assign23060_e17706_d_n10;
        locals.var_tmf2_dn11 = assign23060_e17706_d_n11;
        locals.var_tmf2_dn14 = assign23060_e17706_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23070_e17716, assign23070_e17716_d_n0, assign23070_e17716_d_n2, assign23070_e17716_d_n4, assign23070_e17716_d_n5, assign23070_e17716_d_n6, assign23070_e17716_d_n7, assign23070_e17716_d_n8, assign23070_e17716_d_n9, assign23070_e17716_d_n10, assign23070_e17716_d_n11, assign23070_e17716_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23070_e17712: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23070_e17713: f64 = (1.0 + assign23070_e17712);
        let assign23070_e17714: f64 = (0.5 * assign23070_e17713);
        (assign23070_e17714, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23070_e17716;
        locals.var_t0_dn0 = assign23070_e17716_d_n0;
        locals.var_t0_dn2 = assign23070_e17716_d_n2;
        locals.var_t0_dn4 = assign23070_e17716_d_n4;
        locals.var_t0_dn5 = assign23070_e17716_d_n5;
        locals.var_t0_dn6 = assign23070_e17716_d_n6;
        locals.var_t0_dn7 = assign23070_e17716_d_n7;
        locals.var_t0_dn8 = assign23070_e17716_d_n8;
        locals.var_t0_dn9 = assign23070_e17716_d_n9;
        locals.var_t0_dn10 = assign23070_e17716_d_n10;
        locals.var_t0_dn11 = assign23070_e17716_d_n11;
        locals.var_t0_dn14 = assign23070_e17716_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23080_e17728, assign23080_e17728_d_n0, assign23080_e17728_d_n2, assign23080_e17728_d_n4, assign23080_e17728_d_n5, assign23080_e17728_d_n6, assign23080_e17728_d_n7, assign23080_e17728_d_n8, assign23080_e17728_d_n9, assign23080_e17728_d_n10, assign23080_e17728_d_n11, assign23080_e17728_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23080_e17720: f64 = (0.93 * locals.var_pb20);
        let assign23080_e17724: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23080_e17725: f64 = (0.5 * assign23080_e17724);
        let assign23080_e17726: f64 = (assign23080_e17720 - assign23080_e17725);
        (assign23080_e17726, ((0.93 * locals.var_pb20_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((0.93 * locals.var_pb20_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((0.93 * locals.var_pb20_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((0.93 * locals.var_pb20_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((0.93 * locals.var_pb20_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((0.93 * locals.var_pb20_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((0.93 * locals.var_pb20_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((0.93 * locals.var_pb20_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((0.93 * locals.var_pb20_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((0.93 * locals.var_pb20_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((0.93 * locals.var_pb20_dn14) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign23080_e17728;
        locals.var_t10_dn0 = assign23080_e17728_d_n0;
        locals.var_t10_dn2 = assign23080_e17728_d_n2;
        locals.var_t10_dn4 = assign23080_e17728_d_n4;
        locals.var_t10_dn5 = assign23080_e17728_d_n5;
        locals.var_t10_dn6 = assign23080_e17728_d_n6;
        locals.var_t10_dn7 = assign23080_e17728_d_n7;
        locals.var_t10_dn8 = assign23080_e17728_d_n8;
        locals.var_t10_dn9 = assign23080_e17728_d_n9;
        locals.var_t10_dn10 = assign23080_e17728_d_n10;
        locals.var_t10_dn11 = assign23080_e17728_d_n11;
        locals.var_t10_dn14 = assign23080_e17728_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign23090_e17737, assign23090_e17737_d_n0, assign23090_e17737_d_n2, assign23090_e17737_d_n4, assign23090_e17737_d_n5, assign23090_e17737_d_n6, assign23090_e17737_d_n7, assign23090_e17737_d_n8, assign23090_e17737_d_n9, assign23090_e17737_d_n10, assign23090_e17737_d_n11, assign23090_e17737_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23090_e17733: f64 = (locals.var_pb20 - locals.var_t10);
        let assign23090_e17734: f64 = (locals.var_t1 * assign23090_e17733);
        let assign23090_e17735: f64 = (assign23090_e17734).sqrt();
        (assign23090_e17735, (((locals.var_t1_dn0 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_t10_dn0))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn2 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_t10_dn2))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn4 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_t10_dn4))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn5 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_t10_dn5))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn6 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_t10_dn6))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn7 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_t10_dn7))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn8 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_t10_dn8))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn9 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_t10_dn9))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn10 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_t10_dn10))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn11 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_t10_dn11))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn14 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_t10_dn14))) / (2.0 * assign23090_e17735)),)
    } else {
        (locals.var_qbmm, locals.var_qbmm_dn0, locals.var_qbmm_dn2, locals.var_qbmm_dn4, locals.var_qbmm_dn5, locals.var_qbmm_dn6, locals.var_qbmm_dn7, locals.var_qbmm_dn8, locals.var_qbmm_dn9, locals.var_qbmm_dn10, locals.var_qbmm_dn11, locals.var_qbmm_dn14,)
    }
};
        locals.var_qbmm = assign23090_e17737;
        locals.var_qbmm_dn0 = assign23090_e17737_d_n0;
        locals.var_qbmm_dn2 = assign23090_e17737_d_n2;
        locals.var_qbmm_dn4 = assign23090_e17737_d_n4;
        locals.var_qbmm_dn5 = assign23090_e17737_d_n5;
        locals.var_qbmm_dn6 = assign23090_e17737_d_n6;
        locals.var_qbmm_dn7 = assign23090_e17737_d_n7;
        locals.var_qbmm_dn8 = assign23090_e17737_d_n8;
        locals.var_qbmm_dn9 = assign23090_e17737_d_n9;
        locals.var_qbmm_dn10 = assign23090_e17737_d_n10;
        locals.var_qbmm_dn11 = assign23090_e17737_d_n11;
        locals.var_qbmm_dn14 = assign23090_e17737_d_n14;
        locals.var_qbmm_rv = 0.0;

        let (assign23100_e17743, assign23100_e17743_d_n0, assign23100_e17743_d_n2, assign23100_e17743_d_n4, assign23100_e17743_d_n5, assign23100_e17743_d_n6, assign23100_e17743_d_n7, assign23100_e17743_d_n8, assign23100_e17743_d_n9, assign23100_e17743_d_n10, assign23100_e17743_d_n11, assign23100_e17743_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23100_e17741: f64 = (locals.var_t0 / locals.var_qbmm);
        (assign23100_e17741, (((locals.var_t0_dn0 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn0)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn2 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn2)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn4 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn4)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn5 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn5)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn6 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn6)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn7 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn7)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn8 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn8)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn9 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn9)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn10 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn10)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn11 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn11)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn14 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn14)) / (locals.var_qbmm * locals.var_qbmm)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23100_e17743;
        locals.var_t9_dn0 = assign23100_e17743_d_n0;
        locals.var_t9_dn2 = assign23100_e17743_d_n2;
        locals.var_t9_dn4 = assign23100_e17743_d_n4;
        locals.var_t9_dn5 = assign23100_e17743_d_n5;
        locals.var_t9_dn6 = assign23100_e17743_d_n6;
        locals.var_t9_dn7 = assign23100_e17743_d_n7;
        locals.var_t9_dn8 = assign23100_e17743_d_n8;
        locals.var_t9_dn9 = assign23100_e17743_d_n9;
        locals.var_t9_dn10 = assign23100_e17743_d_n10;
        locals.var_t9_dn11 = assign23100_e17743_d_n11;
        locals.var_t9_dn14 = assign23100_e17743_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign23110_e17751, assign23110_e17751_d_n0, assign23110_e17751_d_n2, assign23110_e17751_d_n4, assign23110_e17751_d_n5, assign23110_e17751_d_n6, assign23110_e17751_d_n7, assign23110_e17751_d_n8, assign23110_e17751_d_n9, assign23110_e17751_d_n10, assign23110_e17751_d_n11, assign23110_e17751_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23110_e17747: f64 = (locals.var_qb0 - locals.var_qbmm);
        let assign23110_e17749: f64 = (assign23110_e17747 * locals.var_cox_inv);
        (assign23110_e17749, (((locals.var_qb0_dn0 - locals.var_qbmm_dn0) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn0)), (((locals.var_qb0_dn2 - locals.var_qbmm_dn2) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn2)), (((locals.var_qb0_dn4 - locals.var_qbmm_dn4) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn4)), (((locals.var_qb0_dn5 - locals.var_qbmm_dn5) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn5)), (((locals.var_qb0_dn6 - locals.var_qbmm_dn6) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn6)), (((locals.var_qb0_dn7 - locals.var_qbmm_dn7) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn7)), (((locals.var_qb0_dn8 - locals.var_qbmm_dn8) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn8)), (((locals.var_qb0_dn9 - locals.var_qbmm_dn9) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn9)), (((locals.var_qb0_dn10 - locals.var_qbmm_dn10) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn10)), (((locals.var_qb0_dn11 - locals.var_qbmm_dn11) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn11)), (((locals.var_qb0_dn14 - locals.var_qbmm_dn14) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_dqb, locals.var_dqb_dn0, locals.var_dqb_dn2, locals.var_dqb_dn4, locals.var_dqb_dn5, locals.var_dqb_dn6, locals.var_dqb_dn7, locals.var_dqb_dn8, locals.var_dqb_dn9, locals.var_dqb_dn10, locals.var_dqb_dn11, locals.var_dqb_dn14,)
    }
};
        locals.var_dqb = assign23110_e17751;
        locals.var_dqb_dn0 = assign23110_e17751_d_n0;
        locals.var_dqb_dn2 = assign23110_e17751_d_n2;
        locals.var_dqb_dn4 = assign23110_e17751_d_n4;
        locals.var_dqb_dn5 = assign23110_e17751_d_n5;
        locals.var_dqb_dn6 = assign23110_e17751_d_n6;
        locals.var_dqb_dn7 = assign23110_e17751_d_n7;
        locals.var_dqb_dn8 = assign23110_e17751_d_n8;
        locals.var_dqb_dn9 = assign23110_e17751_d_n9;
        locals.var_dqb_dn10 = assign23110_e17751_d_n10;
        locals.var_dqb_dn11 = assign23110_e17751_d_n11;
        locals.var_dqb_dn14 = assign23110_e17751_d_n14;
        locals.var_dqb_rv = 0.0;

        let (assign23120_e17761, assign23120_e17761_d_n0, assign23120_e17761_d_n2, assign23120_e17761_d_n4, assign23120_e17761_d_n5, assign23120_e17761_d_n6, assign23120_e17761_d_n7, assign23120_e17761_d_n8, assign23120_e17761_d_n9, assign23120_e17761_d_n10, assign23120_e17761_d_n11, assign23120_e17761_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23120_e17755: f64 = (2.0 * 1.6021918e-19);
        let assign23120_e17757: f64 = (assign23120_e17755 * locals.var_ef_nsubc);
        let assign23120_e17759: f64 = (assign23120_e17757 * 1.034943e-10);
        (assign23120_e17759, ((assign23120_e17755 * locals.var_ef_nsubc_dn0) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn2) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn4) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn5) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn6) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn7) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn8) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn9) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn10) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn11) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn14) * 1.034943e-10),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23120_e17761;
        locals.var_t1_dn0 = assign23120_e17761_d_n0;
        locals.var_t1_dn2 = assign23120_e17761_d_n2;
        locals.var_t1_dn4 = assign23120_e17761_d_n4;
        locals.var_t1_dn5 = assign23120_e17761_d_n5;
        locals.var_t1_dn6 = assign23120_e17761_d_n6;
        locals.var_t1_dn7 = assign23120_e17761_d_n7;
        locals.var_t1_dn8 = assign23120_e17761_d_n8;
        locals.var_t1_dn9 = assign23120_e17761_d_n9;
        locals.var_t1_dn10 = assign23120_e17761_d_n10;
        locals.var_t1_dn11 = assign23120_e17761_d_n11;
        locals.var_t1_dn14 = assign23120_e17761_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23130_e17770, assign23130_e17770_d_n0, assign23130_e17770_d_n2, assign23130_e17770_d_n4, assign23130_e17770_d_n5, assign23130_e17770_d_n6, assign23130_e17770_d_n7, assign23130_e17770_d_n8, assign23130_e17770_d_n9, assign23130_e17770_d_n10, assign23130_e17770_d_n11, assign23130_e17770_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23130_e17766: f64 = (locals.var_pb2c - locals.var_vbsz2);
        let assign23130_e17767: f64 = (locals.var_t1 * assign23130_e17766);
        let assign23130_e17768: f64 = (assign23130_e17767).sqrt();
        (assign23130_e17768, (((locals.var_t1_dn0 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn2 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn4 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn5 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn6 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn7 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn8 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn9 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn10 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn11 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn14 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign23130_e17768)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23130_e17770;
        locals.var_t2_dn0 = assign23130_e17770_d_n0;
        locals.var_t2_dn2 = assign23130_e17770_d_n2;
        locals.var_t2_dn4 = assign23130_e17770_d_n4;
        locals.var_t2_dn5 = assign23130_e17770_d_n5;
        locals.var_t2_dn6 = assign23130_e17770_d_n6;
        locals.var_t2_dn7 = assign23130_e17770_d_n7;
        locals.var_t2_dn8 = assign23130_e17770_d_n8;
        locals.var_t2_dn9 = assign23130_e17770_d_n9;
        locals.var_t2_dn10 = assign23130_e17770_d_n10;
        locals.var_t2_dn11 = assign23130_e17770_d_n11;
        locals.var_t2_dn14 = assign23130_e17770_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23140_e17780, assign23140_e17780_d_n0, assign23140_e17780_d_n2, assign23140_e17780_d_n4, assign23140_e17780_d_n5, assign23140_e17780_d_n6, assign23140_e17780_d_n7, assign23140_e17780_d_n8, assign23140_e17780_d_n9, assign23140_e17780_d_n10, assign23140_e17780_d_n11, assign23140_e17780_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23140_e17774: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign23140_e17777: f64 = (locals.var_t2 * locals.var_cox_inv);
        let assign23140_e17778: f64 = (assign23140_e17774 + assign23140_e17777);
        (assign23140_e17778, (locals.var_pb2c_dn0 + ((locals.var_t2_dn0 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2_dn2 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn2))), (locals.var_pb2c_dn4 + ((locals.var_t2_dn4 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn4))), (locals.var_pb2c_dn5 + ((locals.var_t2_dn5 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn5))), (locals.var_pb2c_dn6 + ((locals.var_t2_dn6 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2_dn7 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn7))), (locals.var_pb2c_dn8 + ((locals.var_t2_dn8 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn8))), (locals.var_pb2c_dn9 + ((locals.var_t2_dn9 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn9))), (locals.var_pb2c_dn10 + ((locals.var_t2_dn10 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn10))), (locals.var_pb2c_dn11 + ((locals.var_t2_dn11 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn11))), (locals.var_pb2c_dn14 + ((locals.var_t2_dn14 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn14))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn4, locals.var_vth0_dn5, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn8, locals.var_vth0_dn9, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn14,)
    }
};
        locals.var_vth0 = assign23140_e17780;
        locals.var_vth0_dn0 = assign23140_e17780_d_n0;
        locals.var_vth0_dn2 = assign23140_e17780_d_n2;
        locals.var_vth0_dn4 = assign23140_e17780_d_n4;
        locals.var_vth0_dn5 = assign23140_e17780_d_n5;
        locals.var_vth0_dn6 = assign23140_e17780_d_n6;
        locals.var_vth0_dn7 = assign23140_e17780_d_n7;
        locals.var_vth0_dn8 = assign23140_e17780_d_n8;
        locals.var_vth0_dn9 = assign23140_e17780_d_n9;
        locals.var_vth0_dn10 = assign23140_e17780_d_n10;
        locals.var_vth0_dn11 = assign23140_e17780_d_n11;
        locals.var_vth0_dn14 = assign23140_e17780_d_n14;
        locals.var_vth0_rv = 0.0;

        let (assign23150_e17790, assign23150_e17790_d_n0, assign23150_e17790_d_n2, assign23150_e17790_d_n4, assign23150_e17790_d_n5, assign23150_e17790_d_n6, assign23150_e17790_d_n7, assign23150_e17790_d_n8, assign23150_e17790_d_n9, assign23150_e17790_d_n10, assign23150_e17790_d_n11, assign23150_e17790_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23150_e17784: f64 = (0.5 * locals.var_t1);
        let assign23150_e17786: f64 = (assign23150_e17784 / locals.var_t2);
        let assign23150_e17788: f64 = (assign23150_e17786 * locals.var_cox_inv);
        (assign23150_e17788, ((((((0.5 * locals.var_t1_dn0) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn0)), ((((((0.5 * locals.var_t1_dn2) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn2)), ((((((0.5 * locals.var_t1_dn4) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn4)), ((((((0.5 * locals.var_t1_dn5) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn5)), ((((((0.5 * locals.var_t1_dn6) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn6)), ((((((0.5 * locals.var_t1_dn7) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn7)), ((((((0.5 * locals.var_t1_dn8) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn8)), ((((((0.5 * locals.var_t1_dn9) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn9)), ((((((0.5 * locals.var_t1_dn10) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn10)), ((((((0.5 * locals.var_t1_dn11) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn11)), ((((((0.5 * locals.var_t1_dn14) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23150_e17790;
        locals.var_t3_dn0 = assign23150_e17790_d_n0;
        locals.var_t3_dn2 = assign23150_e17790_d_n2;
        locals.var_t3_dn4 = assign23150_e17790_d_n4;
        locals.var_t3_dn5 = assign23150_e17790_d_n5;
        locals.var_t3_dn6 = assign23150_e17790_d_n6;
        locals.var_t3_dn7 = assign23150_e17790_d_n7;
        locals.var_t3_dn8 = assign23150_e17790_d_n8;
        locals.var_t3_dn9 = assign23150_e17790_d_n9;
        locals.var_t3_dn10 = assign23150_e17790_d_n10;
        locals.var_t3_dn11 = assign23150_e17790_d_n11;
        locals.var_t3_dn14 = assign23150_e17790_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23160_e17796, assign23160_e17796_d_n0, assign23160_e17796_d_n2, assign23160_e17796_d_n4, assign23160_e17796_d_n5, assign23160_e17796_d_n6, assign23160_e17796_d_n7, assign23160_e17796_d_n8, assign23160_e17796_d_n9, assign23160_e17796_d_n10, assign23160_e17796_d_n11, assign23160_e17796_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23160_e17794: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign23160_e17794, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23160_e17796;
        locals.var_t1_dn0 = assign23160_e17796_d_n0;
        locals.var_t1_dn2 = assign23160_e17796_d_n2;
        locals.var_t1_dn4 = assign23160_e17796_d_n4;
        locals.var_t1_dn5 = assign23160_e17796_d_n5;
        locals.var_t1_dn6 = assign23160_e17796_d_n6;
        locals.var_t1_dn7 = assign23160_e17796_d_n7;
        locals.var_t1_dn8 = assign23160_e17796_d_n8;
        locals.var_t1_dn9 = assign23160_e17796_d_n9;
        locals.var_t1_dn10 = assign23160_e17796_d_n10;
        locals.var_t1_dn11 = assign23160_e17796_d_n11;
        locals.var_t1_dn14 = assign23160_e17796_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23170_e17800, assign23170_e17800_d_n0, assign23170_e17800_d_n2, assign23170_e17800_d_n4, assign23170_e17800_d_n5, assign23170_e17800_d_n6, assign23170_e17800_d_n7, assign23170_e17800_d_n8, assign23170_e17800_d_n9, assign23170_e17800_d_n10, assign23170_e17800_d_n11, assign23170_e17800_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23170_e17800;
        locals.var_t2_dn0 = assign23170_e17800_d_n0;
        locals.var_t2_dn2 = assign23170_e17800_d_n2;
        locals.var_t2_dn4 = assign23170_e17800_d_n4;
        locals.var_t2_dn5 = assign23170_e17800_d_n5;
        locals.var_t2_dn6 = assign23170_e17800_d_n6;
        locals.var_t2_dn7 = assign23170_e17800_d_n7;
        locals.var_t2_dn8 = assign23170_e17800_d_n8;
        locals.var_t2_dn9 = assign23170_e17800_d_n9;
        locals.var_t2_dn10 = assign23170_e17800_d_n10;
        locals.var_t2_dn11 = assign23170_e17800_d_n11;
        locals.var_t2_dn14 = assign23170_e17800_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23180_e17808, assign23180_e17808_d_n0, assign23180_e17808_d_n2, assign23180_e17808_d_n4, assign23180_e17808_d_n5, assign23180_e17808_d_n6, assign23180_e17808_d_n7, assign23180_e17808_d_n8, assign23180_e17808_d_n9, assign23180_e17808_d_n10, assign23180_e17808_d_n11, assign23180_e17808_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23180_e17805: f64 = (p.p140 * p.p140);
        let assign23180_e17806: f64 = (1.0 / assign23180_e17805);
        (assign23180_e17806, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23180_e17808;
        locals.var_t4_dn0 = assign23180_e17808_d_n0;
        locals.var_t4_dn2 = assign23180_e17808_d_n2;
        locals.var_t4_dn4 = assign23180_e17808_d_n4;
        locals.var_t4_dn5 = assign23180_e17808_d_n5;
        locals.var_t4_dn6 = assign23180_e17808_d_n6;
        locals.var_t4_dn7 = assign23180_e17808_d_n7;
        locals.var_t4_dn8 = assign23180_e17808_d_n8;
        locals.var_t4_dn9 = assign23180_e17808_d_n9;
        locals.var_t4_dn10 = assign23180_e17808_d_n10;
        locals.var_t4_dn11 = assign23180_e17808_d_n11;
        locals.var_t4_dn14 = assign23180_e17808_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23190_e17822, assign23190_e17822_d_n0, assign23190_e17822_d_n2, assign23190_e17822_d_n4, assign23190_e17822_d_n5, assign23190_e17822_d_n6, assign23190_e17822_d_n7, assign23190_e17822_d_n8, assign23190_e17822_d_n9, assign23190_e17822_d_n10, assign23190_e17822_d_n11, assign23190_e17822_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23190_e17813: f64 = (p.p137 - locals.var_pb20b);
        let assign23190_e17814: f64 = (2.0 * assign23190_e17813);
        let assign23190_e17816: f64 = (assign23190_e17814 * locals.var_t1);
        let assign23190_e17818: f64 = (assign23190_e17816 * locals.var_t2);
        let assign23190_e17820: f64 = (assign23190_e17818 * locals.var_t4);
        (assign23190_e17820, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn0)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn0)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn2)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn2)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn4)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn4)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn5)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn5)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn6)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn6)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn7)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn7)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn8)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn8)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn9)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn9)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn10)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn10)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn11)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn11)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn14)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn14)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23190_e17822;
        locals.var_t5_dn0 = assign23190_e17822_d_n0;
        locals.var_t5_dn2 = assign23190_e17822_d_n2;
        locals.var_t5_dn4 = assign23190_e17822_d_n4;
        locals.var_t5_dn5 = assign23190_e17822_d_n5;
        locals.var_t5_dn6 = assign23190_e17822_d_n6;
        locals.var_t5_dn7 = assign23190_e17822_d_n7;
        locals.var_t5_dn8 = assign23190_e17822_d_n8;
        locals.var_t5_dn9 = assign23190_e17822_d_n9;
        locals.var_t5_dn10 = assign23190_e17822_d_n10;
        locals.var_t5_dn11 = assign23190_e17822_d_n11;
        locals.var_t5_dn14 = assign23190_e17822_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23200_e17828, assign23200_e17828_d_n0, assign23200_e17828_d_n2, assign23200_e17828_d_n4, assign23200_e17828_d_n5, assign23200_e17828_d_n6, assign23200_e17828_d_n7, assign23200_e17828_d_n8, assign23200_e17828_d_n9, assign23200_e17828_d_n10, assign23200_e17828_d_n11, assign23200_e17828_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23200_e17826: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign23200_e17826, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign23200_e17828;
        locals.var_dvth0_dn0 = assign23200_e17828_d_n0;
        locals.var_dvth0_dn2 = assign23200_e17828_d_n2;
        locals.var_dvth0_dn4 = assign23200_e17828_d_n4;
        locals.var_dvth0_dn5 = assign23200_e17828_d_n5;
        locals.var_dvth0_dn6 = assign23200_e17828_d_n6;
        locals.var_dvth0_dn7 = assign23200_e17828_d_n7;
        locals.var_dvth0_dn8 = assign23200_e17828_d_n8;
        locals.var_dvth0_dn9 = assign23200_e17828_d_n9;
        locals.var_dvth0_dn10 = assign23200_e17828_d_n10;
        locals.var_dvth0_dn11 = assign23200_e17828_d_n11;
        locals.var_dvth0_dn14 = assign23200_e17828_d_n14;
        locals.var_dvth0_rv = 0.0;

        let (assign23210_e17836, assign23210_e17836_d_n0, assign23210_e17836_d_n2, assign23210_e17836_d_n4, assign23210_e17836_d_n5, assign23210_e17836_d_n6, assign23210_e17836_d_n7, assign23210_e17836_d_n8, assign23210_e17836_d_n9, assign23210_e17836_d_n10, assign23210_e17836_d_n11, assign23210_e17836_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23210_e17832: f64 = (0.5 * locals.var_t5);
        let assign23210_e17834: f64 = (assign23210_e17832 / locals.var_sqrt_pbsum);
        (assign23210_e17834, ((((0.5 * locals.var_t5_dn0) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn2) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn4) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn5) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn6) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn7) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn8) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn9) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn10) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn11) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn14) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23210_e17836;
        locals.var_t6_dn0 = assign23210_e17836_d_n0;
        locals.var_t6_dn2 = assign23210_e17836_d_n2;
        locals.var_t6_dn4 = assign23210_e17836_d_n4;
        locals.var_t6_dn5 = assign23210_e17836_d_n5;
        locals.var_t6_dn6 = assign23210_e17836_d_n6;
        locals.var_t6_dn7 = assign23210_e17836_d_n7;
        locals.var_t6_dn8 = assign23210_e17836_d_n8;
        locals.var_t6_dn9 = assign23210_e17836_d_n9;
        locals.var_t6_dn10 = assign23210_e17836_d_n10;
        locals.var_t6_dn11 = assign23210_e17836_d_n11;
        locals.var_t6_dn14 = assign23210_e17836_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23220_e17852, assign23220_e17852_d_n0, assign23220_e17852_d_n2, assign23220_e17852_d_n4, assign23220_e17852_d_n5, assign23220_e17852_d_n6, assign23220_e17852_d_n7, assign23220_e17852_d_n8, assign23220_e17852_d_n9, assign23220_e17852_d_n10, assign23220_e17852_d_n11, assign23220_e17852_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23220_e17841: f64 = (p.p137 - locals.var_pb20b);
        let assign23220_e17842: f64 = (2.0 * assign23220_e17841);
        let assign23220_e17844: f64 = (assign23220_e17842 * 1.034943e-10);
        let assign23220_e17846: f64 = (assign23220_e17844 * locals.var_t2);
        let assign23220_e17848: f64 = (assign23220_e17846 * locals.var_t4);
        let assign23220_e17850: f64 = (assign23220_e17848 * locals.var_sqrt_pbsum);
        (assign23220_e17850, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn0)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn2)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn4)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn5)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn6)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn7)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn8)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn9)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn10)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn11)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn14)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23220_e17852;
        locals.var_t7_dn0 = assign23220_e17852_d_n0;
        locals.var_t7_dn2 = assign23220_e17852_d_n2;
        locals.var_t7_dn4 = assign23220_e17852_d_n4;
        locals.var_t7_dn5 = assign23220_e17852_d_n5;
        locals.var_t7_dn6 = assign23220_e17852_d_n6;
        locals.var_t7_dn7 = assign23220_e17852_d_n7;
        locals.var_t7_dn8 = assign23220_e17852_d_n8;
        locals.var_t7_dn9 = assign23220_e17852_d_n9;
        locals.var_t7_dn10 = assign23220_e17852_d_n10;
        locals.var_t7_dn11 = assign23220_e17852_d_n11;
        locals.var_t7_dn14 = assign23220_e17852_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23230_e17865, assign23230_e17865_d_n0, assign23230_e17865_d_n2, assign23230_e17865_d_n4, assign23230_e17865_d_n5, assign23230_e17865_d_n6, assign23230_e17865_d_n7, assign23230_e17865_d_n8, assign23230_e17865_d_n9, assign23230_e17865_d_n10, assign23230_e17865_d_n11, assign23230_e17865_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23230_e17855: f64 = (-2.0);
        let assign23230_e17857: f64 = (assign23230_e17855 * locals.var_t1);
        let assign23230_e17859: f64 = (assign23230_e17857 * locals.var_t2);
        let assign23230_e17861: f64 = (assign23230_e17859 * locals.var_t4);
        let assign23230_e17863: f64 = (assign23230_e17861 * locals.var_sqrt_pbsum);
        (assign23230_e17863, (((((((assign23230_e17855 * locals.var_t1_dn0) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn0)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn0)), (((((((assign23230_e17855 * locals.var_t1_dn2) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn2)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn2)), (((((((assign23230_e17855 * locals.var_t1_dn4) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn4)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn4)), (((((((assign23230_e17855 * locals.var_t1_dn5) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn5)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn5)), (((((((assign23230_e17855 * locals.var_t1_dn6) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn6)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn6)), (((((((assign23230_e17855 * locals.var_t1_dn7) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn7)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn7)), (((((((assign23230_e17855 * locals.var_t1_dn8) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn8)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn8)), (((((((assign23230_e17855 * locals.var_t1_dn9) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn9)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn9)), (((((((assign23230_e17855 * locals.var_t1_dn10) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn10)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn10)), (((((((assign23230_e17855 * locals.var_t1_dn11) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn11)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn11)), (((((((assign23230_e17855 * locals.var_t1_dn14) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn14)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23230_e17865;
        locals.var_t8_dn0 = assign23230_e17865_d_n0;
        locals.var_t8_dn2 = assign23230_e17865_d_n2;
        locals.var_t8_dn4 = assign23230_e17865_d_n4;
        locals.var_t8_dn5 = assign23230_e17865_d_n5;
        locals.var_t8_dn6 = assign23230_e17865_d_n6;
        locals.var_t8_dn7 = assign23230_e17865_d_n7;
        locals.var_t8_dn8 = assign23230_e17865_d_n8;
        locals.var_t8_dn9 = assign23230_e17865_d_n9;
        locals.var_t8_dn10 = assign23230_e17865_d_n10;
        locals.var_t8_dn11 = assign23230_e17865_d_n11;
        locals.var_t8_dn14 = assign23230_e17865_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign23240_e17871, assign23240_e17871_d_n0, assign23240_e17871_d_n2, assign23240_e17871_d_n4, assign23240_e17871_d_n5, assign23240_e17871_d_n6, assign23240_e17871_d_n7, assign23240_e17871_d_n8, assign23240_e17871_d_n9, assign23240_e17871_d_n10, assign23240_e17871_d_n11, assign23240_e17871_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23240_e17869: f64 = (locals.var_vthp - locals.var_vth0);
        (assign23240_e17869, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn4 - locals.var_vth0_dn4), (locals.var_vthp_dn5 - locals.var_vth0_dn5), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn8 - locals.var_vth0_dn8), (locals.var_vthp_dn9 - locals.var_vth0_dn9), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn11 - locals.var_vth0_dn11), (locals.var_vthp_dn14 - locals.var_vth0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23240_e17871;
        locals.var_t1_dn0 = assign23240_e17871_d_n0;
        locals.var_t1_dn2 = assign23240_e17871_d_n2;
        locals.var_t1_dn4 = assign23240_e17871_d_n4;
        locals.var_t1_dn5 = assign23240_e17871_d_n5;
        locals.var_t1_dn6 = assign23240_e17871_d_n6;
        locals.var_t1_dn7 = assign23240_e17871_d_n7;
        locals.var_t1_dn8 = assign23240_e17871_d_n8;
        locals.var_t1_dn9 = assign23240_e17871_d_n9;
        locals.var_t1_dn10 = assign23240_e17871_d_n10;
        locals.var_t1_dn11 = assign23240_e17871_d_n11;
        locals.var_t1_dn14 = assign23240_e17871_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23250_e17881, assign23250_e17881_d_n0, assign23250_e17881_d_n2, assign23250_e17881_d_n4, assign23250_e17881_d_n5, assign23250_e17881_d_n6, assign23250_e17881_d_n7, assign23250_e17881_d_n8, assign23250_e17881_d_n9, assign23250_e17881_d_n10, assign23250_e17881_d_n11, assign23250_e17881_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23250_e17876: f64 = (locals.var_uc_scp3 * locals.var_pbsum);
        let assign23250_e17878: f64 = (assign23250_e17876 / p.p140);
        let assign23250_e17879: f64 = (locals.var_uc_scp1 + assign23250_e17878);
        (assign23250_e17879, ((locals.var_uc_scp3 * locals.var_pbsum_dn0) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn2) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn4) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn5) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn6) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn7) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn8) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn9) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn10) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn11) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn14) / p.p140),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23250_e17881;
        locals.var_t2_dn0 = assign23250_e17881_d_n0;
        locals.var_t2_dn2 = assign23250_e17881_d_n2;
        locals.var_t2_dn4 = assign23250_e17881_d_n4;
        locals.var_t2_dn5 = assign23250_e17881_d_n5;
        locals.var_t2_dn6 = assign23250_e17881_d_n6;
        locals.var_t2_dn7 = assign23250_e17881_d_n7;
        locals.var_t2_dn8 = assign23250_e17881_d_n8;
        locals.var_t2_dn9 = assign23250_e17881_d_n9;
        locals.var_t2_dn10 = assign23250_e17881_d_n10;
        locals.var_t2_dn11 = assign23250_e17881_d_n11;
        locals.var_t2_dn14 = assign23250_e17881_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23260_e17889, assign23260_e17889_d_n0, assign23260_e17889_d_n2, assign23260_e17889_d_n4, assign23260_e17889_d_n5, assign23260_e17889_d_n6, assign23260_e17889_d_n7, assign23260_e17889_d_n8, assign23260_e17889_d_n9, assign23260_e17889_d_n10, assign23260_e17889_d_n11, assign23260_e17889_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23260_e17886: f64 = (locals.var_uc_scp2 * locals.var_vdsz);
        let assign23260_e17887: f64 = (locals.var_t2 + assign23260_e17886);
        (assign23260_e17887, (locals.var_t2_dn0 + (locals.var_uc_scp2 * locals.var_vdsz_dn0)), (locals.var_t2_dn2 + (locals.var_uc_scp2 * locals.var_vdsz_dn2)), (locals.var_t2_dn4 + (locals.var_uc_scp2 * locals.var_vdsz_dn4)), (locals.var_t2_dn5 + (locals.var_uc_scp2 * locals.var_vdsz_dn5)), (locals.var_t2_dn6 + (locals.var_uc_scp2 * locals.var_vdsz_dn6)), (locals.var_t2_dn7 + (locals.var_uc_scp2 * locals.var_vdsz_dn7)), (locals.var_t2_dn8 + (locals.var_uc_scp2 * locals.var_vdsz_dn8)), (locals.var_t2_dn9 + (locals.var_uc_scp2 * locals.var_vdsz_dn9)), (locals.var_t2_dn10 + (locals.var_uc_scp2 * locals.var_vdsz_dn10)), (locals.var_t2_dn11 + (locals.var_uc_scp2 * locals.var_vdsz_dn11)), (locals.var_t2_dn14 + (locals.var_uc_scp2 * locals.var_vdsz_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23260_e17889;
        locals.var_t3_dn0 = assign23260_e17889_d_n0;
        locals.var_t3_dn2 = assign23260_e17889_d_n2;
        locals.var_t3_dn4 = assign23260_e17889_d_n4;
        locals.var_t3_dn5 = assign23260_e17889_d_n5;
        locals.var_t3_dn6 = assign23260_e17889_d_n6;
        locals.var_t3_dn7 = assign23260_e17889_d_n7;
        locals.var_t3_dn8 = assign23260_e17889_d_n8;
        locals.var_t3_dn9 = assign23260_e17889_d_n9;
        locals.var_t3_dn10 = assign23260_e17889_d_n10;
        locals.var_t3_dn11 = assign23260_e17889_d_n11;
        locals.var_t3_dn14 = assign23260_e17889_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23270_e17895, assign23270_e17895_d_n0, assign23270_e17895_d_n2, assign23270_e17895_d_n4, assign23270_e17895_d_n5, assign23270_e17895_d_n6, assign23270_e17895_d_n7, assign23270_e17895_d_n8, assign23270_e17895_d_n9, assign23270_e17895_d_n10, assign23270_e17895_d_n11, assign23270_e17895_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23270_e17893: f64 = (p.p221 + locals.var_vdsz);
        (assign23270_e17893, locals.var_vdsz_dn0, locals.var_vdsz_dn2, locals.var_vdsz_dn4, locals.var_vdsz_dn5, locals.var_vdsz_dn6, locals.var_vdsz_dn7, locals.var_vdsz_dn8, locals.var_vdsz_dn9, locals.var_vdsz_dn10, locals.var_vdsz_dn11, locals.var_vdsz_dn14,)
    } else {
        (locals.var_vdx, locals.var_vdx_dn0, locals.var_vdx_dn2, locals.var_vdx_dn4, locals.var_vdx_dn5, locals.var_vdx_dn6, locals.var_vdx_dn7, locals.var_vdx_dn8, locals.var_vdx_dn9, locals.var_vdx_dn10, locals.var_vdx_dn11, locals.var_vdx_dn14,)
    }
};
        locals.var_vdx = assign23270_e17895;
        locals.var_vdx_dn0 = assign23270_e17895_d_n0;
        locals.var_vdx_dn2 = assign23270_e17895_d_n2;
        locals.var_vdx_dn4 = assign23270_e17895_d_n4;
        locals.var_vdx_dn5 = assign23270_e17895_d_n5;
        locals.var_vdx_dn6 = assign23270_e17895_d_n6;
        locals.var_vdx_dn7 = assign23270_e17895_d_n7;
        locals.var_vdx_dn8 = assign23270_e17895_d_n8;
        locals.var_vdx_dn9 = assign23270_e17895_d_n9;
        locals.var_vdx_dn10 = assign23270_e17895_d_n10;
        locals.var_vdx_dn11 = assign23270_e17895_d_n11;
        locals.var_vdx_dn14 = assign23270_e17895_d_n14;
        locals.var_vdx_rv = 0.0;

        let (assign23280_e17901, assign23280_e17901_d_n0, assign23280_e17901_d_n2, assign23280_e17901_d_n4, assign23280_e17901_d_n5, assign23280_e17901_d_n6, assign23280_e17901_d_n7, assign23280_e17901_d_n8, assign23280_e17901_d_n9, assign23280_e17901_d_n10, assign23280_e17901_d_n11, assign23280_e17901_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23280_e17899: f64 = (locals.var_vdx * locals.var_vdx);
        (assign23280_e17899, ((locals.var_vdx_dn0 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn0)), ((locals.var_vdx_dn2 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn2)), ((locals.var_vdx_dn4 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn4)), ((locals.var_vdx_dn5 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn5)), ((locals.var_vdx_dn6 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn6)), ((locals.var_vdx_dn7 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn7)), ((locals.var_vdx_dn8 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn8)), ((locals.var_vdx_dn9 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn9)), ((locals.var_vdx_dn10 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn10)), ((locals.var_vdx_dn11 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn11)), ((locals.var_vdx_dn14 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn14)),)
    } else {
        (locals.var_vdx2, locals.var_vdx2_dn0, locals.var_vdx2_dn2, locals.var_vdx2_dn4, locals.var_vdx2_dn5, locals.var_vdx2_dn6, locals.var_vdx2_dn7, locals.var_vdx2_dn8, locals.var_vdx2_dn9, locals.var_vdx2_dn10, locals.var_vdx2_dn11, locals.var_vdx2_dn14,)
    }
};
        locals.var_vdx2 = assign23280_e17901;
        locals.var_vdx2_dn0 = assign23280_e17901_d_n0;
        locals.var_vdx2_dn2 = assign23280_e17901_d_n2;
        locals.var_vdx2_dn4 = assign23280_e17901_d_n4;
        locals.var_vdx2_dn5 = assign23280_e17901_d_n5;
        locals.var_vdx2_dn6 = assign23280_e17901_d_n6;
        locals.var_vdx2_dn7 = assign23280_e17901_d_n7;
        locals.var_vdx2_dn8 = assign23280_e17901_d_n8;
        locals.var_vdx2_dn9 = assign23280_e17901_d_n9;
        locals.var_vdx2_dn10 = assign23280_e17901_d_n10;
        locals.var_vdx2_dn11 = assign23280_e17901_d_n11;
        locals.var_vdx2_dn14 = assign23280_e17901_d_n14;
        locals.var_vdx2_rv = 0.0;

        let (assign23290_e17915, assign23290_e17915_d_n0, assign23290_e17915_d_n2, assign23290_e17915_d_n4, assign23290_e17915_d_n5, assign23290_e17915_d_n6, assign23290_e17915_d_n7, assign23290_e17915_d_n8, assign23290_e17915_d_n9, assign23290_e17915_d_n10, assign23290_e17915_d_n11, assign23290_e17915_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23290_e17905: f64 = (locals.var_t1 * locals.var_dvth0);
        let assign23290_e17907: f64 = (assign23290_e17905 * locals.var_t3);
        let assign23290_e17909: f64 = (assign23290_e17907 + locals.var_dqb);
        let assign23290_e17912: f64 = (locals.var_msc / locals.var_vdx2);
        let assign23290_e17913: f64 = (assign23290_e17909 - assign23290_e17912);
        (assign23290_e17913, ((((((locals.var_t1_dn0 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn0)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn0)) + locals.var_dqb_dn0) - (-((locals.var_msc * locals.var_vdx2_dn0) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn2 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn2)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn2)) + locals.var_dqb_dn2) - (-((locals.var_msc * locals.var_vdx2_dn2) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn4 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn4)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn4)) + locals.var_dqb_dn4) - (-((locals.var_msc * locals.var_vdx2_dn4) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn5 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn5)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn5)) + locals.var_dqb_dn5) - (-((locals.var_msc * locals.var_vdx2_dn5) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn6 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn6)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn6)) + locals.var_dqb_dn6) - (-((locals.var_msc * locals.var_vdx2_dn6) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn7 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn7)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn7)) + locals.var_dqb_dn7) - (-((locals.var_msc * locals.var_vdx2_dn7) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn8 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn8)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn8)) + locals.var_dqb_dn8) - (-((locals.var_msc * locals.var_vdx2_dn8) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn9 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn9)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn9)) + locals.var_dqb_dn9) - (-((locals.var_msc * locals.var_vdx2_dn9) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn10 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn10)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn10)) + locals.var_dqb_dn10) - (-((locals.var_msc * locals.var_vdx2_dn10) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn11 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn11)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn11)) + locals.var_dqb_dn11) - (-((locals.var_msc * locals.var_vdx2_dn11) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn14 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn14)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn14)) + locals.var_dqb_dn14) - (-((locals.var_msc * locals.var_vdx2_dn14) / (locals.var_vdx2 * locals.var_vdx2)))),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23290_e17915;
        locals.var_dvthlp_dn0 = assign23290_e17915_d_n0;
        locals.var_dvthlp_dn2 = assign23290_e17915_d_n2;
        locals.var_dvthlp_dn4 = assign23290_e17915_d_n4;
        locals.var_dvthlp_dn5 = assign23290_e17915_d_n5;
        locals.var_dvthlp_dn6 = assign23290_e17915_d_n6;
        locals.var_dvthlp_dn7 = assign23290_e17915_d_n7;
        locals.var_dvthlp_dn8 = assign23290_e17915_d_n8;
        locals.var_dvthlp_dn9 = assign23290_e17915_d_n9;
        locals.var_dvthlp_dn10 = assign23290_e17915_d_n10;
        locals.var_dvthlp_dn11 = assign23290_e17915_d_n11;
        locals.var_dvthlp_dn14 = assign23290_e17915_d_n14;
        locals.var_dvthlp_rv = 0.0;

        let (assign23300_e17920, assign23300_e17920_d_n0, assign23300_e17920_d_n2, assign23300_e17920_d_n4, assign23300_e17920_d_n5, assign23300_e17920_d_n6, assign23300_e17920_d_n7, assign23300_e17920_d_n8, assign23300_e17920_d_n9, assign23300_e17920_d_n10, assign23300_e17920_d_n11, assign23300_e17920_d_n14,) = {
    if (locals.var_guard432 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23300_e17920;
        locals.var_dvthlp_dn0 = assign23300_e17920_d_n0;
        locals.var_dvthlp_dn2 = assign23300_e17920_d_n2;
        locals.var_dvthlp_dn4 = assign23300_e17920_d_n4;
        locals.var_dvthlp_dn5 = assign23300_e17920_d_n5;
        locals.var_dvthlp_dn6 = assign23300_e17920_d_n6;
        locals.var_dvthlp_dn7 = assign23300_e17920_d_n7;
        locals.var_dvthlp_dn8 = assign23300_e17920_d_n8;
        locals.var_dvthlp_dn9 = assign23300_e17920_d_n9;
        locals.var_dvthlp_dn10 = assign23300_e17920_d_n10;
        locals.var_dvthlp_dn11 = assign23300_e17920_d_n11;
        locals.var_dvthlp_dn14 = assign23300_e17920_d_n14;
        locals.var_dvthlp_rv = 0.0;

        let assign23310_e17923: f64 = (1.034943e-10 * locals.var_cox_inv);
        locals.var_t1 = assign23310_e17923;
        locals.var_t1_dn0 = (1.034943e-10 * locals.var_cox_inv_dn0);
        locals.var_t1_dn2 = (1.034943e-10 * locals.var_cox_inv_dn2);
        locals.var_t1_dn4 = (1.034943e-10 * locals.var_cox_inv_dn4);
        locals.var_t1_dn5 = (1.034943e-10 * locals.var_cox_inv_dn5);
        locals.var_t1_dn6 = (1.034943e-10 * locals.var_cox_inv_dn6);
        locals.var_t1_dn7 = (1.034943e-10 * locals.var_cox_inv_dn7);
        locals.var_t1_dn8 = (1.034943e-10 * locals.var_cox_inv_dn8);
        locals.var_t1_dn9 = (1.034943e-10 * locals.var_cox_inv_dn9);
        locals.var_t1_dn10 = (1.034943e-10 * locals.var_cox_inv_dn10);
        locals.var_t1_dn11 = (1.034943e-10 * locals.var_cox_inv_dn11);
        locals.var_t1_dn14 = (1.034943e-10 * locals.var_cox_inv_dn14);
        locals.var_t1_rv = 0.0;

        locals.var_t2 = locals.var_wdpl;
        locals.var_t2_dn0 = locals.var_wdpl_dn0;
        locals.var_t2_dn2 = locals.var_wdpl_dn2;
        locals.var_t2_dn4 = locals.var_wdpl_dn4;
        locals.var_t2_dn5 = locals.var_wdpl_dn5;
        locals.var_t2_dn6 = locals.var_wdpl_dn6;
        locals.var_t2_dn7 = locals.var_wdpl_dn7;
        locals.var_t2_dn8 = locals.var_wdpl_dn8;
        locals.var_t2_dn9 = locals.var_wdpl_dn9;
        locals.var_t2_dn10 = locals.var_wdpl_dn10;
        locals.var_t2_dn11 = locals.var_wdpl_dn11;
        locals.var_t2_dn14 = locals.var_wdpl_dn14;
        locals.var_t2_rv = 0.0;

        let assign23330_e17927: f64 = (locals.var_lgate - p.p139);
        locals.var_t3 = assign23330_e17927;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign23340_e17931: f64 = (locals.var_t3 * locals.var_t3);
        let assign23340_e17932: f64 = (1.0 / assign23340_e17931);
        locals.var_t4 = assign23340_e17932;
        locals.var_t4_dn0 = (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn2 = (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn4 = (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn5 = (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn6 = (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn7 = (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn8 = (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn9 = (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn10 = (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn11 = (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn14 = (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_rv = 0.0;

        let assign23350_e17936: f64 = (p.p137 - locals.var_pb20b);
        let assign23350_e17937: f64 = (2.0 * assign23350_e17936);
        let assign23350_e17939: f64 = (assign23350_e17937 * locals.var_t1);
        let assign23350_e17941: f64 = (assign23350_e17939 * locals.var_t2);
        let assign23350_e17943: f64 = (assign23350_e17941 * locals.var_t4);
        locals.var_t5 = assign23350_e17943;
        locals.var_t5_dn0 = (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn0)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn0)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn0));
        locals.var_t5_dn2 = (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn2)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn2)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn2));
        locals.var_t5_dn4 = (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn4)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn4)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn4));
        locals.var_t5_dn5 = (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn5)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn5)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn5));
        locals.var_t5_dn6 = (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn6)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn6)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn6));
        locals.var_t5_dn7 = (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn7)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn7)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn7));
        locals.var_t5_dn8 = (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn8)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn8)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn8));
        locals.var_t5_dn9 = (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn9)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn9)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn9));
        locals.var_t5_dn10 = (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn10)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn10)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn10));
        locals.var_t5_dn11 = (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn11)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn11)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn11));
        locals.var_t5_dn14 = (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn14)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn14)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn14));
        locals.var_t5_rv = 0.0;

        let assign23360_e17946: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        locals.var_dvth0 = assign23360_e17946;
        locals.var_dvth0_dn0 = ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0));
        locals.var_dvth0_dn2 = ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2));
        locals.var_dvth0_dn4 = ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4));
        locals.var_dvth0_dn5 = ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5));
        locals.var_dvth0_dn6 = ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6));
        locals.var_dvth0_dn7 = ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7));
        locals.var_dvth0_dn8 = ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8));
        locals.var_dvth0_dn9 = ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9));
        locals.var_dvth0_dn10 = ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10));
        locals.var_dvth0_dn11 = ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11));
        locals.var_dvth0_dn14 = ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14));
        locals.var_dvth0_rv = 0.0;

        let assign23370_e17949: f64 = (locals.var_t5 / 2.0);
        let assign23370_e17951: f64 = (assign23370_e17949 / locals.var_sqrt_pbsum);
        locals.var_t6 = assign23370_e17951;
        locals.var_t6_dn0 = ((((locals.var_t5_dn0 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn2 = ((((locals.var_t5_dn2 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn4 = ((((locals.var_t5_dn4 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn5 = ((((locals.var_t5_dn5 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn6 = ((((locals.var_t5_dn6 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn7 = ((((locals.var_t5_dn7 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn8 = ((((locals.var_t5_dn8 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn9 = ((((locals.var_t5_dn9 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn10 = ((((locals.var_t5_dn10 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn11 = ((((locals.var_t5_dn11 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn14 = ((((locals.var_t5_dn14 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_rv = 0.0;

        let assign23380_e17955: f64 = (p.p137 - locals.var_pb20b);
        let assign23380_e17956: f64 = (2.0 * assign23380_e17955);
        let assign23380_e17958: f64 = (assign23380_e17956 * 1.034943e-10);
        let assign23380_e17960: f64 = (assign23380_e17958 * locals.var_t2);
        let assign23380_e17962: f64 = (assign23380_e17960 * locals.var_t4);
        let assign23380_e17964: f64 = (assign23380_e17962 * locals.var_sqrt_pbsum);
        locals.var_t7 = assign23380_e17964;
        locals.var_t7_dn0 = ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn0)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn0));
        locals.var_t7_dn2 = ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn2)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn2));
        locals.var_t7_dn4 = ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn4)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn4));
        locals.var_t7_dn5 = ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn5)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn5));
        locals.var_t7_dn6 = ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn6)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn6));
        locals.var_t7_dn7 = ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn7)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn7));
        locals.var_t7_dn8 = ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn8)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn8));
        locals.var_t7_dn9 = ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn9)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn9));
        locals.var_t7_dn10 = ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn10)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn10));
        locals.var_t7_dn11 = ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn11)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn11));
        locals.var_t7_dn14 = ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn14)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn14));
        locals.var_t7_rv = 0.0;

        let assign23390_e17966: f64 = (-2.0);
        let assign23390_e17968: f64 = (assign23390_e17966 * locals.var_t1);
        let assign23390_e17970: f64 = (assign23390_e17968 * locals.var_t2);
        let assign23390_e17972: f64 = (assign23390_e17970 * locals.var_t4);
        let assign23390_e17974: f64 = (assign23390_e17972 * locals.var_sqrt_pbsum);
        locals.var_t8 = assign23390_e17974;
        locals.var_t8_dn0 = (((((((assign23390_e17966 * locals.var_t1_dn0) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn0)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn0));
        locals.var_t8_dn2 = (((((((assign23390_e17966 * locals.var_t1_dn2) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn2)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn2));
        locals.var_t8_dn4 = (((((((assign23390_e17966 * locals.var_t1_dn4) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn4)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn4));
        locals.var_t8_dn5 = (((((((assign23390_e17966 * locals.var_t1_dn5) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn5)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn5));
        locals.var_t8_dn6 = (((((((assign23390_e17966 * locals.var_t1_dn6) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn6)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn6));
        locals.var_t8_dn7 = (((((((assign23390_e17966 * locals.var_t1_dn7) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn7)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn7));
        locals.var_t8_dn8 = (((((((assign23390_e17966 * locals.var_t1_dn8) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn8)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn8));
        locals.var_t8_dn9 = (((((((assign23390_e17966 * locals.var_t1_dn9) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn9)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn9));
        locals.var_t8_dn10 = (((((((assign23390_e17966 * locals.var_t1_dn10) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn10)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn10));
        locals.var_t8_dn11 = (((((((assign23390_e17966 * locals.var_t1_dn11) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn11)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn11));
        locals.var_t8_dn14 = (((((((assign23390_e17966 * locals.var_t1_dn14) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn14)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn14));
        locals.var_t8_rv = 0.0;

        let assign23400_e17977: f64 = (locals.var_uc_sc3 / locals.var_lgate);
        locals.var_t1 = assign23400_e17977;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign23410_e17981: f64 = (locals.var_t1 * locals.var_pbsum);
        let assign23410_e17982: f64 = (locals.var_uc_sc1 + assign23410_e17981);
        locals.var_t4 = assign23410_e17982;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn6));
        locals.var_t4_dn7 = ((locals.var_t1_dn7 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn7));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn8));
        locals.var_t4_dn9 = ((locals.var_t1_dn9 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn9));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn10));
        locals.var_t4_dn11 = ((locals.var_t1_dn11 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn11));
        locals.var_t4_dn14 = ((locals.var_t1_dn14 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn14));
        locals.var_t4_rv = 0.0;

        let assign23420_e17986: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign23420_e17990: f64 = (p.p150 * locals.var_pbsum);
        let assign23420_e17991: f64 = (1.0 + assign23420_e17990);
        let assign23420_e17992: f64 = (assign23420_e17986 * assign23420_e17991);
        let assign23420_e17993: f64 = (locals.var_t4 + assign23420_e17992);
        locals.var_t5 = assign23420_e17993;
        locals.var_t5_dn0 = (locals.var_t4_dn0 + (((locals.var_uc_sc2 * locals.var_vdsz_dn0) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn0))));
        locals.var_t5_dn2 = (locals.var_t4_dn2 + (((locals.var_uc_sc2 * locals.var_vdsz_dn2) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn2))));
        locals.var_t5_dn4 = (locals.var_t4_dn4 + (((locals.var_uc_sc2 * locals.var_vdsz_dn4) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn4))));
        locals.var_t5_dn5 = (locals.var_t4_dn5 + (((locals.var_uc_sc2 * locals.var_vdsz_dn5) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn5))));
        locals.var_t5_dn6 = (locals.var_t4_dn6 + (((locals.var_uc_sc2 * locals.var_vdsz_dn6) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn6))));
        locals.var_t5_dn7 = (locals.var_t4_dn7 + (((locals.var_uc_sc2 * locals.var_vdsz_dn7) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn7))));
        locals.var_t5_dn8 = (locals.var_t4_dn8 + (((locals.var_uc_sc2 * locals.var_vdsz_dn8) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn8))));
        locals.var_t5_dn9 = (locals.var_t4_dn9 + (((locals.var_uc_sc2 * locals.var_vdsz_dn9) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn9))));
        locals.var_t5_dn10 = (locals.var_t4_dn10 + (((locals.var_uc_sc2 * locals.var_vdsz_dn10) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn10))));
        locals.var_t5_dn11 = (locals.var_t4_dn11 + (((locals.var_uc_sc2 * locals.var_vdsz_dn11) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn11))));
        locals.var_t5_dn14 = (locals.var_t4_dn14 + (((locals.var_uc_sc2 * locals.var_vdsz_dn14) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn14))));
        locals.var_t5_rv = 0.0;

        let assign23430_e17996: f64 = (locals.var_dvth0 * locals.var_t5);
        locals.var_dvthsc = assign23430_e17996;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0_dn0 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0_dn2 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn2));
        locals.var_dvthsc_dn4 = ((locals.var_dvth0_dn4 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn4));
        locals.var_dvthsc_dn5 = ((locals.var_dvth0_dn5 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn5));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0_dn6 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0_dn7 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn7));
        locals.var_dvthsc_dn8 = ((locals.var_dvth0_dn8 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn8));
        locals.var_dvthsc_dn9 = ((locals.var_dvth0_dn9 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn9));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0_dn10 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0_dn11 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn11));
        locals.var_dvthsc_dn14 = ((locals.var_dvth0_dn14 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn14));
        locals.var_dvthsc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign23440_e17999: f64 = (1.0 / locals.var_cox);
        locals.var_t1 = assign23440_e17999;
        locals.var_t1_dn0 = (-(locals.var_cox_dn0 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn2 = (-(locals.var_cox_dn2 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn4 = (-(locals.var_cox_dn4 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn5 = (-(locals.var_cox_dn5 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn6 = (-(locals.var_cox_dn6 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn7 = (-(locals.var_cox_dn7 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn8 = (-(locals.var_cox_dn8 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn9 = (-(locals.var_cox_dn9 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn10 = (-(locals.var_cox_dn10 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn11 = (-(locals.var_cox_dn11 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn14 = (-(locals.var_cox_dn14 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_rv = 0.0;

        let assign23450_e18002: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign23450_e18002;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let assign23460_e18007: f64 = (locals.var_uc_wfc / locals.var_weff);
        let assign23460_e18008: f64 = (locals.var_cox + assign23460_e18007);
        let assign23460_e18009: f64 = (1.0 / assign23460_e18008);
        locals.var_t3 = assign23460_e18009;
        locals.var_t3_dn0 = (-(locals.var_cox_dn0 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn2 = (-(locals.var_cox_dn2 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn4 = (-(locals.var_cox_dn4 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn5 = (-(locals.var_cox_dn5 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn6 = (-(locals.var_cox_dn6 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn7 = (-(locals.var_cox_dn7 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn8 = (-(locals.var_cox_dn8 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn9 = (-(locals.var_cox_dn9 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn10 = (-(locals.var_cox_dn10 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn11 = (-(locals.var_cox_dn11 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn14 = (-(locals.var_cox_dn14 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_rv = 0.0;

        let assign23470_e18012: f64 = (locals.var_t3 * locals.var_t3);
        locals.var_t4 = assign23470_e18012;
        locals.var_t4_dn0 = ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6));
        locals.var_t4_dn7 = ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7));
        locals.var_t4_dn8 = ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8));
        locals.var_t4_dn9 = ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9));
        locals.var_t4_dn10 = ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10));
        locals.var_t4_dn11 = ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11));
        locals.var_t4_dn14 = ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14));
        locals.var_t4_rv = 0.0;

        let assign23480_e18015: f64 = (locals.var_t1 - locals.var_t3);
        locals.var_t5 = assign23480_e18015;
        locals.var_t5_dn0 = (locals.var_t1_dn0 - locals.var_t3_dn0);
        locals.var_t5_dn2 = (locals.var_t1_dn2 - locals.var_t3_dn2);
        locals.var_t5_dn4 = (locals.var_t1_dn4 - locals.var_t3_dn4);
        locals.var_t5_dn5 = (locals.var_t1_dn5 - locals.var_t3_dn5);
        locals.var_t5_dn6 = (locals.var_t1_dn6 - locals.var_t3_dn6);
        locals.var_t5_dn7 = (locals.var_t1_dn7 - locals.var_t3_dn7);
        locals.var_t5_dn8 = (locals.var_t1_dn8 - locals.var_t3_dn8);
        locals.var_t5_dn9 = (locals.var_t1_dn9 - locals.var_t3_dn9);
        locals.var_t5_dn10 = (locals.var_t1_dn10 - locals.var_t3_dn10);
        locals.var_t5_dn11 = (locals.var_t1_dn11 - locals.var_t3_dn11);
        locals.var_t5_dn14 = (locals.var_t1_dn14 - locals.var_t3_dn14);
        locals.var_t5_rv = 0.0;

        let assign23490_e18019: f64 = (locals.var_t2 - locals.var_t4);
        let assign23490_e18020: f64 = (locals.var_qb0 * assign23490_e18019);
        locals.var_t6 = assign23490_e18020;
        locals.var_t6_dn0 = ((locals.var_qb0_dn0 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn0 - locals.var_t4_dn0)));
        locals.var_t6_dn2 = ((locals.var_qb0_dn2 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn2 - locals.var_t4_dn2)));
        locals.var_t6_dn4 = ((locals.var_qb0_dn4 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn4 - locals.var_t4_dn4)));
        locals.var_t6_dn5 = ((locals.var_qb0_dn5 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn5 - locals.var_t4_dn5)));
        locals.var_t6_dn6 = ((locals.var_qb0_dn6 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn6 - locals.var_t4_dn6)));
        locals.var_t6_dn7 = ((locals.var_qb0_dn7 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn7 - locals.var_t4_dn7)));
        locals.var_t6_dn8 = ((locals.var_qb0_dn8 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn8 - locals.var_t4_dn8)));
        locals.var_t6_dn9 = ((locals.var_qb0_dn9 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn9 - locals.var_t4_dn9)));
        locals.var_t6_dn10 = ((locals.var_qb0_dn10 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn10 - locals.var_t4_dn10)));
        locals.var_t6_dn11 = ((locals.var_qb0_dn11 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn11 - locals.var_t4_dn11)));
        locals.var_t6_dn14 = ((locals.var_qb0_dn14 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn14 - locals.var_t4_dn14)));
        locals.var_t6_rv = 0.0;

        let assign23500_e18023: f64 = (locals.var_qb0 * locals.var_t5);
        let assign23500_e18026: f64 = (locals.var_uc_wvth0 / locals.var_wg);
        let assign23500_e18027: f64 = (assign23500_e18023 + assign23500_e18026);
        locals.var_dvthw = assign23500_e18027;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn2));
        locals.var_dvthw_dn4 = ((locals.var_qb0_dn4 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn4));
        locals.var_dvthw_dn5 = ((locals.var_qb0_dn5 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn5));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn7));
        locals.var_dvthw_dn8 = ((locals.var_qb0_dn8 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn8));
        locals.var_dvthw_dn9 = ((locals.var_qb0_dn9 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn9));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn11));
        locals.var_dvthw_dn14 = ((locals.var_qb0_dn14 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn14));
        locals.var_dvthw_rv = 0.0;

        let assign23510_e18030: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign23510_e18032: f64 = (assign23510_e18030 + locals.var_dvthw);
        let assign23510_e18034: f64 = (assign23510_e18032 + locals.var_dvthsm);
        locals.var_dvth = assign23510_e18034;
        locals.var_dvth_dn0 = ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0);
        locals.var_dvth_dn2 = ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2);
        locals.var_dvth_dn4 = ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) + locals.var_dvthw_dn4);
        locals.var_dvth_dn5 = ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) + locals.var_dvthw_dn5);
        locals.var_dvth_dn6 = ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6);
        locals.var_dvth_dn7 = ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7);
        locals.var_dvth_dn8 = ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) + locals.var_dvthw_dn8);
        locals.var_dvth_dn9 = ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) + locals.var_dvthw_dn9);
        locals.var_dvth_dn10 = ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10);
        locals.var_dvth_dn11 = ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11);
        locals.var_dvth_dn14 = ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) + locals.var_dvthw_dn14);
        locals.var_dvth_rv = 0.0;

        let assign23520_e18038: f64 = (locals.var_pb2 - locals.var_vbsz);
        let assign23520_e18039: f64 = (locals.var_qnsub_esi2 * assign23520_e18038);
        let assign23520_e18040: f64 = (assign23520_e18039).sqrt();
        locals.var_t2 = assign23520_e18040;
        locals.var_t2_dn0 = (((locals.var_qnsub_esi2_dn0 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn0 - locals.var_vbsz_dn0))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn2 = (((locals.var_qnsub_esi2_dn2 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn2 - locals.var_vbsz_dn2))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn4 = (((locals.var_qnsub_esi2_dn4 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn4 - locals.var_vbsz_dn4))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn5 = (((locals.var_qnsub_esi2_dn5 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn5 - locals.var_vbsz_dn5))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn6 = (((locals.var_qnsub_esi2_dn6 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn6 - locals.var_vbsz_dn6))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn7 = (((locals.var_qnsub_esi2_dn7 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn7 - locals.var_vbsz_dn7))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn8 = (((locals.var_qnsub_esi2_dn8 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn8 - locals.var_vbsz_dn8))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn9 = (((locals.var_qnsub_esi2_dn9 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn9 - locals.var_vbsz_dn9))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn10 = (((locals.var_qnsub_esi2_dn10 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn10 - locals.var_vbsz_dn10))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn11 = (((locals.var_qnsub_esi2_dn11 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn11 - locals.var_vbsz_dn11))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn14 = (((locals.var_qnsub_esi2_dn14 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn14 - locals.var_vbsz_dn14))) / (2.0 * assign23520_e18040));
        locals.var_t2_rv = 0.0;

        let assign23530_e18043: f64 = (locals.var_pb2 + locals.var_vfb);
        let assign23530_e18046: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign23530_e18047: f64 = (assign23530_e18043 + assign23530_e18046);
        let assign23530_e18049: f64 = (assign23530_e18047 - locals.var_dvth);
        locals.var_vth = assign23530_e18049;
        locals.var_vth_dn0 = ((locals.var_pb2_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv)) - locals.var_dvth_dn0);
        locals.var_vth_dn2 = ((locals.var_pb2_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv)) - locals.var_dvth_dn2);
        locals.var_vth_dn4 = ((locals.var_pb2_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv)) - locals.var_dvth_dn4);
        locals.var_vth_dn5 = ((locals.var_pb2_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv)) - locals.var_dvth_dn5);
        locals.var_vth_dn6 = ((locals.var_pb2_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv)) - locals.var_dvth_dn6);
        locals.var_vth_dn7 = ((locals.var_pb2_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv)) - locals.var_dvth_dn7);
        locals.var_vth_dn8 = ((locals.var_pb2_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv)) - locals.var_dvth_dn8);
        locals.var_vth_dn9 = ((locals.var_pb2_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv)) - locals.var_dvth_dn9);
        locals.var_vth_dn10 = ((locals.var_pb2_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv)) - locals.var_dvth_dn10);
        locals.var_vth_dn11 = ((locals.var_pb2_dn11 + (locals.var_t2_dn11 * locals.var_cox0_inv)) - locals.var_dvth_dn11);
        locals.var_vth_dn14 = ((locals.var_pb2_dn14 + (locals.var_t2_dn14 * locals.var_cox0_inv)) - locals.var_dvth_dn14);
        locals.var_vth_rv = 0.0;

        let assign23540_e18052: f64 = (locals.var_cnst0 * locals.var_cox_inv);
        locals.var_fac1 = assign23540_e18052;
        locals.var_fac1_dn0 = ((locals.var_cnst0_dn0 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0_dn2 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn2));
        locals.var_fac1_dn4 = ((locals.var_cnst0_dn4 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn4));
        locals.var_fac1_dn5 = ((locals.var_cnst0_dn5 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn5));
        locals.var_fac1_dn6 = ((locals.var_cnst0_dn6 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0_dn7 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn7));
        locals.var_fac1_dn8 = ((locals.var_cnst0_dn8 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn8));
        locals.var_fac1_dn9 = ((locals.var_cnst0_dn9 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn9));
        locals.var_fac1_dn10 = ((locals.var_cnst0_dn10 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0_dn11 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn11));
        locals.var_fac1_dn14 = ((locals.var_cnst0_dn14 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn14));
        locals.var_fac1_rv = 0.0;

        let assign23550_e18055: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign23550_e18055;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn4 = ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4));
        locals.var_fac1p2_dn5 = ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn8 = ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8));
        locals.var_fac1p2_dn9 = ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn14 = ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14));
        locals.var_fac1p2_rv = 0.0;

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn11 = 0.0;
        locals.var_dppg_dn14 = 0.0;
        locals.var_dppg_rv = 0.0;

        let assign23570_e18059: f64 = if locals.var_flg_pgd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign23570_e18059;
        locals.var_guard433_rv = 0.0;

        let (assign23580_e18063, assign23580_e18063_d_n0, assign23580_e18063_d_n2, assign23580_e18063_d_n4, assign23580_e18063_d_n5, assign23580_e18063_d_n6, assign23580_e18063_d_n7, assign23580_e18063_d_n8, assign23580_e18063_d_n9, assign23580_e18063_d_n10, assign23580_e18063_d_n11, assign23580_e18063_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn4, locals.var_vgsz_dn5, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn8, locals.var_vgsz_dn9, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23580_e18063;
        locals.var_t7_dn0 = assign23580_e18063_d_n0;
        locals.var_t7_dn2 = assign23580_e18063_d_n2;
        locals.var_t7_dn4 = assign23580_e18063_d_n4;
        locals.var_t7_dn5 = assign23580_e18063_d_n5;
        locals.var_t7_dn6 = assign23580_e18063_d_n6;
        locals.var_t7_dn7 = assign23580_e18063_d_n7;
        locals.var_t7_dn8 = assign23580_e18063_d_n8;
        locals.var_t7_dn9 = assign23580_e18063_d_n9;
        locals.var_t7_dn10 = assign23580_e18063_d_n10;
        locals.var_t7_dn11 = assign23580_e18063_d_n11;
        locals.var_t7_dn14 = assign23580_e18063_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23590_e18067, assign23590_e18067_d_n0, assign23590_e18067_d_n2, assign23590_e18067_d_n4, assign23590_e18067_d_n5, assign23590_e18067_d_n6, assign23590_e18067_d_n7, assign23590_e18067_d_n8, assign23590_e18067_d_n9, assign23590_e18067_d_n10, assign23590_e18067_d_n11, assign23590_e18067_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        (locals.var_cnstpgd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23590_e18067;
        locals.var_t0_dn0 = assign23590_e18067_d_n0;
        locals.var_t0_dn2 = assign23590_e18067_d_n2;
        locals.var_t0_dn4 = assign23590_e18067_d_n4;
        locals.var_t0_dn5 = assign23590_e18067_d_n5;
        locals.var_t0_dn6 = assign23590_e18067_d_n6;
        locals.var_t0_dn7 = assign23590_e18067_d_n7;
        locals.var_t0_dn8 = assign23590_e18067_d_n8;
        locals.var_t0_dn9 = assign23590_e18067_d_n9;
        locals.var_t0_dn10 = assign23590_e18067_d_n10;
        locals.var_t0_dn11 = assign23590_e18067_d_n11;
        locals.var_t0_dn14 = assign23590_e18067_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23600_e18073, assign23600_e18073_d_n0, assign23600_e18073_d_n2, assign23600_e18073_d_n4, assign23600_e18073_d_n5, assign23600_e18073_d_n6, assign23600_e18073_d_n7, assign23600_e18073_d_n8, assign23600_e18073_d_n9, assign23600_e18073_d_n10, assign23600_e18073_d_n11, assign23600_e18073_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23600_e18071: f64 = (locals.var_t7 - p.p152);
        (assign23600_e18071, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23600_e18073;
        locals.var_t3_dn0 = assign23600_e18073_d_n0;
        locals.var_t3_dn2 = assign23600_e18073_d_n2;
        locals.var_t3_dn4 = assign23600_e18073_d_n4;
        locals.var_t3_dn5 = assign23600_e18073_d_n5;
        locals.var_t3_dn6 = assign23600_e18073_d_n6;
        locals.var_t3_dn7 = assign23600_e18073_d_n7;
        locals.var_t3_dn8 = assign23600_e18073_d_n8;
        locals.var_t3_dn9 = assign23600_e18073_d_n9;
        locals.var_t3_dn10 = assign23600_e18073_d_n10;
        locals.var_t3_dn11 = assign23600_e18073_d_n11;
        locals.var_t3_dn14 = assign23600_e18073_d_n14;
        locals.var_t3_rv = 0.0;

        let assign23610_e18076: f64 = (-3.0);
        let assign23610_e18077: f64 = if locals.var_t3 < assign23610_e18076 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23610_e18077;
        locals.var_guard434_rv = 0.0;

        let (assign23620_e18083, assign23620_e18083_d_n0, assign23620_e18083_d_n2, assign23620_e18083_d_n4, assign23620_e18083_d_n5, assign23620_e18083_d_n6, assign23620_e18083_d_n7, assign23620_e18083_d_n8, assign23620_e18083_d_n9, assign23620_e18083_d_n10, assign23620_e18083_d_n11, assign23620_e18083_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23620_e18083;
        locals.var_t6_dn0 = assign23620_e18083_d_n0;
        locals.var_t6_dn2 = assign23620_e18083_d_n2;
        locals.var_t6_dn4 = assign23620_e18083_d_n4;
        locals.var_t6_dn5 = assign23620_e18083_d_n5;
        locals.var_t6_dn6 = assign23620_e18083_d_n6;
        locals.var_t6_dn7 = assign23620_e18083_d_n7;
        locals.var_t6_dn8 = assign23620_e18083_d_n8;
        locals.var_t6_dn9 = assign23620_e18083_d_n9;
        locals.var_t6_dn10 = assign23620_e18083_d_n10;
        locals.var_t6_dn11 = assign23620_e18083_d_n11;
        locals.var_t6_dn14 = assign23620_e18083_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23630_e18089, assign23630_e18089_d_n0, assign23630_e18089_d_n2, assign23630_e18089_d_n4, assign23630_e18089_d_n5, assign23630_e18089_d_n6, assign23630_e18089_d_n7, assign23630_e18089_d_n8, assign23630_e18089_d_n9, assign23630_e18089_d_n10, assign23630_e18089_d_n11, assign23630_e18089_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23630_e18089;
        locals.var_dppg_dn0 = assign23630_e18089_d_n0;
        locals.var_dppg_dn2 = assign23630_e18089_d_n2;
        locals.var_dppg_dn4 = assign23630_e18089_d_n4;
        locals.var_dppg_dn5 = assign23630_e18089_d_n5;
        locals.var_dppg_dn6 = assign23630_e18089_d_n6;
        locals.var_dppg_dn7 = assign23630_e18089_d_n7;
        locals.var_dppg_dn8 = assign23630_e18089_d_n8;
        locals.var_dppg_dn9 = assign23630_e18089_d_n9;
        locals.var_dppg_dn10 = assign23630_e18089_d_n10;
        locals.var_dppg_dn11 = assign23630_e18089_d_n11;
        locals.var_dppg_dn14 = assign23630_e18089_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23640_e18092: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23640_e18092;
        locals.var_guard435_rv = 0.0;

        let (assign23650_e18117, assign23650_e18117_d_n0, assign23650_e18117_d_n2, assign23650_e18117_d_n4, assign23650_e18117_d_n5, assign23650_e18117_d_n6, assign23650_e18117_d_n7, assign23650_e18117_d_n8, assign23650_e18117_d_n9, assign23650_e18117_d_n10, assign23650_e18117_d_n11, assign23650_e18117_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        let assign23650_e18104: f64 = (1.0 / 3.0);
        let assign23650_e18105: f64 = (2.0 * assign23650_e18104);
        let assign23650_e18108: f64 = (locals.var_t3 * 3.0);
        let assign23650_e18111: f64 = (1.0 / 27.0);
        let assign23650_e18112: f64 = (assign23650_e18108 * assign23650_e18111);
        let assign23650_e18113: f64 = (assign23650_e18105 + assign23650_e18112);
        let assign23650_e18114: f64 = (locals.var_t3 * assign23650_e18113);
        let assign23650_e18115: f64 = (1.0 + assign23650_e18114);
        (assign23650_e18115, ((locals.var_t3_dn0 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn0 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn2 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn2 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn4 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn4 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn5 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn5 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn6 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn6 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn7 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn7 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn8 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn8 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn9 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn9 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn10 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn10 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn11 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn11 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn14 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn14 * 3.0) * assign23650_e18111))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23650_e18117;
        locals.var_t6_dn0 = assign23650_e18117_d_n0;
        locals.var_t6_dn2 = assign23650_e18117_d_n2;
        locals.var_t6_dn4 = assign23650_e18117_d_n4;
        locals.var_t6_dn5 = assign23650_e18117_d_n5;
        locals.var_t6_dn6 = assign23650_e18117_d_n6;
        locals.var_t6_dn7 = assign23650_e18117_d_n7;
        locals.var_t6_dn8 = assign23650_e18117_d_n8;
        locals.var_t6_dn9 = assign23650_e18117_d_n9;
        locals.var_t6_dn10 = assign23650_e18117_d_n10;
        locals.var_t6_dn11 = assign23650_e18117_d_n11;
        locals.var_t6_dn14 = assign23650_e18117_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23660_e18142, assign23660_e18142_d_n0, assign23660_e18142_d_n2, assign23660_e18142_d_n4, assign23660_e18142_d_n5, assign23660_e18142_d_n6, assign23660_e18142_d_n7, assign23660_e18142_d_n8, assign23660_e18142_d_n9, assign23660_e18142_d_n10, assign23660_e18142_d_n11, assign23660_e18142_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        let assign23660_e18130: f64 = (1.0 / 3.0);
        let assign23660_e18134: f64 = (1.0 / 27.0);
        let assign23660_e18135: f64 = (locals.var_t3 * assign23660_e18134);
        let assign23660_e18136: f64 = (assign23660_e18130 + assign23660_e18135);
        let assign23660_e18137: f64 = (locals.var_t3 * assign23660_e18136);
        let assign23660_e18138: f64 = (1.0 + assign23660_e18137);
        let assign23660_e18139: f64 = (locals.var_t3 * assign23660_e18138);
        let assign23660_e18140: f64 = (1.0 + assign23660_e18139);
        (assign23660_e18140, ((locals.var_t3_dn0 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn0 * assign23660_e18134))))), ((locals.var_t3_dn2 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn2 * assign23660_e18134))))), ((locals.var_t3_dn4 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn4 * assign23660_e18134))))), ((locals.var_t3_dn5 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn5 * assign23660_e18134))))), ((locals.var_t3_dn6 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn6 * assign23660_e18134))))), ((locals.var_t3_dn7 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn7 * assign23660_e18134))))), ((locals.var_t3_dn8 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn8 * assign23660_e18134))))), ((locals.var_t3_dn9 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn9 * assign23660_e18134))))), ((locals.var_t3_dn10 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn10 * assign23660_e18134))))), ((locals.var_t3_dn11 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn11 * assign23660_e18134))))), ((locals.var_t3_dn14 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn14 * assign23660_e18134))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23660_e18142;
        locals.var_dppg_dn0 = assign23660_e18142_d_n0;
        locals.var_dppg_dn2 = assign23660_e18142_d_n2;
        locals.var_dppg_dn4 = assign23660_e18142_d_n4;
        locals.var_dppg_dn5 = assign23660_e18142_d_n5;
        locals.var_dppg_dn6 = assign23660_e18142_d_n6;
        locals.var_dppg_dn7 = assign23660_e18142_d_n7;
        locals.var_dppg_dn8 = assign23660_e18142_d_n8;
        locals.var_dppg_dn9 = assign23660_e18142_d_n9;
        locals.var_dppg_dn10 = assign23660_e18142_d_n10;
        locals.var_dppg_dn11 = assign23660_e18142_d_n11;
        locals.var_dppg_dn14 = assign23660_e18142_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23670_e18172, assign23670_e18172_d_n0, assign23670_e18172_d_n2, assign23670_e18172_d_n4, assign23670_e18172_d_n5, assign23670_e18172_d_n6, assign23670_e18172_d_n7, assign23670_e18172_d_n8, assign23670_e18172_d_n9, assign23670_e18172_d_n10, assign23670_e18172_d_n11, assign23670_e18172_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) {
        let assign23670_e18155: f64 = (1.0 / 3.0);
        let assign23670_e18156: f64 = (2.0 * assign23670_e18155);
        let assign23670_e18160: f64 = (3.0 * 0.0402052934513951);
        let assign23670_e18163: f64 = (locals.var_t3 * 4.0);
        let assign23670_e18165: f64 = (assign23670_e18163 * 0.148148111111111);
        let assign23670_e18166: f64 = (assign23670_e18160 + assign23670_e18165);
        let assign23670_e18167: f64 = (locals.var_t3 * assign23670_e18166);
        let assign23670_e18168: f64 = (assign23670_e18156 + assign23670_e18167);
        let assign23670_e18169: f64 = (locals.var_t3 * assign23670_e18168);
        let assign23670_e18170: f64 = (1.0 + assign23670_e18169);
        (assign23670_e18170, ((locals.var_t3_dn0 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn0 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn2 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn2 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn4 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn4 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn5 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn5 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn6 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn6 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn7 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn7 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn8 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn8 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn9 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn9 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn10 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn10 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn11 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn11 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn14 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn14 * 4.0) * 0.148148111111111))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23670_e18172;
        locals.var_t6_dn0 = assign23670_e18172_d_n0;
        locals.var_t6_dn2 = assign23670_e18172_d_n2;
        locals.var_t6_dn4 = assign23670_e18172_d_n4;
        locals.var_t6_dn5 = assign23670_e18172_d_n5;
        locals.var_t6_dn6 = assign23670_e18172_d_n6;
        locals.var_t6_dn7 = assign23670_e18172_d_n7;
        locals.var_t6_dn8 = assign23670_e18172_d_n8;
        locals.var_t6_dn9 = assign23670_e18172_d_n9;
        locals.var_t6_dn10 = assign23670_e18172_d_n10;
        locals.var_t6_dn11 = assign23670_e18172_d_n11;
        locals.var_t6_dn14 = assign23670_e18172_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23680_e18200, assign23680_e18200_d_n0, assign23680_e18200_d_n2, assign23680_e18200_d_n4, assign23680_e18200_d_n5, assign23680_e18200_d_n6, assign23680_e18200_d_n7, assign23680_e18200_d_n8, assign23680_e18200_d_n9, assign23680_e18200_d_n10, assign23680_e18200_d_n11, assign23680_e18200_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) {
        let assign23680_e18186: f64 = (1.0 / 3.0);
        let assign23680_e18191: f64 = (locals.var_t3 * 0.148148111111111);
        let assign23680_e18192: f64 = (0.0402052934513951 + assign23680_e18191);
        let assign23680_e18193: f64 = (locals.var_t3 * assign23680_e18192);
        let assign23680_e18194: f64 = (assign23680_e18186 + assign23680_e18193);
        let assign23680_e18195: f64 = (locals.var_t3 * assign23680_e18194);
        let assign23680_e18196: f64 = (1.0 + assign23680_e18195);
        let assign23680_e18197: f64 = (locals.var_t3 * assign23680_e18196);
        let assign23680_e18198: f64 = (1.0 + assign23680_e18197);
        (assign23680_e18198, ((locals.var_t3_dn0 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn0 * 0.148148111111111))))))), ((locals.var_t3_dn2 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn2 * 0.148148111111111))))))), ((locals.var_t3_dn4 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn4 * 0.148148111111111))))))), ((locals.var_t3_dn5 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn5 * 0.148148111111111))))))), ((locals.var_t3_dn6 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn6 * 0.148148111111111))))))), ((locals.var_t3_dn7 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn7 * 0.148148111111111))))))), ((locals.var_t3_dn8 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn8 * 0.148148111111111))))))), ((locals.var_t3_dn9 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn9 * 0.148148111111111))))))), ((locals.var_t3_dn10 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn10 * 0.148148111111111))))))), ((locals.var_t3_dn11 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn11 * 0.148148111111111))))))), ((locals.var_t3_dn14 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn14 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23680_e18200;
        locals.var_dppg_dn0 = assign23680_e18200_d_n0;
        locals.var_dppg_dn2 = assign23680_e18200_d_n2;
        locals.var_dppg_dn4 = assign23680_e18200_d_n4;
        locals.var_dppg_dn5 = assign23680_e18200_d_n5;
        locals.var_dppg_dn6 = assign23680_e18200_d_n6;
        locals.var_dppg_dn7 = assign23680_e18200_d_n7;
        locals.var_dppg_dn8 = assign23680_e18200_d_n8;
        locals.var_dppg_dn9 = assign23680_e18200_d_n9;
        locals.var_dppg_dn10 = assign23680_e18200_d_n10;
        locals.var_dppg_dn11 = assign23680_e18200_d_n11;
        locals.var_dppg_dn14 = assign23680_e18200_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23690_e18217, assign23690_e18217_d_n0, assign23690_e18217_d_n2, assign23690_e18217_d_n4, assign23690_e18217_d_n5, assign23690_e18217_d_n6, assign23690_e18217_d_n7, assign23690_e18217_d_n8, assign23690_e18217_d_n9, assign23690_e18217_d_n10, assign23690_e18217_d_n11, assign23690_e18217_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23690_e18204: f64 = (locals.var_dppg - 1.0);
        let assign23690_e18207: f64 = (locals.var_dppg - 1.0);
        let assign23690_e18208: f64 = (assign23690_e18204 * assign23690_e18207);
        let assign23690_e18211: f64 = (4.0 * 0.05);
        let assign23690_e18213: f64 = (assign23690_e18211 * 0.05);
        let assign23690_e18214: f64 = (assign23690_e18208 + assign23690_e18213);
        let assign23690_e18215: f64 = (assign23690_e18214).sqrt();
        (assign23690_e18215, (((locals.var_dppg_dn0 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn0)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn2 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn2)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn4 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn4)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn5 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn5)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn6 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn6)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn7 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn7)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn8 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn8)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn9 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn9)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn10 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn10)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn11 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn11)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn14 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn14)) / (2.0 * assign23690_e18215)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23690_e18217;
        locals.var_tmf2_dn0 = assign23690_e18217_d_n0;
        locals.var_tmf2_dn2 = assign23690_e18217_d_n2;
        locals.var_tmf2_dn4 = assign23690_e18217_d_n4;
        locals.var_tmf2_dn5 = assign23690_e18217_d_n5;
        locals.var_tmf2_dn6 = assign23690_e18217_d_n6;
        locals.var_tmf2_dn7 = assign23690_e18217_d_n7;
        locals.var_tmf2_dn8 = assign23690_e18217_d_n8;
        locals.var_tmf2_dn9 = assign23690_e18217_d_n9;
        locals.var_tmf2_dn10 = assign23690_e18217_d_n10;
        locals.var_tmf2_dn11 = assign23690_e18217_d_n11;
        locals.var_tmf2_dn14 = assign23690_e18217_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23700_e18229, assign23700_e18229_d_n0, assign23700_e18229_d_n2, assign23700_e18229_d_n4, assign23700_e18229_d_n5, assign23700_e18229_d_n6, assign23700_e18229_d_n7, assign23700_e18229_d_n8, assign23700_e18229_d_n9, assign23700_e18229_d_n10, assign23700_e18229_d_n11, assign23700_e18229_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23700_e18223: f64 = (locals.var_dppg - 1.0);
        let assign23700_e18225: f64 = (assign23700_e18223 / locals.var_tmf2);
        let assign23700_e18226: f64 = (1.0 + assign23700_e18225);
        let assign23700_e18227: f64 = (0.5 * assign23700_e18226);
        (assign23700_e18227, (0.5 * (((locals.var_dppg_dn0 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn2 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn4 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn5 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn6 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn7 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn8 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn9 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn10 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn11 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn14 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23700_e18229;
        locals.var_t6_dn0 = assign23700_e18229_d_n0;
        locals.var_t6_dn2 = assign23700_e18229_d_n2;
        locals.var_t6_dn4 = assign23700_e18229_d_n4;
        locals.var_t6_dn5 = assign23700_e18229_d_n5;
        locals.var_t6_dn6 = assign23700_e18229_d_n6;
        locals.var_t6_dn7 = assign23700_e18229_d_n7;
        locals.var_t6_dn8 = assign23700_e18229_d_n8;
        locals.var_t6_dn9 = assign23700_e18229_d_n9;
        locals.var_t6_dn10 = assign23700_e18229_d_n10;
        locals.var_t6_dn11 = assign23700_e18229_d_n11;
        locals.var_t6_dn14 = assign23700_e18229_d_n14;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23710_e18239, assign23710_e18239_d_n0, assign23710_e18239_d_n2, assign23710_e18239_d_n4, assign23710_e18239_d_n5, assign23710_e18239_d_n6, assign23710_e18239_d_n7, assign23710_e18239_d_n8, assign23710_e18239_d_n9, assign23710_e18239_d_n10, assign23710_e18239_d_n11, assign23710_e18239_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23710_e18234: f64 = (locals.var_dppg - 1.0);
        let assign23710_e18236: f64 = (assign23710_e18234 + locals.var_tmf2);
        let assign23710_e18237: f64 = (0.5 * assign23710_e18236);
        (assign23710_e18237, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_dppg_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_dppg_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_dppg_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_dppg_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_dppg_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23710_e18239;
        locals.var_dppg_dn0 = assign23710_e18239_d_n0;
        locals.var_dppg_dn2 = assign23710_e18239_d_n2;
        locals.var_dppg_dn4 = assign23710_e18239_d_n4;
        locals.var_dppg_dn5 = assign23710_e18239_d_n5;
        locals.var_dppg_dn6 = assign23710_e18239_d_n6;
        locals.var_dppg_dn7 = assign23710_e18239_d_n7;
        locals.var_dppg_dn8 = assign23710_e18239_d_n8;
        locals.var_dppg_dn9 = assign23710_e18239_d_n9;
        locals.var_dppg_dn10 = assign23710_e18239_d_n10;
        locals.var_dppg_dn11 = assign23710_e18239_d_n11;
        locals.var_dppg_dn14 = assign23710_e18239_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23720_e18242: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23720_e18242;
        locals.var_guard436_rv = 0.0;

        let (assign23730_e18248, assign23730_e18248_d_n0, assign23730_e18248_d_n2, assign23730_e18248_d_n4, assign23730_e18248_d_n5, assign23730_e18248_d_n6, assign23730_e18248_d_n7, assign23730_e18248_d_n8, assign23730_e18248_d_n9, assign23730_e18248_d_n10, assign23730_e18248_d_n11, assign23730_e18248_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23730_e18248;
        locals.var_dppg_dn0 = assign23730_e18248_d_n0;
        locals.var_dppg_dn2 = assign23730_e18248_d_n2;
        locals.var_dppg_dn4 = assign23730_e18248_d_n4;
        locals.var_dppg_dn5 = assign23730_e18248_d_n5;
        locals.var_dppg_dn6 = assign23730_e18248_d_n6;
        locals.var_dppg_dn7 = assign23730_e18248_d_n7;
        locals.var_dppg_dn8 = assign23730_e18248_d_n8;
        locals.var_dppg_dn9 = assign23730_e18248_d_n9;
        locals.var_dppg_dn10 = assign23730_e18248_d_n10;
        locals.var_dppg_dn11 = assign23730_e18248_d_n11;
        locals.var_dppg_dn14 = assign23730_e18248_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23740_e18254, assign23740_e18254_d_n0, assign23740_e18254_d_n2, assign23740_e18254_d_n4, assign23740_e18254_d_n5, assign23740_e18254_d_n6, assign23740_e18254_d_n7, assign23740_e18254_d_n8, assign23740_e18254_d_n9, assign23740_e18254_d_n10, assign23740_e18254_d_n11, assign23740_e18254_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23740_e18254;
        locals.var_t6_dn0 = assign23740_e18254_d_n0;
        locals.var_t6_dn2 = assign23740_e18254_d_n2;
        locals.var_t6_dn4 = assign23740_e18254_d_n4;
        locals.var_t6_dn5 = assign23740_e18254_d_n5;
        locals.var_t6_dn6 = assign23740_e18254_d_n6;
        locals.var_t6_dn7 = assign23740_e18254_d_n7;
        locals.var_t6_dn8 = assign23740_e18254_d_n8;
        locals.var_t6_dn9 = assign23740_e18254_d_n9;
        locals.var_t6_dn10 = assign23740_e18254_d_n10;
        locals.var_t6_dn11 = assign23740_e18254_d_n11;
        locals.var_t6_dn14 = assign23740_e18254_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23750_e18260, assign23750_e18260_d_n0, assign23750_e18260_d_n2, assign23750_e18260_d_n4, assign23750_e18260_d_n5, assign23750_e18260_d_n6, assign23750_e18260_d_n7, assign23750_e18260_d_n8, assign23750_e18260_d_n9, assign23750_e18260_d_n10, assign23750_e18260_d_n11, assign23750_e18260_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23750_e18258: f64 = (locals.var_dppg * locals.var_t0);
        (assign23750_e18258, ((locals.var_dppg_dn0 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn0)), ((locals.var_dppg_dn2 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn2)), ((locals.var_dppg_dn4 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn4)), ((locals.var_dppg_dn5 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn5)), ((locals.var_dppg_dn6 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn6)), ((locals.var_dppg_dn7 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn7)), ((locals.var_dppg_dn8 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn8)), ((locals.var_dppg_dn9 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn9)), ((locals.var_dppg_dn10 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn10)), ((locals.var_dppg_dn11 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn11)), ((locals.var_dppg_dn14 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23750_e18260;
        locals.var_dppg_dn0 = assign23750_e18260_d_n0;
        locals.var_dppg_dn2 = assign23750_e18260_d_n2;
        locals.var_dppg_dn4 = assign23750_e18260_d_n4;
        locals.var_dppg_dn5 = assign23750_e18260_d_n5;
        locals.var_dppg_dn6 = assign23750_e18260_d_n6;
        locals.var_dppg_dn7 = assign23750_e18260_d_n7;
        locals.var_dppg_dn8 = assign23750_e18260_d_n8;
        locals.var_dppg_dn9 = assign23750_e18260_d_n9;
        locals.var_dppg_dn10 = assign23750_e18260_d_n10;
        locals.var_dppg_dn11 = assign23750_e18260_d_n11;
        locals.var_dppg_dn14 = assign23750_e18260_d_n14;
        locals.var_dppg_rv = 0.0;

        let (assign23760_e18268, assign23760_e18268_d_n0, assign23760_e18268_d_n2, assign23760_e18268_d_n4, assign23760_e18268_d_n5, assign23760_e18268_d_n6, assign23760_e18268_d_n7, assign23760_e18268_d_n8, assign23760_e18268_d_n9, assign23760_e18268_d_n10, assign23760_e18268_d_n11, assign23760_e18268_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23760_e18264: f64 = (1.0 - locals.var_dppg);
        let assign23760_e18266: f64 = (assign23760_e18264 - 0.05);
        (assign23760_e18266, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn4), (-locals.var_dppg_dn5), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn8), (-locals.var_dppg_dn9), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23760_e18268;
        locals.var_tmf1_dn0 = assign23760_e18268_d_n0;
        locals.var_tmf1_dn2 = assign23760_e18268_d_n2;
        locals.var_tmf1_dn4 = assign23760_e18268_d_n4;
        locals.var_tmf1_dn5 = assign23760_e18268_d_n5;
        locals.var_tmf1_dn6 = assign23760_e18268_d_n6;
        locals.var_tmf1_dn7 = assign23760_e18268_d_n7;
        locals.var_tmf1_dn8 = assign23760_e18268_d_n8;
        locals.var_tmf1_dn9 = assign23760_e18268_d_n9;
        locals.var_tmf1_dn10 = assign23760_e18268_d_n10;
        locals.var_tmf1_dn11 = assign23760_e18268_d_n11;
        locals.var_tmf1_dn14 = assign23760_e18268_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23770_e18276, assign23770_e18276_d_n0, assign23770_e18276_d_n2, assign23770_e18276_d_n4, assign23770_e18276_d_n5, assign23770_e18276_d_n6, assign23770_e18276_d_n7, assign23770_e18276_d_n8, assign23770_e18276_d_n9, assign23770_e18276_d_n10, assign23770_e18276_d_n11, assign23770_e18276_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23770_e18272: f64 = 4.0;
        let assign23770_e18274: f64 = (assign23770_e18272 * 0.05);
        (assign23770_e18274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23770_e18276;
        locals.var_tmf2_dn0 = assign23770_e18276_d_n0;
        locals.var_tmf2_dn2 = assign23770_e18276_d_n2;
        locals.var_tmf2_dn4 = assign23770_e18276_d_n4;
        locals.var_tmf2_dn5 = assign23770_e18276_d_n5;
        locals.var_tmf2_dn6 = assign23770_e18276_d_n6;
        locals.var_tmf2_dn7 = assign23770_e18276_d_n7;
        locals.var_tmf2_dn8 = assign23770_e18276_d_n8;
        locals.var_tmf2_dn9 = assign23770_e18276_d_n9;
        locals.var_tmf2_dn10 = assign23770_e18276_d_n10;
        locals.var_tmf2_dn11 = assign23770_e18276_d_n11;
        locals.var_tmf2_dn14 = assign23770_e18276_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23780_e18286, assign23780_e18286_d_n0, assign23780_e18286_d_n2, assign23780_e18286_d_n4, assign23780_e18286_d_n5, assign23780_e18286_d_n6, assign23780_e18286_d_n7, assign23780_e18286_d_n8, assign23780_e18286_d_n9, assign23780_e18286_d_n10, assign23780_e18286_d_n11, assign23780_e18286_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let (assign23780_e18284, assign23780_e18284_d_n0, assign23780_e18284_d_n2, assign23780_e18284_d_n4, assign23780_e18284_d_n5, assign23780_e18284_d_n6, assign23780_e18284_d_n7, assign23780_e18284_d_n8, assign23780_e18284_d_n9, assign23780_e18284_d_n10, assign23780_e18284_d_n11, assign23780_e18284_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23780_e18283: f64 = (-locals.var_tmf2);
                (assign23780_e18283, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23780_e18284, assign23780_e18284_d_n0, assign23780_e18284_d_n2, assign23780_e18284_d_n4, assign23780_e18284_d_n5, assign23780_e18284_d_n6, assign23780_e18284_d_n7, assign23780_e18284_d_n8, assign23780_e18284_d_n9, assign23780_e18284_d_n10, assign23780_e18284_d_n11, assign23780_e18284_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23780_e18286;
        locals.var_tmf2_dn0 = assign23780_e18286_d_n0;
        locals.var_tmf2_dn2 = assign23780_e18286_d_n2;
        locals.var_tmf2_dn4 = assign23780_e18286_d_n4;
        locals.var_tmf2_dn5 = assign23780_e18286_d_n5;
        locals.var_tmf2_dn6 = assign23780_e18286_d_n6;
        locals.var_tmf2_dn7 = assign23780_e18286_d_n7;
        locals.var_tmf2_dn8 = assign23780_e18286_d_n8;
        locals.var_tmf2_dn9 = assign23780_e18286_d_n9;
        locals.var_tmf2_dn10 = assign23780_e18286_d_n10;
        locals.var_tmf2_dn11 = assign23780_e18286_d_n11;
        locals.var_tmf2_dn14 = assign23780_e18286_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23790_e18295, assign23790_e18295_d_n0, assign23790_e18295_d_n2, assign23790_e18295_d_n4, assign23790_e18295_d_n5, assign23790_e18295_d_n6, assign23790_e18295_d_n7, assign23790_e18295_d_n8, assign23790_e18295_d_n9, assign23790_e18295_d_n10, assign23790_e18295_d_n11, assign23790_e18295_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23790_e18290: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23790_e18292: f64 = (assign23790_e18290 + locals.var_tmf2);
        let assign23790_e18293: f64 = (assign23790_e18292).sqrt();
        (assign23790_e18293, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23790_e18293)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23790_e18295;
        locals.var_tmf2_dn0 = assign23790_e18295_d_n0;
        locals.var_tmf2_dn2 = assign23790_e18295_d_n2;
        locals.var_tmf2_dn4 = assign23790_e18295_d_n4;
        locals.var_tmf2_dn5 = assign23790_e18295_d_n5;
        locals.var_tmf2_dn6 = assign23790_e18295_d_n6;
        locals.var_tmf2_dn7 = assign23790_e18295_d_n7;
        locals.var_tmf2_dn8 = assign23790_e18295_d_n8;
        locals.var_tmf2_dn9 = assign23790_e18295_d_n9;
        locals.var_tmf2_dn10 = assign23790_e18295_d_n10;
        locals.var_tmf2_dn11 = assign23790_e18295_d_n11;
        locals.var_tmf2_dn14 = assign23790_e18295_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23800_e18305, assign23800_e18305_d_n0, assign23800_e18305_d_n2, assign23800_e18305_d_n4, assign23800_e18305_d_n5, assign23800_e18305_d_n6, assign23800_e18305_d_n7, assign23800_e18305_d_n8, assign23800_e18305_d_n9, assign23800_e18305_d_n10, assign23800_e18305_d_n11, assign23800_e18305_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23800_e18301: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23800_e18302: f64 = (1.0 + assign23800_e18301);
        let assign23800_e18303: f64 = (0.5 * assign23800_e18302);
        (assign23800_e18303, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23800_e18305;
        locals.var_t9_dn0 = assign23800_e18305_d_n0;
        locals.var_t9_dn2 = assign23800_e18305_d_n2;
        locals.var_t9_dn4 = assign23800_e18305_d_n4;
        locals.var_t9_dn5 = assign23800_e18305_d_n5;
        locals.var_t9_dn6 = assign23800_e18305_d_n6;
        locals.var_t9_dn7 = assign23800_e18305_d_n7;
        locals.var_t9_dn8 = assign23800_e18305_d_n8;
        locals.var_t9_dn9 = assign23800_e18305_d_n9;
        locals.var_t9_dn10 = assign23800_e18305_d_n10;
        locals.var_t9_dn11 = assign23800_e18305_d_n11;
        locals.var_t9_dn14 = assign23800_e18305_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign23810_e18315, assign23810_e18315_d_n0, assign23810_e18315_d_n2, assign23810_e18315_d_n4, assign23810_e18315_d_n5, assign23810_e18315_d_n6, assign23810_e18315_d_n7, assign23810_e18315_d_n8, assign23810_e18315_d_n9, assign23810_e18315_d_n10, assign23810_e18315_d_n11, assign23810_e18315_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23810_e18311: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23810_e18312: f64 = (0.5 * assign23810_e18311);
        let assign23810_e18313: f64 = (1.0 - assign23810_e18312);
        (assign23810_e18313, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23810_e18315;
        locals.var_dppg_dn0 = assign23810_e18315_d_n0;
        locals.var_dppg_dn2 = assign23810_e18315_d_n2;
        locals.var_dppg_dn4 = assign23810_e18315_d_n4;
        locals.var_dppg_dn5 = assign23810_e18315_d_n5;
        locals.var_dppg_dn6 = assign23810_e18315_d_n6;
        locals.var_dppg_dn7 = assign23810_e18315_d_n7;
        locals.var_dppg_dn8 = assign23810_e18315_d_n8;
        locals.var_dppg_dn9 = assign23810_e18315_d_n9;
        locals.var_dppg_dn10 = assign23810_e18315_d_n10;
        locals.var_dppg_dn11 = assign23810_e18315_d_n11;
        locals.var_dppg_dn14 = assign23810_e18315_d_n14;
        locals.var_dppg_rv = 0.0;

        let assign23820_e18318: f64 = if locals.var_vbs > locals.var_vbs_bnd_local { 1.0 } else { 0.0 };
        locals.var_guard443 = assign23820_e18318;
        locals.var_guard443_rv = 0.0;

        let (assign23830_e18326, assign23830_e18326_d_n0, assign23830_e18326_d_n2, assign23830_e18326_d_n4, assign23830_e18326_d_n5, assign23830_e18326_d_n6, assign23830_e18326_d_n7, assign23830_e18326_d_n8, assign23830_e18326_d_n9, assign23830_e18326_d_n10, assign23830_e18326_d_n11, assign23830_e18326_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23830_e18324: f64 = (locals.var_vbs - locals.var_vbs_bnd_local);
        (assign23830_e18324, (-locals.var_vbs_bnd_local_dn0), (-locals.var_vbs_bnd_local_dn2), (-locals.var_vbs_bnd_local_dn4), (-locals.var_vbs_bnd_local_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_local_dn6), (-locals.var_vbs_bnd_local_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_local_dn9), (-locals.var_vbs_bnd_local_dn10), (-locals.var_vbs_bnd_local_dn11), (-locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23830_e18326;
        locals.var_t1_dn0 = assign23830_e18326_d_n0;
        locals.var_t1_dn2 = assign23830_e18326_d_n2;
        locals.var_t1_dn4 = assign23830_e18326_d_n4;
        locals.var_t1_dn5 = assign23830_e18326_d_n5;
        locals.var_t1_dn6 = assign23830_e18326_d_n6;
        locals.var_t1_dn7 = assign23830_e18326_d_n7;
        locals.var_t1_dn8 = assign23830_e18326_d_n8;
        locals.var_t1_dn9 = assign23830_e18326_d_n9;
        locals.var_t1_dn10 = assign23830_e18326_d_n10;
        locals.var_t1_dn11 = assign23830_e18326_d_n11;
        locals.var_t1_dn14 = assign23830_e18326_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23840_e18334, assign23840_e18334_d_n0, assign23840_e18334_d_n2, assign23840_e18334_d_n4, assign23840_e18334_d_n5, assign23840_e18334_d_n6, assign23840_e18334_d_n7, assign23840_e18334_d_n8, assign23840_e18334_d_n9, assign23840_e18334_d_n10, assign23840_e18334_d_n11, assign23840_e18334_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23840_e18332: f64 = (locals.var_vbs_max_local - locals.var_vbs_bnd_local);
        (assign23840_e18332, (locals.var_vbs_max_local_dn0 - locals.var_vbs_bnd_local_dn0), (locals.var_vbs_max_local_dn2 - locals.var_vbs_bnd_local_dn2), (locals.var_vbs_max_local_dn4 - locals.var_vbs_bnd_local_dn4), (locals.var_vbs_max_local_dn5 - locals.var_vbs_bnd_local_dn5), (locals.var_vbs_max_local_dn6 - locals.var_vbs_bnd_local_dn6), (locals.var_vbs_max_local_dn7 - locals.var_vbs_bnd_local_dn7), (locals.var_vbs_max_local_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_max_local_dn9 - locals.var_vbs_bnd_local_dn9), (locals.var_vbs_max_local_dn10 - locals.var_vbs_bnd_local_dn10), (locals.var_vbs_max_local_dn11 - locals.var_vbs_bnd_local_dn11), (locals.var_vbs_max_local_dn14 - locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23840_e18334;
        locals.var_t2_dn0 = assign23840_e18334_d_n0;
        locals.var_t2_dn2 = assign23840_e18334_d_n2;
        locals.var_t2_dn4 = assign23840_e18334_d_n4;
        locals.var_t2_dn5 = assign23840_e18334_d_n5;
        locals.var_t2_dn6 = assign23840_e18334_d_n6;
        locals.var_t2_dn7 = assign23840_e18334_d_n7;
        locals.var_t2_dn8 = assign23840_e18334_d_n8;
        locals.var_t2_dn9 = assign23840_e18334_d_n9;
        locals.var_t2_dn10 = assign23840_e18334_d_n10;
        locals.var_t2_dn11 = assign23840_e18334_d_n11;
        locals.var_t2_dn14 = assign23840_e18334_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23850_e18342, assign23850_e18342_d_n0, assign23850_e18342_d_n2, assign23850_e18342_d_n4, assign23850_e18342_d_n5, assign23850_e18342_d_n6, assign23850_e18342_d_n7, assign23850_e18342_d_n8, assign23850_e18342_d_n9, assign23850_e18342_d_n10, assign23850_e18342_d_n11, assign23850_e18342_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23850_e18340: f64 = (locals.var_t1 / locals.var_t2);
        (assign23850_e18340, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23850_e18342;
        locals.var_tmf1_dn0 = assign23850_e18342_d_n0;
        locals.var_tmf1_dn2 = assign23850_e18342_d_n2;
        locals.var_tmf1_dn4 = assign23850_e18342_d_n4;
        locals.var_tmf1_dn5 = assign23850_e18342_d_n5;
        locals.var_tmf1_dn6 = assign23850_e18342_d_n6;
        locals.var_tmf1_dn7 = assign23850_e18342_d_n7;
        locals.var_tmf1_dn8 = assign23850_e18342_d_n8;
        locals.var_tmf1_dn9 = assign23850_e18342_d_n9;
        locals.var_tmf1_dn10 = assign23850_e18342_d_n10;
        locals.var_tmf1_dn11 = assign23850_e18342_d_n11;
        locals.var_tmf1_dn14 = assign23850_e18342_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign23860_e18350, assign23860_e18350_d_n0, assign23860_e18350_d_n2, assign23860_e18350_d_n4, assign23860_e18350_d_n5, assign23860_e18350_d_n6, assign23860_e18350_d_n7, assign23860_e18350_d_n8, assign23860_e18350_d_n9, assign23860_e18350_d_n10, assign23860_e18350_d_n11, assign23860_e18350_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23860_e18348: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign23860_e18348, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23860_e18350;
        locals.var_tmf2_dn0 = assign23860_e18350_d_n0;
        locals.var_tmf2_dn2 = assign23860_e18350_d_n2;
        locals.var_tmf2_dn4 = assign23860_e18350_d_n4;
        locals.var_tmf2_dn5 = assign23860_e18350_d_n5;
        locals.var_tmf2_dn6 = assign23860_e18350_d_n6;
        locals.var_tmf2_dn7 = assign23860_e18350_d_n7;
        locals.var_tmf2_dn8 = assign23860_e18350_d_n8;
        locals.var_tmf2_dn9 = assign23860_e18350_d_n9;
        locals.var_tmf2_dn10 = assign23860_e18350_d_n10;
        locals.var_tmf2_dn11 = assign23860_e18350_d_n11;
        locals.var_tmf2_dn14 = assign23860_e18350_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign23870_e18358, assign23870_e18358_d_n0, assign23870_e18358_d_n2, assign23870_e18358_d_n4, assign23870_e18358_d_n5, assign23870_e18358_d_n6, assign23870_e18358_d_n7, assign23870_e18358_d_n8, assign23870_e18358_d_n9, assign23870_e18358_d_n10, assign23870_e18358_d_n11, assign23870_e18358_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23870_e18356: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign23870_e18356, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign23870_e18358;
        locals.var_tmf3_dn0 = assign23870_e18358_d_n0;
        locals.var_tmf3_dn2 = assign23870_e18358_d_n2;
        locals.var_tmf3_dn4 = assign23870_e18358_d_n4;
        locals.var_tmf3_dn5 = assign23870_e18358_d_n5;
        locals.var_tmf3_dn6 = assign23870_e18358_d_n6;
        locals.var_tmf3_dn7 = assign23870_e18358_d_n7;
        locals.var_tmf3_dn8 = assign23870_e18358_d_n8;
        locals.var_tmf3_dn9 = assign23870_e18358_d_n9;
        locals.var_tmf3_dn10 = assign23870_e18358_d_n10;
        locals.var_tmf3_dn11 = assign23870_e18358_d_n11;
        locals.var_tmf3_dn14 = assign23870_e18358_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign23880_e18366, assign23880_e18366_d_n0, assign23880_e18366_d_n2, assign23880_e18366_d_n4, assign23880_e18366_d_n5, assign23880_e18366_d_n6, assign23880_e18366_d_n7, assign23880_e18366_d_n8, assign23880_e18366_d_n9, assign23880_e18366_d_n10, assign23880_e18366_d_n11, assign23880_e18366_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23880_e18364: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign23880_e18364, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign23880_e18366;
        locals.var_tmf4_dn0 = assign23880_e18366_d_n0;
        locals.var_tmf4_dn2 = assign23880_e18366_d_n2;
        locals.var_tmf4_dn4 = assign23880_e18366_d_n4;
        locals.var_tmf4_dn5 = assign23880_e18366_d_n5;
        locals.var_tmf4_dn6 = assign23880_e18366_d_n6;
        locals.var_tmf4_dn7 = assign23880_e18366_d_n7;
        locals.var_tmf4_dn8 = assign23880_e18366_d_n8;
        locals.var_tmf4_dn9 = assign23880_e18366_d_n9;
        locals.var_tmf4_dn10 = assign23880_e18366_d_n10;
        locals.var_tmf4_dn11 = assign23880_e18366_d_n11;
        locals.var_tmf4_dn14 = assign23880_e18366_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign23890_e18382, assign23890_e18382_d_n0, assign23890_e18382_d_n2, assign23890_e18382_d_n4, assign23890_e18382_d_n5, assign23890_e18382_d_n6, assign23890_e18382_d_n7, assign23890_e18382_d_n8, assign23890_e18382_d_n9, assign23890_e18382_d_n10, assign23890_e18382_d_n11, assign23890_e18382_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23890_e18373: f64 = (1.0 + locals.var_tmf1);
        let assign23890_e18375: f64 = (assign23890_e18373 + locals.var_tmf2);
        let assign23890_e18377: f64 = (assign23890_e18375 + locals.var_tmf3);
        let assign23890_e18379: f64 = (assign23890_e18377 + locals.var_tmf4);
        let assign23890_e18380: f64 = (1.0 / assign23890_e18379);
        (assign23890_e18380, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign23890_e18379 * assign23890_e18379))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign23890_e18382;
        locals.var_tmf0_dn0 = assign23890_e18382_d_n0;
        locals.var_tmf0_dn2 = assign23890_e18382_d_n2;
        locals.var_tmf0_dn4 = assign23890_e18382_d_n4;
        locals.var_tmf0_dn5 = assign23890_e18382_d_n5;
        locals.var_tmf0_dn6 = assign23890_e18382_d_n6;
        locals.var_tmf0_dn7 = assign23890_e18382_d_n7;
        locals.var_tmf0_dn8 = assign23890_e18382_d_n8;
        locals.var_tmf0_dn9 = assign23890_e18382_d_n9;
        locals.var_tmf0_dn10 = assign23890_e18382_d_n10;
        locals.var_tmf0_dn11 = assign23890_e18382_d_n11;
        locals.var_tmf0_dn14 = assign23890_e18382_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign23900_e18405, assign23900_e18405_d_n0, assign23900_e18405_d_n2, assign23900_e18405_d_n4, assign23900_e18405_d_n5, assign23900_e18405_d_n6, assign23900_e18405_d_n7, assign23900_e18405_d_n8, assign23900_e18405_d_n9, assign23900_e18405_d_n10, assign23900_e18405_d_n11, assign23900_e18405_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23900_e18389: f64 = (2.0 * locals.var_tmf1);
        let assign23900_e18390: f64 = (1.0 + assign23900_e18389);
        let assign23900_e18393: f64 = (3.0 * locals.var_tmf2);
        let assign23900_e18394: f64 = (assign23900_e18390 + assign23900_e18393);
        let assign23900_e18397: f64 = (4.0 * locals.var_tmf3);
        let assign23900_e18398: f64 = (assign23900_e18394 + assign23900_e18397);
        let assign23900_e18399: f64 = (-assign23900_e18398);
        let assign23900_e18401: f64 = (assign23900_e18399 * locals.var_tmf0);
        let assign23900_e18403: f64 = (assign23900_e18401 * locals.var_tmf0);
        (assign23900_e18403, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23900_e18405;
        locals.var_vbscldvbs__blk438_dn0 = assign23900_e18405_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23900_e18405_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23900_e18405_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23900_e18405_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23900_e18405_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23900_e18405_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23900_e18405_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23900_e18405_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23900_e18405_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23900_e18405_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23900_e18405_d_n14;
        locals.var_vbscldvbs__blk438_rv = 0.0;

        let (assign23910_e18415, assign23910_e18415_d_n0, assign23910_e18415_d_n2, assign23910_e18415_d_n4, assign23910_e18415_d_n5, assign23910_e18415_d_n6, assign23910_e18415_d_n7, assign23910_e18415_d_n8, assign23910_e18415_d_n9, assign23910_e18415_d_n10, assign23910_e18415_d_n11, assign23910_e18415_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23910_e18412: f64 = (1.0 - locals.var_tmf0);
        let assign23910_e18413: f64 = (locals.var_t2 * assign23910_e18412);
        (assign23910_e18413, ((locals.var_t2_dn0 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign23910_e18415;
        locals.var_ty_dn0 = assign23910_e18415_d_n0;
        locals.var_ty_dn2 = assign23910_e18415_d_n2;
        locals.var_ty_dn4 = assign23910_e18415_d_n4;
        locals.var_ty_dn5 = assign23910_e18415_d_n5;
        locals.var_ty_dn6 = assign23910_e18415_d_n6;
        locals.var_ty_dn7 = assign23910_e18415_d_n7;
        locals.var_ty_dn8 = assign23910_e18415_d_n8;
        locals.var_ty_dn9 = assign23910_e18415_d_n9;
        locals.var_ty_dn10 = assign23910_e18415_d_n10;
        locals.var_ty_dn11 = assign23910_e18415_d_n11;
        locals.var_ty_dn14 = assign23910_e18415_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign23920_e18427, assign23920_e18427_d_n0, assign23920_e18427_d_n2, assign23920_e18427_d_n4, assign23920_e18427_d_n5, assign23920_e18427_d_n6, assign23920_e18427_d_n7, assign23920_e18427_d_n8, assign23920_e18427_d_n9, assign23920_e18427_d_n10, assign23920_e18427_d_n11, assign23920_e18427_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23920_e18421: f64 = (1.0 - locals.var_tmf0);
        let assign23920_e18424: f64 = (locals.var_tmf1 * locals.var_vbscldvbs__blk438);
        let assign23920_e18425: f64 = (assign23920_e18421 + assign23920_e18424);
        (assign23920_e18425, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23920_e18427;
        locals.var_t0_dn0 = assign23920_e18427_d_n0;
        locals.var_t0_dn2 = assign23920_e18427_d_n2;
        locals.var_t0_dn4 = assign23920_e18427_d_n4;
        locals.var_t0_dn5 = assign23920_e18427_d_n5;
        locals.var_t0_dn6 = assign23920_e18427_d_n6;
        locals.var_t0_dn7 = assign23920_e18427_d_n7;
        locals.var_t0_dn8 = assign23920_e18427_d_n8;
        locals.var_t0_dn9 = assign23920_e18427_d_n9;
        locals.var_t0_dn10 = assign23920_e18427_d_n10;
        locals.var_t0_dn11 = assign23920_e18427_d_n11;
        locals.var_t0_dn14 = assign23920_e18427_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign23930_e18434, assign23930_e18434_d_n0, assign23930_e18434_d_n2, assign23930_e18434_d_n4, assign23930_e18434_d_n5, assign23930_e18434_d_n6, assign23930_e18434_d_n7, assign23930_e18434_d_n8, assign23930_e18434_d_n9, assign23930_e18434_d_n10, assign23930_e18434_d_n11, assign23930_e18434_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23930_e18432: f64 = (-locals.var_vbscldvbs__blk438);
        (assign23930_e18432, (-locals.var_vbscldvbs__blk438_dn0), (-locals.var_vbscldvbs__blk438_dn2), (-locals.var_vbscldvbs__blk438_dn4), (-locals.var_vbscldvbs__blk438_dn5), (-locals.var_vbscldvbs__blk438_dn6), (-locals.var_vbscldvbs__blk438_dn7), (-locals.var_vbscldvbs__blk438_dn8), (-locals.var_vbscldvbs__blk438_dn9), (-locals.var_vbscldvbs__blk438_dn10), (-locals.var_vbscldvbs__blk438_dn11), (-locals.var_vbscldvbs__blk438_dn14),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23930_e18434;
        locals.var_vbscldvbs__blk438_dn0 = assign23930_e18434_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23930_e18434_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23930_e18434_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23930_e18434_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23930_e18434_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23930_e18434_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23930_e18434_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23930_e18434_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23930_e18434_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23930_e18434_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23930_e18434_d_n14;
        locals.var_vbscldvbs__blk438_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23940_e18442, assign23940_e18442_d_n0, assign23940_e18442_d_n2, assign23940_e18442_d_n4, assign23940_e18442_d_n5, assign23940_e18442_d_n6, assign23940_e18442_d_n7, assign23940_e18442_d_n8, assign23940_e18442_d_n9, assign23940_e18442_d_n10, assign23940_e18442_d_n11, assign23940_e18442_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23940_e18440: f64 = (locals.var_vbs_bnd_local + locals.var_ty);
        (assign23940_e18440, (locals.var_vbs_bnd_local_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_local_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_local_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_local_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_local_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_local_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_local_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_local_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_local_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_local_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_local_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign23940_e18442;
        locals.var_vbscl__blk437_dn0 = assign23940_e18442_d_n0;
        locals.var_vbscl__blk437_dn2 = assign23940_e18442_d_n2;
        locals.var_vbscl__blk437_dn4 = assign23940_e18442_d_n4;
        locals.var_vbscl__blk437_dn5 = assign23940_e18442_d_n5;
        locals.var_vbscl__blk437_dn6 = assign23940_e18442_d_n6;
        locals.var_vbscl__blk437_dn7 = assign23940_e18442_d_n7;
        locals.var_vbscl__blk437_dn8 = assign23940_e18442_d_n8;
        locals.var_vbscl__blk437_dn9 = assign23940_e18442_d_n9;
        locals.var_vbscl__blk437_dn10 = assign23940_e18442_d_n10;
        locals.var_vbscl__blk437_dn11 = assign23940_e18442_d_n11;
        locals.var_vbscl__blk437_dn14 = assign23940_e18442_d_n14;
        locals.var_vbscl__blk437_rv = 0.0;

        let (assign23950_e18450, assign23950_e18450_d_n0, assign23950_e18450_d_n2, assign23950_e18450_d_n4, assign23950_e18450_d_n5, assign23950_e18450_d_n6, assign23950_e18450_d_n7, assign23950_e18450_d_n8, assign23950_e18450_d_n9, assign23950_e18450_d_n10, assign23950_e18450_d_n11, assign23950_e18450_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23950_e18448: f64 = (1.0 / locals.var_t2);
        (assign23950_e18448, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23950_e18450;
        locals.var_t3_dn0 = assign23950_e18450_d_n0;
        locals.var_t3_dn2 = assign23950_e18450_d_n2;
        locals.var_t3_dn4 = assign23950_e18450_d_n4;
        locals.var_t3_dn5 = assign23950_e18450_d_n5;
        locals.var_t3_dn6 = assign23950_e18450_d_n6;
        locals.var_t3_dn7 = assign23950_e18450_d_n7;
        locals.var_t3_dn8 = assign23950_e18450_d_n8;
        locals.var_t3_dn9 = assign23950_e18450_d_n9;
        locals.var_t3_dn10 = assign23950_e18450_d_n10;
        locals.var_t3_dn11 = assign23950_e18450_d_n11;
        locals.var_t3_dn14 = assign23950_e18450_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23960_e18458, assign23960_e18458_d_n0, assign23960_e18458_d_n2, assign23960_e18458_d_n4, assign23960_e18458_d_n5, assign23960_e18458_d_n6, assign23960_e18458_d_n7, assign23960_e18458_d_n8, assign23960_e18458_d_n9, assign23960_e18458_d_n10, assign23960_e18458_d_n11, assign23960_e18458_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23960_e18456: f64 = (locals.var_t1 * locals.var_t3);
        (assign23960_e18456, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23960_e18458;
        locals.var_t4_dn0 = assign23960_e18458_d_n0;
        locals.var_t4_dn2 = assign23960_e18458_d_n2;
        locals.var_t4_dn4 = assign23960_e18458_d_n4;
        locals.var_t4_dn5 = assign23960_e18458_d_n5;
        locals.var_t4_dn6 = assign23960_e18458_d_n6;
        locals.var_t4_dn7 = assign23960_e18458_d_n7;
        locals.var_t4_dn8 = assign23960_e18458_d_n8;
        locals.var_t4_dn9 = assign23960_e18458_d_n9;
        locals.var_t4_dn10 = assign23960_e18458_d_n10;
        locals.var_t4_dn11 = assign23960_e18458_d_n11;
        locals.var_t4_dn14 = assign23960_e18458_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23970_e18466, assign23970_e18466_d_n0, assign23970_e18466_d_n2, assign23970_e18466_d_n4, assign23970_e18466_d_n5, assign23970_e18466_d_n6, assign23970_e18466_d_n7, assign23970_e18466_d_n8, assign23970_e18466_d_n9, assign23970_e18466_d_n10, assign23970_e18466_d_n11, assign23970_e18466_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23970_e18464: f64 = (locals.var_t4 * locals.var_t4);
        (assign23970_e18464, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23970_e18466;
        locals.var_t5_dn0 = assign23970_e18466_d_n0;
        locals.var_t5_dn2 = assign23970_e18466_d_n2;
        locals.var_t5_dn4 = assign23970_e18466_d_n4;
        locals.var_t5_dn5 = assign23970_e18466_d_n5;
        locals.var_t5_dn6 = assign23970_e18466_d_n6;
        locals.var_t5_dn7 = assign23970_e18466_d_n7;
        locals.var_t5_dn8 = assign23970_e18466_d_n8;
        locals.var_t5_dn9 = assign23970_e18466_d_n9;
        locals.var_t5_dn10 = assign23970_e18466_d_n10;
        locals.var_t5_dn11 = assign23970_e18466_d_n11;
        locals.var_t5_dn14 = assign23970_e18466_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23980_e18482, assign23980_e18482_d_n0, assign23980_e18482_d_n2, assign23980_e18482_d_n4, assign23980_e18482_d_n5, assign23980_e18482_d_n6, assign23980_e18482_d_n7, assign23980_e18482_d_n8, assign23980_e18482_d_n9, assign23980_e18482_d_n10, assign23980_e18482_d_n11, assign23980_e18482_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23980_e18472: f64 = (1.0 + locals.var_t4);
        let assign23980_e18476: f64 = (1.0 + locals.var_t4);
        let assign23980_e18478: f64 = (assign23980_e18476 + locals.var_t5);
        let assign23980_e18479: f64 = (locals.var_t5 * assign23980_e18478);
        let assign23980_e18480: f64 = (assign23980_e18472 + assign23980_e18479);
        (assign23980_e18480, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23980_e18482;
        locals.var_t7_dn0 = assign23980_e18482_d_n0;
        locals.var_t7_dn2 = assign23980_e18482_d_n2;
        locals.var_t7_dn4 = assign23980_e18482_d_n4;
        locals.var_t7_dn5 = assign23980_e18482_d_n5;
        locals.var_t7_dn6 = assign23980_e18482_d_n6;
        locals.var_t7_dn7 = assign23980_e18482_d_n7;
        locals.var_t7_dn8 = assign23980_e18482_d_n8;
        locals.var_t7_dn9 = assign23980_e18482_d_n9;
        locals.var_t7_dn10 = assign23980_e18482_d_n10;
        locals.var_t7_dn11 = assign23980_e18482_d_n11;
        locals.var_t7_dn14 = assign23980_e18482_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23990_e18506, assign23990_e18506_d_n0, assign23990_e18506_d_n2, assign23990_e18506_d_n4, assign23990_e18506_d_n5, assign23990_e18506_d_n6, assign23990_e18506_d_n7, assign23990_e18506_d_n8, assign23990_e18506_d_n9, assign23990_e18506_d_n10, assign23990_e18506_d_n11, assign23990_e18506_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23990_e18489: f64 = (2.0 * locals.var_t4);
        let assign23990_e18490: f64 = (1.0 + assign23990_e18489);
        let assign23990_e18493: f64 = (3.0 * locals.var_t5);
        let assign23990_e18494: f64 = (assign23990_e18490 + assign23990_e18493);
        let assign23990_e18497: f64 = (4.0 * locals.var_t4);
        let assign23990_e18499: f64 = (assign23990_e18497 * locals.var_t5);
        let assign23990_e18500: f64 = (assign23990_e18494 + assign23990_e18499);
        let assign23990_e18503: f64 = (locals.var_t7 * locals.var_t7);
        let assign23990_e18504: f64 = (assign23990_e18500 / assign23990_e18503);
        (assign23990_e18504, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn0))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn2))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn4))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn5))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn6))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn7))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn8))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn9))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn10))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn11))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn14))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign23990_e18503 * assign23990_e18503)),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23990_e18506;
        locals.var_vbscldvbs__blk438_dn0 = assign23990_e18506_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23990_e18506_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23990_e18506_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23990_e18506_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23990_e18506_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23990_e18506_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23990_e18506_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23990_e18506_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23990_e18506_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23990_e18506_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23990_e18506_d_n14;
        locals.var_vbscldvbs__blk438_rv = 0.0;

        let (assign24000_e18513, assign24000_e18513_d_n0, assign24000_e18513_d_n2, assign24000_e18513_d_n4, assign24000_e18513_d_n5, assign24000_e18513_d_n6, assign24000_e18513_d_n7, assign24000_e18513_d_n8, assign24000_e18513_d_n9, assign24000_e18513_d_n10, assign24000_e18513_d_n11, assign24000_e18513_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 == 0.0)) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign24000_e18513;
        locals.var_vbscl__blk437_dn0 = assign24000_e18513_d_n0;
        locals.var_vbscl__blk437_dn2 = assign24000_e18513_d_n2;
        locals.var_vbscl__blk437_dn4 = assign24000_e18513_d_n4;
        locals.var_vbscl__blk437_dn5 = assign24000_e18513_d_n5;
        locals.var_vbscl__blk437_dn6 = assign24000_e18513_d_n6;
        locals.var_vbscl__blk437_dn7 = assign24000_e18513_d_n7;
        locals.var_vbscl__blk437_dn8 = assign24000_e18513_d_n8;
        locals.var_vbscl__blk437_dn9 = assign24000_e18513_d_n9;
        locals.var_vbscl__blk437_dn10 = assign24000_e18513_d_n10;
        locals.var_vbscl__blk437_dn11 = assign24000_e18513_d_n11;
        locals.var_vbscl__blk437_dn14 = assign24000_e18513_d_n14;
        locals.var_vbscl__blk437_rv = 0.0;

        let (assign24010_e18520, assign24010_e18520_d_n0, assign24010_e18520_d_n2, assign24010_e18520_d_n4, assign24010_e18520_d_n5, assign24010_e18520_d_n6, assign24010_e18520_d_n7, assign24010_e18520_d_n8, assign24010_e18520_d_n9, assign24010_e18520_d_n10, assign24010_e18520_d_n11, assign24010_e18520_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign24010_e18520;
        locals.var_vbscldvbs__blk438_dn0 = assign24010_e18520_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign24010_e18520_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign24010_e18520_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign24010_e18520_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign24010_e18520_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign24010_e18520_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign24010_e18520_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign24010_e18520_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign24010_e18520_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign24010_e18520_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign24010_e18520_d_n14;
        locals.var_vbscldvbs__blk438_rv = 0.0;

        let (assign24020_e18525, assign24020_e18525_d_n0, assign24020_e18525_d_n2, assign24020_e18525_d_n4, assign24020_e18525_d_n5, assign24020_e18525_d_n6, assign24020_e18525_d_n7, assign24020_e18525_d_n8, assign24020_e18525_d_n9, assign24020_e18525_d_n10, assign24020_e18525_d_n11, assign24020_e18525_d_n14,) = {
    if (p.p37 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign24020_e18525;
        locals.var_vbscl__blk437_dn0 = assign24020_e18525_d_n0;
        locals.var_vbscl__blk437_dn2 = assign24020_e18525_d_n2;
        locals.var_vbscl__blk437_dn4 = assign24020_e18525_d_n4;
        locals.var_vbscl__blk437_dn5 = assign24020_e18525_d_n5;
        locals.var_vbscl__blk437_dn6 = assign24020_e18525_d_n6;
        locals.var_vbscl__blk437_dn7 = assign24020_e18525_d_n7;
        locals.var_vbscl__blk437_dn8 = assign24020_e18525_d_n8;
        locals.var_vbscl__blk437_dn9 = assign24020_e18525_d_n9;
        locals.var_vbscl__blk437_dn10 = assign24020_e18525_d_n10;
        locals.var_vbscl__blk437_dn11 = assign24020_e18525_d_n11;
        locals.var_vbscl__blk437_dn14 = assign24020_e18525_d_n14;
        locals.var_vbscl__blk437_rv = 0.0;

        let (assign24030_e18530, assign24030_e18530_d_n0, assign24030_e18530_d_n2, assign24030_e18530_d_n4, assign24030_e18530_d_n5, assign24030_e18530_d_n6, assign24030_e18530_d_n7, assign24030_e18530_d_n8, assign24030_e18530_d_n9, assign24030_e18530_d_n10, assign24030_e18530_d_n11, assign24030_e18530_d_n14,) = {
    if (p.p37 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign24030_e18530;
        locals.var_vbscldvbs__blk438_dn0 = assign24030_e18530_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign24030_e18530_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign24030_e18530_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign24030_e18530_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign24030_e18530_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign24030_e18530_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign24030_e18530_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign24030_e18530_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign24030_e18530_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign24030_e18530_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign24030_e18530_d_n14;
        locals.var_vbscldvbs__blk438_rv = 0.0;

        let assign24040_e18533: f64 = (locals.var_vbscldvbs__blk438 * locals.var_vds);
        let assign24040_e18535: f64 = (assign24040_e18533 / 2.0);
        locals.var_t1 = assign24040_e18535;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs__blk438_dn0 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs__blk438_dn2 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs__blk438_dn4 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs__blk438_dn5 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs__blk438_dn6 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs__blk438_dn7 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs__blk438_dn8 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs__blk438_dn9 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs__blk438_dn10 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs__blk438_dn11 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs__blk438_dn14 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn14)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign24050_e18538: f64 = (2.0 * locals.var_t1);
        let assign24050_e18540: f64 = (assign24050_e18538 / p.p262);
        locals.var_tmf1 = assign24050_e18540;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p262);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign24060_e18545: f64 = (1.0 / 2.0);
        let assign24060_e18549: f64 = (1.0 / 6.0);
        let assign24060_e18553: f64 = (1.0 / 24.0);
        let assign24060_e18557: f64 = (1.0 / 120.0);
        let assign24060_e18561: f64 = (1.0 / 720.0);
        let assign24060_e18565: f64 = (1.0 / 5040.0);
        let assign24060_e18566: f64 = (locals.var_tmf1 * assign24060_e18565);
        let assign24060_e18567: f64 = (assign24060_e18561 + assign24060_e18566);
        let assign24060_e18568: f64 = (locals.var_tmf1 * assign24060_e18567);
        let assign24060_e18569: f64 = (assign24060_e18557 + assign24060_e18568);
        let assign24060_e18570: f64 = (locals.var_tmf1 * assign24060_e18569);
        let assign24060_e18571: f64 = (assign24060_e18553 + assign24060_e18570);
        let assign24060_e18572: f64 = (locals.var_tmf1 * assign24060_e18571);
        let assign24060_e18573: f64 = (assign24060_e18549 + assign24060_e18572);
        let assign24060_e18574: f64 = (locals.var_tmf1 * assign24060_e18573);
        let assign24060_e18575: f64 = (assign24060_e18545 + assign24060_e18574);
        let assign24060_e18576: f64 = (locals.var_tmf1 * assign24060_e18575);
        let assign24060_e18577: f64 = (1.0 + assign24060_e18576);
        locals.var_tmf2 = assign24060_e18577;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24060_e18565)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign24070_e18580: f64 = (1.0 / 2.0);
        let assign24070_e18584: f64 = (1.0 / 3.0);
        let assign24070_e18588: f64 = (1.0 / 8.0);
        let assign24070_e18592: f64 = (1.0 / 30.0);
        let assign24070_e18596: f64 = (1.0 / 144.0);
        let assign24070_e18600: f64 = (1.0 / 840.0);
        let assign24070_e18601: f64 = (locals.var_tmf1 * assign24070_e18600);
        let assign24070_e18602: f64 = (assign24070_e18596 + assign24070_e18601);
        let assign24070_e18603: f64 = (locals.var_tmf1 * assign24070_e18602);
        let assign24070_e18604: f64 = (assign24070_e18592 + assign24070_e18603);
        let assign24070_e18605: f64 = (locals.var_tmf1 * assign24070_e18604);
        let assign24070_e18606: f64 = (assign24070_e18588 + assign24070_e18605);
        let assign24070_e18607: f64 = (locals.var_tmf1 * assign24070_e18606);
        let assign24070_e18608: f64 = (assign24070_e18584 + assign24070_e18607);
        let assign24070_e18609: f64 = (locals.var_tmf1 * assign24070_e18608);
        let assign24070_e18610: f64 = (assign24070_e18580 + assign24070_e18609);
        locals.var_tmf3 = assign24070_e18610;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24070_e18600)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24070_e18600)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24070_e18600)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24070_e18600)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24070_e18600)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24070_e18600)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24070_e18600)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24070_e18600)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24070_e18600)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24070_e18600)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24070_e18600)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign24080_e18613: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd__blk439 = assign24080_e18613;
        locals.var_vzadd__blk439_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_rv = 0.0;

        let assign24090_e18615: f64 = (-2.0);
        let assign24090_e18617: f64 = (assign24090_e18615 * locals.var_tmf3);
        let assign24090_e18620: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign24090_e18621: f64 = (assign24090_e18617 / assign24090_e18620);
        locals.var_t2 = assign24090_e18621;
        locals.var_t2_dn0 = ((((assign24090_e18615 * locals.var_tmf3_dn0) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn2 = ((((assign24090_e18615 * locals.var_tmf3_dn2) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn4 = ((((assign24090_e18615 * locals.var_tmf3_dn4) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn5 = ((((assign24090_e18615 * locals.var_tmf3_dn5) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn6 = ((((assign24090_e18615 * locals.var_tmf3_dn6) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn7 = ((((assign24090_e18615 * locals.var_tmf3_dn7) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn8 = ((((assign24090_e18615 * locals.var_tmf3_dn8) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn9 = ((((assign24090_e18615 * locals.var_tmf3_dn9) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn10 = ((((assign24090_e18615 * locals.var_tmf3_dn10) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn11 = ((((assign24090_e18615 * locals.var_tmf3_dn11) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn14 = ((((assign24090_e18615 * locals.var_tmf3_dn14) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_rv = 0.0;

        let assign24100_e18624: f64 = if locals.var_vzadd__blk439 < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24100_e18624;
        locals.var_guard444_rv = 0.0;

        let (assign24110_e18628, assign24110_e18628_d_n0, assign24110_e18628_d_n2, assign24110_e18628_d_n4, assign24110_e18628_d_n5, assign24110_e18628_d_n6, assign24110_e18628_d_n7, assign24110_e18628_d_n8, assign24110_e18628_d_n9, assign24110_e18628_d_n10, assign24110_e18628_d_n11, assign24110_e18628_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk439, locals.var_vzadd__blk439_dn0, locals.var_vzadd__blk439_dn2, locals.var_vzadd__blk439_dn4, locals.var_vzadd__blk439_dn5, locals.var_vzadd__blk439_dn6, locals.var_vzadd__blk439_dn7, locals.var_vzadd__blk439_dn8, locals.var_vzadd__blk439_dn9, locals.var_vzadd__blk439_dn10, locals.var_vzadd__blk439_dn11, locals.var_vzadd__blk439_dn14,)
    }
};
        locals.var_vzadd__blk439 = assign24110_e18628;
        locals.var_vzadd__blk439_dn0 = assign24110_e18628_d_n0;
        locals.var_vzadd__blk439_dn2 = assign24110_e18628_d_n2;
        locals.var_vzadd__blk439_dn4 = assign24110_e18628_d_n4;
        locals.var_vzadd__blk439_dn5 = assign24110_e18628_d_n5;
        locals.var_vzadd__blk439_dn6 = assign24110_e18628_d_n6;
        locals.var_vzadd__blk439_dn7 = assign24110_e18628_d_n7;
        locals.var_vzadd__blk439_dn8 = assign24110_e18628_d_n8;
        locals.var_vzadd__blk439_dn9 = assign24110_e18628_d_n9;
        locals.var_vzadd__blk439_dn10 = assign24110_e18628_d_n10;
        locals.var_vzadd__blk439_dn11 = assign24110_e18628_d_n11;
        locals.var_vzadd__blk439_dn14 = assign24110_e18628_d_n14;
        locals.var_vzadd__blk439_rv = 0.0;

        let assign24120_e18631: f64 = (locals.var_vbscl__blk437 + locals.var_vzadd__blk439);
        locals.var_vbsz__blk440 = assign24120_e18631;
        locals.var_vbsz__blk440_dn0 = (locals.var_vbscl__blk437_dn0 + locals.var_vzadd__blk439_dn0);
        locals.var_vbsz__blk440_dn2 = (locals.var_vbscl__blk437_dn2 + locals.var_vzadd__blk439_dn2);
        locals.var_vbsz__blk440_dn4 = (locals.var_vbscl__blk437_dn4 + locals.var_vzadd__blk439_dn4);
        locals.var_vbsz__blk440_dn5 = (locals.var_vbscl__blk437_dn5 + locals.var_vzadd__blk439_dn5);
        locals.var_vbsz__blk440_dn6 = (locals.var_vbscl__blk437_dn6 + locals.var_vzadd__blk439_dn6);
        locals.var_vbsz__blk440_dn7 = (locals.var_vbscl__blk437_dn7 + locals.var_vzadd__blk439_dn7);
        locals.var_vbsz__blk440_dn8 = (locals.var_vbscl__blk437_dn8 + locals.var_vzadd__blk439_dn8);
        locals.var_vbsz__blk440_dn9 = (locals.var_vbscl__blk437_dn9 + locals.var_vzadd__blk439_dn9);
        locals.var_vbsz__blk440_dn10 = (locals.var_vbscl__blk437_dn10 + locals.var_vzadd__blk439_dn10);
        locals.var_vbsz__blk440_dn11 = (locals.var_vbscl__blk437_dn11 + locals.var_vzadd__blk439_dn11);
        locals.var_vbsz__blk440_dn14 = (locals.var_vbscl__blk437_dn14 + locals.var_vzadd__blk439_dn14);
        locals.var_vbsz__blk440_rv = 0.0;

        let assign24130_e18635: f64 = (2.0 * locals.var_vzadd__blk439);
        let assign24130_e18636: f64 = (locals.var_vds + assign24130_e18635);
        locals.var_vdsz__blk441 = assign24130_e18636;
        locals.var_vdsz__blk441_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd__blk439_dn0));
        locals.var_vdsz__blk441_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd__blk439_dn2));
        locals.var_vdsz__blk441_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd__blk439_dn4));
        locals.var_vdsz__blk441_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd__blk439_dn5));
        locals.var_vdsz__blk441_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd__blk439_dn6));
        locals.var_vdsz__blk441_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd__blk439_dn7));
        locals.var_vdsz__blk441_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd__blk439_dn8));
        locals.var_vdsz__blk441_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd__blk439_dn9));
        locals.var_vdsz__blk441_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd__blk439_dn10));
        locals.var_vdsz__blk441_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd__blk439_dn11));
        locals.var_vdsz__blk441_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd__blk439_dn14));
        locals.var_vdsz__blk441_rv = 0.0;

        let assign24140_e18639: f64 = (locals.var_vgs + locals.var_vzadd__blk439);
        locals.var_vgsz__blk442 = assign24140_e18639;
        locals.var_vgsz__blk442_dn0 = locals.var_vzadd__blk439_dn0;
        locals.var_vgsz__blk442_dn2 = locals.var_vzadd__blk439_dn2;
        locals.var_vgsz__blk442_dn4 = locals.var_vzadd__blk439_dn4;
        locals.var_vgsz__blk442_dn5 = locals.var_vzadd__blk439_dn5;
        locals.var_vgsz__blk442_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd__blk439_dn6);
        locals.var_vgsz__blk442_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd__blk439_dn7);
        locals.var_vgsz__blk442_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd__blk439_dn8);
        locals.var_vgsz__blk442_dn9 = locals.var_vzadd__blk439_dn9;
        locals.var_vgsz__blk442_dn10 = locals.var_vzadd__blk439_dn10;
        locals.var_vgsz__blk442_dn11 = locals.var_vzadd__blk439_dn11;
        locals.var_vgsz__blk442_dn14 = locals.var_vzadd__blk439_dn14;
        locals.var_vgsz__blk442_rv = 0.0;

        let assign24150_e18642: f64 = (locals.var_vgs - locals.var_vfb);
        let assign24150_e18644: f64 = (assign24150_e18642 + locals.var_dvth);
        let assign24150_e18646: f64 = (assign24150_e18644 - locals.var_dppg);
        locals.var_vgp = assign24150_e18646;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn4 = (locals.var_dvth_dn4 - locals.var_dppg_dn4);
        locals.var_vgp_dn5 = (locals.var_dvth_dn5 - locals.var_dppg_dn5);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn8 = ((locals.var_vgs_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8);
        locals.var_vgp_dn9 = (locals.var_dvth_dn9 - locals.var_dppg_dn9);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = (locals.var_dvth_dn11 - locals.var_dppg_dn11);
        locals.var_vgp_dn14 = (locals.var_dvth_dn14 - locals.var_dppg_dn14);
        locals.var_vgp_rv = 0.0;

        let assign24160_e18649: f64 = (locals.var_vfb - locals.var_dvth);
        let assign24160_e18651: f64 = (assign24160_e18649 + locals.var_dppg);
        let assign24160_e18653: f64 = (assign24160_e18651 + locals.var_vbscl__blk437);
        locals.var_vgs_fb = assign24160_e18653;
        locals.var_vgs_fb_dn0 = (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk437_dn0);
        locals.var_vgs_fb_dn2 = (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk437_dn2);
        locals.var_vgs_fb_dn4 = (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk437_dn4);
        locals.var_vgs_fb_dn5 = (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk437_dn5);
        locals.var_vgs_fb_dn6 = (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk437_dn6);
        locals.var_vgs_fb_dn7 = (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk437_dn7);
        locals.var_vgs_fb_dn8 = (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk437_dn8);
        locals.var_vgs_fb_dn9 = (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk437_dn9);
        locals.var_vgs_fb_dn10 = (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk437_dn10);
        locals.var_vgs_fb_dn11 = (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbscl__blk437_dn11);
        locals.var_vgs_fb_dn14 = (((-locals.var_dvth_dn14) + locals.var_dppg_dn14) + locals.var_vbscl__blk437_dn14);
        locals.var_vgs_fb_rv = 0.0;

        let assign24170_e18656: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign24170_e18656;
        locals.var_guard445_rv = 0.0;

        let assign24180_e18659: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24180_e18659;
        locals.var_guard446_rv = 0.0;

        let assign24190_e18662: f64 = if p.p42 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign24190_e18662;
        locals.var_guard447_rv = 0.0;

        let assign24200_e18665: f64 = if p.p42 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign24200_e18665;
        locals.var_guard448_rv = 0.0;

        let (assign24210_e18671, assign24210_e18671_d_n0, assign24210_e18671_d_n2, assign24210_e18671_d_n4, assign24210_e18671_d_n5, assign24210_e18671_d_n6, assign24210_e18671_d_n7, assign24210_e18671_d_n8, assign24210_e18671_d_n9, assign24210_e18671_d_n10, assign24210_e18671_d_n11, assign24210_e18671_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbi_dep, locals.var_vbi_dep_dn0, locals.var_vbi_dep_dn2, locals.var_vbi_dep_dn4, locals.var_vbi_dep_dn5, locals.var_vbi_dep_dn6, locals.var_vbi_dep_dn7, locals.var_vbi_dep_dn8, locals.var_vbi_dep_dn9, locals.var_vbi_dep_dn10, locals.var_vbi_dep_dn11, locals.var_vbi_dep_dn14,)
    }
};
        locals.var_vbi_dep = assign24210_e18671;
        locals.var_vbi_dep_dn0 = assign24210_e18671_d_n0;
        locals.var_vbi_dep_dn2 = assign24210_e18671_d_n2;
        locals.var_vbi_dep_dn4 = assign24210_e18671_d_n4;
        locals.var_vbi_dep_dn5 = assign24210_e18671_d_n5;
        locals.var_vbi_dep_dn6 = assign24210_e18671_d_n6;
        locals.var_vbi_dep_dn7 = assign24210_e18671_d_n7;
        locals.var_vbi_dep_dn8 = assign24210_e18671_d_n8;
        locals.var_vbi_dep_dn9 = assign24210_e18671_d_n9;
        locals.var_vbi_dep_dn10 = assign24210_e18671_d_n10;
        locals.var_vbi_dep_dn11 = assign24210_e18671_d_n11;
        locals.var_vbi_dep_dn14 = assign24210_e18671_d_n14;
        locals.var_vbi_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign24220_e18679, assign24220_e18679_d_n0, assign24220_e18679_d_n2, assign24220_e18679_d_n4, assign24220_e18679_d_n5, assign24220_e18679_d_n6, assign24220_e18679_d_n7, assign24220_e18679_d_n8, assign24220_e18679_d_n9, assign24220_e18679_d_n10, assign24220_e18679_d_n11, assign24220_e18679_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24220_e18677: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign24220_e18677, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn11), (1.6021918e-19 * locals.var_uc_ndepm_dn14),)
    } else {
        (locals.var_q_ndepm, locals.var_q_ndepm_dn0, locals.var_q_ndepm_dn2, locals.var_q_ndepm_dn4, locals.var_q_ndepm_dn5, locals.var_q_ndepm_dn6, locals.var_q_ndepm_dn7, locals.var_q_ndepm_dn8, locals.var_q_ndepm_dn9, locals.var_q_ndepm_dn10, locals.var_q_ndepm_dn11, locals.var_q_ndepm_dn14,)
    }
};
        locals.var_q_ndepm = assign24220_e18679;
        locals.var_q_ndepm_dn0 = assign24220_e18679_d_n0;
        locals.var_q_ndepm_dn2 = assign24220_e18679_d_n2;
        locals.var_q_ndepm_dn4 = assign24220_e18679_d_n4;
        locals.var_q_ndepm_dn5 = assign24220_e18679_d_n5;
        locals.var_q_ndepm_dn6 = assign24220_e18679_d_n6;
        locals.var_q_ndepm_dn7 = assign24220_e18679_d_n7;
        locals.var_q_ndepm_dn8 = assign24220_e18679_d_n8;
        locals.var_q_ndepm_dn9 = assign24220_e18679_d_n9;
        locals.var_q_ndepm_dn10 = assign24220_e18679_d_n10;
        locals.var_q_ndepm_dn11 = assign24220_e18679_d_n11;
        locals.var_q_ndepm_dn14 = assign24220_e18679_d_n14;
        locals.var_q_ndepm_rv = 0.0;

        let (assign24230_e18687, assign24230_e18687_d_n0, assign24230_e18687_d_n2, assign24230_e18687_d_n4, assign24230_e18687_d_n5, assign24230_e18687_d_n6, assign24230_e18687_d_n7, assign24230_e18687_d_n8, assign24230_e18687_d_n9, assign24230_e18687_d_n10, assign24230_e18687_d_n11, assign24230_e18687_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24230_e18685: f64 = (locals.var_uc_ndepm * locals.var_uc_ndepm);
        (assign24230_e18685, ((locals.var_uc_ndepm_dn0 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn0)), ((locals.var_uc_ndepm_dn2 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn2)), ((locals.var_uc_ndepm_dn4 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn4)), ((locals.var_uc_ndepm_dn5 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn5)), ((locals.var_uc_ndepm_dn6 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn6)), ((locals.var_uc_ndepm_dn7 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn7)), ((locals.var_uc_ndepm_dn8 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn8)), ((locals.var_uc_ndepm_dn9 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn9)), ((locals.var_uc_ndepm_dn10 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn10)), ((locals.var_uc_ndepm_dn11 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn11)), ((locals.var_uc_ndepm_dn14 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn14)),)
    } else {
        (locals.var_ndepm2, locals.var_ndepm2_dn0, locals.var_ndepm2_dn2, locals.var_ndepm2_dn4, locals.var_ndepm2_dn5, locals.var_ndepm2_dn6, locals.var_ndepm2_dn7, locals.var_ndepm2_dn8, locals.var_ndepm2_dn9, locals.var_ndepm2_dn10, locals.var_ndepm2_dn11, locals.var_ndepm2_dn14,)
    }
};
        locals.var_ndepm2 = assign24230_e18687;
        locals.var_ndepm2_dn0 = assign24230_e18687_d_n0;
        locals.var_ndepm2_dn2 = assign24230_e18687_d_n2;
        locals.var_ndepm2_dn4 = assign24230_e18687_d_n4;
        locals.var_ndepm2_dn5 = assign24230_e18687_d_n5;
        locals.var_ndepm2_dn6 = assign24230_e18687_d_n6;
        locals.var_ndepm2_dn7 = assign24230_e18687_d_n7;
        locals.var_ndepm2_dn8 = assign24230_e18687_d_n8;
        locals.var_ndepm2_dn9 = assign24230_e18687_d_n9;
        locals.var_ndepm2_dn10 = assign24230_e18687_d_n10;
        locals.var_ndepm2_dn11 = assign24230_e18687_d_n11;
        locals.var_ndepm2_dn14 = assign24230_e18687_d_n14;
        locals.var_ndepm2_rv = 0.0;

        let (assign24240_e18697, assign24240_e18697_d_n0, assign24240_e18697_d_n2, assign24240_e18697_d_n4, assign24240_e18697_d_n5, assign24240_e18697_d_n6, assign24240_e18697_d_n7, assign24240_e18697_d_n8, assign24240_e18697_d_n9, assign24240_e18697_d_n10, assign24240_e18697_d_n11, assign24240_e18697_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24240_e18693: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign24240_e18695: f64 = (assign24240_e18693 * 1.034943e-10);
        (assign24240_e18695, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn11) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn14) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi, locals.var_q_ndepm_esi_dn0, locals.var_q_ndepm_esi_dn2, locals.var_q_ndepm_esi_dn4, locals.var_q_ndepm_esi_dn5, locals.var_q_ndepm_esi_dn6, locals.var_q_ndepm_esi_dn7, locals.var_q_ndepm_esi_dn8, locals.var_q_ndepm_esi_dn9, locals.var_q_ndepm_esi_dn10, locals.var_q_ndepm_esi_dn11, locals.var_q_ndepm_esi_dn14,)
    }
};
        locals.var_q_ndepm_esi = assign24240_e18697;
        locals.var_q_ndepm_esi_dn0 = assign24240_e18697_d_n0;
        locals.var_q_ndepm_esi_dn2 = assign24240_e18697_d_n2;
        locals.var_q_ndepm_esi_dn4 = assign24240_e18697_d_n4;
        locals.var_q_ndepm_esi_dn5 = assign24240_e18697_d_n5;
        locals.var_q_ndepm_esi_dn6 = assign24240_e18697_d_n6;
        locals.var_q_ndepm_esi_dn7 = assign24240_e18697_d_n7;
        locals.var_q_ndepm_esi_dn8 = assign24240_e18697_d_n8;
        locals.var_q_ndepm_esi_dn9 = assign24240_e18697_d_n9;
        locals.var_q_ndepm_esi_dn10 = assign24240_e18697_d_n10;
        locals.var_q_ndepm_esi_dn11 = assign24240_e18697_d_n11;
        locals.var_q_ndepm_esi_dn14 = assign24240_e18697_d_n14;
        locals.var_q_ndepm_esi_rv = 0.0;

        let (assign24250_e18705, assign24250_e18705_d_n0, assign24250_e18705_d_n2, assign24250_e18705_d_n4, assign24250_e18705_d_n5, assign24250_e18705_d_n6, assign24250_e18705_d_n7, assign24250_e18705_d_n8, assign24250_e18705_d_n9, assign24250_e18705_d_n10, assign24250_e18705_d_n11, assign24250_e18705_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24250_e18703: f64 = (1.6021918e-19 * locals.var_ef_nsubc);
        (assign24250_e18703, (1.6021918e-19 * locals.var_ef_nsubc_dn0), (1.6021918e-19 * locals.var_ef_nsubc_dn2), (1.6021918e-19 * locals.var_ef_nsubc_dn4), (1.6021918e-19 * locals.var_ef_nsubc_dn5), (1.6021918e-19 * locals.var_ef_nsubc_dn6), (1.6021918e-19 * locals.var_ef_nsubc_dn7), (1.6021918e-19 * locals.var_ef_nsubc_dn8), (1.6021918e-19 * locals.var_ef_nsubc_dn9), (1.6021918e-19 * locals.var_ef_nsubc_dn10), (1.6021918e-19 * locals.var_ef_nsubc_dn11), (1.6021918e-19 * locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_q_nsub__blk546, locals.var_q_nsub__blk546_dn0, locals.var_q_nsub__blk546_dn2, locals.var_q_nsub__blk546_dn4, locals.var_q_nsub__blk546_dn5, locals.var_q_nsub__blk546_dn6, locals.var_q_nsub__blk546_dn7, locals.var_q_nsub__blk546_dn8, locals.var_q_nsub__blk546_dn9, locals.var_q_nsub__blk546_dn10, locals.var_q_nsub__blk546_dn11, locals.var_q_nsub__blk546_dn14,)
    }
};
        locals.var_q_nsub__blk546 = assign24250_e18705;
        locals.var_q_nsub__blk546_dn0 = assign24250_e18705_d_n0;
        locals.var_q_nsub__blk546_dn2 = assign24250_e18705_d_n2;
        locals.var_q_nsub__blk546_dn4 = assign24250_e18705_d_n4;
        locals.var_q_nsub__blk546_dn5 = assign24250_e18705_d_n5;
        locals.var_q_nsub__blk546_dn6 = assign24250_e18705_d_n6;
        locals.var_q_nsub__blk546_dn7 = assign24250_e18705_d_n7;
        locals.var_q_nsub__blk546_dn8 = assign24250_e18705_d_n8;
        locals.var_q_nsub__blk546_dn9 = assign24250_e18705_d_n9;
        locals.var_q_nsub__blk546_dn10 = assign24250_e18705_d_n10;
        locals.var_q_nsub__blk546_dn11 = assign24250_e18705_d_n11;
        locals.var_q_nsub__blk546_dn14 = assign24250_e18705_d_n14;
        locals.var_q_nsub__blk546_rv = 0.0;

        let (assign24260_e18713,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24260_e18711: f64 = (1.6021918e-19 * 1.6021918e-19);
        (assign24260_e18711,)
    } else {
        (locals.var_c_qe2,)
    }
};
        locals.var_c_qe2 = assign24260_e18713;
        locals.var_c_qe2_rv = 0.0;

        let (assign24270_e18721,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24270_e18719: f64 = (1.034943e-10 * 1.034943e-10);
        (assign24270_e18719,)
    } else {
        (locals.var_c_esi2,)
    }
};
        locals.var_c_esi2 = assign24270_e18721;
        locals.var_c_esi2_rv = 0.0;

        let (assign24280_e18729, assign24280_e18729_d_n0, assign24280_e18729_d_n2, assign24280_e18729_d_n4, assign24280_e18729_d_n5, assign24280_e18729_d_n6, assign24280_e18729_d_n7, assign24280_e18729_d_n8, assign24280_e18729_d_n9, assign24280_e18729_d_n10, assign24280_e18729_d_n11, assign24280_e18729_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24280_e18727: f64 = (locals.var_uc_depthn * locals.var_uc_depthn);
        (assign24280_e18727, ((locals.var_uc_depthn_dn0 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn14)),)
    } else {
        (locals.var_tn2, locals.var_tn2_dn0, locals.var_tn2_dn2, locals.var_tn2_dn4, locals.var_tn2_dn5, locals.var_tn2_dn6, locals.var_tn2_dn7, locals.var_tn2_dn8, locals.var_tn2_dn9, locals.var_tn2_dn10, locals.var_tn2_dn11, locals.var_tn2_dn14,)
    }
};
        locals.var_tn2 = assign24280_e18729;
        locals.var_tn2_dn0 = assign24280_e18729_d_n0;
        locals.var_tn2_dn2 = assign24280_e18729_d_n2;
        locals.var_tn2_dn4 = assign24280_e18729_d_n4;
        locals.var_tn2_dn5 = assign24280_e18729_d_n5;
        locals.var_tn2_dn6 = assign24280_e18729_d_n6;
        locals.var_tn2_dn7 = assign24280_e18729_d_n7;
        locals.var_tn2_dn8 = assign24280_e18729_d_n8;
        locals.var_tn2_dn9 = assign24280_e18729_d_n9;
        locals.var_tn2_dn10 = assign24280_e18729_d_n10;
        locals.var_tn2_dn11 = assign24280_e18729_d_n11;
        locals.var_tn2_dn14 = assign24280_e18729_d_n14;
        locals.var_tn2_rv = 0.0;

        let (assign24290_e18739, assign24290_e18739_d_n0, assign24290_e18739_d_n2, assign24290_e18739_d_n4, assign24290_e18739_d_n5, assign24290_e18739_d_n6, assign24290_e18739_d_n7, assign24290_e18739_d_n8, assign24290_e18739_d_n9, assign24290_e18739_d_n10, assign24290_e18739_d_n11, assign24290_e18739_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24290_e18735: f64 = (2.0 * 1.034943e-10);
        let assign24290_e18737: f64 = (assign24290_e18735 / locals.var_q_ndepm);
        (assign24290_e18737, (-((assign24290_e18735 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))),)
    } else {
        (locals.var_c_2esipq_ndepm, locals.var_c_2esipq_ndepm_dn0, locals.var_c_2esipq_ndepm_dn2, locals.var_c_2esipq_ndepm_dn4, locals.var_c_2esipq_ndepm_dn5, locals.var_c_2esipq_ndepm_dn6, locals.var_c_2esipq_ndepm_dn7, locals.var_c_2esipq_ndepm_dn8, locals.var_c_2esipq_ndepm_dn9, locals.var_c_2esipq_ndepm_dn10, locals.var_c_2esipq_ndepm_dn11, locals.var_c_2esipq_ndepm_dn14,)
    }
};
        locals.var_c_2esipq_ndepm = assign24290_e18739;
        locals.var_c_2esipq_ndepm_dn0 = assign24290_e18739_d_n0;
        locals.var_c_2esipq_ndepm_dn2 = assign24290_e18739_d_n2;
        locals.var_c_2esipq_ndepm_dn4 = assign24290_e18739_d_n4;
        locals.var_c_2esipq_ndepm_dn5 = assign24290_e18739_d_n5;
        locals.var_c_2esipq_ndepm_dn6 = assign24290_e18739_d_n6;
        locals.var_c_2esipq_ndepm_dn7 = assign24290_e18739_d_n7;
        locals.var_c_2esipq_ndepm_dn8 = assign24290_e18739_d_n8;
        locals.var_c_2esipq_ndepm_dn9 = assign24290_e18739_d_n9;
        locals.var_c_2esipq_ndepm_dn10 = assign24290_e18739_d_n10;
        locals.var_c_2esipq_ndepm_dn11 = assign24290_e18739_d_n11;
        locals.var_c_2esipq_ndepm_dn14 = assign24290_e18739_d_n14;
        locals.var_c_2esipq_ndepm_rv = 0.0;

        let (assign24300_e18749, assign24300_e18749_d_n0, assign24300_e18749_d_n2, assign24300_e18749_d_n4, assign24300_e18749_d_n5, assign24300_e18749_d_n6, assign24300_e18749_d_n7, assign24300_e18749_d_n8, assign24300_e18749_d_n9, assign24300_e18749_d_n10, assign24300_e18749_d_n11, assign24300_e18749_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24300_e18746: f64 = (2.0 * 1.034943e-10);
        let assign24300_e18747: f64 = (locals.var_q_ndepm / assign24300_e18746);
        (assign24300_e18747, (locals.var_q_ndepm_dn0 / assign24300_e18746), (locals.var_q_ndepm_dn2 / assign24300_e18746), (locals.var_q_ndepm_dn4 / assign24300_e18746), (locals.var_q_ndepm_dn5 / assign24300_e18746), (locals.var_q_ndepm_dn6 / assign24300_e18746), (locals.var_q_ndepm_dn7 / assign24300_e18746), (locals.var_q_ndepm_dn8 / assign24300_e18746), (locals.var_q_ndepm_dn9 / assign24300_e18746), (locals.var_q_ndepm_dn10 / assign24300_e18746), (locals.var_q_ndepm_dn11 / assign24300_e18746), (locals.var_q_ndepm_dn14 / assign24300_e18746),)
    } else {
        (locals.var_c_2esipq_ndepm_inv, locals.var_c_2esipq_ndepm_inv_dn0, locals.var_c_2esipq_ndepm_inv_dn2, locals.var_c_2esipq_ndepm_inv_dn4, locals.var_c_2esipq_ndepm_inv_dn5, locals.var_c_2esipq_ndepm_inv_dn6, locals.var_c_2esipq_ndepm_inv_dn7, locals.var_c_2esipq_ndepm_inv_dn8, locals.var_c_2esipq_ndepm_inv_dn9, locals.var_c_2esipq_ndepm_inv_dn10, locals.var_c_2esipq_ndepm_inv_dn11, locals.var_c_2esipq_ndepm_inv_dn14,)
    }
};
        locals.var_c_2esipq_ndepm_inv = assign24300_e18749;
        locals.var_c_2esipq_ndepm_inv_dn0 = assign24300_e18749_d_n0;
        locals.var_c_2esipq_ndepm_inv_dn2 = assign24300_e18749_d_n2;
        locals.var_c_2esipq_ndepm_inv_dn4 = assign24300_e18749_d_n4;
        locals.var_c_2esipq_ndepm_inv_dn5 = assign24300_e18749_d_n5;
        locals.var_c_2esipq_ndepm_inv_dn6 = assign24300_e18749_d_n6;
        locals.var_c_2esipq_ndepm_inv_dn7 = assign24300_e18749_d_n7;
        locals.var_c_2esipq_ndepm_inv_dn8 = assign24300_e18749_d_n8;
        locals.var_c_2esipq_ndepm_inv_dn9 = assign24300_e18749_d_n9;
        locals.var_c_2esipq_ndepm_inv_dn10 = assign24300_e18749_d_n10;
        locals.var_c_2esipq_ndepm_inv_dn11 = assign24300_e18749_d_n11;
        locals.var_c_2esipq_ndepm_inv_dn14 = assign24300_e18749_d_n14;
        locals.var_c_2esipq_ndepm_inv_rv = 0.0;

        let (assign24310_e18759, assign24310_e18759_d_n0, assign24310_e18759_d_n2, assign24310_e18759_d_n4, assign24310_e18759_d_n5, assign24310_e18759_d_n6, assign24310_e18759_d_n7, assign24310_e18759_d_n8, assign24310_e18759_d_n9, assign24310_e18759_d_n10, assign24310_e18759_d_n11, assign24310_e18759_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24310_e18755: f64 = (2.0 * 1.034943e-10);
        let assign24310_e18757: f64 = (assign24310_e18755 * locals.var_q_ndepm);
        (assign24310_e18757, (assign24310_e18755 * locals.var_q_ndepm_dn0), (assign24310_e18755 * locals.var_q_ndepm_dn2), (assign24310_e18755 * locals.var_q_ndepm_dn4), (assign24310_e18755 * locals.var_q_ndepm_dn5), (assign24310_e18755 * locals.var_q_ndepm_dn6), (assign24310_e18755 * locals.var_q_ndepm_dn7), (assign24310_e18755 * locals.var_q_ndepm_dn8), (assign24310_e18755 * locals.var_q_ndepm_dn9), (assign24310_e18755 * locals.var_q_ndepm_dn10), (assign24310_e18755 * locals.var_q_ndepm_dn11), (assign24310_e18755 * locals.var_q_ndepm_dn14),)
    } else {
        (locals.var_c_2esi_q_ndepm, locals.var_c_2esi_q_ndepm_dn0, locals.var_c_2esi_q_ndepm_dn2, locals.var_c_2esi_q_ndepm_dn4, locals.var_c_2esi_q_ndepm_dn5, locals.var_c_2esi_q_ndepm_dn6, locals.var_c_2esi_q_ndepm_dn7, locals.var_c_2esi_q_ndepm_dn8, locals.var_c_2esi_q_ndepm_dn9, locals.var_c_2esi_q_ndepm_dn10, locals.var_c_2esi_q_ndepm_dn11, locals.var_c_2esi_q_ndepm_dn14,)
    }
};
        locals.var_c_2esi_q_ndepm = assign24310_e18759;
        locals.var_c_2esi_q_ndepm_dn0 = assign24310_e18759_d_n0;
        locals.var_c_2esi_q_ndepm_dn2 = assign24310_e18759_d_n2;
        locals.var_c_2esi_q_ndepm_dn4 = assign24310_e18759_d_n4;
        locals.var_c_2esi_q_ndepm_dn5 = assign24310_e18759_d_n5;
        locals.var_c_2esi_q_ndepm_dn6 = assign24310_e18759_d_n6;
        locals.var_c_2esi_q_ndepm_dn7 = assign24310_e18759_d_n7;
        locals.var_c_2esi_q_ndepm_dn8 = assign24310_e18759_d_n8;
        locals.var_c_2esi_q_ndepm_dn9 = assign24310_e18759_d_n9;
        locals.var_c_2esi_q_ndepm_dn10 = assign24310_e18759_d_n10;
        locals.var_c_2esi_q_ndepm_dn11 = assign24310_e18759_d_n11;
        locals.var_c_2esi_q_ndepm_dn14 = assign24310_e18759_d_n14;
        locals.var_c_2esi_q_ndepm_rv = 0.0;

        let (assign24320_e18769, assign24320_e18769_d_n0, assign24320_e18769_d_n2, assign24320_e18769_d_n4, assign24320_e18769_d_n5, assign24320_e18769_d_n6, assign24320_e18769_d_n7, assign24320_e18769_d_n8, assign24320_e18769_d_n9, assign24320_e18769_d_n10, assign24320_e18769_d_n11, assign24320_e18769_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24320_e18765: f64 = (2.0 * 1.034943e-10);
        let assign24320_e18767: f64 = (assign24320_e18765 / locals.var_q_nsub__blk546);
        (assign24320_e18767, (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn0) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn2) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn4) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn5) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn6) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn7) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn8) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn9) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn10) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn11) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn14) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))),)
    } else {
        (locals.var_c_2esipq_nsub, locals.var_c_2esipq_nsub_dn0, locals.var_c_2esipq_nsub_dn2, locals.var_c_2esipq_nsub_dn4, locals.var_c_2esipq_nsub_dn5, locals.var_c_2esipq_nsub_dn6, locals.var_c_2esipq_nsub_dn7, locals.var_c_2esipq_nsub_dn8, locals.var_c_2esipq_nsub_dn9, locals.var_c_2esipq_nsub_dn10, locals.var_c_2esipq_nsub_dn11, locals.var_c_2esipq_nsub_dn14,)
    }
};
        locals.var_c_2esipq_nsub = assign24320_e18769;
        locals.var_c_2esipq_nsub_dn0 = assign24320_e18769_d_n0;
        locals.var_c_2esipq_nsub_dn2 = assign24320_e18769_d_n2;
        locals.var_c_2esipq_nsub_dn4 = assign24320_e18769_d_n4;
        locals.var_c_2esipq_nsub_dn5 = assign24320_e18769_d_n5;
        locals.var_c_2esipq_nsub_dn6 = assign24320_e18769_d_n6;
        locals.var_c_2esipq_nsub_dn7 = assign24320_e18769_d_n7;
        locals.var_c_2esipq_nsub_dn8 = assign24320_e18769_d_n8;
        locals.var_c_2esipq_nsub_dn9 = assign24320_e18769_d_n9;
        locals.var_c_2esipq_nsub_dn10 = assign24320_e18769_d_n10;
        locals.var_c_2esipq_nsub_dn11 = assign24320_e18769_d_n11;
        locals.var_c_2esipq_nsub_dn14 = assign24320_e18769_d_n14;
        locals.var_c_2esipq_nsub_rv = 0.0;

        let (assign24330_e18779, assign24330_e18779_d_n0, assign24330_e18779_d_n2, assign24330_e18779_d_n4, assign24330_e18779_d_n5, assign24330_e18779_d_n6, assign24330_e18779_d_n7, assign24330_e18779_d_n8, assign24330_e18779_d_n9, assign24330_e18779_d_n10, assign24330_e18779_d_n11, assign24330_e18779_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24330_e18776: f64 = (2.0 * 1.034943e-10);
        let assign24330_e18777: f64 = (locals.var_q_nsub__blk546 / assign24330_e18776);
        (assign24330_e18777, (locals.var_q_nsub__blk546_dn0 / assign24330_e18776), (locals.var_q_nsub__blk546_dn2 / assign24330_e18776), (locals.var_q_nsub__blk546_dn4 / assign24330_e18776), (locals.var_q_nsub__blk546_dn5 / assign24330_e18776), (locals.var_q_nsub__blk546_dn6 / assign24330_e18776), (locals.var_q_nsub__blk546_dn7 / assign24330_e18776), (locals.var_q_nsub__blk546_dn8 / assign24330_e18776), (locals.var_q_nsub__blk546_dn9 / assign24330_e18776), (locals.var_q_nsub__blk546_dn10 / assign24330_e18776), (locals.var_q_nsub__blk546_dn11 / assign24330_e18776), (locals.var_q_nsub__blk546_dn14 / assign24330_e18776),)
    } else {
        (locals.var_c_2esipq_nsub_inv, locals.var_c_2esipq_nsub_inv_dn0, locals.var_c_2esipq_nsub_inv_dn2, locals.var_c_2esipq_nsub_inv_dn4, locals.var_c_2esipq_nsub_inv_dn5, locals.var_c_2esipq_nsub_inv_dn6, locals.var_c_2esipq_nsub_inv_dn7, locals.var_c_2esipq_nsub_inv_dn8, locals.var_c_2esipq_nsub_inv_dn9, locals.var_c_2esipq_nsub_inv_dn10, locals.var_c_2esipq_nsub_inv_dn11, locals.var_c_2esipq_nsub_inv_dn14,)
    }
};
        locals.var_c_2esipq_nsub_inv = assign24330_e18779;
        locals.var_c_2esipq_nsub_inv_dn0 = assign24330_e18779_d_n0;
        locals.var_c_2esipq_nsub_inv_dn2 = assign24330_e18779_d_n2;
        locals.var_c_2esipq_nsub_inv_dn4 = assign24330_e18779_d_n4;
        locals.var_c_2esipq_nsub_inv_dn5 = assign24330_e18779_d_n5;
        locals.var_c_2esipq_nsub_inv_dn6 = assign24330_e18779_d_n6;
        locals.var_c_2esipq_nsub_inv_dn7 = assign24330_e18779_d_n7;
        locals.var_c_2esipq_nsub_inv_dn8 = assign24330_e18779_d_n8;
        locals.var_c_2esipq_nsub_inv_dn9 = assign24330_e18779_d_n9;
        locals.var_c_2esipq_nsub_inv_dn10 = assign24330_e18779_d_n10;
        locals.var_c_2esipq_nsub_inv_dn11 = assign24330_e18779_d_n11;
        locals.var_c_2esipq_nsub_inv_dn14 = assign24330_e18779_d_n14;
        locals.var_c_2esipq_nsub_inv_rv = 0.0;

        let (assign24340_e18787, assign24340_e18787_d_n0, assign24340_e18787_d_n2, assign24340_e18787_d_n4, assign24340_e18787_d_n5, assign24340_e18787_d_n6, assign24340_e18787_d_n7, assign24340_e18787_d_n8, assign24340_e18787_d_n9, assign24340_e18787_d_n10, assign24340_e18787_d_n11, assign24340_e18787_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24340_e18785: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign24340_e18785, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub, locals.var_ndepmpnsub_dn0, locals.var_ndepmpnsub_dn2, locals.var_ndepmpnsub_dn4, locals.var_ndepmpnsub_dn5, locals.var_ndepmpnsub_dn6, locals.var_ndepmpnsub_dn7, locals.var_ndepmpnsub_dn8, locals.var_ndepmpnsub_dn9, locals.var_ndepmpnsub_dn10, locals.var_ndepmpnsub_dn11, locals.var_ndepmpnsub_dn14,)
    }
};
        locals.var_ndepmpnsub = assign24340_e18787;
        locals.var_ndepmpnsub_dn0 = assign24340_e18787_d_n0;
        locals.var_ndepmpnsub_dn2 = assign24340_e18787_d_n2;
        locals.var_ndepmpnsub_dn4 = assign24340_e18787_d_n4;
        locals.var_ndepmpnsub_dn5 = assign24340_e18787_d_n5;
        locals.var_ndepmpnsub_dn6 = assign24340_e18787_d_n6;
        locals.var_ndepmpnsub_dn7 = assign24340_e18787_d_n7;
        locals.var_ndepmpnsub_dn8 = assign24340_e18787_d_n8;
        locals.var_ndepmpnsub_dn9 = assign24340_e18787_d_n9;
        locals.var_ndepmpnsub_dn10 = assign24340_e18787_d_n10;
        locals.var_ndepmpnsub_dn11 = assign24340_e18787_d_n11;
        locals.var_ndepmpnsub_dn14 = assign24340_e18787_d_n14;
        locals.var_ndepmpnsub_rv = 0.0;

        let (assign24350_e18797, assign24350_e18797_d_n0, assign24350_e18797_d_n2, assign24350_e18797_d_n4, assign24350_e18797_d_n5, assign24350_e18797_d_n6, assign24350_e18797_d_n7, assign24350_e18797_d_n8, assign24350_e18797_d_n9, assign24350_e18797_d_n10, assign24350_e18797_d_n11, assign24350_e18797_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24350_e18794: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign24350_e18795: f64 = (1.0 / assign24350_e18794);
        (assign24350_e18795, (-(locals.var_ndepmpnsub_dn0 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn2 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn4 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn5 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn6 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn7 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn8 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn9 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn10 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn11 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn14 / (assign24350_e18794 * assign24350_e18794))),)
    } else {
        (locals.var_ndepmpnsub_inv1, locals.var_ndepmpnsub_inv1_dn0, locals.var_ndepmpnsub_inv1_dn2, locals.var_ndepmpnsub_inv1_dn4, locals.var_ndepmpnsub_inv1_dn5, locals.var_ndepmpnsub_inv1_dn6, locals.var_ndepmpnsub_inv1_dn7, locals.var_ndepmpnsub_inv1_dn8, locals.var_ndepmpnsub_inv1_dn9, locals.var_ndepmpnsub_inv1_dn10, locals.var_ndepmpnsub_inv1_dn11, locals.var_ndepmpnsub_inv1_dn14,)
    }
};
        locals.var_ndepmpnsub_inv1 = assign24350_e18797;
        locals.var_ndepmpnsub_inv1_dn0 = assign24350_e18797_d_n0;
        locals.var_ndepmpnsub_inv1_dn2 = assign24350_e18797_d_n2;
        locals.var_ndepmpnsub_inv1_dn4 = assign24350_e18797_d_n4;
        locals.var_ndepmpnsub_inv1_dn5 = assign24350_e18797_d_n5;
        locals.var_ndepmpnsub_inv1_dn6 = assign24350_e18797_d_n6;
        locals.var_ndepmpnsub_inv1_dn7 = assign24350_e18797_d_n7;
        locals.var_ndepmpnsub_inv1_dn8 = assign24350_e18797_d_n8;
        locals.var_ndepmpnsub_inv1_dn9 = assign24350_e18797_d_n9;
        locals.var_ndepmpnsub_inv1_dn10 = assign24350_e18797_d_n10;
        locals.var_ndepmpnsub_inv1_dn11 = assign24350_e18797_d_n11;
        locals.var_ndepmpnsub_inv1_dn14 = assign24350_e18797_d_n14;
        locals.var_ndepmpnsub_inv1_rv = 0.0;

        let (assign24360_e18805,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24360_e18803: f64 = (1e-12 * 1000.0);
        (assign24360_e18803,)
    } else {
        (locals.var_ps_conv3,)
    }
};
        locals.var_ps_conv3 = assign24360_e18805;
        locals.var_ps_conv3_rv = 0.0;

        let (assign24370_e18813,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24370_e18811: f64 = (1e-10 * 1000.0);
        (assign24370_e18811,)
    } else {
        (locals.var_ps_conv23,)
    }
};
        locals.var_ps_conv23 = assign24370_e18813;
        locals.var_ps_conv23_rv = 0.0;

        let (assign24380_e18819, assign24380_e18819_d_n0, assign24380_e18819_d_n2, assign24380_e18819_d_n4, assign24380_e18819_d_n5, assign24380_e18819_d_n6, assign24380_e18819_d_n7, assign24380_e18819_d_n8, assign24380_e18819_d_n9, assign24380_e18819_d_n10, assign24380_e18819_d_n11, assign24380_e18819_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    }
};
        locals.var_phi_s0_dep = assign24380_e18819;
        locals.var_phi_s0_dep_dn0 = assign24380_e18819_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24380_e18819_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24380_e18819_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24380_e18819_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24380_e18819_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24380_e18819_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24380_e18819_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24380_e18819_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24380_e18819_d_n10;
        locals.var_phi_s0_dep_dn11 = assign24380_e18819_d_n11;
        locals.var_phi_s0_dep_dn14 = assign24380_e18819_d_n14;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24390_e18825, assign24390_e18825_d_n0, assign24390_e18825_d_n2, assign24390_e18825_d_n4, assign24390_e18825_d_n5, assign24390_e18825_d_n6, assign24390_e18825_d_n7, assign24390_e18825_d_n8, assign24390_e18825_d_n9, assign24390_e18825_d_n10, assign24390_e18825_d_n11, assign24390_e18825_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign24390_e18825;
        locals.var_phi_sl_dep_dn0 = assign24390_e18825_d_n0;
        locals.var_phi_sl_dep_dn2 = assign24390_e18825_d_n2;
        locals.var_phi_sl_dep_dn4 = assign24390_e18825_d_n4;
        locals.var_phi_sl_dep_dn5 = assign24390_e18825_d_n5;
        locals.var_phi_sl_dep_dn6 = assign24390_e18825_d_n6;
        locals.var_phi_sl_dep_dn7 = assign24390_e18825_d_n7;
        locals.var_phi_sl_dep_dn8 = assign24390_e18825_d_n8;
        locals.var_phi_sl_dep_dn9 = assign24390_e18825_d_n9;
        locals.var_phi_sl_dep_dn10 = assign24390_e18825_d_n10;
        locals.var_phi_sl_dep_dn11 = assign24390_e18825_d_n11;
        locals.var_phi_sl_dep_dn14 = assign24390_e18825_d_n14;
        locals.var_phi_sl_dep_rv = 0.0;

        let (assign24400_e18831, assign24400_e18831_d_n0, assign24400_e18831_d_n2, assign24400_e18831_d_n4, assign24400_e18831_d_n5, assign24400_e18831_d_n6, assign24400_e18831_d_n7, assign24400_e18831_d_n8, assign24400_e18831_d_n9, assign24400_e18831_d_n10, assign24400_e18831_d_n11, assign24400_e18831_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign24400_e18831;
        locals.var_q_s0_dn0 = assign24400_e18831_d_n0;
        locals.var_q_s0_dn2 = assign24400_e18831_d_n2;
        locals.var_q_s0_dn4 = assign24400_e18831_d_n4;
        locals.var_q_s0_dn5 = assign24400_e18831_d_n5;
        locals.var_q_s0_dn6 = assign24400_e18831_d_n6;
        locals.var_q_s0_dn7 = assign24400_e18831_d_n7;
        locals.var_q_s0_dn8 = assign24400_e18831_d_n8;
        locals.var_q_s0_dn9 = assign24400_e18831_d_n9;
        locals.var_q_s0_dn10 = assign24400_e18831_d_n10;
        locals.var_q_s0_dn11 = assign24400_e18831_d_n11;
        locals.var_q_s0_dn14 = assign24400_e18831_d_n14;
        locals.var_q_s0_rv = 0.0;

        let (assign24410_e18837, assign24410_e18837_d_n0, assign24410_e18837_d_n2, assign24410_e18837_d_n4, assign24410_e18837_d_n5, assign24410_e18837_d_n6, assign24410_e18837_d_n7, assign24410_e18837_d_n8, assign24410_e18837_d_n9, assign24410_e18837_d_n10, assign24410_e18837_d_n11, assign24410_e18837_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign24410_e18837;
        locals.var_q_sl_dn0 = assign24410_e18837_d_n0;
        locals.var_q_sl_dn2 = assign24410_e18837_d_n2;
        locals.var_q_sl_dn4 = assign24410_e18837_d_n4;
        locals.var_q_sl_dn5 = assign24410_e18837_d_n5;
        locals.var_q_sl_dn6 = assign24410_e18837_d_n6;
        locals.var_q_sl_dn7 = assign24410_e18837_d_n7;
        locals.var_q_sl_dn8 = assign24410_e18837_d_n8;
        locals.var_q_sl_dn9 = assign24410_e18837_d_n9;
        locals.var_q_sl_dn10 = assign24410_e18837_d_n10;
        locals.var_q_sl_dn11 = assign24410_e18837_d_n11;
        locals.var_q_sl_dn14 = assign24410_e18837_d_n14;
        locals.var_q_sl_rv = 0.0;

        let (assign24420_e18843, assign24420_e18843_d_n0, assign24420_e18843_d_n2, assign24420_e18843_d_n4, assign24420_e18843_d_n5, assign24420_e18843_d_n6, assign24420_e18843_d_n7, assign24420_e18843_d_n8, assign24420_e18843_d_n9, assign24420_e18843_d_n10, assign24420_e18843_d_n11, assign24420_e18843_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign24420_e18843;
        locals.var_q_s0_dep_dn0 = assign24420_e18843_d_n0;
        locals.var_q_s0_dep_dn2 = assign24420_e18843_d_n2;
        locals.var_q_s0_dep_dn4 = assign24420_e18843_d_n4;
        locals.var_q_s0_dep_dn5 = assign24420_e18843_d_n5;
        locals.var_q_s0_dep_dn6 = assign24420_e18843_d_n6;
        locals.var_q_s0_dep_dn7 = assign24420_e18843_d_n7;
        locals.var_q_s0_dep_dn8 = assign24420_e18843_d_n8;
        locals.var_q_s0_dep_dn9 = assign24420_e18843_d_n9;
        locals.var_q_s0_dep_dn10 = assign24420_e18843_d_n10;
        locals.var_q_s0_dep_dn11 = assign24420_e18843_d_n11;
        locals.var_q_s0_dep_dn14 = assign24420_e18843_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign24430_e18849, assign24430_e18849_d_n0, assign24430_e18849_d_n2, assign24430_e18849_d_n4, assign24430_e18849_d_n5, assign24430_e18849_d_n6, assign24430_e18849_d_n7, assign24430_e18849_d_n8, assign24430_e18849_d_n9, assign24430_e18849_d_n10, assign24430_e18849_d_n11, assign24430_e18849_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign24430_e18849;
        locals.var_q_sl_dep_dn0 = assign24430_e18849_d_n0;
        locals.var_q_sl_dep_dn2 = assign24430_e18849_d_n2;
        locals.var_q_sl_dep_dn4 = assign24430_e18849_d_n4;
        locals.var_q_sl_dep_dn5 = assign24430_e18849_d_n5;
        locals.var_q_sl_dep_dn6 = assign24430_e18849_d_n6;
        locals.var_q_sl_dep_dn7 = assign24430_e18849_d_n7;
        locals.var_q_sl_dep_dn8 = assign24430_e18849_d_n8;
        locals.var_q_sl_dep_dn9 = assign24430_e18849_d_n9;
        locals.var_q_sl_dep_dn10 = assign24430_e18849_d_n10;
        locals.var_q_sl_dep_dn11 = assign24430_e18849_d_n11;
        locals.var_q_sl_dep_dn14 = assign24430_e18849_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign24440_e18855, assign24440_e18855_d_n0, assign24440_e18855_d_n2, assign24440_e18855_d_n4, assign24440_e18855_d_n5, assign24440_e18855_d_n6, assign24440_e18855_d_n7, assign24440_e18855_d_n8, assign24440_e18855_d_n9, assign24440_e18855_d_n10, assign24440_e18855_d_n11, assign24440_e18855_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign24440_e18855;
        locals.var_q_b0_dep_dn0 = assign24440_e18855_d_n0;
        locals.var_q_b0_dep_dn2 = assign24440_e18855_d_n2;
        locals.var_q_b0_dep_dn4 = assign24440_e18855_d_n4;
        locals.var_q_b0_dep_dn5 = assign24440_e18855_d_n5;
        locals.var_q_b0_dep_dn6 = assign24440_e18855_d_n6;
        locals.var_q_b0_dep_dn7 = assign24440_e18855_d_n7;
        locals.var_q_b0_dep_dn8 = assign24440_e18855_d_n8;
        locals.var_q_b0_dep_dn9 = assign24440_e18855_d_n9;
        locals.var_q_b0_dep_dn10 = assign24440_e18855_d_n10;
        locals.var_q_b0_dep_dn11 = assign24440_e18855_d_n11;
        locals.var_q_b0_dep_dn14 = assign24440_e18855_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign24450_e18861, assign24450_e18861_d_n0, assign24450_e18861_d_n2, assign24450_e18861_d_n4, assign24450_e18861_d_n5, assign24450_e18861_d_n6, assign24450_e18861_d_n7, assign24450_e18861_d_n8, assign24450_e18861_d_n9, assign24450_e18861_d_n10, assign24450_e18861_d_n11, assign24450_e18861_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign24450_e18861;
        locals.var_q_bl_dep_dn0 = assign24450_e18861_d_n0;
        locals.var_q_bl_dep_dn2 = assign24450_e18861_d_n2;
        locals.var_q_bl_dep_dn4 = assign24450_e18861_d_n4;
        locals.var_q_bl_dep_dn5 = assign24450_e18861_d_n5;
        locals.var_q_bl_dep_dn6 = assign24450_e18861_d_n6;
        locals.var_q_bl_dep_dn7 = assign24450_e18861_d_n7;
        locals.var_q_bl_dep_dn8 = assign24450_e18861_d_n8;
        locals.var_q_bl_dep_dn9 = assign24450_e18861_d_n9;
        locals.var_q_bl_dep_dn10 = assign24450_e18861_d_n10;
        locals.var_q_bl_dep_dn11 = assign24450_e18861_d_n11;
        locals.var_q_bl_dep_dn14 = assign24450_e18861_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign24460_e18867, assign24460_e18867_d_n0, assign24460_e18867_d_n2, assign24460_e18867_d_n4, assign24460_e18867_d_n5, assign24460_e18867_d_n6, assign24460_e18867_d_n7, assign24460_e18867_d_n8, assign24460_e18867_d_n9, assign24460_e18867_d_n10, assign24460_e18867_d_n11, assign24460_e18867_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign24460_e18867;
        locals.var_q_sub0_dep_dn0 = assign24460_e18867_d_n0;
        locals.var_q_sub0_dep_dn2 = assign24460_e18867_d_n2;
        locals.var_q_sub0_dep_dn4 = assign24460_e18867_d_n4;
        locals.var_q_sub0_dep_dn5 = assign24460_e18867_d_n5;
        locals.var_q_sub0_dep_dn6 = assign24460_e18867_d_n6;
        locals.var_q_sub0_dep_dn7 = assign24460_e18867_d_n7;
        locals.var_q_sub0_dep_dn8 = assign24460_e18867_d_n8;
        locals.var_q_sub0_dep_dn9 = assign24460_e18867_d_n9;
        locals.var_q_sub0_dep_dn10 = assign24460_e18867_d_n10;
        locals.var_q_sub0_dep_dn11 = assign24460_e18867_d_n11;
        locals.var_q_sub0_dep_dn14 = assign24460_e18867_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign24470_e18873, assign24470_e18873_d_n0, assign24470_e18873_d_n2, assign24470_e18873_d_n4, assign24470_e18873_d_n5, assign24470_e18873_d_n6, assign24470_e18873_d_n7, assign24470_e18873_d_n8, assign24470_e18873_d_n9, assign24470_e18873_d_n10, assign24470_e18873_d_n11, assign24470_e18873_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign24470_e18873;
        locals.var_q_subl_dep_dn0 = assign24470_e18873_d_n0;
        locals.var_q_subl_dep_dn2 = assign24470_e18873_d_n2;
        locals.var_q_subl_dep_dn4 = assign24470_e18873_d_n4;
        locals.var_q_subl_dep_dn5 = assign24470_e18873_d_n5;
        locals.var_q_subl_dep_dn6 = assign24470_e18873_d_n6;
        locals.var_q_subl_dep_dn7 = assign24470_e18873_d_n7;
        locals.var_q_subl_dep_dn8 = assign24470_e18873_d_n8;
        locals.var_q_subl_dep_dn9 = assign24470_e18873_d_n9;
        locals.var_q_subl_dep_dn10 = assign24470_e18873_d_n10;
        locals.var_q_subl_dep_dn11 = assign24470_e18873_d_n11;
        locals.var_q_subl_dep_dn14 = assign24470_e18873_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let (assign24480_e18879, assign24480_e18879_d_n0, assign24480_e18879_d_n2, assign24480_e18879_d_n4, assign24480_e18879_d_n5, assign24480_e18879_d_n6, assign24480_e18879_d_n7, assign24480_e18879_d_n8, assign24480_e18879_d_n9, assign24480_e18879_d_n10, assign24480_e18879_d_n11, assign24480_e18879_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn11, locals.var_phib_ref_dn14,)
    }
};
        locals.var_phib_ref = assign24480_e18879;
        locals.var_phib_ref_dn0 = assign24480_e18879_d_n0;
        locals.var_phib_ref_dn2 = assign24480_e18879_d_n2;
        locals.var_phib_ref_dn4 = assign24480_e18879_d_n4;
        locals.var_phib_ref_dn5 = assign24480_e18879_d_n5;
        locals.var_phib_ref_dn6 = assign24480_e18879_d_n6;
        locals.var_phib_ref_dn7 = assign24480_e18879_d_n7;
        locals.var_phib_ref_dn8 = assign24480_e18879_d_n8;
        locals.var_phib_ref_dn9 = assign24480_e18879_d_n9;
        locals.var_phib_ref_dn10 = assign24480_e18879_d_n10;
        locals.var_phib_ref_dn11 = assign24480_e18879_d_n11;
        locals.var_phib_ref_dn14 = assign24480_e18879_d_n14;
        locals.var_phib_ref_rv = 0.0;

        let (assign24490_e18891, assign24490_e18891_d_n0, assign24490_e18891_d_n2, assign24490_e18891_d_n4, assign24490_e18891_d_n5, assign24490_e18891_d_n6, assign24490_e18891_d_n7, assign24490_e18891_d_n8, assign24490_e18891_d_n9, assign24490_e18891_d_n10, assign24490_e18891_d_n11, assign24490_e18891_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24490_e18886: f64 = (10.0 * 2.220446049250313e-16);
        let assign24490_e18888: f64 = (assign24490_e18886 * 10000000.0);
        let assign24490_e18889: f64 = (locals.var_vgp + assign24490_e18888);
        (assign24490_e18889, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    }
};
        locals.var_vgp = assign24490_e18891;
        locals.var_vgp_dn0 = assign24490_e18891_d_n0;
        locals.var_vgp_dn2 = assign24490_e18891_d_n2;
        locals.var_vgp_dn4 = assign24490_e18891_d_n4;
        locals.var_vgp_dn5 = assign24490_e18891_d_n5;
        locals.var_vgp_dn6 = assign24490_e18891_d_n6;
        locals.var_vgp_dn7 = assign24490_e18891_d_n7;
        locals.var_vgp_dn8 = assign24490_e18891_d_n8;
        locals.var_vgp_dn9 = assign24490_e18891_d_n9;
        locals.var_vgp_dn10 = assign24490_e18891_d_n10;
        locals.var_vgp_dn11 = assign24490_e18891_d_n11;
        locals.var_vgp_dn14 = assign24490_e18891_d_n14;
        locals.var_vgp_rv = 0.0;

        let (assign24500_e18903, assign24500_e18903_d_n0, assign24500_e18903_d_n2, assign24500_e18903_d_n4, assign24500_e18903_d_n5, assign24500_e18903_d_n6, assign24500_e18903_d_n7, assign24500_e18903_d_n8, assign24500_e18903_d_n9, assign24500_e18903_d_n10, assign24500_e18903_d_n11, assign24500_e18903_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24500_e18897: f64 = (locals.var_cox * locals.var_cox);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cnst0;
        let assign24500_e18899: f64 = (assign24500_e18897 * __rspice_inv_cse_0);
        let assign24500_e18901: f64 = (assign24500_e18899 * __rspice_inv_cse_0);
        (assign24500_e18901, ((((((((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn11)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn11)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)) * locals.var_cnst0) - (assign24500_e18897 * locals.var_cnst0_dn14)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24500_e18899 * locals.var_cnst0_dn14)) / (locals.var_cnst0 * locals.var_cnst0)),)
    } else {
        (locals.var_afact, locals.var_afact_dn0, locals.var_afact_dn2, locals.var_afact_dn4, locals.var_afact_dn5, locals.var_afact_dn6, locals.var_afact_dn7, locals.var_afact_dn8, locals.var_afact_dn9, locals.var_afact_dn10, locals.var_afact_dn11, locals.var_afact_dn14,)
    }
};
        locals.var_afact = assign24500_e18903;
        locals.var_afact_dn0 = assign24500_e18903_d_n0;
        locals.var_afact_dn2 = assign24500_e18903_d_n2;
        locals.var_afact_dn4 = assign24500_e18903_d_n4;
        locals.var_afact_dn5 = assign24500_e18903_d_n5;
        locals.var_afact_dn6 = assign24500_e18903_d_n6;
        locals.var_afact_dn7 = assign24500_e18903_d_n7;
        locals.var_afact_dn8 = assign24500_e18903_d_n8;
        locals.var_afact_dn9 = assign24500_e18903_d_n9;
        locals.var_afact_dn10 = assign24500_e18903_d_n10;
        locals.var_afact_dn11 = assign24500_e18903_d_n11;
        locals.var_afact_dn14 = assign24500_e18903_d_n14;
        locals.var_afact_rv = 0.0;

        let (assign24510_e18915, assign24510_e18915_d_n0, assign24510_e18915_d_n2, assign24510_e18915_d_n4, assign24510_e18915_d_n5, assign24510_e18915_d_n6, assign24510_e18915_d_n7, assign24510_e18915_d_n8, assign24510_e18915_d_n9, assign24510_e18915_d_n10, assign24510_e18915_d_n11, assign24510_e18915_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign24510_e18909: f64 = (locals.var_afact * __rspice_inv_cse_1);
        let assign24510_e18911: f64 = (assign24510_e18909 * __rspice_inv_cse_1);
        let assign24510_e18913: f64 = (assign24510_e18911 * locals.var_ndepm2);
        (assign24510_e18913, ((((((((locals.var_afact_dn0 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn0)), ((((((((locals.var_afact_dn2 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn2)), ((((((((locals.var_afact_dn4 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn4)), ((((((((locals.var_afact_dn5 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn5)), ((((((((locals.var_afact_dn6 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn6)), ((((((((locals.var_afact_dn7 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn7)), ((((((((locals.var_afact_dn8 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn8)), ((((((((locals.var_afact_dn9 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn9)), ((((((((locals.var_afact_dn10 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn10)), ((((((((locals.var_afact_dn11 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn11)), ((((((((locals.var_afact_dn14 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24510_e18909 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24510_e18911 * locals.var_ndepm2_dn14)),)
    } else {
        (locals.var_afact2, locals.var_afact2_dn0, locals.var_afact2_dn2, locals.var_afact2_dn4, locals.var_afact2_dn5, locals.var_afact2_dn6, locals.var_afact2_dn7, locals.var_afact2_dn8, locals.var_afact2_dn9, locals.var_afact2_dn10, locals.var_afact2_dn11, locals.var_afact2_dn14,)
    }
};
        locals.var_afact2 = assign24510_e18915;
        locals.var_afact2_dn0 = assign24510_e18915_d_n0;
        locals.var_afact2_dn2 = assign24510_e18915_d_n2;
        locals.var_afact2_dn4 = assign24510_e18915_d_n4;
        locals.var_afact2_dn5 = assign24510_e18915_d_n5;
        locals.var_afact2_dn6 = assign24510_e18915_d_n6;
        locals.var_afact2_dn7 = assign24510_e18915_d_n7;
        locals.var_afact2_dn8 = assign24510_e18915_d_n8;
        locals.var_afact2_dn9 = assign24510_e18915_d_n9;
        locals.var_afact2_dn10 = assign24510_e18915_d_n10;
        locals.var_afact2_dn11 = assign24510_e18915_d_n11;
        locals.var_afact2_dn14 = assign24510_e18915_d_n14;
        locals.var_afact2_rv = 0.0;

        let (assign24520_e18933, assign24520_e18933_d_n0, assign24520_e18933_d_n2, assign24520_e18933_d_n4, assign24520_e18933_d_n5, assign24520_e18933_d_n6, assign24520_e18933_d_n7, assign24520_e18933_d_n8, assign24520_e18933_d_n9, assign24520_e18933_d_n10, assign24520_e18933_d_n11, assign24520_e18933_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24520_e18921: f64 = (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc);
        let assign24520_e18924: f64 = (locals.var_ef_nsubc + locals.var_uc_ndepm);
        let assign24520_e18925: f64 = (assign24520_e18921 / assign24520_e18924);
        let assign24520_e18927: f64 = (-locals.var_vbscl__blk437);
        let assign24520_e18929: f64 = (assign24520_e18927 + locals.var_vbi_dep);
        let assign24520_e18930: f64 = (assign24520_e18925 * assign24520_e18929);
        let assign24520_e18931: f64 = (assign24520_e18930).sqrt();
        (assign24520_e18931, ((((((((locals.var_c_2esipq_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn0)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn0 + locals.var_uc_ndepm_dn0))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn2)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn2 + locals.var_uc_ndepm_dn2))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn4)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn4 + locals.var_uc_ndepm_dn4))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn5)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn5 + locals.var_uc_ndepm_dn5))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn6)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn6 + locals.var_uc_ndepm_dn6))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn7)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn7 + locals.var_uc_ndepm_dn7))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn8)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn8 + locals.var_uc_ndepm_dn8))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn9)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn9 + locals.var_uc_ndepm_dn9))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn10)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn10 + locals.var_uc_ndepm_dn10))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn11)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn11 + locals.var_uc_ndepm_dn11))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign24520_e18931)), ((((((((locals.var_c_2esipq_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn14)) * assign24520_e18924) - (assign24520_e18921 * (locals.var_ef_nsubc_dn14 + locals.var_uc_ndepm_dn14))) / (assign24520_e18924 * assign24520_e18924)) * assign24520_e18929) + (assign24520_e18925 * ((-locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign24520_e18931)),)
    } else {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn11, locals.var_w_bsub0_dn14,)
    }
};
        locals.var_w_bsub0 = assign24520_e18933;
        locals.var_w_bsub0_dn0 = assign24520_e18933_d_n0;
        locals.var_w_bsub0_dn2 = assign24520_e18933_d_n2;
        locals.var_w_bsub0_dn4 = assign24520_e18933_d_n4;
        locals.var_w_bsub0_dn5 = assign24520_e18933_d_n5;
        locals.var_w_bsub0_dn6 = assign24520_e18933_d_n6;
        locals.var_w_bsub0_dn7 = assign24520_e18933_d_n7;
        locals.var_w_bsub0_dn8 = assign24520_e18933_d_n8;
        locals.var_w_bsub0_dn9 = assign24520_e18933_d_n9;
        locals.var_w_bsub0_dn10 = assign24520_e18933_d_n10;
        locals.var_w_bsub0_dn11 = assign24520_e18933_d_n11;
        locals.var_w_bsub0_dn14 = assign24520_e18933_d_n14;
        locals.var_w_bsub0_rv = 0.0;

        let assign24530_e18936: f64 = if locals.var_w_bsub0 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard561 = assign24530_e18936;
        locals.var_guard561_rv = 0.0;

        let (assign24540_e18944, assign24540_e18944_d_n0, assign24540_e18944_d_n2, assign24540_e18944_d_n4, assign24540_e18944_d_n5, assign24540_e18944_d_n6, assign24540_e18944_d_n7, assign24540_e18944_d_n8, assign24540_e18944_d_n9, assign24540_e18944_d_n10, assign24540_e18944_d_n11, assign24540_e18944_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign24540_e18944;
        locals.var_vgp0_dn0 = assign24540_e18944_d_n0;
        locals.var_vgp0_dn2 = assign24540_e18944_d_n2;
        locals.var_vgp0_dn4 = assign24540_e18944_d_n4;
        locals.var_vgp0_dn5 = assign24540_e18944_d_n5;
        locals.var_vgp0_dn6 = assign24540_e18944_d_n6;
        locals.var_vgp0_dn7 = assign24540_e18944_d_n7;
        locals.var_vgp0_dn8 = assign24540_e18944_d_n8;
        locals.var_vgp0_dn9 = assign24540_e18944_d_n9;
        locals.var_vgp0_dn10 = assign24540_e18944_d_n10;
        locals.var_vgp0_dn11 = assign24540_e18944_d_n11;
        locals.var_vgp0_dn14 = assign24540_e18944_d_n14;
        locals.var_vgp0_rv = 0.0;

        let (assign24550_e18952, assign24550_e18952_d_n0, assign24550_e18952_d_n2, assign24550_e18952_d_n4, assign24550_e18952_d_n5, assign24550_e18952_d_n6, assign24550_e18952_d_n7, assign24550_e18952_d_n8, assign24550_e18952_d_n9, assign24550_e18952_d_n10, assign24550_e18952_d_n11, assign24550_e18952_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign24550_e18952;
        locals.var_w_b0_dn0 = assign24550_e18952_d_n0;
        locals.var_w_b0_dn2 = assign24550_e18952_d_n2;
        locals.var_w_b0_dn4 = assign24550_e18952_d_n4;
        locals.var_w_b0_dn5 = assign24550_e18952_d_n5;
        locals.var_w_b0_dn6 = assign24550_e18952_d_n6;
        locals.var_w_b0_dn7 = assign24550_e18952_d_n7;
        locals.var_w_b0_dn8 = assign24550_e18952_d_n8;
        locals.var_w_b0_dn9 = assign24550_e18952_d_n9;
        locals.var_w_b0_dn10 = assign24550_e18952_d_n10;
        locals.var_w_b0_dn11 = assign24550_e18952_d_n11;
        locals.var_w_b0_dn14 = assign24550_e18952_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign24560_e18960, assign24560_e18960_d_n0, assign24560_e18960_d_n2, assign24560_e18960_d_n4, assign24560_e18960_d_n5, assign24560_e18960_d_n6, assign24560_e18960_d_n7, assign24560_e18960_d_n8, assign24560_e18960_d_n9, assign24560_e18960_d_n10, assign24560_e18960_d_n11, assign24560_e18960_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24560_e18960;
        locals.var_phi_b0_dep_dn0 = assign24560_e18960_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24560_e18960_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24560_e18960_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24560_e18960_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24560_e18960_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24560_e18960_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24560_e18960_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24560_e18960_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24560_e18960_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24560_e18960_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24560_e18960_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24570_e18974, assign24570_e18974_d_n0, assign24570_e18974_d_n2, assign24570_e18974_d_n4, assign24570_e18974_d_n5, assign24570_e18974_d_n6, assign24570_e18974_d_n7, assign24570_e18974_d_n8, assign24570_e18974_d_n9, assign24570_e18974_d_n10, assign24570_e18974_d_n11, assign24570_e18974_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24570_e18969: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24570_e18971: f64 = (assign24570_e18969 * locals.var_w_b0);
        let assign24570_e18972: f64 = (locals.var_phi_b0_dep - assign24570_e18971);
        (assign24570_e18972, (locals.var_phi_b0_dep_dn0 - ((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn0))), (locals.var_phi_b0_dep_dn2 - ((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn2))), (locals.var_phi_b0_dep_dn4 - ((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn4))), (locals.var_phi_b0_dep_dn5 - ((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn5))), (locals.var_phi_b0_dep_dn6 - ((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn6))), (locals.var_phi_b0_dep_dn7 - ((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn7))), (locals.var_phi_b0_dep_dn8 - ((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn8))), (locals.var_phi_b0_dep_dn9 - ((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn9))), (locals.var_phi_b0_dep_dn10 - ((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn10))), (locals.var_phi_b0_dep_dn11 - ((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn11)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn11))), (locals.var_phi_b0_dep_dn14 - ((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn14)) * locals.var_w_b0) + (assign24570_e18969 * locals.var_w_b0_dn14))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24570_e18974;
        locals.var_phi_j0_dep_dn0 = assign24570_e18974_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24570_e18974_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24570_e18974_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24570_e18974_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24570_e18974_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24570_e18974_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24570_e18974_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24570_e18974_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24570_e18974_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24570_e18974_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24570_e18974_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24580_e18982, assign24580_e18982_d_n0, assign24580_e18982_d_n2, assign24580_e18982_d_n4, assign24580_e18982_d_n5, assign24580_e18982_d_n6, assign24580_e18982_d_n7, assign24580_e18982_d_n8, assign24580_e18982_d_n9, assign24580_e18982_d_n10, assign24580_e18982_d_n11, assign24580_e18982_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    }
};
        locals.var_vds_maxb0 = assign24580_e18982;
        locals.var_vds_maxb0_dn0 = assign24580_e18982_d_n0;
        locals.var_vds_maxb0_dn2 = assign24580_e18982_d_n2;
        locals.var_vds_maxb0_dn4 = assign24580_e18982_d_n4;
        locals.var_vds_maxb0_dn5 = assign24580_e18982_d_n5;
        locals.var_vds_maxb0_dn6 = assign24580_e18982_d_n6;
        locals.var_vds_maxb0_dn7 = assign24580_e18982_d_n7;
        locals.var_vds_maxb0_dn8 = assign24580_e18982_d_n8;
        locals.var_vds_maxb0_dn9 = assign24580_e18982_d_n9;
        locals.var_vds_maxb0_dn10 = assign24580_e18982_d_n10;
        locals.var_vds_maxb0_dn11 = assign24580_e18982_d_n11;
        locals.var_vds_maxb0_dn14 = assign24580_e18982_d_n14;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24590_e18990, assign24590_e18990_d_n0, assign24590_e18990_d_n2, assign24590_e18990_d_n4, assign24590_e18990_d_n5, assign24590_e18990_d_n6, assign24590_e18990_d_n7, assign24590_e18990_d_n8, assign24590_e18990_d_n9, assign24590_e18990_d_n10, assign24590_e18990_d_n11, assign24590_e18990_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn11, locals.var_vgp0old_dn14,)
    }
};
        locals.var_vgp0old = assign24590_e18990;
        locals.var_vgp0old_dn0 = assign24590_e18990_d_n0;
        locals.var_vgp0old_dn2 = assign24590_e18990_d_n2;
        locals.var_vgp0old_dn4 = assign24590_e18990_d_n4;
        locals.var_vgp0old_dn5 = assign24590_e18990_d_n5;
        locals.var_vgp0old_dn6 = assign24590_e18990_d_n6;
        locals.var_vgp0old_dn7 = assign24590_e18990_d_n7;
        locals.var_vgp0old_dn8 = assign24590_e18990_d_n8;
        locals.var_vgp0old_dn9 = assign24590_e18990_d_n9;
        locals.var_vgp0old_dn10 = assign24590_e18990_d_n10;
        locals.var_vgp0old_dn11 = assign24590_e18990_d_n11;
        locals.var_vgp0old_dn14 = assign24590_e18990_d_n14;
        locals.var_vgp0old_rv = 0.0;

        let (assign24600_e18998, assign24600_e18998_d_n0, assign24600_e18998_d_n2, assign24600_e18998_d_n4, assign24600_e18998_d_n5, assign24600_e18998_d_n6, assign24600_e18998_d_n7, assign24600_e18998_d_n8, assign24600_e18998_d_n9, assign24600_e18998_d_n10, assign24600_e18998_d_n11, assign24600_e18998_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn11, locals.var_phi_j0_dep_old_dn14,)
    }
};
        locals.var_phi_j0_dep_old = assign24600_e18998;
        locals.var_phi_j0_dep_old_dn0 = assign24600_e18998_d_n0;
        locals.var_phi_j0_dep_old_dn2 = assign24600_e18998_d_n2;
        locals.var_phi_j0_dep_old_dn4 = assign24600_e18998_d_n4;
        locals.var_phi_j0_dep_old_dn5 = assign24600_e18998_d_n5;
        locals.var_phi_j0_dep_old_dn6 = assign24600_e18998_d_n6;
        locals.var_phi_j0_dep_old_dn7 = assign24600_e18998_d_n7;
        locals.var_phi_j0_dep_old_dn8 = assign24600_e18998_d_n8;
        locals.var_phi_j0_dep_old_dn9 = assign24600_e18998_d_n9;
        locals.var_phi_j0_dep_old_dn10 = assign24600_e18998_d_n10;
        locals.var_phi_j0_dep_old_dn11 = assign24600_e18998_d_n11;
        locals.var_phi_j0_dep_old_dn14 = assign24600_e18998_d_n14;
        locals.var_phi_j0_dep_old_rv = 0.0;

        let (assign24610_e19006,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign24610_e19006;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        locals: &mut StampLocals,
    ) {
        let mut assign24620_loop_guard: usize = 0;
        while {
            let assign24620_cond_e19015: f64 = (150.0 + 1.0);
            let assign24620_cond_e19017: f64 = if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_lp_s0 <= assign24620_cond_e19015)) { 1.0 } else { 0.0 };
            assign24620_cond_e19017 != 0.0
        } {
            assign24620_loop_guard += 1;
            assert!(assign24620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign24620_body0_e19030, assign24620_body0_e19030_d_n0, assign24620_body0_e19030_d_n2, assign24620_body0_e19030_d_n4, assign24620_body0_e19030_d_n5, assign24620_body0_e19030_d_n6, assign24620_body0_e19030_d_n7, assign24620_body0_e19030_d_n8, assign24620_body0_e19030_d_n9, assign24620_body0_e19030_d_n10, assign24620_body0_e19030_d_n11, assign24620_body0_e19030_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body0_e19026: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign24620_body0_e19027: f64 = (locals.var_c_2esipq_ndepm * assign24620_body0_e19026);
        let assign24620_body0_e19028: f64 = (assign24620_body0_e19027).sqrt();
        (assign24620_body0_e19028, (((locals.var_c_2esipq_ndepm_dn0 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn2 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn4 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn5 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn6 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn7 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn8 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn9 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn10 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn11 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign24620_body0_e19028)), (((locals.var_c_2esipq_ndepm_dn14 * assign24620_body0_e19026) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign24620_body0_e19028)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24620_body0_e19030;
            locals.var_w_b0_dn0 = assign24620_body0_e19030_d_n0;
            locals.var_w_b0_dn2 = assign24620_body0_e19030_d_n2;
            locals.var_w_b0_dn4 = assign24620_body0_e19030_d_n4;
            locals.var_w_b0_dn5 = assign24620_body0_e19030_d_n5;
            locals.var_w_b0_dn6 = assign24620_body0_e19030_d_n6;
            locals.var_w_b0_dn7 = assign24620_body0_e19030_d_n7;
            locals.var_w_b0_dn8 = assign24620_body0_e19030_d_n8;
            locals.var_w_b0_dn9 = assign24620_body0_e19030_d_n9;
            locals.var_w_b0_dn10 = assign24620_body0_e19030_d_n10;
            locals.var_w_b0_dn11 = assign24620_body0_e19030_d_n11;
            locals.var_w_b0_dn14 = assign24620_body0_e19030_d_n14;
            locals.var_w_b0_rv = 0.0;
            let assign24620_body1_e19034: f64 = (locals.var_uc_depthn - 1e-8);
            let assign24620_body1_e19039: f64 = if ((locals.var_w_b0 > assign24620_body1_e19034) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard562 = assign24620_body1_e19039;
            locals.var_guard562_rv = 0.0;
            let (assign24620_body2_e19053, assign24620_body2_e19053_d_n0, assign24620_body2_e19053_d_n2, assign24620_body2_e19053_d_n4, assign24620_body2_e19053_d_n5, assign24620_body2_e19053_d_n6, assign24620_body2_e19053_d_n7, assign24620_body2_e19053_d_n8, assign24620_body2_e19053_d_n9, assign24620_body2_e19053_d_n10, assign24620_body2_e19053_d_n11, assign24620_body2_e19053_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body2_e19049: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign24620_body2_e19051: f64 = (assign24620_body2_e19049 + 1e-8);
        (assign24620_body2_e19051, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign24620_body2_e19053;
            locals.var_tmf1_dn0 = assign24620_body2_e19053_d_n0;
            locals.var_tmf1_dn2 = assign24620_body2_e19053_d_n2;
            locals.var_tmf1_dn4 = assign24620_body2_e19053_d_n4;
            locals.var_tmf1_dn5 = assign24620_body2_e19053_d_n5;
            locals.var_tmf1_dn6 = assign24620_body2_e19053_d_n6;
            locals.var_tmf1_dn7 = assign24620_body2_e19053_d_n7;
            locals.var_tmf1_dn8 = assign24620_body2_e19053_d_n8;
            locals.var_tmf1_dn9 = assign24620_body2_e19053_d_n9;
            locals.var_tmf1_dn10 = assign24620_body2_e19053_d_n10;
            locals.var_tmf1_dn11 = assign24620_body2_e19053_d_n11;
            locals.var_tmf1_dn14 = assign24620_body2_e19053_d_n14;
            locals.var_tmf1_rv = 0.0;
            let (assign24620_body3_e19065, assign24620_body3_e19065_d_n0, assign24620_body3_e19065_d_n2, assign24620_body3_e19065_d_n4, assign24620_body3_e19065_d_n5, assign24620_body3_e19065_d_n6, assign24620_body3_e19065_d_n7, assign24620_body3_e19065_d_n8, assign24620_body3_e19065_d_n9, assign24620_body3_e19065_d_n10, assign24620_body3_e19065_d_n11, assign24620_body3_e19065_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body3_e19063: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24620_body3_e19063, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign24620_body3_e19065;
            locals.var_x2_dn0 = assign24620_body3_e19065_d_n0;
            locals.var_x2_dn2 = assign24620_body3_e19065_d_n2;
            locals.var_x2_dn4 = assign24620_body3_e19065_d_n4;
            locals.var_x2_dn5 = assign24620_body3_e19065_d_n5;
            locals.var_x2_dn6 = assign24620_body3_e19065_d_n6;
            locals.var_x2_dn7 = assign24620_body3_e19065_d_n7;
            locals.var_x2_dn8 = assign24620_body3_e19065_d_n8;
            locals.var_x2_dn9 = assign24620_body3_e19065_d_n9;
            locals.var_x2_dn10 = assign24620_body3_e19065_d_n10;
            locals.var_x2_dn11 = assign24620_body3_e19065_d_n11;
            locals.var_x2_dn14 = assign24620_body3_e19065_d_n14;
            locals.var_x2_rv = 0.0;
            let (assign24620_body4_e19077, assign24620_body4_e19077_d_n0, assign24620_body4_e19077_d_n2, assign24620_body4_e19077_d_n4, assign24620_body4_e19077_d_n5, assign24620_body4_e19077_d_n6, assign24620_body4_e19077_d_n7, assign24620_body4_e19077_d_n8, assign24620_body4_e19077_d_n9, assign24620_body4_e19077_d_n10, assign24620_body4_e19077_d_n11, assign24620_body4_e19077_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body4_e19075: f64 = (1e-8 * 1e-8);
        (assign24620_body4_e19075, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign24620_body4_e19077;
            locals.var_xmax2_dn0 = assign24620_body4_e19077_d_n0;
            locals.var_xmax2_dn2 = assign24620_body4_e19077_d_n2;
            locals.var_xmax2_dn4 = assign24620_body4_e19077_d_n4;
            locals.var_xmax2_dn5 = assign24620_body4_e19077_d_n5;
            locals.var_xmax2_dn6 = assign24620_body4_e19077_d_n6;
            locals.var_xmax2_dn7 = assign24620_body4_e19077_d_n7;
            locals.var_xmax2_dn8 = assign24620_body4_e19077_d_n8;
            locals.var_xmax2_dn9 = assign24620_body4_e19077_d_n9;
            locals.var_xmax2_dn10 = assign24620_body4_e19077_d_n10;
            locals.var_xmax2_dn11 = assign24620_body4_e19077_d_n11;
            locals.var_xmax2_dn14 = assign24620_body4_e19077_d_n14;
            locals.var_xmax2_rv = 0.0;
            let (assign24620_body5_e19087, assign24620_body5_e19087_d_n0, assign24620_body5_e19087_d_n2, assign24620_body5_e19087_d_n4, assign24620_body5_e19087_d_n5, assign24620_body5_e19087_d_n6, assign24620_body5_e19087_d_n7, assign24620_body5_e19087_d_n8, assign24620_body5_e19087_d_n9, assign24620_body5_e19087_d_n10, assign24620_body5_e19087_d_n11, assign24620_body5_e19087_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body5_e19087;
            locals.var_xp_dn0 = assign24620_body5_e19087_d_n0;
            locals.var_xp_dn2 = assign24620_body5_e19087_d_n2;
            locals.var_xp_dn4 = assign24620_body5_e19087_d_n4;
            locals.var_xp_dn5 = assign24620_body5_e19087_d_n5;
            locals.var_xp_dn6 = assign24620_body5_e19087_d_n6;
            locals.var_xp_dn7 = assign24620_body5_e19087_d_n7;
            locals.var_xp_dn8 = assign24620_body5_e19087_d_n8;
            locals.var_xp_dn9 = assign24620_body5_e19087_d_n9;
            locals.var_xp_dn10 = assign24620_body5_e19087_d_n10;
            locals.var_xp_dn11 = assign24620_body5_e19087_d_n11;
            locals.var_xp_dn14 = assign24620_body5_e19087_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body6_e19097, assign24620_body6_e19097_d_n0, assign24620_body6_e19097_d_n2, assign24620_body6_e19097_d_n4, assign24620_body6_e19097_d_n5, assign24620_body6_e19097_d_n6, assign24620_body6_e19097_d_n7, assign24620_body6_e19097_d_n8, assign24620_body6_e19097_d_n9, assign24620_body6_e19097_d_n10, assign24620_body6_e19097_d_n11, assign24620_body6_e19097_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body6_e19097;
            locals.var_xmp_dn0 = assign24620_body6_e19097_d_n0;
            locals.var_xmp_dn2 = assign24620_body6_e19097_d_n2;
            locals.var_xmp_dn4 = assign24620_body6_e19097_d_n4;
            locals.var_xmp_dn5 = assign24620_body6_e19097_d_n5;
            locals.var_xmp_dn6 = assign24620_body6_e19097_d_n6;
            locals.var_xmp_dn7 = assign24620_body6_e19097_d_n7;
            locals.var_xmp_dn8 = assign24620_body6_e19097_d_n8;
            locals.var_xmp_dn9 = assign24620_body6_e19097_d_n9;
            locals.var_xmp_dn10 = assign24620_body6_e19097_d_n10;
            locals.var_xmp_dn11 = assign24620_body6_e19097_d_n11;
            locals.var_xmp_dn14 = assign24620_body6_e19097_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body7_e19107,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24620_body7_e19107;
            locals.var_m0_rv = 0.0;
            let (assign24620_body8_e19117,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body8_e19117;
            locals.var_mm_rv = 0.0;
            let (assign24620_body9_e19127, assign24620_body9_e19127_d_n0, assign24620_body9_e19127_d_n2, assign24620_body9_e19127_d_n4, assign24620_body9_e19127_d_n5, assign24620_body9_e19127_d_n6, assign24620_body9_e19127_d_n7, assign24620_body9_e19127_d_n8, assign24620_body9_e19127_d_n9, assign24620_body9_e19127_d_n10, assign24620_body9_e19127_d_n11, assign24620_body9_e19127_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24620_body9_e19127;
            locals.var_arg_dn0 = assign24620_body9_e19127_d_n0;
            locals.var_arg_dn2 = assign24620_body9_e19127_d_n2;
            locals.var_arg_dn4 = assign24620_body9_e19127_d_n4;
            locals.var_arg_dn5 = assign24620_body9_e19127_d_n5;
            locals.var_arg_dn6 = assign24620_body9_e19127_d_n6;
            locals.var_arg_dn7 = assign24620_body9_e19127_d_n7;
            locals.var_arg_dn8 = assign24620_body9_e19127_d_n8;
            locals.var_arg_dn9 = assign24620_body9_e19127_d_n9;
            locals.var_arg_dn10 = assign24620_body9_e19127_d_n10;
            locals.var_arg_dn11 = assign24620_body9_e19127_d_n11;
            locals.var_arg_dn14 = assign24620_body9_e19127_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24620_body10_e19137, assign24620_body10_e19137_d_n0, assign24620_body10_e19137_d_n2, assign24620_body10_e19137_d_n4, assign24620_body10_e19137_d_n5, assign24620_body10_e19137_d_n6, assign24620_body10_e19137_d_n7, assign24620_body10_e19137_d_n8, assign24620_body10_e19137_d_n9, assign24620_body10_e19137_d_n10, assign24620_body10_e19137_d_n11, assign24620_body10_e19137_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body10_e19137;
            locals.var_dnm_dn0 = assign24620_body10_e19137_d_n0;
            locals.var_dnm_dn2 = assign24620_body10_e19137_d_n2;
            locals.var_dnm_dn4 = assign24620_body10_e19137_d_n4;
            locals.var_dnm_dn5 = assign24620_body10_e19137_d_n5;
            locals.var_dnm_dn6 = assign24620_body10_e19137_d_n6;
            locals.var_dnm_dn7 = assign24620_body10_e19137_d_n7;
            locals.var_dnm_dn8 = assign24620_body10_e19137_d_n8;
            locals.var_dnm_dn9 = assign24620_body10_e19137_d_n9;
            locals.var_dnm_dn10 = assign24620_body10_e19137_d_n10;
            locals.var_dnm_dn11 = assign24620_body10_e19137_d_n11;
            locals.var_dnm_dn14 = assign24620_body10_e19137_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body11_e19149, assign24620_body11_e19149_d_n0, assign24620_body11_e19149_d_n2, assign24620_body11_e19149_d_n4, assign24620_body11_e19149_d_n5, assign24620_body11_e19149_d_n6, assign24620_body11_e19149_d_n7, assign24620_body11_e19149_d_n8, assign24620_body11_e19149_d_n9, assign24620_body11_e19149_d_n10, assign24620_body11_e19149_d_n11, assign24620_body11_e19149_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body11_e19147: f64 = (locals.var_xp * locals.var_x2);
        (assign24620_body11_e19147, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body11_e19149;
            locals.var_xp_dn0 = assign24620_body11_e19149_d_n0;
            locals.var_xp_dn2 = assign24620_body11_e19149_d_n2;
            locals.var_xp_dn4 = assign24620_body11_e19149_d_n4;
            locals.var_xp_dn5 = assign24620_body11_e19149_d_n5;
            locals.var_xp_dn6 = assign24620_body11_e19149_d_n6;
            locals.var_xp_dn7 = assign24620_body11_e19149_d_n7;
            locals.var_xp_dn8 = assign24620_body11_e19149_d_n8;
            locals.var_xp_dn9 = assign24620_body11_e19149_d_n9;
            locals.var_xp_dn10 = assign24620_body11_e19149_d_n10;
            locals.var_xp_dn11 = assign24620_body11_e19149_d_n11;
            locals.var_xp_dn14 = assign24620_body11_e19149_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body12_e19161, assign24620_body12_e19161_d_n0, assign24620_body12_e19161_d_n2, assign24620_body12_e19161_d_n4, assign24620_body12_e19161_d_n5, assign24620_body12_e19161_d_n6, assign24620_body12_e19161_d_n7, assign24620_body12_e19161_d_n8, assign24620_body12_e19161_d_n9, assign24620_body12_e19161_d_n10, assign24620_body12_e19161_d_n11, assign24620_body12_e19161_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body12_e19159: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24620_body12_e19159, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body12_e19161;
            locals.var_xmp_dn0 = assign24620_body12_e19161_d_n0;
            locals.var_xmp_dn2 = assign24620_body12_e19161_d_n2;
            locals.var_xmp_dn4 = assign24620_body12_e19161_d_n4;
            locals.var_xmp_dn5 = assign24620_body12_e19161_d_n5;
            locals.var_xmp_dn6 = assign24620_body12_e19161_d_n6;
            locals.var_xmp_dn7 = assign24620_body12_e19161_d_n7;
            locals.var_xmp_dn8 = assign24620_body12_e19161_d_n8;
            locals.var_xmp_dn9 = assign24620_body12_e19161_d_n9;
            locals.var_xmp_dn10 = assign24620_body12_e19161_d_n10;
            locals.var_xmp_dn11 = assign24620_body12_e19161_d_n11;
            locals.var_xmp_dn14 = assign24620_body12_e19161_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body13_e19173, assign24620_body13_e19173_d_n0, assign24620_body13_e19173_d_n2, assign24620_body13_e19173_d_n4, assign24620_body13_e19173_d_n5, assign24620_body13_e19173_d_n6, assign24620_body13_e19173_d_n7, assign24620_body13_e19173_d_n8, assign24620_body13_e19173_d_n9, assign24620_body13_e19173_d_n10, assign24620_body13_e19173_d_n11, assign24620_body13_e19173_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body13_e19171: f64 = (locals.var_xp * locals.var_x2);
        (assign24620_body13_e19171, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body13_e19173;
            locals.var_xp_dn0 = assign24620_body13_e19173_d_n0;
            locals.var_xp_dn2 = assign24620_body13_e19173_d_n2;
            locals.var_xp_dn4 = assign24620_body13_e19173_d_n4;
            locals.var_xp_dn5 = assign24620_body13_e19173_d_n5;
            locals.var_xp_dn6 = assign24620_body13_e19173_d_n6;
            locals.var_xp_dn7 = assign24620_body13_e19173_d_n7;
            locals.var_xp_dn8 = assign24620_body13_e19173_d_n8;
            locals.var_xp_dn9 = assign24620_body13_e19173_d_n9;
            locals.var_xp_dn10 = assign24620_body13_e19173_d_n10;
            locals.var_xp_dn11 = assign24620_body13_e19173_d_n11;
            locals.var_xp_dn14 = assign24620_body13_e19173_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body14_e19185, assign24620_body14_e19185_d_n0, assign24620_body14_e19185_d_n2, assign24620_body14_e19185_d_n4, assign24620_body14_e19185_d_n5, assign24620_body14_e19185_d_n6, assign24620_body14_e19185_d_n7, assign24620_body14_e19185_d_n8, assign24620_body14_e19185_d_n9, assign24620_body14_e19185_d_n10, assign24620_body14_e19185_d_n11, assign24620_body14_e19185_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body14_e19183: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24620_body14_e19183, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body14_e19185;
            locals.var_xmp_dn0 = assign24620_body14_e19185_d_n0;
            locals.var_xmp_dn2 = assign24620_body14_e19185_d_n2;
            locals.var_xmp_dn4 = assign24620_body14_e19185_d_n4;
            locals.var_xmp_dn5 = assign24620_body14_e19185_d_n5;
            locals.var_xmp_dn6 = assign24620_body14_e19185_d_n6;
            locals.var_xmp_dn7 = assign24620_body14_e19185_d_n7;
            locals.var_xmp_dn8 = assign24620_body14_e19185_d_n8;
            locals.var_xmp_dn9 = assign24620_body14_e19185_d_n9;
            locals.var_xmp_dn10 = assign24620_body14_e19185_d_n10;
            locals.var_xmp_dn11 = assign24620_body14_e19185_d_n11;
            locals.var_xmp_dn14 = assign24620_body14_e19185_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body15_e19197, assign24620_body15_e19197_d_n0, assign24620_body15_e19197_d_n2, assign24620_body15_e19197_d_n4, assign24620_body15_e19197_d_n5, assign24620_body15_e19197_d_n6, assign24620_body15_e19197_d_n7, assign24620_body15_e19197_d_n8, assign24620_body15_e19197_d_n9, assign24620_body15_e19197_d_n10, assign24620_body15_e19197_d_n11, assign24620_body15_e19197_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body15_e19195: f64 = (locals.var_xp + locals.var_xmp);
        (assign24620_body15_e19195, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24620_body15_e19197;
            locals.var_arg_dn0 = assign24620_body15_e19197_d_n0;
            locals.var_arg_dn2 = assign24620_body15_e19197_d_n2;
            locals.var_arg_dn4 = assign24620_body15_e19197_d_n4;
            locals.var_arg_dn5 = assign24620_body15_e19197_d_n5;
            locals.var_arg_dn6 = assign24620_body15_e19197_d_n6;
            locals.var_arg_dn7 = assign24620_body15_e19197_d_n7;
            locals.var_arg_dn8 = assign24620_body15_e19197_d_n8;
            locals.var_arg_dn9 = assign24620_body15_e19197_d_n9;
            locals.var_arg_dn10 = assign24620_body15_e19197_d_n10;
            locals.var_arg_dn11 = assign24620_body15_e19197_d_n11;
            locals.var_arg_dn14 = assign24620_body15_e19197_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24620_body16_e19207, assign24620_body16_e19207_d_n0, assign24620_body16_e19207_d_n2, assign24620_body16_e19207_d_n4, assign24620_body16_e19207_d_n5, assign24620_body16_e19207_d_n6, assign24620_body16_e19207_d_n7, assign24620_body16_e19207_d_n8, assign24620_body16_e19207_d_n9, assign24620_body16_e19207_d_n10, assign24620_body16_e19207_d_n11, assign24620_body16_e19207_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body16_e19207;
            locals.var_dnm_dn0 = assign24620_body16_e19207_d_n0;
            locals.var_dnm_dn2 = assign24620_body16_e19207_d_n2;
            locals.var_dnm_dn4 = assign24620_body16_e19207_d_n4;
            locals.var_dnm_dn5 = assign24620_body16_e19207_d_n5;
            locals.var_dnm_dn6 = assign24620_body16_e19207_d_n6;
            locals.var_dnm_dn7 = assign24620_body16_e19207_d_n7;
            locals.var_dnm_dn8 = assign24620_body16_e19207_d_n8;
            locals.var_dnm_dn9 = assign24620_body16_e19207_d_n9;
            locals.var_dnm_dn10 = assign24620_body16_e19207_d_n10;
            locals.var_dnm_dn11 = assign24620_body16_e19207_d_n11;
            locals.var_dnm_dn14 = assign24620_body16_e19207_d_n14;
            locals.var_dnm_rv = 0.0;
            let assign24620_body17_e19222: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard563 = assign24620_body17_e19222;
            locals.var_guard563_rv = 0.0;
            let assign24620_body18_e19225: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard564 = assign24620_body18_e19225;
            locals.var_guard564_rv = 0.0;
            let (assign24620_body19_e19239,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body19_e19239;
            locals.var_mm_rv = 0.0;
            let assign24620_body20_e19242: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard565 = assign24620_body20_e19242;
            locals.var_guard565_rv = 0.0;
            let (assign24620_body21_e19259,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body21_e19259;
            locals.var_mm_rv = 0.0;
            let assign24620_body22_e19262: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard566 = assign24620_body22_e19262;
            locals.var_guard566_rv = 0.0;
            let (assign24620_body23_e19282,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard566 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body23_e19282;
            locals.var_mm_rv = 0.0;
            let assign24620_body24_e19285: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard567 = assign24620_body24_e19285;
            locals.var_guard567_rv = 0.0;
            let (assign24620_body25_e19308,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard566 == 0.0)) && (locals.var_guard567 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body25_e19308;
            locals.var_mm_rv = 0.0;
            let (assign24620_body26_e19320,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24620_body26_e19320;
            locals.var_m0_rv = 0.0;
            let mut assign24620_body27_loop_guard: usize = 0;
            while {
                let assign24620_body27_cond_e19333: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24620_body27_cond_e19333 != 0.0
            } {
                assign24620_body27_loop_guard += 1;
                assert!(assign24620_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24620_body27_body0_e19346, assign24620_body27_body0_e19346_d_n0, assign24620_body27_body0_e19346_d_n2, assign24620_body27_body0_e19346_d_n4, assign24620_body27_body0_e19346_d_n5, assign24620_body27_body0_e19346_d_n6, assign24620_body27_body0_e19346_d_n7, assign24620_body27_body0_e19346_d_n8, assign24620_body27_body0_e19346_d_n9, assign24620_body27_body0_e19346_d_n10, assign24620_body27_body0_e19346_d_n11, assign24620_body27_body0_e19346_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24620_body27_body0_e19344: f64 = (locals.var_dnm).sqrt();
        (assign24620_body27_body0_e19344, (locals.var_dnm_dn0 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn2 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn4 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn5 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn6 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn7 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn8 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn9 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn10 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn11 / (2.0 * assign24620_body27_body0_e19344)), (locals.var_dnm_dn14 / (2.0 * assign24620_body27_body0_e19344)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign24620_body27_body0_e19346;
                locals.var_dnm_dn0 = assign24620_body27_body0_e19346_d_n0;
                locals.var_dnm_dn2 = assign24620_body27_body0_e19346_d_n2;
                locals.var_dnm_dn4 = assign24620_body27_body0_e19346_d_n4;
                locals.var_dnm_dn5 = assign24620_body27_body0_e19346_d_n5;
                locals.var_dnm_dn6 = assign24620_body27_body0_e19346_d_n6;
                locals.var_dnm_dn7 = assign24620_body27_body0_e19346_d_n7;
                locals.var_dnm_dn8 = assign24620_body27_body0_e19346_d_n8;
                locals.var_dnm_dn9 = assign24620_body27_body0_e19346_d_n9;
                locals.var_dnm_dn10 = assign24620_body27_body0_e19346_d_n10;
                locals.var_dnm_dn11 = assign24620_body27_body0_e19346_d_n11;
                locals.var_dnm_dn14 = assign24620_body27_body0_e19346_d_n14;
                locals.var_dnm_rv = 0.0;
                let (assign24620_body27_body1_e19360,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign24620_body27_body1_e19358: f64 = (locals.var_m0 + 1.0);
        (assign24620_body27_body1_e19358,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24620_body27_body1_e19360;
                locals.var_m0_rv = 0.0;
            }
            let (assign24620_body28_e19384, assign24620_body28_e19384_d_n0, assign24620_body28_e19384_d_n2, assign24620_body28_e19384_d_n4, assign24620_body28_e19384_d_n5, assign24620_body28_e19384_d_n6, assign24620_body28_e19384_d_n7, assign24620_body28_e19384_d_n8, assign24620_body28_e19384_d_n9, assign24620_body28_e19384_d_n10, assign24620_body28_e19384_d_n11, assign24620_body28_e19384_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let (assign24620_body28_e19382, assign24620_body28_e19382_d_n0, assign24620_body28_e19382_d_n2, assign24620_body28_e19382_d_n4, assign24620_body28_e19382_d_n5, assign24620_body28_e19382_d_n6, assign24620_body28_e19382_d_n7, assign24620_body28_e19382_d_n8, assign24620_body28_e19382_d_n9, assign24620_body28_e19382_d_n10, assign24620_body28_e19382_d_n11, assign24620_body28_e19382_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24620_body28_e19379: f64 = (2.0 * 2.0);
                let assign24620_body28_e19380: f64 = (1.0 / assign24620_body28_e19379);
                let assign24620_body28_e19381: f64 = (locals.var_dnm).powf(assign24620_body28_e19380);
                (assign24620_body28_e19381, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn11)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body28_e19380) as f64).is_finite() && ((assign24620_body28_e19380) as f64).fract() == 0.0 { if assign24620_body28_e19380 == 0.0 { 0.0 } else { (assign24620_body28_e19380 * ((locals.var_dnm).powf(assign24620_body28_e19380 - 1.0) * locals.var_dnm_dn14)) } } else { (assign24620_body28_e19381 * (assign24620_body28_e19380 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign24620_body28_e19382, assign24620_body28_e19382_d_n0, assign24620_body28_e19382_d_n2, assign24620_body28_e19382_d_n4, assign24620_body28_e19382_d_n5, assign24620_body28_e19382_d_n6, assign24620_body28_e19382_d_n7, assign24620_body28_e19382_d_n8, assign24620_body28_e19382_d_n9, assign24620_body28_e19382_d_n10, assign24620_body28_e19382_d_n11, assign24620_body28_e19382_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body28_e19384;
            locals.var_dnm_dn0 = assign24620_body28_e19384_d_n0;
            locals.var_dnm_dn2 = assign24620_body28_e19384_d_n2;
            locals.var_dnm_dn4 = assign24620_body28_e19384_d_n4;
            locals.var_dnm_dn5 = assign24620_body28_e19384_d_n5;
            locals.var_dnm_dn6 = assign24620_body28_e19384_d_n6;
            locals.var_dnm_dn7 = assign24620_body28_e19384_d_n7;
            locals.var_dnm_dn8 = assign24620_body28_e19384_d_n8;
            locals.var_dnm_dn9 = assign24620_body28_e19384_d_n9;
            locals.var_dnm_dn10 = assign24620_body28_e19384_d_n10;
            locals.var_dnm_dn11 = assign24620_body28_e19384_d_n11;
            locals.var_dnm_dn14 = assign24620_body28_e19384_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body29_e19396, assign24620_body29_e19396_d_n0, assign24620_body29_e19396_d_n2, assign24620_body29_e19396_d_n4, assign24620_body29_e19396_d_n5, assign24620_body29_e19396_d_n6, assign24620_body29_e19396_d_n7, assign24620_body29_e19396_d_n8, assign24620_body29_e19396_d_n9, assign24620_body29_e19396_d_n10, assign24620_body29_e19396_d_n11, assign24620_body29_e19396_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body29_e19394: f64 = (1.0 / locals.var_dnm);
        (assign24620_body29_e19394, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body29_e19396;
            locals.var_dnm_dn0 = assign24620_body29_e19396_d_n0;
            locals.var_dnm_dn2 = assign24620_body29_e19396_d_n2;
            locals.var_dnm_dn4 = assign24620_body29_e19396_d_n4;
            locals.var_dnm_dn5 = assign24620_body29_e19396_d_n5;
            locals.var_dnm_dn6 = assign24620_body29_e19396_d_n6;
            locals.var_dnm_dn7 = assign24620_body29_e19396_d_n7;
            locals.var_dnm_dn8 = assign24620_body29_e19396_d_n8;
            locals.var_dnm_dn9 = assign24620_body29_e19396_d_n9;
            locals.var_dnm_dn10 = assign24620_body29_e19396_d_n10;
            locals.var_dnm_dn11 = assign24620_body29_e19396_d_n11;
            locals.var_dnm_dn14 = assign24620_body29_e19396_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body30_e19410, assign24620_body30_e19410_d_n0, assign24620_body30_e19410_d_n2, assign24620_body30_e19410_d_n4, assign24620_body30_e19410_d_n5, assign24620_body30_e19410_d_n6, assign24620_body30_e19410_d_n7, assign24620_body30_e19410_d_n8, assign24620_body30_e19410_d_n9, assign24620_body30_e19410_d_n10, assign24620_body30_e19410_d_n11, assign24620_body30_e19410_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body30_e19406: f64 = (locals.var_tmf1 * 1e-8);
        let assign24620_body30_e19408: f64 = (assign24620_body30_e19406 * locals.var_dnm);
        (assign24620_body30_e19408, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign24620_body30_e19406 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign24620_body30_e19410;
            locals.var_tmf0_dn0 = assign24620_body30_e19410_d_n0;
            locals.var_tmf0_dn2 = assign24620_body30_e19410_d_n2;
            locals.var_tmf0_dn4 = assign24620_body30_e19410_d_n4;
            locals.var_tmf0_dn5 = assign24620_body30_e19410_d_n5;
            locals.var_tmf0_dn6 = assign24620_body30_e19410_d_n6;
            locals.var_tmf0_dn7 = assign24620_body30_e19410_d_n7;
            locals.var_tmf0_dn8 = assign24620_body30_e19410_d_n8;
            locals.var_tmf0_dn9 = assign24620_body30_e19410_d_n9;
            locals.var_tmf0_dn10 = assign24620_body30_e19410_d_n10;
            locals.var_tmf0_dn11 = assign24620_body30_e19410_d_n11;
            locals.var_tmf0_dn14 = assign24620_body30_e19410_d_n14;
            locals.var_tmf0_rv = 0.0;
            let (assign24620_body31_e19426, assign24620_body31_e19426_d_n0, assign24620_body31_e19426_d_n2, assign24620_body31_e19426_d_n4, assign24620_body31_e19426_d_n5, assign24620_body31_e19426_d_n6, assign24620_body31_e19426_d_n7, assign24620_body31_e19426_d_n8, assign24620_body31_e19426_d_n9, assign24620_body31_e19426_d_n10, assign24620_body31_e19426_d_n11, assign24620_body31_e19426_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body31_e19420: f64 = (1e-8 * locals.var_xmp);
        let assign24620_body31_e19422: f64 = (assign24620_body31_e19420 * locals.var_dnm);
        let assign24620_body31_e19424: f64 = (assign24620_body31_e19422 / locals.var_arg);
        (assign24620_body31_e19424, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn11)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign24620_body31_e19420 * locals.var_dnm_dn14)) * locals.var_arg) - (assign24620_body31_e19422 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24620_body31_e19426;
            locals.var_t0_dn0 = assign24620_body31_e19426_d_n0;
            locals.var_t0_dn2 = assign24620_body31_e19426_d_n2;
            locals.var_t0_dn4 = assign24620_body31_e19426_d_n4;
            locals.var_t0_dn5 = assign24620_body31_e19426_d_n5;
            locals.var_t0_dn6 = assign24620_body31_e19426_d_n6;
            locals.var_t0_dn7 = assign24620_body31_e19426_d_n7;
            locals.var_t0_dn8 = assign24620_body31_e19426_d_n8;
            locals.var_t0_dn9 = assign24620_body31_e19426_d_n9;
            locals.var_t0_dn10 = assign24620_body31_e19426_d_n10;
            locals.var_t0_dn11 = assign24620_body31_e19426_d_n11;
            locals.var_t0_dn14 = assign24620_body31_e19426_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24620_body32_e19440, assign24620_body32_e19440_d_n0, assign24620_body32_e19440_d_n2, assign24620_body32_e19440_d_n4, assign24620_body32_e19440_d_n5, assign24620_body32_e19440_d_n6, assign24620_body32_e19440_d_n7, assign24620_body32_e19440_d_n8, assign24620_body32_e19440_d_n9, assign24620_body32_e19440_d_n10, assign24620_body32_e19440_d_n11, assign24620_body32_e19440_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign24620_body32_e19436: f64 = (locals.var_uc_depthn - 1e-8);
        let assign24620_body32_e19438: f64 = (assign24620_body32_e19436 + locals.var_tmf0);
        (assign24620_body32_e19438, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24620_body32_e19440;
            locals.var_w_b0_dn0 = assign24620_body32_e19440_d_n0;
            locals.var_w_b0_dn2 = assign24620_body32_e19440_d_n2;
            locals.var_w_b0_dn4 = assign24620_body32_e19440_d_n4;
            locals.var_w_b0_dn5 = assign24620_body32_e19440_d_n5;
            locals.var_w_b0_dn6 = assign24620_body32_e19440_d_n6;
            locals.var_w_b0_dn7 = assign24620_body32_e19440_d_n7;
            locals.var_w_b0_dn8 = assign24620_body32_e19440_d_n8;
            locals.var_w_b0_dn9 = assign24620_body32_e19440_d_n9;
            locals.var_w_b0_dn10 = assign24620_body32_e19440_d_n10;
            locals.var_w_b0_dn11 = assign24620_body32_e19440_d_n11;
            locals.var_w_b0_dn14 = assign24620_body32_e19440_d_n14;
            locals.var_w_b0_rv = 0.0;
            let (assign24620_body33_e19450, assign24620_body33_e19450_d_n0, assign24620_body33_e19450_d_n2, assign24620_body33_e19450_d_n4, assign24620_body33_e19450_d_n5, assign24620_body33_e19450_d_n6, assign24620_body33_e19450_d_n7, assign24620_body33_e19450_d_n8, assign24620_body33_e19450_d_n9, assign24620_body33_e19450_d_n10, assign24620_body33_e19450_d_n11, assign24620_body33_e19450_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24620_body33_e19450;
            locals.var_t0_dn0 = assign24620_body33_e19450_d_n0;
            locals.var_t0_dn2 = assign24620_body33_e19450_d_n2;
            locals.var_t0_dn4 = assign24620_body33_e19450_d_n4;
            locals.var_t0_dn5 = assign24620_body33_e19450_d_n5;
            locals.var_t0_dn6 = assign24620_body33_e19450_d_n6;
            locals.var_t0_dn7 = assign24620_body33_e19450_d_n7;
            locals.var_t0_dn8 = assign24620_body33_e19450_d_n8;
            locals.var_t0_dn9 = assign24620_body33_e19450_d_n9;
            locals.var_t0_dn10 = assign24620_body33_e19450_d_n10;
            locals.var_t0_dn11 = assign24620_body33_e19450_d_n11;
            locals.var_t0_dn14 = assign24620_body33_e19450_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24620_body34_e19461, assign24620_body34_e19461_d_n0, assign24620_body34_e19461_d_n2, assign24620_body34_e19461_d_n4, assign24620_body34_e19461_d_n5, assign24620_body34_e19461_d_n6, assign24620_body34_e19461_d_n7, assign24620_body34_e19461_d_n8, assign24620_body34_e19461_d_n9, assign24620_body34_e19461_d_n10, assign24620_body34_e19461_d_n11, assign24620_body34_e19461_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign24620_body34_e19461;
            locals.var_w_b0_dn0 = assign24620_body34_e19461_d_n0;
            locals.var_w_b0_dn2 = assign24620_body34_e19461_d_n2;
            locals.var_w_b0_dn4 = assign24620_body34_e19461_d_n4;
            locals.var_w_b0_dn5 = assign24620_body34_e19461_d_n5;
            locals.var_w_b0_dn6 = assign24620_body34_e19461_d_n6;
            locals.var_w_b0_dn7 = assign24620_body34_e19461_d_n7;
            locals.var_w_b0_dn8 = assign24620_body34_e19461_d_n8;
            locals.var_w_b0_dn9 = assign24620_body34_e19461_d_n9;
            locals.var_w_b0_dn10 = assign24620_body34_e19461_d_n10;
            locals.var_w_b0_dn11 = assign24620_body34_e19461_d_n11;
            locals.var_w_b0_dn14 = assign24620_body34_e19461_d_n14;
            locals.var_w_b0_rv = 0.0;
            let (assign24620_body35_e19472, assign24620_body35_e19472_d_n0, assign24620_body35_e19472_d_n2, assign24620_body35_e19472_d_n4, assign24620_body35_e19472_d_n5, assign24620_body35_e19472_d_n6, assign24620_body35_e19472_d_n7, assign24620_body35_e19472_d_n8, assign24620_body35_e19472_d_n9, assign24620_body35_e19472_d_n10, assign24620_body35_e19472_d_n11, assign24620_body35_e19472_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign24620_body35_e19472;
            locals.var_t0_dn0 = assign24620_body35_e19472_d_n0;
            locals.var_t0_dn2 = assign24620_body35_e19472_d_n2;
            locals.var_t0_dn4 = assign24620_body35_e19472_d_n4;
            locals.var_t0_dn5 = assign24620_body35_e19472_d_n5;
            locals.var_t0_dn6 = assign24620_body35_e19472_d_n6;
            locals.var_t0_dn7 = assign24620_body35_e19472_d_n7;
            locals.var_t0_dn8 = assign24620_body35_e19472_d_n8;
            locals.var_t0_dn9 = assign24620_body35_e19472_d_n9;
            locals.var_t0_dn10 = assign24620_body35_e19472_d_n10;
            locals.var_t0_dn11 = assign24620_body35_e19472_d_n11;
            locals.var_t0_dn14 = assign24620_body35_e19472_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign24620_body36_e19484, assign24620_body36_e19484_d_n0, assign24620_body36_e19484_d_n2, assign24620_body36_e19484_d_n4, assign24620_body36_e19484_d_n5, assign24620_body36_e19484_d_n6, assign24620_body36_e19484_d_n7, assign24620_body36_e19484_d_n8, assign24620_body36_e19484_d_n9, assign24620_body36_e19484_d_n10, assign24620_body36_e19484_d_n11, assign24620_body36_e19484_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body36_e19480: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk437);
        let assign24620_body36_e19482: f64 = (assign24620_body36_e19480 + locals.var_vbi_dep);
        (assign24620_body36_e19482, ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0), ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2), ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4), ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5), ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6), ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7), ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8), ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9), ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10), ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11), ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign24620_body36_e19484;
            locals.var_t1_dn0 = assign24620_body36_e19484_d_n0;
            locals.var_t1_dn2 = assign24620_body36_e19484_d_n2;
            locals.var_t1_dn4 = assign24620_body36_e19484_d_n4;
            locals.var_t1_dn5 = assign24620_body36_e19484_d_n5;
            locals.var_t1_dn6 = assign24620_body36_e19484_d_n6;
            locals.var_t1_dn7 = assign24620_body36_e19484_d_n7;
            locals.var_t1_dn8 = assign24620_body36_e19484_d_n8;
            locals.var_t1_dn9 = assign24620_body36_e19484_d_n9;
            locals.var_t1_dn10 = assign24620_body36_e19484_d_n10;
            locals.var_t1_dn11 = assign24620_body36_e19484_d_n11;
            locals.var_t1_dn14 = assign24620_body36_e19484_d_n14;
            locals.var_t1_rv = 0.0;
            let assign24620_body37_e19488: f64 = 0.1;
            let assign24620_body37_e19493: f64 = if ((locals.var_t1 < assign24620_body37_e19488) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard568 = assign24620_body37_e19493;
            locals.var_guard568_rv = 0.0;
            let (assign24620_body38_e19507, assign24620_body38_e19507_d_n0, assign24620_body38_e19507_d_n2, assign24620_body38_e19507_d_n4, assign24620_body38_e19507_d_n5, assign24620_body38_e19507_d_n6, assign24620_body38_e19507_d_n7, assign24620_body38_e19507_d_n8, assign24620_body38_e19507_d_n9, assign24620_body38_e19507_d_n10, assign24620_body38_e19507_d_n11, assign24620_body38_e19507_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body38_e19503: f64 = 0.1;
        let assign24620_body38_e19505: f64 = (assign24620_body38_e19503 - locals.var_t1);
        (assign24620_body38_e19505, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign24620_body38_e19507;
            locals.var_tmf1_dn0 = assign24620_body38_e19507_d_n0;
            locals.var_tmf1_dn2 = assign24620_body38_e19507_d_n2;
            locals.var_tmf1_dn4 = assign24620_body38_e19507_d_n4;
            locals.var_tmf1_dn5 = assign24620_body38_e19507_d_n5;
            locals.var_tmf1_dn6 = assign24620_body38_e19507_d_n6;
            locals.var_tmf1_dn7 = assign24620_body38_e19507_d_n7;
            locals.var_tmf1_dn8 = assign24620_body38_e19507_d_n8;
            locals.var_tmf1_dn9 = assign24620_body38_e19507_d_n9;
            locals.var_tmf1_dn10 = assign24620_body38_e19507_d_n10;
            locals.var_tmf1_dn11 = assign24620_body38_e19507_d_n11;
            locals.var_tmf1_dn14 = assign24620_body38_e19507_d_n14;
            locals.var_tmf1_rv = 0.0;
            let (assign24620_body39_e19519, assign24620_body39_e19519_d_n0, assign24620_body39_e19519_d_n2, assign24620_body39_e19519_d_n4, assign24620_body39_e19519_d_n5, assign24620_body39_e19519_d_n6, assign24620_body39_e19519_d_n7, assign24620_body39_e19519_d_n8, assign24620_body39_e19519_d_n9, assign24620_body39_e19519_d_n10, assign24620_body39_e19519_d_n11, assign24620_body39_e19519_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body39_e19517: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24620_body39_e19517, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign24620_body39_e19519;
            locals.var_x2_dn0 = assign24620_body39_e19519_d_n0;
            locals.var_x2_dn2 = assign24620_body39_e19519_d_n2;
            locals.var_x2_dn4 = assign24620_body39_e19519_d_n4;
            locals.var_x2_dn5 = assign24620_body39_e19519_d_n5;
            locals.var_x2_dn6 = assign24620_body39_e19519_d_n6;
            locals.var_x2_dn7 = assign24620_body39_e19519_d_n7;
            locals.var_x2_dn8 = assign24620_body39_e19519_d_n8;
            locals.var_x2_dn9 = assign24620_body39_e19519_d_n9;
            locals.var_x2_dn10 = assign24620_body39_e19519_d_n10;
            locals.var_x2_dn11 = assign24620_body39_e19519_d_n11;
            locals.var_x2_dn14 = assign24620_body39_e19519_d_n14;
            locals.var_x2_rv = 0.0;
            let (assign24620_body40_e19531, assign24620_body40_e19531_d_n0, assign24620_body40_e19531_d_n2, assign24620_body40_e19531_d_n4, assign24620_body40_e19531_d_n5, assign24620_body40_e19531_d_n6, assign24620_body40_e19531_d_n7, assign24620_body40_e19531_d_n8, assign24620_body40_e19531_d_n9, assign24620_body40_e19531_d_n10, assign24620_body40_e19531_d_n11, assign24620_body40_e19531_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body40_e19529: f64 = (0.1 * 0.1);
        (assign24620_body40_e19529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign24620_body40_e19531;
            locals.var_xmax2_dn0 = assign24620_body40_e19531_d_n0;
            locals.var_xmax2_dn2 = assign24620_body40_e19531_d_n2;
            locals.var_xmax2_dn4 = assign24620_body40_e19531_d_n4;
            locals.var_xmax2_dn5 = assign24620_body40_e19531_d_n5;
            locals.var_xmax2_dn6 = assign24620_body40_e19531_d_n6;
            locals.var_xmax2_dn7 = assign24620_body40_e19531_d_n7;
            locals.var_xmax2_dn8 = assign24620_body40_e19531_d_n8;
            locals.var_xmax2_dn9 = assign24620_body40_e19531_d_n9;
            locals.var_xmax2_dn10 = assign24620_body40_e19531_d_n10;
            locals.var_xmax2_dn11 = assign24620_body40_e19531_d_n11;
            locals.var_xmax2_dn14 = assign24620_body40_e19531_d_n14;
            locals.var_xmax2_rv = 0.0;
            let (assign24620_body41_e19541, assign24620_body41_e19541_d_n0, assign24620_body41_e19541_d_n2, assign24620_body41_e19541_d_n4, assign24620_body41_e19541_d_n5, assign24620_body41_e19541_d_n6, assign24620_body41_e19541_d_n7, assign24620_body41_e19541_d_n8, assign24620_body41_e19541_d_n9, assign24620_body41_e19541_d_n10, assign24620_body41_e19541_d_n11, assign24620_body41_e19541_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body41_e19541;
            locals.var_xp_dn0 = assign24620_body41_e19541_d_n0;
            locals.var_xp_dn2 = assign24620_body41_e19541_d_n2;
            locals.var_xp_dn4 = assign24620_body41_e19541_d_n4;
            locals.var_xp_dn5 = assign24620_body41_e19541_d_n5;
            locals.var_xp_dn6 = assign24620_body41_e19541_d_n6;
            locals.var_xp_dn7 = assign24620_body41_e19541_d_n7;
            locals.var_xp_dn8 = assign24620_body41_e19541_d_n8;
            locals.var_xp_dn9 = assign24620_body41_e19541_d_n9;
            locals.var_xp_dn10 = assign24620_body41_e19541_d_n10;
            locals.var_xp_dn11 = assign24620_body41_e19541_d_n11;
            locals.var_xp_dn14 = assign24620_body41_e19541_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body42_e19551, assign24620_body42_e19551_d_n0, assign24620_body42_e19551_d_n2, assign24620_body42_e19551_d_n4, assign24620_body42_e19551_d_n5, assign24620_body42_e19551_d_n6, assign24620_body42_e19551_d_n7, assign24620_body42_e19551_d_n8, assign24620_body42_e19551_d_n9, assign24620_body42_e19551_d_n10, assign24620_body42_e19551_d_n11, assign24620_body42_e19551_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body42_e19551;
            locals.var_xmp_dn0 = assign24620_body42_e19551_d_n0;
            locals.var_xmp_dn2 = assign24620_body42_e19551_d_n2;
            locals.var_xmp_dn4 = assign24620_body42_e19551_d_n4;
            locals.var_xmp_dn5 = assign24620_body42_e19551_d_n5;
            locals.var_xmp_dn6 = assign24620_body42_e19551_d_n6;
            locals.var_xmp_dn7 = assign24620_body42_e19551_d_n7;
            locals.var_xmp_dn8 = assign24620_body42_e19551_d_n8;
            locals.var_xmp_dn9 = assign24620_body42_e19551_d_n9;
            locals.var_xmp_dn10 = assign24620_body42_e19551_d_n10;
            locals.var_xmp_dn11 = assign24620_body42_e19551_d_n11;
            locals.var_xmp_dn14 = assign24620_body42_e19551_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body43_e19561,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24620_body43_e19561;
            locals.var_m0_rv = 0.0;
            let (assign24620_body44_e19571,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body44_e19571;
            locals.var_mm_rv = 0.0;
            let (assign24620_body45_e19581, assign24620_body45_e19581_d_n0, assign24620_body45_e19581_d_n2, assign24620_body45_e19581_d_n4, assign24620_body45_e19581_d_n5, assign24620_body45_e19581_d_n6, assign24620_body45_e19581_d_n7, assign24620_body45_e19581_d_n8, assign24620_body45_e19581_d_n9, assign24620_body45_e19581_d_n10, assign24620_body45_e19581_d_n11, assign24620_body45_e19581_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24620_body45_e19581;
            locals.var_arg_dn0 = assign24620_body45_e19581_d_n0;
            locals.var_arg_dn2 = assign24620_body45_e19581_d_n2;
            locals.var_arg_dn4 = assign24620_body45_e19581_d_n4;
            locals.var_arg_dn5 = assign24620_body45_e19581_d_n5;
            locals.var_arg_dn6 = assign24620_body45_e19581_d_n6;
            locals.var_arg_dn7 = assign24620_body45_e19581_d_n7;
            locals.var_arg_dn8 = assign24620_body45_e19581_d_n8;
            locals.var_arg_dn9 = assign24620_body45_e19581_d_n9;
            locals.var_arg_dn10 = assign24620_body45_e19581_d_n10;
            locals.var_arg_dn11 = assign24620_body45_e19581_d_n11;
            locals.var_arg_dn14 = assign24620_body45_e19581_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24620_body46_e19591, assign24620_body46_e19591_d_n0, assign24620_body46_e19591_d_n2, assign24620_body46_e19591_d_n4, assign24620_body46_e19591_d_n5, assign24620_body46_e19591_d_n6, assign24620_body46_e19591_d_n7, assign24620_body46_e19591_d_n8, assign24620_body46_e19591_d_n9, assign24620_body46_e19591_d_n10, assign24620_body46_e19591_d_n11, assign24620_body46_e19591_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body46_e19591;
            locals.var_dnm_dn0 = assign24620_body46_e19591_d_n0;
            locals.var_dnm_dn2 = assign24620_body46_e19591_d_n2;
            locals.var_dnm_dn4 = assign24620_body46_e19591_d_n4;
            locals.var_dnm_dn5 = assign24620_body46_e19591_d_n5;
            locals.var_dnm_dn6 = assign24620_body46_e19591_d_n6;
            locals.var_dnm_dn7 = assign24620_body46_e19591_d_n7;
            locals.var_dnm_dn8 = assign24620_body46_e19591_d_n8;
            locals.var_dnm_dn9 = assign24620_body46_e19591_d_n9;
            locals.var_dnm_dn10 = assign24620_body46_e19591_d_n10;
            locals.var_dnm_dn11 = assign24620_body46_e19591_d_n11;
            locals.var_dnm_dn14 = assign24620_body46_e19591_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body47_e19603, assign24620_body47_e19603_d_n0, assign24620_body47_e19603_d_n2, assign24620_body47_e19603_d_n4, assign24620_body47_e19603_d_n5, assign24620_body47_e19603_d_n6, assign24620_body47_e19603_d_n7, assign24620_body47_e19603_d_n8, assign24620_body47_e19603_d_n9, assign24620_body47_e19603_d_n10, assign24620_body47_e19603_d_n11, assign24620_body47_e19603_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body47_e19601: f64 = (locals.var_xp * locals.var_x2);
        (assign24620_body47_e19601, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body47_e19603;
            locals.var_xp_dn0 = assign24620_body47_e19603_d_n0;
            locals.var_xp_dn2 = assign24620_body47_e19603_d_n2;
            locals.var_xp_dn4 = assign24620_body47_e19603_d_n4;
            locals.var_xp_dn5 = assign24620_body47_e19603_d_n5;
            locals.var_xp_dn6 = assign24620_body47_e19603_d_n6;
            locals.var_xp_dn7 = assign24620_body47_e19603_d_n7;
            locals.var_xp_dn8 = assign24620_body47_e19603_d_n8;
            locals.var_xp_dn9 = assign24620_body47_e19603_d_n9;
            locals.var_xp_dn10 = assign24620_body47_e19603_d_n10;
            locals.var_xp_dn11 = assign24620_body47_e19603_d_n11;
            locals.var_xp_dn14 = assign24620_body47_e19603_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body48_e19615, assign24620_body48_e19615_d_n0, assign24620_body48_e19615_d_n2, assign24620_body48_e19615_d_n4, assign24620_body48_e19615_d_n5, assign24620_body48_e19615_d_n6, assign24620_body48_e19615_d_n7, assign24620_body48_e19615_d_n8, assign24620_body48_e19615_d_n9, assign24620_body48_e19615_d_n10, assign24620_body48_e19615_d_n11, assign24620_body48_e19615_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body48_e19613: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24620_body48_e19613, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body48_e19615;
            locals.var_xmp_dn0 = assign24620_body48_e19615_d_n0;
            locals.var_xmp_dn2 = assign24620_body48_e19615_d_n2;
            locals.var_xmp_dn4 = assign24620_body48_e19615_d_n4;
            locals.var_xmp_dn5 = assign24620_body48_e19615_d_n5;
            locals.var_xmp_dn6 = assign24620_body48_e19615_d_n6;
            locals.var_xmp_dn7 = assign24620_body48_e19615_d_n7;
            locals.var_xmp_dn8 = assign24620_body48_e19615_d_n8;
            locals.var_xmp_dn9 = assign24620_body48_e19615_d_n9;
            locals.var_xmp_dn10 = assign24620_body48_e19615_d_n10;
            locals.var_xmp_dn11 = assign24620_body48_e19615_d_n11;
            locals.var_xmp_dn14 = assign24620_body48_e19615_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body49_e19627, assign24620_body49_e19627_d_n0, assign24620_body49_e19627_d_n2, assign24620_body49_e19627_d_n4, assign24620_body49_e19627_d_n5, assign24620_body49_e19627_d_n6, assign24620_body49_e19627_d_n7, assign24620_body49_e19627_d_n8, assign24620_body49_e19627_d_n9, assign24620_body49_e19627_d_n10, assign24620_body49_e19627_d_n11, assign24620_body49_e19627_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body49_e19625: f64 = (locals.var_xp * locals.var_x2);
        (assign24620_body49_e19625, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign24620_body49_e19627;
            locals.var_xp_dn0 = assign24620_body49_e19627_d_n0;
            locals.var_xp_dn2 = assign24620_body49_e19627_d_n2;
            locals.var_xp_dn4 = assign24620_body49_e19627_d_n4;
            locals.var_xp_dn5 = assign24620_body49_e19627_d_n5;
            locals.var_xp_dn6 = assign24620_body49_e19627_d_n6;
            locals.var_xp_dn7 = assign24620_body49_e19627_d_n7;
            locals.var_xp_dn8 = assign24620_body49_e19627_d_n8;
            locals.var_xp_dn9 = assign24620_body49_e19627_d_n9;
            locals.var_xp_dn10 = assign24620_body49_e19627_d_n10;
            locals.var_xp_dn11 = assign24620_body49_e19627_d_n11;
            locals.var_xp_dn14 = assign24620_body49_e19627_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign24620_body50_e19639, assign24620_body50_e19639_d_n0, assign24620_body50_e19639_d_n2, assign24620_body50_e19639_d_n4, assign24620_body50_e19639_d_n5, assign24620_body50_e19639_d_n6, assign24620_body50_e19639_d_n7, assign24620_body50_e19639_d_n8, assign24620_body50_e19639_d_n9, assign24620_body50_e19639_d_n10, assign24620_body50_e19639_d_n11, assign24620_body50_e19639_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body50_e19637: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24620_body50_e19637, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign24620_body50_e19639;
            locals.var_xmp_dn0 = assign24620_body50_e19639_d_n0;
            locals.var_xmp_dn2 = assign24620_body50_e19639_d_n2;
            locals.var_xmp_dn4 = assign24620_body50_e19639_d_n4;
            locals.var_xmp_dn5 = assign24620_body50_e19639_d_n5;
            locals.var_xmp_dn6 = assign24620_body50_e19639_d_n6;
            locals.var_xmp_dn7 = assign24620_body50_e19639_d_n7;
            locals.var_xmp_dn8 = assign24620_body50_e19639_d_n8;
            locals.var_xmp_dn9 = assign24620_body50_e19639_d_n9;
            locals.var_xmp_dn10 = assign24620_body50_e19639_d_n10;
            locals.var_xmp_dn11 = assign24620_body50_e19639_d_n11;
            locals.var_xmp_dn14 = assign24620_body50_e19639_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign24620_body51_e19651, assign24620_body51_e19651_d_n0, assign24620_body51_e19651_d_n2, assign24620_body51_e19651_d_n4, assign24620_body51_e19651_d_n5, assign24620_body51_e19651_d_n6, assign24620_body51_e19651_d_n7, assign24620_body51_e19651_d_n8, assign24620_body51_e19651_d_n9, assign24620_body51_e19651_d_n10, assign24620_body51_e19651_d_n11, assign24620_body51_e19651_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body51_e19649: f64 = (locals.var_xp + locals.var_xmp);
        (assign24620_body51_e19649, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign24620_body51_e19651;
            locals.var_arg_dn0 = assign24620_body51_e19651_d_n0;
            locals.var_arg_dn2 = assign24620_body51_e19651_d_n2;
            locals.var_arg_dn4 = assign24620_body51_e19651_d_n4;
            locals.var_arg_dn5 = assign24620_body51_e19651_d_n5;
            locals.var_arg_dn6 = assign24620_body51_e19651_d_n6;
            locals.var_arg_dn7 = assign24620_body51_e19651_d_n7;
            locals.var_arg_dn8 = assign24620_body51_e19651_d_n8;
            locals.var_arg_dn9 = assign24620_body51_e19651_d_n9;
            locals.var_arg_dn10 = assign24620_body51_e19651_d_n10;
            locals.var_arg_dn11 = assign24620_body51_e19651_d_n11;
            locals.var_arg_dn14 = assign24620_body51_e19651_d_n14;
            locals.var_arg_rv = 0.0;
            let (assign24620_body52_e19661, assign24620_body52_e19661_d_n0, assign24620_body52_e19661_d_n2, assign24620_body52_e19661_d_n4, assign24620_body52_e19661_d_n5, assign24620_body52_e19661_d_n6, assign24620_body52_e19661_d_n7, assign24620_body52_e19661_d_n8, assign24620_body52_e19661_d_n9, assign24620_body52_e19661_d_n10, assign24620_body52_e19661_d_n11, assign24620_body52_e19661_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body52_e19661;
            locals.var_dnm_dn0 = assign24620_body52_e19661_d_n0;
            locals.var_dnm_dn2 = assign24620_body52_e19661_d_n2;
            locals.var_dnm_dn4 = assign24620_body52_e19661_d_n4;
            locals.var_dnm_dn5 = assign24620_body52_e19661_d_n5;
            locals.var_dnm_dn6 = assign24620_body52_e19661_d_n6;
            locals.var_dnm_dn7 = assign24620_body52_e19661_d_n7;
            locals.var_dnm_dn8 = assign24620_body52_e19661_d_n8;
            locals.var_dnm_dn9 = assign24620_body52_e19661_d_n9;
            locals.var_dnm_dn10 = assign24620_body52_e19661_d_n10;
            locals.var_dnm_dn11 = assign24620_body52_e19661_d_n11;
            locals.var_dnm_dn14 = assign24620_body52_e19661_d_n14;
            locals.var_dnm_rv = 0.0;
            let assign24620_body53_e19676: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard569 = assign24620_body53_e19676;
            locals.var_guard569_rv = 0.0;
            let assign24620_body54_e19679: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard570 = assign24620_body54_e19679;
            locals.var_guard570_rv = 0.0;
            let (assign24620_body55_e19693,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body55_e19693;
            locals.var_mm_rv = 0.0;
            let assign24620_body56_e19696: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard571 = assign24620_body56_e19696;
            locals.var_guard571_rv = 0.0;
            let (assign24620_body57_e19713,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body57_e19713;
            locals.var_mm_rv = 0.0;
            let assign24620_body58_e19716: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard572 = assign24620_body58_e19716;
            locals.var_guard572_rv = 0.0;
            let (assign24620_body59_e19736,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 == 0.0)) && (locals.var_guard572 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body59_e19736;
            locals.var_mm_rv = 0.0;
            let assign24620_body60_e19739: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard573 = assign24620_body60_e19739;
            locals.var_guard573_rv = 0.0;
            let (assign24620_body61_e19762,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 == 0.0)) && (locals.var_guard572 == 0.0)) && (locals.var_guard573 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24620_body61_e19762;
            locals.var_mm_rv = 0.0;
            let (assign24620_body62_e19774,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24620_body62_e19774;
            locals.var_m0_rv = 0.0;
            let mut assign24620_body63_loop_guard: usize = 0;
            while {
                let assign24620_body63_cond_e19787: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24620_body63_cond_e19787 != 0.0
            } {
                assign24620_body63_loop_guard += 1;
                assert!(assign24620_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24620_body63_body0_e19800, assign24620_body63_body0_e19800_d_n0, assign24620_body63_body0_e19800_d_n2, assign24620_body63_body0_e19800_d_n4, assign24620_body63_body0_e19800_d_n5, assign24620_body63_body0_e19800_d_n6, assign24620_body63_body0_e19800_d_n7, assign24620_body63_body0_e19800_d_n8, assign24620_body63_body0_e19800_d_n9, assign24620_body63_body0_e19800_d_n10, assign24620_body63_body0_e19800_d_n11, assign24620_body63_body0_e19800_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        let assign24620_body63_body0_e19798: f64 = (locals.var_dnm).sqrt();
        (assign24620_body63_body0_e19798, (locals.var_dnm_dn0 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn2 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn4 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn5 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn6 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn7 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn8 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn9 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn10 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn11 / (2.0 * assign24620_body63_body0_e19798)), (locals.var_dnm_dn14 / (2.0 * assign24620_body63_body0_e19798)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign24620_body63_body0_e19800;
                locals.var_dnm_dn0 = assign24620_body63_body0_e19800_d_n0;
                locals.var_dnm_dn2 = assign24620_body63_body0_e19800_d_n2;
                locals.var_dnm_dn4 = assign24620_body63_body0_e19800_d_n4;
                locals.var_dnm_dn5 = assign24620_body63_body0_e19800_d_n5;
                locals.var_dnm_dn6 = assign24620_body63_body0_e19800_d_n6;
                locals.var_dnm_dn7 = assign24620_body63_body0_e19800_d_n7;
                locals.var_dnm_dn8 = assign24620_body63_body0_e19800_d_n8;
                locals.var_dnm_dn9 = assign24620_body63_body0_e19800_d_n9;
                locals.var_dnm_dn10 = assign24620_body63_body0_e19800_d_n10;
                locals.var_dnm_dn11 = assign24620_body63_body0_e19800_d_n11;
                locals.var_dnm_dn14 = assign24620_body63_body0_e19800_d_n14;
                locals.var_dnm_rv = 0.0;
                let (assign24620_body63_body1_e19814,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        let assign24620_body63_body1_e19812: f64 = (locals.var_m0 + 1.0);
        (assign24620_body63_body1_e19812,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24620_body63_body1_e19814;
                locals.var_m0_rv = 0.0;
            }
            let (assign24620_body64_e19838, assign24620_body64_e19838_d_n0, assign24620_body64_e19838_d_n2, assign24620_body64_e19838_d_n4, assign24620_body64_e19838_d_n5, assign24620_body64_e19838_d_n6, assign24620_body64_e19838_d_n7, assign24620_body64_e19838_d_n8, assign24620_body64_e19838_d_n9, assign24620_body64_e19838_d_n10, assign24620_body64_e19838_d_n11, assign24620_body64_e19838_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 == 0.0)) {
        let (assign24620_body64_e19836, assign24620_body64_e19836_d_n0, assign24620_body64_e19836_d_n2, assign24620_body64_e19836_d_n4, assign24620_body64_e19836_d_n5, assign24620_body64_e19836_d_n6, assign24620_body64_e19836_d_n7, assign24620_body64_e19836_d_n8, assign24620_body64_e19836_d_n9, assign24620_body64_e19836_d_n10, assign24620_body64_e19836_d_n11, assign24620_body64_e19836_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24620_body64_e19833: f64 = (2.0 * 2.0);
                let assign24620_body64_e19834: f64 = (1.0 / assign24620_body64_e19833);
                let assign24620_body64_e19835: f64 = (locals.var_dnm).powf(assign24620_body64_e19834);
                (assign24620_body64_e19835, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn11)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24620_body64_e19834) as f64).is_finite() && ((assign24620_body64_e19834) as f64).fract() == 0.0 { if assign24620_body64_e19834 == 0.0 { 0.0 } else { (assign24620_body64_e19834 * ((locals.var_dnm).powf(assign24620_body64_e19834 - 1.0) * locals.var_dnm_dn14)) } } else { (assign24620_body64_e19835 * (assign24620_body64_e19834 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign24620_body64_e19836, assign24620_body64_e19836_d_n0, assign24620_body64_e19836_d_n2, assign24620_body64_e19836_d_n4, assign24620_body64_e19836_d_n5, assign24620_body64_e19836_d_n6, assign24620_body64_e19836_d_n7, assign24620_body64_e19836_d_n8, assign24620_body64_e19836_d_n9, assign24620_body64_e19836_d_n10, assign24620_body64_e19836_d_n11, assign24620_body64_e19836_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body64_e19838;
            locals.var_dnm_dn0 = assign24620_body64_e19838_d_n0;
            locals.var_dnm_dn2 = assign24620_body64_e19838_d_n2;
            locals.var_dnm_dn4 = assign24620_body64_e19838_d_n4;
            locals.var_dnm_dn5 = assign24620_body64_e19838_d_n5;
            locals.var_dnm_dn6 = assign24620_body64_e19838_d_n6;
            locals.var_dnm_dn7 = assign24620_body64_e19838_d_n7;
            locals.var_dnm_dn8 = assign24620_body64_e19838_d_n8;
            locals.var_dnm_dn9 = assign24620_body64_e19838_d_n9;
            locals.var_dnm_dn10 = assign24620_body64_e19838_d_n10;
            locals.var_dnm_dn11 = assign24620_body64_e19838_d_n11;
            locals.var_dnm_dn14 = assign24620_body64_e19838_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body65_e19850, assign24620_body65_e19850_d_n0, assign24620_body65_e19850_d_n2, assign24620_body65_e19850_d_n4, assign24620_body65_e19850_d_n5, assign24620_body65_e19850_d_n6, assign24620_body65_e19850_d_n7, assign24620_body65_e19850_d_n8, assign24620_body65_e19850_d_n9, assign24620_body65_e19850_d_n10, assign24620_body65_e19850_d_n11, assign24620_body65_e19850_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body65_e19848: f64 = (1.0 / locals.var_dnm);
        (assign24620_body65_e19848, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign24620_body65_e19850;
            locals.var_dnm_dn0 = assign24620_body65_e19850_d_n0;
            locals.var_dnm_dn2 = assign24620_body65_e19850_d_n2;
            locals.var_dnm_dn4 = assign24620_body65_e19850_d_n4;
            locals.var_dnm_dn5 = assign24620_body65_e19850_d_n5;
            locals.var_dnm_dn6 = assign24620_body65_e19850_d_n6;
            locals.var_dnm_dn7 = assign24620_body65_e19850_d_n7;
            locals.var_dnm_dn8 = assign24620_body65_e19850_d_n8;
            locals.var_dnm_dn9 = assign24620_body65_e19850_d_n9;
            locals.var_dnm_dn10 = assign24620_body65_e19850_d_n10;
            locals.var_dnm_dn11 = assign24620_body65_e19850_d_n11;
            locals.var_dnm_dn14 = assign24620_body65_e19850_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign24620_body66_e19864, assign24620_body66_e19864_d_n0, assign24620_body66_e19864_d_n2, assign24620_body66_e19864_d_n4, assign24620_body66_e19864_d_n5, assign24620_body66_e19864_d_n6, assign24620_body66_e19864_d_n7, assign24620_body66_e19864_d_n8, assign24620_body66_e19864_d_n9, assign24620_body66_e19864_d_n10, assign24620_body66_e19864_d_n11, assign24620_body66_e19864_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body66_e19860: f64 = (locals.var_tmf1 * 0.1);
        let assign24620_body66_e19862: f64 = (assign24620_body66_e19860 * locals.var_dnm);
        (assign24620_body66_e19862, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign24620_body66_e19860 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign24620_body66_e19864;
            locals.var_tmf0_dn0 = assign24620_body66_e19864_d_n0;
            locals.var_tmf0_dn2 = assign24620_body66_e19864_d_n2;
            locals.var_tmf0_dn4 = assign24620_body66_e19864_d_n4;
            locals.var_tmf0_dn5 = assign24620_body66_e19864_d_n5;
            locals.var_tmf0_dn6 = assign24620_body66_e19864_d_n6;
            locals.var_tmf0_dn7 = assign24620_body66_e19864_d_n7;
            locals.var_tmf0_dn8 = assign24620_body66_e19864_d_n8;
            locals.var_tmf0_dn9 = assign24620_body66_e19864_d_n9;
            locals.var_tmf0_dn10 = assign24620_body66_e19864_d_n10;
            locals.var_tmf0_dn11 = assign24620_body66_e19864_d_n11;
            locals.var_tmf0_dn14 = assign24620_body66_e19864_d_n14;
            locals.var_tmf0_rv = 0.0;
            let (assign24620_body67_e19880, assign24620_body67_e19880_d_n0, assign24620_body67_e19880_d_n2, assign24620_body67_e19880_d_n4, assign24620_body67_e19880_d_n5, assign24620_body67_e19880_d_n6, assign24620_body67_e19880_d_n7, assign24620_body67_e19880_d_n8, assign24620_body67_e19880_d_n9, assign24620_body67_e19880_d_n10, assign24620_body67_e19880_d_n11, assign24620_body67_e19880_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body67_e19874: f64 = (0.1 * locals.var_xmp);
        let assign24620_body67_e19876: f64 = (assign24620_body67_e19874 * locals.var_dnm);
        let assign24620_body67_e19878: f64 = (assign24620_body67_e19876 / locals.var_arg);
        (assign24620_body67_e19878, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn11)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign24620_body67_e19874 * locals.var_dnm_dn14)) * locals.var_arg) - (assign24620_body67_e19876 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24620_body67_e19880;
            locals.var_t7_dn0 = assign24620_body67_e19880_d_n0;
            locals.var_t7_dn2 = assign24620_body67_e19880_d_n2;
            locals.var_t7_dn4 = assign24620_body67_e19880_d_n4;
            locals.var_t7_dn5 = assign24620_body67_e19880_d_n5;
            locals.var_t7_dn6 = assign24620_body67_e19880_d_n6;
            locals.var_t7_dn7 = assign24620_body67_e19880_d_n7;
            locals.var_t7_dn8 = assign24620_body67_e19880_d_n8;
            locals.var_t7_dn9 = assign24620_body67_e19880_d_n9;
            locals.var_t7_dn10 = assign24620_body67_e19880_d_n10;
            locals.var_t7_dn11 = assign24620_body67_e19880_d_n11;
            locals.var_t7_dn14 = assign24620_body67_e19880_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24620_body68_e19894, assign24620_body68_e19894_d_n0, assign24620_body68_e19894_d_n2, assign24620_body68_e19894_d_n4, assign24620_body68_e19894_d_n5, assign24620_body68_e19894_d_n6, assign24620_body68_e19894_d_n7, assign24620_body68_e19894_d_n8, assign24620_body68_e19894_d_n9, assign24620_body68_e19894_d_n10, assign24620_body68_e19894_d_n11, assign24620_body68_e19894_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign24620_body68_e19890: f64 = 0.1;
        let assign24620_body68_e19892: f64 = (assign24620_body68_e19890 - locals.var_tmf0);
        (assign24620_body68_e19892, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign24620_body68_e19894;
            locals.var_t2_dn0 = assign24620_body68_e19894_d_n0;
            locals.var_t2_dn2 = assign24620_body68_e19894_d_n2;
            locals.var_t2_dn4 = assign24620_body68_e19894_d_n4;
            locals.var_t2_dn5 = assign24620_body68_e19894_d_n5;
            locals.var_t2_dn6 = assign24620_body68_e19894_d_n6;
            locals.var_t2_dn7 = assign24620_body68_e19894_d_n7;
            locals.var_t2_dn8 = assign24620_body68_e19894_d_n8;
            locals.var_t2_dn9 = assign24620_body68_e19894_d_n9;
            locals.var_t2_dn10 = assign24620_body68_e19894_d_n10;
            locals.var_t2_dn11 = assign24620_body68_e19894_d_n11;
            locals.var_t2_dn14 = assign24620_body68_e19894_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign24620_body69_e19904, assign24620_body69_e19904_d_n0, assign24620_body69_e19904_d_n2, assign24620_body69_e19904_d_n4, assign24620_body69_e19904_d_n5, assign24620_body69_e19904_d_n6, assign24620_body69_e19904_d_n7, assign24620_body69_e19904_d_n8, assign24620_body69_e19904_d_n9, assign24620_body69_e19904_d_n10, assign24620_body69_e19904_d_n11, assign24620_body69_e19904_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24620_body69_e19904;
            locals.var_t7_dn0 = assign24620_body69_e19904_d_n0;
            locals.var_t7_dn2 = assign24620_body69_e19904_d_n2;
            locals.var_t7_dn4 = assign24620_body69_e19904_d_n4;
            locals.var_t7_dn5 = assign24620_body69_e19904_d_n5;
            locals.var_t7_dn6 = assign24620_body69_e19904_d_n6;
            locals.var_t7_dn7 = assign24620_body69_e19904_d_n7;
            locals.var_t7_dn8 = assign24620_body69_e19904_d_n8;
            locals.var_t7_dn9 = assign24620_body69_e19904_d_n9;
            locals.var_t7_dn10 = assign24620_body69_e19904_d_n10;
            locals.var_t7_dn11 = assign24620_body69_e19904_d_n11;
            locals.var_t7_dn14 = assign24620_body69_e19904_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24620_body70_e19915, assign24620_body70_e19915_d_n0, assign24620_body70_e19915_d_n2, assign24620_body70_e19915_d_n4, assign24620_body70_e19915_d_n5, assign24620_body70_e19915_d_n6, assign24620_body70_e19915_d_n7, assign24620_body70_e19915_d_n8, assign24620_body70_e19915_d_n9, assign24620_body70_e19915_d_n10, assign24620_body70_e19915_d_n11, assign24620_body70_e19915_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign24620_body70_e19915;
            locals.var_t2_dn0 = assign24620_body70_e19915_d_n0;
            locals.var_t2_dn2 = assign24620_body70_e19915_d_n2;
            locals.var_t2_dn4 = assign24620_body70_e19915_d_n4;
            locals.var_t2_dn5 = assign24620_body70_e19915_d_n5;
            locals.var_t2_dn6 = assign24620_body70_e19915_d_n6;
            locals.var_t2_dn7 = assign24620_body70_e19915_d_n7;
            locals.var_t2_dn8 = assign24620_body70_e19915_d_n8;
            locals.var_t2_dn9 = assign24620_body70_e19915_d_n9;
            locals.var_t2_dn10 = assign24620_body70_e19915_d_n10;
            locals.var_t2_dn11 = assign24620_body70_e19915_d_n11;
            locals.var_t2_dn14 = assign24620_body70_e19915_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign24620_body71_e19926, assign24620_body71_e19926_d_n0, assign24620_body71_e19926_d_n2, assign24620_body71_e19926_d_n4, assign24620_body71_e19926_d_n5, assign24620_body71_e19926_d_n6, assign24620_body71_e19926_d_n7, assign24620_body71_e19926_d_n8, assign24620_body71_e19926_d_n9, assign24620_body71_e19926_d_n10, assign24620_body71_e19926_d_n11, assign24620_body71_e19926_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard568 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign24620_body71_e19926;
            locals.var_t7_dn0 = assign24620_body71_e19926_d_n0;
            locals.var_t7_dn2 = assign24620_body71_e19926_d_n2;
            locals.var_t7_dn4 = assign24620_body71_e19926_d_n4;
            locals.var_t7_dn5 = assign24620_body71_e19926_d_n5;
            locals.var_t7_dn6 = assign24620_body71_e19926_d_n6;
            locals.var_t7_dn7 = assign24620_body71_e19926_d_n7;
            locals.var_t7_dn8 = assign24620_body71_e19926_d_n8;
            locals.var_t7_dn9 = assign24620_body71_e19926_d_n9;
            locals.var_t7_dn10 = assign24620_body71_e19926_d_n10;
            locals.var_t7_dn11 = assign24620_body71_e19926_d_n11;
            locals.var_t7_dn14 = assign24620_body71_e19926_d_n14;
            locals.var_t7_rv = 0.0;
            let (assign24620_body72_e19937, assign24620_body72_e19937_d_n0, assign24620_body72_e19937_d_n2, assign24620_body72_e19937_d_n4, assign24620_body72_e19937_d_n5, assign24620_body72_e19937_d_n6, assign24620_body72_e19937_d_n7, assign24620_body72_e19937_d_n8, assign24620_body72_e19937_d_n9, assign24620_body72_e19937_d_n10, assign24620_body72_e19937_d_n11, assign24620_body72_e19937_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body72_e19934: f64 = (locals.var_c_2esipq_nsub * locals.var_t2);
        let assign24620_body72_e19935: f64 = (assign24620_body72_e19934).sqrt();
        (assign24620_body72_e19935, (((locals.var_c_2esipq_nsub_dn0 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn0)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn2 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn2)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn4 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn4)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn5 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn5)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn6 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn6)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn7 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn7)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn8 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn8)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn9 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn9)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn10 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn10)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn11 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn11)) / (2.0 * assign24620_body72_e19935)), (((locals.var_c_2esipq_nsub_dn14 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn14)) / (2.0 * assign24620_body72_e19935)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
            locals.var_w_sub0 = assign24620_body72_e19937;
            locals.var_w_sub0_dn0 = assign24620_body72_e19937_d_n0;
            locals.var_w_sub0_dn2 = assign24620_body72_e19937_d_n2;
            locals.var_w_sub0_dn4 = assign24620_body72_e19937_d_n4;
            locals.var_w_sub0_dn5 = assign24620_body72_e19937_d_n5;
            locals.var_w_sub0_dn6 = assign24620_body72_e19937_d_n6;
            locals.var_w_sub0_dn7 = assign24620_body72_e19937_d_n7;
            locals.var_w_sub0_dn8 = assign24620_body72_e19937_d_n8;
            locals.var_w_sub0_dn9 = assign24620_body72_e19937_d_n9;
            locals.var_w_sub0_dn10 = assign24620_body72_e19937_d_n10;
            locals.var_w_sub0_dn11 = assign24620_body72_e19937_d_n11;
            locals.var_w_sub0_dn14 = assign24620_body72_e19937_d_n14;
            locals.var_w_sub0_rv = 0.0;
            let (assign24620_body73_e19947, assign24620_body73_e19947_d_n0, assign24620_body73_e19947_d_n2, assign24620_body73_e19947_d_n4, assign24620_body73_e19947_d_n5, assign24620_body73_e19947_d_n6, assign24620_body73_e19947_d_n7, assign24620_body73_e19947_d_n8, assign24620_body73_e19947_d_n9, assign24620_body73_e19947_d_n10, assign24620_body73_e19947_d_n11, assign24620_body73_e19947_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body73_e19945: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign24620_body73_e19945, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
            locals.var_q_b0_dep = assign24620_body73_e19947;
            locals.var_q_b0_dep_dn0 = assign24620_body73_e19947_d_n0;
            locals.var_q_b0_dep_dn2 = assign24620_body73_e19947_d_n2;
            locals.var_q_b0_dep_dn4 = assign24620_body73_e19947_d_n4;
            locals.var_q_b0_dep_dn5 = assign24620_body73_e19947_d_n5;
            locals.var_q_b0_dep_dn6 = assign24620_body73_e19947_d_n6;
            locals.var_q_b0_dep_dn7 = assign24620_body73_e19947_d_n7;
            locals.var_q_b0_dep_dn8 = assign24620_body73_e19947_d_n8;
            locals.var_q_b0_dep_dn9 = assign24620_body73_e19947_d_n9;
            locals.var_q_b0_dep_dn10 = assign24620_body73_e19947_d_n10;
            locals.var_q_b0_dep_dn11 = assign24620_body73_e19947_d_n11;
            locals.var_q_b0_dep_dn14 = assign24620_body73_e19947_d_n14;
            locals.var_q_b0_dep_rv = 0.0;
            let (assign24620_body74_e19960, assign24620_body74_e19960_d_n0, assign24620_body74_e19960_d_n2, assign24620_body74_e19960_d_n4, assign24620_body74_e19960_d_n5, assign24620_body74_e19960_d_n6, assign24620_body74_e19960_d_n7, assign24620_body74_e19960_d_n8, assign24620_body74_e19960_d_n9, assign24620_body74_e19960_d_n10, assign24620_body74_e19960_d_n11, assign24620_body74_e19960_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body74_e19954: f64 = (-1.034943e-10);
        let assign24620_body74_e19956: f64 = (assign24620_body74_e19954 / locals.var_w_b0);
        let assign24620_body74_e19958: f64 = (assign24620_body74_e19956 * locals.var_t0);
        (assign24620_body74_e19958, (((-((assign24620_body74_e19954 * locals.var_w_b0_dn0) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn0)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn2) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn2)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn4) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn4)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn5) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn5)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn6) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn6)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn7) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn7)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn8) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn8)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn9) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn9)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn10) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn10)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn11) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn11)), (((-((assign24620_body74_e19954 * locals.var_w_b0_dn14) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24620_body74_e19956 * locals.var_t0_dn14)),)
    } else {
        (locals.var_q_b0_dep_dpd, locals.var_q_b0_dep_dpd_dn0, locals.var_q_b0_dep_dpd_dn2, locals.var_q_b0_dep_dpd_dn4, locals.var_q_b0_dep_dpd_dn5, locals.var_q_b0_dep_dpd_dn6, locals.var_q_b0_dep_dpd_dn7, locals.var_q_b0_dep_dpd_dn8, locals.var_q_b0_dep_dpd_dn9, locals.var_q_b0_dep_dpd_dn10, locals.var_q_b0_dep_dpd_dn11, locals.var_q_b0_dep_dpd_dn14,)
    }
};
            locals.var_q_b0_dep_dpd = assign24620_body74_e19960;
            locals.var_q_b0_dep_dpd_dn0 = assign24620_body74_e19960_d_n0;
            locals.var_q_b0_dep_dpd_dn2 = assign24620_body74_e19960_d_n2;
            locals.var_q_b0_dep_dpd_dn4 = assign24620_body74_e19960_d_n4;
            locals.var_q_b0_dep_dpd_dn5 = assign24620_body74_e19960_d_n5;
            locals.var_q_b0_dep_dpd_dn6 = assign24620_body74_e19960_d_n6;
            locals.var_q_b0_dep_dpd_dn7 = assign24620_body74_e19960_d_n7;
            locals.var_q_b0_dep_dpd_dn8 = assign24620_body74_e19960_d_n8;
            locals.var_q_b0_dep_dpd_dn9 = assign24620_body74_e19960_d_n9;
            locals.var_q_b0_dep_dpd_dn10 = assign24620_body74_e19960_d_n10;
            locals.var_q_b0_dep_dpd_dn11 = assign24620_body74_e19960_d_n11;
            locals.var_q_b0_dep_dpd_dn14 = assign24620_body74_e19960_d_n14;
            locals.var_q_b0_dep_dpd_rv = 0.0;
            let (assign24620_body75_e19971, assign24620_body75_e19971_d_n0, assign24620_body75_e19971_d_n2, assign24620_body75_e19971_d_n4, assign24620_body75_e19971_d_n5, assign24620_body75_e19971_d_n6, assign24620_body75_e19971_d_n7, assign24620_body75_e19971_d_n8, assign24620_body75_e19971_d_n9, assign24620_body75_e19971_d_n10, assign24620_body75_e19971_d_n11, assign24620_body75_e19971_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body75_e19967: f64 = (-locals.var_w_sub0);
        let assign24620_body75_e19969: f64 = (assign24620_body75_e19967 * locals.var_q_nsub__blk546);
        (assign24620_body75_e19969, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk546) + (assign24620_body75_e19967 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
            locals.var_q_sub0_dep = assign24620_body75_e19971;
            locals.var_q_sub0_dep_dn0 = assign24620_body75_e19971_d_n0;
            locals.var_q_sub0_dep_dn2 = assign24620_body75_e19971_d_n2;
            locals.var_q_sub0_dep_dn4 = assign24620_body75_e19971_d_n4;
            locals.var_q_sub0_dep_dn5 = assign24620_body75_e19971_d_n5;
            locals.var_q_sub0_dep_dn6 = assign24620_body75_e19971_d_n6;
            locals.var_q_sub0_dep_dn7 = assign24620_body75_e19971_d_n7;
            locals.var_q_sub0_dep_dn8 = assign24620_body75_e19971_d_n8;
            locals.var_q_sub0_dep_dn9 = assign24620_body75_e19971_d_n9;
            locals.var_q_sub0_dep_dn10 = assign24620_body75_e19971_d_n10;
            locals.var_q_sub0_dep_dn11 = assign24620_body75_e19971_d_n11;
            locals.var_q_sub0_dep_dn14 = assign24620_body75_e19971_d_n14;
            locals.var_q_sub0_dep_rv = 0.0;
            let (assign24620_body76_e19984, assign24620_body76_e19984_d_n0, assign24620_body76_e19984_d_n2, assign24620_body76_e19984_d_n4, assign24620_body76_e19984_d_n5, assign24620_body76_e19984_d_n6, assign24620_body76_e19984_d_n7, assign24620_body76_e19984_d_n8, assign24620_body76_e19984_d_n9, assign24620_body76_e19984_d_n10, assign24620_body76_e19984_d_n11, assign24620_body76_e19984_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body76_e19978: f64 = (-1.034943e-10);
        let assign24620_body76_e19980: f64 = (assign24620_body76_e19978 / locals.var_w_sub0);
        let assign24620_body76_e19982: f64 = (assign24620_body76_e19980 * locals.var_t7);
        (assign24620_body76_e19982, (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn0) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn0)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn2) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn2)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn4) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn4)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn5) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn5)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn6) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn6)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn7) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn7)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn8) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn8)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn9) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn9)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn10) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn10)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn11) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn11)), (((-((assign24620_body76_e19978 * locals.var_w_sub0_dn14) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24620_body76_e19980 * locals.var_t7_dn14)),)
    } else {
        (locals.var_q_sub0_dep_dpd, locals.var_q_sub0_dep_dpd_dn0, locals.var_q_sub0_dep_dpd_dn2, locals.var_q_sub0_dep_dpd_dn4, locals.var_q_sub0_dep_dpd_dn5, locals.var_q_sub0_dep_dpd_dn6, locals.var_q_sub0_dep_dpd_dn7, locals.var_q_sub0_dep_dpd_dn8, locals.var_q_sub0_dep_dpd_dn9, locals.var_q_sub0_dep_dpd_dn10, locals.var_q_sub0_dep_dpd_dn11, locals.var_q_sub0_dep_dpd_dn14,)
    }
};
            locals.var_q_sub0_dep_dpd = assign24620_body76_e19984;
            locals.var_q_sub0_dep_dpd_dn0 = assign24620_body76_e19984_d_n0;
            locals.var_q_sub0_dep_dpd_dn2 = assign24620_body76_e19984_d_n2;
            locals.var_q_sub0_dep_dpd_dn4 = assign24620_body76_e19984_d_n4;
            locals.var_q_sub0_dep_dpd_dn5 = assign24620_body76_e19984_d_n5;
            locals.var_q_sub0_dep_dpd_dn6 = assign24620_body76_e19984_d_n6;
            locals.var_q_sub0_dep_dpd_dn7 = assign24620_body76_e19984_d_n7;
            locals.var_q_sub0_dep_dpd_dn8 = assign24620_body76_e19984_d_n8;
            locals.var_q_sub0_dep_dpd_dn9 = assign24620_body76_e19984_d_n9;
            locals.var_q_sub0_dep_dpd_dn10 = assign24620_body76_e19984_d_n10;
            locals.var_q_sub0_dep_dpd_dn11 = assign24620_body76_e19984_d_n11;
            locals.var_q_sub0_dep_dpd_dn14 = assign24620_body76_e19984_d_n14;
            locals.var_q_sub0_dep_dpd_rv = 0.0;
            let (assign24620_body77_e20000, assign24620_body77_e20000_d_n0, assign24620_body77_e20000_d_n2, assign24620_body77_e20000_d_n4, assign24620_body77_e20000_d_n5, assign24620_body77_e20000_d_n6, assign24620_body77_e20000_d_n7, assign24620_body77_e20000_d_n8, assign24620_body77_e20000_d_n9, assign24620_body77_e20000_d_n10, assign24620_body77_e20000_d_n11, assign24620_body77_e20000_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body77_e19993: f64 = (locals.var_vgp0 - locals.var_phi_b0_dep);
        let assign24620_body77_e19994: f64 = (locals.var_cox * assign24620_body77_e19993);
        let assign24620_body77_e19996: f64 = (assign24620_body77_e19994 + locals.var_q_b0_dep);
        let assign24620_body77_e19998: f64 = (assign24620_body77_e19996 + locals.var_q_sub0_dep);
        (assign24620_body77_e19998, ((((locals.var_cox_dn0 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn0 - locals.var_phi_b0_dep_dn0))) + locals.var_q_b0_dep_dn0) + locals.var_q_sub0_dep_dn0), ((((locals.var_cox_dn2 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn2 - locals.var_phi_b0_dep_dn2))) + locals.var_q_b0_dep_dn2) + locals.var_q_sub0_dep_dn2), ((((locals.var_cox_dn4 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn4 - locals.var_phi_b0_dep_dn4))) + locals.var_q_b0_dep_dn4) + locals.var_q_sub0_dep_dn4), ((((locals.var_cox_dn5 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn5 - locals.var_phi_b0_dep_dn5))) + locals.var_q_b0_dep_dn5) + locals.var_q_sub0_dep_dn5), ((((locals.var_cox_dn6 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn6 - locals.var_phi_b0_dep_dn6))) + locals.var_q_b0_dep_dn6) + locals.var_q_sub0_dep_dn6), ((((locals.var_cox_dn7 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn7 - locals.var_phi_b0_dep_dn7))) + locals.var_q_b0_dep_dn7) + locals.var_q_sub0_dep_dn7), ((((locals.var_cox_dn8 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn8 - locals.var_phi_b0_dep_dn8))) + locals.var_q_b0_dep_dn8) + locals.var_q_sub0_dep_dn8), ((((locals.var_cox_dn9 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn9 - locals.var_phi_b0_dep_dn9))) + locals.var_q_b0_dep_dn9) + locals.var_q_sub0_dep_dn9), ((((locals.var_cox_dn10 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn10 - locals.var_phi_b0_dep_dn10))) + locals.var_q_b0_dep_dn10) + locals.var_q_sub0_dep_dn10), ((((locals.var_cox_dn11 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn11 - locals.var_phi_b0_dep_dn11))) + locals.var_q_b0_dep_dn11) + locals.var_q_sub0_dep_dn11), ((((locals.var_cox_dn14 * assign24620_body77_e19993) + (locals.var_cox * (locals.var_vgp0_dn14 - locals.var_phi_b0_dep_dn14))) + locals.var_q_b0_dep_dn14) + locals.var_q_sub0_dep_dn14),)
    } else {
        (locals.var_y1, locals.var_y1_dn0, locals.var_y1_dn2, locals.var_y1_dn4, locals.var_y1_dn5, locals.var_y1_dn6, locals.var_y1_dn7, locals.var_y1_dn8, locals.var_y1_dn9, locals.var_y1_dn10, locals.var_y1_dn11, locals.var_y1_dn14,)
    }
};
            locals.var_y1 = assign24620_body77_e20000;
            locals.var_y1_dn0 = assign24620_body77_e20000_d_n0;
            locals.var_y1_dn2 = assign24620_body77_e20000_d_n2;
            locals.var_y1_dn4 = assign24620_body77_e20000_d_n4;
            locals.var_y1_dn5 = assign24620_body77_e20000_d_n5;
            locals.var_y1_dn6 = assign24620_body77_e20000_d_n6;
            locals.var_y1_dn7 = assign24620_body77_e20000_d_n7;
            locals.var_y1_dn8 = assign24620_body77_e20000_d_n8;
            locals.var_y1_dn9 = assign24620_body77_e20000_d_n9;
            locals.var_y1_dn10 = assign24620_body77_e20000_d_n10;
            locals.var_y1_dn11 = assign24620_body77_e20000_d_n11;
            locals.var_y1_dn14 = assign24620_body77_e20000_d_n14;
            locals.var_y1_rv = 0.0;
            let (assign24620_body78_e20008, assign24620_body78_e20008_d_n0, assign24620_body78_e20008_d_n2, assign24620_body78_e20008_d_n4, assign24620_body78_e20008_d_n5, assign24620_body78_e20008_d_n6, assign24620_body78_e20008_d_n7, assign24620_body78_e20008_d_n8, assign24620_body78_e20008_d_n9, assign24620_body78_e20008_d_n10, assign24620_body78_e20008_d_n11, assign24620_body78_e20008_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    } else {
        (locals.var_y11, locals.var_y11_dn0, locals.var_y11_dn2, locals.var_y11_dn4, locals.var_y11_dn5, locals.var_y11_dn6, locals.var_y11_dn7, locals.var_y11_dn8, locals.var_y11_dn9, locals.var_y11_dn10, locals.var_y11_dn11, locals.var_y11_dn14,)
    }
};
            locals.var_y11 = assign24620_body78_e20008;
            locals.var_y11_dn0 = assign24620_body78_e20008_d_n0;
            locals.var_y11_dn2 = assign24620_body78_e20008_d_n2;
            locals.var_y11_dn4 = assign24620_body78_e20008_d_n4;
            locals.var_y11_dn5 = assign24620_body78_e20008_d_n5;
            locals.var_y11_dn6 = assign24620_body78_e20008_d_n6;
            locals.var_y11_dn7 = assign24620_body78_e20008_d_n7;
            locals.var_y11_dn8 = assign24620_body78_e20008_d_n8;
            locals.var_y11_dn9 = assign24620_body78_e20008_d_n9;
            locals.var_y11_dn10 = assign24620_body78_e20008_d_n10;
            locals.var_y11_dn11 = assign24620_body78_e20008_d_n11;
            locals.var_y11_dn14 = assign24620_body78_e20008_d_n14;
            locals.var_y11_rv = 0.0;
            let (assign24620_body79_e20018, assign24620_body79_e20018_d_n0, assign24620_body79_e20018_d_n2, assign24620_body79_e20018_d_n4, assign24620_body79_e20018_d_n5, assign24620_body79_e20018_d_n6, assign24620_body79_e20018_d_n7, assign24620_body79_e20018_d_n8, assign24620_body79_e20018_d_n9, assign24620_body79_e20018_d_n10, assign24620_body79_e20018_d_n11, assign24620_body79_e20018_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body79_e20016: f64 = (locals.var_q_b0_dep_dpd + locals.var_q_sub0_dep_dpd);
        (assign24620_body79_e20016, (locals.var_q_b0_dep_dpd_dn0 + locals.var_q_sub0_dep_dpd_dn0), (locals.var_q_b0_dep_dpd_dn2 + locals.var_q_sub0_dep_dpd_dn2), (locals.var_q_b0_dep_dpd_dn4 + locals.var_q_sub0_dep_dpd_dn4), (locals.var_q_b0_dep_dpd_dn5 + locals.var_q_sub0_dep_dpd_dn5), (locals.var_q_b0_dep_dpd_dn6 + locals.var_q_sub0_dep_dpd_dn6), (locals.var_q_b0_dep_dpd_dn7 + locals.var_q_sub0_dep_dpd_dn7), (locals.var_q_b0_dep_dpd_dn8 + locals.var_q_sub0_dep_dpd_dn8), (locals.var_q_b0_dep_dpd_dn9 + locals.var_q_sub0_dep_dpd_dn9), (locals.var_q_b0_dep_dpd_dn10 + locals.var_q_sub0_dep_dpd_dn10), (locals.var_q_b0_dep_dpd_dn11 + locals.var_q_sub0_dep_dpd_dn11), (locals.var_q_b0_dep_dpd_dn14 + locals.var_q_sub0_dep_dpd_dn14),)
    } else {
        (locals.var_y12, locals.var_y12_dn0, locals.var_y12_dn2, locals.var_y12_dn4, locals.var_y12_dn5, locals.var_y12_dn6, locals.var_y12_dn7, locals.var_y12_dn8, locals.var_y12_dn9, locals.var_y12_dn10, locals.var_y12_dn11, locals.var_y12_dn14,)
    }
};
            locals.var_y12 = assign24620_body79_e20018;
            locals.var_y12_dn0 = assign24620_body79_e20018_d_n0;
            locals.var_y12_dn2 = assign24620_body79_e20018_d_n2;
            locals.var_y12_dn4 = assign24620_body79_e20018_d_n4;
            locals.var_y12_dn5 = assign24620_body79_e20018_d_n5;
            locals.var_y12_dn6 = assign24620_body79_e20018_d_n6;
            locals.var_y12_dn7 = assign24620_body79_e20018_d_n7;
            locals.var_y12_dn8 = assign24620_body79_e20018_d_n8;
            locals.var_y12_dn9 = assign24620_body79_e20018_d_n9;
            locals.var_y12_dn10 = assign24620_body79_e20018_d_n10;
            locals.var_y12_dn11 = assign24620_body79_e20018_d_n11;
            locals.var_y12_dn14 = assign24620_body79_e20018_d_n14;
            locals.var_y12_rv = 0.0;
            let (assign24620_body80_e20036, assign24620_body80_e20036_d_n0, assign24620_body80_e20036_d_n2, assign24620_body80_e20036_d_n4, assign24620_body80_e20036_d_n5, assign24620_body80_e20036_d_n6, assign24620_body80_e20036_d_n7, assign24620_body80_e20036_d_n8, assign24620_body80_e20036_d_n9, assign24620_body80_e20036_d_n10, assign24620_body80_e20036_d_n11, assign24620_body80_e20036_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body80_e20028: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign24620_body80_e20030: f64 = (assign24620_body80_e20028 + locals.var_vbscl__blk437);
        let assign24620_body80_e20032: f64 = (assign24620_body80_e20030 - locals.var_vbi_dep);
        let assign24620_body80_e20033: f64 = (locals.var_ndepmpnsub_inv1 * assign24620_body80_e20032);
        let assign24620_body80_e20034: f64 = (locals.var_phi_j0_dep - assign24620_body80_e20033);
        (assign24620_body80_e20034, (locals.var_phi_j0_dep_dn0 - ((locals.var_ndepmpnsub_inv1_dn0 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0)))), (locals.var_phi_j0_dep_dn2 - ((locals.var_ndepmpnsub_inv1_dn2 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2)))), (locals.var_phi_j0_dep_dn4 - ((locals.var_ndepmpnsub_inv1_dn4 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4)))), (locals.var_phi_j0_dep_dn5 - ((locals.var_ndepmpnsub_inv1_dn5 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5)))), (locals.var_phi_j0_dep_dn6 - ((locals.var_ndepmpnsub_inv1_dn6 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6)))), (locals.var_phi_j0_dep_dn7 - ((locals.var_ndepmpnsub_inv1_dn7 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7)))), (locals.var_phi_j0_dep_dn8 - ((locals.var_ndepmpnsub_inv1_dn8 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8)))), (locals.var_phi_j0_dep_dn9 - ((locals.var_ndepmpnsub_inv1_dn9 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9)))), (locals.var_phi_j0_dep_dn10 - ((locals.var_ndepmpnsub_inv1_dn10 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10)))), (locals.var_phi_j0_dep_dn11 - ((locals.var_ndepmpnsub_inv1_dn11 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11)))), (locals.var_phi_j0_dep_dn14 - ((locals.var_ndepmpnsub_inv1_dn14 * assign24620_body80_e20032) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14)))),)
    } else {
        (locals.var_y2, locals.var_y2_dn0, locals.var_y2_dn2, locals.var_y2_dn4, locals.var_y2_dn5, locals.var_y2_dn6, locals.var_y2_dn7, locals.var_y2_dn8, locals.var_y2_dn9, locals.var_y2_dn10, locals.var_y2_dn11, locals.var_y2_dn14,)
    }
};
            locals.var_y2 = assign24620_body80_e20036;
            locals.var_y2_dn0 = assign24620_body80_e20036_d_n0;
            locals.var_y2_dn2 = assign24620_body80_e20036_d_n2;
            locals.var_y2_dn4 = assign24620_body80_e20036_d_n4;
            locals.var_y2_dn5 = assign24620_body80_e20036_d_n5;
            locals.var_y2_dn6 = assign24620_body80_e20036_d_n6;
            locals.var_y2_dn7 = assign24620_body80_e20036_d_n7;
            locals.var_y2_dn8 = assign24620_body80_e20036_d_n8;
            locals.var_y2_dn9 = assign24620_body80_e20036_d_n9;
            locals.var_y2_dn10 = assign24620_body80_e20036_d_n10;
            locals.var_y2_dn11 = assign24620_body80_e20036_d_n11;
            locals.var_y2_dn14 = assign24620_body80_e20036_d_n14;
            locals.var_y2_rv = 0.0;
            let (assign24620_body81_e20044, assign24620_body81_e20044_d_n0, assign24620_body81_e20044_d_n2, assign24620_body81_e20044_d_n4, assign24620_body81_e20044_d_n5, assign24620_body81_e20044_d_n6, assign24620_body81_e20044_d_n7, assign24620_body81_e20044_d_n8, assign24620_body81_e20044_d_n9, assign24620_body81_e20044_d_n10, assign24620_body81_e20044_d_n11, assign24620_body81_e20044_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y21, locals.var_y21_dn0, locals.var_y21_dn2, locals.var_y21_dn4, locals.var_y21_dn5, locals.var_y21_dn6, locals.var_y21_dn7, locals.var_y21_dn8, locals.var_y21_dn9, locals.var_y21_dn10, locals.var_y21_dn11, locals.var_y21_dn14,)
    }
};
            locals.var_y21 = assign24620_body81_e20044;
            locals.var_y21_dn0 = assign24620_body81_e20044_d_n0;
            locals.var_y21_dn2 = assign24620_body81_e20044_d_n2;
            locals.var_y21_dn4 = assign24620_body81_e20044_d_n4;
            locals.var_y21_dn5 = assign24620_body81_e20044_d_n5;
            locals.var_y21_dn6 = assign24620_body81_e20044_d_n6;
            locals.var_y21_dn7 = assign24620_body81_e20044_d_n7;
            locals.var_y21_dn8 = assign24620_body81_e20044_d_n8;
            locals.var_y21_dn9 = assign24620_body81_e20044_d_n9;
            locals.var_y21_dn10 = assign24620_body81_e20044_d_n10;
            locals.var_y21_dn11 = assign24620_body81_e20044_d_n11;
            locals.var_y21_dn14 = assign24620_body81_e20044_d_n14;
            locals.var_y21_rv = 0.0;
            let (assign24620_body82_e20052, assign24620_body82_e20052_d_n0, assign24620_body82_e20052_d_n2, assign24620_body82_e20052_d_n4, assign24620_body82_e20052_d_n5, assign24620_body82_e20052_d_n6, assign24620_body82_e20052_d_n7, assign24620_body82_e20052_d_n8, assign24620_body82_e20052_d_n9, assign24620_body82_e20052_d_n10, assign24620_body82_e20052_d_n11, assign24620_body82_e20052_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y22, locals.var_y22_dn0, locals.var_y22_dn2, locals.var_y22_dn4, locals.var_y22_dn5, locals.var_y22_dn6, locals.var_y22_dn7, locals.var_y22_dn8, locals.var_y22_dn9, locals.var_y22_dn10, locals.var_y22_dn11, locals.var_y22_dn14,)
    }
};
            locals.var_y22 = assign24620_body82_e20052;
            locals.var_y22_dn0 = assign24620_body82_e20052_d_n0;
            locals.var_y22_dn2 = assign24620_body82_e20052_d_n2;
            locals.var_y22_dn4 = assign24620_body82_e20052_d_n4;
            locals.var_y22_dn5 = assign24620_body82_e20052_d_n5;
            locals.var_y22_dn6 = assign24620_body82_e20052_d_n6;
            locals.var_y22_dn7 = assign24620_body82_e20052_d_n7;
            locals.var_y22_dn8 = assign24620_body82_e20052_d_n8;
            locals.var_y22_dn9 = assign24620_body82_e20052_d_n9;
            locals.var_y22_dn10 = assign24620_body82_e20052_d_n10;
            locals.var_y22_dn11 = assign24620_body82_e20052_d_n11;
            locals.var_y22_dn14 = assign24620_body82_e20052_d_n14;
            locals.var_y22_rv = 0.0;
            let (assign24620_body83_e20066, assign24620_body83_e20066_d_n0, assign24620_body83_e20066_d_n2, assign24620_body83_e20066_d_n4, assign24620_body83_e20066_d_n5, assign24620_body83_e20066_d_n6, assign24620_body83_e20066_d_n7, assign24620_body83_e20066_d_n8, assign24620_body83_e20066_d_n9, assign24620_body83_e20066_d_n10, assign24620_body83_e20066_d_n11, assign24620_body83_e20066_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body83_e20060: f64 = (locals.var_y11 * locals.var_y22);
        let assign24620_body83_e20063: f64 = (locals.var_y21 * locals.var_y12);
        let assign24620_body83_e20064: f64 = (assign24620_body83_e20060 - assign24620_body83_e20063);
        (assign24620_body83_e20064, (((locals.var_y11_dn0 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn0)) - ((locals.var_y21_dn0 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn0))), (((locals.var_y11_dn2 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn2)) - ((locals.var_y21_dn2 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn2))), (((locals.var_y11_dn4 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn4)) - ((locals.var_y21_dn4 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn4))), (((locals.var_y11_dn5 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn5)) - ((locals.var_y21_dn5 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn5))), (((locals.var_y11_dn6 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn6)) - ((locals.var_y21_dn6 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn6))), (((locals.var_y11_dn7 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn7)) - ((locals.var_y21_dn7 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn7))), (((locals.var_y11_dn8 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn8)) - ((locals.var_y21_dn8 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn8))), (((locals.var_y11_dn9 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn9)) - ((locals.var_y21_dn9 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn9))), (((locals.var_y11_dn10 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn10)) - ((locals.var_y21_dn10 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn10))), (((locals.var_y11_dn11 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn11)) - ((locals.var_y21_dn11 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn11))), (((locals.var_y11_dn14 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn14)) - ((locals.var_y21_dn14 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn14))),)
    } else {
        (locals.var_dety, locals.var_dety_dn0, locals.var_dety_dn2, locals.var_dety_dn4, locals.var_dety_dn5, locals.var_dety_dn6, locals.var_dety_dn7, locals.var_dety_dn8, locals.var_dety_dn9, locals.var_dety_dn10, locals.var_dety_dn11, locals.var_dety_dn14,)
    }
};
            locals.var_dety = assign24620_body83_e20066;
            locals.var_dety_dn0 = assign24620_body83_e20066_d_n0;
            locals.var_dety_dn2 = assign24620_body83_e20066_d_n2;
            locals.var_dety_dn4 = assign24620_body83_e20066_d_n4;
            locals.var_dety_dn5 = assign24620_body83_e20066_d_n5;
            locals.var_dety_dn6 = assign24620_body83_e20066_d_n6;
            locals.var_dety_dn7 = assign24620_body83_e20066_d_n7;
            locals.var_dety_dn8 = assign24620_body83_e20066_d_n8;
            locals.var_dety_dn9 = assign24620_body83_e20066_d_n9;
            locals.var_dety_dn10 = assign24620_body83_e20066_d_n10;
            locals.var_dety_dn11 = assign24620_body83_e20066_d_n11;
            locals.var_dety_dn14 = assign24620_body83_e20066_d_n14;
            locals.var_dety_rv = 0.0;
            let (assign24620_body84_e20076, assign24620_body84_e20076_d_n0, assign24620_body84_e20076_d_n2, assign24620_body84_e20076_d_n4, assign24620_body84_e20076_d_n5, assign24620_body84_e20076_d_n6, assign24620_body84_e20076_d_n7, assign24620_body84_e20076_d_n8, assign24620_body84_e20076_d_n9, assign24620_body84_e20076_d_n10, assign24620_body84_e20076_d_n11, assign24620_body84_e20076_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body84_e20074: f64 = (locals.var_y22 / locals.var_dety);
        (assign24620_body84_e20074, (((locals.var_y22_dn0 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn2 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn4 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn5 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn6 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn7 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn8 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn9 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn10 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn11 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn14 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev11, locals.var_rev11_dn0, locals.var_rev11_dn2, locals.var_rev11_dn4, locals.var_rev11_dn5, locals.var_rev11_dn6, locals.var_rev11_dn7, locals.var_rev11_dn8, locals.var_rev11_dn9, locals.var_rev11_dn10, locals.var_rev11_dn11, locals.var_rev11_dn14,)
    }
};
            locals.var_rev11 = assign24620_body84_e20076;
            locals.var_rev11_dn0 = assign24620_body84_e20076_d_n0;
            locals.var_rev11_dn2 = assign24620_body84_e20076_d_n2;
            locals.var_rev11_dn4 = assign24620_body84_e20076_d_n4;
            locals.var_rev11_dn5 = assign24620_body84_e20076_d_n5;
            locals.var_rev11_dn6 = assign24620_body84_e20076_d_n6;
            locals.var_rev11_dn7 = assign24620_body84_e20076_d_n7;
            locals.var_rev11_dn8 = assign24620_body84_e20076_d_n8;
            locals.var_rev11_dn9 = assign24620_body84_e20076_d_n9;
            locals.var_rev11_dn10 = assign24620_body84_e20076_d_n10;
            locals.var_rev11_dn11 = assign24620_body84_e20076_d_n11;
            locals.var_rev11_dn14 = assign24620_body84_e20076_d_n14;
            locals.var_rev11_rv = 0.0;
            let (assign24620_body85_e20087, assign24620_body85_e20087_d_n0, assign24620_body85_e20087_d_n2, assign24620_body85_e20087_d_n4, assign24620_body85_e20087_d_n5, assign24620_body85_e20087_d_n6, assign24620_body85_e20087_d_n7, assign24620_body85_e20087_d_n8, assign24620_body85_e20087_d_n9, assign24620_body85_e20087_d_n10, assign24620_body85_e20087_d_n11, assign24620_body85_e20087_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body85_e20083: f64 = (-locals.var_y12);
        let assign24620_body85_e20085: f64 = (assign24620_body85_e20083 / locals.var_dety);
        (assign24620_body85_e20085, ((((-locals.var_y12_dn0) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn2) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn4) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn5) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn6) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn7) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn8) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn9) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn10) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn11) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn14) * locals.var_dety) - (assign24620_body85_e20083 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev12, locals.var_rev12_dn0, locals.var_rev12_dn2, locals.var_rev12_dn4, locals.var_rev12_dn5, locals.var_rev12_dn6, locals.var_rev12_dn7, locals.var_rev12_dn8, locals.var_rev12_dn9, locals.var_rev12_dn10, locals.var_rev12_dn11, locals.var_rev12_dn14,)
    }
};
            locals.var_rev12 = assign24620_body85_e20087;
            locals.var_rev12_dn0 = assign24620_body85_e20087_d_n0;
            locals.var_rev12_dn2 = assign24620_body85_e20087_d_n2;
            locals.var_rev12_dn4 = assign24620_body85_e20087_d_n4;
            locals.var_rev12_dn5 = assign24620_body85_e20087_d_n5;
            locals.var_rev12_dn6 = assign24620_body85_e20087_d_n6;
            locals.var_rev12_dn7 = assign24620_body85_e20087_d_n7;
            locals.var_rev12_dn8 = assign24620_body85_e20087_d_n8;
            locals.var_rev12_dn9 = assign24620_body85_e20087_d_n9;
            locals.var_rev12_dn10 = assign24620_body85_e20087_d_n10;
            locals.var_rev12_dn11 = assign24620_body85_e20087_d_n11;
            locals.var_rev12_dn14 = assign24620_body85_e20087_d_n14;
            locals.var_rev12_rv = 0.0;
            let (assign24620_body86_e20098, assign24620_body86_e20098_d_n0, assign24620_body86_e20098_d_n2, assign24620_body86_e20098_d_n4, assign24620_body86_e20098_d_n5, assign24620_body86_e20098_d_n6, assign24620_body86_e20098_d_n7, assign24620_body86_e20098_d_n8, assign24620_body86_e20098_d_n9, assign24620_body86_e20098_d_n10, assign24620_body86_e20098_d_n11, assign24620_body86_e20098_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body86_e20094: f64 = (-locals.var_y21);
        let assign24620_body86_e20096: f64 = (assign24620_body86_e20094 / locals.var_dety);
        (assign24620_body86_e20096, ((((-locals.var_y21_dn0) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn2) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn4) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn5) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn6) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn7) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn8) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn9) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn10) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn11) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn14) * locals.var_dety) - (assign24620_body86_e20094 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev21, locals.var_rev21_dn0, locals.var_rev21_dn2, locals.var_rev21_dn4, locals.var_rev21_dn5, locals.var_rev21_dn6, locals.var_rev21_dn7, locals.var_rev21_dn8, locals.var_rev21_dn9, locals.var_rev21_dn10, locals.var_rev21_dn11, locals.var_rev21_dn14,)
    }
};
            locals.var_rev21 = assign24620_body86_e20098;
            locals.var_rev21_dn0 = assign24620_body86_e20098_d_n0;
            locals.var_rev21_dn2 = assign24620_body86_e20098_d_n2;
            locals.var_rev21_dn4 = assign24620_body86_e20098_d_n4;
            locals.var_rev21_dn5 = assign24620_body86_e20098_d_n5;
            locals.var_rev21_dn6 = assign24620_body86_e20098_d_n6;
            locals.var_rev21_dn7 = assign24620_body86_e20098_d_n7;
            locals.var_rev21_dn8 = assign24620_body86_e20098_d_n8;
            locals.var_rev21_dn9 = assign24620_body86_e20098_d_n9;
            locals.var_rev21_dn10 = assign24620_body86_e20098_d_n10;
            locals.var_rev21_dn11 = assign24620_body86_e20098_d_n11;
            locals.var_rev21_dn14 = assign24620_body86_e20098_d_n14;
            locals.var_rev21_rv = 0.0;
            let (assign24620_body87_e20108, assign24620_body87_e20108_d_n0, assign24620_body87_e20108_d_n2, assign24620_body87_e20108_d_n4, assign24620_body87_e20108_d_n5, assign24620_body87_e20108_d_n6, assign24620_body87_e20108_d_n7, assign24620_body87_e20108_d_n8, assign24620_body87_e20108_d_n9, assign24620_body87_e20108_d_n10, assign24620_body87_e20108_d_n11, assign24620_body87_e20108_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body87_e20106: f64 = (locals.var_y11 / locals.var_dety);
        (assign24620_body87_e20106, (((locals.var_y11_dn0 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn2 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn4 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn5 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn6 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn7 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn8 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn9 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn10 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn11 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn14 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev22, locals.var_rev22_dn0, locals.var_rev22_dn2, locals.var_rev22_dn4, locals.var_rev22_dn5, locals.var_rev22_dn6, locals.var_rev22_dn7, locals.var_rev22_dn8, locals.var_rev22_dn9, locals.var_rev22_dn10, locals.var_rev22_dn11, locals.var_rev22_dn14,)
    }
};
            locals.var_rev22 = assign24620_body87_e20108;
            locals.var_rev22_dn0 = assign24620_body87_e20108_d_n0;
            locals.var_rev22_dn2 = assign24620_body87_e20108_d_n2;
            locals.var_rev22_dn4 = assign24620_body87_e20108_d_n4;
            locals.var_rev22_dn5 = assign24620_body87_e20108_d_n5;
            locals.var_rev22_dn6 = assign24620_body87_e20108_d_n6;
            locals.var_rev22_dn7 = assign24620_body87_e20108_d_n7;
            locals.var_rev22_dn8 = assign24620_body87_e20108_d_n8;
            locals.var_rev22_dn9 = assign24620_body87_e20108_d_n9;
            locals.var_rev22_dn10 = assign24620_body87_e20108_d_n10;
            locals.var_rev22_dn11 = assign24620_body87_e20108_d_n11;
            locals.var_rev22_dn14 = assign24620_body87_e20108_d_n14;
            locals.var_rev22_rv = 0.0;
            let assign24620_body88_e20111: f64 = (locals.var_rev11 * locals.var_y1);
            let assign24620_body88_e20114: f64 = (locals.var_rev12 * locals.var_y2);
            let assign24620_body88_e20115: f64 = (assign24620_body88_e20111 + assign24620_body88_e20114);
            let assign24620_body88_e20116: f64 = (assign24620_body88_e20115).abs();
            let assign24620_body88_e20118: f64 = if assign24620_body88_e20116 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard574 = assign24620_body88_e20118;
            locals.var_guard574_rv = 0.0;
            let (assign24620_body89_e20144, assign24620_body89_e20144_d_n0, assign24620_body89_e20144_d_n2, assign24620_body89_e20144_d_n4, assign24620_body89_e20144_d_n5, assign24620_body89_e20144_d_n6, assign24620_body89_e20144_d_n7, assign24620_body89_e20144_d_n8, assign24620_body89_e20144_d_n9, assign24620_body89_e20144_d_n10, assign24620_body89_e20144_d_n11, assign24620_body89_e20144_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign24620_body89_e20130: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24620_body89_e20133: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24620_body89_e20134: f64 = (assign24620_body89_e20130 + assign24620_body89_e20133);
        let (assign24620_body89_e20140,) = {
            if (assign24620_body89_e20134 >= 0.0) {
                (1.0,)
            } else {
                let assign24620_body89_e20139: f64 = (-1.0);
                (assign24620_body89_e20139,)
            }
        };
        let assign24620_body89_e20141: f64 = (0.5 * assign24620_body89_e20140);
        let assign24620_body89_e20142: f64 = (locals.var_vgp0 - assign24620_body89_e20141);
        (assign24620_body89_e20142, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign24620_body89_e20144;
            locals.var_vgp0_dn0 = assign24620_body89_e20144_d_n0;
            locals.var_vgp0_dn2 = assign24620_body89_e20144_d_n2;
            locals.var_vgp0_dn4 = assign24620_body89_e20144_d_n4;
            locals.var_vgp0_dn5 = assign24620_body89_e20144_d_n5;
            locals.var_vgp0_dn6 = assign24620_body89_e20144_d_n6;
            locals.var_vgp0_dn7 = assign24620_body89_e20144_d_n7;
            locals.var_vgp0_dn8 = assign24620_body89_e20144_d_n8;
            locals.var_vgp0_dn9 = assign24620_body89_e20144_d_n9;
            locals.var_vgp0_dn10 = assign24620_body89_e20144_d_n10;
            locals.var_vgp0_dn11 = assign24620_body89_e20144_d_n11;
            locals.var_vgp0_dn14 = assign24620_body89_e20144_d_n14;
            locals.var_vgp0_rv = 0.0;
            let (assign24620_body90_e20170, assign24620_body90_e20170_d_n0, assign24620_body90_e20170_d_n2, assign24620_body90_e20170_d_n4, assign24620_body90_e20170_d_n5, assign24620_body90_e20170_d_n6, assign24620_body90_e20170_d_n7, assign24620_body90_e20170_d_n8, assign24620_body90_e20170_d_n9, assign24620_body90_e20170_d_n10, assign24620_body90_e20170_d_n11, assign24620_body90_e20170_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign24620_body90_e20156: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24620_body90_e20159: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24620_body90_e20160: f64 = (assign24620_body90_e20156 + assign24620_body90_e20159);
        let (assign24620_body90_e20166,) = {
            if (assign24620_body90_e20160 >= 0.0) {
                (1.0,)
            } else {
                let assign24620_body90_e20165: f64 = (-1.0);
                (assign24620_body90_e20165,)
            }
        };
        let assign24620_body90_e20167: f64 = (0.5 * assign24620_body90_e20166);
        let assign24620_body90_e20168: f64 = (locals.var_phi_j0_dep - assign24620_body90_e20167);
        (assign24620_body90_e20168, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
            locals.var_phi_j0_dep = assign24620_body90_e20170;
            locals.var_phi_j0_dep_dn0 = assign24620_body90_e20170_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24620_body90_e20170_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24620_body90_e20170_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24620_body90_e20170_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24620_body90_e20170_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24620_body90_e20170_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24620_body90_e20170_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24620_body90_e20170_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24620_body90_e20170_d_n10;
            locals.var_phi_j0_dep_dn11 = assign24620_body90_e20170_d_n11;
            locals.var_phi_j0_dep_dn14 = assign24620_body90_e20170_d_n14;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign24620_body91_e20189, assign24620_body91_e20189_d_n0, assign24620_body91_e20189_d_n2, assign24620_body91_e20189_d_n4, assign24620_body91_e20189_d_n5, assign24620_body91_e20189_d_n6, assign24620_body91_e20189_d_n7, assign24620_body91_e20189_d_n8, assign24620_body91_e20189_d_n9, assign24620_body91_e20189_d_n10, assign24620_body91_e20189_d_n11, assign24620_body91_e20189_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard574 == 0.0)) {
        let assign24620_body91_e20182: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24620_body91_e20185: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24620_body91_e20186: f64 = (assign24620_body91_e20182 + assign24620_body91_e20185);
        let assign24620_body91_e20187: f64 = (locals.var_vgp0 - assign24620_body91_e20186);
        (assign24620_body91_e20187, (locals.var_vgp0_dn0 - (((locals.var_rev11_dn0 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn0)) + ((locals.var_rev12_dn0 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn0)))), (locals.var_vgp0_dn2 - (((locals.var_rev11_dn2 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn2)) + ((locals.var_rev12_dn2 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn2)))), (locals.var_vgp0_dn4 - (((locals.var_rev11_dn4 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn4)) + ((locals.var_rev12_dn4 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn4)))), (locals.var_vgp0_dn5 - (((locals.var_rev11_dn5 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn5)) + ((locals.var_rev12_dn5 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn5)))), (locals.var_vgp0_dn6 - (((locals.var_rev11_dn6 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn6)) + ((locals.var_rev12_dn6 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn6)))), (locals.var_vgp0_dn7 - (((locals.var_rev11_dn7 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn7)) + ((locals.var_rev12_dn7 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn7)))), (locals.var_vgp0_dn8 - (((locals.var_rev11_dn8 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn8)) + ((locals.var_rev12_dn8 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn8)))), (locals.var_vgp0_dn9 - (((locals.var_rev11_dn9 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn9)) + ((locals.var_rev12_dn9 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn9)))), (locals.var_vgp0_dn10 - (((locals.var_rev11_dn10 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn10)) + ((locals.var_rev12_dn10 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn10)))), (locals.var_vgp0_dn11 - (((locals.var_rev11_dn11 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn11)) + ((locals.var_rev12_dn11 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn11)))), (locals.var_vgp0_dn14 - (((locals.var_rev11_dn14 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn14)) + ((locals.var_rev12_dn14 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign24620_body91_e20189;
            locals.var_vgp0_dn0 = assign24620_body91_e20189_d_n0;
            locals.var_vgp0_dn2 = assign24620_body91_e20189_d_n2;
            locals.var_vgp0_dn4 = assign24620_body91_e20189_d_n4;
            locals.var_vgp0_dn5 = assign24620_body91_e20189_d_n5;
            locals.var_vgp0_dn6 = assign24620_body91_e20189_d_n6;
            locals.var_vgp0_dn7 = assign24620_body91_e20189_d_n7;
            locals.var_vgp0_dn8 = assign24620_body91_e20189_d_n8;
            locals.var_vgp0_dn9 = assign24620_body91_e20189_d_n9;
            locals.var_vgp0_dn10 = assign24620_body91_e20189_d_n10;
            locals.var_vgp0_dn11 = assign24620_body91_e20189_d_n11;
            locals.var_vgp0_dn14 = assign24620_body91_e20189_d_n14;
            locals.var_vgp0_rv = 0.0;
            let (assign24620_body92_e20208, assign24620_body92_e20208_d_n0, assign24620_body92_e20208_d_n2, assign24620_body92_e20208_d_n4, assign24620_body92_e20208_d_n5, assign24620_body92_e20208_d_n6, assign24620_body92_e20208_d_n7, assign24620_body92_e20208_d_n8, assign24620_body92_e20208_d_n9, assign24620_body92_e20208_d_n10, assign24620_body92_e20208_d_n11, assign24620_body92_e20208_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard574 == 0.0)) {
        let assign24620_body92_e20201: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24620_body92_e20204: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24620_body92_e20205: f64 = (assign24620_body92_e20201 + assign24620_body92_e20204);
        let assign24620_body92_e20206: f64 = (locals.var_phi_j0_dep - assign24620_body92_e20205);
        (assign24620_body92_e20206, (locals.var_phi_j0_dep_dn0 - (((locals.var_rev21_dn0 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn0)) + ((locals.var_rev22_dn0 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn0)))), (locals.var_phi_j0_dep_dn2 - (((locals.var_rev21_dn2 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn2)) + ((locals.var_rev22_dn2 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn2)))), (locals.var_phi_j0_dep_dn4 - (((locals.var_rev21_dn4 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn4)) + ((locals.var_rev22_dn4 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn4)))), (locals.var_phi_j0_dep_dn5 - (((locals.var_rev21_dn5 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn5)) + ((locals.var_rev22_dn5 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn5)))), (locals.var_phi_j0_dep_dn6 - (((locals.var_rev21_dn6 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn6)) + ((locals.var_rev22_dn6 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn6)))), (locals.var_phi_j0_dep_dn7 - (((locals.var_rev21_dn7 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn7)) + ((locals.var_rev22_dn7 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn7)))), (locals.var_phi_j0_dep_dn8 - (((locals.var_rev21_dn8 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn8)) + ((locals.var_rev22_dn8 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn8)))), (locals.var_phi_j0_dep_dn9 - (((locals.var_rev21_dn9 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn9)) + ((locals.var_rev22_dn9 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn9)))), (locals.var_phi_j0_dep_dn10 - (((locals.var_rev21_dn10 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn10)) + ((locals.var_rev22_dn10 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn10)))), (locals.var_phi_j0_dep_dn11 - (((locals.var_rev21_dn11 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn11)) + ((locals.var_rev22_dn11 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn11)))), (locals.var_phi_j0_dep_dn14 - (((locals.var_rev21_dn14 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn14)) + ((locals.var_rev22_dn14 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
            locals.var_phi_j0_dep = assign24620_body92_e20208;
            locals.var_phi_j0_dep_dn0 = assign24620_body92_e20208_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24620_body92_e20208_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24620_body92_e20208_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24620_body92_e20208_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24620_body92_e20208_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24620_body92_e20208_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24620_body92_e20208_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24620_body92_e20208_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24620_body92_e20208_d_n10;
            locals.var_phi_j0_dep_dn11 = assign24620_body92_e20208_d_n11;
            locals.var_phi_j0_dep_dn14 = assign24620_body92_e20208_d_n14;
            locals.var_phi_j0_dep_rv = 0.0;
            let assign24620_body93_e20211: f64 = (locals.var_vgp0 - locals.var_vgp0old);
            let assign24620_body93_e20212: f64 = (assign24620_body93_e20211).abs();
            let assign24620_body93_e20217: f64 = (locals.var_phi_j0_dep - locals.var_phi_j0_dep_old);
            let assign24620_body93_e20218: f64 = (assign24620_body93_e20217).abs();
            let assign24620_body93_e20221: f64 = if ((assign24620_body93_e20212 <= 1e-12) && (assign24620_body93_e20218 <= 1e-12)) { 1.0 } else { 0.0 };
            locals.var_guard575 = assign24620_body93_e20221;
            locals.var_guard575_rv = 0.0;
            let (assign24620_body94_e20233,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard575 != 0.0)) {
        let assign24620_body94_e20231: f64 = (150.0 + 1.0);
        (assign24620_body94_e20231,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24620_body94_e20233;
            locals.var_lp_s0_rv = 0.0;
            let (assign24620_body95_e20241, assign24620_body95_e20241_d_n0, assign24620_body95_e20241_d_n2, assign24620_body95_e20241_d_n4, assign24620_body95_e20241_d_n5, assign24620_body95_e20241_d_n6, assign24620_body95_e20241_d_n7, assign24620_body95_e20241_d_n8, assign24620_body95_e20241_d_n9, assign24620_body95_e20241_d_n10, assign24620_body95_e20241_d_n11, assign24620_body95_e20241_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn11, locals.var_vgp0old_dn14,)
    }
};
            locals.var_vgp0old = assign24620_body95_e20241;
            locals.var_vgp0old_dn0 = assign24620_body95_e20241_d_n0;
            locals.var_vgp0old_dn2 = assign24620_body95_e20241_d_n2;
            locals.var_vgp0old_dn4 = assign24620_body95_e20241_d_n4;
            locals.var_vgp0old_dn5 = assign24620_body95_e20241_d_n5;
            locals.var_vgp0old_dn6 = assign24620_body95_e20241_d_n6;
            locals.var_vgp0old_dn7 = assign24620_body95_e20241_d_n7;
            locals.var_vgp0old_dn8 = assign24620_body95_e20241_d_n8;
            locals.var_vgp0old_dn9 = assign24620_body95_e20241_d_n9;
            locals.var_vgp0old_dn10 = assign24620_body95_e20241_d_n10;
            locals.var_vgp0old_dn11 = assign24620_body95_e20241_d_n11;
            locals.var_vgp0old_dn14 = assign24620_body95_e20241_d_n14;
            locals.var_vgp0old_rv = 0.0;
            let (assign24620_body96_e20249, assign24620_body96_e20249_d_n0, assign24620_body96_e20249_d_n2, assign24620_body96_e20249_d_n4, assign24620_body96_e20249_d_n5, assign24620_body96_e20249_d_n6, assign24620_body96_e20249_d_n7, assign24620_body96_e20249_d_n8, assign24620_body96_e20249_d_n9, assign24620_body96_e20249_d_n10, assign24620_body96_e20249_d_n11, assign24620_body96_e20249_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn11, locals.var_phi_j0_dep_old_dn14,)
    }
};
            locals.var_phi_j0_dep_old = assign24620_body96_e20249;
            locals.var_phi_j0_dep_old_dn0 = assign24620_body96_e20249_d_n0;
            locals.var_phi_j0_dep_old_dn2 = assign24620_body96_e20249_d_n2;
            locals.var_phi_j0_dep_old_dn4 = assign24620_body96_e20249_d_n4;
            locals.var_phi_j0_dep_old_dn5 = assign24620_body96_e20249_d_n5;
            locals.var_phi_j0_dep_old_dn6 = assign24620_body96_e20249_d_n6;
            locals.var_phi_j0_dep_old_dn7 = assign24620_body96_e20249_d_n7;
            locals.var_phi_j0_dep_old_dn8 = assign24620_body96_e20249_d_n8;
            locals.var_phi_j0_dep_old_dn9 = assign24620_body96_e20249_d_n9;
            locals.var_phi_j0_dep_old_dn10 = assign24620_body96_e20249_d_n10;
            locals.var_phi_j0_dep_old_dn11 = assign24620_body96_e20249_d_n11;
            locals.var_phi_j0_dep_old_dn14 = assign24620_body96_e20249_d_n14;
            locals.var_phi_j0_dep_old_rv = 0.0;
            let (assign24620_body97_e20259,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24620_body97_e20257: f64 = (locals.var_lp_s0 + 1.0);
        (assign24620_body97_e20257,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24620_body97_e20259;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign24630_e20267, assign24630_e20267_d_n0, assign24630_e20267_d_n2, assign24630_e20267_d_n4, assign24630_e20267_d_n5, assign24630_e20267_d_n6, assign24630_e20267_d_n7, assign24630_e20267_d_n8, assign24630_e20267_d_n9, assign24630_e20267_d_n10, assign24630_e20267_d_n11, assign24630_e20267_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    }
};
        locals.var_phi_j0_dep_acc = assign24630_e20267;
        locals.var_phi_j0_dep_acc_dn0 = assign24630_e20267_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24630_e20267_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24630_e20267_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24630_e20267_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24630_e20267_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24630_e20267_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24630_e20267_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24630_e20267_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24630_e20267_d_n10;
        locals.var_phi_j0_dep_acc_dn11 = assign24630_e20267_d_n11;
        locals.var_phi_j0_dep_acc_dn14 = assign24630_e20267_d_n14;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let (assign24640_e20277, assign24640_e20277_d_n0, assign24640_e20277_d_n2, assign24640_e20277_d_n4, assign24640_e20277_d_n5, assign24640_e20277_d_n6, assign24640_e20277_d_n7, assign24640_e20277_d_n8, assign24640_e20277_d_n9, assign24640_e20277_d_n10, assign24640_e20277_d_n11, assign24640_e20277_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24640_e20275: f64 = (locals.var_uc_depthn * locals.var_ndepmpnsub);
        (assign24640_e20275, ((locals.var_uc_depthn_dn0 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign24640_e20277;
        locals.var_w_sub0_dn0 = assign24640_e20277_d_n0;
        locals.var_w_sub0_dn2 = assign24640_e20277_d_n2;
        locals.var_w_sub0_dn4 = assign24640_e20277_d_n4;
        locals.var_w_sub0_dn5 = assign24640_e20277_d_n5;
        locals.var_w_sub0_dn6 = assign24640_e20277_d_n6;
        locals.var_w_sub0_dn7 = assign24640_e20277_d_n7;
        locals.var_w_sub0_dn8 = assign24640_e20277_d_n8;
        locals.var_w_sub0_dn9 = assign24640_e20277_d_n9;
        locals.var_w_sub0_dn10 = assign24640_e20277_d_n10;
        locals.var_w_sub0_dn11 = assign24640_e20277_d_n11;
        locals.var_w_sub0_dn14 = assign24640_e20277_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign24650_e20293, assign24650_e20293_d_n0, assign24650_e20293_d_n2, assign24650_e20293_d_n4, assign24650_e20293_d_n5, assign24650_e20293_d_n6, assign24650_e20293_d_n7, assign24650_e20293_d_n8, assign24650_e20293_d_n9, assign24650_e20293_d_n10, assign24650_e20293_d_n11, assign24650_e20293_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24650_e20285: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24650_e20287: f64 = (assign24650_e20285 * locals.var_w_sub0);
        let assign24650_e20289: f64 = (assign24650_e20287 + locals.var_vbscl__blk437);
        let assign24650_e20291: f64 = (assign24650_e20289 - locals.var_vbi_dep);
        (assign24650_e20291, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn11)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn14)) * locals.var_w_sub0) + (assign24650_e20285 * locals.var_w_sub0_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24650_e20293;
        locals.var_phi_j0_dep_dn0 = assign24650_e20293_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24650_e20293_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24650_e20293_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24650_e20293_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24650_e20293_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24650_e20293_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24650_e20293_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24650_e20293_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24650_e20293_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24650_e20293_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24650_e20293_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24660_e20305, assign24660_e20305_d_n0, assign24660_e20305_d_n2, assign24660_e20305_d_n4, assign24660_e20305_d_n5, assign24660_e20305_d_n6, assign24660_e20305_d_n7, assign24660_e20305_d_n8, assign24660_e20305_d_n9, assign24660_e20305_d_n10, assign24660_e20305_d_n11, assign24660_e20305_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24660_e20302: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_tn2);
        let assign24660_e20303: f64 = (locals.var_phi_j0_dep + assign24660_e20302);
        (assign24660_e20303, (locals.var_phi_j0_dep_dn0 + ((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn0))), (locals.var_phi_j0_dep_dn2 + ((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn2))), (locals.var_phi_j0_dep_dn4 + ((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn4))), (locals.var_phi_j0_dep_dn5 + ((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn5))), (locals.var_phi_j0_dep_dn6 + ((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn6))), (locals.var_phi_j0_dep_dn7 + ((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn7))), (locals.var_phi_j0_dep_dn8 + ((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn8))), (locals.var_phi_j0_dep_dn9 + ((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn9))), (locals.var_phi_j0_dep_dn10 + ((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn10))), (locals.var_phi_j0_dep_dn11 + ((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn11))), (locals.var_phi_j0_dep_dn14 + ((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn14))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24660_e20305;
        locals.var_phi_b0_dep_dn0 = assign24660_e20305_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24660_e20305_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24660_e20305_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24660_e20305_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24660_e20305_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24660_e20305_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24660_e20305_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24660_e20305_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24660_e20305_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24660_e20305_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24660_e20305_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24670_e20313, assign24670_e20313_d_n0, assign24670_e20313_d_n2, assign24670_e20313_d_n4, assign24670_e20313_d_n5, assign24670_e20313_d_n6, assign24670_e20313_d_n7, assign24670_e20313_d_n8, assign24670_e20313_d_n9, assign24670_e20313_d_n10, assign24670_e20313_d_n11, assign24670_e20313_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    }
};
        locals.var_phi_s0_dep = assign24670_e20313;
        locals.var_phi_s0_dep_dn0 = assign24670_e20313_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24670_e20313_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24670_e20313_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24670_e20313_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24670_e20313_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24670_e20313_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24670_e20313_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24670_e20313_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24670_e20313_d_n10;
        locals.var_phi_s0_dep_dn11 = assign24670_e20313_d_n11;
        locals.var_phi_s0_dep_dn14 = assign24670_e20313_d_n14;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24680_e20321, assign24680_e20321_d_n0, assign24680_e20321_d_n2, assign24680_e20321_d_n4, assign24680_e20321_d_n5, assign24680_e20321_d_n6, assign24680_e20321_d_n7, assign24680_e20321_d_n8, assign24680_e20321_d_n9, assign24680_e20321_d_n10, assign24680_e20321_d_n11, assign24680_e20321_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign24680_e20321;
        locals.var_psbmax_dn0 = assign24680_e20321_d_n0;
        locals.var_psbmax_dn2 = assign24680_e20321_d_n2;
        locals.var_psbmax_dn4 = assign24680_e20321_d_n4;
        locals.var_psbmax_dn5 = assign24680_e20321_d_n5;
        locals.var_psbmax_dn6 = assign24680_e20321_d_n6;
        locals.var_psbmax_dn7 = assign24680_e20321_d_n7;
        locals.var_psbmax_dn8 = assign24680_e20321_d_n8;
        locals.var_psbmax_dn9 = assign24680_e20321_d_n9;
        locals.var_psbmax_dn10 = assign24680_e20321_d_n10;
        locals.var_psbmax_dn11 = assign24680_e20321_d_n11;
        locals.var_psbmax_dn14 = assign24680_e20321_d_n14;
        locals.var_psbmax_rv = 0.0;

        let (assign24690_e20329, assign24690_e20329_d_n0, assign24690_e20329_d_n2, assign24690_e20329_d_n4, assign24690_e20329_d_n5, assign24690_e20329_d_n6, assign24690_e20329_d_n7, assign24690_e20329_d_n8, assign24690_e20329_d_n9, assign24690_e20329_d_n10, assign24690_e20329_d_n11, assign24690_e20329_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn11, locals.var_vgp1_dn14,)
    }
};
        locals.var_vgp1 = assign24690_e20329;
        locals.var_vgp1_dn0 = assign24690_e20329_d_n0;
        locals.var_vgp1_dn2 = assign24690_e20329_d_n2;
        locals.var_vgp1_dn4 = assign24690_e20329_d_n4;
        locals.var_vgp1_dn5 = assign24690_e20329_d_n5;
        locals.var_vgp1_dn6 = assign24690_e20329_d_n6;
        locals.var_vgp1_dn7 = assign24690_e20329_d_n7;
        locals.var_vgp1_dn8 = assign24690_e20329_d_n8;
        locals.var_vgp1_dn9 = assign24690_e20329_d_n9;
        locals.var_vgp1_dn10 = assign24690_e20329_d_n10;
        locals.var_vgp1_dn11 = assign24690_e20329_d_n11;
        locals.var_vgp1_dn14 = assign24690_e20329_d_n14;
        locals.var_vgp1_rv = 0.0;

        let assign24700_e20332: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign24700_e20332;
        locals.var_guard576_rv = 0.0;

        let (assign24710_e20342,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard576 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24710_e20342;
        locals.var_depmode_rv = 0.0;

        let assign24720_e20345: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign24720_e20345;
        locals.var_guard577_rv = 0.0;

        let (assign24730_e20358,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard576 == 0.0)) && (locals.var_guard577 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24730_e20358;
        locals.var_depmode_rv = 0.0;

        let (assign24740_e20372,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard576 == 0.0)) && (locals.var_guard577 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24740_e20372;
        locals.var_depmode_rv = 0.0;

        let (assign24750_e20381, assign24750_e20381_d_n0, assign24750_e20381_d_n2, assign24750_e20381_d_n4, assign24750_e20381_d_n5, assign24750_e20381_d_n6, assign24750_e20381_d_n7, assign24750_e20381_d_n8, assign24750_e20381_d_n9, assign24750_e20381_d_n10, assign24750_e20381_d_n11, assign24750_e20381_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign24750_e20381;
        locals.var_vgp0_dn0 = assign24750_e20381_d_n0;
        locals.var_vgp0_dn2 = assign24750_e20381_d_n2;
        locals.var_vgp0_dn4 = assign24750_e20381_d_n4;
        locals.var_vgp0_dn5 = assign24750_e20381_d_n5;
        locals.var_vgp0_dn6 = assign24750_e20381_d_n6;
        locals.var_vgp0_dn7 = assign24750_e20381_d_n7;
        locals.var_vgp0_dn8 = assign24750_e20381_d_n8;
        locals.var_vgp0_dn9 = assign24750_e20381_d_n9;
        locals.var_vgp0_dn10 = assign24750_e20381_d_n10;
        locals.var_vgp0_dn11 = assign24750_e20381_d_n11;
        locals.var_vgp0_dn14 = assign24750_e20381_d_n14;
        locals.var_vgp0_rv = 0.0;

        let (assign24760_e20390, assign24760_e20390_d_n0, assign24760_e20390_d_n2, assign24760_e20390_d_n4, assign24760_e20390_d_n5, assign24760_e20390_d_n6, assign24760_e20390_d_n7, assign24760_e20390_d_n8, assign24760_e20390_d_n9, assign24760_e20390_d_n10, assign24760_e20390_d_n11, assign24760_e20390_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn11, locals.var_vgp1_dn14,)
    }
};
        locals.var_vgp1 = assign24760_e20390;
        locals.var_vgp1_dn0 = assign24760_e20390_d_n0;
        locals.var_vgp1_dn2 = assign24760_e20390_d_n2;
        locals.var_vgp1_dn4 = assign24760_e20390_d_n4;
        locals.var_vgp1_dn5 = assign24760_e20390_d_n5;
        locals.var_vgp1_dn6 = assign24760_e20390_d_n6;
        locals.var_vgp1_dn7 = assign24760_e20390_d_n7;
        locals.var_vgp1_dn8 = assign24760_e20390_d_n8;
        locals.var_vgp1_dn9 = assign24760_e20390_d_n9;
        locals.var_vgp1_dn10 = assign24760_e20390_d_n10;
        locals.var_vgp1_dn11 = assign24760_e20390_d_n11;
        locals.var_vgp1_dn14 = assign24760_e20390_d_n14;
        locals.var_vgp1_rv = 0.0;

        let (assign24770_e20399, assign24770_e20399_d_n0, assign24770_e20399_d_n2, assign24770_e20399_d_n4, assign24770_e20399_d_n5, assign24770_e20399_d_n6, assign24770_e20399_d_n7, assign24770_e20399_d_n8, assign24770_e20399_d_n9, assign24770_e20399_d_n10, assign24770_e20399_d_n11, assign24770_e20399_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign24770_e20399;
        locals.var_psbmax_dn0 = assign24770_e20399_d_n0;
        locals.var_psbmax_dn2 = assign24770_e20399_d_n2;
        locals.var_psbmax_dn4 = assign24770_e20399_d_n4;
        locals.var_psbmax_dn5 = assign24770_e20399_d_n5;
        locals.var_psbmax_dn6 = assign24770_e20399_d_n6;
        locals.var_psbmax_dn7 = assign24770_e20399_d_n7;
        locals.var_psbmax_dn8 = assign24770_e20399_d_n8;
        locals.var_psbmax_dn9 = assign24770_e20399_d_n9;
        locals.var_psbmax_dn10 = assign24770_e20399_d_n10;
        locals.var_psbmax_dn11 = assign24770_e20399_d_n11;
        locals.var_psbmax_dn14 = assign24770_e20399_d_n14;
        locals.var_psbmax_rv = 0.0;

        let (assign24780_e20408, assign24780_e20408_d_n0, assign24780_e20408_d_n2, assign24780_e20408_d_n4, assign24780_e20408_d_n5, assign24780_e20408_d_n6, assign24780_e20408_d_n7, assign24780_e20408_d_n8, assign24780_e20408_d_n9, assign24780_e20408_d_n10, assign24780_e20408_d_n11, assign24780_e20408_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    }
};
        locals.var_vds_maxb0 = assign24780_e20408;
        locals.var_vds_maxb0_dn0 = assign24780_e20408_d_n0;
        locals.var_vds_maxb0_dn2 = assign24780_e20408_d_n2;
        locals.var_vds_maxb0_dn4 = assign24780_e20408_d_n4;
        locals.var_vds_maxb0_dn5 = assign24780_e20408_d_n5;
        locals.var_vds_maxb0_dn6 = assign24780_e20408_d_n6;
        locals.var_vds_maxb0_dn7 = assign24780_e20408_d_n7;
        locals.var_vds_maxb0_dn8 = assign24780_e20408_d_n8;
        locals.var_vds_maxb0_dn9 = assign24780_e20408_d_n9;
        locals.var_vds_maxb0_dn10 = assign24780_e20408_d_n10;
        locals.var_vds_maxb0_dn11 = assign24780_e20408_d_n11;
        locals.var_vds_maxb0_dn14 = assign24780_e20408_d_n14;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24790_e20417, assign24790_e20417_d_n0, assign24790_e20417_d_n2, assign24790_e20417_d_n4, assign24790_e20417_d_n5, assign24790_e20417_d_n6, assign24790_e20417_d_n7, assign24790_e20417_d_n8, assign24790_e20417_d_n9, assign24790_e20417_d_n10, assign24790_e20417_d_n11, assign24790_e20417_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn11, locals.var_w_bsub0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign24790_e20417;
        locals.var_w_b0_dn0 = assign24790_e20417_d_n0;
        locals.var_w_b0_dn2 = assign24790_e20417_d_n2;
        locals.var_w_b0_dn4 = assign24790_e20417_d_n4;
        locals.var_w_b0_dn5 = assign24790_e20417_d_n5;
        locals.var_w_b0_dn6 = assign24790_e20417_d_n6;
        locals.var_w_b0_dn7 = assign24790_e20417_d_n7;
        locals.var_w_b0_dn8 = assign24790_e20417_d_n8;
        locals.var_w_b0_dn9 = assign24790_e20417_d_n9;
        locals.var_w_b0_dn10 = assign24790_e20417_d_n10;
        locals.var_w_b0_dn11 = assign24790_e20417_d_n11;
        locals.var_w_b0_dn14 = assign24790_e20417_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign24800_e20428, assign24800_e20428_d_n0, assign24800_e20428_d_n2, assign24800_e20428_d_n4, assign24800_e20428_d_n5, assign24800_e20428_d_n6, assign24800_e20428_d_n7, assign24800_e20428_d_n8, assign24800_e20428_d_n9, assign24800_e20428_d_n10, assign24800_e20428_d_n11, assign24800_e20428_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        let assign24800_e20426: f64 = (locals.var_w_b0 * locals.var_ndepmpnsub);
        (assign24800_e20426, ((locals.var_w_b0_dn0 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn0)), ((locals.var_w_b0_dn2 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn2)), ((locals.var_w_b0_dn4 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn4)), ((locals.var_w_b0_dn5 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn5)), ((locals.var_w_b0_dn6 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn6)), ((locals.var_w_b0_dn7 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn7)), ((locals.var_w_b0_dn8 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn8)), ((locals.var_w_b0_dn9 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn9)), ((locals.var_w_b0_dn10 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn10)), ((locals.var_w_b0_dn11 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn11)), ((locals.var_w_b0_dn14 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign24800_e20428;
        locals.var_w_sub0_dn0 = assign24800_e20428_d_n0;
        locals.var_w_sub0_dn2 = assign24800_e20428_d_n2;
        locals.var_w_sub0_dn4 = assign24800_e20428_d_n4;
        locals.var_w_sub0_dn5 = assign24800_e20428_d_n5;
        locals.var_w_sub0_dn6 = assign24800_e20428_d_n6;
        locals.var_w_sub0_dn7 = assign24800_e20428_d_n7;
        locals.var_w_sub0_dn8 = assign24800_e20428_d_n8;
        locals.var_w_sub0_dn9 = assign24800_e20428_d_n9;
        locals.var_w_sub0_dn10 = assign24800_e20428_d_n10;
        locals.var_w_sub0_dn11 = assign24800_e20428_d_n11;
        locals.var_w_sub0_dn14 = assign24800_e20428_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign24810_e20445, assign24810_e20445_d_n0, assign24810_e20445_d_n2, assign24810_e20445_d_n4, assign24810_e20445_d_n5, assign24810_e20445_d_n6, assign24810_e20445_d_n7, assign24810_e20445_d_n8, assign24810_e20445_d_n9, assign24810_e20445_d_n10, assign24810_e20445_d_n11, assign24810_e20445_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        let assign24810_e20437: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24810_e20439: f64 = (assign24810_e20437 * locals.var_w_sub0);
        let assign24810_e20441: f64 = (assign24810_e20439 + locals.var_vbscl__blk437);
        let assign24810_e20443: f64 = (assign24810_e20441 - locals.var_vbi_dep);
        (assign24810_e20443, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn11)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn14)) * locals.var_w_sub0) + (assign24810_e20437 * locals.var_w_sub0_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24810_e20445;
        locals.var_phi_j0_dep_dn0 = assign24810_e20445_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24810_e20445_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24810_e20445_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24810_e20445_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24810_e20445_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24810_e20445_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24810_e20445_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24810_e20445_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24810_e20445_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24810_e20445_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24810_e20445_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24820_e20460, assign24820_e20460_d_n0, assign24820_e20460_d_n2, assign24820_e20460_d_n4, assign24820_e20460_d_n5, assign24820_e20460_d_n6, assign24820_e20460_d_n7, assign24820_e20460_d_n8, assign24820_e20460_d_n9, assign24820_e20460_d_n10, assign24820_e20460_d_n11, assign24820_e20460_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        let assign24820_e20454: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24820_e20456: f64 = (assign24820_e20454 * locals.var_w_b0);
        let assign24820_e20458: f64 = (assign24820_e20456 + locals.var_phi_j0_dep);
        (assign24820_e20458, (((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn0)) + locals.var_phi_j0_dep_dn0), (((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn2)) + locals.var_phi_j0_dep_dn2), (((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn4)) + locals.var_phi_j0_dep_dn4), (((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn5)) + locals.var_phi_j0_dep_dn5), (((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn6)) + locals.var_phi_j0_dep_dn6), (((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn7)) + locals.var_phi_j0_dep_dn7), (((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn8)) + locals.var_phi_j0_dep_dn8), (((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn9)) + locals.var_phi_j0_dep_dn9), (((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn10)) + locals.var_phi_j0_dep_dn10), (((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn11)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn11)) + locals.var_phi_j0_dep_dn11), (((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn14)) * locals.var_w_b0) + (assign24820_e20454 * locals.var_w_b0_dn14)) + locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24820_e20460;
        locals.var_phi_b0_dep_dn0 = assign24820_e20460_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24820_e20460_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24820_e20460_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24820_e20460_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24820_e20460_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24820_e20460_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24820_e20460_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24820_e20460_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24820_e20460_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24820_e20460_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24820_e20460_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24830_e20469, assign24830_e20469_d_n0, assign24830_e20469_d_n2, assign24830_e20469_d_n4, assign24830_e20469_d_n5, assign24830_e20469_d_n6, assign24830_e20469_d_n7, assign24830_e20469_d_n8, assign24830_e20469_d_n9, assign24830_e20469_d_n10, assign24830_e20469_d_n11, assign24830_e20469_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    }
};
        locals.var_phi_j0_dep_acc = assign24830_e20469;
        locals.var_phi_j0_dep_acc_dn0 = assign24830_e20469_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24830_e20469_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24830_e20469_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24830_e20469_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24830_e20469_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24830_e20469_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24830_e20469_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24830_e20469_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24830_e20469_d_n10;
        locals.var_phi_j0_dep_acc_dn11 = assign24830_e20469_d_n11;
        locals.var_phi_j0_dep_acc_dn14 = assign24830_e20469_d_n14;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let assign24840_e20472: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign24840_e20472;
        locals.var_guard578_rv = 0.0;

        let (assign24850_e20483,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard578 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24850_e20483;
        locals.var_depmode_rv = 0.0;

        let (assign24860_e20495,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard561 == 0.0)) && (locals.var_guard578 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24860_e20495;
        locals.var_depmode_rv = 0.0;

        let (assign24870_e20508, assign24870_e20508_d_n0, assign24870_e20508_d_n2, assign24870_e20508_d_n4, assign24870_e20508_d_n5, assign24870_e20508_d_n6, assign24870_e20508_d_n7, assign24870_e20508_d_n8, assign24870_e20508_d_n9, assign24870_e20508_d_n10, assign24870_e20508_d_n11, assign24870_e20508_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24870_e20502: f64 = (-locals.var_pb2n);
        let assign24870_e20504: f64 = (assign24870_e20502 + locals.var_vbscl__blk437);
        let assign24870_e20505: f64 = (locals.var_psbmax - assign24870_e20504);
        let assign24870_e20506: f64 = (locals.var_c_2esi_q_ndepm * assign24870_e20505);
        (assign24870_e20506, ((locals.var_c_2esi_q_ndepm_dn0 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn0 - ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk437_dn0)))), ((locals.var_c_2esi_q_ndepm_dn2 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn2 - ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk437_dn2)))), ((locals.var_c_2esi_q_ndepm_dn4 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn4 - ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk437_dn4)))), ((locals.var_c_2esi_q_ndepm_dn5 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn5 - ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk437_dn5)))), ((locals.var_c_2esi_q_ndepm_dn6 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn6 - ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk437_dn6)))), ((locals.var_c_2esi_q_ndepm_dn7 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn7 - ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk437_dn7)))), ((locals.var_c_2esi_q_ndepm_dn8 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn8 - ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk437_dn8)))), ((locals.var_c_2esi_q_ndepm_dn9 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn9 - ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk437_dn9)))), ((locals.var_c_2esi_q_ndepm_dn10 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn10 - ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk437_dn10)))), ((locals.var_c_2esi_q_ndepm_dn11 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn11 - ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk437_dn11)))), ((locals.var_c_2esi_q_ndepm_dn14 * assign24870_e20505) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn14 - ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24870_e20508;
        locals.var_t1_dn0 = assign24870_e20508_d_n0;
        locals.var_t1_dn2 = assign24870_e20508_d_n2;
        locals.var_t1_dn4 = assign24870_e20508_d_n4;
        locals.var_t1_dn5 = assign24870_e20508_d_n5;
        locals.var_t1_dn6 = assign24870_e20508_d_n6;
        locals.var_t1_dn7 = assign24870_e20508_d_n7;
        locals.var_t1_dn8 = assign24870_e20508_d_n8;
        locals.var_t1_dn9 = assign24870_e20508_d_n9;
        locals.var_t1_dn10 = assign24870_e20508_d_n10;
        locals.var_t1_dn11 = assign24870_e20508_d_n11;
        locals.var_t1_dn14 = assign24870_e20508_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24880_e20511: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign24880_e20511;
        locals.var_guard579_rv = 0.0;

        let (assign24890_e20527, assign24890_e20527_d_n0, assign24890_e20527_d_n2, assign24890_e20527_d_n4, assign24890_e20527_d_n5, assign24890_e20527_d_n6, assign24890_e20527_d_n7, assign24890_e20527_d_n8, assign24890_e20527_d_n9, assign24890_e20527_d_n10, assign24890_e20527_d_n11, assign24890_e20527_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard579 != 0.0)) {
        let assign24890_e20518: f64 = (-locals.var_pb2n);
        let assign24890_e20520: f64 = (assign24890_e20518 + locals.var_vbscl__blk437);
        let assign24890_e20522: f64 = (locals.var_t1).sqrt();
        let assign24890_e20524: f64 = (assign24890_e20522 / locals.var_cox);
        let assign24890_e20525: f64 = (assign24890_e20520 - assign24890_e20524);
        (assign24890_e20525, (((-locals.var_pb2n_dn0) + locals.var_vbscl__blk437_dn0) - ((((locals.var_t1_dn0 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn0)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn2) + locals.var_vbscl__blk437_dn2) - ((((locals.var_t1_dn2 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn2)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn4) + locals.var_vbscl__blk437_dn4) - ((((locals.var_t1_dn4 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn5) + locals.var_vbscl__blk437_dn5) - ((((locals.var_t1_dn5 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn6) + locals.var_vbscl__blk437_dn6) - ((((locals.var_t1_dn6 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn7) + locals.var_vbscl__blk437_dn7) - ((((locals.var_t1_dn7 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn8) + locals.var_vbscl__blk437_dn8) - ((((locals.var_t1_dn8 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn9) + locals.var_vbscl__blk437_dn9) - ((((locals.var_t1_dn9 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn10) + locals.var_vbscl__blk437_dn10) - ((((locals.var_t1_dn10 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn11) + locals.var_vbscl__blk437_dn11) - ((((locals.var_t1_dn11 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn14) + locals.var_vbscl__blk437_dn14) - ((((locals.var_t1_dn14 / (2.0 * assign24890_e20522)) * locals.var_cox) - (assign24890_e20522 * locals.var_cox_dn14)) / (locals.var_cox * locals.var_cox))),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn11, locals.var_vthn_dn14,)
    }
};
        locals.var_vthn = assign24890_e20527;
        locals.var_vthn_dn0 = assign24890_e20527_d_n0;
        locals.var_vthn_dn2 = assign24890_e20527_d_n2;
        locals.var_vthn_dn4 = assign24890_e20527_d_n4;
        locals.var_vthn_dn5 = assign24890_e20527_d_n5;
        locals.var_vthn_dn6 = assign24890_e20527_d_n6;
        locals.var_vthn_dn7 = assign24890_e20527_d_n7;
        locals.var_vthn_dn8 = assign24890_e20527_d_n8;
        locals.var_vthn_dn9 = assign24890_e20527_d_n9;
        locals.var_vthn_dn10 = assign24890_e20527_d_n10;
        locals.var_vthn_dn11 = assign24890_e20527_d_n11;
        locals.var_vthn_dn14 = assign24890_e20527_d_n14;
        locals.var_vthn_rv = 0.0;

        let (assign24900_e20539, assign24900_e20539_d_n0, assign24900_e20539_d_n2, assign24900_e20539_d_n4, assign24900_e20539_d_n5, assign24900_e20539_d_n6, assign24900_e20539_d_n7, assign24900_e20539_d_n8, assign24900_e20539_d_n9, assign24900_e20539_d_n10, assign24900_e20539_d_n11, assign24900_e20539_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard579 == 0.0)) {
        let assign24900_e20535: f64 = (-locals.var_pb2n);
        let assign24900_e20537: f64 = (assign24900_e20535 + locals.var_vbscl__blk437);
        (assign24900_e20537, ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk437_dn0), ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk437_dn2), ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk437_dn4), ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk437_dn5), ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk437_dn6), ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk437_dn7), ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk437_dn8), ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk437_dn9), ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk437_dn10), ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk437_dn11), ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk437_dn14),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn11, locals.var_vthn_dn14,)
    }
};
        locals.var_vthn = assign24900_e20539;
        locals.var_vthn_dn0 = assign24900_e20539_d_n0;
        locals.var_vthn_dn2 = assign24900_e20539_d_n2;
        locals.var_vthn_dn4 = assign24900_e20539_d_n4;
        locals.var_vthn_dn5 = assign24900_e20539_d_n5;
        locals.var_vthn_dn6 = assign24900_e20539_d_n6;
        locals.var_vthn_dn7 = assign24900_e20539_d_n7;
        locals.var_vthn_dn8 = assign24900_e20539_d_n8;
        locals.var_vthn_dn9 = assign24900_e20539_d_n9;
        locals.var_vthn_dn10 = assign24900_e20539_d_n10;
        locals.var_vthn_dn11 = assign24900_e20539_d_n11;
        locals.var_vthn_dn14 = assign24900_e20539_d_n14;
        locals.var_vthn_rv = 0.0;

        let assign24910_e20542: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign24910_e20542;
        locals.var_guard580_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        locals: &mut StampLocals,
    ) {
        let (assign24920_e20550, assign24920_e20550_d_n0, assign24920_e20550_d_n2, assign24920_e20550_d_n4, assign24920_e20550_d_n5, assign24920_e20550_d_n6, assign24920_e20550_d_n7, assign24920_e20550_d_n8, assign24920_e20550_d_n9, assign24920_e20550_d_n10, assign24920_e20550_d_n11, assign24920_e20550_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 != 0.0)) {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn11, locals.var_phi_j0_dep_acc_dn14,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign24920_e20550;
        locals.var_phi_j0_dep_dn0 = assign24920_e20550_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24920_e20550_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24920_e20550_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24920_e20550_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24920_e20550_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24920_e20550_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24920_e20550_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24920_e20550_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24920_e20550_d_n10;
        locals.var_phi_j0_dep_dn11 = assign24920_e20550_d_n11;
        locals.var_phi_j0_dep_dn14 = assign24920_e20550_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24930_e20558, assign24930_e20558_d_n0, assign24930_e20558_d_n2, assign24930_e20558_d_n4, assign24930_e20558_d_n5, assign24930_e20558_d_n6, assign24930_e20558_d_n7, assign24930_e20558_d_n8, assign24930_e20558_d_n9, assign24930_e20558_d_n10, assign24930_e20558_d_n11, assign24930_e20558_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign24930_e20558;
        locals.var_phi_b0_dep_dn0 = assign24930_e20558_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24930_e20558_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24930_e20558_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24930_e20558_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24930_e20558_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24930_e20558_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24930_e20558_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24930_e20558_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24930_e20558_d_n10;
        locals.var_phi_b0_dep_dn11 = assign24930_e20558_d_n11;
        locals.var_phi_b0_dep_dn14 = assign24930_e20558_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24940_e20579, assign24940_e20579_d_n0, assign24940_e20579_d_n2, assign24940_e20579_d_n4, assign24940_e20579_d_n5, assign24940_e20579_d_n6, assign24940_e20579_d_n7, assign24940_e20579_d_n8, assign24940_e20579_d_n9, assign24940_e20579_d_n10, assign24940_e20579_d_n11, assign24940_e20579_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 != 0.0)) {
        let assign24940_e20566: f64 = (locals.var_afact * locals.var_vgp);
        let assign24940_e20568: f64 = (assign24940_e20566 * locals.var_vgp);
        let assign24940_e20569: f64 = (assign24940_e20568).ln();
        let assign24940_e20573: f64 = (2.0 / locals.var_vgp);
        let assign24940_e20574: f64 = (locals.var_beta + assign24940_e20573);
        let assign24940_e20575: f64 = (assign24940_e20569 / assign24940_e20574);
        let assign24940_e20577: f64 = (assign24940_e20575 + locals.var_phi_b0_dep);
        (assign24940_e20577, (((((((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn0)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn0), (((((((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn2)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn2), (((((((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn4)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn4), (((((((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn5)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn5), (((((((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn6)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn6), (((((((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn7)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn7), (((((((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn8)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn8), (((((((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn9)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn9), (((((((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn10)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn10), (((((((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn11)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn11), (((((((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign24940_e20566 * locals.var_vgp_dn14)) / assign24940_e20568) * assign24940_e20574) - (assign24940_e20569 * (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))))) / (assign24940_e20574 * assign24940_e20574)) + locals.var_phi_b0_dep_dn14),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign24940_e20579;
        locals.var_phi_s0_dep_ini_dn0 = assign24940_e20579_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24940_e20579_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24940_e20579_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24940_e20579_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24940_e20579_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24940_e20579_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24940_e20579_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24940_e20579_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24940_e20579_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign24940_e20579_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign24940_e20579_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24950_e20583: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        let assign24950_e20584: f64 = if locals.var_phi_s0_dep_ini < assign24950_e20583 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign24950_e20584;
        locals.var_guard581_rv = 0.0;

        let (assign24960_e20596, assign24960_e20596_d_n0, assign24960_e20596_d_n2, assign24960_e20596_d_n4, assign24960_e20596_d_n5, assign24960_e20596_d_n6, assign24960_e20596_d_n7, assign24960_e20596_d_n8, assign24960_e20596_d_n9, assign24960_e20596_d_n10, assign24960_e20596_d_n11, assign24960_e20596_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 != 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign24960_e20594: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        (assign24960_e20594, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn11, locals.var_vds_maxb0_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign24960_e20596;
        locals.var_phi_s0_dep_ini_dn0 = assign24960_e20596_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24960_e20596_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24960_e20596_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24960_e20596_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24960_e20596_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24960_e20596_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24960_e20596_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24960_e20596_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24960_e20596_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign24960_e20596_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign24960_e20596_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24970_e20599: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign24970_e20599;
        locals.var_guard582_rv = 0.0;

        let (assign24980_e20610, assign24980_e20610_d_n0, assign24980_e20610_d_n2, assign24980_e20610_d_n4, assign24980_e20610_d_n5, assign24980_e20610_d_n6, assign24980_e20610_d_n7, assign24980_e20610_d_n8, assign24980_e20610_d_n9, assign24980_e20610_d_n10, assign24980_e20610_d_n11, assign24980_e20610_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign24980_e20610;
        locals.var_phi_s0_dep_ini_dn0 = assign24980_e20610_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24980_e20610_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24980_e20610_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24980_e20610_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24980_e20610_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24980_e20610_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24980_e20610_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24980_e20610_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24980_e20610_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign24980_e20610_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign24980_e20610_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24990_e20613: f64 = if locals.var_vgp > locals.var_vthn { 1.0 } else { 0.0 };
        locals.var_guard583 = assign24990_e20613;
        locals.var_guard583_rv = 0.0;

        let (assign25000_e20634, assign25000_e20634_d_n0, assign25000_e20634_d_n2, assign25000_e20634_d_n4, assign25000_e20634_d_n5, assign25000_e20634_d_n6, assign25000_e20634_d_n7, assign25000_e20634_d_n8, assign25000_e20634_d_n9, assign25000_e20634_d_n10, assign25000_e20634_d_n11, assign25000_e20634_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25000_e20626: f64 = (-2.0);
        let assign25000_e20628: f64 = (assign25000_e20626 * locals.var_afact);
        let assign25000_e20630: f64 = (assign25000_e20628 * locals.var_vgp);
        let assign25000_e20632: f64 = (assign25000_e20630 + locals.var_beta);
        (assign25000_e20632, ((((assign25000_e20626 * locals.var_afact_dn0) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn0)) + locals.var_beta_dn0), ((((assign25000_e20626 * locals.var_afact_dn2) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn2)) + locals.var_beta_dn2), ((((assign25000_e20626 * locals.var_afact_dn4) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn4)) + locals.var_beta_dn4), ((((assign25000_e20626 * locals.var_afact_dn5) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn5)) + locals.var_beta_dn5), ((((assign25000_e20626 * locals.var_afact_dn6) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn6)) + locals.var_beta_dn6), ((((assign25000_e20626 * locals.var_afact_dn7) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn7)) + locals.var_beta_dn7), ((((assign25000_e20626 * locals.var_afact_dn8) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn8)) + locals.var_beta_dn8), ((((assign25000_e20626 * locals.var_afact_dn9) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn9)) + locals.var_beta_dn9), ((((assign25000_e20626 * locals.var_afact_dn10) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn10)) + locals.var_beta_dn10), ((((assign25000_e20626 * locals.var_afact_dn11) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn11)) + locals.var_beta_dn11), ((((assign25000_e20626 * locals.var_afact_dn14) * locals.var_vgp) + (assign25000_e20628 * locals.var_vgp_dn14)) + locals.var_beta_dn14),)
    } else {
        (locals.var_bfact, locals.var_bfact_dn0, locals.var_bfact_dn2, locals.var_bfact_dn4, locals.var_bfact_dn5, locals.var_bfact_dn6, locals.var_bfact_dn7, locals.var_bfact_dn8, locals.var_bfact_dn9, locals.var_bfact_dn10, locals.var_bfact_dn11, locals.var_bfact_dn14,)
    }
};
        locals.var_bfact = assign25000_e20634;
        locals.var_bfact_dn0 = assign25000_e20634_d_n0;
        locals.var_bfact_dn2 = assign25000_e20634_d_n2;
        locals.var_bfact_dn4 = assign25000_e20634_d_n4;
        locals.var_bfact_dn5 = assign25000_e20634_d_n5;
        locals.var_bfact_dn6 = assign25000_e20634_d_n6;
        locals.var_bfact_dn7 = assign25000_e20634_d_n7;
        locals.var_bfact_dn8 = assign25000_e20634_d_n8;
        locals.var_bfact_dn9 = assign25000_e20634_d_n9;
        locals.var_bfact_dn10 = assign25000_e20634_d_n10;
        locals.var_bfact_dn11 = assign25000_e20634_d_n11;
        locals.var_bfact_dn14 = assign25000_e20634_d_n14;
        locals.var_bfact_rv = 0.0;

        let (assign25010_e20656, assign25010_e20656_d_n0, assign25010_e20656_d_n2, assign25010_e20656_d_n4, assign25010_e20656_d_n5, assign25010_e20656_d_n6, assign25010_e20656_d_n7, assign25010_e20656_d_n8, assign25010_e20656_d_n9, assign25010_e20656_d_n10, assign25010_e20656_d_n11, assign25010_e20656_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25010_e20648: f64 = (locals.var_afact * locals.var_vgp);
        let assign25010_e20650: f64 = (assign25010_e20648 * locals.var_vgp);
        let assign25010_e20653: f64 = (locals.var_beta * locals.var_phi_b0_dep);
        let assign25010_e20654: f64 = (assign25010_e20650 - assign25010_e20653);
        (assign25010_e20654, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign25010_e20648 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
        locals.var_cfact = assign25010_e20656;
        locals.var_cfact_dn0 = assign25010_e20656_d_n0;
        locals.var_cfact_dn2 = assign25010_e20656_d_n2;
        locals.var_cfact_dn4 = assign25010_e20656_d_n4;
        locals.var_cfact_dn5 = assign25010_e20656_d_n5;
        locals.var_cfact_dn6 = assign25010_e20656_d_n6;
        locals.var_cfact_dn7 = assign25010_e20656_d_n7;
        locals.var_cfact_dn8 = assign25010_e20656_d_n8;
        locals.var_cfact_dn9 = assign25010_e20656_d_n9;
        locals.var_cfact_dn10 = assign25010_e20656_d_n10;
        locals.var_cfact_dn11 = assign25010_e20656_d_n11;
        locals.var_cfact_dn14 = assign25010_e20656_d_n14;
        locals.var_cfact_rv = 0.0;

        let (assign25020_e20670, assign25020_e20670_d_n0, assign25020_e20670_d_n2, assign25020_e20670_d_n4, assign25020_e20670_d_n5, assign25020_e20670_d_n6, assign25020_e20670_d_n7, assign25020_e20670_d_n8, assign25020_e20670_d_n9, assign25020_e20670_d_n10, assign25020_e20670_d_n11, assign25020_e20670_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn11, locals.var_phi_b0_dep_old_dn14,)
    }
};
        locals.var_phi_b0_dep_old = assign25020_e20670;
        locals.var_phi_b0_dep_old_dn0 = assign25020_e20670_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25020_e20670_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25020_e20670_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25020_e20670_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25020_e20670_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25020_e20670_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25020_e20670_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25020_e20670_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25020_e20670_d_n10;
        locals.var_phi_b0_dep_old_dn11 = assign25020_e20670_d_n11;
        locals.var_phi_b0_dep_old_dn14 = assign25020_e20670_d_n14;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25030_e20700, assign25030_e20700_d_n0, assign25030_e20700_d_n2, assign25030_e20700_d_n4, assign25030_e20700_d_n5, assign25030_e20700_d_n6, assign25030_e20700_d_n7, assign25030_e20700_d_n8, assign25030_e20700_d_n9, assign25030_e20700_d_n10, assign25030_e20700_d_n11, assign25030_e20700_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25030_e20683: f64 = (-locals.var_bfact);
        let assign25030_e20686: f64 = (locals.var_bfact * locals.var_bfact);
        let assign25030_e20689: f64 = (4.0 * locals.var_afact);
        let assign25030_e20691: f64 = (assign25030_e20689 * locals.var_cfact);
        let assign25030_e20692: f64 = (assign25030_e20686 - assign25030_e20691);
        let assign25030_e20693: f64 = (assign25030_e20692).sqrt();
        let assign25030_e20694: f64 = (assign25030_e20683 + assign25030_e20693);
        let assign25030_e20696: f64 = (assign25030_e20694 / 2.0);
        let assign25030_e20698: f64 = (assign25030_e20696 / locals.var_afact);
        (assign25030_e20698, ((((((-locals.var_bfact_dn0) + ((((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn0))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + ((((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn2))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + ((((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn4))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + ((((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn5))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + ((((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn6))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + ((((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn7))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + ((((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn8))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + ((((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn9))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + ((((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn10))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + ((((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn11))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + ((((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign25030_e20689 * locals.var_cfact_dn14))) / (2.0 * assign25030_e20693))) / 2.0) * locals.var_afact) - (assign25030_e20696 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25030_e20700;
        locals.var_phi_s0_dep_ini_dn0 = assign25030_e20700_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25030_e20700_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25030_e20700_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25030_e20700_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25030_e20700_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25030_e20700_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25030_e20700_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25030_e20700_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25030_e20700_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25030_e20700_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25030_e20700_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign25040_e20704: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        let assign25040_e20705: f64 = if locals.var_phi_s0_dep_ini > assign25040_e20704 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign25040_e20705;
        locals.var_guard584_rv = 0.0;

        let (assign25050_e20723, assign25050_e20723_d_n0, assign25050_e20723_d_n2, assign25050_e20723_d_n4, assign25050_e20723_d_n5, assign25050_e20723_d_n6, assign25050_e20723_d_n7, assign25050_e20723_d_n8, assign25050_e20723_d_n9, assign25050_e20723_d_n10, assign25050_e20723_d_n11, assign25050_e20723_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign25050_e20721: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        (assign25050_e20721, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25050_e20723;
        locals.var_phi_s0_dep_ini_dn0 = assign25050_e20723_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25050_e20723_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25050_e20723_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25050_e20723_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25050_e20723_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25050_e20723_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25050_e20723_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25050_e20723_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25050_e20723_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25050_e20723_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25050_e20723_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let (assign25060_e20742, assign25060_e20742_d_n0, assign25060_e20742_d_n2, assign25060_e20742_d_n4, assign25060_e20742_d_n5, assign25060_e20742_d_n6, assign25060_e20742_d_n7, assign25060_e20742_d_n8, assign25060_e20742_d_n9, assign25060_e20742_d_n10, assign25060_e20742_d_n11, assign25060_e20742_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25060_e20738: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25060_e20739: f64 = (locals.var_c_2esipq_ndepm * assign25060_e20738);
        let assign25060_e20740: f64 = (assign25060_e20739).sqrt();
        (assign25060_e20740, (((locals.var_c_2esipq_ndepm_dn0 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn2 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn4 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn5 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn6 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn7 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn8 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn9 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn10 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn11 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_ini_dn11))) / (2.0 * assign25060_e20740)), (((locals.var_c_2esipq_ndepm_dn14 * assign25060_e20738) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_ini_dn14))) / (2.0 * assign25060_e20740)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
        locals.var_w_s0 = assign25060_e20742;
        locals.var_w_s0_dn0 = assign25060_e20742_d_n0;
        locals.var_w_s0_dn2 = assign25060_e20742_d_n2;
        locals.var_w_s0_dn4 = assign25060_e20742_d_n4;
        locals.var_w_s0_dn5 = assign25060_e20742_d_n5;
        locals.var_w_s0_dn6 = assign25060_e20742_d_n6;
        locals.var_w_s0_dn7 = assign25060_e20742_d_n7;
        locals.var_w_s0_dn8 = assign25060_e20742_d_n8;
        locals.var_w_s0_dn9 = assign25060_e20742_d_n9;
        locals.var_w_s0_dn10 = assign25060_e20742_d_n10;
        locals.var_w_s0_dn11 = assign25060_e20742_d_n11;
        locals.var_w_s0_dn14 = assign25060_e20742_d_n14;
        locals.var_w_s0_rv = 0.0;

        let (assign25070_e20761, assign25070_e20761_d_n0, assign25070_e20761_d_n2, assign25070_e20761_d_n4, assign25070_e20761_d_n5, assign25070_e20761_d_n6, assign25070_e20761_d_n7, assign25070_e20761_d_n8, assign25070_e20761_d_n9, assign25070_e20761_d_n10, assign25070_e20761_d_n11, assign25070_e20761_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25070_e20757: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25070_e20758: f64 = (locals.var_c_2esipq_ndepm * assign25070_e20757);
        let assign25070_e20759: f64 = (assign25070_e20758).sqrt();
        (assign25070_e20759, (((locals.var_c_2esipq_ndepm_dn0 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn2 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn4 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn5 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn6 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn7 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn8 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn9 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn10 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn11 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign25070_e20759)), (((locals.var_c_2esipq_ndepm_dn14 * assign25070_e20757) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign25070_e20759)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign25070_e20761;
        locals.var_w_b0_dn0 = assign25070_e20761_d_n0;
        locals.var_w_b0_dn2 = assign25070_e20761_d_n2;
        locals.var_w_b0_dn4 = assign25070_e20761_d_n4;
        locals.var_w_b0_dn5 = assign25070_e20761_d_n5;
        locals.var_w_b0_dn6 = assign25070_e20761_d_n6;
        locals.var_w_b0_dn7 = assign25070_e20761_d_n7;
        locals.var_w_b0_dn8 = assign25070_e20761_d_n8;
        locals.var_w_b0_dn9 = assign25070_e20761_d_n9;
        locals.var_w_b0_dn10 = assign25070_e20761_d_n10;
        locals.var_w_b0_dn11 = assign25070_e20761_d_n11;
        locals.var_w_b0_dn14 = assign25070_e20761_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign25080_e20764: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25080_e20766: f64 = if assign25080_e20764 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard585 = assign25080_e20766;
        locals.var_guard585_rv = 0.0;

        let (assign25090_e20782,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign25090_e20782;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        locals: &mut StampLocals,
    ) {
        let mut assign25100_loop_guard: usize = 0;
        while {
            let assign25100_cond_e20799: f64 = (150.0 + 1.0);
            let assign25100_cond_e20801: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_lp_s0 <= assign25100_cond_e20799)) { 1.0 } else { 0.0 };
            assign25100_cond_e20801 != 0.0
        } {
            assign25100_loop_guard += 1;
            assert!(assign25100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25100_body0_e20821, assign25100_body0_e20821_d_n0, assign25100_body0_e20821_d_n2, assign25100_body0_e20821_d_n4, assign25100_body0_e20821_d_n5, assign25100_body0_e20821_d_n6, assign25100_body0_e20821_d_n7, assign25100_body0_e20821_d_n8, assign25100_body0_e20821_d_n9, assign25100_body0_e20821_d_n10, assign25100_body0_e20821_d_n11, assign25100_body0_e20821_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body0_e20817: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25100_body0_e20819: f64 = (assign25100_body0_e20817 - locals.var_uc_depthn);
        (assign25100_body0_e20819, ((locals.var_w_s0_dn0 + locals.var_w_b0_dn0) - locals.var_uc_depthn_dn0), ((locals.var_w_s0_dn2 + locals.var_w_b0_dn2) - locals.var_uc_depthn_dn2), ((locals.var_w_s0_dn4 + locals.var_w_b0_dn4) - locals.var_uc_depthn_dn4), ((locals.var_w_s0_dn5 + locals.var_w_b0_dn5) - locals.var_uc_depthn_dn5), ((locals.var_w_s0_dn6 + locals.var_w_b0_dn6) - locals.var_uc_depthn_dn6), ((locals.var_w_s0_dn7 + locals.var_w_b0_dn7) - locals.var_uc_depthn_dn7), ((locals.var_w_s0_dn8 + locals.var_w_b0_dn8) - locals.var_uc_depthn_dn8), ((locals.var_w_s0_dn9 + locals.var_w_b0_dn9) - locals.var_uc_depthn_dn9), ((locals.var_w_s0_dn10 + locals.var_w_b0_dn10) - locals.var_uc_depthn_dn10), ((locals.var_w_s0_dn11 + locals.var_w_b0_dn11) - locals.var_uc_depthn_dn11), ((locals.var_w_s0_dn14 + locals.var_w_b0_dn14) - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_y0, locals.var_y0_dn0, locals.var_y0_dn2, locals.var_y0_dn4, locals.var_y0_dn5, locals.var_y0_dn6, locals.var_y0_dn7, locals.var_y0_dn8, locals.var_y0_dn9, locals.var_y0_dn10, locals.var_y0_dn11, locals.var_y0_dn14,)
    }
};
            locals.var_y0 = assign25100_body0_e20821;
            locals.var_y0_dn0 = assign25100_body0_e20821_d_n0;
            locals.var_y0_dn2 = assign25100_body0_e20821_d_n2;
            locals.var_y0_dn4 = assign25100_body0_e20821_d_n4;
            locals.var_y0_dn5 = assign25100_body0_e20821_d_n5;
            locals.var_y0_dn6 = assign25100_body0_e20821_d_n6;
            locals.var_y0_dn7 = assign25100_body0_e20821_d_n7;
            locals.var_y0_dn8 = assign25100_body0_e20821_d_n8;
            locals.var_y0_dn9 = assign25100_body0_e20821_d_n9;
            locals.var_y0_dn10 = assign25100_body0_e20821_d_n10;
            locals.var_y0_dn11 = assign25100_body0_e20821_d_n11;
            locals.var_y0_dn14 = assign25100_body0_e20821_d_n14;
            locals.var_y0_rv = 0.0;
            let (assign25100_body1_e20855, assign25100_body1_e20855_d_n0, assign25100_body1_e20855_d_n2, assign25100_body1_e20855_d_n4, assign25100_body1_e20855_d_n5, assign25100_body1_e20855_d_n6, assign25100_body1_e20855_d_n7, assign25100_body1_e20855_d_n8, assign25100_body1_e20855_d_n9, assign25100_body1_e20855_d_n10, assign25100_body1_e20855_d_n11, assign25100_body1_e20855_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body1_e20837: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25100_body1_e20839: f64 = (assign25100_body1_e20837 / locals.var_w_s0);
        let assign25100_body1_e20842: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25100_body1_e20847: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25100_body1_e20848: f64 = (locals.var_ndepmpnsub / assign25100_body1_e20847);
        let assign25100_body1_e20849: f64 = (1.0 - assign25100_body1_e20848);
        let assign25100_body1_e20850: f64 = (assign25100_body1_e20842 * assign25100_body1_e20849);
        let assign25100_body1_e20852: f64 = (assign25100_body1_e20850 / locals.var_w_b0);
        let assign25100_body1_e20853: f64 = (assign25100_body1_e20839 + assign25100_body1_e20852);
        (assign25100_body1_e20853, (((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn0)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn0 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn0)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn0)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn2)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn2 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn2)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn2)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn4)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn4 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn4)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn4)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn5)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn5 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn5)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn5)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn6)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn6 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn6)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn6)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn7)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn7 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn7)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn7)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn8)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn8 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn8)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn8)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn9)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn9 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn9)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn9)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn10)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn10 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn10)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn10)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn11)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn11 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn11)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn11)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25100_body1_e20837 * locals.var_w_s0_dn14)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25100_body1_e20849) + (assign25100_body1_e20842 * (-(((locals.var_ndepmpnsub_dn14 * assign25100_body1_e20847) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn14)) / (assign25100_body1_e20847 * assign25100_body1_e20847))))) * locals.var_w_b0) - (assign25100_body1_e20850 * locals.var_w_b0_dn14)) / (locals.var_w_b0 * locals.var_w_b0))),)
    } else {
        (locals.var_dydpsm, locals.var_dydpsm_dn0, locals.var_dydpsm_dn2, locals.var_dydpsm_dn4, locals.var_dydpsm_dn5, locals.var_dydpsm_dn6, locals.var_dydpsm_dn7, locals.var_dydpsm_dn8, locals.var_dydpsm_dn9, locals.var_dydpsm_dn10, locals.var_dydpsm_dn11, locals.var_dydpsm_dn14,)
    }
};
            locals.var_dydpsm = assign25100_body1_e20855;
            locals.var_dydpsm_dn0 = assign25100_body1_e20855_d_n0;
            locals.var_dydpsm_dn2 = assign25100_body1_e20855_d_n2;
            locals.var_dydpsm_dn4 = assign25100_body1_e20855_d_n4;
            locals.var_dydpsm_dn5 = assign25100_body1_e20855_d_n5;
            locals.var_dydpsm_dn6 = assign25100_body1_e20855_d_n6;
            locals.var_dydpsm_dn7 = assign25100_body1_e20855_d_n7;
            locals.var_dydpsm_dn8 = assign25100_body1_e20855_d_n8;
            locals.var_dydpsm_dn9 = assign25100_body1_e20855_d_n9;
            locals.var_dydpsm_dn10 = assign25100_body1_e20855_d_n10;
            locals.var_dydpsm_dn11 = assign25100_body1_e20855_d_n11;
            locals.var_dydpsm_dn14 = assign25100_body1_e20855_d_n14;
            locals.var_dydpsm_rv = 0.0;
            let assign25100_body2_e20858: f64 = (locals.var_y0 / locals.var_dydpsm);
            let assign25100_body2_e20859: f64 = (assign25100_body2_e20858).abs();
            let assign25100_body2_e20861: f64 = if assign25100_body2_e20859 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard586 = assign25100_body2_e20861;
            locals.var_guard586_rv = 0.0;
            let (assign25100_body3_e20891, assign25100_body3_e20891_d_n0, assign25100_body3_e20891_d_n2, assign25100_body3_e20891_d_n4, assign25100_body3_e20891_d_n5, assign25100_body3_e20891_d_n6, assign25100_body3_e20891_d_n7, assign25100_body3_e20891_d_n8, assign25100_body3_e20891_d_n9, assign25100_body3_e20891_d_n10, assign25100_body3_e20891_d_n11, assign25100_body3_e20891_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign25100_body3_e20881: f64 = (locals.var_y0 / locals.var_dydpsm);
        let (assign25100_body3_e20887,) = {
            if (assign25100_body3_e20881 >= 0.0) {
                (1.0,)
            } else {
                let assign25100_body3_e20886: f64 = (-1.0);
                (assign25100_body3_e20886,)
            }
        };
        let assign25100_body3_e20888: f64 = (0.5 * assign25100_body3_e20887);
        let assign25100_body3_e20889: f64 = (locals.var_phi_b0_dep - assign25100_body3_e20888);
        (assign25100_body3_e20889, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
            locals.var_phi_b0_dep = assign25100_body3_e20891;
            locals.var_phi_b0_dep_dn0 = assign25100_body3_e20891_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25100_body3_e20891_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25100_body3_e20891_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25100_body3_e20891_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25100_body3_e20891_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25100_body3_e20891_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25100_body3_e20891_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25100_body3_e20891_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25100_body3_e20891_d_n10;
            locals.var_phi_b0_dep_dn11 = assign25100_body3_e20891_d_n11;
            locals.var_phi_b0_dep_dn14 = assign25100_body3_e20891_d_n14;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25100_body4_e20914, assign25100_body4_e20914_d_n0, assign25100_body4_e20914_d_n2, assign25100_body4_e20914_d_n4, assign25100_body4_e20914_d_n5, assign25100_body4_e20914_d_n6, assign25100_body4_e20914_d_n7, assign25100_body4_e20914_d_n8, assign25100_body4_e20914_d_n9, assign25100_body4_e20914_d_n10, assign25100_body4_e20914_d_n11, assign25100_body4_e20914_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign25100_body4_e20911: f64 = (locals.var_y0 / locals.var_dydpsm);
        let assign25100_body4_e20912: f64 = (locals.var_phi_b0_dep - assign25100_body4_e20911);
        (assign25100_body4_e20912, (locals.var_phi_b0_dep_dn0 - (((locals.var_y0_dn0 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn0)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn2 - (((locals.var_y0_dn2 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn2)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn4 - (((locals.var_y0_dn4 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn4)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn5 - (((locals.var_y0_dn5 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn5)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn6 - (((locals.var_y0_dn6 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn6)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn7 - (((locals.var_y0_dn7 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn7)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn8 - (((locals.var_y0_dn8 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn8)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn9 - (((locals.var_y0_dn9 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn9)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn10 - (((locals.var_y0_dn10 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn10)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn11 - (((locals.var_y0_dn11 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn11)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn14 - (((locals.var_y0_dn14 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn14)) / (locals.var_dydpsm * locals.var_dydpsm))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
            locals.var_phi_b0_dep = assign25100_body4_e20914;
            locals.var_phi_b0_dep_dn0 = assign25100_body4_e20914_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25100_body4_e20914_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25100_body4_e20914_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25100_body4_e20914_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25100_body4_e20914_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25100_body4_e20914_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25100_body4_e20914_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25100_body4_e20914_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25100_body4_e20914_d_n10;
            locals.var_phi_b0_dep_dn11 = assign25100_body4_e20914_d_n11;
            locals.var_phi_b0_dep_dn14 = assign25100_body4_e20914_d_n14;
            locals.var_phi_b0_dep_rv = 0.0;
            let assign25100_body5_e20917: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk437);
            let assign25100_body5_e20919: f64 = (assign25100_body5_e20917 + locals.var_vbi_dep);
            let assign25100_body5_e20922: f64 = (10.0 * 2.220446049250313e-16);
            let assign25100_body5_e20923: f64 = if assign25100_body5_e20919 < assign25100_body5_e20922 { 1.0 } else { 0.0 };
            locals.var_guard587 = assign25100_body5_e20923;
            locals.var_guard587_rv = 0.0;
            let (assign25100_body6_e20947, assign25100_body6_e20947_d_n0, assign25100_body6_e20947_d_n2, assign25100_body6_e20947_d_n4, assign25100_body6_e20947_d_n5, assign25100_body6_e20947_d_n6, assign25100_body6_e20947_d_n7, assign25100_body6_e20947_d_n8, assign25100_body6_e20947_d_n9, assign25100_body6_e20947_d_n10, assign25100_body6_e20947_d_n11, assign25100_body6_e20947_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign25100_body6_e20941: f64 = (locals.var_vbscl__blk437 - locals.var_vbi_dep);
        let assign25100_body6_e20944: f64 = (10.0 * 2.220446049250313e-16);
        let assign25100_body6_e20945: f64 = (assign25100_body6_e20941 + assign25100_body6_e20944);
        (assign25100_body6_e20945, (locals.var_vbscl__blk437_dn0 - locals.var_vbi_dep_dn0), (locals.var_vbscl__blk437_dn2 - locals.var_vbi_dep_dn2), (locals.var_vbscl__blk437_dn4 - locals.var_vbi_dep_dn4), (locals.var_vbscl__blk437_dn5 - locals.var_vbi_dep_dn5), (locals.var_vbscl__blk437_dn6 - locals.var_vbi_dep_dn6), (locals.var_vbscl__blk437_dn7 - locals.var_vbi_dep_dn7), (locals.var_vbscl__blk437_dn8 - locals.var_vbi_dep_dn8), (locals.var_vbscl__blk437_dn9 - locals.var_vbi_dep_dn9), (locals.var_vbscl__blk437_dn10 - locals.var_vbi_dep_dn10), (locals.var_vbscl__blk437_dn11 - locals.var_vbi_dep_dn11), (locals.var_vbscl__blk437_dn14 - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
            locals.var_phi_b0_dep = assign25100_body6_e20947;
            locals.var_phi_b0_dep_dn0 = assign25100_body6_e20947_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25100_body6_e20947_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25100_body6_e20947_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25100_body6_e20947_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25100_body6_e20947_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25100_body6_e20947_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25100_body6_e20947_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25100_body6_e20947_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25100_body6_e20947_d_n10;
            locals.var_phi_b0_dep_dn11 = assign25100_body6_e20947_d_n11;
            locals.var_phi_b0_dep_dn14 = assign25100_body6_e20947_d_n14;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25100_body7_e20971, assign25100_body7_e20971_d_n0, assign25100_body7_e20971_d_n2, assign25100_body7_e20971_d_n4, assign25100_body7_e20971_d_n5, assign25100_body7_e20971_d_n6, assign25100_body7_e20971_d_n7, assign25100_body7_e20971_d_n8, assign25100_body7_e20971_d_n9, assign25100_body7_e20971_d_n10, assign25100_body7_e20971_d_n11, assign25100_body7_e20971_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body7_e20963: f64 = (locals.var_afact * locals.var_vgp);
        let assign25100_body7_e20965: f64 = (assign25100_body7_e20963 * locals.var_vgp);
        let assign25100_body7_e20968: f64 = (locals.var_beta * locals.var_phi_b0_dep);
        let assign25100_body7_e20969: f64 = (assign25100_body7_e20965 - assign25100_body7_e20968);
        (assign25100_body7_e20969, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign25100_body7_e20963 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
            locals.var_cfact = assign25100_body7_e20971;
            locals.var_cfact_dn0 = assign25100_body7_e20971_d_n0;
            locals.var_cfact_dn2 = assign25100_body7_e20971_d_n2;
            locals.var_cfact_dn4 = assign25100_body7_e20971_d_n4;
            locals.var_cfact_dn5 = assign25100_body7_e20971_d_n5;
            locals.var_cfact_dn6 = assign25100_body7_e20971_d_n6;
            locals.var_cfact_dn7 = assign25100_body7_e20971_d_n7;
            locals.var_cfact_dn8 = assign25100_body7_e20971_d_n8;
            locals.var_cfact_dn9 = assign25100_body7_e20971_d_n9;
            locals.var_cfact_dn10 = assign25100_body7_e20971_d_n10;
            locals.var_cfact_dn11 = assign25100_body7_e20971_d_n11;
            locals.var_cfact_dn14 = assign25100_body7_e20971_d_n14;
            locals.var_cfact_rv = 0.0;
            let (assign25100_body8_e20995, assign25100_body8_e20995_d_n0, assign25100_body8_e20995_d_n2, assign25100_body8_e20995_d_n4, assign25100_body8_e20995_d_n5, assign25100_body8_e20995_d_n6, assign25100_body8_e20995_d_n7, assign25100_body8_e20995_d_n8, assign25100_body8_e20995_d_n9, assign25100_body8_e20995_d_n10, assign25100_body8_e20995_d_n11, assign25100_body8_e20995_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body8_e20987: f64 = (locals.var_bfact * locals.var_bfact);
        let assign25100_body8_e20990: f64 = (4.0 * locals.var_afact);
        let assign25100_body8_e20992: f64 = (assign25100_body8_e20990 * locals.var_cfact);
        let assign25100_body8_e20993: f64 = (assign25100_body8_e20987 - assign25100_body8_e20992);
        (assign25100_body8_e20993, (((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn0))), (((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn2))), (((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn4))), (((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn5))), (((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn6))), (((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn7))), (((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn8))), (((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn9))), (((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn10))), (((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn11))), (((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign25100_body8_e20990 * locals.var_cfact_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign25100_body8_e20995;
            locals.var_t1_dn0 = assign25100_body8_e20995_d_n0;
            locals.var_t1_dn2 = assign25100_body8_e20995_d_n2;
            locals.var_t1_dn4 = assign25100_body8_e20995_d_n4;
            locals.var_t1_dn5 = assign25100_body8_e20995_d_n5;
            locals.var_t1_dn6 = assign25100_body8_e20995_d_n6;
            locals.var_t1_dn7 = assign25100_body8_e20995_d_n7;
            locals.var_t1_dn8 = assign25100_body8_e20995_d_n8;
            locals.var_t1_dn9 = assign25100_body8_e20995_d_n9;
            locals.var_t1_dn10 = assign25100_body8_e20995_d_n10;
            locals.var_t1_dn11 = assign25100_body8_e20995_d_n11;
            locals.var_t1_dn14 = assign25100_body8_e20995_d_n14;
            locals.var_t1_rv = 0.0;
            let assign25100_body9_e20998: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard588 = assign25100_body9_e20998;
            locals.var_guard588_rv = 0.0;
            let (assign25100_body10_e21024, assign25100_body10_e21024_d_n0, assign25100_body10_e21024_d_n2, assign25100_body10_e21024_d_n4, assign25100_body10_e21024_d_n5, assign25100_body10_e21024_d_n6, assign25100_body10_e21024_d_n7, assign25100_body10_e21024_d_n8, assign25100_body10_e21024_d_n9, assign25100_body10_e21024_d_n10, assign25100_body10_e21024_d_n11, assign25100_body10_e21024_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign25100_body10_e21015: f64 = (-locals.var_bfact);
        let assign25100_body10_e21017: f64 = (locals.var_t1).sqrt();
        let assign25100_body10_e21018: f64 = (assign25100_body10_e21015 + assign25100_body10_e21017);
        let assign25100_body10_e21020: f64 = (assign25100_body10_e21018 / 2.0);
        let assign25100_body10_e21022: f64 = (assign25100_body10_e21020 / locals.var_afact);
        (assign25100_body10_e21022, ((((((-locals.var_bfact_dn0) + (locals.var_t1_dn0 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + (locals.var_t1_dn2 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + (locals.var_t1_dn4 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + (locals.var_t1_dn5 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + (locals.var_t1_dn6 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + (locals.var_t1_dn7 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + (locals.var_t1_dn8 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + (locals.var_t1_dn9 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + (locals.var_t1_dn10 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + (locals.var_t1_dn11 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + (locals.var_t1_dn14 / (2.0 * assign25100_body10_e21017))) / 2.0) * locals.var_afact) - (assign25100_body10_e21020 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
            locals.var_phi_s0_dep_ini = assign25100_body10_e21024;
            locals.var_phi_s0_dep_ini_dn0 = assign25100_body10_e21024_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25100_body10_e21024_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25100_body10_e21024_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25100_body10_e21024_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25100_body10_e21024_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25100_body10_e21024_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25100_body10_e21024_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25100_body10_e21024_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25100_body10_e21024_d_n10;
            locals.var_phi_s0_dep_ini_dn11 = assign25100_body10_e21024_d_n11;
            locals.var_phi_s0_dep_ini_dn14 = assign25100_body10_e21024_d_n14;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let (assign25100_body11_e21048, assign25100_body11_e21048_d_n0, assign25100_body11_e21048_d_n2, assign25100_body11_e21048_d_n4, assign25100_body11_e21048_d_n5, assign25100_body11_e21048_d_n6, assign25100_body11_e21048_d_n7, assign25100_body11_e21048_d_n8, assign25100_body11_e21048_d_n9, assign25100_body11_e21048_d_n10, assign25100_body11_e21048_d_n11, assign25100_body11_e21048_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard588 == 0.0)) {
        let assign25100_body11_e21042: f64 = (-locals.var_bfact);
        let assign25100_body11_e21044: f64 = (assign25100_body11_e21042 / 2.0);
        let assign25100_body11_e21046: f64 = (assign25100_body11_e21044 / locals.var_afact);
        (assign25100_body11_e21046, (((((-locals.var_bfact_dn0) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn2) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn4) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn5) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn6) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn7) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn8) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn9) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn10) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn11) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn14) / 2.0) * locals.var_afact) - (assign25100_body11_e21044 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
            locals.var_phi_s0_dep_ini = assign25100_body11_e21048;
            locals.var_phi_s0_dep_ini_dn0 = assign25100_body11_e21048_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25100_body11_e21048_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25100_body11_e21048_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25100_body11_e21048_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25100_body11_e21048_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25100_body11_e21048_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25100_body11_e21048_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25100_body11_e21048_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25100_body11_e21048_d_n10;
            locals.var_phi_s0_dep_ini_dn11 = assign25100_body11_e21048_d_n11;
            locals.var_phi_s0_dep_ini_dn14 = assign25100_body11_e21048_d_n14;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let assign25100_body12_e21051: f64 = if locals.var_phi_s0_dep_ini > locals.var_psbmax { 1.0 } else { 0.0 };
            locals.var_guard589 = assign25100_body12_e21051;
            locals.var_guard589_rv = 0.0;
            let (assign25100_body13_e21069, assign25100_body13_e21069_d_n0, assign25100_body13_e21069_d_n2, assign25100_body13_e21069_d_n4, assign25100_body13_e21069_d_n5, assign25100_body13_e21069_d_n6, assign25100_body13_e21069_d_n7, assign25100_body13_e21069_d_n8, assign25100_body13_e21069_d_n9, assign25100_body13_e21069_d_n10, assign25100_body13_e21069_d_n11, assign25100_body13_e21069_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard589 != 0.0)) {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
            locals.var_phi_s0_dep_ini = assign25100_body13_e21069;
            locals.var_phi_s0_dep_ini_dn0 = assign25100_body13_e21069_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25100_body13_e21069_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25100_body13_e21069_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25100_body13_e21069_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25100_body13_e21069_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25100_body13_e21069_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25100_body13_e21069_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25100_body13_e21069_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25100_body13_e21069_d_n10;
            locals.var_phi_s0_dep_ini_dn11 = assign25100_body13_e21069_d_n11;
            locals.var_phi_s0_dep_ini_dn14 = assign25100_body13_e21069_d_n14;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let assign25100_body14_e21072: f64 = if locals.var_phi_s0_dep_ini > locals.var_phi_b0_dep { 1.0 } else { 0.0 };
            locals.var_guard590 = assign25100_body14_e21072;
            locals.var_guard590_rv = 0.0;
            let (assign25100_body15_e21092, assign25100_body15_e21092_d_n0, assign25100_body15_e21092_d_n2, assign25100_body15_e21092_d_n4, assign25100_body15_e21092_d_n5, assign25100_body15_e21092_d_n6, assign25100_body15_e21092_d_n7, assign25100_body15_e21092_d_n8, assign25100_body15_e21092_d_n9, assign25100_body15_e21092_d_n10, assign25100_body15_e21092_d_n11, assign25100_body15_e21092_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25100_body15_e21090: f64 = (locals.var_phi_b0_dep - locals.var_ps_conv23);
        (assign25100_body15_e21090, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
            locals.var_phi_s0_dep_ini = assign25100_body15_e21092;
            locals.var_phi_s0_dep_ini_dn0 = assign25100_body15_e21092_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25100_body15_e21092_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25100_body15_e21092_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25100_body15_e21092_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25100_body15_e21092_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25100_body15_e21092_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25100_body15_e21092_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25100_body15_e21092_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25100_body15_e21092_d_n10;
            locals.var_phi_s0_dep_ini_dn11 = assign25100_body15_e21092_d_n11;
            locals.var_phi_s0_dep_ini_dn14 = assign25100_body15_e21092_d_n14;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let (assign25100_body16_e21112,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25100_body16_e21110: f64 = (150.0 + 1.0);
        (assign25100_body16_e21110,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25100_body16_e21112;
            locals.var_lp_s0_rv = 0.0;
            let (assign25100_body17_e21133, assign25100_body17_e21133_d_n0, assign25100_body17_e21133_d_n2, assign25100_body17_e21133_d_n4, assign25100_body17_e21133_d_n5, assign25100_body17_e21133_d_n6, assign25100_body17_e21133_d_n7, assign25100_body17_e21133_d_n8, assign25100_body17_e21133_d_n9, assign25100_body17_e21133_d_n10, assign25100_body17_e21133_d_n11, assign25100_body17_e21133_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body17_e21129: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25100_body17_e21130: f64 = (locals.var_c_2esipq_ndepm * assign25100_body17_e21129);
        let assign25100_body17_e21131: f64 = (assign25100_body17_e21130).sqrt();
        (assign25100_body17_e21131, (((locals.var_c_2esipq_ndepm_dn0 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn2 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn4 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn5 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn6 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn7 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn8 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn9 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn10 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn11 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_ini_dn11))) / (2.0 * assign25100_body17_e21131)), (((locals.var_c_2esipq_ndepm_dn14 * assign25100_body17_e21129) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_ini_dn14))) / (2.0 * assign25100_body17_e21131)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
            locals.var_w_s0 = assign25100_body17_e21133;
            locals.var_w_s0_dn0 = assign25100_body17_e21133_d_n0;
            locals.var_w_s0_dn2 = assign25100_body17_e21133_d_n2;
            locals.var_w_s0_dn4 = assign25100_body17_e21133_d_n4;
            locals.var_w_s0_dn5 = assign25100_body17_e21133_d_n5;
            locals.var_w_s0_dn6 = assign25100_body17_e21133_d_n6;
            locals.var_w_s0_dn7 = assign25100_body17_e21133_d_n7;
            locals.var_w_s0_dn8 = assign25100_body17_e21133_d_n8;
            locals.var_w_s0_dn9 = assign25100_body17_e21133_d_n9;
            locals.var_w_s0_dn10 = assign25100_body17_e21133_d_n10;
            locals.var_w_s0_dn11 = assign25100_body17_e21133_d_n11;
            locals.var_w_s0_dn14 = assign25100_body17_e21133_d_n14;
            locals.var_w_s0_rv = 0.0;
            let (assign25100_body18_e21159, assign25100_body18_e21159_d_n0, assign25100_body18_e21159_d_n2, assign25100_body18_e21159_d_n4, assign25100_body18_e21159_d_n5, assign25100_body18_e21159_d_n6, assign25100_body18_e21159_d_n7, assign25100_body18_e21159_d_n8, assign25100_body18_e21159_d_n9, assign25100_body18_e21159_d_n10, assign25100_body18_e21159_d_n11, assign25100_body18_e21159_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body18_e21149: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25100_body18_e21151: f64 = (assign25100_body18_e21149 + locals.var_vbscl__blk437);
        let assign25100_body18_e21153: f64 = (assign25100_body18_e21151 - locals.var_vbi_dep);
        let assign25100_body18_e21156: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25100_body18_e21157: f64 = (assign25100_body18_e21153 / assign25100_body18_e21156);
        (assign25100_body18_e21157, (((((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn0)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn2)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn4)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn5)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn6)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn7)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn8)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn9)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn10)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn11 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn11)) / (assign25100_body18_e21156 * assign25100_body18_e21156)), (((((((locals.var_ndepmpnsub_dn14 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14) * assign25100_body18_e21156) - (assign25100_body18_e21153 * locals.var_ndepmpnsub_dn14)) / (assign25100_body18_e21156 * assign25100_body18_e21156)),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
            locals.var_phi_j0_dep = assign25100_body18_e21159;
            locals.var_phi_j0_dep_dn0 = assign25100_body18_e21159_d_n0;
            locals.var_phi_j0_dep_dn2 = assign25100_body18_e21159_d_n2;
            locals.var_phi_j0_dep_dn4 = assign25100_body18_e21159_d_n4;
            locals.var_phi_j0_dep_dn5 = assign25100_body18_e21159_d_n5;
            locals.var_phi_j0_dep_dn6 = assign25100_body18_e21159_d_n6;
            locals.var_phi_j0_dep_dn7 = assign25100_body18_e21159_d_n7;
            locals.var_phi_j0_dep_dn8 = assign25100_body18_e21159_d_n8;
            locals.var_phi_j0_dep_dn9 = assign25100_body18_e21159_d_n9;
            locals.var_phi_j0_dep_dn10 = assign25100_body18_e21159_d_n10;
            locals.var_phi_j0_dep_dn11 = assign25100_body18_e21159_d_n11;
            locals.var_phi_j0_dep_dn14 = assign25100_body18_e21159_d_n14;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign25100_body19_e21180, assign25100_body19_e21180_d_n0, assign25100_body19_e21180_d_n2, assign25100_body19_e21180_d_n4, assign25100_body19_e21180_d_n5, assign25100_body19_e21180_d_n6, assign25100_body19_e21180_d_n7, assign25100_body19_e21180_d_n8, assign25100_body19_e21180_d_n9, assign25100_body19_e21180_d_n10, assign25100_body19_e21180_d_n11, assign25100_body19_e21180_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body19_e21176: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25100_body19_e21177: f64 = (locals.var_c_2esipq_ndepm * assign25100_body19_e21176);
        let assign25100_body19_e21178: f64 = (assign25100_body19_e21177).sqrt();
        (assign25100_body19_e21178, (((locals.var_c_2esipq_ndepm_dn0 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn2 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn4 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn5 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn6 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn7 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn8 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn9 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn10 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn11 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign25100_body19_e21178)), (((locals.var_c_2esipq_ndepm_dn14 * assign25100_body19_e21176) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign25100_body19_e21178)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
            locals.var_w_b0 = assign25100_body19_e21180;
            locals.var_w_b0_dn0 = assign25100_body19_e21180_d_n0;
            locals.var_w_b0_dn2 = assign25100_body19_e21180_d_n2;
            locals.var_w_b0_dn4 = assign25100_body19_e21180_d_n4;
            locals.var_w_b0_dn5 = assign25100_body19_e21180_d_n5;
            locals.var_w_b0_dn6 = assign25100_body19_e21180_d_n6;
            locals.var_w_b0_dn7 = assign25100_body19_e21180_d_n7;
            locals.var_w_b0_dn8 = assign25100_body19_e21180_d_n8;
            locals.var_w_b0_dn9 = assign25100_body19_e21180_d_n9;
            locals.var_w_b0_dn10 = assign25100_body19_e21180_d_n10;
            locals.var_w_b0_dn11 = assign25100_body19_e21180_d_n11;
            locals.var_w_b0_dn14 = assign25100_body19_e21180_d_n14;
            locals.var_w_b0_rv = 0.0;
            let assign25100_body20_e21183: f64 = (locals.var_phi_b0_dep - locals.var_phi_b0_dep_old);
            let assign25100_body20_e21184: f64 = (assign25100_body20_e21183).abs();
            let assign25100_body20_e21186: f64 = if assign25100_body20_e21184 <= 1e-8 { 1.0 } else { 0.0 };
            locals.var_guard591 = assign25100_body20_e21186;
            locals.var_guard591_rv = 0.0;
            let (assign25100_body21_e21206,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign25100_body21_e21204: f64 = (150.0 + 1.0);
        (assign25100_body21_e21204,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25100_body21_e21206;
            locals.var_lp_s0_rv = 0.0;
            let (assign25100_body22_e21222, assign25100_body22_e21222_d_n0, assign25100_body22_e21222_d_n2, assign25100_body22_e21222_d_n4, assign25100_body22_e21222_d_n5, assign25100_body22_e21222_d_n6, assign25100_body22_e21222_d_n7, assign25100_body22_e21222_d_n8, assign25100_body22_e21222_d_n9, assign25100_body22_e21222_d_n10, assign25100_body22_e21222_d_n11, assign25100_body22_e21222_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn11, locals.var_phi_b0_dep_old_dn14,)
    }
};
            locals.var_phi_b0_dep_old = assign25100_body22_e21222;
            locals.var_phi_b0_dep_old_dn0 = assign25100_body22_e21222_d_n0;
            locals.var_phi_b0_dep_old_dn2 = assign25100_body22_e21222_d_n2;
            locals.var_phi_b0_dep_old_dn4 = assign25100_body22_e21222_d_n4;
            locals.var_phi_b0_dep_old_dn5 = assign25100_body22_e21222_d_n5;
            locals.var_phi_b0_dep_old_dn6 = assign25100_body22_e21222_d_n6;
            locals.var_phi_b0_dep_old_dn7 = assign25100_body22_e21222_d_n7;
            locals.var_phi_b0_dep_old_dn8 = assign25100_body22_e21222_d_n8;
            locals.var_phi_b0_dep_old_dn9 = assign25100_body22_e21222_d_n9;
            locals.var_phi_b0_dep_old_dn10 = assign25100_body22_e21222_d_n10;
            locals.var_phi_b0_dep_old_dn11 = assign25100_body22_e21222_d_n11;
            locals.var_phi_b0_dep_old_dn14 = assign25100_body22_e21222_d_n14;
            locals.var_phi_b0_dep_old_rv = 0.0;
            let (assign25100_body23_e21240,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25100_body23_e21238: f64 = (locals.var_lp_s0 + 1.0);
        (assign25100_body23_e21238,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25100_body23_e21240;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign25110_e21260, assign25110_e21260_d_n0, assign25110_e21260_d_n2, assign25110_e21260_d_n4, assign25110_e21260_d_n5, assign25110_e21260_d_n6, assign25110_e21260_d_n7, assign25110_e21260_d_n8, assign25110_e21260_d_n9, assign25110_e21260_d_n10, assign25110_e21260_d_n11, assign25110_e21260_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign25110_e21256: f64 = (locals.var_beta * locals.var_vbscl__blk437);
        let assign25110_e21257: f64 = (assign25110_e21256).exp();
        let assign25110_e21258: f64 = (locals.var_afact2 / assign25110_e21257);
        (assign25110_e21258, (((locals.var_afact2_dn0 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn0 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn0))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn2 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn2 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn2))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn4 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn4 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn4))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn5 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn5 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn5))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn6 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn6 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn6))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn7 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn7 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn7))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn8 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn8 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn8))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn9 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn9 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn9))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn10 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn10 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn10))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn11 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn11 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn11))))) / (assign25110_e21257 * assign25110_e21257)), (((locals.var_afact2_dn14 * assign25110_e21257) - (locals.var_afact2 * (assign25110_e21257 * ((locals.var_beta_dn14 * locals.var_vbscl__blk437) + (locals.var_beta * locals.var_vbscl__blk437_dn14))))) / (assign25110_e21257 * assign25110_e21257)),)
    } else {
        (locals.var_afact3, locals.var_afact3_dn0, locals.var_afact3_dn2, locals.var_afact3_dn4, locals.var_afact3_dn5, locals.var_afact3_dn6, locals.var_afact3_dn7, locals.var_afact3_dn8, locals.var_afact3_dn9, locals.var_afact3_dn10, locals.var_afact3_dn11, locals.var_afact3_dn14,)
    }
};
        locals.var_afact3 = assign25110_e21260;
        locals.var_afact3_dn0 = assign25110_e21260_d_n0;
        locals.var_afact3_dn2 = assign25110_e21260_d_n2;
        locals.var_afact3_dn4 = assign25110_e21260_d_n4;
        locals.var_afact3_dn5 = assign25110_e21260_d_n5;
        locals.var_afact3_dn6 = assign25110_e21260_d_n6;
        locals.var_afact3_dn7 = assign25110_e21260_d_n7;
        locals.var_afact3_dn8 = assign25110_e21260_d_n8;
        locals.var_afact3_dn9 = assign25110_e21260_d_n9;
        locals.var_afact3_dn10 = assign25110_e21260_d_n10;
        locals.var_afact3_dn11 = assign25110_e21260_d_n11;
        locals.var_afact3_dn14 = assign25110_e21260_d_n14;
        locals.var_afact3_rv = 0.0;

        let (assign25120_e21275, assign25120_e21275_d_n0, assign25120_e21275_d_n2, assign25120_e21275_d_n4, assign25120_e21275_d_n5, assign25120_e21275_d_n6, assign25120_e21275_d_n7, assign25120_e21275_d_n8, assign25120_e21275_d_n9, assign25120_e21275_d_n10, assign25120_e21275_d_n11, assign25120_e21275_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn11, locals.var_phi_b0_dep_old_dn14,)
    }
};
        locals.var_phi_b0_dep_old = assign25120_e21275;
        locals.var_phi_b0_dep_old_dn0 = assign25120_e21275_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25120_e21275_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25120_e21275_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25120_e21275_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25120_e21275_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25120_e21275_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25120_e21275_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25120_e21275_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25120_e21275_d_n10;
        locals.var_phi_b0_dep_old_dn11 = assign25120_e21275_d_n11;
        locals.var_phi_b0_dep_old_dn14 = assign25120_e21275_d_n14;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25130_e21302, assign25130_e21302_d_n0, assign25130_e21302_d_n2, assign25130_e21302_d_n4, assign25130_e21302_d_n5, assign25130_e21302_d_n6, assign25130_e21302_d_n7, assign25130_e21302_d_n8, assign25130_e21302_d_n9, assign25130_e21302_d_n10, assign25130_e21302_d_n11, assign25130_e21302_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard582 == 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign25130_e21290: f64 = (locals.var_afact3 * locals.var_vgp);
        let assign25130_e21292: f64 = (assign25130_e21290 * locals.var_vgp);
        let assign25130_e21293: f64 = (assign25130_e21292).ln();
        let assign25130_e21295: f64 = (-locals.var_beta);
        let assign25130_e21298: f64 = (2.0 / locals.var_vgp);
        let assign25130_e21299: f64 = (assign25130_e21295 + assign25130_e21298);
        let assign25130_e21300: f64 = (assign25130_e21293 / assign25130_e21299);
        (assign25130_e21300, ((((((((locals.var_afact3_dn0 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn0)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn0) + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn2 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn2)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn2) + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn4 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn4)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn4) + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn5 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn5)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn5) + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn6 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn6)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn6) + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn7 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn7)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn7) + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn8 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn8)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn8) + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn9 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn9)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn9) + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn10 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn10)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn10) + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn11 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn11)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn11) + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)), ((((((((locals.var_afact3_dn14 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn14)) * locals.var_vgp) + (assign25130_e21290 * locals.var_vgp_dn14)) / assign25130_e21292) * assign25130_e21299) - (assign25130_e21293 * ((-locals.var_beta_dn14) + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))))) / (assign25130_e21299 * assign25130_e21299)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn11, locals.var_phi_s0_dep_ini_dn14,)
    }
};
        locals.var_phi_s0_dep_ini = assign25130_e21302;
        locals.var_phi_s0_dep_ini_dn0 = assign25130_e21302_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25130_e21302_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25130_e21302_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25130_e21302_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25130_e21302_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25130_e21302_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25130_e21302_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25130_e21302_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25130_e21302_d_n10;
        locals.var_phi_s0_dep_ini_dn11 = assign25130_e21302_d_n11;
        locals.var_phi_s0_dep_ini_dn14 = assign25130_e21302_d_n14;
        locals.var_phi_s0_dep_ini_rv = 0.0;

    }
}
