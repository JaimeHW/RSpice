#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
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

        let assign23630_e32726: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign23630_e32726;

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

        let assign23670_e32769: f64 = if locals.var_t3__blk726 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign23670_e32769;

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

        let (assign23770_e32865, assign23770_e32865_d_n0, assign23770_e32865_d_n2, assign23770_e32865_d_n6, assign23770_e32865_d_n7, assign23770_e32865_d_n10, assign23770_e32865_d_n11, assign23770_e32865_d_n12, assign23770_e32865_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23770_e32862: f64 = (locals.var_t6__blk729 + locals.var_vdsz);
        let assign23770_e32863: f64 = (locals.var_t6__blk729 / assign23770_e32862);
        (assign23770_e32863, (((locals.var_t6__blk729_dn0 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn0 + locals.var_vdsz_dn0))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn2 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn2 + locals.var_vdsz_dn2))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn6 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn6 + locals.var_vdsz_dn6))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn7 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn7 + locals.var_vdsz_dn7))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn10 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn10 + locals.var_vdsz_dn10))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn11 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn11 + locals.var_vdsz_dn11))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn12 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn12 + locals.var_vdsz_dn12))) / (assign23770_e32862 * assign23770_e32862)), (((locals.var_t6__blk729_dn17 * assign23770_e32862) - (locals.var_t6__blk729 * (locals.var_t6__blk729_dn17 + locals.var_vdsz_dn17))) / (assign23770_e32862 * assign23770_e32862)),)
    } else {
        (locals.var_t9__blk731, locals.var_t9__blk731_dn0, locals.var_t9__blk731_dn2, locals.var_t9__blk731_dn6, locals.var_t9__blk731_dn7, locals.var_t9__blk731_dn10, locals.var_t9__blk731_dn11, locals.var_t9__blk731_dn12, locals.var_t9__blk731_dn17,)
    }
};
        locals.var_t9__blk731 = assign23770_e32865;
        locals.var_t9__blk731_dn0 = assign23770_e32865_d_n0;
        locals.var_t9__blk731_dn2 = assign23770_e32865_d_n2;
        locals.var_t9__blk731_dn6 = assign23770_e32865_d_n6;
        locals.var_t9__blk731_dn7 = assign23770_e32865_d_n7;
        locals.var_t9__blk731_dn10 = assign23770_e32865_d_n10;
        locals.var_t9__blk731_dn11 = assign23770_e32865_d_n11;
        locals.var_t9__blk731_dn12 = assign23770_e32865_d_n12;
        locals.var_t9__blk731_dn17 = assign23770_e32865_d_n17;

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

        let assign23800_e32891: f64 = (-34.0);
        let assign23800_e32892: f64 = if locals.var_t1__blk724 < assign23800_e32891 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign23800_e32892;

        let (assign23810_e32901, assign23810_e32901_d_n0, assign23810_e32901_d_n2, assign23810_e32901_d_n6, assign23810_e32901_d_n7, assign23810_e32901_d_n10, assign23810_e32901_d_n11, assign23810_e32901_d_n12, assign23810_e32901_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23810_e32901;
        locals.var_igate_dn0 = assign23810_e32901_d_n0;
        locals.var_igate_dn2 = assign23810_e32901_d_n2;
        locals.var_igate_dn6 = assign23810_e32901_d_n6;
        locals.var_igate_dn7 = assign23810_e32901_d_n7;
        locals.var_igate_dn10 = assign23810_e32901_d_n10;
        locals.var_igate_dn11 = assign23810_e32901_d_n11;
        locals.var_igate_dn12 = assign23810_e32901_d_n12;
        locals.var_igate_dn17 = assign23810_e32901_d_n17;

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

    }

    pub(super) fn stamp_transient_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let (assign23870_e32985, assign23870_e32985_d_n0, assign23870_e32985_d_n2, assign23870_e32985_d_n6, assign23870_e32985_d_n7, assign23870_e32985_d_n10, assign23870_e32985_d_n11, assign23870_e32985_d_n12, assign23870_e32985_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23870_e32981: f64 = (locals.var_t4__blk727 * locals.var_etun);
        let assign23870_e32983: f64 = (assign23870_e32981 * locals.var_etun);
        (assign23870_e32983, ((((locals.var_t4__blk727_dn0 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn0)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn0)), ((((locals.var_t4__blk727_dn2 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn2)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn2)), ((((locals.var_t4__blk727_dn6 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn6)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn6)), ((((locals.var_t4__blk727_dn7 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn7)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn7)), ((((locals.var_t4__blk727_dn10 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn10)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn10)), ((((locals.var_t4__blk727_dn11 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn11)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn11)), ((((locals.var_t4__blk727_dn12 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn12)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn12)), ((((locals.var_t4__blk727_dn17 * locals.var_etun) + (locals.var_t4__blk727 * locals.var_etun_dn17)) * locals.var_etun) + (assign23870_e32981 * locals.var_etun_dn17)),)
    } else {
        (locals.var_t10__blk732, locals.var_t10__blk732_dn0, locals.var_t10__blk732_dn2, locals.var_t10__blk732_dn6, locals.var_t10__blk732_dn7, locals.var_t10__blk732_dn10, locals.var_t10__blk732_dn11, locals.var_t10__blk732_dn12, locals.var_t10__blk732_dn17,)
    }
};
        locals.var_t10__blk732 = assign23870_e32985;
        locals.var_t10__blk732_dn0 = assign23870_e32985_d_n0;
        locals.var_t10__blk732_dn2 = assign23870_e32985_d_n2;
        locals.var_t10__blk732_dn6 = assign23870_e32985_d_n6;
        locals.var_t10__blk732_dn7 = assign23870_e32985_d_n7;
        locals.var_t10__blk732_dn10 = assign23870_e32985_d_n10;
        locals.var_t10__blk732_dn11 = assign23870_e32985_d_n11;
        locals.var_t10__blk732_dn12 = assign23870_e32985_d_n12;
        locals.var_t10__blk732_dn17 = assign23870_e32985_d_n17;

        let (assign23880_e32999, assign23880_e32999_d_n0, assign23880_e32999_d_n2, assign23880_e32999_d_n6, assign23880_e32999_d_n7, assign23880_e32999_d_n10, assign23880_e32999_d_n11, assign23880_e32999_d_n12, assign23880_e32999_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23880_e32995: f64 = (locals.var_t7__blk730 * locals.var_t9__blk731);
        let assign23880_e32997: f64 = (assign23880_e32995 * locals.var_t10__blk732);
        (assign23880_e32997, ((((locals.var_t7__blk730_dn0 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn0)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn0)), ((((locals.var_t7__blk730_dn2 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn2)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn2)), ((((locals.var_t7__blk730_dn6 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn6)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn6)), ((((locals.var_t7__blk730_dn7 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn7)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn7)), ((((locals.var_t7__blk730_dn10 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn10)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn10)), ((((locals.var_t7__blk730_dn11 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn11)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn11)), ((((locals.var_t7__blk730_dn12 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn12)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn12)), ((((locals.var_t7__blk730_dn17 * locals.var_t9__blk731) + (locals.var_t7__blk730 * locals.var_t9__blk731_dn17)) * locals.var_t10__blk732) + (assign23880_e32995 * locals.var_t10__blk732_dn17)),)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23880_e32999;
        locals.var_igate_dn0 = assign23880_e32999_d_n0;
        locals.var_igate_dn2 = assign23880_e32999_d_n2;
        locals.var_igate_dn6 = assign23880_e32999_d_n6;
        locals.var_igate_dn7 = assign23880_e32999_d_n7;
        locals.var_igate_dn10 = assign23880_e32999_d_n10;
        locals.var_igate_dn11 = assign23880_e32999_d_n11;
        locals.var_igate_dn12 = assign23880_e32999_d_n12;
        locals.var_igate_dn17 = assign23880_e32999_d_n17;

        let (assign23890_e33007, assign23890_e33007_d_n0, assign23890_e33007_d_n2, assign23890_e33007_d_n6, assign23890_e33007_d_n7, assign23890_e33007_d_n10, assign23890_e33007_d_n11, assign23890_e33007_d_n12, assign23890_e33007_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23890_e33007;
        locals.var_igate_dn0 = assign23890_e33007_d_n0;
        locals.var_igate_dn2 = assign23890_e33007_d_n2;
        locals.var_igate_dn6 = assign23890_e33007_d_n6;
        locals.var_igate_dn7 = assign23890_e33007_d_n7;
        locals.var_igate_dn10 = assign23890_e33007_d_n10;
        locals.var_igate_dn11 = assign23890_e33007_d_n11;
        locals.var_igate_dn12 = assign23890_e33007_d_n12;
        locals.var_igate_dn17 = assign23890_e33007_d_n17;

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

        let (assign23950_e33059, assign23950_e33059_d_n0, assign23950_e33059_d_n2, assign23950_e33059_d_n6, assign23950_e33059_d_n7, assign23950_e33059_d_n10, assign23950_e33059_d_n11, assign23950_e33059_d_n12, assign23950_e33059_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23950_e33055: f64 = (locals.var_t4__blk727 * locals.var_t2__blk725);
        let assign23950_e33057: f64 = (assign23950_e33055 * locals.var_t3__blk726);
        (assign23950_e33057, ((((locals.var_t4__blk727_dn0 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn0)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn0)), ((((locals.var_t4__blk727_dn2 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn2)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn2)), ((((locals.var_t4__blk727_dn6 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn6)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn6)), ((((locals.var_t4__blk727_dn7 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn7)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn7)), ((((locals.var_t4__blk727_dn10 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn10)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn10)), ((((locals.var_t4__blk727_dn11 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn11)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn11)), ((((locals.var_t4__blk727_dn12 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn12)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn12)), ((((locals.var_t4__blk727_dn17 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn17)) * locals.var_t3__blk726) + (assign23950_e33055 * locals.var_t3__blk726_dn17)),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn12, locals.var_igs_dn17,)
    }
};
        locals.var_igs = assign23950_e33059;
        locals.var_igs_dn0 = assign23950_e33059_d_n0;
        locals.var_igs_dn2 = assign23950_e33059_d_n2;
        locals.var_igs_dn6 = assign23950_e33059_d_n6;
        locals.var_igs_dn7 = assign23950_e33059_d_n7;
        locals.var_igs_dn10 = assign23950_e33059_d_n10;
        locals.var_igs_dn11 = assign23950_e33059_d_n11;
        locals.var_igs_dn12 = assign23950_e33059_d_n12;
        locals.var_igs_dn17 = assign23950_e33059_d_n17;

        let assign23960_e33062: f64 = if locals.var_vgs >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign23960_e33062;

        let (assign23970_e33072, assign23970_e33072_d_n0, assign23970_e33072_d_n2, assign23970_e33072_d_n6, assign23970_e33072_d_n7, assign23970_e33072_d_n10, assign23970_e33072_d_n11, assign23970_e33072_d_n12, assign23970_e33072_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard747 != 0.0)) {
        let assign23970_e33069: f64 = (-1.0);
        let assign23970_e33070: f64 = (locals.var_igs * assign23970_e33069);
        (assign23970_e33070, (locals.var_igs_dn0 * assign23970_e33069), (locals.var_igs_dn2 * assign23970_e33069), (locals.var_igs_dn6 * assign23970_e33069), (locals.var_igs_dn7 * assign23970_e33069), (locals.var_igs_dn10 * assign23970_e33069), (locals.var_igs_dn11 * assign23970_e33069), (locals.var_igs_dn12 * assign23970_e33069), (locals.var_igs_dn17 * assign23970_e33069),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn12, locals.var_igs_dn17,)
    }
};
        locals.var_igs = assign23970_e33072;
        locals.var_igs_dn0 = assign23970_e33072_d_n0;
        locals.var_igs_dn2 = assign23970_e33072_d_n2;
        locals.var_igs_dn6 = assign23970_e33072_d_n6;
        locals.var_igs_dn7 = assign23970_e33072_d_n7;
        locals.var_igs_dn10 = assign23970_e33072_d_n10;
        locals.var_igs_dn11 = assign23970_e33072_d_n11;
        locals.var_igs_dn12 = assign23970_e33072_d_n12;
        locals.var_igs_dn17 = assign23970_e33072_d_n17;

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

        let (assign24040_e33131, assign24040_e33131_d_n0, assign24040_e33131_d_n2, assign24040_e33131_d_n6, assign24040_e33131_d_n7, assign24040_e33131_d_n10, assign24040_e33131_d_n11, assign24040_e33131_d_n12, assign24040_e33131_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24040_e33127: f64 = (locals.var_t4__blk727 * locals.var_t2__blk725);
        let assign24040_e33129: f64 = (assign24040_e33127 * locals.var_t3__blk726);
        (assign24040_e33129, ((((locals.var_t4__blk727_dn0 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn0)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn0)), ((((locals.var_t4__blk727_dn2 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn2)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn2)), ((((locals.var_t4__blk727_dn6 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn6)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn6)), ((((locals.var_t4__blk727_dn7 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn7)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn7)), ((((locals.var_t4__blk727_dn10 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn10)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn10)), ((((locals.var_t4__blk727_dn11 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn11)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn11)), ((((locals.var_t4__blk727_dn12 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn12)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn12)), ((((locals.var_t4__blk727_dn17 * locals.var_t2__blk725) + (locals.var_t4__blk727 * locals.var_t2__blk725_dn17)) * locals.var_t3__blk726) + (assign24040_e33127 * locals.var_t3__blk726_dn17)),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn12, locals.var_igd_dn17,)
    }
};
        locals.var_igd = assign24040_e33131;
        locals.var_igd_dn0 = assign24040_e33131_d_n0;
        locals.var_igd_dn2 = assign24040_e33131_d_n2;
        locals.var_igd_dn6 = assign24040_e33131_d_n6;
        locals.var_igd_dn7 = assign24040_e33131_d_n7;
        locals.var_igd_dn10 = assign24040_e33131_d_n10;
        locals.var_igd_dn11 = assign24040_e33131_d_n11;
        locals.var_igd_dn12 = assign24040_e33131_d_n12;
        locals.var_igd_dn17 = assign24040_e33131_d_n17;

        let assign24050_e33134: f64 = if locals.var_t1__blk724 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign24050_e33134;

        let (assign24060_e33144, assign24060_e33144_d_n0, assign24060_e33144_d_n2, assign24060_e33144_d_n6, assign24060_e33144_d_n7, assign24060_e33144_d_n10, assign24060_e33144_d_n11, assign24060_e33144_d_n12, assign24060_e33144_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard748 != 0.0)) {
        let assign24060_e33141: f64 = (-1.0);
        let assign24060_e33142: f64 = (locals.var_igd * assign24060_e33141);
        (assign24060_e33142, (locals.var_igd_dn0 * assign24060_e33141), (locals.var_igd_dn2 * assign24060_e33141), (locals.var_igd_dn6 * assign24060_e33141), (locals.var_igd_dn7 * assign24060_e33141), (locals.var_igd_dn10 * assign24060_e33141), (locals.var_igd_dn11 * assign24060_e33141), (locals.var_igd_dn12 * assign24060_e33141), (locals.var_igd_dn17 * assign24060_e33141),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn12, locals.var_igd_dn17,)
    }
};
        locals.var_igd = assign24060_e33144;
        locals.var_igd_dn0 = assign24060_e33144_d_n0;
        locals.var_igd_dn2 = assign24060_e33144_d_n2;
        locals.var_igd_dn6 = assign24060_e33144_d_n6;
        locals.var_igd_dn7 = assign24060_e33144_d_n7;
        locals.var_igd_dn10 = assign24060_e33144_d_n10;
        locals.var_igd_dn11 = assign24060_e33144_d_n11;
        locals.var_igd_dn12 = assign24060_e33144_d_n12;
        locals.var_igd_dn17 = assign24060_e33144_d_n17;

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

        let assign24100_e33188: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign24100_e33188;

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

        let assign24140_e33213: f64 = (-34.0);
        let assign24140_e33214: f64 = if locals.var_t1__blk724 < assign24140_e33213 { 1.0 } else { 0.0 };
        locals.var_guard750 = assign24140_e33214;

    }

    pub(super) fn stamp_transient_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24150_e33221, assign24150_e33221_d_n0, assign24150_e33221_d_n2, assign24150_e33221_d_n6, assign24150_e33221_d_n7, assign24150_e33221_d_n10, assign24150_e33221_d_n11, assign24150_e33221_d_n12, assign24150_e33221_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard750 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn12, locals.var_igb_dn17,)
    }
};
        locals.var_igb = assign24150_e33221;
        locals.var_igb_dn0 = assign24150_e33221_d_n0;
        locals.var_igb_dn2 = assign24150_e33221_d_n2;
        locals.var_igb_dn6 = assign24150_e33221_d_n6;
        locals.var_igb_dn7 = assign24150_e33221_d_n7;
        locals.var_igb_dn10 = assign24150_e33221_d_n10;
        locals.var_igb_dn11 = assign24150_e33221_d_n11;
        locals.var_igb_dn12 = assign24150_e33221_d_n12;
        locals.var_igb_dn17 = assign24150_e33221_d_n17;

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

        let (assign24180_e33256, assign24180_e33256_d_n0, assign24180_e33256_d_n2, assign24180_e33256_d_n6, assign24180_e33256_d_n7, assign24180_e33256_d_n10, assign24180_e33256_d_n11, assign24180_e33256_d_n12, assign24180_e33256_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign24180_e33250: f64 = (locals.var_t3__blk726 * locals.var_etun);
        let assign24180_e33252: f64 = (assign24180_e33250 * locals.var_etun);
        let assign24180_e33254: f64 = (assign24180_e33252 * locals.var_t2__blk725);
        (assign24180_e33254, ((((((locals.var_t3__blk726_dn0 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn0)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn0)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn0)), ((((((locals.var_t3__blk726_dn2 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn2)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn2)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn2)), ((((((locals.var_t3__blk726_dn6 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn6)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn6)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn6)), ((((((locals.var_t3__blk726_dn7 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn7)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn7)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn7)), ((((((locals.var_t3__blk726_dn10 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn10)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn10)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn10)), ((((((locals.var_t3__blk726_dn11 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn11)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn11)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn11)), ((((((locals.var_t3__blk726_dn12 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn12)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn12)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn12)), ((((((locals.var_t3__blk726_dn17 * locals.var_etun) + (locals.var_t3__blk726 * locals.var_etun_dn17)) * locals.var_etun) + (assign24180_e33250 * locals.var_etun_dn17)) * locals.var_t2__blk725) + (assign24180_e33252 * locals.var_t2__blk725_dn17)),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn12, locals.var_igb_dn17,)
    }
};
        locals.var_igb = assign24180_e33256;
        locals.var_igb_dn0 = assign24180_e33256_d_n0;
        locals.var_igb_dn2 = assign24180_e33256_d_n2;
        locals.var_igb_dn6 = assign24180_e33256_d_n6;
        locals.var_igb_dn7 = assign24180_e33256_d_n7;
        locals.var_igb_dn10 = assign24180_e33256_d_n10;
        locals.var_igb_dn11 = assign24180_e33256_d_n11;
        locals.var_igb_dn12 = assign24180_e33256_d_n12;
        locals.var_igb_dn17 = assign24180_e33256_d_n17;

        let (assign24190_e33261,) = {
    if (locals.var_guard742 == 0.0) {
        (0.5,)
    } else {
        (locals.var_glpart1,)
    }
};
        locals.var_glpart1 = assign24190_e33261;

        let assign24200_e33264: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign24200_e33264;

        let (assign24210_e33268, assign24210_e33268_d_n0, assign24210_e33268_d_n2, assign24210_e33268_d_n6, assign24210_e33268_d_n7, assign24210_e33268_d_n10, assign24210_e33268_d_n11, assign24210_e33268_d_n12, assign24210_e33268_d_n17,) = {
    if (locals.var_guard758 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24210_e33268;
        locals.var_igidl_dn0 = assign24210_e33268_d_n0;
        locals.var_igidl_dn2 = assign24210_e33268_d_n2;
        locals.var_igidl_dn6 = assign24210_e33268_d_n6;
        locals.var_igidl_dn7 = assign24210_e33268_d_n7;
        locals.var_igidl_dn10 = assign24210_e33268_d_n10;
        locals.var_igidl_dn11 = assign24210_e33268_d_n11;
        locals.var_igidl_dn12 = assign24210_e33268_d_n12;
        locals.var_igidl_dn17 = assign24210_e33268_d_n17;

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

        let assign24270_e33329: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign24270_e33329;

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

        let assign24310_e33358: f64 = (-34.0);
        let assign24310_e33359: f64 = if locals.var_t0__blk755 < assign24310_e33358 { 1.0 } else { 0.0 };
        locals.var_guard760 = assign24310_e33359;

        let (assign24320_e33366, assign24320_e33366_d_n0, assign24320_e33366_d_n2, assign24320_e33366_d_n6, assign24320_e33366_d_n7, assign24320_e33366_d_n10, assign24320_e33366_d_n11, assign24320_e33366_d_n12, assign24320_e33366_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24320_e33366;
        locals.var_igidl_dn0 = assign24320_e33366_d_n0;
        locals.var_igidl_dn2 = assign24320_e33366_d_n2;
        locals.var_igidl_dn6 = assign24320_e33366_d_n6;
        locals.var_igidl_dn7 = assign24320_e33366_d_n7;
        locals.var_igidl_dn10 = assign24320_e33366_d_n10;
        locals.var_igidl_dn11 = assign24320_e33366_d_n11;
        locals.var_igidl_dn12 = assign24320_e33366_d_n12;
        locals.var_igidl_dn17 = assign24320_e33366_d_n17;

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

        let (assign24350_e33403, assign24350_e33403_d_n0, assign24350_e33403_d_n2, assign24350_e33403_d_n6, assign24350_e33403_d_n7, assign24350_e33403_d_n10, assign24350_e33403_d_n11, assign24350_e33403_d_n12, assign24350_e33403_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard760 == 0.0)) {
        let assign24350_e33397: f64 = (locals.var_t2__blk752 * locals.var_egidl);
        let assign24350_e33399: f64 = (assign24350_e33397 * locals.var_egidl);
        let assign24350_e33401: f64 = (assign24350_e33399 * locals.var_t1__blk751);
        (assign24350_e33401, ((((((locals.var_t2__blk752_dn0 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn0)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn0)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn0)), ((((((locals.var_t2__blk752_dn2 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn2)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn2)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn2)), ((((((locals.var_t2__blk752_dn6 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn6)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn6)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn6)), ((((((locals.var_t2__blk752_dn7 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn7)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn7)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn7)), ((((((locals.var_t2__blk752_dn10 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn10)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn10)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn10)), ((((((locals.var_t2__blk752_dn11 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn11)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn11)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn11)), ((((((locals.var_t2__blk752_dn12 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn12)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn12)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn12)), ((((((locals.var_t2__blk752_dn17 * locals.var_egidl) + (locals.var_t2__blk752 * locals.var_egidl_dn17)) * locals.var_egidl) + (assign24350_e33397 * locals.var_egidl_dn17)) * locals.var_t1__blk751) + (assign24350_e33399 * locals.var_t1__blk751_dn17)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24350_e33403;
        locals.var_igidl_dn0 = assign24350_e33403_d_n0;
        locals.var_igidl_dn2 = assign24350_e33403_d_n2;
        locals.var_igidl_dn6 = assign24350_e33403_d_n6;
        locals.var_igidl_dn7 = assign24350_e33403_d_n7;
        locals.var_igidl_dn10 = assign24350_e33403_d_n10;
        locals.var_igidl_dn11 = assign24350_e33403_d_n11;
        locals.var_igidl_dn12 = assign24350_e33403_d_n12;
        locals.var_igidl_dn17 = assign24350_e33403_d_n17;

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

        let assign24370_e33413: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign24370_e33413;

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

        let (assign24410_e33449, assign24410_e33449_d_n0, assign24410_e33449_d_n2, assign24410_e33449_d_n6, assign24410_e33449_d_n7, assign24410_e33449_d_n10, assign24410_e33449_d_n11, assign24410_e33449_d_n12, assign24410_e33449_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        let assign24410_e33447: f64 = (locals.var_t4 / locals.var_t0__blk755);
        (assign24410_e33447, (((locals.var_t4_dn0 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn0)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn2 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn2)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn6 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn6)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn7 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn7)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn10 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn10)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn11 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn11)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn12 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn12)) / (locals.var_t0__blk755 * locals.var_t0__blk755)), (((locals.var_t4_dn17 * locals.var_t0__blk755) - (locals.var_t4 * locals.var_t0__blk755_dn17)) / (locals.var_t0__blk755 * locals.var_t0__blk755)),)
    } else {
        (locals.var_t5__blk756, locals.var_t5__blk756_dn0, locals.var_t5__blk756_dn2, locals.var_t5__blk756_dn6, locals.var_t5__blk756_dn7, locals.var_t5__blk756_dn10, locals.var_t5__blk756_dn11, locals.var_t5__blk756_dn12, locals.var_t5__blk756_dn17,)
    }
};
        locals.var_t5__blk756 = assign24410_e33449;
        locals.var_t5__blk756_dn0 = assign24410_e33449_d_n0;
        locals.var_t5__blk756_dn2 = assign24410_e33449_d_n2;
        locals.var_t5__blk756_dn6 = assign24410_e33449_d_n6;
        locals.var_t5__blk756_dn7 = assign24410_e33449_d_n7;
        locals.var_t5__blk756_dn10 = assign24410_e33449_d_n10;
        locals.var_t5__blk756_dn11 = assign24410_e33449_d_n11;
        locals.var_t5__blk756_dn12 = assign24410_e33449_d_n12;
        locals.var_t5__blk756_dn17 = assign24410_e33449_d_n17;

        let (assign24420_e33458, assign24420_e33458_d_n0, assign24420_e33458_d_n2, assign24420_e33458_d_n6, assign24420_e33458_d_n7, assign24420_e33458_d_n10, assign24420_e33458_d_n11, assign24420_e33458_d_n12, assign24420_e33458_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        let assign24420_e33456: f64 = (locals.var_igidl * locals.var_t5__blk756);
        (assign24420_e33456, ((locals.var_igidl_dn0 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn0)), ((locals.var_igidl_dn2 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn2)), ((locals.var_igidl_dn6 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn6)), ((locals.var_igidl_dn7 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn7)), ((locals.var_igidl_dn10 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn10)), ((locals.var_igidl_dn11 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn11)), ((locals.var_igidl_dn12 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn12)), ((locals.var_igidl_dn17 * locals.var_t5__blk756) + (locals.var_igidl * locals.var_t5__blk756_dn17)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24420_e33458;
        locals.var_igidl_dn0 = assign24420_e33458_d_n0;
        locals.var_igidl_dn2 = assign24420_e33458_d_n2;
        locals.var_igidl_dn6 = assign24420_e33458_d_n6;
        locals.var_igidl_dn7 = assign24420_e33458_d_n7;
        locals.var_igidl_dn10 = assign24420_e33458_d_n10;
        locals.var_igidl_dn11 = assign24420_e33458_d_n11;
        locals.var_igidl_dn12 = assign24420_e33458_d_n12;
        locals.var_igidl_dn17 = assign24420_e33458_d_n17;

        let (assign24430_e33466, assign24430_e33466_d_n0, assign24430_e33466_d_n2, assign24430_e33466_d_n6, assign24430_e33466_d_n7, assign24430_e33466_d_n10, assign24430_e33466_d_n11, assign24430_e33466_d_n12, assign24430_e33466_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24430_e33466;
        locals.var_igidl_dn0 = assign24430_e33466_d_n0;
        locals.var_igidl_dn2 = assign24430_e33466_d_n2;
        locals.var_igidl_dn6 = assign24430_e33466_d_n6;
        locals.var_igidl_dn7 = assign24430_e33466_d_n7;
        locals.var_igidl_dn10 = assign24430_e33466_d_n10;
        locals.var_igidl_dn11 = assign24430_e33466_d_n11;
        locals.var_igidl_dn12 = assign24430_e33466_d_n12;
        locals.var_igidl_dn17 = assign24430_e33466_d_n17;

        let assign24440_e33469: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign24440_e33469;

        let (assign24450_e33473, assign24450_e33473_d_n0, assign24450_e33473_d_n2, assign24450_e33473_d_n6, assign24450_e33473_d_n7, assign24450_e33473_d_n10, assign24450_e33473_d_n11, assign24450_e33473_d_n12, assign24450_e33473_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24450_e33473;
        locals.var_igisl_dn0 = assign24450_e33473_d_n0;
        locals.var_igisl_dn2 = assign24450_e33473_d_n2;
        locals.var_igisl_dn6 = assign24450_e33473_d_n6;
        locals.var_igisl_dn7 = assign24450_e33473_d_n7;
        locals.var_igisl_dn10 = assign24450_e33473_d_n10;
        locals.var_igisl_dn11 = assign24450_e33473_d_n11;
        locals.var_igisl_dn12 = assign24450_e33473_d_n12;
        locals.var_igisl_dn17 = assign24450_e33473_d_n17;

    }

    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign24510_e33537: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign24510_e33537;

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

        let assign24550_e33566: f64 = (-34.0);
        let assign24550_e33567: f64 = if locals.var_t0__blk766 < assign24550_e33566 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign24550_e33567;

        let (assign24560_e33574, assign24560_e33574_d_n0, assign24560_e33574_d_n2, assign24560_e33574_d_n6, assign24560_e33574_d_n7, assign24560_e33574_d_n10, assign24560_e33574_d_n11, assign24560_e33574_d_n12, assign24560_e33574_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard771 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24560_e33574;
        locals.var_igisl_dn0 = assign24560_e33574_d_n0;
        locals.var_igisl_dn2 = assign24560_e33574_d_n2;
        locals.var_igisl_dn6 = assign24560_e33574_d_n6;
        locals.var_igisl_dn7 = assign24560_e33574_d_n7;
        locals.var_igisl_dn10 = assign24560_e33574_d_n10;
        locals.var_igisl_dn11 = assign24560_e33574_d_n11;
        locals.var_igisl_dn12 = assign24560_e33574_d_n12;
        locals.var_igisl_dn17 = assign24560_e33574_d_n17;

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

        let (assign24600_e33621, assign24600_e33621_d_n0, assign24600_e33621_d_n2, assign24600_e33621_d_n6, assign24600_e33621_d_n7, assign24600_e33621_d_n10, assign24600_e33621_d_n11, assign24600_e33621_d_n12, assign24600_e33621_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard771 == 0.0)) {
        let assign24600_e33615: f64 = (locals.var_t2__blk763 * locals.var_egisl);
        let assign24600_e33617: f64 = (assign24600_e33615 * locals.var_egisl);
        let assign24600_e33619: f64 = (assign24600_e33617 * locals.var_t1__blk762);
        (assign24600_e33619, ((((((locals.var_t2__blk763_dn0 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn0)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn0)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn0)), ((((((locals.var_t2__blk763_dn2 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn2)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn2)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn2)), ((((((locals.var_t2__blk763_dn6 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn6)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn6)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn6)), ((((((locals.var_t2__blk763_dn7 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn7)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn7)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn7)), ((((((locals.var_t2__blk763_dn10 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn10)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn10)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn10)), ((((((locals.var_t2__blk763_dn11 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn11)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn11)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn11)), ((((((locals.var_t2__blk763_dn12 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn12)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn12)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn12)), ((((((locals.var_t2__blk763_dn17 * locals.var_egisl) + (locals.var_t2__blk763 * locals.var_egisl_dn17)) * locals.var_egisl) + (assign24600_e33615 * locals.var_egisl_dn17)) * locals.var_t1__blk762) + (assign24600_e33617 * locals.var_t1__blk762_dn17)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24600_e33621;
        locals.var_igisl_dn0 = assign24600_e33621_d_n0;
        locals.var_igisl_dn2 = assign24600_e33621_d_n2;
        locals.var_igisl_dn6 = assign24600_e33621_d_n6;
        locals.var_igisl_dn7 = assign24600_e33621_d_n7;
        locals.var_igisl_dn10 = assign24600_e33621_d_n10;
        locals.var_igisl_dn11 = assign24600_e33621_d_n11;
        locals.var_igisl_dn12 = assign24600_e33621_d_n12;
        locals.var_igisl_dn17 = assign24600_e33621_d_n17;

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

        let assign24620_e33630: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard772 = assign24620_e33630;

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

        let (assign24660_e33666, assign24660_e33666_d_n0, assign24660_e33666_d_n2, assign24660_e33666_d_n6, assign24660_e33666_d_n7, assign24660_e33666_d_n10, assign24660_e33666_d_n11, assign24660_e33666_d_n12, assign24660_e33666_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 != 0.0)) {
        let assign24660_e33664: f64 = (locals.var_t4 / locals.var_t0__blk766);
        (assign24660_e33664, (((locals.var_t4_dn0 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn0)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn2 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn2)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn6 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn6)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn7 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn7)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn10 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn10)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn11 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn11)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn12 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn12)) / (locals.var_t0__blk766 * locals.var_t0__blk766)), (((locals.var_t4_dn17 * locals.var_t0__blk766) - (locals.var_t4 * locals.var_t0__blk766_dn17)) / (locals.var_t0__blk766 * locals.var_t0__blk766)),)
    } else {
        (locals.var_t5__blk767, locals.var_t5__blk767_dn0, locals.var_t5__blk767_dn2, locals.var_t5__blk767_dn6, locals.var_t5__blk767_dn7, locals.var_t5__blk767_dn10, locals.var_t5__blk767_dn11, locals.var_t5__blk767_dn12, locals.var_t5__blk767_dn17,)
    }
};
        locals.var_t5__blk767 = assign24660_e33666;
        locals.var_t5__blk767_dn0 = assign24660_e33666_d_n0;
        locals.var_t5__blk767_dn2 = assign24660_e33666_d_n2;
        locals.var_t5__blk767_dn6 = assign24660_e33666_d_n6;
        locals.var_t5__blk767_dn7 = assign24660_e33666_d_n7;
        locals.var_t5__blk767_dn10 = assign24660_e33666_d_n10;
        locals.var_t5__blk767_dn11 = assign24660_e33666_d_n11;
        locals.var_t5__blk767_dn12 = assign24660_e33666_d_n12;
        locals.var_t5__blk767_dn17 = assign24660_e33666_d_n17;

        let (assign24670_e33675, assign24670_e33675_d_n0, assign24670_e33675_d_n2, assign24670_e33675_d_n6, assign24670_e33675_d_n7, assign24670_e33675_d_n10, assign24670_e33675_d_n11, assign24670_e33675_d_n12, assign24670_e33675_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 != 0.0)) {
        let assign24670_e33673: f64 = (locals.var_igisl * locals.var_t5__blk767);
        (assign24670_e33673, ((locals.var_igisl_dn0 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn0)), ((locals.var_igisl_dn2 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn2)), ((locals.var_igisl_dn6 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn6)), ((locals.var_igisl_dn7 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn7)), ((locals.var_igisl_dn10 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn10)), ((locals.var_igisl_dn11 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn11)), ((locals.var_igisl_dn12 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn12)), ((locals.var_igisl_dn17 * locals.var_t5__blk767) + (locals.var_igisl * locals.var_t5__blk767_dn17)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24670_e33675;
        locals.var_igisl_dn0 = assign24670_e33675_d_n0;
        locals.var_igisl_dn2 = assign24670_e33675_d_n2;
        locals.var_igisl_dn6 = assign24670_e33675_d_n6;
        locals.var_igisl_dn7 = assign24670_e33675_d_n7;
        locals.var_igisl_dn10 = assign24670_e33675_d_n10;
        locals.var_igisl_dn11 = assign24670_e33675_d_n11;
        locals.var_igisl_dn12 = assign24670_e33675_d_n12;
        locals.var_igisl_dn17 = assign24670_e33675_d_n17;

        let (assign24680_e33683, assign24680_e33683_d_n0, assign24680_e33683_d_n2, assign24680_e33683_d_n6, assign24680_e33683_d_n7, assign24680_e33683_d_n10, assign24680_e33683_d_n11, assign24680_e33683_d_n12, assign24680_e33683_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24680_e33683;
        locals.var_igisl_dn0 = assign24680_e33683_d_n0;
        locals.var_igisl_dn2 = assign24680_e33683_d_n2;
        locals.var_igisl_dn6 = assign24680_e33683_d_n6;
        locals.var_igisl_dn7 = assign24680_e33683_d_n7;
        locals.var_igisl_dn10 = assign24680_e33683_d_n10;
        locals.var_igisl_dn11 = assign24680_e33683_d_n11;
        locals.var_igisl_dn12 = assign24680_e33683_d_n12;
        locals.var_igisl_dn17 = assign24680_e33683_d_n17;

        let assign24690_e33686: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard773 = assign24690_e33686;

        let (assign24700_e33690,) = {
    if (locals.var_guard773 != 0.0) {
        (locals.var_c_fox0,)
    } else {
        (locals.var_cox0,)
    }
};
        locals.var_cox0 = assign24700_e33690;

        let (assign24710_e33696,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24710_e33694: f64 = (1.0 / locals.var_cox0);
        (assign24710_e33694,)
    } else {
        (locals.var_cox0_inv,)
    }
};
        locals.var_cox0_inv = assign24710_e33696;

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

    }

    pub(super) fn stamp_transient_block_84(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let (assign24830_e33763,) = {
    if ((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24830_e33763;

        let assign24840_e33766: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard853 = assign24840_e33766;

        let assign24850_e33769: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard854 = assign24850_e33769;

        let (assign24860_e33779,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24860_e33777: f64 = (locals.var_area_bt_p * 0.5);
        (assign24860_e33777,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24860_e33779;

        let (assign24870_e33787,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24870_e33787;

        let (assign24880_e33795,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        (locals.var_cbtbp_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24880_e33795;

        let (assign24890_e33808,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        let assign24890_e33806: f64 = (locals.var_area_bt_n * 0.5);
        (assign24890_e33806,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24890_e33808;

        let (assign24900_e33819,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (p.p68,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24900_e33819;

        let (assign24910_e33830,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (locals.var_cbtbn_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24910_e33830;

        let (assign24920_e33841,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (1.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24920_e33841;

        let assign24930_e33844: f64 = if locals.var_cbtb_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard855 = assign24930_e33844;

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

        let assign25080_e34035: f64 = if locals.var_t0__blk774 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard856 = assign25080_e34035;

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

    }

    pub(super) fn stamp_transient_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let (assign25280_e34286,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25280_e34284: f64 = (-locals.var_vxbgmtcl);
        (assign25280_e34284,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign25280_e34286;

        let assign25290_e34289: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard857 = assign25290_e34289;

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

        let assign25390_e34420: f64 = (locals.var_ac3__blk813 * 1e-8);
        let assign25390_e34421: f64 = if locals.var_ac4__blk810 < assign25390_e34420 { 1.0 } else { 0.0 };
        locals.var_guard858 = assign25390_e34421;

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

    }

    pub(super) fn stamp_transient_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign25590_e34727: f64 = (10.0 * 2.220446049250313e-16);
        let assign25590_e34728: f64 = if locals.var_tx__blk781 < assign25590_e34727 { 1.0 } else { 0.0 };
        locals.var_guard859 = assign25590_e34728;

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

        let assign25650_e34824: f64 = (10.0 * 2.220446049250313e-16);
        let assign25650_e34825: f64 = if locals.var_tx__blk781 < assign25650_e34824 { 1.0 } else { 0.0 };
        locals.var_guard860 = assign25650_e34825;

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

        let assign25690_e34880: f64 = if locals.var_chi__blk818 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard861 = assign25690_e34880;

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

    }

    pub(super) fn stamp_transient_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

    }

    pub(super) fn stamp_transient_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign26120_e35650: f64 = (10.0 * 2.220446049250313e-16);
        let assign26120_e35651: f64 = if locals.var_t1__blk775 < assign26120_e35650 { 1.0 } else { 0.0 };
        locals.var_guard862 = assign26120_e35651;

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

        let assign26170_e35709: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard863 = assign26170_e35709;

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

        let (assign26220_e35784,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
        locals.var_flg_conv__blk791 = assign26220_e35784;

        let (assign26230_e35797,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign26230_e35797;

    }

    pub(super) fn stamp_transient_block_89(
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
            let assign26240_body2_e35848: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard864 = assign26240_body2_e35848;
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
            let assign26240_body11_e36082: f64 = if locals.var_chi__blk818 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard865 = assign26240_body11_e36082;
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
            let assign26240_body22_e36296: f64 = if locals.var_flg_conv__blk791 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard866 = assign26240_body22_e36296;
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
            let assign26240_body26_e36365: f64 = (locals.var_dps0).abs();
            let assign26240_body26_e36367: f64 = if assign26240_body26_e36365 > locals.var_dplim__blk850 { 1.0 } else { 0.0 };
            locals.var_guard867 = assign26240_body26_e36367;
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
            let assign26240_body29_e36413: f64 = (locals.var_dps0).abs();
            let assign26240_body29_e36417: f64 = (locals.var_fs0__blk848).abs();
            let assign26240_body29_e36420: f64 = if ((assign26240_body29_e36413 <= 5e-12) && (assign26240_body29_e36417 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard868 = assign26240_body29_e36420;
            let (assign26240_body30_e36438,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) && (locals.var_guard868 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
            locals.var_flg_conv__blk791 = assign26240_body30_e36438;
            let (assign26240_body31_e36453,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26240_body31_e36451: f64 = (locals.var_lp_s0 + 1.0);
        (assign26240_body31_e36451,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26240_body31_e36453;
        }

    }

    pub(super) fn stamp_transient_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign26260_e36459: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard870 = assign26260_e36459;

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

        let assign26400_e36665: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign26400_e36665;

        let assign26410_e36668: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign26410_e36668;

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

    }

    pub(super) fn stamp_transient_block_91(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign26630_e36978: f64 = if locals.var_t0__blk774 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard874 = assign26630_e36978;

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

        let (assign26830_e37229,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26830_e37227: f64 = (-locals.var_vxbgmtcl);
        (assign26830_e37227,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign26830_e37229;

        let assign26840_e37232: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard875 = assign26840_e37232;

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

    }

    pub(super) fn stamp_transient_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign26940_e37363: f64 = (locals.var_ac3__blk813 * 1e-8);
        let assign26940_e37364: f64 = if locals.var_ac4__blk810 < assign26940_e37363 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign26940_e37364;

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

        let assign27140_e37670: f64 = (10.0 * 2.220446049250313e-16);
        let assign27140_e37671: f64 = if locals.var_tx__blk781 < assign27140_e37670 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign27140_e37671;

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

        let assign27200_e37767: f64 = (10.0 * 2.220446049250313e-16);
        let assign27200_e37768: f64 = if locals.var_tx__blk781 < assign27200_e37767 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign27200_e37768;

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

    }

    pub(super) fn stamp_transient_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign27240_e37823: f64 = if locals.var_chi__blk818 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard879 = assign27240_e37823;

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

    }

    pub(super) fn stamp_transient_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign27670_e38593: f64 = (10.0 * 2.220446049250313e-16);
        let assign27670_e38594: f64 = if locals.var_t1__blk775 < assign27670_e38593 { 1.0 } else { 0.0 };
        locals.var_guard880 = assign27670_e38594;

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

        let assign27720_e38652: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard881 = assign27720_e38652;

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

    }

    pub(super) fn stamp_transient_block_95(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let (assign27770_e38727,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
        locals.var_flg_conv__blk791 = assign27770_e38727;

        let (assign27780_e38740,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign27780_e38740;

    }
}
