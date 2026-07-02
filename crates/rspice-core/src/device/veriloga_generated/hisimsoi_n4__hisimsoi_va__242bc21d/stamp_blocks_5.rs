#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23580_e32698, assign23580_e32698_d_n0, assign23580_e32698_d_n2, assign23580_e32698_d_n6, assign23580_e32698_d_n7, assign23580_e32698_d_n10, assign23580_e32698_d_n11, assign23580_e32698_d_n12, assign23580_e32698_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23580_e32691: f64 = (locals.var_etun + locals.var_tmf1);
        let assign23580_e32692: f64 = (0.5 * assign23580_e32691);
        let assign23580_e32695: f64 = (1e-10 * 0.01);
        let assign23580_e32696: f64 = (assign23580_e32692 + assign23580_e32695);
        (assign23580_e32696, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23580_e32698;
        locals.var_etun_dn0 = assign23580_e32698_d_n0;
        locals.var_etun_dn2 = assign23580_e32698_d_n2;
        locals.var_etun_dn6 = assign23580_e32698_d_n6;
        locals.var_etun_dn7 = assign23580_e32698_d_n7;
        locals.var_etun_dn10 = assign23580_e32698_d_n10;
        locals.var_etun_dn11 = assign23580_e32698_d_n11;
        locals.var_etun_dn12 = assign23580_e32698_d_n12;
        locals.var_etun_dn17 = assign23580_e32698_d_n17;

        let assign23590_e32701: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign23590_e32701;

        let (assign23600_e32710, assign23600_e32710_d_n0, assign23600_e32710_d_n2, assign23600_e32710_d_n6, assign23600_e32710_d_n7, assign23600_e32710_d_n10, assign23600_e32710_d_n11, assign23600_e32710_d_n12, assign23600_e32710_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23600_e32710;
        locals.var_etun_dn0 = assign23600_e32710_d_n0;
        locals.var_etun_dn2 = assign23600_e32710_d_n2;
        locals.var_etun_dn6 = assign23600_e32710_d_n6;
        locals.var_etun_dn7 = assign23600_e32710_d_n7;
        locals.var_etun_dn10 = assign23600_e32710_d_n10;
        locals.var_etun_dn11 = assign23600_e32710_d_n11;
        locals.var_etun_dn12 = assign23600_e32710_d_n12;
        locals.var_etun_dn17 = assign23600_e32710_d_n17;

        let (assign23610_e32726, assign23610_e32726_d_n0, assign23610_e32726_d_n2, assign23610_e32726_d_n6, assign23610_e32726_d_n7, assign23610_e32726_d_n10, assign23610_e32726_d_n11, assign23610_e32726_d_n12, assign23610_e32726_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23610_e32717: f64 = (locals.var_vgsz * locals.var_vgsz);
        let assign23610_e32720: f64 = (4.0 * 0.001);
        let assign23610_e32722: f64 = (assign23610_e32720 * 0.001);
        let assign23610_e32723: f64 = (assign23610_e32717 + assign23610_e32722);
        let assign23610_e32724: f64 = (assign23610_e32723).sqrt();
        (assign23610_e32724, (((locals.var_vgsz_dn0 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn0)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn2 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn2)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn6 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn6)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn7 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn7)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn10 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn10)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn11 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn11)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn12 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn12)) / (2.0 * assign23610_e32724)), (((locals.var_vgsz_dn17 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn17)) / (2.0 * assign23610_e32724)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23610_e32726;
        locals.var_tmf1_dn0 = assign23610_e32726_d_n0;
        locals.var_tmf1_dn2 = assign23610_e32726_d_n2;
        locals.var_tmf1_dn6 = assign23610_e32726_d_n6;
        locals.var_tmf1_dn7 = assign23610_e32726_d_n7;
        locals.var_tmf1_dn10 = assign23610_e32726_d_n10;
        locals.var_tmf1_dn11 = assign23610_e32726_d_n11;
        locals.var_tmf1_dn12 = assign23610_e32726_d_n12;
        locals.var_tmf1_dn17 = assign23610_e32726_d_n17;

        let (assign23620_e32741, assign23620_e32741_d_n0, assign23620_e32741_d_n2, assign23620_e32741_d_n6, assign23620_e32741_d_n7, assign23620_e32741_d_n10, assign23620_e32741_d_n11, assign23620_e32741_d_n12, assign23620_e32741_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23620_e32734: f64 = (locals.var_vgsz + locals.var_tmf1);
        let assign23620_e32735: f64 = (0.5 * assign23620_e32734);
        let assign23620_e32738: f64 = (1e-10 * 0.001);
        let assign23620_e32739: f64 = (assign23620_e32735 + assign23620_e32738);
        (assign23620_e32739, (0.5 * (locals.var_vgsz_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_vgsz_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_vgsz_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_vgsz_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_vgsz_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_vgsz_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_vgsz_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_vgsz_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23620_e32741;
        locals.var_t3__blk722_dn0 = assign23620_e32741_d_n0;
        locals.var_t3__blk722_dn2 = assign23620_e32741_d_n2;
        locals.var_t3__blk722_dn6 = assign23620_e32741_d_n6;
        locals.var_t3__blk722_dn7 = assign23620_e32741_d_n7;
        locals.var_t3__blk722_dn10 = assign23620_e32741_d_n10;
        locals.var_t3__blk722_dn11 = assign23620_e32741_d_n11;
        locals.var_t3__blk722_dn12 = assign23620_e32741_d_n12;
        locals.var_t3__blk722_dn17 = assign23620_e32741_d_n17;

        let assign23630_e32744: f64 = if locals.var_t3__blk722 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign23630_e32744;

        let (assign23640_e32753, assign23640_e32753_d_n0, assign23640_e32753_d_n2, assign23640_e32753_d_n6, assign23640_e32753_d_n7, assign23640_e32753_d_n10, assign23640_e32753_d_n11, assign23640_e32753_d_n12, assign23640_e32753_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard741 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23640_e32753;
        locals.var_t3__blk722_dn0 = assign23640_e32753_d_n0;
        locals.var_t3__blk722_dn2 = assign23640_e32753_d_n2;
        locals.var_t3__blk722_dn6 = assign23640_e32753_d_n6;
        locals.var_t3__blk722_dn7 = assign23640_e32753_d_n7;
        locals.var_t3__blk722_dn10 = assign23640_e32753_d_n10;
        locals.var_t3__blk722_dn11 = assign23640_e32753_d_n11;
        locals.var_t3__blk722_dn12 = assign23640_e32753_d_n12;
        locals.var_t3__blk722_dn17 = assign23640_e32753_d_n17;

        let (assign23650_e32762, assign23650_e32762_d_n0, assign23650_e32762_d_n2, assign23650_e32762_d_n6, assign23650_e32762_d_n7, assign23650_e32762_d_n10, assign23650_e32762_d_n11, assign23650_e32762_d_n12, assign23650_e32762_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23650_e32760: f64 = (locals.var_t3__blk722 - p.p226);
        (assign23650_e32760, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23650_e32762;
        locals.var_t3__blk722_dn0 = assign23650_e32762_d_n0;
        locals.var_t3__blk722_dn2 = assign23650_e32762_d_n2;
        locals.var_t3__blk722_dn6 = assign23650_e32762_d_n6;
        locals.var_t3__blk722_dn7 = assign23650_e32762_d_n7;
        locals.var_t3__blk722_dn10 = assign23650_e32762_d_n10;
        locals.var_t3__blk722_dn11 = assign23650_e32762_d_n11;
        locals.var_t3__blk722_dn12 = assign23650_e32762_d_n12;
        locals.var_t3__blk722_dn17 = assign23650_e32762_d_n17;

        let (assign23660_e32771, assign23660_e32771_d_n0, assign23660_e32771_d_n2, assign23660_e32771_d_n6, assign23660_e32771_d_n7, assign23660_e32771_d_n10, assign23660_e32771_d_n11, assign23660_e32771_d_n12, assign23660_e32771_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23660_e32769: f64 = (locals.var_t3__blk722 / 0.1);
        (assign23660_e32769, (locals.var_t3__blk722_dn0 / 0.1), (locals.var_t3__blk722_dn2 / 0.1), (locals.var_t3__blk722_dn6 / 0.1), (locals.var_t3__blk722_dn7 / 0.1), (locals.var_t3__blk722_dn10 / 0.1), (locals.var_t3__blk722_dn11 / 0.1), (locals.var_t3__blk722_dn12 / 0.1), (locals.var_t3__blk722_dn17 / 0.1),)
    } else {
        (locals.var_tx__blk718, locals.var_tx__blk718_dn0, locals.var_tx__blk718_dn2, locals.var_tx__blk718_dn6, locals.var_tx__blk718_dn7, locals.var_tx__blk718_dn10, locals.var_tx__blk718_dn11, locals.var_tx__blk718_dn12, locals.var_tx__blk718_dn17,)
    }
};
        locals.var_tx__blk718 = assign23660_e32771;
        locals.var_tx__blk718_dn0 = assign23660_e32771_d_n0;
        locals.var_tx__blk718_dn2 = assign23660_e32771_d_n2;
        locals.var_tx__blk718_dn6 = assign23660_e32771_d_n6;
        locals.var_tx__blk718_dn7 = assign23660_e32771_d_n7;
        locals.var_tx__blk718_dn10 = assign23660_e32771_d_n10;
        locals.var_tx__blk718_dn11 = assign23660_e32771_d_n11;
        locals.var_tx__blk718_dn12 = assign23660_e32771_d_n12;
        locals.var_tx__blk718_dn17 = assign23660_e32771_d_n17;

        let (assign23670_e32782, assign23670_e32782_d_n0, assign23670_e32782_d_n2, assign23670_e32782_d_n6, assign23670_e32782_d_n7, assign23670_e32782_d_n10, assign23670_e32782_d_n11, assign23670_e32782_d_n12, assign23670_e32782_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23670_e32779: f64 = (locals.var_tx__blk718 * locals.var_tx__blk718);
        let assign23670_e32780: f64 = (1.0 + assign23670_e32779);
        (assign23670_e32780, ((locals.var_tx__blk718_dn0 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn0)), ((locals.var_tx__blk718_dn2 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn2)), ((locals.var_tx__blk718_dn6 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn6)), ((locals.var_tx__blk718_dn7 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn7)), ((locals.var_tx__blk718_dn10 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn10)), ((locals.var_tx__blk718_dn11 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn11)), ((locals.var_tx__blk718_dn12 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn12)), ((locals.var_tx__blk718_dn17 * locals.var_tx__blk718) + (locals.var_tx__blk718 * locals.var_tx__blk718_dn17)),)
    } else {
        (locals.var_t2__blk721, locals.var_t2__blk721_dn0, locals.var_t2__blk721_dn2, locals.var_t2__blk721_dn6, locals.var_t2__blk721_dn7, locals.var_t2__blk721_dn10, locals.var_t2__blk721_dn11, locals.var_t2__blk721_dn12, locals.var_t2__blk721_dn17,)
    }
};
        locals.var_t2__blk721 = assign23670_e32782;
        locals.var_t2__blk721_dn0 = assign23670_e32782_d_n0;
        locals.var_t2__blk721_dn2 = assign23670_e32782_d_n2;
        locals.var_t2__blk721_dn6 = assign23670_e32782_d_n6;
        locals.var_t2__blk721_dn7 = assign23670_e32782_d_n7;
        locals.var_t2__blk721_dn10 = assign23670_e32782_d_n10;
        locals.var_t2__blk721_dn11 = assign23670_e32782_d_n11;
        locals.var_t2__blk721_dn12 = assign23670_e32782_d_n12;
        locals.var_t2__blk721_dn17 = assign23670_e32782_d_n17;

        let (assign23680_e32793, assign23680_e32793_d_n0, assign23680_e32793_d_n2, assign23680_e32793_d_n6, assign23680_e32793_d_n7, assign23680_e32793_d_n10, assign23680_e32793_d_n11, assign23680_e32793_d_n12, assign23680_e32793_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23680_e32790: f64 = (1.0 / locals.var_t2__blk721);
        let assign23680_e32791: f64 = (1.0 - assign23680_e32790);
        (assign23680_e32791, (-(-(locals.var_t2__blk721_dn0 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn2 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn6 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn7 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn10 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn11 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn12 / (locals.var_t2__blk721 * locals.var_t2__blk721)))), (-(-(locals.var_t2__blk721_dn17 / (locals.var_t2__blk721 * locals.var_t2__blk721)))),)
    } else {
        (locals.var_t1__blk720, locals.var_t1__blk720_dn0, locals.var_t1__blk720_dn2, locals.var_t1__blk720_dn6, locals.var_t1__blk720_dn7, locals.var_t1__blk720_dn10, locals.var_t1__blk720_dn11, locals.var_t1__blk720_dn12, locals.var_t1__blk720_dn17,)
    }
};
        locals.var_t1__blk720 = assign23680_e32793;
        locals.var_t1__blk720_dn0 = assign23680_e32793_d_n0;
        locals.var_t1__blk720_dn2 = assign23680_e32793_d_n2;
        locals.var_t1__blk720_dn6 = assign23680_e32793_d_n6;
        locals.var_t1__blk720_dn7 = assign23680_e32793_d_n7;
        locals.var_t1__blk720_dn10 = assign23680_e32793_d_n10;
        locals.var_t1__blk720_dn11 = assign23680_e32793_d_n11;
        locals.var_t1__blk720_dn12 = assign23680_e32793_d_n12;
        locals.var_t1__blk720_dn17 = assign23680_e32793_d_n17;

        let (assign23690_e32802, assign23690_e32802_d_n0, assign23690_e32802_d_n2, assign23690_e32802_d_n6, assign23690_e32802_d_n7, assign23690_e32802_d_n10, assign23690_e32802_d_n11, assign23690_e32802_d_n12, assign23690_e32802_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23690_e32800: f64 = (locals.var_etun * locals.var_t1__blk720);
        (assign23690_e32800, ((locals.var_etun_dn0 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn0)), ((locals.var_etun_dn2 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn2)), ((locals.var_etun_dn6 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn6)), ((locals.var_etun_dn7 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn7)), ((locals.var_etun_dn10 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn10)), ((locals.var_etun_dn11 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn11)), ((locals.var_etun_dn12 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn12)), ((locals.var_etun_dn17 * locals.var_t1__blk720) + (locals.var_etun * locals.var_t1__blk720_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23690_e32802;
        locals.var_etun_dn0 = assign23690_e32802_d_n0;
        locals.var_etun_dn2 = assign23690_e32802_d_n2;
        locals.var_etun_dn6 = assign23690_e32802_d_n6;
        locals.var_etun_dn7 = assign23690_e32802_d_n7;
        locals.var_etun_dn10 = assign23690_e32802_d_n10;
        locals.var_etun_dn11 = assign23690_e32802_d_n11;
        locals.var_etun_dn12 = assign23690_e32802_d_n12;
        locals.var_etun_dn17 = assign23690_e32802_d_n17;

        let (assign23700_e32811, assign23700_e32811_d_n0, assign23700_e32811_d_n2, assign23700_e32811_d_n6, assign23700_e32811_d_n7, assign23700_e32811_d_n10, assign23700_e32811_d_n11, assign23700_e32811_d_n12, assign23700_e32811_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23700_e32809: f64 = (locals.var_cgs_leff__blk733 * locals.var_cgs_weff_nf__blk734);
        (assign23700_e32809, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk719, locals.var_t0__blk719_dn0, locals.var_t0__blk719_dn2, locals.var_t0__blk719_dn6, locals.var_t0__blk719_dn7, locals.var_t0__blk719_dn10, locals.var_t0__blk719_dn11, locals.var_t0__blk719_dn12, locals.var_t0__blk719_dn17,)
    }
};
        locals.var_t0__blk719 = assign23700_e32811;
        locals.var_t0__blk719_dn0 = assign23700_e32811_d_n0;
        locals.var_t0__blk719_dn2 = assign23700_e32811_d_n2;
        locals.var_t0__blk719_dn6 = assign23700_e32811_d_n6;
        locals.var_t0__blk719_dn7 = assign23700_e32811_d_n7;
        locals.var_t0__blk719_dn10 = assign23700_e32811_d_n10;
        locals.var_t0__blk719_dn11 = assign23700_e32811_d_n11;
        locals.var_t0__blk719_dn12 = assign23700_e32811_d_n12;
        locals.var_t0__blk719_dn17 = assign23700_e32811_d_n17;

        let (assign23710_e32822, assign23710_e32822_d_n0, assign23710_e32822_d_n2, assign23710_e32822_d_n6, assign23710_e32822_d_n7, assign23710_e32822_d_n10, assign23710_e32822_d_n11, assign23710_e32822_d_n12, assign23710_e32822_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23710_e32819: f64 = (p.p219 + locals.var_t0__blk719);
        let assign23710_e32820: f64 = (p.p219 / assign23710_e32819);
        (assign23710_e32820, (-((p.p219 * locals.var_t0__blk719_dn0) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn2) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn6) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn7) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn10) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn11) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn12) / (assign23710_e32819 * assign23710_e32819))), (-((p.p219 * locals.var_t0__blk719_dn17) / (assign23710_e32819 * assign23710_e32819))),)
    } else {
        (locals.var_t7__blk726, locals.var_t7__blk726_dn0, locals.var_t7__blk726_dn2, locals.var_t7__blk726_dn6, locals.var_t7__blk726_dn7, locals.var_t7__blk726_dn10, locals.var_t7__blk726_dn11, locals.var_t7__blk726_dn12, locals.var_t7__blk726_dn17,)
    }
};
        locals.var_t7__blk726 = assign23710_e32822;
        locals.var_t7__blk726_dn0 = assign23710_e32822_d_n0;
        locals.var_t7__blk726_dn2 = assign23710_e32822_d_n2;
        locals.var_t7__blk726_dn6 = assign23710_e32822_d_n6;
        locals.var_t7__blk726_dn7 = assign23710_e32822_d_n7;
        locals.var_t7__blk726_dn10 = assign23710_e32822_d_n10;
        locals.var_t7__blk726_dn11 = assign23710_e32822_d_n11;
        locals.var_t7__blk726_dn12 = assign23710_e32822_d_n12;
        locals.var_t7__blk726_dn17 = assign23710_e32822_d_n17;

        let (assign23720_e32829, assign23720_e32829_d_n0, assign23720_e32829_d_n2, assign23720_e32829_d_n6, assign23720_e32829_d_n7, assign23720_e32829_d_n10, assign23720_e32829_d_n11, assign23720_e32829_d_n12, assign23720_e32829_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        (p.p218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk725, locals.var_t6__blk725_dn0, locals.var_t6__blk725_dn2, locals.var_t6__blk725_dn6, locals.var_t6__blk725_dn7, locals.var_t6__blk725_dn10, locals.var_t6__blk725_dn11, locals.var_t6__blk725_dn12, locals.var_t6__blk725_dn17,)
    }
};
        locals.var_t6__blk725 = assign23720_e32829;
        locals.var_t6__blk725_dn0 = assign23720_e32829_d_n0;
        locals.var_t6__blk725_dn2 = assign23720_e32829_d_n2;
        locals.var_t6__blk725_dn6 = assign23720_e32829_d_n6;
        locals.var_t6__blk725_dn7 = assign23720_e32829_d_n7;
        locals.var_t6__blk725_dn10 = assign23720_e32829_d_n10;
        locals.var_t6__blk725_dn11 = assign23720_e32829_d_n11;
        locals.var_t6__blk725_dn12 = assign23720_e32829_d_n12;
        locals.var_t6__blk725_dn17 = assign23720_e32829_d_n17;

        let (assign23730_e32840, assign23730_e32840_d_n0, assign23730_e32840_d_n2, assign23730_e32840_d_n6, assign23730_e32840_d_n7, assign23730_e32840_d_n10, assign23730_e32840_d_n11, assign23730_e32840_d_n12, assign23730_e32840_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23730_e32837: f64 = (locals.var_t6__blk725 + locals.var_vdsz);
        let assign23730_e32838: f64 = (locals.var_t6__blk725 / assign23730_e32837);
        (assign23730_e32838, (((locals.var_t6__blk725_dn0 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn0 + locals.var_vdsz_dn0))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn2 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn2 + locals.var_vdsz_dn2))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn6 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn6 + locals.var_vdsz_dn6))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn7 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn7 + locals.var_vdsz_dn7))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn10 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn10 + locals.var_vdsz_dn10))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn11 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn11 + locals.var_vdsz_dn11))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn12 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn12 + locals.var_vdsz_dn12))) / (assign23730_e32837 * assign23730_e32837)), (((locals.var_t6__blk725_dn17 * assign23730_e32837) - (locals.var_t6__blk725 * (locals.var_t6__blk725_dn17 + locals.var_vdsz_dn17))) / (assign23730_e32837 * assign23730_e32837)),)
    } else {
        (locals.var_t9__blk727, locals.var_t9__blk727_dn0, locals.var_t9__blk727_dn2, locals.var_t9__blk727_dn6, locals.var_t9__blk727_dn7, locals.var_t9__blk727_dn10, locals.var_t9__blk727_dn11, locals.var_t9__blk727_dn12, locals.var_t9__blk727_dn17,)
    }
};
        locals.var_t9__blk727 = assign23730_e32840;
        locals.var_t9__blk727_dn0 = assign23730_e32840_d_n0;
        locals.var_t9__blk727_dn2 = assign23730_e32840_d_n2;
        locals.var_t9__blk727_dn6 = assign23730_e32840_d_n6;
        locals.var_t9__blk727_dn7 = assign23730_e32840_d_n7;
        locals.var_t9__blk727_dn10 = assign23730_e32840_d_n10;
        locals.var_t9__blk727_dn11 = assign23730_e32840_d_n11;
        locals.var_t9__blk727_dn12 = assign23730_e32840_d_n12;
        locals.var_t9__blk727_dn17 = assign23730_e32840_d_n17;

        let (assign23740_e32851, assign23740_e32851_d_n0, assign23740_e32851_d_n2, assign23740_e32851_d_n6, assign23740_e32851_d_n7, assign23740_e32851_d_n10, assign23740_e32851_d_n11, assign23740_e32851_d_n12, assign23740_e32851_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23740_e32848: f64 = (locals.var_etun + 1e-50);
        let assign23740_e32849: f64 = (1.0 / assign23740_e32848);
        (assign23740_e32849, (-(locals.var_etun_dn0 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn2 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn6 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn7 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn10 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn11 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn12 / (assign23740_e32848 * assign23740_e32848))), (-(locals.var_etun_dn17 / (assign23740_e32848 * assign23740_e32848))),)
    } else {
        (locals.var_t4__blk723, locals.var_t4__blk723_dn0, locals.var_t4__blk723_dn2, locals.var_t4__blk723_dn6, locals.var_t4__blk723_dn7, locals.var_t4__blk723_dn10, locals.var_t4__blk723_dn11, locals.var_t4__blk723_dn12, locals.var_t4__blk723_dn17,)
    }
};
        locals.var_t4__blk723 = assign23740_e32851;
        locals.var_t4__blk723_dn0 = assign23740_e32851_d_n0;
        locals.var_t4__blk723_dn2 = assign23740_e32851_d_n2;
        locals.var_t4__blk723_dn6 = assign23740_e32851_d_n6;
        locals.var_t4__blk723_dn7 = assign23740_e32851_d_n7;
        locals.var_t4__blk723_dn10 = assign23740_e32851_d_n10;
        locals.var_t4__blk723_dn11 = assign23740_e32851_d_n11;
        locals.var_t4__blk723_dn12 = assign23740_e32851_d_n12;
        locals.var_t4__blk723_dn17 = assign23740_e32851_d_n17;

        let (assign23750_e32863, assign23750_e32863_d_n0, assign23750_e32863_d_n2, assign23750_e32863_d_n6, assign23750_e32863_d_n7, assign23750_e32863_d_n10, assign23750_e32863_d_n11, assign23750_e32863_d_n12, assign23750_e32863_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign23750_e32857: f64 = (-p.p214);
        let assign23750_e32859: f64 = (assign23750_e32857 * locals.var_egp32);
        let assign23750_e32861: f64 = (assign23750_e32859 * locals.var_t4__blk723);
        (assign23750_e32861, (((assign23750_e32857 * locals.var_egp32_dn0) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn0)), (((assign23750_e32857 * locals.var_egp32_dn2) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn2)), (((assign23750_e32857 * locals.var_egp32_dn6) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn6)), (((assign23750_e32857 * locals.var_egp32_dn7) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn7)), (((assign23750_e32857 * locals.var_egp32_dn10) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn10)), (((assign23750_e32857 * locals.var_egp32_dn11) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn11)), (((assign23750_e32857 * locals.var_egp32_dn12) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn12)), (((assign23750_e32857 * locals.var_egp32_dn17) * locals.var_t4__blk723) + (assign23750_e32859 * locals.var_t4__blk723_dn17)),)
    } else {
        (locals.var_t1__blk720, locals.var_t1__blk720_dn0, locals.var_t1__blk720_dn2, locals.var_t1__blk720_dn6, locals.var_t1__blk720_dn7, locals.var_t1__blk720_dn10, locals.var_t1__blk720_dn11, locals.var_t1__blk720_dn12, locals.var_t1__blk720_dn17,)
    }
};
        locals.var_t1__blk720 = assign23750_e32863;
        locals.var_t1__blk720_dn0 = assign23750_e32863_d_n0;
        locals.var_t1__blk720_dn2 = assign23750_e32863_d_n2;
        locals.var_t1__blk720_dn6 = assign23750_e32863_d_n6;
        locals.var_t1__blk720_dn7 = assign23750_e32863_d_n7;
        locals.var_t1__blk720_dn10 = assign23750_e32863_d_n10;
        locals.var_t1__blk720_dn11 = assign23750_e32863_d_n11;
        locals.var_t1__blk720_dn12 = assign23750_e32863_d_n12;
        locals.var_t1__blk720_dn17 = assign23750_e32863_d_n17;

        let assign23760_e32866: f64 = (-34.0);
        let assign23760_e32867: f64 = if locals.var_t1__blk720 < assign23760_e32866 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign23760_e32867;

        let (assign23770_e32876, assign23770_e32876_d_n0, assign23770_e32876_d_n2, assign23770_e32876_d_n6, assign23770_e32876_d_n7, assign23770_e32876_d_n10, assign23770_e32876_d_n11, assign23770_e32876_d_n12, assign23770_e32876_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23770_e32876;
        locals.var_igate_dn0 = assign23770_e32876_d_n0;
        locals.var_igate_dn2 = assign23770_e32876_d_n2;
        locals.var_igate_dn6 = assign23770_e32876_d_n6;
        locals.var_igate_dn7 = assign23770_e32876_d_n7;
        locals.var_igate_dn10 = assign23770_e32876_d_n10;
        locals.var_igate_dn11 = assign23770_e32876_d_n11;
        locals.var_igate_dn12 = assign23770_e32876_d_n12;
        locals.var_igate_dn17 = assign23770_e32876_d_n17;

        let (assign23780_e32887, assign23780_e32887_d_n0, assign23780_e32887_d_n2, assign23780_e32887_d_n6, assign23780_e32887_d_n7, assign23780_e32887_d_n10, assign23780_e32887_d_n11, assign23780_e32887_d_n12, assign23780_e32887_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23780_e32885: f64 = (locals.var_t1__blk720).exp();
        (assign23780_e32885, (assign23780_e32885 * locals.var_t1__blk720_dn0), (assign23780_e32885 * locals.var_t1__blk720_dn2), (assign23780_e32885 * locals.var_t1__blk720_dn6), (assign23780_e32885 * locals.var_t1__blk720_dn7), (assign23780_e32885 * locals.var_t1__blk720_dn10), (assign23780_e32885 * locals.var_t1__blk720_dn11), (assign23780_e32885 * locals.var_t1__blk720_dn12), (assign23780_e32885 * locals.var_t1__blk720_dn17),)
    } else {
        (locals.var_t2__blk721, locals.var_t2__blk721_dn0, locals.var_t2__blk721_dn2, locals.var_t2__blk721_dn6, locals.var_t2__blk721_dn7, locals.var_t2__blk721_dn10, locals.var_t2__blk721_dn11, locals.var_t2__blk721_dn12, locals.var_t2__blk721_dn17,)
    }
};
        locals.var_t2__blk721 = assign23780_e32887;
        locals.var_t2__blk721_dn0 = assign23780_e32887_d_n0;
        locals.var_t2__blk721_dn2 = assign23780_e32887_d_n2;
        locals.var_t2__blk721_dn6 = assign23780_e32887_d_n6;
        locals.var_t2__blk721_dn7 = assign23780_e32887_d_n7;
        locals.var_t2__blk721_dn10 = assign23780_e32887_d_n10;
        locals.var_t2__blk721_dn11 = assign23780_e32887_d_n11;
        locals.var_t2__blk721_dn12 = assign23780_e32887_d_n12;
        locals.var_t2__blk721_dn17 = assign23780_e32887_d_n17;

        let (assign23790_e32903, assign23790_e32903_d_n0, assign23790_e32903_d_n2, assign23790_e32903_d_n6, assign23790_e32903_d_n7, assign23790_e32903_d_n10, assign23790_e32903_d_n11, assign23790_e32903_d_n12, assign23790_e32903_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23790_e32897: f64 = (p.p213 / locals.var_egp12);
        let assign23790_e32899: f64 = (assign23790_e32897 * 1.6021918e-19);
        let assign23790_e32901: f64 = (assign23790_e32899 * locals.var_t0__blk719);
        (assign23790_e32901, ((((-((p.p213 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn0)), ((((-((p.p213 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn2)), ((((-((p.p213 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn6)), ((((-((p.p213 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn7)), ((((-((p.p213 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn10)), ((((-((p.p213 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn11)), ((((-((p.p213 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn12)), ((((-((p.p213 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk719) + (assign23790_e32899 * locals.var_t0__blk719_dn17)),)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23790_e32903;
        locals.var_t3__blk722_dn0 = assign23790_e32903_d_n0;
        locals.var_t3__blk722_dn2 = assign23790_e32903_d_n2;
        locals.var_t3__blk722_dn6 = assign23790_e32903_d_n6;
        locals.var_t3__blk722_dn7 = assign23790_e32903_d_n7;
        locals.var_t3__blk722_dn10 = assign23790_e32903_d_n10;
        locals.var_t3__blk722_dn11 = assign23790_e32903_d_n11;
        locals.var_t3__blk722_dn12 = assign23790_e32903_d_n12;
        locals.var_t3__blk722_dn17 = assign23790_e32903_d_n17;

        let (assign23800_e32915, assign23800_e32915_d_n0, assign23800_e32915_d_n2, assign23800_e32915_d_n6, assign23800_e32915_d_n7, assign23800_e32915_d_n10, assign23800_e32915_d_n11, assign23800_e32915_d_n12, assign23800_e32915_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23800_e32913: f64 = (1.0 / locals.var_cgs_cnst0soi);
        (assign23800_e32913, (-(locals.var_cgs_cnst0soi_dn0 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn2 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn6 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn7 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn10 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn11 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn12 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn17 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))),)
    } else {
        (locals.var_t5__blk724, locals.var_t5__blk724_dn0, locals.var_t5__blk724_dn2, locals.var_t5__blk724_dn6, locals.var_t5__blk724_dn7, locals.var_t5__blk724_dn10, locals.var_t5__blk724_dn11, locals.var_t5__blk724_dn12, locals.var_t5__blk724_dn17,)
    }
};
        locals.var_t5__blk724 = assign23800_e32915;
        locals.var_t5__blk724_dn0 = assign23800_e32915_d_n0;
        locals.var_t5__blk724_dn2 = assign23800_e32915_d_n2;
        locals.var_t5__blk724_dn6 = assign23800_e32915_d_n6;
        locals.var_t5__blk724_dn7 = assign23800_e32915_d_n7;
        locals.var_t5__blk724_dn10 = assign23800_e32915_d_n10;
        locals.var_t5__blk724_dn11 = assign23800_e32915_d_n11;
        locals.var_t5__blk724_dn12 = assign23800_e32915_d_n12;
        locals.var_t5__blk724_dn17 = assign23800_e32915_d_n17;

        let (assign23810_e32932, assign23810_e32932_d_n0, assign23810_e32932_d_n2, assign23810_e32932_d_n6, assign23810_e32932_d_n7, assign23810_e32932_d_n10, assign23810_e32932_d_n11, assign23810_e32932_d_n12, assign23810_e32932_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23810_e32926: f64 = (locals.var_cgs_c_fox * 1e-12);
        let assign23810_e32927: f64 = (locals.var_cgs_qiu__blk736 + assign23810_e32926);
        let assign23810_e32929: f64 = (assign23810_e32927 * locals.var_t5__blk724);
        let assign23810_e32930: f64 = (assign23810_e32929).sqrt();
        (assign23810_e32930, ((((locals.var_cgs_qiu__blk736_dn0 + (locals.var_cgs_c_fox_dn0 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn0)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn2 + (locals.var_cgs_c_fox_dn2 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn2)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn6 + (locals.var_cgs_c_fox_dn6 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn6)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn7 + (locals.var_cgs_c_fox_dn7 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn7)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn10 + (locals.var_cgs_c_fox_dn10 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn10)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn11 + (locals.var_cgs_c_fox_dn11 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn11)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn12 + (locals.var_cgs_c_fox_dn12 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn12)) / (2.0 * assign23810_e32930)), ((((locals.var_cgs_qiu__blk736_dn17 + (locals.var_cgs_c_fox_dn17 * 1e-12)) * locals.var_t5__blk724) + (assign23810_e32927 * locals.var_t5__blk724_dn17)) / (2.0 * assign23810_e32930)),)
    } else {
        (locals.var_t6__blk725, locals.var_t6__blk725_dn0, locals.var_t6__blk725_dn2, locals.var_t6__blk725_dn6, locals.var_t6__blk725_dn7, locals.var_t6__blk725_dn10, locals.var_t6__blk725_dn11, locals.var_t6__blk725_dn12, locals.var_t6__blk725_dn17,)
    }
};
        locals.var_t6__blk725 = assign23810_e32932;
        locals.var_t6__blk725_dn0 = assign23810_e32932_d_n0;
        locals.var_t6__blk725_dn2 = assign23810_e32932_d_n2;
        locals.var_t6__blk725_dn6 = assign23810_e32932_d_n6;
        locals.var_t6__blk725_dn7 = assign23810_e32932_d_n7;
        locals.var_t6__blk725_dn10 = assign23810_e32932_d_n10;
        locals.var_t6__blk725_dn11 = assign23810_e32932_d_n11;
        locals.var_t6__blk725_dn12 = assign23810_e32932_d_n12;
        locals.var_t6__blk725_dn17 = assign23810_e32932_d_n17;

        let (assign23820_e32946, assign23820_e32946_d_n0, assign23820_e32946_d_n2, assign23820_e32946_d_n6, assign23820_e32946_d_n7, assign23820_e32946_d_n10, assign23820_e32946_d_n11, assign23820_e32946_d_n12, assign23820_e32946_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23820_e32942: f64 = (locals.var_t2__blk721 * locals.var_t3__blk722);
        let assign23820_e32944: f64 = (assign23820_e32942 * locals.var_t6__blk725);
        (assign23820_e32944, ((((locals.var_t2__blk721_dn0 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn0)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn0)), ((((locals.var_t2__blk721_dn2 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn2)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn2)), ((((locals.var_t2__blk721_dn6 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn6)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn6)), ((((locals.var_t2__blk721_dn7 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn7)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn7)), ((((locals.var_t2__blk721_dn10 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn10)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn10)), ((((locals.var_t2__blk721_dn11 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn11)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn11)), ((((locals.var_t2__blk721_dn12 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn12)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn12)), ((((locals.var_t2__blk721_dn17 * locals.var_t3__blk722) + (locals.var_t2__blk721 * locals.var_t3__blk722_dn17)) * locals.var_t6__blk725) + (assign23820_e32942 * locals.var_t6__blk725_dn17)),)
    } else {
        (locals.var_t4__blk723, locals.var_t4__blk723_dn0, locals.var_t4__blk723_dn2, locals.var_t4__blk723_dn6, locals.var_t4__blk723_dn7, locals.var_t4__blk723_dn10, locals.var_t4__blk723_dn11, locals.var_t4__blk723_dn12, locals.var_t4__blk723_dn17,)
    }
};
        locals.var_t4__blk723 = assign23820_e32946;
        locals.var_t4__blk723_dn0 = assign23820_e32946_d_n0;
        locals.var_t4__blk723_dn2 = assign23820_e32946_d_n2;
        locals.var_t4__blk723_dn6 = assign23820_e32946_d_n6;
        locals.var_t4__blk723_dn7 = assign23820_e32946_d_n7;
        locals.var_t4__blk723_dn10 = assign23820_e32946_d_n10;
        locals.var_t4__blk723_dn11 = assign23820_e32946_d_n11;
        locals.var_t4__blk723_dn12 = assign23820_e32946_d_n12;
        locals.var_t4__blk723_dn17 = assign23820_e32946_d_n17;

        let (assign23830_e32960, assign23830_e32960_d_n0, assign23830_e32960_d_n2, assign23830_e32960_d_n6, assign23830_e32960_d_n7, assign23830_e32960_d_n10, assign23830_e32960_d_n11, assign23830_e32960_d_n12, assign23830_e32960_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23830_e32956: f64 = (locals.var_t4__blk723 * locals.var_etun);
        let assign23830_e32958: f64 = (assign23830_e32956 * locals.var_etun);
        (assign23830_e32958, ((((locals.var_t4__blk723_dn0 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn0)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn0)), ((((locals.var_t4__blk723_dn2 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn2)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn2)), ((((locals.var_t4__blk723_dn6 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn6)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn6)), ((((locals.var_t4__blk723_dn7 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn7)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn7)), ((((locals.var_t4__blk723_dn10 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn10)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn10)), ((((locals.var_t4__blk723_dn11 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn11)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn11)), ((((locals.var_t4__blk723_dn12 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn12)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn12)), ((((locals.var_t4__blk723_dn17 * locals.var_etun) + (locals.var_t4__blk723 * locals.var_etun_dn17)) * locals.var_etun) + (assign23830_e32956 * locals.var_etun_dn17)),)
    } else {
        (locals.var_t10__blk728, locals.var_t10__blk728_dn0, locals.var_t10__blk728_dn2, locals.var_t10__blk728_dn6, locals.var_t10__blk728_dn7, locals.var_t10__blk728_dn10, locals.var_t10__blk728_dn11, locals.var_t10__blk728_dn12, locals.var_t10__blk728_dn17,)
    }
};
        locals.var_t10__blk728 = assign23830_e32960;
        locals.var_t10__blk728_dn0 = assign23830_e32960_d_n0;
        locals.var_t10__blk728_dn2 = assign23830_e32960_d_n2;
        locals.var_t10__blk728_dn6 = assign23830_e32960_d_n6;
        locals.var_t10__blk728_dn7 = assign23830_e32960_d_n7;
        locals.var_t10__blk728_dn10 = assign23830_e32960_d_n10;
        locals.var_t10__blk728_dn11 = assign23830_e32960_d_n11;
        locals.var_t10__blk728_dn12 = assign23830_e32960_d_n12;
        locals.var_t10__blk728_dn17 = assign23830_e32960_d_n17;

        let (assign23840_e32974, assign23840_e32974_d_n0, assign23840_e32974_d_n2, assign23840_e32974_d_n6, assign23840_e32974_d_n7, assign23840_e32974_d_n10, assign23840_e32974_d_n11, assign23840_e32974_d_n12, assign23840_e32974_d_n17,) = {
    if (((locals.var_guard738 == 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign23840_e32970: f64 = (locals.var_t7__blk726 * locals.var_t9__blk727);
        let assign23840_e32972: f64 = (assign23840_e32970 * locals.var_t10__blk728);
        (assign23840_e32972, ((((locals.var_t7__blk726_dn0 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn0)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn0)), ((((locals.var_t7__blk726_dn2 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn2)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn2)), ((((locals.var_t7__blk726_dn6 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn6)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn6)), ((((locals.var_t7__blk726_dn7 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn7)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn7)), ((((locals.var_t7__blk726_dn10 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn10)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn10)), ((((locals.var_t7__blk726_dn11 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn11)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn11)), ((((locals.var_t7__blk726_dn12 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn12)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn12)), ((((locals.var_t7__blk726_dn17 * locals.var_t9__blk727) + (locals.var_t7__blk726 * locals.var_t9__blk727_dn17)) * locals.var_t10__blk728) + (assign23840_e32970 * locals.var_t10__blk728_dn17)),)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23840_e32974;
        locals.var_igate_dn0 = assign23840_e32974_d_n0;
        locals.var_igate_dn2 = assign23840_e32974_d_n2;
        locals.var_igate_dn6 = assign23840_e32974_d_n6;
        locals.var_igate_dn7 = assign23840_e32974_d_n7;
        locals.var_igate_dn10 = assign23840_e32974_d_n10;
        locals.var_igate_dn11 = assign23840_e32974_d_n11;
        locals.var_igate_dn12 = assign23840_e32974_d_n12;
        locals.var_igate_dn17 = assign23840_e32974_d_n17;

        let (assign23850_e32982, assign23850_e32982_d_n0, assign23850_e32982_d_n2, assign23850_e32982_d_n6, assign23850_e32982_d_n7, assign23850_e32982_d_n10, assign23850_e32982_d_n11, assign23850_e32982_d_n12, assign23850_e32982_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard739 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn12, locals.var_igate_dn17,)
    }
};
        locals.var_igate = assign23850_e32982;
        locals.var_igate_dn0 = assign23850_e32982_d_n0;
        locals.var_igate_dn2 = assign23850_e32982_d_n2;
        locals.var_igate_dn6 = assign23850_e32982_d_n6;
        locals.var_igate_dn7 = assign23850_e32982_d_n7;
        locals.var_igate_dn10 = assign23850_e32982_d_n10;
        locals.var_igate_dn11 = assign23850_e32982_d_n11;
        locals.var_igate_dn12 = assign23850_e32982_d_n12;
        locals.var_igate_dn17 = assign23850_e32982_d_n17;

        let (assign23860_e32992, assign23860_e32992_d_n0, assign23860_e32992_d_n2, assign23860_e32992_d_n6, assign23860_e32992_d_n7, assign23860_e32992_d_n10, assign23860_e32992_d_n11, assign23860_e32992_d_n12, assign23860_e32992_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23860_e32986: f64 = (-p.p221);
        let assign23860_e32988: f64 = (assign23860_e32986 * locals.var_vgs);
        let assign23860_e32990: f64 = (assign23860_e32988 + p.p222);
        (assign23860_e32990, 0.0, 0.0, (assign23860_e32986 * locals.var_vgs_dn6), (assign23860_e32986 * locals.var_vgs_dn7), 0.0, (assign23860_e32986 * locals.var_vgs_dn11), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk719, locals.var_t0__blk719_dn0, locals.var_t0__blk719_dn2, locals.var_t0__blk719_dn6, locals.var_t0__blk719_dn7, locals.var_t0__blk719_dn10, locals.var_t0__blk719_dn11, locals.var_t0__blk719_dn12, locals.var_t0__blk719_dn17,)
    }
};
        locals.var_t0__blk719 = assign23860_e32992;
        locals.var_t0__blk719_dn0 = assign23860_e32992_d_n0;
        locals.var_t0__blk719_dn2 = assign23860_e32992_d_n2;
        locals.var_t0__blk719_dn6 = assign23860_e32992_d_n6;
        locals.var_t0__blk719_dn7 = assign23860_e32992_d_n7;
        locals.var_t0__blk719_dn10 = assign23860_e32992_d_n10;
        locals.var_t0__blk719_dn11 = assign23860_e32992_d_n11;
        locals.var_t0__blk719_dn12 = assign23860_e32992_d_n12;
        locals.var_t0__blk719_dn17 = assign23860_e32992_d_n17;

    }

    pub(super) fn stamp_transient_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23870_e33000, assign23870_e33000_d_n0, assign23870_e33000_d_n2, assign23870_e33000_d_n6, assign23870_e33000_d_n7, assign23870_e33000_d_n10, assign23870_e33000_d_n11, assign23870_e33000_d_n12, assign23870_e33000_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23870_e32997: f64 = (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719);
        let assign23870_e32998: f64 = (assign23870_e32997).exp();
        (assign23870_e32998, (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn0)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn2)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn6)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn7)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn10)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn11)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn12)), (assign23870_e32998 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn17)),)
    } else {
        (locals.var_t2__blk721, locals.var_t2__blk721_dn0, locals.var_t2__blk721_dn2, locals.var_t2__blk721_dn6, locals.var_t2__blk721_dn7, locals.var_t2__blk721_dn10, locals.var_t2__blk721_dn11, locals.var_t2__blk721_dn12, locals.var_t2__blk721_dn17,)
    }
};
        locals.var_t2__blk721 = assign23870_e33000;
        locals.var_t2__blk721_dn0 = assign23870_e33000_d_n0;
        locals.var_t2__blk721_dn2 = assign23870_e33000_d_n2;
        locals.var_t2__blk721_dn6 = assign23870_e33000_d_n6;
        locals.var_t2__blk721_dn7 = assign23870_e33000_d_n7;
        locals.var_t2__blk721_dn10 = assign23870_e33000_d_n10;
        locals.var_t2__blk721_dn11 = assign23870_e33000_d_n11;
        locals.var_t2__blk721_dn12 = assign23870_e33000_d_n12;
        locals.var_t2__blk721_dn17 = assign23870_e33000_d_n17;

        let (assign23880_e33009, assign23880_e33009_d_n0, assign23880_e33009_d_n2, assign23880_e33009_d_n6, assign23880_e33009_d_n7, assign23880_e33009_d_n10, assign23880_e33009_d_n11, assign23880_e33009_d_n12, assign23880_e33009_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cgs_tfox0__blk731;
        let assign23880_e33005: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign23880_e33007: f64 = (assign23880_e33005 * __rspice_inv_cse_0);
        (assign23880_e33007, 0.0, 0.0, ((locals.var_vgs_dn6 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_vgs_dn7 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), 0.0, ((locals.var_vgs_dn11 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk719, locals.var_t0__blk719_dn0, locals.var_t0__blk719_dn2, locals.var_t0__blk719_dn6, locals.var_t0__blk719_dn7, locals.var_t0__blk719_dn10, locals.var_t0__blk719_dn11, locals.var_t0__blk719_dn12, locals.var_t0__blk719_dn17,)
    }
};
        locals.var_t0__blk719 = assign23880_e33009;
        locals.var_t0__blk719_dn0 = assign23880_e33009_d_n0;
        locals.var_t0__blk719_dn2 = assign23880_e33009_d_n2;
        locals.var_t0__blk719_dn6 = assign23880_e33009_d_n6;
        locals.var_t0__blk719_dn7 = assign23880_e33009_d_n7;
        locals.var_t0__blk719_dn10 = assign23880_e33009_d_n10;
        locals.var_t0__blk719_dn11 = assign23880_e33009_d_n11;
        locals.var_t0__blk719_dn12 = assign23880_e33009_d_n12;
        locals.var_t0__blk719_dn17 = assign23880_e33009_d_n17;

        let (assign23890_e33016, assign23890_e33016_d_n0, assign23890_e33016_d_n2, assign23890_e33016_d_n6, assign23890_e33016_d_n7, assign23890_e33016_d_n10, assign23890_e33016_d_n11, assign23890_e33016_d_n12, assign23890_e33016_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23890_e33014: f64 = (locals.var_vgs * locals.var_t0__blk719);
        (assign23890_e33014, (locals.var_vgs * locals.var_t0__blk719_dn0), (locals.var_vgs * locals.var_t0__blk719_dn2), ((locals.var_vgs_dn6 * locals.var_t0__blk719) + (locals.var_vgs * locals.var_t0__blk719_dn6)), ((locals.var_vgs_dn7 * locals.var_t0__blk719) + (locals.var_vgs * locals.var_t0__blk719_dn7)), (locals.var_vgs * locals.var_t0__blk719_dn10), ((locals.var_vgs_dn11 * locals.var_t0__blk719) + (locals.var_vgs * locals.var_t0__blk719_dn11)), (locals.var_vgs * locals.var_t0__blk719_dn12), (locals.var_vgs * locals.var_t0__blk719_dn17),)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23890_e33016;
        locals.var_t3__blk722_dn0 = assign23890_e33016_d_n0;
        locals.var_t3__blk722_dn2 = assign23890_e33016_d_n2;
        locals.var_t3__blk722_dn6 = assign23890_e33016_d_n6;
        locals.var_t3__blk722_dn7 = assign23890_e33016_d_n7;
        locals.var_t3__blk722_dn10 = assign23890_e33016_d_n10;
        locals.var_t3__blk722_dn11 = assign23890_e33016_d_n11;
        locals.var_t3__blk722_dn12 = assign23890_e33016_d_n12;
        locals.var_t3__blk722_dn17 = assign23890_e33016_d_n17;

        let (assign23900_e33025, assign23900_e33025_d_n0, assign23900_e33025_d_n2, assign23900_e33025_d_n6, assign23900_e33025_d_n7, assign23900_e33025_d_n10, assign23900_e33025_d_n11, assign23900_e33025_d_n12, assign23900_e33025_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23900_e33021: f64 = (p.p220 / 1000000.0);
        let assign23900_e33023: f64 = (assign23900_e33021 * locals.var_cgs_weff_nf__blk734);
        (assign23900_e33023, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk723, locals.var_t4__blk723_dn0, locals.var_t4__blk723_dn2, locals.var_t4__blk723_dn6, locals.var_t4__blk723_dn7, locals.var_t4__blk723_dn10, locals.var_t4__blk723_dn11, locals.var_t4__blk723_dn12, locals.var_t4__blk723_dn17,)
    }
};
        locals.var_t4__blk723 = assign23900_e33025;
        locals.var_t4__blk723_dn0 = assign23900_e33025_d_n0;
        locals.var_t4__blk723_dn2 = assign23900_e33025_d_n2;
        locals.var_t4__blk723_dn6 = assign23900_e33025_d_n6;
        locals.var_t4__blk723_dn7 = assign23900_e33025_d_n7;
        locals.var_t4__blk723_dn10 = assign23900_e33025_d_n10;
        locals.var_t4__blk723_dn11 = assign23900_e33025_d_n11;
        locals.var_t4__blk723_dn12 = assign23900_e33025_d_n12;
        locals.var_t4__blk723_dn17 = assign23900_e33025_d_n17;

        let (assign23910_e33034, assign23910_e33034_d_n0, assign23910_e33034_d_n2, assign23910_e33034_d_n6, assign23910_e33034_d_n7, assign23910_e33034_d_n10, assign23910_e33034_d_n11, assign23910_e33034_d_n12, assign23910_e33034_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23910_e33030: f64 = (locals.var_t4__blk723 * locals.var_t2__blk721);
        let assign23910_e33032: f64 = (assign23910_e33030 * locals.var_t3__blk722);
        (assign23910_e33032, ((((locals.var_t4__blk723_dn0 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn0)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn0)), ((((locals.var_t4__blk723_dn2 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn2)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn2)), ((((locals.var_t4__blk723_dn6 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn6)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn6)), ((((locals.var_t4__blk723_dn7 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn7)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn7)), ((((locals.var_t4__blk723_dn10 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn10)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn10)), ((((locals.var_t4__blk723_dn11 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn11)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn11)), ((((locals.var_t4__blk723_dn12 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn12)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn12)), ((((locals.var_t4__blk723_dn17 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn17)) * locals.var_t3__blk722) + (assign23910_e33030 * locals.var_t3__blk722_dn17)),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn12, locals.var_igs_dn17,)
    }
};
        locals.var_igs = assign23910_e33034;
        locals.var_igs_dn0 = assign23910_e33034_d_n0;
        locals.var_igs_dn2 = assign23910_e33034_d_n2;
        locals.var_igs_dn6 = assign23910_e33034_d_n6;
        locals.var_igs_dn7 = assign23910_e33034_d_n7;
        locals.var_igs_dn10 = assign23910_e33034_d_n10;
        locals.var_igs_dn11 = assign23910_e33034_d_n11;
        locals.var_igs_dn12 = assign23910_e33034_d_n12;
        locals.var_igs_dn17 = assign23910_e33034_d_n17;

        let assign23920_e33037: f64 = if locals.var_vgs >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign23920_e33037;

        let (assign23930_e33047, assign23930_e33047_d_n0, assign23930_e33047_d_n2, assign23930_e33047_d_n6, assign23930_e33047_d_n7, assign23930_e33047_d_n10, assign23930_e33047_d_n11, assign23930_e33047_d_n12, assign23930_e33047_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23930_e33044: f64 = (-1.0);
        let assign23930_e33045: f64 = (locals.var_igs * assign23930_e33044);
        (assign23930_e33045, (locals.var_igs_dn0 * assign23930_e33044), (locals.var_igs_dn2 * assign23930_e33044), (locals.var_igs_dn6 * assign23930_e33044), (locals.var_igs_dn7 * assign23930_e33044), (locals.var_igs_dn10 * assign23930_e33044), (locals.var_igs_dn11 * assign23930_e33044), (locals.var_igs_dn12 * assign23930_e33044), (locals.var_igs_dn17 * assign23930_e33044),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn12, locals.var_igs_dn17,)
    }
};
        locals.var_igs = assign23930_e33047;
        locals.var_igs_dn0 = assign23930_e33047_d_n0;
        locals.var_igs_dn2 = assign23930_e33047_d_n2;
        locals.var_igs_dn6 = assign23930_e33047_d_n6;
        locals.var_igs_dn7 = assign23930_e33047_d_n7;
        locals.var_igs_dn10 = assign23930_e33047_d_n10;
        locals.var_igs_dn11 = assign23930_e33047_d_n11;
        locals.var_igs_dn12 = assign23930_e33047_d_n12;
        locals.var_igs_dn17 = assign23930_e33047_d_n17;

        let (assign23940_e33054, assign23940_e33054_d_n0, assign23940_e33054_d_n2, assign23940_e33054_d_n6, assign23940_e33054_d_n7, assign23940_e33054_d_n10, assign23940_e33054_d_n11, assign23940_e33054_d_n12, assign23940_e33054_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23940_e33052: f64 = (locals.var_vgs - locals.var_vds);
        (assign23940_e33052, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (-locals.var_vds_dn10), (locals.var_vgs_dn11 - locals.var_vds_dn11), (-locals.var_vds_dn12), (-locals.var_vds_dn17),)
    } else {
        (locals.var_t1__blk720, locals.var_t1__blk720_dn0, locals.var_t1__blk720_dn2, locals.var_t1__blk720_dn6, locals.var_t1__blk720_dn7, locals.var_t1__blk720_dn10, locals.var_t1__blk720_dn11, locals.var_t1__blk720_dn12, locals.var_t1__blk720_dn17,)
    }
};
        locals.var_t1__blk720 = assign23940_e33054;
        locals.var_t1__blk720_dn0 = assign23940_e33054_d_n0;
        locals.var_t1__blk720_dn2 = assign23940_e33054_d_n2;
        locals.var_t1__blk720_dn6 = assign23940_e33054_d_n6;
        locals.var_t1__blk720_dn7 = assign23940_e33054_d_n7;
        locals.var_t1__blk720_dn10 = assign23940_e33054_d_n10;
        locals.var_t1__blk720_dn11 = assign23940_e33054_d_n11;
        locals.var_t1__blk720_dn12 = assign23940_e33054_d_n12;
        locals.var_t1__blk720_dn17 = assign23940_e33054_d_n17;

        let (assign23950_e33064, assign23950_e33064_d_n0, assign23950_e33064_d_n2, assign23950_e33064_d_n6, assign23950_e33064_d_n7, assign23950_e33064_d_n10, assign23950_e33064_d_n11, assign23950_e33064_d_n12, assign23950_e33064_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23950_e33058: f64 = (-p.p221);
        let assign23950_e33060: f64 = (assign23950_e33058 * locals.var_t1__blk720);
        let assign23950_e33062: f64 = (assign23950_e33060 + p.p222);
        (assign23950_e33062, (assign23950_e33058 * locals.var_t1__blk720_dn0), (assign23950_e33058 * locals.var_t1__blk720_dn2), (assign23950_e33058 * locals.var_t1__blk720_dn6), (assign23950_e33058 * locals.var_t1__blk720_dn7), (assign23950_e33058 * locals.var_t1__blk720_dn10), (assign23950_e33058 * locals.var_t1__blk720_dn11), (assign23950_e33058 * locals.var_t1__blk720_dn12), (assign23950_e33058 * locals.var_t1__blk720_dn17),)
    } else {
        (locals.var_t0__blk719, locals.var_t0__blk719_dn0, locals.var_t0__blk719_dn2, locals.var_t0__blk719_dn6, locals.var_t0__blk719_dn7, locals.var_t0__blk719_dn10, locals.var_t0__blk719_dn11, locals.var_t0__blk719_dn12, locals.var_t0__blk719_dn17,)
    }
};
        locals.var_t0__blk719 = assign23950_e33064;
        locals.var_t0__blk719_dn0 = assign23950_e33064_d_n0;
        locals.var_t0__blk719_dn2 = assign23950_e33064_d_n2;
        locals.var_t0__blk719_dn6 = assign23950_e33064_d_n6;
        locals.var_t0__blk719_dn7 = assign23950_e33064_d_n7;
        locals.var_t0__blk719_dn10 = assign23950_e33064_d_n10;
        locals.var_t0__blk719_dn11 = assign23950_e33064_d_n11;
        locals.var_t0__blk719_dn12 = assign23950_e33064_d_n12;
        locals.var_t0__blk719_dn17 = assign23950_e33064_d_n17;

        let (assign23960_e33072, assign23960_e33072_d_n0, assign23960_e33072_d_n2, assign23960_e33072_d_n6, assign23960_e33072_d_n7, assign23960_e33072_d_n10, assign23960_e33072_d_n11, assign23960_e33072_d_n12, assign23960_e33072_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23960_e33069: f64 = (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719);
        let assign23960_e33070: f64 = (assign23960_e33069).exp();
        (assign23960_e33070, (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn0)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn2)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn6)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn7)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn10)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn11)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn12)), (assign23960_e33070 * (locals.var_cgs_tfox0__blk731 * locals.var_t0__blk719_dn17)),)
    } else {
        (locals.var_t2__blk721, locals.var_t2__blk721_dn0, locals.var_t2__blk721_dn2, locals.var_t2__blk721_dn6, locals.var_t2__blk721_dn7, locals.var_t2__blk721_dn10, locals.var_t2__blk721_dn11, locals.var_t2__blk721_dn12, locals.var_t2__blk721_dn17,)
    }
};
        locals.var_t2__blk721 = assign23960_e33072;
        locals.var_t2__blk721_dn0 = assign23960_e33072_d_n0;
        locals.var_t2__blk721_dn2 = assign23960_e33072_d_n2;
        locals.var_t2__blk721_dn6 = assign23960_e33072_d_n6;
        locals.var_t2__blk721_dn7 = assign23960_e33072_d_n7;
        locals.var_t2__blk721_dn10 = assign23960_e33072_d_n10;
        locals.var_t2__blk721_dn11 = assign23960_e33072_d_n11;
        locals.var_t2__blk721_dn12 = assign23960_e33072_d_n12;
        locals.var_t2__blk721_dn17 = assign23960_e33072_d_n17;

        let (assign23970_e33081, assign23970_e33081_d_n0, assign23970_e33081_d_n2, assign23970_e33081_d_n6, assign23970_e33081_d_n7, assign23970_e33081_d_n10, assign23970_e33081_d_n11, assign23970_e33081_d_n12, assign23970_e33081_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_cgs_tfox0__blk731;
        let assign23970_e33077: f64 = (locals.var_t1__blk720 * __rspice_inv_cse_1);
        let assign23970_e33079: f64 = (assign23970_e33077 * __rspice_inv_cse_1);
        (assign23970_e33079, ((locals.var_t1__blk720_dn0 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn2 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn6 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn7 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn10 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn11 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn12 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731), ((locals.var_t1__blk720_dn17 / locals.var_cgs_tfox0__blk731) / locals.var_cgs_tfox0__blk731),)
    } else {
        (locals.var_t0__blk719, locals.var_t0__blk719_dn0, locals.var_t0__blk719_dn2, locals.var_t0__blk719_dn6, locals.var_t0__blk719_dn7, locals.var_t0__blk719_dn10, locals.var_t0__blk719_dn11, locals.var_t0__blk719_dn12, locals.var_t0__blk719_dn17,)
    }
};
        locals.var_t0__blk719 = assign23970_e33081;
        locals.var_t0__blk719_dn0 = assign23970_e33081_d_n0;
        locals.var_t0__blk719_dn2 = assign23970_e33081_d_n2;
        locals.var_t0__blk719_dn6 = assign23970_e33081_d_n6;
        locals.var_t0__blk719_dn7 = assign23970_e33081_d_n7;
        locals.var_t0__blk719_dn10 = assign23970_e33081_d_n10;
        locals.var_t0__blk719_dn11 = assign23970_e33081_d_n11;
        locals.var_t0__blk719_dn12 = assign23970_e33081_d_n12;
        locals.var_t0__blk719_dn17 = assign23970_e33081_d_n17;

        let (assign23980_e33088, assign23980_e33088_d_n0, assign23980_e33088_d_n2, assign23980_e33088_d_n6, assign23980_e33088_d_n7, assign23980_e33088_d_n10, assign23980_e33088_d_n11, assign23980_e33088_d_n12, assign23980_e33088_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23980_e33086: f64 = (locals.var_t1__blk720 * locals.var_t0__blk719);
        (assign23980_e33086, ((locals.var_t1__blk720_dn0 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn0)), ((locals.var_t1__blk720_dn2 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn2)), ((locals.var_t1__blk720_dn6 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn6)), ((locals.var_t1__blk720_dn7 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn7)), ((locals.var_t1__blk720_dn10 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn10)), ((locals.var_t1__blk720_dn11 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn11)), ((locals.var_t1__blk720_dn12 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn12)), ((locals.var_t1__blk720_dn17 * locals.var_t0__blk719) + (locals.var_t1__blk720 * locals.var_t0__blk719_dn17)),)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign23980_e33088;
        locals.var_t3__blk722_dn0 = assign23980_e33088_d_n0;
        locals.var_t3__blk722_dn2 = assign23980_e33088_d_n2;
        locals.var_t3__blk722_dn6 = assign23980_e33088_d_n6;
        locals.var_t3__blk722_dn7 = assign23980_e33088_d_n7;
        locals.var_t3__blk722_dn10 = assign23980_e33088_d_n10;
        locals.var_t3__blk722_dn11 = assign23980_e33088_d_n11;
        locals.var_t3__blk722_dn12 = assign23980_e33088_d_n12;
        locals.var_t3__blk722_dn17 = assign23980_e33088_d_n17;

        let (assign23990_e33097, assign23990_e33097_d_n0, assign23990_e33097_d_n2, assign23990_e33097_d_n6, assign23990_e33097_d_n7, assign23990_e33097_d_n10, assign23990_e33097_d_n11, assign23990_e33097_d_n12, assign23990_e33097_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign23990_e33093: f64 = (p.p220 / 1000000.0);
        let assign23990_e33095: f64 = (assign23990_e33093 * locals.var_cgs_weff_nf__blk734);
        (assign23990_e33095, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk723, locals.var_t4__blk723_dn0, locals.var_t4__blk723_dn2, locals.var_t4__blk723_dn6, locals.var_t4__blk723_dn7, locals.var_t4__blk723_dn10, locals.var_t4__blk723_dn11, locals.var_t4__blk723_dn12, locals.var_t4__blk723_dn17,)
    }
};
        locals.var_t4__blk723 = assign23990_e33097;
        locals.var_t4__blk723_dn0 = assign23990_e33097_d_n0;
        locals.var_t4__blk723_dn2 = assign23990_e33097_d_n2;
        locals.var_t4__blk723_dn6 = assign23990_e33097_d_n6;
        locals.var_t4__blk723_dn7 = assign23990_e33097_d_n7;
        locals.var_t4__blk723_dn10 = assign23990_e33097_d_n10;
        locals.var_t4__blk723_dn11 = assign23990_e33097_d_n11;
        locals.var_t4__blk723_dn12 = assign23990_e33097_d_n12;
        locals.var_t4__blk723_dn17 = assign23990_e33097_d_n17;

        let (assign24000_e33106, assign24000_e33106_d_n0, assign24000_e33106_d_n2, assign24000_e33106_d_n6, assign24000_e33106_d_n7, assign24000_e33106_d_n10, assign24000_e33106_d_n11, assign24000_e33106_d_n12, assign24000_e33106_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24000_e33102: f64 = (locals.var_t4__blk723 * locals.var_t2__blk721);
        let assign24000_e33104: f64 = (assign24000_e33102 * locals.var_t3__blk722);
        (assign24000_e33104, ((((locals.var_t4__blk723_dn0 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn0)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn0)), ((((locals.var_t4__blk723_dn2 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn2)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn2)), ((((locals.var_t4__blk723_dn6 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn6)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn6)), ((((locals.var_t4__blk723_dn7 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn7)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn7)), ((((locals.var_t4__blk723_dn10 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn10)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn10)), ((((locals.var_t4__blk723_dn11 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn11)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn11)), ((((locals.var_t4__blk723_dn12 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn12)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn12)), ((((locals.var_t4__blk723_dn17 * locals.var_t2__blk721) + (locals.var_t4__blk723 * locals.var_t2__blk721_dn17)) * locals.var_t3__blk722) + (assign24000_e33102 * locals.var_t3__blk722_dn17)),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn12, locals.var_igd_dn17,)
    }
};
        locals.var_igd = assign24000_e33106;
        locals.var_igd_dn0 = assign24000_e33106_d_n0;
        locals.var_igd_dn2 = assign24000_e33106_d_n2;
        locals.var_igd_dn6 = assign24000_e33106_d_n6;
        locals.var_igd_dn7 = assign24000_e33106_d_n7;
        locals.var_igd_dn10 = assign24000_e33106_d_n10;
        locals.var_igd_dn11 = assign24000_e33106_d_n11;
        locals.var_igd_dn12 = assign24000_e33106_d_n12;
        locals.var_igd_dn17 = assign24000_e33106_d_n17;

        let assign24010_e33109: f64 = if locals.var_t1__blk720 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign24010_e33109;

        let (assign24020_e33119, assign24020_e33119_d_n0, assign24020_e33119_d_n2, assign24020_e33119_d_n6, assign24020_e33119_d_n7, assign24020_e33119_d_n10, assign24020_e33119_d_n11, assign24020_e33119_d_n12, assign24020_e33119_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard744 != 0.0)) {
        let assign24020_e33116: f64 = (-1.0);
        let assign24020_e33117: f64 = (locals.var_igd * assign24020_e33116);
        (assign24020_e33117, (locals.var_igd_dn0 * assign24020_e33116), (locals.var_igd_dn2 * assign24020_e33116), (locals.var_igd_dn6 * assign24020_e33116), (locals.var_igd_dn7 * assign24020_e33116), (locals.var_igd_dn10 * assign24020_e33116), (locals.var_igd_dn11 * assign24020_e33116), (locals.var_igd_dn12 * assign24020_e33116), (locals.var_igd_dn17 * assign24020_e33116),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn12, locals.var_igd_dn17,)
    }
};
        locals.var_igd = assign24020_e33119;
        locals.var_igd_dn0 = assign24020_e33119_d_n0;
        locals.var_igd_dn2 = assign24020_e33119_d_n2;
        locals.var_igd_dn6 = assign24020_e33119_d_n6;
        locals.var_igd_dn7 = assign24020_e33119_d_n7;
        locals.var_igd_dn10 = assign24020_e33119_d_n10;
        locals.var_igd_dn11 = assign24020_e33119_d_n11;
        locals.var_igd_dn12 = assign24020_e33119_d_n12;
        locals.var_igd_dn17 = assign24020_e33119_d_n17;

        let (assign24030_e33133, assign24030_e33133_d_n0, assign24030_e33133_d_n2, assign24030_e33133_d_n6, assign24030_e33133_d_n7, assign24030_e33133_d_n10, assign24030_e33133_d_n11, assign24030_e33133_d_n12, assign24030_e33133_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24030_e33123: f64 = (-locals.var_vgs);
        let assign24030_e33125: f64 = (assign24030_e33123 + locals.var_vbsp);
        let assign24030_e33127: f64 = (assign24030_e33125 + locals.var_vfb);
        let assign24030_e33129: f64 = (assign24030_e33127 + p.p225);
        let assign24030_e33131: f64 = (assign24030_e33129 / locals.var_cgs_tfox0__blk731);
        (assign24030_e33131, (locals.var_vbsp_dn0 / locals.var_cgs_tfox0__blk731), (locals.var_vbsp_dn2 / locals.var_cgs_tfox0__blk731), (((-locals.var_vgs_dn6) + locals.var_vbsp_dn6) / locals.var_cgs_tfox0__blk731), (((-locals.var_vgs_dn7) + locals.var_vbsp_dn7) / locals.var_cgs_tfox0__blk731), (locals.var_vbsp_dn10 / locals.var_cgs_tfox0__blk731), (((-locals.var_vgs_dn11) + locals.var_vbsp_dn11) / locals.var_cgs_tfox0__blk731), (locals.var_vbsp_dn12 / locals.var_cgs_tfox0__blk731), (locals.var_vbsp_dn17 / locals.var_cgs_tfox0__blk731),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24030_e33133;
        locals.var_etun_dn0 = assign24030_e33133_d_n0;
        locals.var_etun_dn2 = assign24030_e33133_d_n2;
        locals.var_etun_dn6 = assign24030_e33133_d_n6;
        locals.var_etun_dn7 = assign24030_e33133_d_n7;
        locals.var_etun_dn10 = assign24030_e33133_d_n10;
        locals.var_etun_dn11 = assign24030_e33133_d_n11;
        locals.var_etun_dn12 = assign24030_e33133_d_n12;
        locals.var_etun_dn17 = assign24030_e33133_d_n17;

        let (assign24040_e33147, assign24040_e33147_d_n0, assign24040_e33147_d_n2, assign24040_e33147_d_n6, assign24040_e33147_d_n7, assign24040_e33147_d_n10, assign24040_e33147_d_n11, assign24040_e33147_d_n12, assign24040_e33147_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24040_e33138: f64 = (locals.var_etun * locals.var_etun);
        let assign24040_e33141: f64 = (4.0 * 0.01);
        let assign24040_e33143: f64 = (assign24040_e33141 * 0.01);
        let assign24040_e33144: f64 = (assign24040_e33138 + assign24040_e33143);
        let assign24040_e33145: f64 = (assign24040_e33144).sqrt();
        (assign24040_e33145, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign24040_e33145)), (((locals.var_etun_dn17 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn17)) / (2.0 * assign24040_e33145)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24040_e33147;
        locals.var_tmf1_dn0 = assign24040_e33147_d_n0;
        locals.var_tmf1_dn2 = assign24040_e33147_d_n2;
        locals.var_tmf1_dn6 = assign24040_e33147_d_n6;
        locals.var_tmf1_dn7 = assign24040_e33147_d_n7;
        locals.var_tmf1_dn10 = assign24040_e33147_d_n10;
        locals.var_tmf1_dn11 = assign24040_e33147_d_n11;
        locals.var_tmf1_dn12 = assign24040_e33147_d_n12;
        locals.var_tmf1_dn17 = assign24040_e33147_d_n17;

        let (assign24050_e33160, assign24050_e33160_d_n0, assign24050_e33160_d_n2, assign24050_e33160_d_n6, assign24050_e33160_d_n7, assign24050_e33160_d_n10, assign24050_e33160_d_n11, assign24050_e33160_d_n12, assign24050_e33160_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24050_e33153: f64 = (locals.var_etun + locals.var_tmf1);
        let assign24050_e33154: f64 = (0.5 * assign24050_e33153);
        let assign24050_e33157: f64 = (1e-10 * 0.01);
        let assign24050_e33158: f64 = (assign24050_e33154 + assign24050_e33157);
        (assign24050_e33158, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24050_e33160;
        locals.var_etun_dn0 = assign24050_e33160_d_n0;
        locals.var_etun_dn2 = assign24050_e33160_d_n2;
        locals.var_etun_dn6 = assign24050_e33160_d_n6;
        locals.var_etun_dn7 = assign24050_e33160_d_n7;
        locals.var_etun_dn10 = assign24050_e33160_d_n10;
        locals.var_etun_dn11 = assign24050_e33160_d_n11;
        locals.var_etun_dn12 = assign24050_e33160_d_n12;
        locals.var_etun_dn17 = assign24050_e33160_d_n17;

        let assign24060_e33163: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign24060_e33163;

        let (assign24070_e33170, assign24070_e33170_d_n0, assign24070_e33170_d_n2, assign24070_e33170_d_n6, assign24070_e33170_d_n7, assign24070_e33170_d_n10, assign24070_e33170_d_n11, assign24070_e33170_d_n12, assign24070_e33170_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard745 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24070_e33170;
        locals.var_etun_dn0 = assign24070_e33170_d_n0;
        locals.var_etun_dn2 = assign24070_e33170_d_n2;
        locals.var_etun_dn6 = assign24070_e33170_d_n6;
        locals.var_etun_dn7 = assign24070_e33170_d_n7;
        locals.var_etun_dn10 = assign24070_e33170_d_n10;
        locals.var_etun_dn11 = assign24070_e33170_d_n11;
        locals.var_etun_dn12 = assign24070_e33170_d_n12;
        locals.var_etun_dn17 = assign24070_e33170_d_n17;

        let (assign24080_e33177, assign24080_e33177_d_n0, assign24080_e33177_d_n2, assign24080_e33177_d_n6, assign24080_e33177_d_n7, assign24080_e33177_d_n10, assign24080_e33177_d_n11, assign24080_e33177_d_n12, assign24080_e33177_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24080_e33175: f64 = (locals.var_etun + 1e-50);
        (assign24080_e33175, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24080_e33177;
        locals.var_etun_dn0 = assign24080_e33177_d_n0;
        locals.var_etun_dn2 = assign24080_e33177_d_n2;
        locals.var_etun_dn6 = assign24080_e33177_d_n6;
        locals.var_etun_dn7 = assign24080_e33177_d_n7;
        locals.var_etun_dn10 = assign24080_e33177_d_n10;
        locals.var_etun_dn11 = assign24080_e33177_d_n11;
        locals.var_etun_dn12 = assign24080_e33177_d_n12;
        locals.var_etun_dn17 = assign24080_e33177_d_n17;

        let (assign24090_e33185, assign24090_e33185_d_n0, assign24090_e33185_d_n2, assign24090_e33185_d_n6, assign24090_e33185_d_n7, assign24090_e33185_d_n10, assign24090_e33185_d_n11, assign24090_e33185_d_n12, assign24090_e33185_d_n17,) = {
    if (locals.var_guard738 == 0.0) {
        let assign24090_e33181: f64 = (-p.p224);
        let assign24090_e33183: f64 = (assign24090_e33181 / locals.var_etun);
        (assign24090_e33183, (-((assign24090_e33181 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn11) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn12) / (locals.var_etun * locals.var_etun))), (-((assign24090_e33181 * locals.var_etun_dn17) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1__blk720, locals.var_t1__blk720_dn0, locals.var_t1__blk720_dn2, locals.var_t1__blk720_dn6, locals.var_t1__blk720_dn7, locals.var_t1__blk720_dn10, locals.var_t1__blk720_dn11, locals.var_t1__blk720_dn12, locals.var_t1__blk720_dn17,)
    }
};
        locals.var_t1__blk720 = assign24090_e33185;
        locals.var_t1__blk720_dn0 = assign24090_e33185_d_n0;
        locals.var_t1__blk720_dn2 = assign24090_e33185_d_n2;
        locals.var_t1__blk720_dn6 = assign24090_e33185_d_n6;
        locals.var_t1__blk720_dn7 = assign24090_e33185_d_n7;
        locals.var_t1__blk720_dn10 = assign24090_e33185_d_n10;
        locals.var_t1__blk720_dn11 = assign24090_e33185_d_n11;
        locals.var_t1__blk720_dn12 = assign24090_e33185_d_n12;
        locals.var_t1__blk720_dn17 = assign24090_e33185_d_n17;

        let assign24100_e33188: f64 = (-34.0);
        let assign24100_e33189: f64 = if locals.var_t1__blk720 < assign24100_e33188 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign24100_e33189;

        let (assign24110_e33196, assign24110_e33196_d_n0, assign24110_e33196_d_n2, assign24110_e33196_d_n6, assign24110_e33196_d_n7, assign24110_e33196_d_n10, assign24110_e33196_d_n11, assign24110_e33196_d_n12, assign24110_e33196_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard746 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn12, locals.var_igb_dn17,)
    }
};
        locals.var_igb = assign24110_e33196;
        locals.var_igb_dn0 = assign24110_e33196_d_n0;
        locals.var_igb_dn2 = assign24110_e33196_d_n2;
        locals.var_igb_dn6 = assign24110_e33196_d_n6;
        locals.var_igb_dn7 = assign24110_e33196_d_n7;
        locals.var_igb_dn10 = assign24110_e33196_d_n10;
        locals.var_igb_dn11 = assign24110_e33196_d_n11;
        locals.var_igb_dn12 = assign24110_e33196_d_n12;
        locals.var_igb_dn17 = assign24110_e33196_d_n17;

        let (assign24120_e33205, assign24120_e33205_d_n0, assign24120_e33205_d_n2, assign24120_e33205_d_n6, assign24120_e33205_d_n7, assign24120_e33205_d_n10, assign24120_e33205_d_n11, assign24120_e33205_d_n12, assign24120_e33205_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign24120_e33203: f64 = (locals.var_t1__blk720).exp();
        (assign24120_e33203, (assign24120_e33203 * locals.var_t1__blk720_dn0), (assign24120_e33203 * locals.var_t1__blk720_dn2), (assign24120_e33203 * locals.var_t1__blk720_dn6), (assign24120_e33203 * locals.var_t1__blk720_dn7), (assign24120_e33203 * locals.var_t1__blk720_dn10), (assign24120_e33203 * locals.var_t1__blk720_dn11), (assign24120_e33203 * locals.var_t1__blk720_dn12), (assign24120_e33203 * locals.var_t1__blk720_dn17),)
    } else {
        (locals.var_t2__blk721, locals.var_t2__blk721_dn0, locals.var_t2__blk721_dn2, locals.var_t2__blk721_dn6, locals.var_t2__blk721_dn7, locals.var_t2__blk721_dn10, locals.var_t2__blk721_dn11, locals.var_t2__blk721_dn12, locals.var_t2__blk721_dn17,)
    }
};
        locals.var_t2__blk721 = assign24120_e33205;
        locals.var_t2__blk721_dn0 = assign24120_e33205_d_n0;
        locals.var_t2__blk721_dn2 = assign24120_e33205_d_n2;
        locals.var_t2__blk721_dn6 = assign24120_e33205_d_n6;
        locals.var_t2__blk721_dn7 = assign24120_e33205_d_n7;
        locals.var_t2__blk721_dn10 = assign24120_e33205_d_n10;
        locals.var_t2__blk721_dn11 = assign24120_e33205_d_n11;
        locals.var_t2__blk721_dn12 = assign24120_e33205_d_n12;
        locals.var_t2__blk721_dn17 = assign24120_e33205_d_n17;

        let (assign24130_e33217, assign24130_e33217_d_n0, assign24130_e33217_d_n2, assign24130_e33217_d_n6, assign24130_e33217_d_n7, assign24130_e33217_d_n10, assign24130_e33217_d_n11, assign24130_e33217_d_n12, assign24130_e33217_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign24130_e33213: f64 = (p.p223 * locals.var_cgs_weff_nf__blk734);
        let assign24130_e33215: f64 = (assign24130_e33213 * locals.var_cgs_leff__blk733);
        (assign24130_e33215, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk722, locals.var_t3__blk722_dn0, locals.var_t3__blk722_dn2, locals.var_t3__blk722_dn6, locals.var_t3__blk722_dn7, locals.var_t3__blk722_dn10, locals.var_t3__blk722_dn11, locals.var_t3__blk722_dn12, locals.var_t3__blk722_dn17,)
    }
};
        locals.var_t3__blk722 = assign24130_e33217;
        locals.var_t3__blk722_dn0 = assign24130_e33217_d_n0;
        locals.var_t3__blk722_dn2 = assign24130_e33217_d_n2;
        locals.var_t3__blk722_dn6 = assign24130_e33217_d_n6;
        locals.var_t3__blk722_dn7 = assign24130_e33217_d_n7;
        locals.var_t3__blk722_dn10 = assign24130_e33217_d_n10;
        locals.var_t3__blk722_dn11 = assign24130_e33217_d_n11;
        locals.var_t3__blk722_dn12 = assign24130_e33217_d_n12;
        locals.var_t3__blk722_dn17 = assign24130_e33217_d_n17;

        let (assign24140_e33231, assign24140_e33231_d_n0, assign24140_e33231_d_n2, assign24140_e33231_d_n6, assign24140_e33231_d_n7, assign24140_e33231_d_n10, assign24140_e33231_d_n11, assign24140_e33231_d_n12, assign24140_e33231_d_n17,) = {
    if ((locals.var_guard738 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign24140_e33225: f64 = (locals.var_t3__blk722 * locals.var_etun);
        let assign24140_e33227: f64 = (assign24140_e33225 * locals.var_etun);
        let assign24140_e33229: f64 = (assign24140_e33227 * locals.var_t2__blk721);
        (assign24140_e33229, ((((((locals.var_t3__blk722_dn0 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn0)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn0)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn0)), ((((((locals.var_t3__blk722_dn2 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn2)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn2)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn2)), ((((((locals.var_t3__blk722_dn6 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn6)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn6)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn6)), ((((((locals.var_t3__blk722_dn7 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn7)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn7)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn7)), ((((((locals.var_t3__blk722_dn10 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn10)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn10)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn10)), ((((((locals.var_t3__blk722_dn11 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn11)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn11)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn11)), ((((((locals.var_t3__blk722_dn12 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn12)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn12)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn12)), ((((((locals.var_t3__blk722_dn17 * locals.var_etun) + (locals.var_t3__blk722 * locals.var_etun_dn17)) * locals.var_etun) + (assign24140_e33225 * locals.var_etun_dn17)) * locals.var_t2__blk721) + (assign24140_e33227 * locals.var_t2__blk721_dn17)),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn12, locals.var_igb_dn17,)
    }
};
        locals.var_igb = assign24140_e33231;
        locals.var_igb_dn0 = assign24140_e33231_d_n0;
        locals.var_igb_dn2 = assign24140_e33231_d_n2;
        locals.var_igb_dn6 = assign24140_e33231_d_n6;
        locals.var_igb_dn7 = assign24140_e33231_d_n7;
        locals.var_igb_dn10 = assign24140_e33231_d_n10;
        locals.var_igb_dn11 = assign24140_e33231_d_n11;
        locals.var_igb_dn12 = assign24140_e33231_d_n12;
        locals.var_igb_dn17 = assign24140_e33231_d_n17;

        let (assign24150_e33236,) = {
    if (locals.var_guard738 == 0.0) {
        (0.5,)
    } else {
        (locals.var_glpart1,)
    }
};
        locals.var_glpart1 = assign24150_e33236;

        let assign24160_e33239: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign24160_e33239;

        let (assign24170_e33243, assign24170_e33243_d_n0, assign24170_e33243_d_n2, assign24170_e33243_d_n6, assign24170_e33243_d_n7, assign24170_e33243_d_n10, assign24170_e33243_d_n11, assign24170_e33243_d_n12, assign24170_e33243_d_n17,) = {
    if (locals.var_guard754 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24170_e33243;
        locals.var_igidl_dn0 = assign24170_e33243_d_n0;
        locals.var_igidl_dn2 = assign24170_e33243_d_n2;
        locals.var_igidl_dn6 = assign24170_e33243_d_n6;
        locals.var_igidl_dn7 = assign24170_e33243_d_n7;
        locals.var_igidl_dn10 = assign24170_e33243_d_n10;
        locals.var_igidl_dn11 = assign24170_e33243_d_n11;
        locals.var_igidl_dn12 = assign24170_e33243_d_n12;
        locals.var_igidl_dn17 = assign24170_e33243_d_n17;

    }

    pub(super) fn stamp_transient_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24180_e33260, assign24180_e33260_d_n0, assign24180_e33260_d_n2, assign24180_e33260_d_n6, assign24180_e33260_d_n7, assign24180_e33260_d_n10, assign24180_e33260_d_n11, assign24180_e33260_d_n12, assign24180_e33260_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24180_e33249: f64 = (locals.var_vds + p.p210);
        let assign24180_e33250: f64 = (p.p209 * assign24180_e33249);
        let assign24180_e33252: f64 = (assign24180_e33250 - locals.var_vgs);
        let assign24180_e33255: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24180_e33257: f64 = (assign24180_e33255 * p.p211);
        let assign24180_e33258: f64 = (assign24180_e33252 + assign24180_e33257);
        (assign24180_e33258, ((p.p209 * locals.var_vds_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), ((p.p209 * locals.var_vds_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * locals.var_vds_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * locals.var_vds_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), ((p.p209 * locals.var_vds_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * locals.var_vds_dn11) - locals.var_vgs_dn11) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), ((p.p209 * locals.var_vds_dn12) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), ((p.p209 * locals.var_vds_dn17) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk747, locals.var_t1__blk747_dn0, locals.var_t1__blk747_dn2, locals.var_t1__blk747_dn6, locals.var_t1__blk747_dn7, locals.var_t1__blk747_dn10, locals.var_t1__blk747_dn11, locals.var_t1__blk747_dn12, locals.var_t1__blk747_dn17,)
    }
};
        locals.var_t1__blk747 = assign24180_e33260;
        locals.var_t1__blk747_dn0 = assign24180_e33260_d_n0;
        locals.var_t1__blk747_dn2 = assign24180_e33260_d_n2;
        locals.var_t1__blk747_dn6 = assign24180_e33260_d_n6;
        locals.var_t1__blk747_dn7 = assign24180_e33260_d_n7;
        locals.var_t1__blk747_dn10 = assign24180_e33260_d_n10;
        locals.var_t1__blk747_dn11 = assign24180_e33260_d_n11;
        locals.var_t1__blk747_dn12 = assign24180_e33260_d_n12;
        locals.var_t1__blk747_dn17 = assign24180_e33260_d_n17;

        let (assign24190_e33267, assign24190_e33267_d_n0, assign24190_e33267_d_n2, assign24190_e33267_d_n6, assign24190_e33267_d_n7, assign24190_e33267_d_n10, assign24190_e33267_d_n11, assign24190_e33267_d_n12, assign24190_e33267_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24190_e33265: f64 = (1.0 / locals.var_tfox0);
        (assign24190_e33265, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk748, locals.var_t2__blk748_dn0, locals.var_t2__blk748_dn2, locals.var_t2__blk748_dn6, locals.var_t2__blk748_dn7, locals.var_t2__blk748_dn10, locals.var_t2__blk748_dn11, locals.var_t2__blk748_dn12, locals.var_t2__blk748_dn17,)
    }
};
        locals.var_t2__blk748 = assign24190_e33267;
        locals.var_t2__blk748_dn0 = assign24190_e33267_d_n0;
        locals.var_t2__blk748_dn2 = assign24190_e33267_d_n2;
        locals.var_t2__blk748_dn6 = assign24190_e33267_d_n6;
        locals.var_t2__blk748_dn7 = assign24190_e33267_d_n7;
        locals.var_t2__blk748_dn10 = assign24190_e33267_d_n10;
        locals.var_t2__blk748_dn11 = assign24190_e33267_d_n11;
        locals.var_t2__blk748_dn12 = assign24190_e33267_d_n12;
        locals.var_t2__blk748_dn17 = assign24190_e33267_d_n17;

        let (assign24200_e33274, assign24200_e33274_d_n0, assign24200_e33274_d_n2, assign24200_e33274_d_n6, assign24200_e33274_d_n7, assign24200_e33274_d_n10, assign24200_e33274_d_n11, assign24200_e33274_d_n12, assign24200_e33274_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24200_e33272: f64 = (locals.var_t1__blk747 * locals.var_t2__blk748);
        (assign24200_e33272, ((locals.var_t1__blk747_dn0 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn0)), ((locals.var_t1__blk747_dn2 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn2)), ((locals.var_t1__blk747_dn6 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn6)), ((locals.var_t1__blk747_dn7 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn7)), ((locals.var_t1__blk747_dn10 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn10)), ((locals.var_t1__blk747_dn11 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn11)), ((locals.var_t1__blk747_dn12 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn12)), ((locals.var_t1__blk747_dn17 * locals.var_t2__blk748) + (locals.var_t1__blk747 * locals.var_t2__blk748_dn17)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn12, locals.var_e1_dn17,)
    }
};
        locals.var_e1 = assign24200_e33274;
        locals.var_e1_dn0 = assign24200_e33274_d_n0;
        locals.var_e1_dn2 = assign24200_e33274_d_n2;
        locals.var_e1_dn6 = assign24200_e33274_d_n6;
        locals.var_e1_dn7 = assign24200_e33274_d_n7;
        locals.var_e1_dn10 = assign24200_e33274_d_n10;
        locals.var_e1_dn11 = assign24200_e33274_d_n11;
        locals.var_e1_dn12 = assign24200_e33274_d_n12;
        locals.var_e1_dn17 = assign24200_e33274_d_n17;

        let (assign24210_e33288, assign24210_e33288_d_n0, assign24210_e33288_d_n2, assign24210_e33288_d_n6, assign24210_e33288_d_n7, assign24210_e33288_d_n10, assign24210_e33288_d_n11, assign24210_e33288_d_n12, assign24210_e33288_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24210_e33279: f64 = (locals.var_e1 * locals.var_e1);
        let assign24210_e33282: f64 = (4.0 * 0.01);
        let assign24210_e33284: f64 = (assign24210_e33282 * 0.01);
        let assign24210_e33285: f64 = (assign24210_e33279 + assign24210_e33284);
        let assign24210_e33286: f64 = (assign24210_e33285).sqrt();
        (assign24210_e33286, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn12 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn12)) / (2.0 * assign24210_e33286)), (((locals.var_e1_dn17 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn17)) / (2.0 * assign24210_e33286)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24210_e33288;
        locals.var_tmf1_dn0 = assign24210_e33288_d_n0;
        locals.var_tmf1_dn2 = assign24210_e33288_d_n2;
        locals.var_tmf1_dn6 = assign24210_e33288_d_n6;
        locals.var_tmf1_dn7 = assign24210_e33288_d_n7;
        locals.var_tmf1_dn10 = assign24210_e33288_d_n10;
        locals.var_tmf1_dn11 = assign24210_e33288_d_n11;
        locals.var_tmf1_dn12 = assign24210_e33288_d_n12;
        locals.var_tmf1_dn17 = assign24210_e33288_d_n17;

        let (assign24220_e33301, assign24220_e33301_d_n0, assign24220_e33301_d_n2, assign24220_e33301_d_n6, assign24220_e33301_d_n7, assign24220_e33301_d_n10, assign24220_e33301_d_n11, assign24220_e33301_d_n12, assign24220_e33301_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24220_e33294: f64 = (locals.var_e1 + locals.var_tmf1);
        let assign24220_e33295: f64 = (0.5 * assign24220_e33294);
        let assign24220_e33298: f64 = (1e-10 * 0.01);
        let assign24220_e33299: f64 = (assign24220_e33295 + assign24220_e33298);
        (assign24220_e33299, (0.5 * (locals.var_e1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24220_e33301;
        locals.var_egidl_dn0 = assign24220_e33301_d_n0;
        locals.var_egidl_dn2 = assign24220_e33301_d_n2;
        locals.var_egidl_dn6 = assign24220_e33301_d_n6;
        locals.var_egidl_dn7 = assign24220_e33301_d_n7;
        locals.var_egidl_dn10 = assign24220_e33301_d_n10;
        locals.var_egidl_dn11 = assign24220_e33301_d_n11;
        locals.var_egidl_dn12 = assign24220_e33301_d_n12;
        locals.var_egidl_dn17 = assign24220_e33301_d_n17;

        let assign24230_e33304: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard755 = assign24230_e33304;

        let (assign24240_e33311, assign24240_e33311_d_n0, assign24240_e33311_d_n2, assign24240_e33311_d_n6, assign24240_e33311_d_n7, assign24240_e33311_d_n10, assign24240_e33311_d_n11, assign24240_e33311_d_n12, assign24240_e33311_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard755 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24240_e33311;
        locals.var_egidl_dn0 = assign24240_e33311_d_n0;
        locals.var_egidl_dn2 = assign24240_e33311_d_n2;
        locals.var_egidl_dn6 = assign24240_e33311_d_n6;
        locals.var_egidl_dn7 = assign24240_e33311_d_n7;
        locals.var_egidl_dn10 = assign24240_e33311_d_n10;
        locals.var_egidl_dn11 = assign24240_e33311_d_n11;
        locals.var_egidl_dn12 = assign24240_e33311_d_n12;
        locals.var_egidl_dn17 = assign24240_e33311_d_n17;

        let (assign24250_e33320, assign24250_e33320_d_n0, assign24250_e33320_d_n2, assign24250_e33320_d_n6, assign24250_e33320_d_n7, assign24250_e33320_d_n10, assign24250_e33320_d_n11, assign24250_e33320_d_n12, assign24250_e33320_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24250_e33317: f64 = (locals.var_egidl + 1e-50);
        let assign24250_e33318: f64 = (1.0 / assign24250_e33317);
        (assign24250_e33318, (-(locals.var_egidl_dn0 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn2 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn6 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn7 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn10 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn11 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn12 / (assign24250_e33317 * assign24250_e33317))), (-(locals.var_egidl_dn17 / (assign24250_e33317 * assign24250_e33317))),)
    } else {
        (locals.var_t3__blk750, locals.var_t3__blk750_dn0, locals.var_t3__blk750_dn2, locals.var_t3__blk750_dn6, locals.var_t3__blk750_dn7, locals.var_t3__blk750_dn10, locals.var_t3__blk750_dn11, locals.var_t3__blk750_dn12, locals.var_t3__blk750_dn17,)
    }
};
        locals.var_t3__blk750 = assign24250_e33320;
        locals.var_t3__blk750_dn0 = assign24250_e33320_d_n0;
        locals.var_t3__blk750_dn2 = assign24250_e33320_d_n2;
        locals.var_t3__blk750_dn6 = assign24250_e33320_d_n6;
        locals.var_t3__blk750_dn7 = assign24250_e33320_d_n7;
        locals.var_t3__blk750_dn10 = assign24250_e33320_d_n10;
        locals.var_t3__blk750_dn11 = assign24250_e33320_d_n11;
        locals.var_t3__blk750_dn12 = assign24250_e33320_d_n12;
        locals.var_t3__blk750_dn17 = assign24250_e33320_d_n17;

        let (assign24260_e33330, assign24260_e33330_d_n0, assign24260_e33330_d_n2, assign24260_e33330_d_n6, assign24260_e33330_d_n7, assign24260_e33330_d_n10, assign24260_e33330_d_n11, assign24260_e33330_d_n12, assign24260_e33330_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24260_e33324: f64 = (-p.p208);
        let assign24260_e33326: f64 = (assign24260_e33324 * locals.var_egp32);
        let assign24260_e33328: f64 = (assign24260_e33326 * locals.var_t3__blk750);
        (assign24260_e33328, (((assign24260_e33324 * locals.var_egp32_dn0) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn0)), (((assign24260_e33324 * locals.var_egp32_dn2) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn2)), (((assign24260_e33324 * locals.var_egp32_dn6) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn6)), (((assign24260_e33324 * locals.var_egp32_dn7) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn7)), (((assign24260_e33324 * locals.var_egp32_dn10) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn10)), (((assign24260_e33324 * locals.var_egp32_dn11) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn11)), (((assign24260_e33324 * locals.var_egp32_dn12) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn12)), (((assign24260_e33324 * locals.var_egp32_dn17) * locals.var_t3__blk750) + (assign24260_e33326 * locals.var_t3__blk750_dn17)),)
    } else {
        (locals.var_t0__blk751, locals.var_t0__blk751_dn0, locals.var_t0__blk751_dn2, locals.var_t0__blk751_dn6, locals.var_t0__blk751_dn7, locals.var_t0__blk751_dn10, locals.var_t0__blk751_dn11, locals.var_t0__blk751_dn12, locals.var_t0__blk751_dn17,)
    }
};
        locals.var_t0__blk751 = assign24260_e33330;
        locals.var_t0__blk751_dn0 = assign24260_e33330_d_n0;
        locals.var_t0__blk751_dn2 = assign24260_e33330_d_n2;
        locals.var_t0__blk751_dn6 = assign24260_e33330_d_n6;
        locals.var_t0__blk751_dn7 = assign24260_e33330_d_n7;
        locals.var_t0__blk751_dn10 = assign24260_e33330_d_n10;
        locals.var_t0__blk751_dn11 = assign24260_e33330_d_n11;
        locals.var_t0__blk751_dn12 = assign24260_e33330_d_n12;
        locals.var_t0__blk751_dn17 = assign24260_e33330_d_n17;

        let assign24270_e33333: f64 = (-34.0);
        let assign24270_e33334: f64 = if locals.var_t0__blk751 < assign24270_e33333 { 1.0 } else { 0.0 };
        locals.var_guard756 = assign24270_e33334;

        let (assign24280_e33341, assign24280_e33341_d_n0, assign24280_e33341_d_n2, assign24280_e33341_d_n6, assign24280_e33341_d_n7, assign24280_e33341_d_n10, assign24280_e33341_d_n11, assign24280_e33341_d_n12, assign24280_e33341_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard756 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24280_e33341;
        locals.var_igidl_dn0 = assign24280_e33341_d_n0;
        locals.var_igidl_dn2 = assign24280_e33341_d_n2;
        locals.var_igidl_dn6 = assign24280_e33341_d_n6;
        locals.var_igidl_dn7 = assign24280_e33341_d_n7;
        locals.var_igidl_dn10 = assign24280_e33341_d_n10;
        locals.var_igidl_dn11 = assign24280_e33341_d_n11;
        locals.var_igidl_dn12 = assign24280_e33341_d_n12;
        locals.var_igidl_dn17 = assign24280_e33341_d_n17;

        let (assign24290_e33350, assign24290_e33350_d_n0, assign24290_e33350_d_n2, assign24290_e33350_d_n6, assign24290_e33350_d_n7, assign24290_e33350_d_n10, assign24290_e33350_d_n11, assign24290_e33350_d_n12, assign24290_e33350_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign24290_e33348: f64 = (locals.var_t0__blk751).exp();
        (assign24290_e33348, (assign24290_e33348 * locals.var_t0__blk751_dn0), (assign24290_e33348 * locals.var_t0__blk751_dn2), (assign24290_e33348 * locals.var_t0__blk751_dn6), (assign24290_e33348 * locals.var_t0__blk751_dn7), (assign24290_e33348 * locals.var_t0__blk751_dn10), (assign24290_e33348 * locals.var_t0__blk751_dn11), (assign24290_e33348 * locals.var_t0__blk751_dn12), (assign24290_e33348 * locals.var_t0__blk751_dn17),)
    } else {
        (locals.var_t1__blk747, locals.var_t1__blk747_dn0, locals.var_t1__blk747_dn2, locals.var_t1__blk747_dn6, locals.var_t1__blk747_dn7, locals.var_t1__blk747_dn10, locals.var_t1__blk747_dn11, locals.var_t1__blk747_dn12, locals.var_t1__blk747_dn17,)
    }
};
        locals.var_t1__blk747 = assign24290_e33350;
        locals.var_t1__blk747_dn0 = assign24290_e33350_d_n0;
        locals.var_t1__blk747_dn2 = assign24290_e33350_d_n2;
        locals.var_t1__blk747_dn6 = assign24290_e33350_d_n6;
        locals.var_t1__blk747_dn7 = assign24290_e33350_d_n7;
        locals.var_t1__blk747_dn10 = assign24290_e33350_d_n10;
        locals.var_t1__blk747_dn11 = assign24290_e33350_d_n11;
        locals.var_t1__blk747_dn12 = assign24290_e33350_d_n12;
        locals.var_t1__blk747_dn17 = assign24290_e33350_d_n17;

        let (assign24300_e33364, assign24300_e33364_d_n0, assign24300_e33364_d_n2, assign24300_e33364_d_n6, assign24300_e33364_d_n7, assign24300_e33364_d_n10, assign24300_e33364_d_n11, assign24300_e33364_d_n12, assign24300_e33364_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign24300_e33358: f64 = (p.p207 / locals.var_egp12);
        let assign24300_e33360: f64 = (assign24300_e33358 * 1.6021918e-19);
        let assign24300_e33362: f64 = (assign24300_e33360 * locals.var_weff_nf);
        (assign24300_e33362, (((-((p.p207 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk748, locals.var_t2__blk748_dn0, locals.var_t2__blk748_dn2, locals.var_t2__blk748_dn6, locals.var_t2__blk748_dn7, locals.var_t2__blk748_dn10, locals.var_t2__blk748_dn11, locals.var_t2__blk748_dn12, locals.var_t2__blk748_dn17,)
    }
};
        locals.var_t2__blk748 = assign24300_e33364;
        locals.var_t2__blk748_dn0 = assign24300_e33364_d_n0;
        locals.var_t2__blk748_dn2 = assign24300_e33364_d_n2;
        locals.var_t2__blk748_dn6 = assign24300_e33364_d_n6;
        locals.var_t2__blk748_dn7 = assign24300_e33364_d_n7;
        locals.var_t2__blk748_dn10 = assign24300_e33364_d_n10;
        locals.var_t2__blk748_dn11 = assign24300_e33364_d_n11;
        locals.var_t2__blk748_dn12 = assign24300_e33364_d_n12;
        locals.var_t2__blk748_dn17 = assign24300_e33364_d_n17;

        let (assign24310_e33378, assign24310_e33378_d_n0, assign24310_e33378_d_n2, assign24310_e33378_d_n6, assign24310_e33378_d_n7, assign24310_e33378_d_n10, assign24310_e33378_d_n11, assign24310_e33378_d_n12, assign24310_e33378_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign24310_e33372: f64 = (locals.var_t2__blk748 * locals.var_egidl);
        let assign24310_e33374: f64 = (assign24310_e33372 * locals.var_egidl);
        let assign24310_e33376: f64 = (assign24310_e33374 * locals.var_t1__blk747);
        (assign24310_e33376, ((((((locals.var_t2__blk748_dn0 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn0)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn0)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn0)), ((((((locals.var_t2__blk748_dn2 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn2)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn2)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn2)), ((((((locals.var_t2__blk748_dn6 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn6)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn6)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn6)), ((((((locals.var_t2__blk748_dn7 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn7)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn7)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn7)), ((((((locals.var_t2__blk748_dn10 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn10)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn10)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn10)), ((((((locals.var_t2__blk748_dn11 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn11)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn11)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn11)), ((((((locals.var_t2__blk748_dn12 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn12)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn12)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn12)), ((((((locals.var_t2__blk748_dn17 * locals.var_egidl) + (locals.var_t2__blk748 * locals.var_egidl_dn17)) * locals.var_egidl) + (assign24310_e33372 * locals.var_egidl_dn17)) * locals.var_t1__blk747) + (assign24310_e33374 * locals.var_t1__blk747_dn17)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24310_e33378;
        locals.var_igidl_dn0 = assign24310_e33378_d_n0;
        locals.var_igidl_dn2 = assign24310_e33378_d_n2;
        locals.var_igidl_dn6 = assign24310_e33378_d_n6;
        locals.var_igidl_dn7 = assign24310_e33378_d_n7;
        locals.var_igidl_dn10 = assign24310_e33378_d_n10;
        locals.var_igidl_dn11 = assign24310_e33378_d_n11;
        locals.var_igidl_dn12 = assign24310_e33378_d_n12;
        locals.var_igidl_dn17 = assign24310_e33378_d_n17;

        let (assign24320_e33385, assign24320_e33385_d_n0, assign24320_e33385_d_n2, assign24320_e33385_d_n6, assign24320_e33385_d_n7, assign24320_e33385_d_n10, assign24320_e33385_d_n11, assign24320_e33385_d_n12, assign24320_e33385_d_n17,) = {
    if (locals.var_guard754 == 0.0) {
        let assign24320_e33383: f64 = (locals.var_vds - locals.var_vbsp);
        (assign24320_e33383, (locals.var_vds_dn0 - locals.var_vbsp_dn0), (locals.var_vds_dn2 - locals.var_vbsp_dn2), (locals.var_vds_dn6 - locals.var_vbsp_dn6), (locals.var_vds_dn7 - locals.var_vbsp_dn7), (locals.var_vds_dn10 - locals.var_vbsp_dn10), (locals.var_vds_dn11 - locals.var_vbsp_dn11), (locals.var_vds_dn12 - locals.var_vbsp_dn12), (locals.var_vds_dn17 - locals.var_vbsp_dn17),)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn10, locals.var_vdb_dn11, locals.var_vdb_dn12, locals.var_vdb_dn17,)
    }
};
        locals.var_vdb = assign24320_e33385;
        locals.var_vdb_dn0 = assign24320_e33385_d_n0;
        locals.var_vdb_dn2 = assign24320_e33385_d_n2;
        locals.var_vdb_dn6 = assign24320_e33385_d_n6;
        locals.var_vdb_dn7 = assign24320_e33385_d_n7;
        locals.var_vdb_dn10 = assign24320_e33385_d_n10;
        locals.var_vdb_dn11 = assign24320_e33385_d_n11;
        locals.var_vdb_dn12 = assign24320_e33385_d_n12;
        locals.var_vdb_dn17 = assign24320_e33385_d_n17;

        let assign24330_e33388: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign24330_e33388;

        let (assign24340_e33397, assign24340_e33397_d_n0, assign24340_e33397_d_n2, assign24340_e33397_d_n6, assign24340_e33397_d_n7, assign24340_e33397_d_n10, assign24340_e33397_d_n11, assign24340_e33397_d_n12, assign24340_e33397_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 != 0.0)) {
        let assign24340_e33395: f64 = (locals.var_vdb * locals.var_vdb);
        (assign24340_e33395, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn11 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn11)), ((locals.var_vdb_dn12 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn12)), ((locals.var_vdb_dn17 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t2__blk748, locals.var_t2__blk748_dn0, locals.var_t2__blk748_dn2, locals.var_t2__blk748_dn6, locals.var_t2__blk748_dn7, locals.var_t2__blk748_dn10, locals.var_t2__blk748_dn11, locals.var_t2__blk748_dn12, locals.var_t2__blk748_dn17,)
    }
};
        locals.var_t2__blk748 = assign24340_e33397;
        locals.var_t2__blk748_dn0 = assign24340_e33397_d_n0;
        locals.var_t2__blk748_dn2 = assign24340_e33397_d_n2;
        locals.var_t2__blk748_dn6 = assign24340_e33397_d_n6;
        locals.var_t2__blk748_dn7 = assign24340_e33397_d_n7;
        locals.var_t2__blk748_dn10 = assign24340_e33397_d_n10;
        locals.var_t2__blk748_dn11 = assign24340_e33397_d_n11;
        locals.var_t2__blk748_dn12 = assign24340_e33397_d_n12;
        locals.var_t2__blk748_dn17 = assign24340_e33397_d_n17;

        let (assign24350_e33406, assign24350_e33406_d_n0, assign24350_e33406_d_n2, assign24350_e33406_d_n6, assign24350_e33406_d_n7, assign24350_e33406_d_n10, assign24350_e33406_d_n11, assign24350_e33406_d_n12, assign24350_e33406_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 != 0.0)) {
        let assign24350_e33404: f64 = (locals.var_t2__blk748 * locals.var_vdb);
        (assign24350_e33404, ((locals.var_t2__blk748_dn0 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn0)), ((locals.var_t2__blk748_dn2 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn2)), ((locals.var_t2__blk748_dn6 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn6)), ((locals.var_t2__blk748_dn7 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn7)), ((locals.var_t2__blk748_dn10 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn10)), ((locals.var_t2__blk748_dn11 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn11)), ((locals.var_t2__blk748_dn12 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn12)), ((locals.var_t2__blk748_dn17 * locals.var_vdb) + (locals.var_t2__blk748 * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24350_e33406;
        locals.var_t4_dn0 = assign24350_e33406_d_n0;
        locals.var_t4_dn2 = assign24350_e33406_d_n2;
        locals.var_t4_dn6 = assign24350_e33406_d_n6;
        locals.var_t4_dn7 = assign24350_e33406_d_n7;
        locals.var_t4_dn10 = assign24350_e33406_d_n10;
        locals.var_t4_dn11 = assign24350_e33406_d_n11;
        locals.var_t4_dn12 = assign24350_e33406_d_n12;
        locals.var_t4_dn17 = assign24350_e33406_d_n17;

        let (assign24360_e33415, assign24360_e33415_d_n0, assign24360_e33415_d_n2, assign24360_e33415_d_n6, assign24360_e33415_d_n7, assign24360_e33415_d_n10, assign24360_e33415_d_n11, assign24360_e33415_d_n12, assign24360_e33415_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 != 0.0)) {
        let assign24360_e33413: f64 = (locals.var_t4 + p.p212);
        (assign24360_e33413, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk751, locals.var_t0__blk751_dn0, locals.var_t0__blk751_dn2, locals.var_t0__blk751_dn6, locals.var_t0__blk751_dn7, locals.var_t0__blk751_dn10, locals.var_t0__blk751_dn11, locals.var_t0__blk751_dn12, locals.var_t0__blk751_dn17,)
    }
};
        locals.var_t0__blk751 = assign24360_e33415;
        locals.var_t0__blk751_dn0 = assign24360_e33415_d_n0;
        locals.var_t0__blk751_dn2 = assign24360_e33415_d_n2;
        locals.var_t0__blk751_dn6 = assign24360_e33415_d_n6;
        locals.var_t0__blk751_dn7 = assign24360_e33415_d_n7;
        locals.var_t0__blk751_dn10 = assign24360_e33415_d_n10;
        locals.var_t0__blk751_dn11 = assign24360_e33415_d_n11;
        locals.var_t0__blk751_dn12 = assign24360_e33415_d_n12;
        locals.var_t0__blk751_dn17 = assign24360_e33415_d_n17;

        let (assign24370_e33424, assign24370_e33424_d_n0, assign24370_e33424_d_n2, assign24370_e33424_d_n6, assign24370_e33424_d_n7, assign24370_e33424_d_n10, assign24370_e33424_d_n11, assign24370_e33424_d_n12, assign24370_e33424_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 != 0.0)) {
        let assign24370_e33422: f64 = (locals.var_t4 / locals.var_t0__blk751);
        (assign24370_e33422, (((locals.var_t4_dn0 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn0)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn2 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn2)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn6 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn6)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn7 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn7)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn10 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn10)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn11 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn11)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn12 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn12)) / (locals.var_t0__blk751 * locals.var_t0__blk751)), (((locals.var_t4_dn17 * locals.var_t0__blk751) - (locals.var_t4 * locals.var_t0__blk751_dn17)) / (locals.var_t0__blk751 * locals.var_t0__blk751)),)
    } else {
        (locals.var_t5__blk752, locals.var_t5__blk752_dn0, locals.var_t5__blk752_dn2, locals.var_t5__blk752_dn6, locals.var_t5__blk752_dn7, locals.var_t5__blk752_dn10, locals.var_t5__blk752_dn11, locals.var_t5__blk752_dn12, locals.var_t5__blk752_dn17,)
    }
};
        locals.var_t5__blk752 = assign24370_e33424;
        locals.var_t5__blk752_dn0 = assign24370_e33424_d_n0;
        locals.var_t5__blk752_dn2 = assign24370_e33424_d_n2;
        locals.var_t5__blk752_dn6 = assign24370_e33424_d_n6;
        locals.var_t5__blk752_dn7 = assign24370_e33424_d_n7;
        locals.var_t5__blk752_dn10 = assign24370_e33424_d_n10;
        locals.var_t5__blk752_dn11 = assign24370_e33424_d_n11;
        locals.var_t5__blk752_dn12 = assign24370_e33424_d_n12;
        locals.var_t5__blk752_dn17 = assign24370_e33424_d_n17;

        let (assign24380_e33433, assign24380_e33433_d_n0, assign24380_e33433_d_n2, assign24380_e33433_d_n6, assign24380_e33433_d_n7, assign24380_e33433_d_n10, assign24380_e33433_d_n11, assign24380_e33433_d_n12, assign24380_e33433_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 != 0.0)) {
        let assign24380_e33431: f64 = (locals.var_igidl * locals.var_t5__blk752);
        (assign24380_e33431, ((locals.var_igidl_dn0 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn0)), ((locals.var_igidl_dn2 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn2)), ((locals.var_igidl_dn6 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn6)), ((locals.var_igidl_dn7 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn7)), ((locals.var_igidl_dn10 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn10)), ((locals.var_igidl_dn11 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn11)), ((locals.var_igidl_dn12 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn12)), ((locals.var_igidl_dn17 * locals.var_t5__blk752) + (locals.var_igidl * locals.var_t5__blk752_dn17)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24380_e33433;
        locals.var_igidl_dn0 = assign24380_e33433_d_n0;
        locals.var_igidl_dn2 = assign24380_e33433_d_n2;
        locals.var_igidl_dn6 = assign24380_e33433_d_n6;
        locals.var_igidl_dn7 = assign24380_e33433_d_n7;
        locals.var_igidl_dn10 = assign24380_e33433_d_n10;
        locals.var_igidl_dn11 = assign24380_e33433_d_n11;
        locals.var_igidl_dn12 = assign24380_e33433_d_n12;
        locals.var_igidl_dn17 = assign24380_e33433_d_n17;

        let (assign24390_e33441, assign24390_e33441_d_n0, assign24390_e33441_d_n2, assign24390_e33441_d_n6, assign24390_e33441_d_n7, assign24390_e33441_d_n10, assign24390_e33441_d_n11, assign24390_e33441_d_n12, assign24390_e33441_d_n17,) = {
    if ((locals.var_guard754 == 0.0) && (locals.var_guard757 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn12, locals.var_igidl_dn17,)
    }
};
        locals.var_igidl = assign24390_e33441;
        locals.var_igidl_dn0 = assign24390_e33441_d_n0;
        locals.var_igidl_dn2 = assign24390_e33441_d_n2;
        locals.var_igidl_dn6 = assign24390_e33441_d_n6;
        locals.var_igidl_dn7 = assign24390_e33441_d_n7;
        locals.var_igidl_dn10 = assign24390_e33441_d_n10;
        locals.var_igidl_dn11 = assign24390_e33441_d_n11;
        locals.var_igidl_dn12 = assign24390_e33441_d_n12;
        locals.var_igidl_dn17 = assign24390_e33441_d_n17;

        let assign24400_e33444: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign24400_e33444;

        let (assign24410_e33448, assign24410_e33448_d_n0, assign24410_e33448_d_n2, assign24410_e33448_d_n6, assign24410_e33448_d_n7, assign24410_e33448_d_n10, assign24410_e33448_d_n11, assign24410_e33448_d_n12, assign24410_e33448_d_n17,) = {
    if (locals.var_guard765 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24410_e33448;
        locals.var_igisl_dn0 = assign24410_e33448_d_n0;
        locals.var_igisl_dn2 = assign24410_e33448_d_n2;
        locals.var_igisl_dn6 = assign24410_e33448_d_n6;
        locals.var_igisl_dn7 = assign24410_e33448_d_n7;
        locals.var_igisl_dn10 = assign24410_e33448_d_n10;
        locals.var_igisl_dn11 = assign24410_e33448_d_n11;
        locals.var_igisl_dn12 = assign24410_e33448_d_n12;
        locals.var_igisl_dn17 = assign24410_e33448_d_n17;

        let (assign24420_e33468, assign24420_e33468_d_n0, assign24420_e33468_d_n2, assign24420_e33468_d_n6, assign24420_e33468_d_n7, assign24420_e33468_d_n10, assign24420_e33468_d_n11, assign24420_e33468_d_n12, assign24420_e33468_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24420_e33453: f64 = (-locals.var_vds);
        let assign24420_e33455: f64 = (assign24420_e33453 + p.p210);
        let assign24420_e33456: f64 = (p.p209 * assign24420_e33455);
        let assign24420_e33459: f64 = (locals.var_vgs - locals.var_vds);
        let assign24420_e33460: f64 = (assign24420_e33456 - assign24420_e33459);
        let assign24420_e33463: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24420_e33465: f64 = (assign24420_e33463 * p.p211);
        let assign24420_e33466: f64 = (assign24420_e33460 + assign24420_e33465);
        (assign24420_e33466, (((p.p209 * (-locals.var_vds_dn0)) - (-locals.var_vds_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), (((p.p209 * (-locals.var_vds_dn2)) - (-locals.var_vds_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * (-locals.var_vds_dn6)) - (locals.var_vgs_dn6 - locals.var_vds_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * (-locals.var_vds_dn7)) - (locals.var_vgs_dn7 - locals.var_vds_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), (((p.p209 * (-locals.var_vds_dn10)) - (-locals.var_vds_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * (-locals.var_vds_dn11)) - (locals.var_vgs_dn11 - locals.var_vds_dn11)) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), (((p.p209 * (-locals.var_vds_dn12)) - (-locals.var_vds_dn12)) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), (((p.p209 * (-locals.var_vds_dn17)) - (-locals.var_vds_dn17)) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk758, locals.var_t1__blk758_dn0, locals.var_t1__blk758_dn2, locals.var_t1__blk758_dn6, locals.var_t1__blk758_dn7, locals.var_t1__blk758_dn10, locals.var_t1__blk758_dn11, locals.var_t1__blk758_dn12, locals.var_t1__blk758_dn17,)
    }
};
        locals.var_t1__blk758 = assign24420_e33468;
        locals.var_t1__blk758_dn0 = assign24420_e33468_d_n0;
        locals.var_t1__blk758_dn2 = assign24420_e33468_d_n2;
        locals.var_t1__blk758_dn6 = assign24420_e33468_d_n6;
        locals.var_t1__blk758_dn7 = assign24420_e33468_d_n7;
        locals.var_t1__blk758_dn10 = assign24420_e33468_d_n10;
        locals.var_t1__blk758_dn11 = assign24420_e33468_d_n11;
        locals.var_t1__blk758_dn12 = assign24420_e33468_d_n12;
        locals.var_t1__blk758_dn17 = assign24420_e33468_d_n17;

        let (assign24430_e33475, assign24430_e33475_d_n0, assign24430_e33475_d_n2, assign24430_e33475_d_n6, assign24430_e33475_d_n7, assign24430_e33475_d_n10, assign24430_e33475_d_n11, assign24430_e33475_d_n12, assign24430_e33475_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24430_e33473: f64 = (1.0 / locals.var_tfox0);
        (assign24430_e33473, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk759, locals.var_t2__blk759_dn0, locals.var_t2__blk759_dn2, locals.var_t2__blk759_dn6, locals.var_t2__blk759_dn7, locals.var_t2__blk759_dn10, locals.var_t2__blk759_dn11, locals.var_t2__blk759_dn12, locals.var_t2__blk759_dn17,)
    }
};
        locals.var_t2__blk759 = assign24430_e33475;
        locals.var_t2__blk759_dn0 = assign24430_e33475_d_n0;
        locals.var_t2__blk759_dn2 = assign24430_e33475_d_n2;
        locals.var_t2__blk759_dn6 = assign24430_e33475_d_n6;
        locals.var_t2__blk759_dn7 = assign24430_e33475_d_n7;
        locals.var_t2__blk759_dn10 = assign24430_e33475_d_n10;
        locals.var_t2__blk759_dn11 = assign24430_e33475_d_n11;
        locals.var_t2__blk759_dn12 = assign24430_e33475_d_n12;
        locals.var_t2__blk759_dn17 = assign24430_e33475_d_n17;

        let (assign24440_e33482, assign24440_e33482_d_n0, assign24440_e33482_d_n2, assign24440_e33482_d_n6, assign24440_e33482_d_n7, assign24440_e33482_d_n10, assign24440_e33482_d_n11, assign24440_e33482_d_n12, assign24440_e33482_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24440_e33480: f64 = (locals.var_t1__blk758 * locals.var_t2__blk759);
        (assign24440_e33480, ((locals.var_t1__blk758_dn0 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn0)), ((locals.var_t1__blk758_dn2 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn2)), ((locals.var_t1__blk758_dn6 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn6)), ((locals.var_t1__blk758_dn7 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn7)), ((locals.var_t1__blk758_dn10 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn10)), ((locals.var_t1__blk758_dn11 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn11)), ((locals.var_t1__blk758_dn12 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn12)), ((locals.var_t1__blk758_dn17 * locals.var_t2__blk759) + (locals.var_t1__blk758 * locals.var_t2__blk759_dn17)),)
    } else {
        (locals.var_e1__blk760, locals.var_e1__blk760_dn0, locals.var_e1__blk760_dn2, locals.var_e1__blk760_dn6, locals.var_e1__blk760_dn7, locals.var_e1__blk760_dn10, locals.var_e1__blk760_dn11, locals.var_e1__blk760_dn12, locals.var_e1__blk760_dn17,)
    }
};
        locals.var_e1__blk760 = assign24440_e33482;
        locals.var_e1__blk760_dn0 = assign24440_e33482_d_n0;
        locals.var_e1__blk760_dn2 = assign24440_e33482_d_n2;
        locals.var_e1__blk760_dn6 = assign24440_e33482_d_n6;
        locals.var_e1__blk760_dn7 = assign24440_e33482_d_n7;
        locals.var_e1__blk760_dn10 = assign24440_e33482_d_n10;
        locals.var_e1__blk760_dn11 = assign24440_e33482_d_n11;
        locals.var_e1__blk760_dn12 = assign24440_e33482_d_n12;
        locals.var_e1__blk760_dn17 = assign24440_e33482_d_n17;

        let (assign24450_e33496, assign24450_e33496_d_n0, assign24450_e33496_d_n2, assign24450_e33496_d_n6, assign24450_e33496_d_n7, assign24450_e33496_d_n10, assign24450_e33496_d_n11, assign24450_e33496_d_n12, assign24450_e33496_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24450_e33487: f64 = (locals.var_e1__blk760 * locals.var_e1__blk760);
        let assign24450_e33490: f64 = (4.0 * 0.01);
        let assign24450_e33492: f64 = (assign24450_e33490 * 0.01);
        let assign24450_e33493: f64 = (assign24450_e33487 + assign24450_e33492);
        let assign24450_e33494: f64 = (assign24450_e33493).sqrt();
        (assign24450_e33494, (((locals.var_e1__blk760_dn0 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn0)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn2 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn2)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn6 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn6)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn7 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn7)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn10 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn10)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn11 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn11)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn12 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn12)) / (2.0 * assign24450_e33494)), (((locals.var_e1__blk760_dn17 * locals.var_e1__blk760) + (locals.var_e1__blk760 * locals.var_e1__blk760_dn17)) / (2.0 * assign24450_e33494)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24450_e33496;
        locals.var_tmf1_dn0 = assign24450_e33496_d_n0;
        locals.var_tmf1_dn2 = assign24450_e33496_d_n2;
        locals.var_tmf1_dn6 = assign24450_e33496_d_n6;
        locals.var_tmf1_dn7 = assign24450_e33496_d_n7;
        locals.var_tmf1_dn10 = assign24450_e33496_d_n10;
        locals.var_tmf1_dn11 = assign24450_e33496_d_n11;
        locals.var_tmf1_dn12 = assign24450_e33496_d_n12;
        locals.var_tmf1_dn17 = assign24450_e33496_d_n17;

        let (assign24460_e33509, assign24460_e33509_d_n0, assign24460_e33509_d_n2, assign24460_e33509_d_n6, assign24460_e33509_d_n7, assign24460_e33509_d_n10, assign24460_e33509_d_n11, assign24460_e33509_d_n12, assign24460_e33509_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24460_e33502: f64 = (locals.var_e1__blk760 + locals.var_tmf1);
        let assign24460_e33503: f64 = (0.5 * assign24460_e33502);
        let assign24460_e33506: f64 = (1e-10 * 0.01);
        let assign24460_e33507: f64 = (assign24460_e33503 + assign24460_e33506);
        (assign24460_e33507, (0.5 * (locals.var_e1__blk760_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1__blk760_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1__blk760_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1__blk760_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1__blk760_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1__blk760_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1__blk760_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1__blk760_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24460_e33509;
        locals.var_egisl_dn0 = assign24460_e33509_d_n0;
        locals.var_egisl_dn2 = assign24460_e33509_d_n2;
        locals.var_egisl_dn6 = assign24460_e33509_d_n6;
        locals.var_egisl_dn7 = assign24460_e33509_d_n7;
        locals.var_egisl_dn10 = assign24460_e33509_d_n10;
        locals.var_egisl_dn11 = assign24460_e33509_d_n11;
        locals.var_egisl_dn12 = assign24460_e33509_d_n12;
        locals.var_egisl_dn17 = assign24460_e33509_d_n17;

        let assign24470_e33512: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard766 = assign24470_e33512;

        let (assign24480_e33519, assign24480_e33519_d_n0, assign24480_e33519_d_n2, assign24480_e33519_d_n6, assign24480_e33519_d_n7, assign24480_e33519_d_n10, assign24480_e33519_d_n11, assign24480_e33519_d_n12, assign24480_e33519_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard766 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24480_e33519;
        locals.var_egisl_dn0 = assign24480_e33519_d_n0;
        locals.var_egisl_dn2 = assign24480_e33519_d_n2;
        locals.var_egisl_dn6 = assign24480_e33519_d_n6;
        locals.var_egisl_dn7 = assign24480_e33519_d_n7;
        locals.var_egisl_dn10 = assign24480_e33519_d_n10;
        locals.var_egisl_dn11 = assign24480_e33519_d_n11;
        locals.var_egisl_dn12 = assign24480_e33519_d_n12;
        locals.var_egisl_dn17 = assign24480_e33519_d_n17;

    }

    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24490_e33528, assign24490_e33528_d_n0, assign24490_e33528_d_n2, assign24490_e33528_d_n6, assign24490_e33528_d_n7, assign24490_e33528_d_n10, assign24490_e33528_d_n11, assign24490_e33528_d_n12, assign24490_e33528_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24490_e33525: f64 = (locals.var_egisl + 1e-50);
        let assign24490_e33526: f64 = (1.0 / assign24490_e33525);
        (assign24490_e33526, (-(locals.var_egisl_dn0 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn2 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn6 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn7 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn10 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn11 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn12 / (assign24490_e33525 * assign24490_e33525))), (-(locals.var_egisl_dn17 / (assign24490_e33525 * assign24490_e33525))),)
    } else {
        (locals.var_t3__blk761, locals.var_t3__blk761_dn0, locals.var_t3__blk761_dn2, locals.var_t3__blk761_dn6, locals.var_t3__blk761_dn7, locals.var_t3__blk761_dn10, locals.var_t3__blk761_dn11, locals.var_t3__blk761_dn12, locals.var_t3__blk761_dn17,)
    }
};
        locals.var_t3__blk761 = assign24490_e33528;
        locals.var_t3__blk761_dn0 = assign24490_e33528_d_n0;
        locals.var_t3__blk761_dn2 = assign24490_e33528_d_n2;
        locals.var_t3__blk761_dn6 = assign24490_e33528_d_n6;
        locals.var_t3__blk761_dn7 = assign24490_e33528_d_n7;
        locals.var_t3__blk761_dn10 = assign24490_e33528_d_n10;
        locals.var_t3__blk761_dn11 = assign24490_e33528_d_n11;
        locals.var_t3__blk761_dn12 = assign24490_e33528_d_n12;
        locals.var_t3__blk761_dn17 = assign24490_e33528_d_n17;

        let (assign24500_e33538, assign24500_e33538_d_n0, assign24500_e33538_d_n2, assign24500_e33538_d_n6, assign24500_e33538_d_n7, assign24500_e33538_d_n10, assign24500_e33538_d_n11, assign24500_e33538_d_n12, assign24500_e33538_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24500_e33532: f64 = (-p.p208);
        let assign24500_e33534: f64 = (assign24500_e33532 * locals.var_egp32);
        let assign24500_e33536: f64 = (assign24500_e33534 * locals.var_t3__blk761);
        (assign24500_e33536, (((assign24500_e33532 * locals.var_egp32_dn0) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn0)), (((assign24500_e33532 * locals.var_egp32_dn2) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn2)), (((assign24500_e33532 * locals.var_egp32_dn6) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn6)), (((assign24500_e33532 * locals.var_egp32_dn7) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn7)), (((assign24500_e33532 * locals.var_egp32_dn10) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn10)), (((assign24500_e33532 * locals.var_egp32_dn11) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn11)), (((assign24500_e33532 * locals.var_egp32_dn12) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn12)), (((assign24500_e33532 * locals.var_egp32_dn17) * locals.var_t3__blk761) + (assign24500_e33534 * locals.var_t3__blk761_dn17)),)
    } else {
        (locals.var_t0__blk762, locals.var_t0__blk762_dn0, locals.var_t0__blk762_dn2, locals.var_t0__blk762_dn6, locals.var_t0__blk762_dn7, locals.var_t0__blk762_dn10, locals.var_t0__blk762_dn11, locals.var_t0__blk762_dn12, locals.var_t0__blk762_dn17,)
    }
};
        locals.var_t0__blk762 = assign24500_e33538;
        locals.var_t0__blk762_dn0 = assign24500_e33538_d_n0;
        locals.var_t0__blk762_dn2 = assign24500_e33538_d_n2;
        locals.var_t0__blk762_dn6 = assign24500_e33538_d_n6;
        locals.var_t0__blk762_dn7 = assign24500_e33538_d_n7;
        locals.var_t0__blk762_dn10 = assign24500_e33538_d_n10;
        locals.var_t0__blk762_dn11 = assign24500_e33538_d_n11;
        locals.var_t0__blk762_dn12 = assign24500_e33538_d_n12;
        locals.var_t0__blk762_dn17 = assign24500_e33538_d_n17;

        let assign24510_e33541: f64 = (-34.0);
        let assign24510_e33542: f64 = if locals.var_t0__blk762 < assign24510_e33541 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign24510_e33542;

        let (assign24520_e33549, assign24520_e33549_d_n0, assign24520_e33549_d_n2, assign24520_e33549_d_n6, assign24520_e33549_d_n7, assign24520_e33549_d_n10, assign24520_e33549_d_n11, assign24520_e33549_d_n12, assign24520_e33549_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard767 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24520_e33549;
        locals.var_igisl_dn0 = assign24520_e33549_d_n0;
        locals.var_igisl_dn2 = assign24520_e33549_d_n2;
        locals.var_igisl_dn6 = assign24520_e33549_d_n6;
        locals.var_igisl_dn7 = assign24520_e33549_d_n7;
        locals.var_igisl_dn10 = assign24520_e33549_d_n10;
        locals.var_igisl_dn11 = assign24520_e33549_d_n11;
        locals.var_igisl_dn12 = assign24520_e33549_d_n12;
        locals.var_igisl_dn17 = assign24520_e33549_d_n17;

        let (assign24530_e33558, assign24530_e33558_d_n0, assign24530_e33558_d_n2, assign24530_e33558_d_n6, assign24530_e33558_d_n7, assign24530_e33558_d_n10, assign24530_e33558_d_n11, assign24530_e33558_d_n12, assign24530_e33558_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard767 == 0.0)) {
        let assign24530_e33556: f64 = (locals.var_t0__blk762).exp();
        (assign24530_e33556, (assign24530_e33556 * locals.var_t0__blk762_dn0), (assign24530_e33556 * locals.var_t0__blk762_dn2), (assign24530_e33556 * locals.var_t0__blk762_dn6), (assign24530_e33556 * locals.var_t0__blk762_dn7), (assign24530_e33556 * locals.var_t0__blk762_dn10), (assign24530_e33556 * locals.var_t0__blk762_dn11), (assign24530_e33556 * locals.var_t0__blk762_dn12), (assign24530_e33556 * locals.var_t0__blk762_dn17),)
    } else {
        (locals.var_t1__blk758, locals.var_t1__blk758_dn0, locals.var_t1__blk758_dn2, locals.var_t1__blk758_dn6, locals.var_t1__blk758_dn7, locals.var_t1__blk758_dn10, locals.var_t1__blk758_dn11, locals.var_t1__blk758_dn12, locals.var_t1__blk758_dn17,)
    }
};
        locals.var_t1__blk758 = assign24530_e33558;
        locals.var_t1__blk758_dn0 = assign24530_e33558_d_n0;
        locals.var_t1__blk758_dn2 = assign24530_e33558_d_n2;
        locals.var_t1__blk758_dn6 = assign24530_e33558_d_n6;
        locals.var_t1__blk758_dn7 = assign24530_e33558_d_n7;
        locals.var_t1__blk758_dn10 = assign24530_e33558_d_n10;
        locals.var_t1__blk758_dn11 = assign24530_e33558_d_n11;
        locals.var_t1__blk758_dn12 = assign24530_e33558_d_n12;
        locals.var_t1__blk758_dn17 = assign24530_e33558_d_n17;

        let (assign24540_e33568, assign24540_e33568_d_n0, assign24540_e33568_d_n2, assign24540_e33568_d_n6, assign24540_e33568_d_n7, assign24540_e33568_d_n10, assign24540_e33568_d_n11, assign24540_e33568_d_n12, assign24540_e33568_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard767 == 0.0)) {
        let assign24540_e33566: f64 = (1.0 / locals.var_egp12);
        (assign24540_e33566, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn11 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn12 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn17 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3__blk761, locals.var_t3__blk761_dn0, locals.var_t3__blk761_dn2, locals.var_t3__blk761_dn6, locals.var_t3__blk761_dn7, locals.var_t3__blk761_dn10, locals.var_t3__blk761_dn11, locals.var_t3__blk761_dn12, locals.var_t3__blk761_dn17,)
    }
};
        locals.var_t3__blk761 = assign24540_e33568;
        locals.var_t3__blk761_dn0 = assign24540_e33568_d_n0;
        locals.var_t3__blk761_dn2 = assign24540_e33568_d_n2;
        locals.var_t3__blk761_dn6 = assign24540_e33568_d_n6;
        locals.var_t3__blk761_dn7 = assign24540_e33568_d_n7;
        locals.var_t3__blk761_dn10 = assign24540_e33568_d_n10;
        locals.var_t3__blk761_dn11 = assign24540_e33568_d_n11;
        locals.var_t3__blk761_dn12 = assign24540_e33568_d_n12;
        locals.var_t3__blk761_dn17 = assign24540_e33568_d_n17;

        let (assign24550_e33582, assign24550_e33582_d_n0, assign24550_e33582_d_n2, assign24550_e33582_d_n6, assign24550_e33582_d_n7, assign24550_e33582_d_n10, assign24550_e33582_d_n11, assign24550_e33582_d_n12, assign24550_e33582_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard767 == 0.0)) {
        let assign24550_e33576: f64 = (p.p207 * locals.var_t3__blk761);
        let assign24550_e33578: f64 = (assign24550_e33576 * 1.6021918e-19);
        let assign24550_e33580: f64 = (assign24550_e33578 * locals.var_weff_nf);
        (assign24550_e33580, (((p.p207 * locals.var_t3__blk761_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn11) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn12) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk761_dn17) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk759, locals.var_t2__blk759_dn0, locals.var_t2__blk759_dn2, locals.var_t2__blk759_dn6, locals.var_t2__blk759_dn7, locals.var_t2__blk759_dn10, locals.var_t2__blk759_dn11, locals.var_t2__blk759_dn12, locals.var_t2__blk759_dn17,)
    }
};
        locals.var_t2__blk759 = assign24550_e33582;
        locals.var_t2__blk759_dn0 = assign24550_e33582_d_n0;
        locals.var_t2__blk759_dn2 = assign24550_e33582_d_n2;
        locals.var_t2__blk759_dn6 = assign24550_e33582_d_n6;
        locals.var_t2__blk759_dn7 = assign24550_e33582_d_n7;
        locals.var_t2__blk759_dn10 = assign24550_e33582_d_n10;
        locals.var_t2__blk759_dn11 = assign24550_e33582_d_n11;
        locals.var_t2__blk759_dn12 = assign24550_e33582_d_n12;
        locals.var_t2__blk759_dn17 = assign24550_e33582_d_n17;

        let (assign24560_e33596, assign24560_e33596_d_n0, assign24560_e33596_d_n2, assign24560_e33596_d_n6, assign24560_e33596_d_n7, assign24560_e33596_d_n10, assign24560_e33596_d_n11, assign24560_e33596_d_n12, assign24560_e33596_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard767 == 0.0)) {
        let assign24560_e33590: f64 = (locals.var_t2__blk759 * locals.var_egisl);
        let assign24560_e33592: f64 = (assign24560_e33590 * locals.var_egisl);
        let assign24560_e33594: f64 = (assign24560_e33592 * locals.var_t1__blk758);
        (assign24560_e33594, ((((((locals.var_t2__blk759_dn0 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn0)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn0)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn0)), ((((((locals.var_t2__blk759_dn2 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn2)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn2)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn2)), ((((((locals.var_t2__blk759_dn6 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn6)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn6)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn6)), ((((((locals.var_t2__blk759_dn7 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn7)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn7)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn7)), ((((((locals.var_t2__blk759_dn10 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn10)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn10)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn10)), ((((((locals.var_t2__blk759_dn11 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn11)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn11)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn11)), ((((((locals.var_t2__blk759_dn12 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn12)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn12)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn12)), ((((((locals.var_t2__blk759_dn17 * locals.var_egisl) + (locals.var_t2__blk759 * locals.var_egisl_dn17)) * locals.var_egisl) + (assign24560_e33590 * locals.var_egisl_dn17)) * locals.var_t1__blk758) + (assign24560_e33592 * locals.var_t1__blk758_dn17)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24560_e33596;
        locals.var_igisl_dn0 = assign24560_e33596_d_n0;
        locals.var_igisl_dn2 = assign24560_e33596_d_n2;
        locals.var_igisl_dn6 = assign24560_e33596_d_n6;
        locals.var_igisl_dn7 = assign24560_e33596_d_n7;
        locals.var_igisl_dn10 = assign24560_e33596_d_n10;
        locals.var_igisl_dn11 = assign24560_e33596_d_n11;
        locals.var_igisl_dn12 = assign24560_e33596_d_n12;
        locals.var_igisl_dn17 = assign24560_e33596_d_n17;

        let (assign24570_e33602, assign24570_e33602_d_n0, assign24570_e33602_d_n2, assign24570_e33602_d_n6, assign24570_e33602_d_n7, assign24570_e33602_d_n10, assign24570_e33602_d_n11, assign24570_e33602_d_n12, assign24570_e33602_d_n17,) = {
    if (locals.var_guard765 == 0.0) {
        let assign24570_e33600: f64 = (-locals.var_vbsp);
        (assign24570_e33600, (-locals.var_vbsp_dn0), (-locals.var_vbsp_dn2), (-locals.var_vbsp_dn6), (-locals.var_vbsp_dn7), (-locals.var_vbsp_dn10), (-locals.var_vbsp_dn11), (-locals.var_vbsp_dn12), (-locals.var_vbsp_dn17),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn0, locals.var_vsb_dn2, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn10, locals.var_vsb_dn11, locals.var_vsb_dn12, locals.var_vsb_dn17,)
    }
};
        locals.var_vsb = assign24570_e33602;
        locals.var_vsb_dn0 = assign24570_e33602_d_n0;
        locals.var_vsb_dn2 = assign24570_e33602_d_n2;
        locals.var_vsb_dn6 = assign24570_e33602_d_n6;
        locals.var_vsb_dn7 = assign24570_e33602_d_n7;
        locals.var_vsb_dn10 = assign24570_e33602_d_n10;
        locals.var_vsb_dn11 = assign24570_e33602_d_n11;
        locals.var_vsb_dn12 = assign24570_e33602_d_n12;
        locals.var_vsb_dn17 = assign24570_e33602_d_n17;

        let assign24580_e33605: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign24580_e33605;

        let (assign24590_e33614, assign24590_e33614_d_n0, assign24590_e33614_d_n2, assign24590_e33614_d_n6, assign24590_e33614_d_n7, assign24590_e33614_d_n10, assign24590_e33614_d_n11, assign24590_e33614_d_n12, assign24590_e33614_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 != 0.0)) {
        let assign24590_e33612: f64 = (locals.var_vsb * locals.var_vsb);
        (assign24590_e33612, ((locals.var_vsb_dn0 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn0)), ((locals.var_vsb_dn2 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn2)), ((locals.var_vsb_dn6 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn6)), ((locals.var_vsb_dn7 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn7)), ((locals.var_vsb_dn10 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn10)), ((locals.var_vsb_dn11 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn11)), ((locals.var_vsb_dn12 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn12)), ((locals.var_vsb_dn17 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t2__blk759, locals.var_t2__blk759_dn0, locals.var_t2__blk759_dn2, locals.var_t2__blk759_dn6, locals.var_t2__blk759_dn7, locals.var_t2__blk759_dn10, locals.var_t2__blk759_dn11, locals.var_t2__blk759_dn12, locals.var_t2__blk759_dn17,)
    }
};
        locals.var_t2__blk759 = assign24590_e33614;
        locals.var_t2__blk759_dn0 = assign24590_e33614_d_n0;
        locals.var_t2__blk759_dn2 = assign24590_e33614_d_n2;
        locals.var_t2__blk759_dn6 = assign24590_e33614_d_n6;
        locals.var_t2__blk759_dn7 = assign24590_e33614_d_n7;
        locals.var_t2__blk759_dn10 = assign24590_e33614_d_n10;
        locals.var_t2__blk759_dn11 = assign24590_e33614_d_n11;
        locals.var_t2__blk759_dn12 = assign24590_e33614_d_n12;
        locals.var_t2__blk759_dn17 = assign24590_e33614_d_n17;

        let (assign24600_e33623, assign24600_e33623_d_n0, assign24600_e33623_d_n2, assign24600_e33623_d_n6, assign24600_e33623_d_n7, assign24600_e33623_d_n10, assign24600_e33623_d_n11, assign24600_e33623_d_n12, assign24600_e33623_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 != 0.0)) {
        let assign24600_e33621: f64 = (locals.var_t2__blk759 * locals.var_vsb);
        (assign24600_e33621, ((locals.var_t2__blk759_dn0 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn0)), ((locals.var_t2__blk759_dn2 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn2)), ((locals.var_t2__blk759_dn6 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn6)), ((locals.var_t2__blk759_dn7 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn7)), ((locals.var_t2__blk759_dn10 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn10)), ((locals.var_t2__blk759_dn11 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn11)), ((locals.var_t2__blk759_dn12 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn12)), ((locals.var_t2__blk759_dn17 * locals.var_vsb) + (locals.var_t2__blk759 * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24600_e33623;
        locals.var_t4_dn0 = assign24600_e33623_d_n0;
        locals.var_t4_dn2 = assign24600_e33623_d_n2;
        locals.var_t4_dn6 = assign24600_e33623_d_n6;
        locals.var_t4_dn7 = assign24600_e33623_d_n7;
        locals.var_t4_dn10 = assign24600_e33623_d_n10;
        locals.var_t4_dn11 = assign24600_e33623_d_n11;
        locals.var_t4_dn12 = assign24600_e33623_d_n12;
        locals.var_t4_dn17 = assign24600_e33623_d_n17;

        let (assign24610_e33632, assign24610_e33632_d_n0, assign24610_e33632_d_n2, assign24610_e33632_d_n6, assign24610_e33632_d_n7, assign24610_e33632_d_n10, assign24610_e33632_d_n11, assign24610_e33632_d_n12, assign24610_e33632_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 != 0.0)) {
        let assign24610_e33630: f64 = (locals.var_t4 + p.p212);
        (assign24610_e33630, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk762, locals.var_t0__blk762_dn0, locals.var_t0__blk762_dn2, locals.var_t0__blk762_dn6, locals.var_t0__blk762_dn7, locals.var_t0__blk762_dn10, locals.var_t0__blk762_dn11, locals.var_t0__blk762_dn12, locals.var_t0__blk762_dn17,)
    }
};
        locals.var_t0__blk762 = assign24610_e33632;
        locals.var_t0__blk762_dn0 = assign24610_e33632_d_n0;
        locals.var_t0__blk762_dn2 = assign24610_e33632_d_n2;
        locals.var_t0__blk762_dn6 = assign24610_e33632_d_n6;
        locals.var_t0__blk762_dn7 = assign24610_e33632_d_n7;
        locals.var_t0__blk762_dn10 = assign24610_e33632_d_n10;
        locals.var_t0__blk762_dn11 = assign24610_e33632_d_n11;
        locals.var_t0__blk762_dn12 = assign24610_e33632_d_n12;
        locals.var_t0__blk762_dn17 = assign24610_e33632_d_n17;

        let (assign24620_e33641, assign24620_e33641_d_n0, assign24620_e33641_d_n2, assign24620_e33641_d_n6, assign24620_e33641_d_n7, assign24620_e33641_d_n10, assign24620_e33641_d_n11, assign24620_e33641_d_n12, assign24620_e33641_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 != 0.0)) {
        let assign24620_e33639: f64 = (locals.var_t4 / locals.var_t0__blk762);
        (assign24620_e33639, (((locals.var_t4_dn0 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn0)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn2 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn2)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn6 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn6)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn7 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn7)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn10 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn10)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn11 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn11)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn12 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn12)) / (locals.var_t0__blk762 * locals.var_t0__blk762)), (((locals.var_t4_dn17 * locals.var_t0__blk762) - (locals.var_t4 * locals.var_t0__blk762_dn17)) / (locals.var_t0__blk762 * locals.var_t0__blk762)),)
    } else {
        (locals.var_t5__blk763, locals.var_t5__blk763_dn0, locals.var_t5__blk763_dn2, locals.var_t5__blk763_dn6, locals.var_t5__blk763_dn7, locals.var_t5__blk763_dn10, locals.var_t5__blk763_dn11, locals.var_t5__blk763_dn12, locals.var_t5__blk763_dn17,)
    }
};
        locals.var_t5__blk763 = assign24620_e33641;
        locals.var_t5__blk763_dn0 = assign24620_e33641_d_n0;
        locals.var_t5__blk763_dn2 = assign24620_e33641_d_n2;
        locals.var_t5__blk763_dn6 = assign24620_e33641_d_n6;
        locals.var_t5__blk763_dn7 = assign24620_e33641_d_n7;
        locals.var_t5__blk763_dn10 = assign24620_e33641_d_n10;
        locals.var_t5__blk763_dn11 = assign24620_e33641_d_n11;
        locals.var_t5__blk763_dn12 = assign24620_e33641_d_n12;
        locals.var_t5__blk763_dn17 = assign24620_e33641_d_n17;

        let (assign24630_e33650, assign24630_e33650_d_n0, assign24630_e33650_d_n2, assign24630_e33650_d_n6, assign24630_e33650_d_n7, assign24630_e33650_d_n10, assign24630_e33650_d_n11, assign24630_e33650_d_n12, assign24630_e33650_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 != 0.0)) {
        let assign24630_e33648: f64 = (locals.var_igisl * locals.var_t5__blk763);
        (assign24630_e33648, ((locals.var_igisl_dn0 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn0)), ((locals.var_igisl_dn2 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn2)), ((locals.var_igisl_dn6 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn6)), ((locals.var_igisl_dn7 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn7)), ((locals.var_igisl_dn10 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn10)), ((locals.var_igisl_dn11 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn11)), ((locals.var_igisl_dn12 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn12)), ((locals.var_igisl_dn17 * locals.var_t5__blk763) + (locals.var_igisl * locals.var_t5__blk763_dn17)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24630_e33650;
        locals.var_igisl_dn0 = assign24630_e33650_d_n0;
        locals.var_igisl_dn2 = assign24630_e33650_d_n2;
        locals.var_igisl_dn6 = assign24630_e33650_d_n6;
        locals.var_igisl_dn7 = assign24630_e33650_d_n7;
        locals.var_igisl_dn10 = assign24630_e33650_d_n10;
        locals.var_igisl_dn11 = assign24630_e33650_d_n11;
        locals.var_igisl_dn12 = assign24630_e33650_d_n12;
        locals.var_igisl_dn17 = assign24630_e33650_d_n17;

        let (assign24640_e33658, assign24640_e33658_d_n0, assign24640_e33658_d_n2, assign24640_e33658_d_n6, assign24640_e33658_d_n7, assign24640_e33658_d_n10, assign24640_e33658_d_n11, assign24640_e33658_d_n12, assign24640_e33658_d_n17,) = {
    if ((locals.var_guard765 == 0.0) && (locals.var_guard768 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn12, locals.var_igisl_dn17,)
    }
};
        locals.var_igisl = assign24640_e33658;
        locals.var_igisl_dn0 = assign24640_e33658_d_n0;
        locals.var_igisl_dn2 = assign24640_e33658_d_n2;
        locals.var_igisl_dn6 = assign24640_e33658_d_n6;
        locals.var_igisl_dn7 = assign24640_e33658_d_n7;
        locals.var_igisl_dn10 = assign24640_e33658_d_n10;
        locals.var_igisl_dn11 = assign24640_e33658_d_n11;
        locals.var_igisl_dn12 = assign24640_e33658_d_n12;
        locals.var_igisl_dn17 = assign24640_e33658_d_n17;

        let assign24650_e33661: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign24650_e33661;

        let (assign24660_e33665,) = {
    if (locals.var_guard769 != 0.0) {
        (locals.var_c_fox0,)
    } else {
        (locals.var_cox0,)
    }
};
        locals.var_cox0 = assign24660_e33665;

        let (assign24670_e33671,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24670_e33669: f64 = (1.0 / locals.var_cox0);
        (assign24670_e33669,)
    } else {
        (locals.var_cox0_inv,)
    }
};
        locals.var_cox0_inv = assign24670_e33671;

        let (assign24680_e33675, assign24680_e33675_d_n0, assign24680_e33675_d_n2, assign24680_e33675_d_n6, assign24680_e33675_d_n7, assign24680_e33675_d_n10, assign24680_e33675_d_n11, assign24680_e33675_d_n12, assign24680_e33675_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
        locals.var_fs01__blk836 = assign24680_e33675;
        locals.var_fs01__blk836_dn0 = assign24680_e33675_d_n0;
        locals.var_fs01__blk836_dn2 = assign24680_e33675_d_n2;
        locals.var_fs01__blk836_dn6 = assign24680_e33675_d_n6;
        locals.var_fs01__blk836_dn7 = assign24680_e33675_d_n7;
        locals.var_fs01__blk836_dn10 = assign24680_e33675_d_n10;
        locals.var_fs01__blk836_dn11 = assign24680_e33675_d_n11;
        locals.var_fs01__blk836_dn12 = assign24680_e33675_d_n12;
        locals.var_fs01__blk836_dn17 = assign24680_e33675_d_n17;

        let (assign24690_e33679, assign24690_e33679_d_n0, assign24690_e33679_d_n2, assign24690_e33679_d_n6, assign24690_e33679_d_n7, assign24690_e33679_d_n10, assign24690_e33679_d_n11, assign24690_e33679_d_n12, assign24690_e33679_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk838, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    }
};
        locals.var_fb__blk838 = assign24690_e33679;
        locals.var_fb__blk838_dn0 = assign24690_e33679_d_n0;
        locals.var_fb__blk838_dn2 = assign24690_e33679_d_n2;
        locals.var_fb__blk838_dn6 = assign24690_e33679_d_n6;
        locals.var_fb__blk838_dn7 = assign24690_e33679_d_n7;
        locals.var_fb__blk838_dn10 = assign24690_e33679_d_n10;
        locals.var_fb__blk838_dn11 = assign24690_e33679_d_n11;
        locals.var_fb__blk838_dn12 = assign24690_e33679_d_n12;
        locals.var_fb__blk838_dn17 = assign24690_e33679_d_n17;

        let (assign24700_e33683, assign24700_e33683_d_n0, assign24700_e33683_d_n2, assign24700_e33683_d_n6, assign24700_e33683_d_n7, assign24700_e33683_d_n10, assign24700_e33683_d_n11, assign24700_e33683_d_n12, assign24700_e33683_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk840, locals.var_fs02__blk840_dn0, locals.var_fs02__blk840_dn2, locals.var_fs02__blk840_dn6, locals.var_fs02__blk840_dn7, locals.var_fs02__blk840_dn10, locals.var_fs02__blk840_dn11, locals.var_fs02__blk840_dn12, locals.var_fs02__blk840_dn17,)
    }
};
        locals.var_fs02__blk840 = assign24700_e33683;
        locals.var_fs02__blk840_dn0 = assign24700_e33683_d_n0;
        locals.var_fs02__blk840_dn2 = assign24700_e33683_d_n2;
        locals.var_fs02__blk840_dn6 = assign24700_e33683_d_n6;
        locals.var_fs02__blk840_dn7 = assign24700_e33683_d_n7;
        locals.var_fs02__blk840_dn10 = assign24700_e33683_d_n10;
        locals.var_fs02__blk840_dn11 = assign24700_e33683_d_n11;
        locals.var_fs02__blk840_dn12 = assign24700_e33683_d_n12;
        locals.var_fs02__blk840_dn17 = assign24700_e33683_d_n17;

        let (assign24710_e33688, assign24710_e33688_d_n0, assign24710_e33688_d_n2, assign24710_e33688_d_n6, assign24710_e33688_d_n7, assign24710_e33688_d_n10, assign24710_e33688_d_n11, assign24710_e33688_d_n12, assign24710_e33688_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24710_e33686: f64 = (-locals.var_area_bt_n);
        (assign24710_e33686, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign24710_e33688;
        locals.var_t2__blk772_dn0 = assign24710_e33688_d_n0;
        locals.var_t2__blk772_dn2 = assign24710_e33688_d_n2;
        locals.var_t2__blk772_dn6 = assign24710_e33688_d_n6;
        locals.var_t2__blk772_dn7 = assign24710_e33688_d_n7;
        locals.var_t2__blk772_dn10 = assign24710_e33688_d_n10;
        locals.var_t2__blk772_dn11 = assign24710_e33688_d_n11;
        locals.var_t2__blk772_dn12 = assign24710_e33688_d_n12;
        locals.var_t2__blk772_dn17 = assign24710_e33688_d_n17;

        let (assign24720_e33694, assign24720_e33694_d_n0, assign24720_e33694_d_n2, assign24720_e33694_d_n6, assign24720_e33694_d_n7, assign24720_e33694_d_n10, assign24720_e33694_d_n11, assign24720_e33694_d_n12, assign24720_e33694_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24720_e33692: f64 = (locals.var_t2__blk772 * locals.var_qiu);
        (assign24720_e33692, ((locals.var_t2__blk772_dn0 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn0)), ((locals.var_t2__blk772_dn2 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn2)), ((locals.var_t2__blk772_dn6 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn6)), ((locals.var_t2__blk772_dn7 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn7)), ((locals.var_t2__blk772_dn10 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn10)), ((locals.var_t2__blk772_dn11 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn11)), ((locals.var_t2__blk772_dn12 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn12)), ((locals.var_t2__blk772_dn17 * locals.var_qiu) + (locals.var_t2__blk772 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_t3__blk773, locals.var_t3__blk773_dn0, locals.var_t3__blk773_dn2, locals.var_t3__blk773_dn6, locals.var_t3__blk773_dn7, locals.var_t3__blk773_dn10, locals.var_t3__blk773_dn11, locals.var_t3__blk773_dn12, locals.var_t3__blk773_dn17,)
    }
};
        locals.var_t3__blk773 = assign24720_e33694;
        locals.var_t3__blk773_dn0 = assign24720_e33694_d_n0;
        locals.var_t3__blk773_dn2 = assign24720_e33694_d_n2;
        locals.var_t3__blk773_dn6 = assign24720_e33694_d_n6;
        locals.var_t3__blk773_dn7 = assign24720_e33694_d_n7;
        locals.var_t3__blk773_dn10 = assign24720_e33694_d_n10;
        locals.var_t3__blk773_dn11 = assign24720_e33694_d_n11;
        locals.var_t3__blk773_dn12 = assign24720_e33694_d_n12;
        locals.var_t3__blk773_dn17 = assign24720_e33694_d_n17;

        let (assign24730_e33702, assign24730_e33702_d_n0, assign24730_e33702_d_n2, assign24730_e33702_d_n6, assign24730_e33702_d_n7, assign24730_e33702_d_n10, assign24730_e33702_d_n11, assign24730_e33702_d_n12, assign24730_e33702_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24730_e33699: f64 = (locals.var_t2__blk772 * locals.var_qbu);
        let assign24730_e33700: f64 = (locals.var_t3__blk773 + assign24730_e33699);
        (assign24730_e33700, (locals.var_t3__blk773_dn0 + ((locals.var_t2__blk772_dn0 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn0))), (locals.var_t3__blk773_dn2 + ((locals.var_t2__blk772_dn2 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn2))), (locals.var_t3__blk773_dn6 + ((locals.var_t2__blk772_dn6 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn6))), (locals.var_t3__blk773_dn7 + ((locals.var_t2__blk772_dn7 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn7))), (locals.var_t3__blk773_dn10 + ((locals.var_t2__blk772_dn10 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn10))), (locals.var_t3__blk773_dn11 + ((locals.var_t2__blk772_dn11 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn11))), (locals.var_t3__blk773_dn12 + ((locals.var_t2__blk772_dn12 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn12))), (locals.var_t3__blk773_dn17 + ((locals.var_t2__blk772_dn17 * locals.var_qbu) + (locals.var_t2__blk772 * locals.var_qbu_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24730_e33702;
        locals.var_t4_dn0 = assign24730_e33702_d_n0;
        locals.var_t4_dn2 = assign24730_e33702_d_n2;
        locals.var_t4_dn6 = assign24730_e33702_d_n6;
        locals.var_t4_dn7 = assign24730_e33702_d_n7;
        locals.var_t4_dn10 = assign24730_e33702_d_n10;
        locals.var_t4_dn11 = assign24730_e33702_d_n11;
        locals.var_t4_dn12 = assign24730_e33702_d_n12;
        locals.var_t4_dn17 = assign24730_e33702_d_n17;

        let (assign24740_e33708, assign24740_e33708_d_n0, assign24740_e33708_d_n2, assign24740_e33708_d_n6, assign24740_e33708_d_n7, assign24740_e33708_d_n10, assign24740_e33708_d_n11, assign24740_e33708_d_n12, assign24740_e33708_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24740_e33706: f64 = (locals.var_t3__blk773 * locals.var_qdrat);
        (assign24740_e33706, ((locals.var_t3__blk773_dn0 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn0)), ((locals.var_t3__blk773_dn2 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn2)), ((locals.var_t3__blk773_dn6 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn6)), ((locals.var_t3__blk773_dn7 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn7)), ((locals.var_t3__blk773_dn10 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn10)), ((locals.var_t3__blk773_dn11 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn11)), ((locals.var_t3__blk773_dn12 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn12)), ((locals.var_t3__blk773_dn17 * locals.var_qdrat) + (locals.var_t3__blk773 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign24740_e33708;
        locals.var_qbody_bt_n_iud_dn0 = assign24740_e33708_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign24740_e33708_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign24740_e33708_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign24740_e33708_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign24740_e33708_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign24740_e33708_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign24740_e33708_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign24740_e33708_d_n17;

        let (assign24750_e33714, assign24750_e33714_d_n0, assign24750_e33714_d_n2, assign24750_e33714_d_n6, assign24750_e33714_d_n7, assign24750_e33714_d_n10, assign24750_e33714_d_n11, assign24750_e33714_d_n12, assign24750_e33714_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24750_e33712: f64 = (locals.var_t3__blk773 - locals.var_qbody_bt_n_iud);
        (assign24750_e33712, (locals.var_t3__blk773_dn0 - locals.var_qbody_bt_n_iud_dn0), (locals.var_t3__blk773_dn2 - locals.var_qbody_bt_n_iud_dn2), (locals.var_t3__blk773_dn6 - locals.var_qbody_bt_n_iud_dn6), (locals.var_t3__blk773_dn7 - locals.var_qbody_bt_n_iud_dn7), (locals.var_t3__blk773_dn10 - locals.var_qbody_bt_n_iud_dn10), (locals.var_t3__blk773_dn11 - locals.var_qbody_bt_n_iud_dn11), (locals.var_t3__blk773_dn12 - locals.var_qbody_bt_n_iud_dn12), (locals.var_t3__blk773_dn17 - locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign24750_e33714;
        locals.var_qbody_bt_n_ius_dn0 = assign24750_e33714_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign24750_e33714_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign24750_e33714_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign24750_e33714_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign24750_e33714_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign24750_e33714_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign24750_e33714_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign24750_e33714_d_n17;

        let (assign24760_e33720, assign24760_e33720_d_n0, assign24760_e33720_d_n2, assign24760_e33720_d_n6, assign24760_e33720_d_n7, assign24760_e33720_d_n10, assign24760_e33720_d_n11, assign24760_e33720_d_n12, assign24760_e33720_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24760_e33718: f64 = (locals.var_t4 * locals.var_qdrat);
        (assign24760_e33718, ((locals.var_t4_dn0 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn0)), ((locals.var_t4_dn2 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn2)), ((locals.var_t4_dn6 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn6)), ((locals.var_t4_dn7 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn7)), ((locals.var_t4_dn10 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn10)), ((locals.var_t4_dn11 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn11)), ((locals.var_t4_dn12 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn12)), ((locals.var_t4_dn17 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign24760_e33720;
        locals.var_qbody_bt_n_sud_dn0 = assign24760_e33720_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign24760_e33720_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign24760_e33720_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign24760_e33720_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign24760_e33720_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign24760_e33720_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign24760_e33720_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign24760_e33720_d_n17;

        let (assign24770_e33726, assign24770_e33726_d_n0, assign24770_e33726_d_n2, assign24770_e33726_d_n6, assign24770_e33726_d_n7, assign24770_e33726_d_n10, assign24770_e33726_d_n11, assign24770_e33726_d_n12, assign24770_e33726_d_n17,) = {
    if (locals.var_guard769 != 0.0) {
        let assign24770_e33724: f64 = (locals.var_t4 - locals.var_qbody_bt_n_sud);
        (assign24770_e33724, (locals.var_t4_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t4_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t4_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t4_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t4_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t4_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t4_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t4_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign24770_e33726;
        locals.var_qbody_bt_n_sus_dn0 = assign24770_e33726_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign24770_e33726_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign24770_e33726_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign24770_e33726_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign24770_e33726_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign24770_e33726_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign24770_e33726_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign24770_e33726_d_n17;

        let (assign24780_e33732, assign24780_e33732_d_n0, assign24780_e33732_d_n2, assign24780_e33732_d_n6, assign24780_e33732_d_n7, assign24780_e33732_d_n10, assign24780_e33732_d_n11, assign24780_e33732_d_n12, assign24780_e33732_d_n17,) = {
    if ((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    } else {
        (locals.var_uc_nsubbttub, locals.var_uc_nsubbttub_dn0, locals.var_uc_nsubbttub_dn2, locals.var_uc_nsubbttub_dn6, locals.var_uc_nsubbttub_dn7, locals.var_uc_nsubbttub_dn10, locals.var_uc_nsubbttub_dn11, locals.var_uc_nsubbttub_dn12, locals.var_uc_nsubbttub_dn17,)
    }
};
        locals.var_uc_nsubbttub = assign24780_e33732;
        locals.var_uc_nsubbttub_dn0 = assign24780_e33732_d_n0;
        locals.var_uc_nsubbttub_dn2 = assign24780_e33732_d_n2;
        locals.var_uc_nsubbttub_dn6 = assign24780_e33732_d_n6;
        locals.var_uc_nsubbttub_dn7 = assign24780_e33732_d_n7;
        locals.var_uc_nsubbttub_dn10 = assign24780_e33732_d_n10;
        locals.var_uc_nsubbttub_dn11 = assign24780_e33732_d_n11;
        locals.var_uc_nsubbttub_dn12 = assign24780_e33732_d_n12;
        locals.var_uc_nsubbttub_dn17 = assign24780_e33732_d_n17;

        let (assign24790_e33738,) = {
    if ((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24790_e33738;

        let assign24800_e33741: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard849 = assign24800_e33741;

        let assign24810_e33744: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard850 = assign24810_e33744;

        let (assign24820_e33754,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard849 != 0.0)) {
        let assign24820_e33752: f64 = (locals.var_area_bt_p * 0.5);
        (assign24820_e33752,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24820_e33754;

    }

    pub(super) fn stamp_transient_block_84(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24830_e33762,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard849 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24830_e33762;

        let (assign24840_e33770,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard849 != 0.0)) {
        (locals.var_cbtbp_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24840_e33770;

        let (assign24850_e33783,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard850 != 0.0) && (locals.var_guard849 == 0.0))) {
        let assign24850_e33781: f64 = (locals.var_area_bt_n * 0.5);
        (assign24850_e33781,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24850_e33783;

        let (assign24860_e33794,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard850 != 0.0) && (locals.var_guard849 == 0.0))) {
        (p.p68,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24860_e33794;

        let (assign24870_e33805,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard850 != 0.0) && (locals.var_guard849 == 0.0))) {
        (locals.var_cbtbn_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24870_e33805;

        let (assign24880_e33816,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard850 != 0.0) && (locals.var_guard849 == 0.0))) {
        (1.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24880_e33816;

        let assign24890_e33819: f64 = if locals.var_cbtb_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard851 = assign24890_e33819;

        let (assign24900_e33832, assign24900_e33832_d_n0, assign24900_e33832_d_n2, assign24900_e33832_d_n6, assign24900_e33832_d_n7, assign24900_e33832_d_n10, assign24900_e33832_d_n11, assign24900_e33832_d_n12, assign24900_e33832_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24900_e33828: f64 = (locals.var_uc_nsubbttub / locals.var_nsub);
        let assign24900_e33829: f64 = (assign24900_e33828).sqrt();
        let assign24900_e33830: f64 = (locals.var_cnst0soi * assign24900_e33829);
        (assign24900_e33830, ((locals.var_cnst0soi_dn0 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn0 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn2 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn2 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn6 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn6 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn7 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn7 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn10 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn10 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn11 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn11 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn12 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn12 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))), ((locals.var_cnst0soi_dn17 * assign24900_e33829) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn17 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn17)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24900_e33829)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn12, locals.var_cnst0over_dn17,)
    }
};
        locals.var_cnst0over = assign24900_e33832;
        locals.var_cnst0over_dn0 = assign24900_e33832_d_n0;
        locals.var_cnst0over_dn2 = assign24900_e33832_d_n2;
        locals.var_cnst0over_dn6 = assign24900_e33832_d_n6;
        locals.var_cnst0over_dn7 = assign24900_e33832_d_n7;
        locals.var_cnst0over_dn10 = assign24900_e33832_d_n10;
        locals.var_cnst0over_dn11 = assign24900_e33832_d_n11;
        locals.var_cnst0over_dn12 = assign24900_e33832_d_n12;
        locals.var_cnst0over_dn17 = assign24900_e33832_d_n17;

        let (assign24910_e33844,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24910_e33840: f64 = (1.0 - -1.0);
        let assign24910_e33842: f64 = (assign24910_e33840 / 2.0);
        (assign24910_e33842,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign24910_e33844;

        let (assign24920_e33856,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24920_e33852: f64 = (1.0 + -1.0);
        let assign24920_e33854: f64 = (assign24920_e33852 / 2.0);
        (assign24920_e33854,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign24920_e33856;

        let (assign24930_e33872, assign24930_e33872_d_n0, assign24930_e33872_d_n2, assign24930_e33872_d_n6, assign24930_e33872_d_n7, assign24930_e33872_d_n10, assign24930_e33872_d_n11, assign24930_e33872_d_n12, assign24930_e33872_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24930_e33864: f64 = (locals.var_modenml * locals.var_vbs);
        let assign24930_e33868: f64 = (locals.var_vbs - locals.var_vds);
        let assign24930_e33869: f64 = (locals.var_modervs * assign24930_e33868);
        let assign24930_e33870: f64 = (assign24930_e33864 + assign24930_e33869);
        (assign24930_e33870, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign24930_e33872;
        locals.var_vbsgmt_dn0 = assign24930_e33872_d_n0;
        locals.var_vbsgmt_dn2 = assign24930_e33872_d_n2;
        locals.var_vbsgmt_dn6 = assign24930_e33872_d_n6;
        locals.var_vbsgmt_dn7 = assign24930_e33872_d_n7;
        locals.var_vbsgmt_dn10 = assign24930_e33872_d_n10;
        locals.var_vbsgmt_dn11 = assign24930_e33872_d_n11;
        locals.var_vbsgmt_dn12 = assign24930_e33872_d_n12;
        locals.var_vbsgmt_dn17 = assign24930_e33872_d_n17;

        let (assign24940_e33887, assign24940_e33887_d_n0, assign24940_e33887_d_n2, assign24940_e33887_d_n6, assign24940_e33887_d_n7, assign24940_e33887_d_n10, assign24940_e33887_d_n11, assign24940_e33887_d_n12, assign24940_e33887_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24940_e33880: f64 = (locals.var_modenml * locals.var_vds);
        let assign24940_e33883: f64 = (-locals.var_vds);
        let assign24940_e33884: f64 = (locals.var_modervs * assign24940_e33883);
        let assign24940_e33885: f64 = (assign24940_e33880 + assign24940_e33884);
        (assign24940_e33885, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign24940_e33887;
        locals.var_vdsgmt_dn0 = assign24940_e33887_d_n0;
        locals.var_vdsgmt_dn2 = assign24940_e33887_d_n2;
        locals.var_vdsgmt_dn6 = assign24940_e33887_d_n6;
        locals.var_vdsgmt_dn7 = assign24940_e33887_d_n7;
        locals.var_vdsgmt_dn10 = assign24940_e33887_d_n10;
        locals.var_vdsgmt_dn11 = assign24940_e33887_d_n11;
        locals.var_vdsgmt_dn12 = assign24940_e33887_d_n12;
        locals.var_vdsgmt_dn17 = assign24940_e33887_d_n17;

        let (assign24950_e33903, assign24950_e33903_d_n0, assign24950_e33903_d_n2, assign24950_e33903_d_n6, assign24950_e33903_d_n7, assign24950_e33903_d_n10, assign24950_e33903_d_n11, assign24950_e33903_d_n12, assign24950_e33903_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24950_e33895: f64 = (locals.var_modenml * locals.var_vgs);
        let assign24950_e33899: f64 = (locals.var_vgs - locals.var_vds);
        let assign24950_e33900: f64 = (locals.var_modervs * assign24950_e33899);
        let assign24950_e33901: f64 = (assign24950_e33895 + assign24950_e33900);
        (assign24950_e33901, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign24950_e33903;
        locals.var_vgsgmt_dn0 = assign24950_e33903_d_n0;
        locals.var_vgsgmt_dn2 = assign24950_e33903_d_n2;
        locals.var_vgsgmt_dn6 = assign24950_e33903_d_n6;
        locals.var_vgsgmt_dn7 = assign24950_e33903_d_n7;
        locals.var_vgsgmt_dn10 = assign24950_e33903_d_n10;
        locals.var_vgsgmt_dn11 = assign24950_e33903_d_n11;
        locals.var_vgsgmt_dn12 = assign24950_e33903_d_n12;
        locals.var_vgsgmt_dn17 = assign24950_e33903_d_n17;

        let (assign24960_e33919, assign24960_e33919_d_n0, assign24960_e33919_d_n2, assign24960_e33919_d_n6, assign24960_e33919_d_n7, assign24960_e33919_d_n10, assign24960_e33919_d_n11, assign24960_e33919_d_n12, assign24960_e33919_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24960_e33911: f64 = (locals.var_modervs * locals.var_vgs);
        let assign24960_e33915: f64 = (locals.var_vgs - locals.var_vds);
        let assign24960_e33916: f64 = (locals.var_modenml * assign24960_e33915);
        let assign24960_e33917: f64 = (assign24960_e33911 + assign24960_e33916);
        (assign24960_e33917, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign24960_e33919;
        locals.var_vgdgmt_dn0 = assign24960_e33919_d_n0;
        locals.var_vgdgmt_dn2 = assign24960_e33919_d_n2;
        locals.var_vgdgmt_dn6 = assign24960_e33919_d_n6;
        locals.var_vgdgmt_dn7 = assign24960_e33919_d_n7;
        locals.var_vgdgmt_dn10 = assign24960_e33919_d_n10;
        locals.var_vgdgmt_dn11 = assign24960_e33919_d_n11;
        locals.var_vgdgmt_dn12 = assign24960_e33919_d_n12;
        locals.var_vgdgmt_dn17 = assign24960_e33919_d_n17;

        let (assign24970_e33929, assign24970_e33929_d_n0, assign24970_e33929_d_n2, assign24970_e33929_d_n6, assign24970_e33929_d_n7, assign24970_e33929_d_n10, assign24970_e33929_d_n11, assign24970_e33929_d_n12, assign24970_e33929_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24970_e33927: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign24970_e33927, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign24970_e33929;
        locals.var_vdbgmt_dn0 = assign24970_e33929_d_n0;
        locals.var_vdbgmt_dn2 = assign24970_e33929_d_n2;
        locals.var_vdbgmt_dn6 = assign24970_e33929_d_n6;
        locals.var_vdbgmt_dn7 = assign24970_e33929_d_n7;
        locals.var_vdbgmt_dn10 = assign24970_e33929_d_n10;
        locals.var_vdbgmt_dn11 = assign24970_e33929_d_n11;
        locals.var_vdbgmt_dn12 = assign24970_e33929_d_n12;
        locals.var_vdbgmt_dn17 = assign24970_e33929_d_n17;

        let (assign24980_e33938, assign24980_e33938_d_n0, assign24980_e33938_d_n2, assign24980_e33938_d_n6, assign24980_e33938_d_n7, assign24980_e33938_d_n10, assign24980_e33938_d_n11, assign24980_e33938_d_n12, assign24980_e33938_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24980_e33936: f64 = (-locals.var_vbsgmt);
        (assign24980_e33936, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign24980_e33938;
        locals.var_vsbgmt_dn0 = assign24980_e33938_d_n0;
        locals.var_vsbgmt_dn2 = assign24980_e33938_d_n2;
        locals.var_vsbgmt_dn6 = assign24980_e33938_d_n6;
        locals.var_vsbgmt_dn7 = assign24980_e33938_d_n7;
        locals.var_vsbgmt_dn10 = assign24980_e33938_d_n10;
        locals.var_vsbgmt_dn11 = assign24980_e33938_d_n11;
        locals.var_vsbgmt_dn12 = assign24980_e33938_d_n12;
        locals.var_vsbgmt_dn17 = assign24980_e33938_d_n17;

        let (assign24990_e33952,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24990_e33946: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign24990_e33949: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign24990_e33950: f64 = (assign24990_e33946 + assign24990_e33949);
        (assign24990_e33950,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign24990_e33952;

        let (assign25000_e33966,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25000_e33960: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign25000_e33963: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign25000_e33964: f64 = (assign25000_e33960 + assign25000_e33963);
        (assign25000_e33964,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign25000_e33966;

        let (assign25010_e33980, assign25010_e33980_d_n0, assign25010_e33980_d_n2, assign25010_e33980_d_n6, assign25010_e33980_d_n7, assign25010_e33980_d_n10, assign25010_e33980_d_n11, assign25010_e33980_d_n12, assign25010_e33980_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25010_e33974: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign25010_e33977: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign25010_e33978: f64 = (assign25010_e33974 + assign25010_e33977);
        (assign25010_e33978, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign25010_e33980;
        locals.var_vgbgmt_dn0 = assign25010_e33980_d_n0;
        locals.var_vgbgmt_dn2 = assign25010_e33980_d_n2;
        locals.var_vgbgmt_dn6 = assign25010_e33980_d_n6;
        locals.var_vgbgmt_dn7 = assign25010_e33980_d_n7;
        locals.var_vgbgmt_dn10 = assign25010_e33980_d_n10;
        locals.var_vgbgmt_dn11 = assign25010_e33980_d_n11;
        locals.var_vgbgmt_dn12 = assign25010_e33980_d_n12;
        locals.var_vgbgmt_dn17 = assign25010_e33980_d_n17;

        let (assign25020_e33998, assign25020_e33998_d_n0, assign25020_e33998_d_n2, assign25020_e33998_d_n6, assign25020_e33998_d_n7, assign25020_e33998_d_n10, assign25020_e33998_d_n11, assign25020_e33998_d_n12, assign25020_e33998_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25020_e33988: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign25020_e33991: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign25020_e33992: f64 = (assign25020_e33988 + assign25020_e33991);
        let assign25020_e33995: f64 = (10.0 * 2.220446049250313e-16);
        let assign25020_e33996: f64 = (assign25020_e33992 + assign25020_e33995);
        (assign25020_e33996, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign25020_e33998;
        locals.var_vxbgmt_dn0 = assign25020_e33998_d_n0;
        locals.var_vxbgmt_dn2 = assign25020_e33998_d_n2;
        locals.var_vxbgmt_dn6 = assign25020_e33998_d_n6;
        locals.var_vxbgmt_dn7 = assign25020_e33998_d_n7;
        locals.var_vxbgmt_dn10 = assign25020_e33998_d_n10;
        locals.var_vxbgmt_dn11 = assign25020_e33998_d_n11;
        locals.var_vxbgmt_dn12 = assign25020_e33998_d_n12;
        locals.var_vxbgmt_dn17 = assign25020_e33998_d_n17;

        let (assign25030_e34007, assign25030_e34007_d_n0, assign25030_e34007_d_n2, assign25030_e34007_d_n6, assign25030_e34007_d_n7, assign25030_e34007_d_n10, assign25030_e34007_d_n11, assign25030_e34007_d_n12, assign25030_e34007_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25030_e34005: f64 = (-locals.var_vxbgmt);
        (assign25030_e34005, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign25030_e34007;
        locals.var_t0__blk770_dn0 = assign25030_e34007_d_n0;
        locals.var_t0__blk770_dn2 = assign25030_e34007_d_n2;
        locals.var_t0__blk770_dn6 = assign25030_e34007_d_n6;
        locals.var_t0__blk770_dn7 = assign25030_e34007_d_n7;
        locals.var_t0__blk770_dn10 = assign25030_e34007_d_n10;
        locals.var_t0__blk770_dn11 = assign25030_e34007_d_n11;
        locals.var_t0__blk770_dn12 = assign25030_e34007_d_n12;
        locals.var_t0__blk770_dn17 = assign25030_e34007_d_n17;

        let assign25040_e34010: f64 = if locals.var_t0__blk770 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard852 = assign25040_e34010;

        let (assign25050_e34022, assign25050_e34022_d_n0, assign25050_e34022_d_n2, assign25050_e34022_d_n6, assign25050_e34022_d_n7, assign25050_e34022_d_n10, assign25050_e34022_d_n11, assign25050_e34022_d_n12, assign25050_e34022_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25050_e34020: f64 = (locals.var_t0__blk770 - locals.var_vbs_bnd);
        (assign25050_e34020, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign25050_e34022;
        locals.var_t1__blk771_dn0 = assign25050_e34022_d_n0;
        locals.var_t1__blk771_dn2 = assign25050_e34022_d_n2;
        locals.var_t1__blk771_dn6 = assign25050_e34022_d_n6;
        locals.var_t1__blk771_dn7 = assign25050_e34022_d_n7;
        locals.var_t1__blk771_dn10 = assign25050_e34022_d_n10;
        locals.var_t1__blk771_dn11 = assign25050_e34022_d_n11;
        locals.var_t1__blk771_dn12 = assign25050_e34022_d_n12;
        locals.var_t1__blk771_dn17 = assign25050_e34022_d_n17;

        let (assign25060_e34034, assign25060_e34034_d_n0, assign25060_e34034_d_n2, assign25060_e34034_d_n6, assign25060_e34034_d_n7, assign25060_e34034_d_n10, assign25060_e34034_d_n11, assign25060_e34034_d_n12, assign25060_e34034_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25060_e34032: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign25060_e34032, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign25060_e34034;
        locals.var_t2__blk772_dn0 = assign25060_e34034_d_n0;
        locals.var_t2__blk772_dn2 = assign25060_e34034_d_n2;
        locals.var_t2__blk772_dn6 = assign25060_e34034_d_n6;
        locals.var_t2__blk772_dn7 = assign25060_e34034_d_n7;
        locals.var_t2__blk772_dn10 = assign25060_e34034_d_n10;
        locals.var_t2__blk772_dn11 = assign25060_e34034_d_n11;
        locals.var_t2__blk772_dn12 = assign25060_e34034_d_n12;
        locals.var_t2__blk772_dn17 = assign25060_e34034_d_n17;

        let (assign25070_e34046, assign25070_e34046_d_n0, assign25070_e34046_d_n2, assign25070_e34046_d_n6, assign25070_e34046_d_n7, assign25070_e34046_d_n10, assign25070_e34046_d_n11, assign25070_e34046_d_n12, assign25070_e34046_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25070_e34044: f64 = (locals.var_t1__blk771 / locals.var_t2__blk772);
        (assign25070_e34044, (((locals.var_t1__blk771_dn0 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn0)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn2 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn2)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn6 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn6)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn7 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn7)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn10 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn10)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn11 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn11)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn12 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn12)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn17 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn17)) / (locals.var_t2__blk772 * locals.var_t2__blk772)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25070_e34046;
        locals.var_tmf1_dn0 = assign25070_e34046_d_n0;
        locals.var_tmf1_dn2 = assign25070_e34046_d_n2;
        locals.var_tmf1_dn6 = assign25070_e34046_d_n6;
        locals.var_tmf1_dn7 = assign25070_e34046_d_n7;
        locals.var_tmf1_dn10 = assign25070_e34046_d_n10;
        locals.var_tmf1_dn11 = assign25070_e34046_d_n11;
        locals.var_tmf1_dn12 = assign25070_e34046_d_n12;
        locals.var_tmf1_dn17 = assign25070_e34046_d_n17;

        let (assign25080_e34058, assign25080_e34058_d_n0, assign25080_e34058_d_n2, assign25080_e34058_d_n6, assign25080_e34058_d_n7, assign25080_e34058_d_n10, assign25080_e34058_d_n11, assign25080_e34058_d_n12, assign25080_e34058_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25080_e34056: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25080_e34056, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25080_e34058;
        locals.var_tmf2_dn0 = assign25080_e34058_d_n0;
        locals.var_tmf2_dn2 = assign25080_e34058_d_n2;
        locals.var_tmf2_dn6 = assign25080_e34058_d_n6;
        locals.var_tmf2_dn7 = assign25080_e34058_d_n7;
        locals.var_tmf2_dn10 = assign25080_e34058_d_n10;
        locals.var_tmf2_dn11 = assign25080_e34058_d_n11;
        locals.var_tmf2_dn12 = assign25080_e34058_d_n12;
        locals.var_tmf2_dn17 = assign25080_e34058_d_n17;

        let (assign25090_e34070, assign25090_e34070_d_n0, assign25090_e34070_d_n2, assign25090_e34070_d_n6, assign25090_e34070_d_n7, assign25090_e34070_d_n10, assign25090_e34070_d_n11, assign25090_e34070_d_n12, assign25090_e34070_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25090_e34068: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign25090_e34068, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign25090_e34070;
        locals.var_tmf3_dn0 = assign25090_e34070_d_n0;
        locals.var_tmf3_dn2 = assign25090_e34070_d_n2;
        locals.var_tmf3_dn6 = assign25090_e34070_d_n6;
        locals.var_tmf3_dn7 = assign25090_e34070_d_n7;
        locals.var_tmf3_dn10 = assign25090_e34070_d_n10;
        locals.var_tmf3_dn11 = assign25090_e34070_d_n11;
        locals.var_tmf3_dn12 = assign25090_e34070_d_n12;
        locals.var_tmf3_dn17 = assign25090_e34070_d_n17;

        let (assign25100_e34082, assign25100_e34082_d_n0, assign25100_e34082_d_n2, assign25100_e34082_d_n6, assign25100_e34082_d_n7, assign25100_e34082_d_n10, assign25100_e34082_d_n11, assign25100_e34082_d_n12, assign25100_e34082_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25100_e34080: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign25100_e34080, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign25100_e34082;
        locals.var_tmf4_dn0 = assign25100_e34082_d_n0;
        locals.var_tmf4_dn2 = assign25100_e34082_d_n2;
        locals.var_tmf4_dn6 = assign25100_e34082_d_n6;
        locals.var_tmf4_dn7 = assign25100_e34082_d_n7;
        locals.var_tmf4_dn10 = assign25100_e34082_d_n10;
        locals.var_tmf4_dn11 = assign25100_e34082_d_n11;
        locals.var_tmf4_dn12 = assign25100_e34082_d_n12;
        locals.var_tmf4_dn17 = assign25100_e34082_d_n17;

        let (assign25110_e34102, assign25110_e34102_d_n0, assign25110_e34102_d_n2, assign25110_e34102_d_n6, assign25110_e34102_d_n7, assign25110_e34102_d_n10, assign25110_e34102_d_n11, assign25110_e34102_d_n12, assign25110_e34102_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25110_e34093: f64 = (1.0 + locals.var_tmf1);
        let assign25110_e34095: f64 = (assign25110_e34093 + locals.var_tmf2);
        let assign25110_e34097: f64 = (assign25110_e34095 + locals.var_tmf3);
        let assign25110_e34099: f64 = (assign25110_e34097 + locals.var_tmf4);
        let assign25110_e34100: f64 = (1.0 / assign25110_e34099);
        (assign25110_e34100, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign25110_e34099 * assign25110_e34099))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign25110_e34099 * assign25110_e34099))),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign25110_e34102;
        locals.var_ty__blk778_dn0 = assign25110_e34102_d_n0;
        locals.var_ty__blk778_dn2 = assign25110_e34102_d_n2;
        locals.var_ty__blk778_dn6 = assign25110_e34102_d_n6;
        locals.var_ty__blk778_dn7 = assign25110_e34102_d_n7;
        locals.var_ty__blk778_dn10 = assign25110_e34102_d_n10;
        locals.var_ty__blk778_dn11 = assign25110_e34102_d_n11;
        locals.var_ty__blk778_dn12 = assign25110_e34102_d_n12;
        locals.var_ty__blk778_dn17 = assign25110_e34102_d_n17;

        let (assign25130_e34143, assign25130_e34143_d_n0, assign25130_e34143_d_n2, assign25130_e34143_d_n6, assign25130_e34143_d_n7, assign25130_e34143_d_n10, assign25130_e34143_d_n11, assign25130_e34143_d_n12, assign25130_e34143_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25130_e34140: f64 = (1.0 - locals.var_ty__blk778);
        let assign25130_e34141: f64 = (locals.var_t2__blk772 * assign25130_e34140);
        (assign25130_e34141, ((locals.var_t2__blk772_dn0 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn0))), ((locals.var_t2__blk772_dn2 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn2))), ((locals.var_t2__blk772_dn6 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn6))), ((locals.var_t2__blk772_dn7 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn7))), ((locals.var_t2__blk772_dn10 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn10))), ((locals.var_t2__blk772_dn11 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn11))), ((locals.var_t2__blk772_dn12 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn12))), ((locals.var_t2__blk772_dn17 * assign25130_e34140) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn17))),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign25130_e34143;
        locals.var_ty__blk778_dn0 = assign25130_e34143_d_n0;
        locals.var_ty__blk778_dn2 = assign25130_e34143_d_n2;
        locals.var_ty__blk778_dn6 = assign25130_e34143_d_n6;
        locals.var_ty__blk778_dn7 = assign25130_e34143_d_n7;
        locals.var_ty__blk778_dn10 = assign25130_e34143_d_n10;
        locals.var_ty__blk778_dn11 = assign25130_e34143_d_n11;
        locals.var_ty__blk778_dn12 = assign25130_e34143_d_n12;
        locals.var_ty__blk778_dn17 = assign25130_e34143_d_n17;

        let (assign25150_e34166, assign25150_e34166_d_n0, assign25150_e34166_d_n2, assign25150_e34166_d_n6, assign25150_e34166_d_n7, assign25150_e34166_d_n10, assign25150_e34166_d_n11, assign25150_e34166_d_n12, assign25150_e34166_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 != 0.0)) {
        let assign25150_e34164: f64 = (locals.var_vbs_bnd + locals.var_ty__blk778);
        (assign25150_e34164, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    } else {
        (locals.var_t10__blk775, locals.var_t10__blk775_dn0, locals.var_t10__blk775_dn2, locals.var_t10__blk775_dn6, locals.var_t10__blk775_dn7, locals.var_t10__blk775_dn10, locals.var_t10__blk775_dn11, locals.var_t10__blk775_dn12, locals.var_t10__blk775_dn17,)
    }
};
        locals.var_t10__blk775 = assign25150_e34166;
        locals.var_t10__blk775_dn0 = assign25150_e34166_d_n0;
        locals.var_t10__blk775_dn2 = assign25150_e34166_d_n2;
        locals.var_t10__blk775_dn6 = assign25150_e34166_d_n6;
        locals.var_t10__blk775_dn7 = assign25150_e34166_d_n7;
        locals.var_t10__blk775_dn10 = assign25150_e34166_d_n10;
        locals.var_t10__blk775_dn11 = assign25150_e34166_d_n11;
        locals.var_t10__blk775_dn12 = assign25150_e34166_d_n12;
        locals.var_t10__blk775_dn17 = assign25150_e34166_d_n17;

        let (assign25160_e34177, assign25160_e34177_d_n0, assign25160_e34177_d_n2, assign25160_e34177_d_n6, assign25160_e34177_d_n7, assign25160_e34177_d_n10, assign25160_e34177_d_n11, assign25160_e34177_d_n12, assign25160_e34177_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard852 == 0.0)) {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    } else {
        (locals.var_t10__blk775, locals.var_t10__blk775_dn0, locals.var_t10__blk775_dn2, locals.var_t10__blk775_dn6, locals.var_t10__blk775_dn7, locals.var_t10__blk775_dn10, locals.var_t10__blk775_dn11, locals.var_t10__blk775_dn12, locals.var_t10__blk775_dn17,)
    }
};
        locals.var_t10__blk775 = assign25160_e34177;
        locals.var_t10__blk775_dn0 = assign25160_e34177_d_n0;
        locals.var_t10__blk775_dn2 = assign25160_e34177_d_n2;
        locals.var_t10__blk775_dn6 = assign25160_e34177_d_n6;
        locals.var_t10__blk775_dn7 = assign25160_e34177_d_n7;
        locals.var_t10__blk775_dn10 = assign25160_e34177_d_n10;
        locals.var_t10__blk775_dn11 = assign25160_e34177_d_n11;
        locals.var_t10__blk775_dn12 = assign25160_e34177_d_n12;
        locals.var_t10__blk775_dn17 = assign25160_e34177_d_n17;

        let (assign25180_e34199, assign25180_e34199_d_n0, assign25180_e34199_d_n2, assign25180_e34199_d_n6, assign25180_e34199_d_n7, assign25180_e34199_d_n10, assign25180_e34199_d_n11, assign25180_e34199_d_n12, assign25180_e34199_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25180_e34195: f64 = (-locals.var_t10__blk775);
        let assign25180_e34197: f64 = (assign25180_e34195 - 1e-12);
        (assign25180_e34197, (-locals.var_t10__blk775_dn0), (-locals.var_t10__blk775_dn2), (-locals.var_t10__blk775_dn6), (-locals.var_t10__blk775_dn7), (-locals.var_t10__blk775_dn10), (-locals.var_t10__blk775_dn11), (-locals.var_t10__blk775_dn12), (-locals.var_t10__blk775_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign25180_e34199;
        locals.var_vxbgmtcl_dn0 = assign25180_e34199_d_n0;
        locals.var_vxbgmtcl_dn2 = assign25180_e34199_d_n2;
        locals.var_vxbgmtcl_dn6 = assign25180_e34199_d_n6;
        locals.var_vxbgmtcl_dn7 = assign25180_e34199_d_n7;
        locals.var_vxbgmtcl_dn10 = assign25180_e34199_d_n10;
        locals.var_vxbgmtcl_dn11 = assign25180_e34199_d_n11;
        locals.var_vxbgmtcl_dn12 = assign25180_e34199_d_n12;
        locals.var_vxbgmtcl_dn17 = assign25180_e34199_d_n17;

    }

    pub(super) fn stamp_transient_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25190_e34209, assign25190_e34209_d_n0, assign25190_e34209_d_n2, assign25190_e34209_d_n6, assign25190_e34209_d_n7, assign25190_e34209_d_n10, assign25190_e34209_d_n11, assign25190_e34209_d_n12, assign25190_e34209_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25190_e34207: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign25190_e34207, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk800, locals.var_fac1__blk800_dn0, locals.var_fac1__blk800_dn2, locals.var_fac1__blk800_dn6, locals.var_fac1__blk800_dn7, locals.var_fac1__blk800_dn10, locals.var_fac1__blk800_dn11, locals.var_fac1__blk800_dn12, locals.var_fac1__blk800_dn17,)
    }
};
        locals.var_fac1__blk800 = assign25190_e34209;
        locals.var_fac1__blk800_dn0 = assign25190_e34209_d_n0;
        locals.var_fac1__blk800_dn2 = assign25190_e34209_d_n2;
        locals.var_fac1__blk800_dn6 = assign25190_e34209_d_n6;
        locals.var_fac1__blk800_dn7 = assign25190_e34209_d_n7;
        locals.var_fac1__blk800_dn10 = assign25190_e34209_d_n10;
        locals.var_fac1__blk800_dn11 = assign25190_e34209_d_n11;
        locals.var_fac1__blk800_dn12 = assign25190_e34209_d_n12;
        locals.var_fac1__blk800_dn17 = assign25190_e34209_d_n17;

        let (assign25200_e34219, assign25200_e34219_d_n0, assign25200_e34219_d_n2, assign25200_e34219_d_n6, assign25200_e34219_d_n7, assign25200_e34219_d_n10, assign25200_e34219_d_n11, assign25200_e34219_d_n12, assign25200_e34219_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25200_e34217: f64 = (locals.var_fac1__blk800 * locals.var_fac1__blk800);
        (assign25200_e34217, ((locals.var_fac1__blk800_dn0 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn0)), ((locals.var_fac1__blk800_dn2 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn2)), ((locals.var_fac1__blk800_dn6 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn6)), ((locals.var_fac1__blk800_dn7 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn7)), ((locals.var_fac1__blk800_dn10 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn10)), ((locals.var_fac1__blk800_dn11 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn11)), ((locals.var_fac1__blk800_dn12 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn12)), ((locals.var_fac1__blk800_dn17 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn17)),)
    } else {
        (locals.var_fac1p2__blk801, locals.var_fac1p2__blk801_dn0, locals.var_fac1p2__blk801_dn2, locals.var_fac1p2__blk801_dn6, locals.var_fac1p2__blk801_dn7, locals.var_fac1p2__blk801_dn10, locals.var_fac1p2__blk801_dn11, locals.var_fac1p2__blk801_dn12, locals.var_fac1p2__blk801_dn17,)
    }
};
        locals.var_fac1p2__blk801 = assign25200_e34219;
        locals.var_fac1p2__blk801_dn0 = assign25200_e34219_d_n0;
        locals.var_fac1p2__blk801_dn2 = assign25200_e34219_d_n2;
        locals.var_fac1p2__blk801_dn6 = assign25200_e34219_d_n6;
        locals.var_fac1p2__blk801_dn7 = assign25200_e34219_d_n7;
        locals.var_fac1p2__blk801_dn10 = assign25200_e34219_d_n10;
        locals.var_fac1p2__blk801_dn11 = assign25200_e34219_d_n11;
        locals.var_fac1p2__blk801_dn12 = assign25200_e34219_d_n12;
        locals.var_fac1p2__blk801_dn17 = assign25200_e34219_d_n17;

        let (assign25210_e34229, assign25210_e34229_d_n0, assign25210_e34229_d_n2, assign25210_e34229_d_n6, assign25210_e34229_d_n7, assign25210_e34229_d_n10, assign25210_e34229_d_n11, assign25210_e34229_d_n12, assign25210_e34229_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25210_e34227: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign25210_e34227, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign25210_e34229;
        locals.var_vgpld_dn0 = assign25210_e34229_d_n0;
        locals.var_vgpld_dn2 = assign25210_e34229_d_n2;
        locals.var_vgpld_dn6 = assign25210_e34229_d_n6;
        locals.var_vgpld_dn7 = assign25210_e34229_d_n7;
        locals.var_vgpld_dn10 = assign25210_e34229_d_n10;
        locals.var_vgpld_dn11 = assign25210_e34229_d_n11;
        locals.var_vgpld_dn12 = assign25210_e34229_d_n12;
        locals.var_vgpld_dn17 = assign25210_e34229_d_n17;

        let (assign25220_e34239, assign25220_e34239_d_n0, assign25220_e34239_d_n2, assign25220_e34239_d_n6, assign25220_e34239_d_n7, assign25220_e34239_d_n10, assign25220_e34239_d_n11, assign25220_e34239_d_n12, assign25220_e34239_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25220_e34237: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign25220_e34237, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign25220_e34239;
        locals.var_t0__blk770_dn0 = assign25220_e34239_d_n0;
        locals.var_t0__blk770_dn2 = assign25220_e34239_d_n2;
        locals.var_t0__blk770_dn6 = assign25220_e34239_d_n6;
        locals.var_t0__blk770_dn7 = assign25220_e34239_d_n7;
        locals.var_t0__blk770_dn10 = assign25220_e34239_d_n10;
        locals.var_t0__blk770_dn11 = assign25220_e34239_d_n11;
        locals.var_t0__blk770_dn12 = assign25220_e34239_d_n12;
        locals.var_t0__blk770_dn17 = assign25220_e34239_d_n17;

        let (assign25230_e34252, assign25230_e34252_d_n0, assign25230_e34252_d_n2, assign25230_e34252_d_n6, assign25230_e34252_d_n7, assign25230_e34252_d_n10, assign25230_e34252_d_n11, assign25230_e34252_d_n12, assign25230_e34252_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25230_e34247: f64 = (2.0 / locals.var_beta);
        let assign25230_e34249: f64 = (locals.var_t0__blk770).ln();
        let assign25230_e34250: f64 = (assign25230_e34247 * assign25230_e34249);
        (assign25230_e34250, (assign25230_e34247 * (locals.var_t0__blk770_dn0 / locals.var_t0__blk770)), (assign25230_e34247 * (locals.var_t0__blk770_dn2 / locals.var_t0__blk770)), (assign25230_e34247 * (locals.var_t0__blk770_dn6 / locals.var_t0__blk770)), (assign25230_e34247 * (locals.var_t0__blk770_dn7 / locals.var_t0__blk770)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign25230_e34249) + (assign25230_e34247 * (locals.var_t0__blk770_dn10 / locals.var_t0__blk770))), (assign25230_e34247 * (locals.var_t0__blk770_dn11 / locals.var_t0__blk770)), (assign25230_e34247 * (locals.var_t0__blk770_dn12 / locals.var_t0__blk770)), (assign25230_e34247 * (locals.var_t0__blk770_dn17 / locals.var_t0__blk770)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign25230_e34252;
        locals.var_pb2over_dn0 = assign25230_e34252_d_n0;
        locals.var_pb2over_dn2 = assign25230_e34252_d_n2;
        locals.var_pb2over_dn6 = assign25230_e34252_d_n6;
        locals.var_pb2over_dn7 = assign25230_e34252_d_n7;
        locals.var_pb2over_dn10 = assign25230_e34252_d_n10;
        locals.var_pb2over_dn11 = assign25230_e34252_d_n11;
        locals.var_pb2over_dn12 = assign25230_e34252_d_n12;
        locals.var_pb2over_dn17 = assign25230_e34252_d_n17;

        let (assign25240_e34261,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign25240_e34259: f64 = (-locals.var_vxbgmtcl);
        (assign25240_e34259,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign25240_e34261;

        let assign25250_e34264: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard853 = assign25250_e34264;

        let (assign25270_e34289, assign25270_e34289_d_n0, assign25270_e34289_d_n2, assign25270_e34289_d_n6, assign25270_e34289_d_n7, assign25270_e34289_d_n10, assign25270_e34289_d_n11, assign25270_e34289_d_n12, assign25270_e34289_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25270_e34286: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign25270_e34287: f64 = (1.0 / assign25270_e34286);
        (assign25270_e34287, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign25270_e34286 * assign25270_e34286))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign25270_e34286 * assign25270_e34286))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign25270_e34286 * assign25270_e34286))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign25270_e34289;
        locals.var_t1__blk771_dn0 = assign25270_e34289_d_n0;
        locals.var_t1__blk771_dn2 = assign25270_e34289_d_n2;
        locals.var_t1__blk771_dn6 = assign25270_e34289_d_n6;
        locals.var_t1__blk771_dn7 = assign25270_e34289_d_n7;
        locals.var_t1__blk771_dn10 = assign25270_e34289_d_n10;
        locals.var_t1__blk771_dn11 = assign25270_e34289_d_n11;
        locals.var_t1__blk771_dn12 = assign25270_e34289_d_n12;
        locals.var_t1__blk771_dn17 = assign25270_e34289_d_n17;

        let (assign25280_e34301, assign25280_e34301_d_n0, assign25280_e34301_d_n2, assign25280_e34301_d_n6, assign25280_e34301_d_n7, assign25280_e34301_d_n10, assign25280_e34301_d_n11, assign25280_e34301_d_n12, assign25280_e34301_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25280_e34299: f64 = (locals.var_t1__blk771 * locals.var_cox0);
        (assign25280_e34299, (locals.var_t1__blk771_dn0 * locals.var_cox0), (locals.var_t1__blk771_dn2 * locals.var_cox0), (locals.var_t1__blk771_dn6 * locals.var_cox0), (locals.var_t1__blk771_dn7 * locals.var_cox0), (locals.var_t1__blk771_dn10 * locals.var_cox0), (locals.var_t1__blk771_dn11 * locals.var_cox0), (locals.var_t1__blk771_dn12 * locals.var_cox0), (locals.var_t1__blk771_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign25280_e34301;
        locals.var_ty__blk778_dn0 = assign25280_e34301_d_n0;
        locals.var_ty__blk778_dn2 = assign25280_e34301_d_n2;
        locals.var_ty__blk778_dn6 = assign25280_e34301_d_n6;
        locals.var_ty__blk778_dn7 = assign25280_e34301_d_n7;
        locals.var_ty__blk778_dn10 = assign25280_e34301_d_n10;
        locals.var_ty__blk778_dn11 = assign25280_e34301_d_n11;
        locals.var_ty__blk778_dn12 = assign25280_e34301_d_n12;
        locals.var_ty__blk778_dn17 = assign25280_e34301_d_n17;

        let (assign25290_e34317, assign25290_e34317_d_n0, assign25290_e34317_d_n2, assign25290_e34317_d_n6, assign25290_e34317_d_n7, assign25290_e34317_d_n10, assign25290_e34317_d_n11, assign25290_e34317_d_n12, assign25290_e34317_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25290_e34312: f64 = (3.0 * 1.414213562373095);
        let assign25290_e34314: f64 = (assign25290_e34312 * locals.var_ty__blk778);
        let assign25290_e34315: f64 = (2.0 + assign25290_e34314);
        (assign25290_e34315, (assign25290_e34312 * locals.var_ty__blk778_dn0), (assign25290_e34312 * locals.var_ty__blk778_dn2), (assign25290_e34312 * locals.var_ty__blk778_dn6), (assign25290_e34312 * locals.var_ty__blk778_dn7), (assign25290_e34312 * locals.var_ty__blk778_dn10), (assign25290_e34312 * locals.var_ty__blk778_dn11), (assign25290_e34312 * locals.var_ty__blk778_dn12), (assign25290_e34312 * locals.var_ty__blk778_dn17),)
    } else {
        (locals.var_ac41__blk805, locals.var_ac41__blk805_dn0, locals.var_ac41__blk805_dn2, locals.var_ac41__blk805_dn6, locals.var_ac41__blk805_dn7, locals.var_ac41__blk805_dn10, locals.var_ac41__blk805_dn11, locals.var_ac41__blk805_dn12, locals.var_ac41__blk805_dn17,)
    }
};
        locals.var_ac41__blk805 = assign25290_e34317;
        locals.var_ac41__blk805_dn0 = assign25290_e34317_d_n0;
        locals.var_ac41__blk805_dn2 = assign25290_e34317_d_n2;
        locals.var_ac41__blk805_dn6 = assign25290_e34317_d_n6;
        locals.var_ac41__blk805_dn7 = assign25290_e34317_d_n7;
        locals.var_ac41__blk805_dn10 = assign25290_e34317_d_n10;
        locals.var_ac41__blk805_dn11 = assign25290_e34317_d_n11;
        locals.var_ac41__blk805_dn12 = assign25290_e34317_d_n12;
        locals.var_ac41__blk805_dn17 = assign25290_e34317_d_n17;

        let (assign25300_e34333, assign25300_e34333_d_n0, assign25300_e34333_d_n2, assign25300_e34333_d_n6, assign25300_e34333_d_n7, assign25300_e34333_d_n10, assign25300_e34333_d_n11, assign25300_e34333_d_n12, assign25300_e34333_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25300_e34327: f64 = (8.0 * locals.var_ac41__blk805);
        let assign25300_e34329: f64 = (assign25300_e34327 * locals.var_ac41__blk805);
        let assign25300_e34331: f64 = (assign25300_e34329 * locals.var_ac41__blk805);
        (assign25300_e34331, (((((8.0 * locals.var_ac41__blk805_dn0) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn0)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn0)), (((((8.0 * locals.var_ac41__blk805_dn2) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn2)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn2)), (((((8.0 * locals.var_ac41__blk805_dn6) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn6)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn6)), (((((8.0 * locals.var_ac41__blk805_dn7) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn7)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn7)), (((((8.0 * locals.var_ac41__blk805_dn10) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn10)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn10)), (((((8.0 * locals.var_ac41__blk805_dn11) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn11)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn11)), (((((8.0 * locals.var_ac41__blk805_dn12) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn12)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn12)), (((((8.0 * locals.var_ac41__blk805_dn17) * locals.var_ac41__blk805) + (assign25300_e34327 * locals.var_ac41__blk805_dn17)) * locals.var_ac41__blk805) + (assign25300_e34329 * locals.var_ac41__blk805_dn17)),)
    } else {
        (locals.var_ac4__blk806, locals.var_ac4__blk806_dn0, locals.var_ac4__blk806_dn2, locals.var_ac4__blk806_dn6, locals.var_ac4__blk806_dn7, locals.var_ac4__blk806_dn10, locals.var_ac4__blk806_dn11, locals.var_ac4__blk806_dn12, locals.var_ac4__blk806_dn17,)
    }
};
        locals.var_ac4__blk806 = assign25300_e34333;
        locals.var_ac4__blk806_dn0 = assign25300_e34333_d_n0;
        locals.var_ac4__blk806_dn2 = assign25300_e34333_d_n2;
        locals.var_ac4__blk806_dn6 = assign25300_e34333_d_n6;
        locals.var_ac4__blk806_dn7 = assign25300_e34333_d_n7;
        locals.var_ac4__blk806_dn10 = assign25300_e34333_d_n10;
        locals.var_ac4__blk806_dn11 = assign25300_e34333_d_n11;
        locals.var_ac4__blk806_dn12 = assign25300_e34333_d_n12;
        locals.var_ac4__blk806_dn17 = assign25300_e34333_d_n17;

        let (assign25310_e34345, assign25310_e34345_d_n0, assign25310_e34345_d_n2, assign25310_e34345_d_n6, assign25310_e34345_d_n7, assign25310_e34345_d_n10, assign25310_e34345_d_n11, assign25310_e34345_d_n12, assign25310_e34345_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25310_e34343: f64 = (locals.var_eg - locals.var_pb2over);
        (assign25310_e34343, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk807, locals.var_ps0_min__blk807_dn0, locals.var_ps0_min__blk807_dn2, locals.var_ps0_min__blk807_dn6, locals.var_ps0_min__blk807_dn7, locals.var_ps0_min__blk807_dn10, locals.var_ps0_min__blk807_dn11, locals.var_ps0_min__blk807_dn12, locals.var_ps0_min__blk807_dn17,)
    }
};
        locals.var_ps0_min__blk807 = assign25310_e34345;
        locals.var_ps0_min__blk807_dn0 = assign25310_e34345_d_n0;
        locals.var_ps0_min__blk807_dn2 = assign25310_e34345_d_n2;
        locals.var_ps0_min__blk807_dn6 = assign25310_e34345_d_n6;
        locals.var_ps0_min__blk807_dn7 = assign25310_e34345_d_n7;
        locals.var_ps0_min__blk807_dn10 = assign25310_e34345_d_n10;
        locals.var_ps0_min__blk807_dn11 = assign25310_e34345_d_n11;
        locals.var_ps0_min__blk807_dn12 = assign25310_e34345_d_n12;
        locals.var_ps0_min__blk807_dn17 = assign25310_e34345_d_n17;

        let (assign25320_e34359, assign25320_e34359_d_n0, assign25320_e34359_d_n2, assign25320_e34359_d_n6, assign25320_e34359_d_n7, assign25320_e34359_d_n10, assign25320_e34359_d_n11, assign25320_e34359_d_n12, assign25320_e34359_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25320_e34356: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25320_e34357: f64 = (locals.var_beta * assign25320_e34356);
        (assign25320_e34357, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25320_e34356) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25320_e34359;
        locals.var_tx__blk777_dn0 = assign25320_e34359_d_n0;
        locals.var_tx__blk777_dn2 = assign25320_e34359_d_n2;
        locals.var_tx__blk777_dn6 = assign25320_e34359_d_n6;
        locals.var_tx__blk777_dn7 = assign25320_e34359_d_n7;
        locals.var_tx__blk777_dn10 = assign25320_e34359_d_n10;
        locals.var_tx__blk777_dn11 = assign25320_e34359_d_n11;
        locals.var_tx__blk777_dn12 = assign25320_e34359_d_n12;
        locals.var_tx__blk777_dn17 = assign25320_e34359_d_n17;

        let (assign25330_e34379, assign25330_e34379_d_n0, assign25330_e34379_d_n2, assign25330_e34379_d_n6, assign25330_e34379_d_n7, assign25330_e34379_d_n10, assign25330_e34379_d_n11, assign25330_e34379_d_n12, assign25330_e34379_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25330_e34369: f64 = (7.0 * 1.414213562373095);
        let assign25330_e34372: f64 = (9.0 * locals.var_ty__blk778);
        let assign25330_e34375: f64 = (locals.var_tx__blk777 - 2.0);
        let assign25330_e34376: f64 = (assign25330_e34372 * assign25330_e34375);
        let assign25330_e34377: f64 = (assign25330_e34369 - assign25330_e34376);
        (assign25330_e34377, (-(((9.0 * locals.var_ty__blk778_dn0) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn0))), (-(((9.0 * locals.var_ty__blk778_dn2) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn2))), (-(((9.0 * locals.var_ty__blk778_dn6) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn6))), (-(((9.0 * locals.var_ty__blk778_dn7) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn7))), (-(((9.0 * locals.var_ty__blk778_dn10) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn10))), (-(((9.0 * locals.var_ty__blk778_dn11) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn11))), (-(((9.0 * locals.var_ty__blk778_dn12) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn12))), (-(((9.0 * locals.var_ty__blk778_dn17) * assign25330_e34375) + (assign25330_e34372 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac31__blk808, locals.var_ac31__blk808_dn0, locals.var_ac31__blk808_dn2, locals.var_ac31__blk808_dn6, locals.var_ac31__blk808_dn7, locals.var_ac31__blk808_dn10, locals.var_ac31__blk808_dn11, locals.var_ac31__blk808_dn12, locals.var_ac31__blk808_dn17,)
    }
};
        locals.var_ac31__blk808 = assign25330_e34379;
        locals.var_ac31__blk808_dn0 = assign25330_e34379_d_n0;
        locals.var_ac31__blk808_dn2 = assign25330_e34379_d_n2;
        locals.var_ac31__blk808_dn6 = assign25330_e34379_d_n6;
        locals.var_ac31__blk808_dn7 = assign25330_e34379_d_n7;
        locals.var_ac31__blk808_dn10 = assign25330_e34379_d_n10;
        locals.var_ac31__blk808_dn11 = assign25330_e34379_d_n11;
        locals.var_ac31__blk808_dn12 = assign25330_e34379_d_n12;
        locals.var_ac31__blk808_dn17 = assign25330_e34379_d_n17;

        let (assign25340_e34391, assign25340_e34391_d_n0, assign25340_e34391_d_n2, assign25340_e34391_d_n6, assign25340_e34391_d_n7, assign25340_e34391_d_n10, assign25340_e34391_d_n11, assign25340_e34391_d_n12, assign25340_e34391_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25340_e34389: f64 = (locals.var_ac31__blk808 * locals.var_ac31__blk808);
        (assign25340_e34389, ((locals.var_ac31__blk808_dn0 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn0)), ((locals.var_ac31__blk808_dn2 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn2)), ((locals.var_ac31__blk808_dn6 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn6)), ((locals.var_ac31__blk808_dn7 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn7)), ((locals.var_ac31__blk808_dn10 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn10)), ((locals.var_ac31__blk808_dn11 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn11)), ((locals.var_ac31__blk808_dn12 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn12)), ((locals.var_ac31__blk808_dn17 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn17)),)
    } else {
        (locals.var_ac3__blk809, locals.var_ac3__blk809_dn0, locals.var_ac3__blk809_dn2, locals.var_ac3__blk809_dn6, locals.var_ac3__blk809_dn7, locals.var_ac3__blk809_dn10, locals.var_ac3__blk809_dn11, locals.var_ac3__blk809_dn12, locals.var_ac3__blk809_dn17,)
    }
};
        locals.var_ac3__blk809 = assign25340_e34391;
        locals.var_ac3__blk809_dn0 = assign25340_e34391_d_n0;
        locals.var_ac3__blk809_dn2 = assign25340_e34391_d_n2;
        locals.var_ac3__blk809_dn6 = assign25340_e34391_d_n6;
        locals.var_ac3__blk809_dn7 = assign25340_e34391_d_n7;
        locals.var_ac3__blk809_dn10 = assign25340_e34391_d_n10;
        locals.var_ac3__blk809_dn11 = assign25340_e34391_d_n11;
        locals.var_ac3__blk809_dn12 = assign25340_e34391_d_n12;
        locals.var_ac3__blk809_dn17 = assign25340_e34391_d_n17;

        let assign25350_e34395: f64 = (locals.var_ac3__blk809 * 1e-8);
        let assign25350_e34396: f64 = if locals.var_ac4__blk806 < assign25350_e34395 { 1.0 } else { 0.0 };
        locals.var_guard854 = assign25350_e34396;

        let (assign25360_e34427, assign25360_e34427_d_n0, assign25360_e34427_d_n2, assign25360_e34427_d_n6, assign25360_e34427_d_n7, assign25360_e34427_d_n10, assign25360_e34427_d_n11, assign25360_e34427_d_n12, assign25360_e34427_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25360_e34407: f64 = (-7.0);
        let assign25360_e34409: f64 = (assign25360_e34407 * 1.414213562373095);
        let assign25360_e34411: f64 = (assign25360_e34409 + locals.var_ac31__blk808);
        let assign25360_e34414: f64 = (0.5 * locals.var_ac4__blk806);
        let assign25360_e34416: f64 = (assign25360_e34414 / locals.var_ac31__blk808);
        let assign25360_e34417: f64 = (assign25360_e34411 + assign25360_e34416);
        let assign25360_e34420: f64 = (9.0 * locals.var_ty__blk778);
        let assign25360_e34423: f64 = (locals.var_tx__blk777 - 2.0);
        let assign25360_e34424: f64 = (assign25360_e34420 * assign25360_e34423);
        let assign25360_e34425: f64 = (assign25360_e34417 + assign25360_e34424);
        (assign25360_e34425, ((locals.var_ac31__blk808_dn0 + ((((0.5 * locals.var_ac4__blk806_dn0) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn0)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn0) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn0))), ((locals.var_ac31__blk808_dn2 + ((((0.5 * locals.var_ac4__blk806_dn2) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn2)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn2) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn2))), ((locals.var_ac31__blk808_dn6 + ((((0.5 * locals.var_ac4__blk806_dn6) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn6)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn6) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn6))), ((locals.var_ac31__blk808_dn7 + ((((0.5 * locals.var_ac4__blk806_dn7) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn7)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn7) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn7))), ((locals.var_ac31__blk808_dn10 + ((((0.5 * locals.var_ac4__blk806_dn10) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn10)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn10) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn10))), ((locals.var_ac31__blk808_dn11 + ((((0.5 * locals.var_ac4__blk806_dn11) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn11)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn11) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn11))), ((locals.var_ac31__blk808_dn12 + ((((0.5 * locals.var_ac4__blk806_dn12) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn12)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn12) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn12))), ((locals.var_ac31__blk808_dn17 + ((((0.5 * locals.var_ac4__blk806_dn17) * locals.var_ac31__blk808) - (assign25360_e34414 * locals.var_ac31__blk808_dn17)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn17) * assign25360_e34423) + (assign25360_e34420 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign25360_e34427;
        locals.var_ac1__blk811_dn0 = assign25360_e34427_d_n0;
        locals.var_ac1__blk811_dn2 = assign25360_e34427_d_n2;
        locals.var_ac1__blk811_dn6 = assign25360_e34427_d_n6;
        locals.var_ac1__blk811_dn7 = assign25360_e34427_d_n7;
        locals.var_ac1__blk811_dn10 = assign25360_e34427_d_n10;
        locals.var_ac1__blk811_dn11 = assign25360_e34427_d_n11;
        locals.var_ac1__blk811_dn12 = assign25360_e34427_d_n12;
        locals.var_ac1__blk811_dn17 = assign25360_e34427_d_n17;

        let (assign25370_e34443, assign25370_e34443_d_n0, assign25370_e34443_d_n2, assign25370_e34443_d_n6, assign25370_e34443_d_n7, assign25370_e34443_d_n10, assign25370_e34443_d_n11, assign25370_e34443_d_n12, assign25370_e34443_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 == 0.0)) {
        let assign25370_e34440: f64 = (locals.var_ac4__blk806 + locals.var_ac3__blk809);
        let assign25370_e34441: f64 = (assign25370_e34440).sqrt();
        (assign25370_e34441, ((locals.var_ac4__blk806_dn0 + locals.var_ac3__blk809_dn0) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn2 + locals.var_ac3__blk809_dn2) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn6 + locals.var_ac3__blk809_dn6) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn7 + locals.var_ac3__blk809_dn7) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn10 + locals.var_ac3__blk809_dn10) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn11 + locals.var_ac3__blk809_dn11) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn12 + locals.var_ac3__blk809_dn12) / (2.0 * assign25370_e34441)), ((locals.var_ac4__blk806_dn17 + locals.var_ac3__blk809_dn17) / (2.0 * assign25370_e34441)),)
    } else {
        (locals.var_ac2__blk810, locals.var_ac2__blk810_dn0, locals.var_ac2__blk810_dn2, locals.var_ac2__blk810_dn6, locals.var_ac2__blk810_dn7, locals.var_ac2__blk810_dn10, locals.var_ac2__blk810_dn11, locals.var_ac2__blk810_dn12, locals.var_ac2__blk810_dn17,)
    }
};
        locals.var_ac2__blk810 = assign25370_e34443;
        locals.var_ac2__blk810_dn0 = assign25370_e34443_d_n0;
        locals.var_ac2__blk810_dn2 = assign25370_e34443_d_n2;
        locals.var_ac2__blk810_dn6 = assign25370_e34443_d_n6;
        locals.var_ac2__blk810_dn7 = assign25370_e34443_d_n7;
        locals.var_ac2__blk810_dn10 = assign25370_e34443_d_n10;
        locals.var_ac2__blk810_dn11 = assign25370_e34443_d_n11;
        locals.var_ac2__blk810_dn12 = assign25370_e34443_d_n12;
        locals.var_ac2__blk810_dn17 = assign25370_e34443_d_n17;

        let (assign25380_e34469, assign25380_e34469_d_n0, assign25380_e34469_d_n2, assign25380_e34469_d_n6, assign25380_e34469_d_n7, assign25380_e34469_d_n10, assign25380_e34469_d_n11, assign25380_e34469_d_n12, assign25380_e34469_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 == 0.0)) {
        let assign25380_e34455: f64 = (-7.0);
        let assign25380_e34457: f64 = (assign25380_e34455 * 1.414213562373095);
        let assign25380_e34459: f64 = (assign25380_e34457 + locals.var_ac2__blk810);
        let assign25380_e34462: f64 = (9.0 * locals.var_ty__blk778);
        let assign25380_e34465: f64 = (locals.var_tx__blk777 - 2.0);
        let assign25380_e34466: f64 = (assign25380_e34462 * assign25380_e34465);
        let assign25380_e34467: f64 = (assign25380_e34459 + assign25380_e34466);
        (assign25380_e34467, (locals.var_ac2__blk810_dn0 + (((9.0 * locals.var_ty__blk778_dn0) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn0))), (locals.var_ac2__blk810_dn2 + (((9.0 * locals.var_ty__blk778_dn2) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn2))), (locals.var_ac2__blk810_dn6 + (((9.0 * locals.var_ty__blk778_dn6) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn6))), (locals.var_ac2__blk810_dn7 + (((9.0 * locals.var_ty__blk778_dn7) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn7))), (locals.var_ac2__blk810_dn10 + (((9.0 * locals.var_ty__blk778_dn10) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn10))), (locals.var_ac2__blk810_dn11 + (((9.0 * locals.var_ty__blk778_dn11) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn11))), (locals.var_ac2__blk810_dn12 + (((9.0 * locals.var_ty__blk778_dn12) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn12))), (locals.var_ac2__blk810_dn17 + (((9.0 * locals.var_ty__blk778_dn17) * assign25380_e34465) + (assign25380_e34462 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign25380_e34469;
        locals.var_ac1__blk811_dn0 = assign25380_e34469_d_n0;
        locals.var_ac1__blk811_dn2 = assign25380_e34469_d_n2;
        locals.var_ac1__blk811_dn6 = assign25380_e34469_d_n6;
        locals.var_ac1__blk811_dn7 = assign25380_e34469_d_n7;
        locals.var_ac1__blk811_dn10 = assign25380_e34469_d_n10;
        locals.var_ac1__blk811_dn11 = assign25380_e34469_d_n11;
        locals.var_ac1__blk811_dn12 = assign25380_e34469_d_n12;
        locals.var_ac1__blk811_dn17 = assign25380_e34469_d_n17;

        let (assign25390_e34481, assign25390_e34481_d_n0, assign25390_e34481_d_n2, assign25390_e34481_d_n6, assign25390_e34481_d_n7, assign25390_e34481_d_n10, assign25390_e34481_d_n11, assign25390_e34481_d_n12, assign25390_e34481_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25390_e34479: f64 = (locals.var_ac1__blk811).powf(0.3333333333333333);
        (assign25390_e34479, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn0)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn0 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn2)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn2 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn6)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn6 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn7)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn7 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn10)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn10 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn11)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn11 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn12)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn12 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn17)) } } else { (assign25390_e34479 * (0.3333333333333333 * (locals.var_ac1__blk811_dn17 / locals.var_ac1__blk811))) },)
    } else {
        (locals.var_acd__blk812, locals.var_acd__blk812_dn0, locals.var_acd__blk812_dn2, locals.var_acd__blk812_dn6, locals.var_acd__blk812_dn7, locals.var_acd__blk812_dn10, locals.var_acd__blk812_dn11, locals.var_acd__blk812_dn12, locals.var_acd__blk812_dn17,)
    }
};
        locals.var_acd__blk812 = assign25390_e34481;
        locals.var_acd__blk812_dn0 = assign25390_e34481_d_n0;
        locals.var_acd__blk812_dn2 = assign25390_e34481_d_n2;
        locals.var_acd__blk812_dn6 = assign25390_e34481_d_n6;
        locals.var_acd__blk812_dn7 = assign25390_e34481_d_n7;
        locals.var_acd__blk812_dn10 = assign25390_e34481_d_n10;
        locals.var_acd__blk812_dn11 = assign25390_e34481_d_n11;
        locals.var_acd__blk812_dn12 = assign25390_e34481_d_n12;
        locals.var_acd__blk812_dn17 = assign25390_e34481_d_n17;

        let (assign25400_e34508, assign25400_e34508_d_n0, assign25400_e34508_d_n2, assign25400_e34508_d_n6, assign25400_e34508_d_n7, assign25400_e34508_d_n10, assign25400_e34508_d_n11, assign25400_e34508_d_n12, assign25400_e34508_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25400_e34490: f64 = (-4.0);
        let assign25400_e34492: f64 = (assign25400_e34490 * 1.414213562373095);
        let assign25400_e34495: f64 = (12.0 * locals.var_ty__blk778);
        let assign25400_e34496: f64 = (assign25400_e34492 - assign25400_e34495);
        let assign25400_e34499: f64 = (2.0 * locals.var_acd__blk812);
        let assign25400_e34500: f64 = (assign25400_e34496 + assign25400_e34499);
        let assign25400_e34503: f64 = (1.414213562373095 * locals.var_acd__blk812);
        let assign25400_e34505: f64 = (assign25400_e34503 * locals.var_acd__blk812);
        let assign25400_e34506: f64 = (assign25400_e34500 + assign25400_e34505);
        (assign25400_e34506, (((-(12.0 * locals.var_ty__blk778_dn0)) + (2.0 * locals.var_acd__blk812_dn0)) + (((1.414213562373095 * locals.var_acd__blk812_dn0) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn0))), (((-(12.0 * locals.var_ty__blk778_dn2)) + (2.0 * locals.var_acd__blk812_dn2)) + (((1.414213562373095 * locals.var_acd__blk812_dn2) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn2))), (((-(12.0 * locals.var_ty__blk778_dn6)) + (2.0 * locals.var_acd__blk812_dn6)) + (((1.414213562373095 * locals.var_acd__blk812_dn6) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn6))), (((-(12.0 * locals.var_ty__blk778_dn7)) + (2.0 * locals.var_acd__blk812_dn7)) + (((1.414213562373095 * locals.var_acd__blk812_dn7) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn7))), (((-(12.0 * locals.var_ty__blk778_dn10)) + (2.0 * locals.var_acd__blk812_dn10)) + (((1.414213562373095 * locals.var_acd__blk812_dn10) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn10))), (((-(12.0 * locals.var_ty__blk778_dn11)) + (2.0 * locals.var_acd__blk812_dn11)) + (((1.414213562373095 * locals.var_acd__blk812_dn11) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn11))), (((-(12.0 * locals.var_ty__blk778_dn12)) + (2.0 * locals.var_acd__blk812_dn12)) + (((1.414213562373095 * locals.var_acd__blk812_dn12) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn12))), (((-(12.0 * locals.var_ty__blk778_dn17)) + (2.0 * locals.var_acd__blk812_dn17)) + (((1.414213562373095 * locals.var_acd__blk812_dn17) * locals.var_acd__blk812) + (assign25400_e34503 * locals.var_acd__blk812_dn17))),)
    } else {
        (locals.var_acn__blk813, locals.var_acn__blk813_dn0, locals.var_acn__blk813_dn2, locals.var_acn__blk813_dn6, locals.var_acn__blk813_dn7, locals.var_acn__blk813_dn10, locals.var_acn__blk813_dn11, locals.var_acn__blk813_dn12, locals.var_acn__blk813_dn17,)
    }
};
        locals.var_acn__blk813 = assign25400_e34508;
        locals.var_acn__blk813_dn0 = assign25400_e34508_d_n0;
        locals.var_acn__blk813_dn2 = assign25400_e34508_d_n2;
        locals.var_acn__blk813_dn6 = assign25400_e34508_d_n6;
        locals.var_acn__blk813_dn7 = assign25400_e34508_d_n7;
        locals.var_acn__blk813_dn10 = assign25400_e34508_d_n10;
        locals.var_acn__blk813_dn11 = assign25400_e34508_d_n11;
        locals.var_acn__blk813_dn12 = assign25400_e34508_d_n12;
        locals.var_acn__blk813_dn17 = assign25400_e34508_d_n17;

        let (assign25410_e34520, assign25410_e34520_d_n0, assign25410_e34520_d_n2, assign25410_e34520_d_n6, assign25410_e34520_d_n7, assign25410_e34520_d_n10, assign25410_e34520_d_n11, assign25410_e34520_d_n12, assign25410_e34520_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25410_e34518: f64 = (locals.var_acn__blk813 / locals.var_acd__blk812);
        (assign25410_e34518, (((locals.var_acn__blk813_dn0 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn0)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn2 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn2)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn6 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn6)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn7 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn7)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn10 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn10)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn11 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn11)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn12 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn12)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn17 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn17)) / (locals.var_acd__blk812 * locals.var_acd__blk812)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign25410_e34520;
        locals.var_chi__blk814_dn0 = assign25410_e34520_d_n0;
        locals.var_chi__blk814_dn2 = assign25410_e34520_d_n2;
        locals.var_chi__blk814_dn6 = assign25410_e34520_d_n6;
        locals.var_chi__blk814_dn7 = assign25410_e34520_d_n7;
        locals.var_chi__blk814_dn10 = assign25410_e34520_d_n10;
        locals.var_chi__blk814_dn11 = assign25410_e34520_d_n11;
        locals.var_chi__blk814_dn12 = assign25410_e34520_d_n12;
        locals.var_chi__blk814_dn17 = assign25410_e34520_d_n17;

        let (assign25420_e34534, assign25420_e34534_d_n0, assign25420_e34534_d_n2, assign25420_e34534_d_n6, assign25420_e34534_d_n7, assign25420_e34534_d_n10, assign25420_e34534_d_n11, assign25420_e34534_d_n12, assign25420_e34534_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25420_e34530: f64 = (locals.var_chi__blk814 * locals.var_beta_inv);
        let assign25420_e34532: f64 = (assign25420_e34530 - locals.var_vxbgmtcl);
        (assign25420_e34532, ((locals.var_chi__blk814_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk814_dn10 * locals.var_beta_inv) + (locals.var_chi__blk814 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk815, locals.var_psa__blk815_dn0, locals.var_psa__blk815_dn2, locals.var_psa__blk815_dn6, locals.var_psa__blk815_dn7, locals.var_psa__blk815_dn10, locals.var_psa__blk815_dn11, locals.var_psa__blk815_dn12, locals.var_psa__blk815_dn17,)
    }
};
        locals.var_psa__blk815 = assign25420_e34534;
        locals.var_psa__blk815_dn0 = assign25420_e34534_d_n0;
        locals.var_psa__blk815_dn2 = assign25420_e34534_d_n2;
        locals.var_psa__blk815_dn6 = assign25420_e34534_d_n6;
        locals.var_psa__blk815_dn7 = assign25420_e34534_d_n7;
        locals.var_psa__blk815_dn10 = assign25420_e34534_d_n10;
        locals.var_psa__blk815_dn11 = assign25420_e34534_d_n11;
        locals.var_psa__blk815_dn12 = assign25420_e34534_d_n12;
        locals.var_psa__blk815_dn17 = assign25420_e34534_d_n17;

        let (assign25430_e34546, assign25430_e34546_d_n0, assign25430_e34546_d_n2, assign25430_e34546_d_n6, assign25430_e34546_d_n7, assign25430_e34546_d_n10, assign25430_e34546_d_n11, assign25430_e34546_d_n12, assign25430_e34546_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25430_e34544: f64 = (locals.var_psa__blk815 + locals.var_vxbgmtcl);
        (assign25430_e34544, (locals.var_psa__blk815_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk815_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk815_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk815_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk815_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk815_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk815_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk815_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign25430_e34546;
        locals.var_t1__blk771_dn0 = assign25430_e34546_d_n0;
        locals.var_t1__blk771_dn2 = assign25430_e34546_d_n2;
        locals.var_t1__blk771_dn6 = assign25430_e34546_d_n6;
        locals.var_t1__blk771_dn7 = assign25430_e34546_d_n7;
        locals.var_t1__blk771_dn10 = assign25430_e34546_d_n10;
        locals.var_t1__blk771_dn11 = assign25430_e34546_d_n11;
        locals.var_t1__blk771_dn12 = assign25430_e34546_d_n12;
        locals.var_t1__blk771_dn17 = assign25430_e34546_d_n17;

        let (assign25440_e34558, assign25440_e34558_d_n0, assign25440_e34558_d_n2, assign25440_e34558_d_n6, assign25440_e34558_d_n7, assign25440_e34558_d_n10, assign25440_e34558_d_n11, assign25440_e34558_d_n12, assign25440_e34558_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25440_e34556: f64 = (locals.var_t1__blk771 / locals.var_ps0_min__blk807);
        (assign25440_e34556, (((locals.var_t1__blk771_dn0 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn0)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn2 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn2)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn6 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn6)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn7 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn7)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn10 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn10)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn11 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn11)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn12 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn12)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn17 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn17)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign25440_e34558;
        locals.var_t2__blk772_dn0 = assign25440_e34558_d_n0;
        locals.var_t2__blk772_dn2 = assign25440_e34558_d_n2;
        locals.var_t2__blk772_dn6 = assign25440_e34558_d_n6;
        locals.var_t2__blk772_dn7 = assign25440_e34558_d_n7;
        locals.var_t2__blk772_dn10 = assign25440_e34558_d_n10;
        locals.var_t2__blk772_dn11 = assign25440_e34558_d_n11;
        locals.var_t2__blk772_dn12 = assign25440_e34558_d_n12;
        locals.var_t2__blk772_dn17 = assign25440_e34558_d_n17;

        let (assign25450_e34573, assign25450_e34573_d_n0, assign25450_e34573_d_n2, assign25450_e34573_d_n6, assign25450_e34573_d_n7, assign25450_e34573_d_n10, assign25450_e34573_d_n11, assign25450_e34573_d_n12, assign25450_e34573_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25450_e34569: f64 = (locals.var_t2__blk772 * locals.var_t2__blk772);
        let assign25450_e34570: f64 = (1.0 + assign25450_e34569);
        let assign25450_e34571: f64 = (assign25450_e34570).sqrt();
        (assign25450_e34571, (((locals.var_t2__blk772_dn0 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn0)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn2 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn2)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn6 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn6)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn7 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn7)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn10 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn10)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn11 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn11)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn12 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn12)) / (2.0 * assign25450_e34571)), (((locals.var_t2__blk772_dn17 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn17)) / (2.0 * assign25450_e34571)),)
    } else {
        (locals.var_t3__blk773, locals.var_t3__blk773_dn0, locals.var_t3__blk773_dn2, locals.var_t3__blk773_dn6, locals.var_t3__blk773_dn7, locals.var_t3__blk773_dn10, locals.var_t3__blk773_dn11, locals.var_t3__blk773_dn12, locals.var_t3__blk773_dn17,)
    }
};
        locals.var_t3__blk773 = assign25450_e34573;
        locals.var_t3__blk773_dn0 = assign25450_e34573_d_n0;
        locals.var_t3__blk773_dn2 = assign25450_e34573_d_n2;
        locals.var_t3__blk773_dn6 = assign25450_e34573_d_n6;
        locals.var_t3__blk773_dn7 = assign25450_e34573_d_n7;
        locals.var_t3__blk773_dn10 = assign25450_e34573_d_n10;
        locals.var_t3__blk773_dn11 = assign25450_e34573_d_n11;
        locals.var_t3__blk773_dn12 = assign25450_e34573_d_n12;
        locals.var_t3__blk773_dn17 = assign25450_e34573_d_n17;

        let (assign25460_e34587, assign25460_e34587_d_n0, assign25460_e34587_d_n2, assign25460_e34587_d_n6, assign25460_e34587_d_n7, assign25460_e34587_d_n10, assign25460_e34587_d_n11, assign25460_e34587_d_n12, assign25460_e34587_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25460_e34583: f64 = (locals.var_t1__blk771 / locals.var_t3__blk773);
        let assign25460_e34585: f64 = (assign25460_e34583 - locals.var_vxbgmtcl);
        (assign25460_e34585, ((((locals.var_t1__blk771_dn0 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn0)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk771_dn2 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn2)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk771_dn6 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn6)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk771_dn7 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn7)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk771_dn10 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn10)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk771_dn11 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn11)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk771_dn12 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn12)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk771_dn17 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn17)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign25460_e34587;
        locals.var_ps0ld_dn0 = assign25460_e34587_d_n0;
        locals.var_ps0ld_dn2 = assign25460_e34587_d_n2;
        locals.var_ps0ld_dn6 = assign25460_e34587_d_n6;
        locals.var_ps0ld_dn7 = assign25460_e34587_d_n7;
        locals.var_ps0ld_dn10 = assign25460_e34587_d_n10;
        locals.var_ps0ld_dn11 = assign25460_e34587_d_n11;
        locals.var_ps0ld_dn12 = assign25460_e34587_d_n12;
        locals.var_ps0ld_dn17 = assign25460_e34587_d_n17;

        let (assign25470_e34599, assign25470_e34599_d_n0, assign25470_e34599_d_n2, assign25470_e34599_d_n6, assign25470_e34599_d_n7, assign25470_e34599_d_n10, assign25470_e34599_d_n11, assign25470_e34599_d_n12, assign25470_e34599_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25470_e34597: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign25470_e34597, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign25470_e34599;
        locals.var_t2__blk772_dn0 = assign25470_e34599_d_n0;
        locals.var_t2__blk772_dn2 = assign25470_e34599_d_n2;
        locals.var_t2__blk772_dn6 = assign25470_e34599_d_n6;
        locals.var_t2__blk772_dn7 = assign25470_e34599_d_n7;
        locals.var_t2__blk772_dn10 = assign25470_e34599_d_n10;
        locals.var_t2__blk772_dn11 = assign25470_e34599_d_n11;
        locals.var_t2__blk772_dn12 = assign25470_e34599_d_n12;
        locals.var_t2__blk772_dn17 = assign25470_e34599_d_n17;

    }

    pub(super) fn stamp_transient_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25480_e34611, assign25480_e34611_d_n0, assign25480_e34611_d_n2, assign25480_e34611_d_n6, assign25480_e34611_d_n7, assign25480_e34611_d_n10, assign25480_e34611_d_n11, assign25480_e34611_d_n12, assign25480_e34611_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25480_e34609: f64 = (locals.var_cox0 * locals.var_t2__blk772);
        (assign25480_e34609, (locals.var_cox0 * locals.var_t2__blk772_dn0), (locals.var_cox0 * locals.var_t2__blk772_dn2), (locals.var_cox0 * locals.var_t2__blk772_dn6), (locals.var_cox0 * locals.var_t2__blk772_dn7), (locals.var_cox0 * locals.var_t2__blk772_dn10), (locals.var_cox0 * locals.var_t2__blk772_dn11), (locals.var_cox0 * locals.var_t2__blk772_dn12), (locals.var_cox0 * locals.var_t2__blk772_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign25480_e34611;
        locals.var_qsuld_dn0 = assign25480_e34611_d_n0;
        locals.var_qsuld_dn2 = assign25480_e34611_d_n2;
        locals.var_qsuld_dn6 = assign25480_e34611_d_n6;
        locals.var_qsuld_dn7 = assign25480_e34611_d_n7;
        locals.var_qsuld_dn10 = assign25480_e34611_d_n10;
        locals.var_qsuld_dn11 = assign25480_e34611_d_n11;
        locals.var_qsuld_dn12 = assign25480_e34611_d_n12;
        locals.var_qsuld_dn17 = assign25480_e34611_d_n17;

        let (assign25490_e34621, assign25490_e34621_d_n0, assign25490_e34621_d_n2, assign25490_e34621_d_n6, assign25490_e34621_d_n7, assign25490_e34621_d_n10, assign25490_e34621_d_n11, assign25490_e34621_d_n12, assign25490_e34621_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign25490_e34621;
        locals.var_qbuld_dn0 = assign25490_e34621_d_n0;
        locals.var_qbuld_dn2 = assign25490_e34621_d_n2;
        locals.var_qbuld_dn6 = assign25490_e34621_d_n6;
        locals.var_qbuld_dn7 = assign25490_e34621_d_n7;
        locals.var_qbuld_dn10 = assign25490_e34621_d_n10;
        locals.var_qbuld_dn11 = assign25490_e34621_d_n11;
        locals.var_qbuld_dn12 = assign25490_e34621_d_n12;
        locals.var_qbuld_dn17 = assign25490_e34621_d_n17;

        let (assign25510_e34643, assign25510_e34643_d_n0, assign25510_e34643_d_n2, assign25510_e34643_d_n6, assign25510_e34643_d_n7, assign25510_e34643_d_n10, assign25510_e34643_d_n11, assign25510_e34643_d_n12, assign25510_e34643_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign25510_e34643;
        locals.var_chi__blk814_dn0 = assign25510_e34643_d_n0;
        locals.var_chi__blk814_dn2 = assign25510_e34643_d_n2;
        locals.var_chi__blk814_dn6 = assign25510_e34643_d_n6;
        locals.var_chi__blk814_dn7 = assign25510_e34643_d_n7;
        locals.var_chi__blk814_dn10 = assign25510_e34643_d_n10;
        locals.var_chi__blk814_dn11 = assign25510_e34643_d_n11;
        locals.var_chi__blk814_dn12 = assign25510_e34643_d_n12;
        locals.var_chi__blk814_dn17 = assign25510_e34643_d_n17;

        let (assign25520_e34658, assign25520_e34658_d_n0, assign25520_e34658_d_n2, assign25520_e34658_d_n6, assign25520_e34658_d_n7, assign25520_e34658_d_n10, assign25520_e34658_d_n11, assign25520_e34658_d_n12, assign25520_e34658_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25520_e34654: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign25520_e34656: f64 = (assign25520_e34654 - locals.var_vxbgmtcl);
        (assign25520_e34656, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign25520_e34658;
        locals.var_ps0_inia__blk817_dn0 = assign25520_e34658_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign25520_e34658_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign25520_e34658_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign25520_e34658_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign25520_e34658_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign25520_e34658_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign25520_e34658_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign25520_e34658_d_n17;

        let (assign25530_e34671, assign25530_e34671_d_n0, assign25530_e34671_d_n2, assign25530_e34671_d_n6, assign25530_e34671_d_n7, assign25530_e34671_d_n10, assign25530_e34671_d_n11, assign25530_e34671_d_n12, assign25530_e34671_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25530_e34668: f64 = (-locals.var_chi__blk814);
        let assign25530_e34669: f64 = (assign25530_e34668).exp();
        (assign25530_e34669, (assign25530_e34669 * (-locals.var_chi__blk814_dn0)), (assign25530_e34669 * (-locals.var_chi__blk814_dn2)), (assign25530_e34669 * (-locals.var_chi__blk814_dn6)), (assign25530_e34669 * (-locals.var_chi__blk814_dn7)), (assign25530_e34669 * (-locals.var_chi__blk814_dn10)), (assign25530_e34669 * (-locals.var_chi__blk814_dn11)), (assign25530_e34669 * (-locals.var_chi__blk814_dn12)), (assign25530_e34669 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign25530_e34671;
        locals.var_ty__blk778_dn0 = assign25530_e34671_d_n0;
        locals.var_ty__blk778_dn2 = assign25530_e34671_d_n2;
        locals.var_ty__blk778_dn6 = assign25530_e34671_d_n6;
        locals.var_ty__blk778_dn7 = assign25530_e34671_d_n7;
        locals.var_ty__blk778_dn10 = assign25530_e34671_d_n10;
        locals.var_ty__blk778_dn11 = assign25530_e34671_d_n11;
        locals.var_ty__blk778_dn12 = assign25530_e34671_d_n12;
        locals.var_ty__blk778_dn17 = assign25530_e34671_d_n17;

        let (assign25540_e34698, assign25540_e34698_d_n0, assign25540_e34698_d_n2, assign25540_e34698_d_n6, assign25540_e34698_d_n7, assign25540_e34698_d_n10, assign25540_e34698_d_n11, assign25540_e34698_d_n12, assign25540_e34698_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25540_e34685: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25540_e34686: f64 = (locals.var_beta * assign25540_e34685);
        let assign25540_e34688: f64 = (assign25540_e34686 - 1.0);
        let assign25540_e34690: f64 = (assign25540_e34688 + locals.var_ty__blk778);
        let assign25540_e34691: f64 = (4.0 * assign25540_e34690);
        let assign25540_e34694: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign25540_e34695: f64 = (assign25540_e34691 / assign25540_e34694);
        let assign25540_e34696: f64 = (1.0 + assign25540_e34695);
        (assign25540_e34696, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * (((locals.var_beta_dn10 * assign25540_e34685) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign25540_e34694) - (assign25540_e34691 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign25540_e34694) - (assign25540_e34691 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign25540_e34694 * assign25540_e34694)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25540_e34698;
        locals.var_tx__blk777_dn0 = assign25540_e34698_d_n0;
        locals.var_tx__blk777_dn2 = assign25540_e34698_d_n2;
        locals.var_tx__blk777_dn6 = assign25540_e34698_d_n6;
        locals.var_tx__blk777_dn7 = assign25540_e34698_d_n7;
        locals.var_tx__blk777_dn10 = assign25540_e34698_d_n10;
        locals.var_tx__blk777_dn11 = assign25540_e34698_d_n11;
        locals.var_tx__blk777_dn12 = assign25540_e34698_d_n12;
        locals.var_tx__blk777_dn17 = assign25540_e34698_d_n17;

        let assign25550_e34702: f64 = (10.0 * 2.220446049250313e-16);
        let assign25550_e34703: f64 = if locals.var_tx__blk777 < assign25550_e34702 { 1.0 } else { 0.0 };
        locals.var_guard855 = assign25550_e34703;

        let (assign25560_e34718, assign25560_e34718_d_n0, assign25560_e34718_d_n2, assign25560_e34718_d_n6, assign25560_e34718_d_n7, assign25560_e34718_d_n10, assign25560_e34718_d_n11, assign25560_e34718_d_n12, assign25560_e34718_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25560_e34716: f64 = (10.0 * 2.220446049250313e-16);
        (assign25560_e34716, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25560_e34718;
        locals.var_tx__blk777_dn0 = assign25560_e34718_d_n0;
        locals.var_tx__blk777_dn2 = assign25560_e34718_d_n2;
        locals.var_tx__blk777_dn6 = assign25560_e34718_d_n6;
        locals.var_tx__blk777_dn7 = assign25560_e34718_d_n7;
        locals.var_tx__blk777_dn10 = assign25560_e34718_d_n10;
        locals.var_tx__blk777_dn11 = assign25560_e34718_d_n11;
        locals.var_tx__blk777_dn12 = assign25560_e34718_d_n12;
        locals.var_tx__blk777_dn17 = assign25560_e34718_d_n17;

        let (assign25570_e34740, assign25570_e34740_d_n0, assign25570_e34740_d_n2, assign25570_e34740_d_n6, assign25570_e34740_d_n7, assign25570_e34740_d_n10, assign25570_e34740_d_n11, assign25570_e34740_d_n12, assign25570_e34740_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25570_e34730: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign25570_e34732: f64 = (assign25570_e34730 / 2.0);
        let assign25570_e34735: f64 = (locals.var_tx__blk777).sqrt();
        let assign25570_e34736: f64 = (1.0 - assign25570_e34735);
        let assign25570_e34737: f64 = (assign25570_e34732 * assign25570_e34736);
        let assign25570_e34738: f64 = (locals.var_vgpld + assign25570_e34737);
        (assign25570_e34738, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign25570_e34735)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign25570_e34736) + (assign25570_e34732 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign25570_e34735)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign25570_e34740;
        locals.var_ps0_inia__blk817_dn0 = assign25570_e34740_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign25570_e34740_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign25570_e34740_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign25570_e34740_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign25570_e34740_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign25570_e34740_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign25570_e34740_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign25570_e34740_d_n17;

        let (assign25580_e34755, assign25580_e34755_d_n0, assign25580_e34755_d_n2, assign25580_e34755_d_n6, assign25580_e34755_d_n7, assign25580_e34755_d_n10, assign25580_e34755_d_n11, assign25580_e34755_d_n12, assign25580_e34755_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25580_e34752: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign25580_e34753: f64 = (locals.var_beta * assign25580_e34752);
        (assign25580_e34753, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25580_e34752) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign25580_e34755;
        locals.var_chi__blk814_dn0 = assign25580_e34755_d_n0;
        locals.var_chi__blk814_dn2 = assign25580_e34755_d_n2;
        locals.var_chi__blk814_dn6 = assign25580_e34755_d_n6;
        locals.var_chi__blk814_dn7 = assign25580_e34755_d_n7;
        locals.var_chi__blk814_dn10 = assign25580_e34755_d_n10;
        locals.var_chi__blk814_dn11 = assign25580_e34755_d_n11;
        locals.var_chi__blk814_dn12 = assign25580_e34755_d_n12;
        locals.var_chi__blk814_dn17 = assign25580_e34755_d_n17;

        let (assign25590_e34768, assign25590_e34768_d_n0, assign25590_e34768_d_n2, assign25590_e34768_d_n6, assign25590_e34768_d_n7, assign25590_e34768_d_n10, assign25590_e34768_d_n11, assign25590_e34768_d_n12, assign25590_e34768_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25590_e34765: f64 = (-locals.var_chi__blk814);
        let assign25590_e34766: f64 = (assign25590_e34765).exp();
        (assign25590_e34766, (assign25590_e34766 * (-locals.var_chi__blk814_dn0)), (assign25590_e34766 * (-locals.var_chi__blk814_dn2)), (assign25590_e34766 * (-locals.var_chi__blk814_dn6)), (assign25590_e34766 * (-locals.var_chi__blk814_dn7)), (assign25590_e34766 * (-locals.var_chi__blk814_dn10)), (assign25590_e34766 * (-locals.var_chi__blk814_dn11)), (assign25590_e34766 * (-locals.var_chi__blk814_dn12)), (assign25590_e34766 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign25590_e34768;
        locals.var_ty__blk778_dn0 = assign25590_e34768_d_n0;
        locals.var_ty__blk778_dn2 = assign25590_e34768_d_n2;
        locals.var_ty__blk778_dn6 = assign25590_e34768_d_n6;
        locals.var_ty__blk778_dn7 = assign25590_e34768_d_n7;
        locals.var_ty__blk778_dn10 = assign25590_e34768_d_n10;
        locals.var_ty__blk778_dn11 = assign25590_e34768_d_n11;
        locals.var_ty__blk778_dn12 = assign25590_e34768_d_n12;
        locals.var_ty__blk778_dn17 = assign25590_e34768_d_n17;

        let (assign25600_e34795, assign25600_e34795_d_n0, assign25600_e34795_d_n2, assign25600_e34795_d_n6, assign25600_e34795_d_n7, assign25600_e34795_d_n10, assign25600_e34795_d_n11, assign25600_e34795_d_n12, assign25600_e34795_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25600_e34782: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25600_e34783: f64 = (locals.var_beta * assign25600_e34782);
        let assign25600_e34785: f64 = (assign25600_e34783 - 1.0);
        let assign25600_e34787: f64 = (assign25600_e34785 + locals.var_ty__blk778);
        let assign25600_e34788: f64 = (4.0 * assign25600_e34787);
        let assign25600_e34791: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign25600_e34792: f64 = (assign25600_e34788 / assign25600_e34791);
        let assign25600_e34793: f64 = (1.0 + assign25600_e34792);
        (assign25600_e34793, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * (((locals.var_beta_dn10 * assign25600_e34782) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign25600_e34791) - (assign25600_e34788 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign25600_e34791) - (assign25600_e34788 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign25600_e34791 * assign25600_e34791)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25600_e34795;
        locals.var_tx__blk777_dn0 = assign25600_e34795_d_n0;
        locals.var_tx__blk777_dn2 = assign25600_e34795_d_n2;
        locals.var_tx__blk777_dn6 = assign25600_e34795_d_n6;
        locals.var_tx__blk777_dn7 = assign25600_e34795_d_n7;
        locals.var_tx__blk777_dn10 = assign25600_e34795_d_n10;
        locals.var_tx__blk777_dn11 = assign25600_e34795_d_n11;
        locals.var_tx__blk777_dn12 = assign25600_e34795_d_n12;
        locals.var_tx__blk777_dn17 = assign25600_e34795_d_n17;

        let assign25610_e34799: f64 = (10.0 * 2.220446049250313e-16);
        let assign25610_e34800: f64 = if locals.var_tx__blk777 < assign25610_e34799 { 1.0 } else { 0.0 };
        locals.var_guard856 = assign25610_e34800;

        let (assign25620_e34815, assign25620_e34815_d_n0, assign25620_e34815_d_n2, assign25620_e34815_d_n6, assign25620_e34815_d_n7, assign25620_e34815_d_n10, assign25620_e34815_d_n11, assign25620_e34815_d_n12, assign25620_e34815_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25620_e34813: f64 = (10.0 * 2.220446049250313e-16);
        (assign25620_e34813, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25620_e34815;
        locals.var_tx__blk777_dn0 = assign25620_e34815_d_n0;
        locals.var_tx__blk777_dn2 = assign25620_e34815_d_n2;
        locals.var_tx__blk777_dn6 = assign25620_e34815_d_n6;
        locals.var_tx__blk777_dn7 = assign25620_e34815_d_n7;
        locals.var_tx__blk777_dn10 = assign25620_e34815_d_n10;
        locals.var_tx__blk777_dn11 = assign25620_e34815_d_n11;
        locals.var_tx__blk777_dn12 = assign25620_e34815_d_n12;
        locals.var_tx__blk777_dn17 = assign25620_e34815_d_n17;

        let (assign25630_e34837, assign25630_e34837_d_n0, assign25630_e34837_d_n2, assign25630_e34837_d_n6, assign25630_e34837_d_n7, assign25630_e34837_d_n10, assign25630_e34837_d_n11, assign25630_e34837_d_n12, assign25630_e34837_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25630_e34827: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign25630_e34829: f64 = (assign25630_e34827 / 2.0);
        let assign25630_e34832: f64 = (locals.var_tx__blk777).sqrt();
        let assign25630_e34833: f64 = (1.0 - assign25630_e34832);
        let assign25630_e34834: f64 = (assign25630_e34829 * assign25630_e34833);
        let assign25630_e34835: f64 = (locals.var_vgpld + assign25630_e34834);
        (assign25630_e34835, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign25630_e34832)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign25630_e34833) + (assign25630_e34829 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign25630_e34832)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign25630_e34837;
        locals.var_ps0_inia__blk817_dn0 = assign25630_e34837_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign25630_e34837_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign25630_e34837_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign25630_e34837_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign25630_e34837_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign25630_e34837_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign25630_e34837_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign25630_e34837_d_n17;

        let (assign25640_e34852, assign25640_e34852_d_n0, assign25640_e34852_d_n2, assign25640_e34852_d_n6, assign25640_e34852_d_n7, assign25640_e34852_d_n10, assign25640_e34852_d_n11, assign25640_e34852_d_n12, assign25640_e34852_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25640_e34849: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign25640_e34850: f64 = (locals.var_beta * assign25640_e34849);
        (assign25640_e34850, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25640_e34849) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign25640_e34852;
        locals.var_chi__blk814_dn0 = assign25640_e34852_d_n0;
        locals.var_chi__blk814_dn2 = assign25640_e34852_d_n2;
        locals.var_chi__blk814_dn6 = assign25640_e34852_d_n6;
        locals.var_chi__blk814_dn7 = assign25640_e34852_d_n7;
        locals.var_chi__blk814_dn10 = assign25640_e34852_d_n10;
        locals.var_chi__blk814_dn11 = assign25640_e34852_d_n11;
        locals.var_chi__blk814_dn12 = assign25640_e34852_d_n12;
        locals.var_chi__blk814_dn17 = assign25640_e34852_d_n17;

        let assign25650_e34855: f64 = if locals.var_chi__blk814 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard857 = assign25650_e34855;

        let (assign25670_e34898,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25670_e34882: f64 = (9.0 * 1.414213562373095);
        let assign25670_e34883: f64 = (1.0 / assign25670_e34882);
        let assign25670_e34887: f64 = (7.0 * 0.049787068367863944);
        let assign25670_e34888: f64 = (5.0 + assign25670_e34887);
        let assign25670_e34892: f64 = (2.0 + 0.049787068367863944);
        let assign25670_e34893: f64 = (assign25670_e34892).sqrt();
        let assign25670_e34894: f64 = (54.0 * assign25670_e34893);
        let assign25670_e34895: f64 = (assign25670_e34888 / assign25670_e34894);
        let assign25670_e34896: f64 = (assign25670_e34883 - assign25670_e34895);
        (assign25670_e34896,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign25670_e34898;

        let (assign25680_e34924,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25680_e34911: f64 = (1.0 + 0.049787068367863944);
        let assign25680_e34915: f64 = (2.0 + 0.049787068367863944);
        let assign25680_e34916: f64 = (assign25680_e34915).sqrt();
        let assign25680_e34917: f64 = (2.0 * assign25680_e34916);
        let assign25680_e34918: f64 = (assign25680_e34911 / assign25680_e34917);
        let assign25680_e34921: f64 = (1.414213562373095 / 3.0);
        let assign25680_e34922: f64 = (assign25680_e34918 - assign25680_e34921);
        (assign25680_e34922,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign25680_e34924;

        let (assign25690_e34945, assign25690_e34945_d_n0, assign25690_e34945_d_n2, assign25690_e34945_d_n6, assign25690_e34945_d_n7, assign25690_e34945_d_n10, assign25690_e34945_d_n11, assign25690_e34945_d_n12, assign25690_e34945_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25690_e34937: f64 = (1.0 / 1.414213562373095);
        let assign25690_e34941: f64 = (locals.var_beta * locals.var_fac1__blk800);
        let assign25690_e34942: f64 = (1.0 / assign25690_e34941);
        let assign25690_e34943: f64 = (assign25690_e34937 + assign25690_e34942);
        (assign25690_e34943, (-((locals.var_beta * locals.var_fac1__blk800_dn0) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn2) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn6) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn7) / (assign25690_e34941 * assign25690_e34941))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk800) + (locals.var_beta * locals.var_fac1__blk800_dn10)) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn11) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn12) / (assign25690_e34941 * assign25690_e34941))), (-((locals.var_beta * locals.var_fac1__blk800_dn17) / (assign25690_e34941 * assign25690_e34941))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign25690_e34945;
        locals.var_tc_dn0 = assign25690_e34945_d_n0;
        locals.var_tc_dn2 = assign25690_e34945_d_n2;
        locals.var_tc_dn6 = assign25690_e34945_d_n6;
        locals.var_tc_dn7 = assign25690_e34945_d_n7;
        locals.var_tc_dn10 = assign25690_e34945_d_n10;
        locals.var_tc_dn11 = assign25690_e34945_d_n11;
        locals.var_tc_dn12 = assign25690_e34945_d_n12;
        locals.var_tc_dn17 = assign25690_e34945_d_n17;

        let (assign25700_e34963, assign25700_e34963_d_n0, assign25700_e34963_d_n2, assign25700_e34963_d_n6, assign25700_e34963_d_n7, assign25700_e34963_d_n10, assign25700_e34963_d_n11, assign25700_e34963_d_n12, assign25700_e34963_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25700_e34958: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25700_e34959: f64 = (-assign25700_e34958);
        let assign25700_e34961: f64 = (assign25700_e34959 / locals.var_fac1__blk800);
        (assign25700_e34961, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn0)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn2)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn6)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn7)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn10)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn11)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn12)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk800) - (assign25700_e34959 * locals.var_fac1__blk800_dn17)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign25700_e34963;
        locals.var_td_dn0 = assign25700_e34963_d_n0;
        locals.var_td_dn2 = assign25700_e34963_d_n2;
        locals.var_td_dn6 = assign25700_e34963_d_n6;
        locals.var_td_dn7 = assign25700_e34963_d_n7;
        locals.var_td_dn10 = assign25700_e34963_d_n10;
        locals.var_td_dn11 = assign25700_e34963_d_n11;
        locals.var_td_dn12 = assign25700_e34963_d_n12;
        locals.var_td_dn17 = assign25700_e34963_d_n17;

        let (assign25710_e35004, assign25710_e35004_d_n0, assign25710_e35004_d_n2, assign25710_e35004_d_n6, assign25710_e35004_d_n7, assign25710_e35004_d_n10, assign25710_e35004_d_n11, assign25710_e35004_d_n12, assign25710_e35004_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25710_e34976: f64 = (locals.var_tb * locals.var_tb);
        let assign25710_e34978: f64 = (assign25710_e34976 * locals.var_tb);
        let assign25710_e34981: f64 = (27.0 * locals.var_ta);
        let assign25710_e34983: f64 = (assign25710_e34981 * locals.var_ta);
        let assign25710_e34985: f64 = (assign25710_e34983 * locals.var_ta);
        let assign25710_e34986: f64 = (assign25710_e34978 / assign25710_e34985);
        let assign25710_e34989: f64 = (locals.var_tb * locals.var_tc);
        let assign25710_e34992: f64 = (6.0 * locals.var_ta);
        let assign25710_e34994: f64 = (assign25710_e34992 * locals.var_ta);
        let assign25710_e34995: f64 = (assign25710_e34989 / assign25710_e34994);
        let assign25710_e34996: f64 = (assign25710_e34986 - assign25710_e34995);
        let assign25710_e35000: f64 = (2.0 * locals.var_ta);
        let assign25710_e35001: f64 = (locals.var_td / assign25710_e35000);
        let assign25710_e35002: f64 = (assign25710_e34996 + assign25710_e35001);
        (assign25710_e35002, ((-((locals.var_tb * locals.var_tc_dn0) / assign25710_e34994)) + (locals.var_td_dn0 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn2) / assign25710_e34994)) + (locals.var_td_dn2 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn6) / assign25710_e34994)) + (locals.var_td_dn6 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn7) / assign25710_e34994)) + (locals.var_td_dn7 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn10) / assign25710_e34994)) + (locals.var_td_dn10 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn11) / assign25710_e34994)) + (locals.var_td_dn11 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn12) / assign25710_e34994)) + (locals.var_td_dn12 / assign25710_e35000)), ((-((locals.var_tb * locals.var_tc_dn17) / assign25710_e34994)) + (locals.var_td_dn17 / assign25710_e35000)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign25710_e35004;
        locals.var_tq_dn0 = assign25710_e35004_d_n0;
        locals.var_tq_dn2 = assign25710_e35004_d_n2;
        locals.var_tq_dn6 = assign25710_e35004_d_n6;
        locals.var_tq_dn7 = assign25710_e35004_d_n7;
        locals.var_tq_dn10 = assign25710_e35004_d_n10;
        locals.var_tq_dn11 = assign25710_e35004_d_n11;
        locals.var_tq_dn12 = assign25710_e35004_d_n12;
        locals.var_tq_dn17 = assign25710_e35004_d_n17;

        let (assign25720_e35031, assign25720_e35031_d_n0, assign25720_e35031_d_n2, assign25720_e35031_d_n6, assign25720_e35031_d_n7, assign25720_e35031_d_n10, assign25720_e35031_d_n11, assign25720_e35031_d_n12, assign25720_e35031_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25720_e35017: f64 = (3.0 * locals.var_ta);
        let assign25720_e35019: f64 = (assign25720_e35017 * locals.var_tc);
        let assign25720_e35022: f64 = (locals.var_tb * locals.var_tb);
        let assign25720_e35023: f64 = (assign25720_e35019 - assign25720_e35022);
        let assign25720_e35026: f64 = (9.0 * locals.var_ta);
        let assign25720_e35028: f64 = (assign25720_e35026 * locals.var_ta);
        let assign25720_e35029: f64 = (assign25720_e35023 / assign25720_e35028);
        (assign25720_e35029, ((assign25720_e35017 * locals.var_tc_dn0) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn2) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn6) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn7) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn10) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn11) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn12) / assign25720_e35028), ((assign25720_e35017 * locals.var_tc_dn17) / assign25720_e35028),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign25720_e35031;
        locals.var_tp_dn0 = assign25720_e35031_d_n0;
        locals.var_tp_dn2 = assign25720_e35031_d_n2;
        locals.var_tp_dn6 = assign25720_e35031_d_n6;
        locals.var_tp_dn7 = assign25720_e35031_d_n7;
        locals.var_tp_dn10 = assign25720_e35031_d_n10;
        locals.var_tp_dn11 = assign25720_e35031_d_n11;
        locals.var_tp_dn12 = assign25720_e35031_d_n12;
        locals.var_tp_dn17 = assign25720_e35031_d_n17;

        let (assign25730_e35053, assign25730_e35053_d_n0, assign25730_e35053_d_n2, assign25730_e35053_d_n6, assign25730_e35053_d_n7, assign25730_e35053_d_n10, assign25730_e35053_d_n11, assign25730_e35053_d_n12, assign25730_e35053_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25730_e35044: f64 = (locals.var_tq * locals.var_tq);
        let assign25730_e35047: f64 = (locals.var_tp * locals.var_tp);
        let assign25730_e35049: f64 = (assign25730_e35047 * locals.var_tp);
        let assign25730_e35050: f64 = (assign25730_e35044 + assign25730_e35049);
        let assign25730_e35051: f64 = (assign25730_e35050).sqrt();
        (assign25730_e35051, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn0))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn2))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn6))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn7))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn10))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn11))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn12))) / (2.0 * assign25730_e35051)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign25730_e35047 * locals.var_tp_dn17))) / (2.0 * assign25730_e35051)),)
    } else {
        (locals.var_t5__blk774, locals.var_t5__blk774_dn0, locals.var_t5__blk774_dn2, locals.var_t5__blk774_dn6, locals.var_t5__blk774_dn7, locals.var_t5__blk774_dn10, locals.var_t5__blk774_dn11, locals.var_t5__blk774_dn12, locals.var_t5__blk774_dn17,)
    }
};
        locals.var_t5__blk774 = assign25730_e35053;
        locals.var_t5__blk774_dn0 = assign25730_e35053_d_n0;
        locals.var_t5__blk774_dn2 = assign25730_e35053_d_n2;
        locals.var_t5__blk774_dn6 = assign25730_e35053_d_n6;
        locals.var_t5__blk774_dn7 = assign25730_e35053_d_n7;
        locals.var_t5__blk774_dn10 = assign25730_e35053_d_n10;
        locals.var_t5__blk774_dn11 = assign25730_e35053_d_n11;
        locals.var_t5__blk774_dn12 = assign25730_e35053_d_n12;
        locals.var_t5__blk774_dn17 = assign25730_e35053_d_n17;

        let (assign25740_e35071, assign25740_e35071_d_n0, assign25740_e35071_d_n2, assign25740_e35071_d_n6, assign25740_e35071_d_n7, assign25740_e35071_d_n10, assign25740_e35071_d_n11, assign25740_e35071_d_n12, assign25740_e35071_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25740_e35065: f64 = (-locals.var_tq);
        let assign25740_e35067: f64 = (assign25740_e35065 + locals.var_t5__blk774);
        let assign25740_e35069: f64 = (assign25740_e35067).powf(0.3333333333333333);
        (assign25740_e35069, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12) / assign25740_e35067))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25740_e35067).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17))) } } else { (assign25740_e35069 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17) / assign25740_e35067))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign25740_e35071;
        locals.var_tu_dn0 = assign25740_e35071_d_n0;
        locals.var_tu_dn2 = assign25740_e35071_d_n2;
        locals.var_tu_dn6 = assign25740_e35071_d_n6;
        locals.var_tu_dn7 = assign25740_e35071_d_n7;
        locals.var_tu_dn10 = assign25740_e35071_d_n10;
        locals.var_tu_dn11 = assign25740_e35071_d_n11;
        locals.var_tu_dn12 = assign25740_e35071_d_n12;
        locals.var_tu_dn17 = assign25740_e35071_d_n17;

        let (assign25750_e35089, assign25750_e35089_d_n0, assign25750_e35089_d_n2, assign25750_e35089_d_n6, assign25750_e35089_d_n7, assign25750_e35089_d_n10, assign25750_e35089_d_n11, assign25750_e35089_d_n12, assign25750_e35089_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25750_e35084: f64 = (locals.var_tq + locals.var_t5__blk774);
        let assign25750_e35086: f64 = (assign25750_e35084).powf(0.3333333333333333);
        let assign25750_e35087: f64 = (-assign25750_e35086);
        (assign25750_e35087, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk774_dn0))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk774_dn0) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk774_dn2))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk774_dn2) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk774_dn6))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk774_dn6) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk774_dn7))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk774_dn7) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk774_dn10))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk774_dn10) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk774_dn11))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk774_dn11) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk774_dn12))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk774_dn12) / assign25750_e35084))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25750_e35084).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk774_dn17))) } } else { (assign25750_e35086 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk774_dn17) / assign25750_e35084))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign25750_e35089;
        locals.var_tv_dn0 = assign25750_e35089_d_n0;
        locals.var_tv_dn2 = assign25750_e35089_d_n2;
        locals.var_tv_dn6 = assign25750_e35089_d_n6;
        locals.var_tv_dn7 = assign25750_e35089_d_n7;
        locals.var_tv_dn10 = assign25750_e35089_d_n10;
        locals.var_tv_dn11 = assign25750_e35089_d_n11;
        locals.var_tv_dn12 = assign25750_e35089_d_n12;
        locals.var_tv_dn17 = assign25750_e35089_d_n17;

        let (assign25760_e35110, assign25760_e35110_d_n0, assign25760_e35110_d_n2, assign25760_e35110_d_n6, assign25760_e35110_d_n7, assign25760_e35110_d_n10, assign25760_e35110_d_n11, assign25760_e35110_d_n12, assign25760_e35110_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25760_e35102: f64 = (locals.var_tu + locals.var_tv);
        let assign25760_e35106: f64 = (3.0 * locals.var_ta);
        let assign25760_e35107: f64 = (locals.var_tb / assign25760_e35106);
        let assign25760_e35108: f64 = (assign25760_e35102 - assign25760_e35107);
        (assign25760_e35108, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign25760_e35110;
        locals.var_tx__blk777_dn0 = assign25760_e35110_d_n0;
        locals.var_tx__blk777_dn2 = assign25760_e35110_d_n2;
        locals.var_tx__blk777_dn6 = assign25760_e35110_d_n6;
        locals.var_tx__blk777_dn7 = assign25760_e35110_d_n7;
        locals.var_tx__blk777_dn10 = assign25760_e35110_d_n10;
        locals.var_tx__blk777_dn11 = assign25760_e35110_d_n11;
        locals.var_tx__blk777_dn12 = assign25760_e35110_d_n12;
        locals.var_tx__blk777_dn17 = assign25760_e35110_d_n17;

    }

    pub(super) fn stamp_transient_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25770_e35127, assign25770_e35127_d_n0, assign25770_e35127_d_n2, assign25770_e35127_d_n6, assign25770_e35127_d_n7, assign25770_e35127_d_n10, assign25770_e35127_d_n11, assign25770_e35127_d_n12, assign25770_e35127_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25770_e35123: f64 = (locals.var_tx__blk777 * locals.var_beta_inv);
        let assign25770_e35125: f64 = (assign25770_e35123 - locals.var_vxbgmtcl);
        (assign25770_e35125, ((locals.var_tx__blk777_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk777_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk777_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk777_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk777_dn10 * locals.var_beta_inv) + (locals.var_tx__blk777 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk777_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk777_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk777_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign25770_e35127;
        locals.var_ps0_inia__blk817_dn0 = assign25770_e35127_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign25770_e35127_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign25770_e35127_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign25770_e35127_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign25770_e35127_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign25770_e35127_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign25770_e35127_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign25770_e35127_d_n17;

        let (assign25780_e35144, assign25780_e35144_d_n0, assign25780_e35144_d_n2, assign25780_e35144_d_n6, assign25780_e35144_d_n7, assign25780_e35144_d_n10, assign25780_e35144_d_n11, assign25780_e35144_d_n12, assign25780_e35144_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25780_e35141: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign25780_e35142: f64 = (locals.var_beta * assign25780_e35141);
        (assign25780_e35142, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25780_e35141) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign25780_e35144;
        locals.var_chi__blk814_dn0 = assign25780_e35144_d_n0;
        locals.var_chi__blk814_dn2 = assign25780_e35144_d_n2;
        locals.var_chi__blk814_dn6 = assign25780_e35144_d_n6;
        locals.var_chi__blk814_dn7 = assign25780_e35144_d_n7;
        locals.var_chi__blk814_dn10 = assign25780_e35144_d_n10;
        locals.var_chi__blk814_dn11 = assign25780_e35144_d_n11;
        locals.var_chi__blk814_dn12 = assign25780_e35144_d_n12;
        locals.var_chi__blk814_dn17 = assign25780_e35144_d_n17;

        let (assign25800_e35172, assign25800_e35172_d_n0, assign25800_e35172_d_n2, assign25800_e35172_d_n6, assign25800_e35172_d_n7, assign25800_e35172_d_n10, assign25800_e35172_d_n11, assign25800_e35172_d_n12, assign25800_e35172_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25800_e35168: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25800_e35170: f64 = (assign25800_e35168 + 0.1);
        (assign25800_e35170, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign25800_e35172;
        locals.var_vgpld_shift_dn0 = assign25800_e35172_d_n0;
        locals.var_vgpld_shift_dn2 = assign25800_e35172_d_n2;
        locals.var_vgpld_shift_dn6 = assign25800_e35172_d_n6;
        locals.var_vgpld_shift_dn7 = assign25800_e35172_d_n7;
        locals.var_vgpld_shift_dn10 = assign25800_e35172_d_n10;
        locals.var_vgpld_shift_dn11 = assign25800_e35172_d_n11;
        locals.var_vgpld_shift_dn12 = assign25800_e35172_d_n12;
        locals.var_vgpld_shift_dn17 = assign25800_e35172_d_n17;

        let (assign25810_e35189, assign25810_e35189_d_n0, assign25810_e35189_d_n2, assign25810_e35189_d_n6, assign25810_e35189_d_n7, assign25810_e35189_d_n10, assign25810_e35189_d_n11, assign25810_e35189_d_n12, assign25810_e35189_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25810_e35183: f64 = (-locals.var_vxbgmtcl);
        let assign25810_e35184: f64 = (locals.var_beta * assign25810_e35183);
        let assign25810_e35185: f64 = (assign25810_e35184).exp();
        let assign25810_e35187: f64 = (assign25810_e35185 + 1e-50);
        (assign25810_e35187, (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign25810_e35185 * ((locals.var_beta_dn10 * assign25810_e35183) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign25810_e35185 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign25810_e35189;
        locals.var_exp_bvbs__blk833_dn0 = assign25810_e35189_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign25810_e35189_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign25810_e35189_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign25810_e35189_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign25810_e35189_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign25810_e35189_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign25810_e35189_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign25810_e35189_d_n17;

        let (assign25820_e35202, assign25820_e35202_d_n0, assign25820_e35202_d_n2, assign25820_e35202_d_n6, assign25820_e35202_d_n7, assign25820_e35202_d_n10, assign25820_e35202_d_n11, assign25820_e35202_d_n12, assign25820_e35202_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25820_e35200: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign25820_e35200, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign25820_e35202;
        locals.var_t0__blk770_dn0 = assign25820_e35202_d_n0;
        locals.var_t0__blk770_dn2 = assign25820_e35202_d_n2;
        locals.var_t0__blk770_dn6 = assign25820_e35202_d_n6;
        locals.var_t0__blk770_dn7 = assign25820_e35202_d_n7;
        locals.var_t0__blk770_dn10 = assign25820_e35202_d_n10;
        locals.var_t0__blk770_dn11 = assign25820_e35202_d_n11;
        locals.var_t0__blk770_dn12 = assign25820_e35202_d_n12;
        locals.var_t0__blk770_dn17 = assign25820_e35202_d_n17;

        let (assign25830_e35215, assign25830_e35215_d_n0, assign25830_e35215_d_n2, assign25830_e35215_d_n6, assign25830_e35215_d_n7, assign25830_e35215_d_n10, assign25830_e35215_d_n11, assign25830_e35215_d_n12, assign25830_e35215_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25830_e35213: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign25830_e35213, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign25830_e35215;
        locals.var_cnst1over_dn0 = assign25830_e35215_d_n0;
        locals.var_cnst1over_dn2 = assign25830_e35215_d_n2;
        locals.var_cnst1over_dn6 = assign25830_e35215_d_n6;
        locals.var_cnst1over_dn7 = assign25830_e35215_d_n7;
        locals.var_cnst1over_dn10 = assign25830_e35215_d_n10;
        locals.var_cnst1over_dn11 = assign25830_e35215_d_n11;
        locals.var_cnst1over_dn12 = assign25830_e35215_d_n12;
        locals.var_cnst1over_dn17 = assign25830_e35215_d_n17;

        let (assign25840_e35228, assign25840_e35228_d_n0, assign25840_e35228_d_n2, assign25840_e35228_d_n6, assign25840_e35228_d_n7, assign25840_e35228_d_n10, assign25840_e35228_d_n11, assign25840_e35228_d_n12, assign25840_e35228_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25840_e35226: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign25840_e35226, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign25840_e35228;
        locals.var_gammachi_dn0 = assign25840_e35228_d_n0;
        locals.var_gammachi_dn2 = assign25840_e35228_d_n2;
        locals.var_gammachi_dn6 = assign25840_e35228_d_n6;
        locals.var_gammachi_dn7 = assign25840_e35228_d_n7;
        locals.var_gammachi_dn10 = assign25840_e35228_d_n10;
        locals.var_gammachi_dn11 = assign25840_e35228_d_n11;
        locals.var_gammachi_dn12 = assign25840_e35228_d_n12;
        locals.var_gammachi_dn17 = assign25840_e35228_d_n17;

        let (assign25850_e35241, assign25850_e35241_d_n0, assign25850_e35241_d_n2, assign25850_e35241_d_n6, assign25850_e35241_d_n7, assign25850_e35241_d_n10, assign25850_e35241_d_n11, assign25850_e35241_d_n12, assign25850_e35241_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25850_e35239: f64 = (locals.var_beta2 * locals.var_fac1p2__blk801);
        (assign25850_e35239, (locals.var_beta2 * locals.var_fac1p2__blk801_dn0), (locals.var_beta2 * locals.var_fac1p2__blk801_dn2), (locals.var_beta2 * locals.var_fac1p2__blk801_dn6), (locals.var_beta2 * locals.var_fac1p2__blk801_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk801) + (locals.var_beta2 * locals.var_fac1p2__blk801_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk801_dn11), (locals.var_beta2 * locals.var_fac1p2__blk801_dn12), (locals.var_beta2 * locals.var_fac1p2__blk801_dn17),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign25850_e35241;
        locals.var_t0__blk770_dn0 = assign25850_e35241_d_n0;
        locals.var_t0__blk770_dn2 = assign25850_e35241_d_n2;
        locals.var_t0__blk770_dn6 = assign25850_e35241_d_n6;
        locals.var_t0__blk770_dn7 = assign25850_e35241_d_n7;
        locals.var_t0__blk770_dn10 = assign25850_e35241_d_n10;
        locals.var_t0__blk770_dn11 = assign25850_e35241_d_n11;
        locals.var_t0__blk770_dn12 = assign25850_e35241_d_n12;
        locals.var_t0__blk770_dn17 = assign25850_e35241_d_n17;

        let (assign25860_e35254, assign25860_e35254_d_n0, assign25860_e35254_d_n2, assign25860_e35254_d_n6, assign25860_e35254_d_n7, assign25860_e35254_d_n10, assign25860_e35254_d_n11, assign25860_e35254_d_n12, assign25860_e35254_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25860_e35252: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign25860_e35252, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25860_e35254;
        locals.var_psi_dn0 = assign25860_e35254_d_n0;
        locals.var_psi_dn2 = assign25860_e35254_d_n2;
        locals.var_psi_dn6 = assign25860_e35254_d_n6;
        locals.var_psi_dn7 = assign25860_e35254_d_n7;
        locals.var_psi_dn10 = assign25860_e35254_d_n10;
        locals.var_psi_dn11 = assign25860_e35254_d_n11;
        locals.var_psi_dn12 = assign25860_e35254_d_n12;
        locals.var_psi_dn17 = assign25860_e35254_d_n17;

        let (assign25870_e35281, assign25870_e35281_d_n0, assign25870_e35281_d_n2, assign25870_e35281_d_n6, assign25870_e35281_d_n7, assign25870_e35281_d_n10, assign25870_e35281_d_n11, assign25870_e35281_d_n12, assign25870_e35281_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25870_e35265: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign25870_e35268: f64 = (locals.var_psi * locals.var_psi);
        let assign25870_e35269: f64 = (assign25870_e35265 + assign25870_e35268);
        let assign25870_e35270: f64 = (assign25870_e35269).ln();
        let assign25870_e35273: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign25870_e35274: f64 = (assign25870_e35273).ln();
        let assign25870_e35275: f64 = (assign25870_e35270 - assign25870_e35274);
        let assign25870_e35278: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign25870_e35279: f64 = (assign25870_e35275 + assign25870_e35278);
        (assign25870_e35279, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign25870_e35269) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign25870_e35269) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign25870_e35269) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign25870_e35269) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign25870_e35269) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign25870_e35273)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign25870_e35269) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign25870_e35269) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign25870_e35269) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign25870_e35273)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25870_e35281;
        locals.var_chi_1_dn0 = assign25870_e35281_d_n0;
        locals.var_chi_1_dn2 = assign25870_e35281_d_n2;
        locals.var_chi_1_dn6 = assign25870_e35281_d_n6;
        locals.var_chi_1_dn7 = assign25870_e35281_d_n7;
        locals.var_chi_1_dn10 = assign25870_e35281_d_n10;
        locals.var_chi_1_dn11 = assign25870_e35281_d_n11;
        locals.var_chi_1_dn12 = assign25870_e35281_d_n12;
        locals.var_chi_1_dn17 = assign25870_e35281_d_n17;

        let (assign25880_e35296, assign25880_e35296_d_n0, assign25880_e35296_d_n2, assign25880_e35296_d_n6, assign25880_e35296_d_n7, assign25880_e35296_d_n10, assign25880_e35296_d_n11, assign25880_e35296_d_n12, assign25880_e35296_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25880_e35292: f64 = (locals.var_psi - locals.var_chi_1);
        let assign25880_e35294: f64 = (assign25880_e35292 - 1.0);
        (assign25880_e35294, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25880_e35296;
        locals.var_tmf1_dn0 = assign25880_e35296_d_n0;
        locals.var_tmf1_dn2 = assign25880_e35296_d_n2;
        locals.var_tmf1_dn6 = assign25880_e35296_d_n6;
        locals.var_tmf1_dn7 = assign25880_e35296_d_n7;
        locals.var_tmf1_dn10 = assign25880_e35296_d_n10;
        locals.var_tmf1_dn11 = assign25880_e35296_d_n11;
        locals.var_tmf1_dn12 = assign25880_e35296_d_n12;
        locals.var_tmf1_dn17 = assign25880_e35296_d_n17;

        let (assign25890_e35311, assign25890_e35311_d_n0, assign25890_e35311_d_n2, assign25890_e35311_d_n6, assign25890_e35311_d_n7, assign25890_e35311_d_n10, assign25890_e35311_d_n11, assign25890_e35311_d_n12, assign25890_e35311_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25890_e35307: f64 = (4.0 * locals.var_psi);
        let assign25890_e35309: f64 = assign25890_e35307;
        (assign25890_e35309, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25890_e35311;
        locals.var_tmf2_dn0 = assign25890_e35311_d_n0;
        locals.var_tmf2_dn2 = assign25890_e35311_d_n2;
        locals.var_tmf2_dn6 = assign25890_e35311_d_n6;
        locals.var_tmf2_dn7 = assign25890_e35311_d_n7;
        locals.var_tmf2_dn10 = assign25890_e35311_d_n10;
        locals.var_tmf2_dn11 = assign25890_e35311_d_n11;
        locals.var_tmf2_dn12 = assign25890_e35311_d_n12;
        locals.var_tmf2_dn17 = assign25890_e35311_d_n17;

        let (assign25900_e35328, assign25900_e35328_d_n0, assign25900_e35328_d_n2, assign25900_e35328_d_n6, assign25900_e35328_d_n7, assign25900_e35328_d_n10, assign25900_e35328_d_n11, assign25900_e35328_d_n12, assign25900_e35328_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let (assign25900_e35326, assign25900_e35326_d_n0, assign25900_e35326_d_n2, assign25900_e35326_d_n6, assign25900_e35326_d_n7, assign25900_e35326_d_n10, assign25900_e35326_d_n11, assign25900_e35326_d_n12, assign25900_e35326_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign25900_e35325: f64 = (-locals.var_tmf2);
                (assign25900_e35325, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign25900_e35326, assign25900_e35326_d_n0, assign25900_e35326_d_n2, assign25900_e35326_d_n6, assign25900_e35326_d_n7, assign25900_e35326_d_n10, assign25900_e35326_d_n11, assign25900_e35326_d_n12, assign25900_e35326_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25900_e35328;
        locals.var_tmf2_dn0 = assign25900_e35328_d_n0;
        locals.var_tmf2_dn2 = assign25900_e35328_d_n2;
        locals.var_tmf2_dn6 = assign25900_e35328_d_n6;
        locals.var_tmf2_dn7 = assign25900_e35328_d_n7;
        locals.var_tmf2_dn10 = assign25900_e35328_d_n10;
        locals.var_tmf2_dn11 = assign25900_e35328_d_n11;
        locals.var_tmf2_dn12 = assign25900_e35328_d_n12;
        locals.var_tmf2_dn17 = assign25900_e35328_d_n17;

        let (assign25910_e35344, assign25910_e35344_d_n0, assign25910_e35344_d_n2, assign25910_e35344_d_n6, assign25910_e35344_d_n7, assign25910_e35344_d_n10, assign25910_e35344_d_n11, assign25910_e35344_d_n12, assign25910_e35344_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25910_e35339: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25910_e35341: f64 = (assign25910_e35339 + locals.var_tmf2);
        let assign25910_e35342: f64 = (assign25910_e35341).sqrt();
        (assign25910_e35342, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign25910_e35342)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign25910_e35342)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25910_e35344;
        locals.var_tmf2_dn0 = assign25910_e35344_d_n0;
        locals.var_tmf2_dn2 = assign25910_e35344_d_n2;
        locals.var_tmf2_dn6 = assign25910_e35344_d_n6;
        locals.var_tmf2_dn7 = assign25910_e35344_d_n7;
        locals.var_tmf2_dn10 = assign25910_e35344_d_n10;
        locals.var_tmf2_dn11 = assign25910_e35344_d_n11;
        locals.var_tmf2_dn12 = assign25910_e35344_d_n12;
        locals.var_tmf2_dn17 = assign25910_e35344_d_n17;

        let (assign25920_e35361, assign25920_e35361_d_n0, assign25920_e35361_d_n2, assign25920_e35361_d_n6, assign25920_e35361_d_n7, assign25920_e35361_d_n10, assign25920_e35361_d_n11, assign25920_e35361_d_n12, assign25920_e35361_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25920_e35357: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25920_e35358: f64 = (1.0 + assign25920_e35357);
        let assign25920_e35359: f64 = (0.5 * assign25920_e35358);
        (assign25920_e35359, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign25920_e35361;
        locals.var_t1__blk771_dn0 = assign25920_e35361_d_n0;
        locals.var_t1__blk771_dn2 = assign25920_e35361_d_n2;
        locals.var_t1__blk771_dn6 = assign25920_e35361_d_n6;
        locals.var_t1__blk771_dn7 = assign25920_e35361_d_n7;
        locals.var_t1__blk771_dn10 = assign25920_e35361_d_n10;
        locals.var_t1__blk771_dn11 = assign25920_e35361_d_n11;
        locals.var_t1__blk771_dn12 = assign25920_e35361_d_n12;
        locals.var_t1__blk771_dn17 = assign25920_e35361_d_n17;

        let (assign25930_e35382, assign25930_e35382_d_n0, assign25930_e35382_d_n2, assign25930_e35382_d_n6, assign25930_e35382_d_n7, assign25930_e35382_d_n10, assign25930_e35382_d_n11, assign25930_e35382_d_n12, assign25930_e35382_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25930_e35375: f64 = 2.0;
        let assign25930_e35376: f64 = (locals.var_tmf1 + assign25930_e35375);
        let assign25930_e35378: f64 = (assign25930_e35376 / locals.var_tmf2);
        let assign25930_e35379: f64 = (1.0 - assign25930_e35378);
        let assign25930_e35380: f64 = (0.5 * assign25930_e35379);
        (assign25930_e35380, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign25930_e35376 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign25930_e35382;
        locals.var_t2__blk772_dn0 = assign25930_e35382_d_n0;
        locals.var_t2__blk772_dn2 = assign25930_e35382_d_n2;
        locals.var_t2__blk772_dn6 = assign25930_e35382_d_n6;
        locals.var_t2__blk772_dn7 = assign25930_e35382_d_n7;
        locals.var_t2__blk772_dn10 = assign25930_e35382_d_n10;
        locals.var_t2__blk772_dn11 = assign25930_e35382_d_n11;
        locals.var_t2__blk772_dn12 = assign25930_e35382_d_n12;
        locals.var_t2__blk772_dn17 = assign25930_e35382_d_n17;

        let (assign25940_e35399, assign25940_e35399_d_n0, assign25940_e35399_d_n2, assign25940_e35399_d_n6, assign25940_e35399_d_n7, assign25940_e35399_d_n10, assign25940_e35399_d_n11, assign25940_e35399_d_n12, assign25940_e35399_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25940_e35395: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25940_e35396: f64 = (0.5 * assign25940_e35395);
        let assign25940_e35397: f64 = (locals.var_psi - assign25940_e35396);
        (assign25940_e35397, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25940_e35399;
        locals.var_chi_1_dn0 = assign25940_e35399_d_n0;
        locals.var_chi_1_dn2 = assign25940_e35399_d_n2;
        locals.var_chi_1_dn6 = assign25940_e35399_d_n6;
        locals.var_chi_1_dn7 = assign25940_e35399_d_n7;
        locals.var_chi_1_dn10 = assign25940_e35399_d_n10;
        locals.var_chi_1_dn11 = assign25940_e35399_d_n11;
        locals.var_chi_1_dn12 = assign25940_e35399_d_n12;
        locals.var_chi_1_dn17 = assign25940_e35399_d_n17;

        let (assign25950_e35412, assign25950_e35412_d_n0, assign25950_e35412_d_n2, assign25950_e35412_d_n6, assign25950_e35412_d_n7, assign25950_e35412_d_n10, assign25950_e35412_d_n11, assign25950_e35412_d_n12, assign25950_e35412_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25950_e35410: f64 = (locals.var_psi - locals.var_chi_1);
        (assign25950_e35410, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25950_e35412;
        locals.var_psi_dn0 = assign25950_e35412_d_n0;
        locals.var_psi_dn2 = assign25950_e35412_d_n2;
        locals.var_psi_dn6 = assign25950_e35412_d_n6;
        locals.var_psi_dn7 = assign25950_e35412_d_n7;
        locals.var_psi_dn10 = assign25950_e35412_d_n10;
        locals.var_psi_dn11 = assign25950_e35412_d_n11;
        locals.var_psi_dn12 = assign25950_e35412_d_n12;
        locals.var_psi_dn17 = assign25950_e35412_d_n17;

        let (assign25960_e35427, assign25960_e35427_d_n0, assign25960_e35427_d_n2, assign25960_e35427_d_n6, assign25960_e35427_d_n7, assign25960_e35427_d_n10, assign25960_e35427_d_n11, assign25960_e35427_d_n12, assign25960_e35427_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25960_e35424: f64 = (locals.var_beta * 0.1);
        let assign25960_e35425: f64 = (locals.var_psi + assign25960_e35424);
        (assign25960_e35425, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25960_e35427;
        locals.var_psi_dn0 = assign25960_e35427_d_n0;
        locals.var_psi_dn2 = assign25960_e35427_d_n2;
        locals.var_psi_dn6 = assign25960_e35427_d_n6;
        locals.var_psi_dn7 = assign25960_e35427_d_n7;
        locals.var_psi_dn10 = assign25960_e35427_d_n10;
        locals.var_psi_dn11 = assign25960_e35427_d_n11;
        locals.var_psi_dn12 = assign25960_e35427_d_n12;
        locals.var_psi_dn17 = assign25960_e35427_d_n17;

        let (assign25970_e35454, assign25970_e35454_d_n0, assign25970_e35454_d_n2, assign25970_e35454_d_n6, assign25970_e35454_d_n7, assign25970_e35454_d_n10, assign25970_e35454_d_n11, assign25970_e35454_d_n12, assign25970_e35454_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25970_e35438: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign25970_e35441: f64 = (locals.var_psi * locals.var_psi);
        let assign25970_e35442: f64 = (assign25970_e35438 + assign25970_e35441);
        let assign25970_e35443: f64 = (assign25970_e35442).ln();
        let assign25970_e35446: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign25970_e35447: f64 = (assign25970_e35446).ln();
        let assign25970_e35448: f64 = (assign25970_e35443 - assign25970_e35447);
        let assign25970_e35451: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign25970_e35452: f64 = (assign25970_e35448 + assign25970_e35451);
        (assign25970_e35452, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign25970_e35442) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign25970_e35442) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign25970_e35442) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign25970_e35442) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign25970_e35442) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign25970_e35446)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign25970_e35442) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign25970_e35442) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign25970_e35442) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign25970_e35446)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign25970_e35454;
        locals.var_chi_b_dn0 = assign25970_e35454_d_n0;
        locals.var_chi_b_dn2 = assign25970_e35454_d_n2;
        locals.var_chi_b_dn6 = assign25970_e35454_d_n6;
        locals.var_chi_b_dn7 = assign25970_e35454_d_n7;
        locals.var_chi_b_dn10 = assign25970_e35454_d_n10;
        locals.var_chi_b_dn11 = assign25970_e35454_d_n11;
        locals.var_chi_b_dn12 = assign25970_e35454_d_n12;
        locals.var_chi_b_dn17 = assign25970_e35454_d_n17;

        let (assign25980_e35465, assign25980_e35465_d_n0, assign25980_e35465_d_n2, assign25980_e35465_d_n6, assign25980_e35465_d_n7, assign25980_e35465_d_n10, assign25980_e35465_d_n11, assign25980_e35465_d_n12, assign25980_e35465_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign25980_e35465;
        locals.var_chi_a_dn0 = assign25980_e35465_d_n0;
        locals.var_chi_a_dn2 = assign25980_e35465_d_n2;
        locals.var_chi_a_dn6 = assign25980_e35465_d_n6;
        locals.var_chi_a_dn7 = assign25980_e35465_d_n7;
        locals.var_chi_a_dn10 = assign25980_e35465_d_n10;
        locals.var_chi_a_dn11 = assign25980_e35465_d_n11;
        locals.var_chi_a_dn12 = assign25980_e35465_d_n12;
        locals.var_chi_a_dn17 = assign25980_e35465_d_n17;

        let (assign25990_e35482, assign25990_e35482_d_n0, assign25990_e35482_d_n2, assign25990_e35482_d_n6, assign25990_e35482_d_n7, assign25990_e35482_d_n10, assign25990_e35482_d_n11, assign25990_e35482_d_n12, assign25990_e35482_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign25990_e35476: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign25990_e35479: f64 = (0.0008 * 75.0);
        let assign25990_e35480: f64 = (assign25990_e35476 - assign25990_e35479);
        (assign25990_e35480, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25990_e35482;
        locals.var_tmf1_dn0 = assign25990_e35482_d_n0;
        locals.var_tmf1_dn2 = assign25990_e35482_d_n2;
        locals.var_tmf1_dn6 = assign25990_e35482_d_n6;
        locals.var_tmf1_dn7 = assign25990_e35482_d_n7;
        locals.var_tmf1_dn10 = assign25990_e35482_d_n10;
        locals.var_tmf1_dn11 = assign25990_e35482_d_n11;
        locals.var_tmf1_dn12 = assign25990_e35482_d_n12;
        locals.var_tmf1_dn17 = assign25990_e35482_d_n17;

        let (assign26000_e35499, assign26000_e35499_d_n0, assign26000_e35499_d_n2, assign26000_e35499_d_n6, assign26000_e35499_d_n7, assign26000_e35499_d_n10, assign26000_e35499_d_n11, assign26000_e35499_d_n12, assign26000_e35499_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26000_e35493: f64 = (4.0 * locals.var_chi_b);
        let assign26000_e35496: f64 = (0.0008 * 75.0);
        let assign26000_e35497: f64 = (assign26000_e35493 * assign26000_e35496);
        (assign26000_e35497, ((4.0 * locals.var_chi_b_dn0) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn2) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn6) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn7) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn10) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn11) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn12) * assign26000_e35496), ((4.0 * locals.var_chi_b_dn17) * assign26000_e35496),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26000_e35499;
        locals.var_tmf2_dn0 = assign26000_e35499_d_n0;
        locals.var_tmf2_dn2 = assign26000_e35499_d_n2;
        locals.var_tmf2_dn6 = assign26000_e35499_d_n6;
        locals.var_tmf2_dn7 = assign26000_e35499_d_n7;
        locals.var_tmf2_dn10 = assign26000_e35499_d_n10;
        locals.var_tmf2_dn11 = assign26000_e35499_d_n11;
        locals.var_tmf2_dn12 = assign26000_e35499_d_n12;
        locals.var_tmf2_dn17 = assign26000_e35499_d_n17;

        let (assign26010_e35516, assign26010_e35516_d_n0, assign26010_e35516_d_n2, assign26010_e35516_d_n6, assign26010_e35516_d_n7, assign26010_e35516_d_n10, assign26010_e35516_d_n11, assign26010_e35516_d_n12, assign26010_e35516_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let (assign26010_e35514, assign26010_e35514_d_n0, assign26010_e35514_d_n2, assign26010_e35514_d_n6, assign26010_e35514_d_n7, assign26010_e35514_d_n10, assign26010_e35514_d_n11, assign26010_e35514_d_n12, assign26010_e35514_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign26010_e35513: f64 = (-locals.var_tmf2);
                (assign26010_e35513, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign26010_e35514, assign26010_e35514_d_n0, assign26010_e35514_d_n2, assign26010_e35514_d_n6, assign26010_e35514_d_n7, assign26010_e35514_d_n10, assign26010_e35514_d_n11, assign26010_e35514_d_n12, assign26010_e35514_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26010_e35516;
        locals.var_tmf2_dn0 = assign26010_e35516_d_n0;
        locals.var_tmf2_dn2 = assign26010_e35516_d_n2;
        locals.var_tmf2_dn6 = assign26010_e35516_d_n6;
        locals.var_tmf2_dn7 = assign26010_e35516_d_n7;
        locals.var_tmf2_dn10 = assign26010_e35516_d_n10;
        locals.var_tmf2_dn11 = assign26010_e35516_d_n11;
        locals.var_tmf2_dn12 = assign26010_e35516_d_n12;
        locals.var_tmf2_dn17 = assign26010_e35516_d_n17;

        let (assign26020_e35532, assign26020_e35532_d_n0, assign26020_e35532_d_n2, assign26020_e35532_d_n6, assign26020_e35532_d_n7, assign26020_e35532_d_n10, assign26020_e35532_d_n11, assign26020_e35532_d_n12, assign26020_e35532_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26020_e35527: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign26020_e35529: f64 = (assign26020_e35527 + locals.var_tmf2);
        let assign26020_e35530: f64 = (assign26020_e35529).sqrt();
        (assign26020_e35530, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign26020_e35530)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign26020_e35530)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26020_e35532;
        locals.var_tmf2_dn0 = assign26020_e35532_d_n0;
        locals.var_tmf2_dn2 = assign26020_e35532_d_n2;
        locals.var_tmf2_dn6 = assign26020_e35532_d_n6;
        locals.var_tmf2_dn7 = assign26020_e35532_d_n7;
        locals.var_tmf2_dn10 = assign26020_e35532_d_n10;
        locals.var_tmf2_dn11 = assign26020_e35532_d_n11;
        locals.var_tmf2_dn12 = assign26020_e35532_d_n12;
        locals.var_tmf2_dn17 = assign26020_e35532_d_n17;

    }

    pub(super) fn stamp_transient_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26030_e35549, assign26030_e35549_d_n0, assign26030_e35549_d_n2, assign26030_e35549_d_n6, assign26030_e35549_d_n7, assign26030_e35549_d_n10, assign26030_e35549_d_n11, assign26030_e35549_d_n12, assign26030_e35549_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26030_e35545: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign26030_e35546: f64 = (1.0 + assign26030_e35545);
        let assign26030_e35547: f64 = (0.5 * assign26030_e35546);
        (assign26030_e35547, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26030_e35549;
        locals.var_t1__blk771_dn0 = assign26030_e35549_d_n0;
        locals.var_t1__blk771_dn2 = assign26030_e35549_d_n2;
        locals.var_t1__blk771_dn6 = assign26030_e35549_d_n6;
        locals.var_t1__blk771_dn7 = assign26030_e35549_d_n7;
        locals.var_t1__blk771_dn10 = assign26030_e35549_d_n10;
        locals.var_t1__blk771_dn11 = assign26030_e35549_d_n11;
        locals.var_t1__blk771_dn12 = assign26030_e35549_d_n12;
        locals.var_t1__blk771_dn17 = assign26030_e35549_d_n17;

        let (assign26040_e35572, assign26040_e35572_d_n0, assign26040_e35572_d_n2, assign26040_e35572_d_n6, assign26040_e35572_d_n7, assign26040_e35572_d_n10, assign26040_e35572_d_n11, assign26040_e35572_d_n12, assign26040_e35572_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26040_e35563: f64 = (2.0 * 0.0008);
        let assign26040_e35565: f64 = (assign26040_e35563 * 75.0);
        let assign26040_e35566: f64 = (locals.var_tmf1 + assign26040_e35565);
        let assign26040_e35568: f64 = (assign26040_e35566 / locals.var_tmf2);
        let assign26040_e35569: f64 = (1.0 - assign26040_e35568);
        let assign26040_e35570: f64 = (0.5 * assign26040_e35569);
        (assign26040_e35570, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign26040_e35566 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign26040_e35572;
        locals.var_t2__blk772_dn0 = assign26040_e35572_d_n0;
        locals.var_t2__blk772_dn2 = assign26040_e35572_d_n2;
        locals.var_t2__blk772_dn6 = assign26040_e35572_d_n6;
        locals.var_t2__blk772_dn7 = assign26040_e35572_d_n7;
        locals.var_t2__blk772_dn10 = assign26040_e35572_d_n10;
        locals.var_t2__blk772_dn11 = assign26040_e35572_d_n11;
        locals.var_t2__blk772_dn12 = assign26040_e35572_d_n12;
        locals.var_t2__blk772_dn17 = assign26040_e35572_d_n17;

        let (assign26050_e35589, assign26050_e35589_d_n0, assign26050_e35589_d_n2, assign26050_e35589_d_n6, assign26050_e35589_d_n7, assign26050_e35589_d_n10, assign26050_e35589_d_n11, assign26050_e35589_d_n12, assign26050_e35589_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26050_e35585: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign26050_e35586: f64 = (0.5 * assign26050_e35585);
        let assign26050_e35587: f64 = (locals.var_chi_b - assign26050_e35586);
        (assign26050_e35587, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign26050_e35589;
        locals.var_chi__blk814_dn0 = assign26050_e35589_d_n0;
        locals.var_chi__blk814_dn2 = assign26050_e35589_d_n2;
        locals.var_chi__blk814_dn6 = assign26050_e35589_d_n6;
        locals.var_chi__blk814_dn7 = assign26050_e35589_d_n7;
        locals.var_chi__blk814_dn10 = assign26050_e35589_d_n10;
        locals.var_chi__blk814_dn11 = assign26050_e35589_d_n11;
        locals.var_chi__blk814_dn12 = assign26050_e35589_d_n12;
        locals.var_chi__blk814_dn17 = assign26050_e35589_d_n17;

        let (assign26060_e35604, assign26060_e35604_d_n0, assign26060_e35604_d_n2, assign26060_e35604_d_n6, assign26060_e35604_d_n7, assign26060_e35604_d_n10, assign26060_e35604_d_n11, assign26060_e35604_d_n12, assign26060_e35604_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26060_e35600: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign26060_e35602: f64 = (assign26060_e35600 - locals.var_vxbgmtcl);
        (assign26060_e35602, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign26060_e35604;
        locals.var_ps0ld_dn0 = assign26060_e35604_d_n0;
        locals.var_ps0ld_dn2 = assign26060_e35604_d_n2;
        locals.var_ps0ld_dn6 = assign26060_e35604_d_n6;
        locals.var_ps0ld_dn7 = assign26060_e35604_d_n7;
        locals.var_ps0ld_dn10 = assign26060_e35604_d_n10;
        locals.var_ps0ld_dn11 = assign26060_e35604_d_n11;
        locals.var_ps0ld_dn12 = assign26060_e35604_d_n12;
        locals.var_ps0ld_dn17 = assign26060_e35604_d_n17;

        let (assign26070_e35621, assign26070_e35621_d_n0, assign26070_e35621_d_n2, assign26070_e35621_d_n6, assign26070_e35621_d_n7, assign26070_e35621_d_n10, assign26070_e35621_d_n11, assign26070_e35621_d_n12, assign26070_e35621_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26070_e35615: f64 = (locals.var_chi__blk814 - 1.0);
        let assign26070_e35617: f64 = (-locals.var_chi__blk814);
        let assign26070_e35618: f64 = (assign26070_e35617).exp();
        let assign26070_e35619: f64 = (assign26070_e35615 + assign26070_e35618);
        (assign26070_e35619, (locals.var_chi__blk814_dn0 + (assign26070_e35618 * (-locals.var_chi__blk814_dn0))), (locals.var_chi__blk814_dn2 + (assign26070_e35618 * (-locals.var_chi__blk814_dn2))), (locals.var_chi__blk814_dn6 + (assign26070_e35618 * (-locals.var_chi__blk814_dn6))), (locals.var_chi__blk814_dn7 + (assign26070_e35618 * (-locals.var_chi__blk814_dn7))), (locals.var_chi__blk814_dn10 + (assign26070_e35618 * (-locals.var_chi__blk814_dn10))), (locals.var_chi__blk814_dn11 + (assign26070_e35618 * (-locals.var_chi__blk814_dn11))), (locals.var_chi__blk814_dn12 + (assign26070_e35618 * (-locals.var_chi__blk814_dn12))), (locals.var_chi__blk814_dn17 + (assign26070_e35618 * (-locals.var_chi__blk814_dn17))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26070_e35621;
        locals.var_t1__blk771_dn0 = assign26070_e35621_d_n0;
        locals.var_t1__blk771_dn2 = assign26070_e35621_d_n2;
        locals.var_t1__blk771_dn6 = assign26070_e35621_d_n6;
        locals.var_t1__blk771_dn7 = assign26070_e35621_d_n7;
        locals.var_t1__blk771_dn10 = assign26070_e35621_d_n10;
        locals.var_t1__blk771_dn11 = assign26070_e35621_d_n11;
        locals.var_t1__blk771_dn12 = assign26070_e35621_d_n12;
        locals.var_t1__blk771_dn17 = assign26070_e35621_d_n17;

        let assign26080_e35625: f64 = (10.0 * 2.220446049250313e-16);
        let assign26080_e35626: f64 = if locals.var_t1__blk771 < assign26080_e35625 { 1.0 } else { 0.0 };
        locals.var_guard858 = assign26080_e35626;

        let (assign26090_e35641, assign26090_e35641_d_n0, assign26090_e35641_d_n2, assign26090_e35641_d_n6, assign26090_e35641_d_n7, assign26090_e35641_d_n10, assign26090_e35641_d_n11, assign26090_e35641_d_n12, assign26090_e35641_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard858 != 0.0)) {
        let assign26090_e35639: f64 = (10.0 * 2.220446049250313e-16);
        (assign26090_e35639, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26090_e35641;
        locals.var_t1__blk771_dn0 = assign26090_e35641_d_n0;
        locals.var_t1__blk771_dn2 = assign26090_e35641_d_n2;
        locals.var_t1__blk771_dn6 = assign26090_e35641_d_n6;
        locals.var_t1__blk771_dn7 = assign26090_e35641_d_n7;
        locals.var_t1__blk771_dn10 = assign26090_e35641_d_n10;
        locals.var_t1__blk771_dn11 = assign26090_e35641_d_n11;
        locals.var_t1__blk771_dn12 = assign26090_e35641_d_n12;
        locals.var_t1__blk771_dn17 = assign26090_e35641_d_n17;

        let (assign26100_e35653, assign26100_e35653_d_n0, assign26100_e35653_d_n2, assign26100_e35653_d_n6, assign26100_e35653_d_n7, assign26100_e35653_d_n10, assign26100_e35653_d_n11, assign26100_e35653_d_n12, assign26100_e35653_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26100_e35651: f64 = (locals.var_t1__blk771).sqrt();
        (assign26100_e35651, (locals.var_t1__blk771_dn0 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn2 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn6 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn7 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn10 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn11 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn12 / (2.0 * assign26100_e35651)), (locals.var_t1__blk771_dn17 / (2.0 * assign26100_e35651)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign26100_e35653;
        locals.var_t2__blk772_dn0 = assign26100_e35653_d_n0;
        locals.var_t2__blk772_dn2 = assign26100_e35653_d_n2;
        locals.var_t2__blk772_dn6 = assign26100_e35653_d_n6;
        locals.var_t2__blk772_dn7 = assign26100_e35653_d_n7;
        locals.var_t2__blk772_dn10 = assign26100_e35653_d_n10;
        locals.var_t2__blk772_dn11 = assign26100_e35653_d_n11;
        locals.var_t2__blk772_dn12 = assign26100_e35653_d_n12;
        locals.var_t2__blk772_dn17 = assign26100_e35653_d_n17;

        let (assign26110_e35666, assign26110_e35666_d_n0, assign26110_e35666_d_n2, assign26110_e35666_d_n6, assign26110_e35666_d_n7, assign26110_e35666_d_n10, assign26110_e35666_d_n11, assign26110_e35666_d_n12, assign26110_e35666_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26110_e35664: f64 = (locals.var_cnst0over * locals.var_t2__blk772);
        (assign26110_e35664, ((locals.var_cnst0over_dn0 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26110_e35666;
        locals.var_qbuld_dn0 = assign26110_e35666_d_n0;
        locals.var_qbuld_dn2 = assign26110_e35666_d_n2;
        locals.var_qbuld_dn6 = assign26110_e35666_d_n6;
        locals.var_qbuld_dn7 = assign26110_e35666_d_n7;
        locals.var_qbuld_dn10 = assign26110_e35666_d_n10;
        locals.var_qbuld_dn11 = assign26110_e35666_d_n11;
        locals.var_qbuld_dn12 = assign26110_e35666_d_n12;
        locals.var_qbuld_dn17 = assign26110_e35666_d_n17;

        let (assign26120_e35681, assign26120_e35681_d_n0, assign26120_e35681_d_n2, assign26120_e35681_d_n6, assign26120_e35681_d_n7, assign26120_e35681_d_n10, assign26120_e35681_d_n11, assign26120_e35681_d_n12, assign26120_e35681_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) {
        let assign26120_e35678: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26120_e35679: f64 = (locals.var_cox0 * assign26120_e35678);
        (assign26120_e35679, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26120_e35681;
        locals.var_qsuld_dn0 = assign26120_e35681_d_n0;
        locals.var_qsuld_dn2 = assign26120_e35681_d_n2;
        locals.var_qsuld_dn6 = assign26120_e35681_d_n6;
        locals.var_qsuld_dn7 = assign26120_e35681_d_n7;
        locals.var_qsuld_dn10 = assign26120_e35681_d_n10;
        locals.var_qsuld_dn11 = assign26120_e35681_d_n11;
        locals.var_qsuld_dn12 = assign26120_e35681_d_n12;
        locals.var_qsuld_dn17 = assign26120_e35681_d_n17;

        let assign26130_e35684: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard859 = assign26130_e35684;

        let (assign26140_e35701, assign26140_e35701_d_n0, assign26140_e35701_d_n2, assign26140_e35701_d_n6, assign26140_e35701_d_n7, assign26140_e35701_d_n10, assign26140_e35701_d_n11, assign26140_e35701_d_n12, assign26140_e35701_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26140_e35697: f64 = (-locals.var_vxbgmtcl);
        let assign26140_e35698: f64 = (locals.var_beta * assign26140_e35697);
        let assign26140_e35699: f64 = (assign26140_e35698).exp();
        (assign26140_e35699, (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign26140_e35699 * ((locals.var_beta_dn10 * assign26140_e35697) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign26140_e35699 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign26140_e35701;
        locals.var_exp_bvbs__blk833_dn0 = assign26140_e35701_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign26140_e35701_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign26140_e35701_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign26140_e35701_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign26140_e35701_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign26140_e35701_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign26140_e35701_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign26140_e35701_d_n17;

        let (assign26150_e35716, assign26150_e35716_d_n0, assign26150_e35716_d_n2, assign26150_e35716_d_n6, assign26150_e35716_d_n7, assign26150_e35716_d_n10, assign26150_e35716_d_n11, assign26150_e35716_d_n12, assign26150_e35716_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26150_e35714: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign26150_e35714, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign26150_e35716;
        locals.var_t0__blk770_dn0 = assign26150_e35716_d_n0;
        locals.var_t0__blk770_dn2 = assign26150_e35716_d_n2;
        locals.var_t0__blk770_dn6 = assign26150_e35716_d_n6;
        locals.var_t0__blk770_dn7 = assign26150_e35716_d_n7;
        locals.var_t0__blk770_dn10 = assign26150_e35716_d_n10;
        locals.var_t0__blk770_dn11 = assign26150_e35716_d_n11;
        locals.var_t0__blk770_dn12 = assign26150_e35716_d_n12;
        locals.var_t0__blk770_dn17 = assign26150_e35716_d_n17;

        let (assign26160_e35731, assign26160_e35731_d_n0, assign26160_e35731_d_n2, assign26160_e35731_d_n6, assign26160_e35731_d_n7, assign26160_e35731_d_n10, assign26160_e35731_d_n11, assign26160_e35731_d_n12, assign26160_e35731_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26160_e35729: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign26160_e35729, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign26160_e35731;
        locals.var_cnst1over_dn0 = assign26160_e35731_d_n0;
        locals.var_cnst1over_dn2 = assign26160_e35731_d_n2;
        locals.var_cnst1over_dn6 = assign26160_e35731_d_n6;
        locals.var_cnst1over_dn7 = assign26160_e35731_d_n7;
        locals.var_cnst1over_dn10 = assign26160_e35731_d_n10;
        locals.var_cnst1over_dn11 = assign26160_e35731_d_n11;
        locals.var_cnst1over_dn12 = assign26160_e35731_d_n12;
        locals.var_cnst1over_dn17 = assign26160_e35731_d_n17;

        let (assign26170_e35746, assign26170_e35746_d_n0, assign26170_e35746_d_n2, assign26170_e35746_d_n6, assign26170_e35746_d_n7, assign26170_e35746_d_n10, assign26170_e35746_d_n11, assign26170_e35746_d_n12, assign26170_e35746_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26170_e35744: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign26170_e35744, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_cfs1__blk842, locals.var_cfs1__blk842_dn0, locals.var_cfs1__blk842_dn2, locals.var_cfs1__blk842_dn6, locals.var_cfs1__blk842_dn7, locals.var_cfs1__blk842_dn10, locals.var_cfs1__blk842_dn11, locals.var_cfs1__blk842_dn12, locals.var_cfs1__blk842_dn17,)
    }
};
        locals.var_cfs1__blk842 = assign26170_e35746;
        locals.var_cfs1__blk842_dn0 = assign26170_e35746_d_n0;
        locals.var_cfs1__blk842_dn2 = assign26170_e35746_d_n2;
        locals.var_cfs1__blk842_dn6 = assign26170_e35746_d_n6;
        locals.var_cfs1__blk842_dn7 = assign26170_e35746_d_n7;
        locals.var_cfs1__blk842_dn10 = assign26170_e35746_d_n10;
        locals.var_cfs1__blk842_dn11 = assign26170_e35746_d_n11;
        locals.var_cfs1__blk842_dn12 = assign26170_e35746_d_n12;
        locals.var_cfs1__blk842_dn17 = assign26170_e35746_d_n17;

        let (assign26180_e35759,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk787,)
    }
};
        locals.var_flg_conv__blk787 = assign26180_e35759;

        let (assign26190_e35772,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign26190_e35772;

    }

    pub(super) fn stamp_transient_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign26200_loop_guard: usize = 0;
        while {
            let assign26200_cond_e35786: f64 = (2.0 * 20.0);
            let assign26200_cond_e35788: f64 = (assign26200_cond_e35786 + 1.0);
            let assign26200_cond_e35790: f64 = if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_lp_s0 <= assign26200_cond_e35788)) { 1.0 } else { 0.0 };
            assign26200_cond_e35790 != 0.0
        } {
            assign26200_loop_guard += 1;
            assert!(assign26200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26200_body0_e35803, assign26200_body0_e35803_d_n0, assign26200_body0_e35803_d_n2, assign26200_body0_e35803_d_n6, assign26200_body0_e35803_d_n7, assign26200_body0_e35803_d_n10, assign26200_body0_e35803_d_n11, assign26200_body0_e35803_d_n12, assign26200_body0_e35803_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk838, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    }
};
            locals.var_fb__blk838 = assign26200_body0_e35803;
            locals.var_fb__blk838_dn0 = assign26200_body0_e35803_d_n0;
            locals.var_fb__blk838_dn2 = assign26200_body0_e35803_d_n2;
            locals.var_fb__blk838_dn6 = assign26200_body0_e35803_d_n6;
            locals.var_fb__blk838_dn7 = assign26200_body0_e35803_d_n7;
            locals.var_fb__blk838_dn10 = assign26200_body0_e35803_d_n10;
            locals.var_fb__blk838_dn11 = assign26200_body0_e35803_d_n11;
            locals.var_fb__blk838_dn12 = assign26200_body0_e35803_d_n12;
            locals.var_fb__blk838_dn17 = assign26200_body0_e35803_d_n17;
            let (assign26200_body1_e35820, assign26200_body1_e35820_d_n0, assign26200_body1_e35820_d_n2, assign26200_body1_e35820_d_n6, assign26200_body1_e35820_d_n7, assign26200_body1_e35820_d_n10, assign26200_body1_e35820_d_n11, assign26200_body1_e35820_d_n12, assign26200_body1_e35820_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26200_body1_e35817: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign26200_body1_e35818: f64 = (locals.var_beta * assign26200_body1_e35817);
        (assign26200_body1_e35818, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26200_body1_e35817) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
            locals.var_chi__blk814 = assign26200_body1_e35820;
            locals.var_chi__blk814_dn0 = assign26200_body1_e35820_d_n0;
            locals.var_chi__blk814_dn2 = assign26200_body1_e35820_d_n2;
            locals.var_chi__blk814_dn6 = assign26200_body1_e35820_d_n6;
            locals.var_chi__blk814_dn7 = assign26200_body1_e35820_d_n7;
            locals.var_chi__blk814_dn10 = assign26200_body1_e35820_d_n10;
            locals.var_chi__blk814_dn11 = assign26200_body1_e35820_d_n11;
            locals.var_chi__blk814_dn12 = assign26200_body1_e35820_d_n12;
            locals.var_chi__blk814_dn17 = assign26200_body1_e35820_d_n17;
            let assign26200_body2_e35823: f64 = if locals.var_chi__blk814 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard860 = assign26200_body2_e35823;
            let (assign26200_body3_e35853, assign26200_body3_e35853_d_n0, assign26200_body3_e35853_d_n2, assign26200_body3_e35853_d_n6, assign26200_body3_e35853_d_n7, assign26200_body3_e35853_d_n10, assign26200_body3_e35853_d_n11, assign26200_body3_e35853_d_n12, assign26200_body3_e35853_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body3_e35838: f64 = (locals.var_chi__blk814 * locals.var_chi__blk814);
        let assign26200_body3_e35840: f64 = (assign26200_body3_e35838 * locals.var_chi__blk814);
        let assign26200_body3_e35844: f64 = (-0.07053654284009761);
        let assign26200_body3_e35847: f64 = (locals.var_chi__blk814 * 0.006115288895133179);
        let assign26200_body3_e35848: f64 = (assign26200_body3_e35844 + assign26200_body3_e35847);
        let assign26200_body3_e35849: f64 = (locals.var_chi__blk814 * assign26200_body3_e35848);
        let assign26200_body3_e35850: f64 = (0.29693154855771 + assign26200_body3_e35849);
        let assign26200_body3_e35851: f64 = (assign26200_body3_e35840 * assign26200_body3_e35850);
        (assign26200_body3_e35851, ((((((locals.var_chi__blk814_dn0 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn0)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn0)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn0 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn2 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn2)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn2)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn2 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn6 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn6)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn6)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn6 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn7 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn7)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn7)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn7 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn10 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn10)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn10)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn10 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn11 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn11)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn11)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn11 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn12 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn12)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn12)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn12 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn17 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn17)) * locals.var_chi__blk814) + (assign26200_body3_e35838 * locals.var_chi__blk814_dn17)) * assign26200_body3_e35850) + (assign26200_body3_e35840 * ((locals.var_chi__blk814_dn17 * assign26200_body3_e35848) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign26200_body3_e35853;
            locals.var_fi_dn0 = assign26200_body3_e35853_d_n0;
            locals.var_fi_dn2 = assign26200_body3_e35853_d_n2;
            locals.var_fi_dn6 = assign26200_body3_e35853_d_n6;
            locals.var_fi_dn7 = assign26200_body3_e35853_d_n7;
            locals.var_fi_dn10 = assign26200_body3_e35853_d_n10;
            locals.var_fi_dn11 = assign26200_body3_e35853_d_n11;
            locals.var_fi_dn12 = assign26200_body3_e35853_d_n12;
            locals.var_fi_dn17 = assign26200_body3_e35853_d_n17;
            let (assign26200_body4_e35887, assign26200_body4_e35887_d_n0, assign26200_body4_e35887_d_n2, assign26200_body4_e35887_d_n6, assign26200_body4_e35887_d_n7, assign26200_body4_e35887_d_n10, assign26200_body4_e35887_d_n11, assign26200_body4_e35887_d_n12, assign26200_body4_e35887_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body4_e35868: f64 = (locals.var_chi__blk814 * locals.var_chi__blk814);
        let assign26200_body4_e35871: f64 = (3.0 * 0.29693154855771);
        let assign26200_body4_e35875: f64 = (-0.07053654284009761);
        let assign26200_body4_e35876: f64 = (4.0 * assign26200_body4_e35875);
        let assign26200_body4_e35879: f64 = (locals.var_chi__blk814 * 5.0);
        let assign26200_body4_e35881: f64 = (assign26200_body4_e35879 * 0.006115288895133179);
        let assign26200_body4_e35882: f64 = (assign26200_body4_e35876 + assign26200_body4_e35881);
        let assign26200_body4_e35883: f64 = (locals.var_chi__blk814 * assign26200_body4_e35882);
        let assign26200_body4_e35884: f64 = (assign26200_body4_e35871 + assign26200_body4_e35883);
        let assign26200_body4_e35885: f64 = (assign26200_body4_e35868 * assign26200_body4_e35884);
        (assign26200_body4_e35885, ((((locals.var_chi__blk814_dn0 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn0)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn0 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn2 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn2)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn2 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn6 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn6)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn6 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn7 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn7)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn7 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn10 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn10)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn10 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn11 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn11)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn11 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn12 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn12)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn12 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn17 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn17)) * assign26200_body4_e35884) + (assign26200_body4_e35868 * ((locals.var_chi__blk814_dn17 * assign26200_body4_e35882) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign26200_body4_e35887;
            locals.var_fi_dchi_dn0 = assign26200_body4_e35887_d_n0;
            locals.var_fi_dchi_dn2 = assign26200_body4_e35887_d_n2;
            locals.var_fi_dchi_dn6 = assign26200_body4_e35887_d_n6;
            locals.var_fi_dchi_dn7 = assign26200_body4_e35887_d_n7;
            locals.var_fi_dchi_dn10 = assign26200_body4_e35887_d_n10;
            locals.var_fi_dchi_dn11 = assign26200_body4_e35887_d_n11;
            locals.var_fi_dchi_dn12 = assign26200_body4_e35887_d_n12;
            locals.var_fi_dchi_dn17 = assign26200_body4_e35887_d_n17;
            let (assign26200_body5_e35906, assign26200_body5_e35906_d_n0, assign26200_body5_e35906_d_n2, assign26200_body5_e35906_d_n6, assign26200_body5_e35906_d_n7, assign26200_body5_e35906_d_n10, assign26200_body5_e35906_d_n11, assign26200_body5_e35906_d_n12, assign26200_body5_e35906_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body5_e35902: f64 = (locals.var_cfs1__blk842 * locals.var_fi);
        let assign26200_body5_e35904: f64 = (assign26200_body5_e35902 * locals.var_fi);
        (assign26200_body5_e35904, ((((locals.var_cfs1__blk842_dn0 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn0)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk842_dn2 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn2)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk842_dn6 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn6)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk842_dn7 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn7)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk842_dn10 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn10)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk842_dn11 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn11)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk842_dn12 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn12)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk842_dn17 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn17)) * locals.var_fi) + (assign26200_body5_e35902 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign26200_body5_e35906;
            locals.var_fs01__blk836_dn0 = assign26200_body5_e35906_d_n0;
            locals.var_fs01__blk836_dn2 = assign26200_body5_e35906_d_n2;
            locals.var_fs01__blk836_dn6 = assign26200_body5_e35906_d_n6;
            locals.var_fs01__blk836_dn7 = assign26200_body5_e35906_d_n7;
            locals.var_fs01__blk836_dn10 = assign26200_body5_e35906_d_n10;
            locals.var_fs01__blk836_dn11 = assign26200_body5_e35906_d_n11;
            locals.var_fs01__blk836_dn12 = assign26200_body5_e35906_d_n12;
            locals.var_fs01__blk836_dn17 = assign26200_body5_e35906_d_n17;
            let (assign26200_body6_e35929, assign26200_body6_e35929_d_n0, assign26200_body6_e35929_d_n2, assign26200_body6_e35929_d_n6, assign26200_body6_e35929_d_n7, assign26200_body6_e35929_d_n10, assign26200_body6_e35929_d_n11, assign26200_body6_e35929_d_n12, assign26200_body6_e35929_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body6_e35921: f64 = (locals.var_cfs1__blk842 * locals.var_beta);
        let assign26200_body6_e35923: f64 = (assign26200_body6_e35921 * 2.0);
        let assign26200_body6_e35925: f64 = (assign26200_body6_e35923 * locals.var_fi);
        let assign26200_body6_e35927: f64 = (assign26200_body6_e35925 * locals.var_fi_dchi);
        (assign26200_body6_e35927, ((((((locals.var_cfs1__blk842_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk842_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk842_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk842_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk842_dn10 * locals.var_beta) + (locals.var_cfs1__blk842 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk842_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk842_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk842_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26200_body6_e35923 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign26200_body6_e35925 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign26200_body6_e35929;
            locals.var_fs01_dps0__blk837_dn0 = assign26200_body6_e35929_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign26200_body6_e35929_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign26200_body6_e35929_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign26200_body6_e35929_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign26200_body6_e35929_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign26200_body6_e35929_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign26200_body6_e35929_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign26200_body6_e35929_d_n17;
            let (assign26200_body7_e35964, assign26200_body7_e35964_d_n0, assign26200_body7_e35964_d_n2, assign26200_body7_e35964_d_n6, assign26200_body7_e35964_d_n7, assign26200_body7_e35964_d_n10, assign26200_body7_e35964_d_n11, assign26200_body7_e35964_d_n12, assign26200_body7_e35964_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body7_e35946: f64 = (-0.117851130197758);
        let assign26200_body7_e35951: f64 = (-0.00163730162779191);
        let assign26200_body7_e35954: f64 = (locals.var_chi__blk814 * 6.36964918866352e-5);
        let assign26200_body7_e35955: f64 = (assign26200_body7_e35951 + assign26200_body7_e35954);
        let assign26200_body7_e35956: f64 = (locals.var_chi__blk814 * assign26200_body7_e35955);
        let assign26200_body7_e35957: f64 = (0.0178800506338833 + assign26200_body7_e35956);
        let assign26200_body7_e35958: f64 = (locals.var_chi__blk814 * assign26200_body7_e35957);
        let assign26200_body7_e35959: f64 = (assign26200_body7_e35946 + assign26200_body7_e35958);
        let assign26200_body7_e35960: f64 = (locals.var_chi__blk814 * assign26200_body7_e35959);
        let assign26200_body7_e35961: f64 = (0.707106781186548 + assign26200_body7_e35960);
        let assign26200_body7_e35962: f64 = (locals.var_chi__blk814 * assign26200_body7_e35961);
        (assign26200_body7_e35962, ((locals.var_chi__blk814_dn0 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn2 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn6 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn7 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn10 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn11 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn12 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn17 * assign26200_body7_e35961) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign26200_body7_e35959) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign26200_body7_e35957) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign26200_body7_e35955) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk838, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    }
};
            locals.var_fb__blk838 = assign26200_body7_e35964;
            locals.var_fb__blk838_dn0 = assign26200_body7_e35964_d_n0;
            locals.var_fb__blk838_dn2 = assign26200_body7_e35964_d_n2;
            locals.var_fb__blk838_dn6 = assign26200_body7_e35964_d_n6;
            locals.var_fb__blk838_dn7 = assign26200_body7_e35964_d_n7;
            locals.var_fb__blk838_dn10 = assign26200_body7_e35964_d_n10;
            locals.var_fb__blk838_dn11 = assign26200_body7_e35964_d_n11;
            locals.var_fb__blk838_dn12 = assign26200_body7_e35964_d_n12;
            locals.var_fb__blk838_dn17 = assign26200_body7_e35964_d_n17;
            let (assign26200_body8_e36005, assign26200_body8_e36005_d_n0, assign26200_body8_e36005_d_n2, assign26200_body8_e36005_d_n6, assign26200_body8_e36005_d_n7, assign26200_body8_e36005_d_n10, assign26200_body8_e36005_d_n11, assign26200_body8_e36005_d_n12, assign26200_body8_e36005_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body8_e35981: f64 = (-0.117851130197758);
        let assign26200_body8_e35982: f64 = (2.0 * assign26200_body8_e35981);
        let assign26200_body8_e35986: f64 = (3.0 * 0.0178800506338833);
        let assign26200_body8_e35990: f64 = (-0.00163730162779191);
        let assign26200_body8_e35991: f64 = (4.0 * assign26200_body8_e35990);
        let assign26200_body8_e35994: f64 = (locals.var_chi__blk814 * 5.0);
        let assign26200_body8_e35996: f64 = (assign26200_body8_e35994 * 6.36964918866352e-5);
        let assign26200_body8_e35997: f64 = (assign26200_body8_e35991 + assign26200_body8_e35996);
        let assign26200_body8_e35998: f64 = (locals.var_chi__blk814 * assign26200_body8_e35997);
        let assign26200_body8_e35999: f64 = (assign26200_body8_e35986 + assign26200_body8_e35998);
        let assign26200_body8_e36000: f64 = (locals.var_chi__blk814 * assign26200_body8_e35999);
        let assign26200_body8_e36001: f64 = (assign26200_body8_e35982 + assign26200_body8_e36000);
        let assign26200_body8_e36002: f64 = (locals.var_chi__blk814 * assign26200_body8_e36001);
        let assign26200_body8_e36003: f64 = (0.707106781186548 + assign26200_body8_e36002);
        (assign26200_body8_e36003, ((locals.var_chi__blk814_dn0 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn2 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn6 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn7 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn10 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn11 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn12 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn17 * assign26200_body8_e36001) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign26200_body8_e35999) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign26200_body8_e35997) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign26200_body8_e36005;
            locals.var_fb_dchi_dn0 = assign26200_body8_e36005_d_n0;
            locals.var_fb_dchi_dn2 = assign26200_body8_e36005_d_n2;
            locals.var_fb_dchi_dn6 = assign26200_body8_e36005_d_n6;
            locals.var_fb_dchi_dn7 = assign26200_body8_e36005_d_n7;
            locals.var_fb_dchi_dn10 = assign26200_body8_e36005_d_n10;
            locals.var_fb_dchi_dn11 = assign26200_body8_e36005_d_n11;
            locals.var_fb_dchi_dn12 = assign26200_body8_e36005_d_n12;
            locals.var_fb_dchi_dn17 = assign26200_body8_e36005_d_n17;
            let (assign26200_body9_e36027, assign26200_body9_e36027_d_n0, assign26200_body9_e36027_d_n2, assign26200_body9_e36027_d_n6, assign26200_body9_e36027_d_n7, assign26200_body9_e36027_d_n10, assign26200_body9_e36027_d_n11, assign26200_body9_e36027_d_n12, assign26200_body9_e36027_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body9_e36020: f64 = (locals.var_fb__blk838 * locals.var_fb__blk838);
        let assign26200_body9_e36022: f64 = (assign26200_body9_e36020 + locals.var_fs01__blk836);
        let assign26200_body9_e36024: f64 = (assign26200_body9_e36022 + 1e-50);
        let assign26200_body9_e36025: f64 = (assign26200_body9_e36024).sqrt();
        (assign26200_body9_e36025, ((((locals.var_fb__blk838_dn0 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn0)) + locals.var_fs01__blk836_dn0) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn2 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn2)) + locals.var_fs01__blk836_dn2) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn6 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn6)) + locals.var_fs01__blk836_dn6) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn7 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn7)) + locals.var_fs01__blk836_dn7) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn10 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn10)) + locals.var_fs01__blk836_dn10) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn11 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn11)) + locals.var_fs01__blk836_dn11) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn12 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn12)) + locals.var_fs01__blk836_dn12) / (2.0 * assign26200_body9_e36025)), ((((locals.var_fb__blk838_dn17 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn17)) + locals.var_fs01__blk836_dn17) / (2.0 * assign26200_body9_e36025)),)
    } else {
        (locals.var_fs02__blk840, locals.var_fs02__blk840_dn0, locals.var_fs02__blk840_dn2, locals.var_fs02__blk840_dn6, locals.var_fs02__blk840_dn7, locals.var_fs02__blk840_dn10, locals.var_fs02__blk840_dn11, locals.var_fs02__blk840_dn12, locals.var_fs02__blk840_dn17,)
    }
};
            locals.var_fs02__blk840 = assign26200_body9_e36027;
            locals.var_fs02__blk840_dn0 = assign26200_body9_e36027_d_n0;
            locals.var_fs02__blk840_dn2 = assign26200_body9_e36027_d_n2;
            locals.var_fs02__blk840_dn6 = assign26200_body9_e36027_d_n6;
            locals.var_fs02__blk840_dn7 = assign26200_body9_e36027_d_n7;
            locals.var_fs02__blk840_dn10 = assign26200_body9_e36027_d_n10;
            locals.var_fs02__blk840_dn11 = assign26200_body9_e36027_d_n11;
            locals.var_fs02__blk840_dn12 = assign26200_body9_e36027_d_n12;
            locals.var_fs02__blk840_dn17 = assign26200_body9_e36027_d_n17;
            let (assign26200_body10_e36054, assign26200_body10_e36054_d_n0, assign26200_body10_e36054_d_n2, assign26200_body10_e36054_d_n6, assign26200_body10_e36054_d_n7, assign26200_body10_e36054_d_n10, assign26200_body10_e36054_d_n11, assign26200_body10_e36054_d_n12, assign26200_body10_e36054_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26200_body10_e36042: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign26200_body10_e36044: f64 = (assign26200_body10_e36042 * 2.0);
        let assign26200_body10_e36046: f64 = (assign26200_body10_e36044 * locals.var_fb__blk838);
        let assign26200_body10_e36048: f64 = (assign26200_body10_e36046 + locals.var_fs01_dps0__blk837);
        let assign26200_body10_e36051: f64 = (locals.var_fs02__blk840 + locals.var_fs02__blk840);
        let assign26200_body10_e36052: f64 = (assign26200_body10_e36048 / assign26200_body10_e36051);
        (assign26200_body10_e36052, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn0)) + locals.var_fs01_dps0__blk837_dn0) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn0 + locals.var_fs02__blk840_dn0))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn2)) + locals.var_fs01_dps0__blk837_dn2) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn2 + locals.var_fs02__blk840_dn2))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn6)) + locals.var_fs01_dps0__blk837_dn6) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn6 + locals.var_fs02__blk840_dn6))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn7)) + locals.var_fs01_dps0__blk837_dn7) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn7 + locals.var_fs02__blk840_dn7))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn10)) + locals.var_fs01_dps0__blk837_dn10) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn10 + locals.var_fs02__blk840_dn10))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn11)) + locals.var_fs01_dps0__blk837_dn11) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn11 + locals.var_fs02__blk840_dn11))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn12)) + locals.var_fs01_dps0__blk837_dn12) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn12 + locals.var_fs02__blk840_dn12))) / (assign26200_body10_e36051 * assign26200_body10_e36051)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk838) + (assign26200_body10_e36044 * locals.var_fb__blk838_dn17)) + locals.var_fs01_dps0__blk837_dn17) * assign26200_body10_e36051) - (assign26200_body10_e36048 * (locals.var_fs02__blk840_dn17 + locals.var_fs02__blk840_dn17))) / (assign26200_body10_e36051 * assign26200_body10_e36051)),)
    } else {
        (locals.var_fs02_dps0__blk841, locals.var_fs02_dps0__blk841_dn0, locals.var_fs02_dps0__blk841_dn2, locals.var_fs02_dps0__blk841_dn6, locals.var_fs02_dps0__blk841_dn7, locals.var_fs02_dps0__blk841_dn10, locals.var_fs02_dps0__blk841_dn11, locals.var_fs02_dps0__blk841_dn12, locals.var_fs02_dps0__blk841_dn17,)
    }
};
            locals.var_fs02_dps0__blk841 = assign26200_body10_e36054;
            locals.var_fs02_dps0__blk841_dn0 = assign26200_body10_e36054_d_n0;
            locals.var_fs02_dps0__blk841_dn2 = assign26200_body10_e36054_d_n2;
            locals.var_fs02_dps0__blk841_dn6 = assign26200_body10_e36054_d_n6;
            locals.var_fs02_dps0__blk841_dn7 = assign26200_body10_e36054_d_n7;
            locals.var_fs02_dps0__blk841_dn10 = assign26200_body10_e36054_d_n10;
            locals.var_fs02_dps0__blk841_dn11 = assign26200_body10_e36054_d_n11;
            locals.var_fs02_dps0__blk841_dn12 = assign26200_body10_e36054_d_n12;
            locals.var_fs02_dps0__blk841_dn17 = assign26200_body10_e36054_d_n17;
            let assign26200_body11_e36057: f64 = if locals.var_chi__blk814 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard861 = assign26200_body11_e36057;
            let (assign26200_body12_e36076, assign26200_body12_e36076_d_n0, assign26200_body12_e36076_d_n2, assign26200_body12_e36076_d_n6, assign26200_body12_e36076_d_n7, assign26200_body12_e36076_d_n10, assign26200_body12_e36076_d_n11, assign26200_body12_e36076_d_n12, assign26200_body12_e36076_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26200_body12_e36074: f64 = (locals.var_chi__blk814).exp();
        (assign26200_body12_e36074, (assign26200_body12_e36074 * locals.var_chi__blk814_dn0), (assign26200_body12_e36074 * locals.var_chi__blk814_dn2), (assign26200_body12_e36074 * locals.var_chi__blk814_dn6), (assign26200_body12_e36074 * locals.var_chi__blk814_dn7), (assign26200_body12_e36074 * locals.var_chi__blk814_dn10), (assign26200_body12_e36074 * locals.var_chi__blk814_dn11), (assign26200_body12_e36074 * locals.var_chi__blk814_dn12), (assign26200_body12_e36074 * locals.var_chi__blk814_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign26200_body12_e36076;
            locals.var_exp_chi_dn0 = assign26200_body12_e36076_d_n0;
            locals.var_exp_chi_dn2 = assign26200_body12_e36076_d_n2;
            locals.var_exp_chi_dn6 = assign26200_body12_e36076_d_n6;
            locals.var_exp_chi_dn7 = assign26200_body12_e36076_d_n7;
            locals.var_exp_chi_dn10 = assign26200_body12_e36076_d_n10;
            locals.var_exp_chi_dn11 = assign26200_body12_e36076_d_n11;
            locals.var_exp_chi_dn12 = assign26200_body12_e36076_d_n12;
            locals.var_exp_chi_dn17 = assign26200_body12_e36076_d_n17;
            let (assign26200_body13_e36098, assign26200_body13_e36098_d_n0, assign26200_body13_e36098_d_n2, assign26200_body13_e36098_d_n6, assign26200_body13_e36098_d_n7, assign26200_body13_e36098_d_n10, assign26200_body13_e36098_d_n11, assign26200_body13_e36098_d_n12, assign26200_body13_e36098_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26200_body13_e36095: f64 = (locals.var_exp_chi - 1.0);
        let assign26200_body13_e36096: f64 = (locals.var_cfs1__blk842 * assign26200_body13_e36095);
        (assign26200_body13_e36096, ((locals.var_cfs1__blk842_dn0 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk842_dn2 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk842_dn6 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk842_dn7 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk842_dn10 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk842_dn11 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk842_dn12 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk842_dn17 * assign26200_body13_e36095) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign26200_body13_e36098;
            locals.var_fs01__blk836_dn0 = assign26200_body13_e36098_d_n0;
            locals.var_fs01__blk836_dn2 = assign26200_body13_e36098_d_n2;
            locals.var_fs01__blk836_dn6 = assign26200_body13_e36098_d_n6;
            locals.var_fs01__blk836_dn7 = assign26200_body13_e36098_d_n7;
            locals.var_fs01__blk836_dn10 = assign26200_body13_e36098_d_n10;
            locals.var_fs01__blk836_dn11 = assign26200_body13_e36098_d_n11;
            locals.var_fs01__blk836_dn12 = assign26200_body13_e36098_d_n12;
            locals.var_fs01__blk836_dn17 = assign26200_body13_e36098_d_n17;
            let (assign26200_body14_e36120, assign26200_body14_e36120_d_n0, assign26200_body14_e36120_d_n2, assign26200_body14_e36120_d_n6, assign26200_body14_e36120_d_n7, assign26200_body14_e36120_d_n10, assign26200_body14_e36120_d_n11, assign26200_body14_e36120_d_n12, assign26200_body14_e36120_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26200_body14_e36116: f64 = (locals.var_cfs1__blk842 * locals.var_beta);
        let assign26200_body14_e36118: f64 = (assign26200_body14_e36116 * locals.var_exp_chi);
        (assign26200_body14_e36118, (((locals.var_cfs1__blk842_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk842_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk842_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk842_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk842_dn10 * locals.var_beta) + (locals.var_cfs1__blk842 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk842_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk842_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk842_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign26200_body14_e36116 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign26200_body14_e36120;
            locals.var_fs01_dps0__blk837_dn0 = assign26200_body14_e36120_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign26200_body14_e36120_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign26200_body14_e36120_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign26200_body14_e36120_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign26200_body14_e36120_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign26200_body14_e36120_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign26200_body14_e36120_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign26200_body14_e36120_d_n17;
            let (assign26200_body15_e36142, assign26200_body15_e36142_d_n0, assign26200_body15_e36142_d_n2, assign26200_body15_e36142_d_n6, assign26200_body15_e36142_d_n7, assign26200_body15_e36142_d_n10, assign26200_body15_e36142_d_n11, assign26200_body15_e36142_d_n12, assign26200_body15_e36142_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 == 0.0)) {
        let assign26200_body15_e36139: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign26200_body15_e36140: f64 = (assign26200_body15_e36139).exp();
        (assign26200_body15_e36140, (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign26200_body15_e36140 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign26200_body15_e36140 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk843, locals.var_exp_bps0__blk843_dn0, locals.var_exp_bps0__blk843_dn2, locals.var_exp_bps0__blk843_dn6, locals.var_exp_bps0__blk843_dn7, locals.var_exp_bps0__blk843_dn10, locals.var_exp_bps0__blk843_dn11, locals.var_exp_bps0__blk843_dn12, locals.var_exp_bps0__blk843_dn17,)
    }
};
            locals.var_exp_bps0__blk843 = assign26200_body15_e36142;
            locals.var_exp_bps0__blk843_dn0 = assign26200_body15_e36142_d_n0;
            locals.var_exp_bps0__blk843_dn2 = assign26200_body15_e36142_d_n2;
            locals.var_exp_bps0__blk843_dn6 = assign26200_body15_e36142_d_n6;
            locals.var_exp_bps0__blk843_dn7 = assign26200_body15_e36142_d_n7;
            locals.var_exp_bps0__blk843_dn10 = assign26200_body15_e36142_d_n10;
            locals.var_exp_bps0__blk843_dn11 = assign26200_body15_e36142_d_n11;
            locals.var_exp_bps0__blk843_dn12 = assign26200_body15_e36142_d_n12;
            locals.var_exp_bps0__blk843_dn17 = assign26200_body15_e36142_d_n17;
            let (assign26200_body16_e36165, assign26200_body16_e36165_d_n0, assign26200_body16_e36165_d_n2, assign26200_body16_e36165_d_n6, assign26200_body16_e36165_d_n7, assign26200_body16_e36165_d_n10, assign26200_body16_e36165_d_n11, assign26200_body16_e36165_d_n12, assign26200_body16_e36165_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 == 0.0)) {
        let assign26200_body16_e36162: f64 = (locals.var_exp_bps0__blk843 - locals.var_exp_bvbs__blk833);
        let assign26200_body16_e36163: f64 = (locals.var_cnst1over * assign26200_body16_e36162);
        (assign26200_body16_e36163, ((locals.var_cnst1over_dn0 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn0 - locals.var_exp_bvbs__blk833_dn0))), ((locals.var_cnst1over_dn2 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn2 - locals.var_exp_bvbs__blk833_dn2))), ((locals.var_cnst1over_dn6 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn6 - locals.var_exp_bvbs__blk833_dn6))), ((locals.var_cnst1over_dn7 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn7 - locals.var_exp_bvbs__blk833_dn7))), ((locals.var_cnst1over_dn10 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn10 - locals.var_exp_bvbs__blk833_dn10))), ((locals.var_cnst1over_dn11 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn11 - locals.var_exp_bvbs__blk833_dn11))), ((locals.var_cnst1over_dn12 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn12 - locals.var_exp_bvbs__blk833_dn12))), ((locals.var_cnst1over_dn17 * assign26200_body16_e36162) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn17 - locals.var_exp_bvbs__blk833_dn17))),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign26200_body16_e36165;
            locals.var_fs01__blk836_dn0 = assign26200_body16_e36165_d_n0;
            locals.var_fs01__blk836_dn2 = assign26200_body16_e36165_d_n2;
            locals.var_fs01__blk836_dn6 = assign26200_body16_e36165_d_n6;
            locals.var_fs01__blk836_dn7 = assign26200_body16_e36165_d_n7;
            locals.var_fs01__blk836_dn10 = assign26200_body16_e36165_d_n10;
            locals.var_fs01__blk836_dn11 = assign26200_body16_e36165_d_n11;
            locals.var_fs01__blk836_dn12 = assign26200_body16_e36165_d_n12;
            locals.var_fs01__blk836_dn17 = assign26200_body16_e36165_d_n17;
            let (assign26200_body17_e36188, assign26200_body17_e36188_d_n0, assign26200_body17_e36188_d_n2, assign26200_body17_e36188_d_n6, assign26200_body17_e36188_d_n7, assign26200_body17_e36188_d_n10, assign26200_body17_e36188_d_n11, assign26200_body17_e36188_d_n12, assign26200_body17_e36188_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) && (locals.var_guard861 == 0.0)) {
        let assign26200_body17_e36184: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign26200_body17_e36186: f64 = (assign26200_body17_e36184 * locals.var_exp_bps0__blk843);
        (assign26200_body17_e36186, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign26200_body17_e36184 * locals.var_exp_bps0__blk843_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign26200_body17_e36188;
            locals.var_fs01_dps0__blk837_dn0 = assign26200_body17_e36188_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign26200_body17_e36188_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign26200_body17_e36188_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign26200_body17_e36188_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign26200_body17_e36188_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign26200_body17_e36188_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign26200_body17_e36188_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign26200_body17_e36188_d_n17;
            let (assign26200_body18_e36209, assign26200_body18_e36209_d_n0, assign26200_body18_e36209_d_n2, assign26200_body18_e36209_d_n6, assign26200_body18_e36209_d_n7, assign26200_body18_e36209_d_n10, assign26200_body18_e36209_d_n11, assign26200_body18_e36209_d_n12, assign26200_body18_e36209_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) {
        let assign26200_body18_e36204: f64 = (locals.var_chi__blk814 - 1.0);
        let assign26200_body18_e36206: f64 = (assign26200_body18_e36204 + locals.var_fs01__blk836);
        let assign26200_body18_e36207: f64 = (assign26200_body18_e36206).sqrt();
        (assign26200_body18_e36207, ((locals.var_chi__blk814_dn0 + locals.var_fs01__blk836_dn0) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn2 + locals.var_fs01__blk836_dn2) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn6 + locals.var_fs01__blk836_dn6) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn7 + locals.var_fs01__blk836_dn7) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn10 + locals.var_fs01__blk836_dn10) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn11 + locals.var_fs01__blk836_dn11) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn12 + locals.var_fs01__blk836_dn12) / (2.0 * assign26200_body18_e36207)), ((locals.var_chi__blk814_dn17 + locals.var_fs01__blk836_dn17) / (2.0 * assign26200_body18_e36207)),)
    } else {
        (locals.var_fs02__blk840, locals.var_fs02__blk840_dn0, locals.var_fs02__blk840_dn2, locals.var_fs02__blk840_dn6, locals.var_fs02__blk840_dn7, locals.var_fs02__blk840_dn10, locals.var_fs02__blk840_dn11, locals.var_fs02__blk840_dn12, locals.var_fs02__blk840_dn17,)
    }
};
            locals.var_fs02__blk840 = assign26200_body18_e36209;
            locals.var_fs02__blk840_dn0 = assign26200_body18_e36209_d_n0;
            locals.var_fs02__blk840_dn2 = assign26200_body18_e36209_d_n2;
            locals.var_fs02__blk840_dn6 = assign26200_body18_e36209_d_n6;
            locals.var_fs02__blk840_dn7 = assign26200_body18_e36209_d_n7;
            locals.var_fs02__blk840_dn10 = assign26200_body18_e36209_d_n10;
            locals.var_fs02__blk840_dn11 = assign26200_body18_e36209_d_n11;
            locals.var_fs02__blk840_dn12 = assign26200_body18_e36209_d_n12;
            locals.var_fs02__blk840_dn17 = assign26200_body18_e36209_d_n17;
            let (assign26200_body19_e36231, assign26200_body19_e36231_d_n0, assign26200_body19_e36231_d_n2, assign26200_body19_e36231_d_n6, assign26200_body19_e36231_d_n7, assign26200_body19_e36231_d_n10, assign26200_body19_e36231_d_n11, assign26200_body19_e36231_d_n12, assign26200_body19_e36231_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard860 == 0.0)) {
        let assign26200_body19_e36225: f64 = (locals.var_beta + locals.var_fs01_dps0__blk837);
        let assign26200_body19_e36227: f64 = (assign26200_body19_e36225 / locals.var_fs02__blk840);
        let assign26200_body19_e36229: f64 = (assign26200_body19_e36227 * 0.5);
        (assign26200_body19_e36229, ((((locals.var_fs01_dps0__blk837_dn0 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn0)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn2 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn2)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn6 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn6)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn7 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn7)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk837_dn10) * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn10)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn11 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn11)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn12 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn12)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn17 * locals.var_fs02__blk840) - (assign26200_body19_e36225 * locals.var_fs02__blk840_dn17)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk841, locals.var_fs02_dps0__blk841_dn0, locals.var_fs02_dps0__blk841_dn2, locals.var_fs02_dps0__blk841_dn6, locals.var_fs02_dps0__blk841_dn7, locals.var_fs02_dps0__blk841_dn10, locals.var_fs02_dps0__blk841_dn11, locals.var_fs02_dps0__blk841_dn12, locals.var_fs02_dps0__blk841_dn17,)
    }
};
            locals.var_fs02_dps0__blk841 = assign26200_body19_e36231;
            locals.var_fs02_dps0__blk841_dn0 = assign26200_body19_e36231_d_n0;
            locals.var_fs02_dps0__blk841_dn2 = assign26200_body19_e36231_d_n2;
            locals.var_fs02_dps0__blk841_dn6 = assign26200_body19_e36231_d_n6;
            locals.var_fs02_dps0__blk841_dn7 = assign26200_body19_e36231_d_n7;
            locals.var_fs02_dps0__blk841_dn10 = assign26200_body19_e36231_d_n10;
            locals.var_fs02_dps0__blk841_dn11 = assign26200_body19_e36231_d_n11;
            locals.var_fs02_dps0__blk841_dn12 = assign26200_body19_e36231_d_n12;
            locals.var_fs02_dps0__blk841_dn17 = assign26200_body19_e36231_d_n17;
            let (assign26200_body20_e36250, assign26200_body20_e36250_d_n0, assign26200_body20_e36250_d_n2, assign26200_body20_e36250_d_n6, assign26200_body20_e36250_d_n7, assign26200_body20_e36250_d_n10, assign26200_body20_e36250_d_n11, assign26200_body20_e36250_d_n12, assign26200_body20_e36250_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26200_body20_e36244: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26200_body20_e36247: f64 = (locals.var_fac1__blk800 * locals.var_fs02__blk840);
        let assign26200_body20_e36248: f64 = (assign26200_body20_e36244 - assign26200_body20_e36247);
        (assign26200_body20_e36248, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk800_dn0 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk800_dn2 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk800_dn6 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk800_dn7 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk800_dn10 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk800_dn11 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk800_dn12 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk800_dn17 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn17))),)
    } else {
        (locals.var_fs0__blk844, locals.var_fs0__blk844_dn0, locals.var_fs0__blk844_dn2, locals.var_fs0__blk844_dn6, locals.var_fs0__blk844_dn7, locals.var_fs0__blk844_dn10, locals.var_fs0__blk844_dn11, locals.var_fs0__blk844_dn12, locals.var_fs0__blk844_dn17,)
    }
};
            locals.var_fs0__blk844 = assign26200_body20_e36250;
            locals.var_fs0__blk844_dn0 = assign26200_body20_e36250_d_n0;
            locals.var_fs0__blk844_dn2 = assign26200_body20_e36250_d_n2;
            locals.var_fs0__blk844_dn6 = assign26200_body20_e36250_d_n6;
            locals.var_fs0__blk844_dn7 = assign26200_body20_e36250_d_n7;
            locals.var_fs0__blk844_dn10 = assign26200_body20_e36250_d_n10;
            locals.var_fs0__blk844_dn11 = assign26200_body20_e36250_d_n11;
            locals.var_fs0__blk844_dn12 = assign26200_body20_e36250_d_n12;
            locals.var_fs0__blk844_dn17 = assign26200_body20_e36250_d_n17;
            let (assign26200_body21_e36268, assign26200_body21_e36268_d_n0, assign26200_body21_e36268_d_n2, assign26200_body21_e36268_d_n6, assign26200_body21_e36268_d_n7, assign26200_body21_e36268_d_n10, assign26200_body21_e36268_d_n11, assign26200_body21_e36268_d_n12, assign26200_body21_e36268_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26200_body21_e36262: f64 = (-1.0);
        let assign26200_body21_e36265: f64 = (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841);
        let assign26200_body21_e36266: f64 = (assign26200_body21_e36262 - assign26200_body21_e36265);
        (assign26200_body21_e36266, (-((locals.var_fac1__blk800_dn0 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn0))), (-((locals.var_fac1__blk800_dn2 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn2))), (-((locals.var_fac1__blk800_dn6 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn6))), (-((locals.var_fac1__blk800_dn7 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn7))), (-((locals.var_fac1__blk800_dn10 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn10))), (-((locals.var_fac1__blk800_dn11 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn11))), (-((locals.var_fac1__blk800_dn12 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn12))), (-((locals.var_fac1__blk800_dn17 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk845, locals.var_fs0_dps0__blk845_dn0, locals.var_fs0_dps0__blk845_dn2, locals.var_fs0_dps0__blk845_dn6, locals.var_fs0_dps0__blk845_dn7, locals.var_fs0_dps0__blk845_dn10, locals.var_fs0_dps0__blk845_dn11, locals.var_fs0_dps0__blk845_dn12, locals.var_fs0_dps0__blk845_dn17,)
    }
};
            locals.var_fs0_dps0__blk845 = assign26200_body21_e36268;
            locals.var_fs0_dps0__blk845_dn0 = assign26200_body21_e36268_d_n0;
            locals.var_fs0_dps0__blk845_dn2 = assign26200_body21_e36268_d_n2;
            locals.var_fs0_dps0__blk845_dn6 = assign26200_body21_e36268_d_n6;
            locals.var_fs0_dps0__blk845_dn7 = assign26200_body21_e36268_d_n7;
            locals.var_fs0_dps0__blk845_dn10 = assign26200_body21_e36268_d_n10;
            locals.var_fs0_dps0__blk845_dn11 = assign26200_body21_e36268_d_n11;
            locals.var_fs0_dps0__blk845_dn12 = assign26200_body21_e36268_d_n12;
            locals.var_fs0_dps0__blk845_dn17 = assign26200_body21_e36268_d_n17;
            let assign26200_body22_e36271: f64 = if locals.var_flg_conv__blk787 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard862 = assign26200_body22_e36271;
            let (assign26200_body23_e36290,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26200_body23_e36286: f64 = (2.0 * 20.0);
        let assign26200_body23_e36288: f64 = (assign26200_body23_e36286 + 1.0);
        (assign26200_body23_e36288,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26200_body23_e36290;
            let (assign26200_body24_e36309, assign26200_body24_e36309_d_n0, assign26200_body24_e36309_d_n2, assign26200_body24_e36309_d_n6, assign26200_body24_e36309_d_n7, assign26200_body24_e36309_d_n10, assign26200_body24_e36309_d_n11, assign26200_body24_e36309_d_n12, assign26200_body24_e36309_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 == 0.0)) {
        let assign26200_body24_e36305: f64 = (-locals.var_fs0__blk844);
        let assign26200_body24_e36307: f64 = (assign26200_body24_e36305 / locals.var_fs0_dps0__blk845);
        (assign26200_body24_e36307, ((((-locals.var_fs0__blk844_dn0) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn0)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn2) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn2)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn6) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn6)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn7) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn7)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn10) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn10)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn11) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn11)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn12) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn12)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn17) * locals.var_fs0_dps0__blk845) - (assign26200_body24_e36305 * locals.var_fs0_dps0__blk845_dn17)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26200_body24_e36309;
            locals.var_dps0_dn0 = assign26200_body24_e36309_d_n0;
            locals.var_dps0_dn2 = assign26200_body24_e36309_d_n2;
            locals.var_dps0_dn6 = assign26200_body24_e36309_d_n6;
            locals.var_dps0_dn7 = assign26200_body24_e36309_d_n7;
            locals.var_dps0_dn10 = assign26200_body24_e36309_d_n10;
            locals.var_dps0_dn11 = assign26200_body24_e36309_d_n11;
            locals.var_dps0_dn12 = assign26200_body24_e36309_d_n12;
            locals.var_dps0_dn17 = assign26200_body24_e36309_d_n17;
            let (assign26200_body25_e36338, assign26200_body25_e36338_d_n0, assign26200_body25_e36338_d_n2, assign26200_body25_e36338_d_n6, assign26200_body25_e36338_d_n7, assign26200_body25_e36338_d_n10, assign26200_body25_e36338_d_n11, assign26200_body25_e36338_d_n12, assign26200_body25_e36338_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 == 0.0)) {
        let assign26200_body25_e36325: f64 = (0.5 * 0.1);
        let assign26200_body25_e36329: f64 = (locals.var_ps0ld).abs();
        let (assign26200_body25_e36334, assign26200_body25_e36334_d_n0, assign26200_body25_e36334_d_n2, assign26200_body25_e36334_d_n6, assign26200_body25_e36334_d_n7, assign26200_body25_e36334_d_n10, assign26200_body25_e36334_d_n11, assign26200_body25_e36334_d_n12, assign26200_body25_e36334_d_n17,) = {
            if (1.0 >= assign26200_body25_e36329) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26200_body25_e36333: f64 = (locals.var_ps0ld).abs();
                (assign26200_body25_e36333, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign26200_body25_e36335: f64 = (1.0 + assign26200_body25_e36334);
        let assign26200_body25_e36336: f64 = (assign26200_body25_e36325 * assign26200_body25_e36335);
        (assign26200_body25_e36336, (assign26200_body25_e36325 * assign26200_body25_e36334_d_n0), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n2), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n6), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n7), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n10), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n11), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n12), (assign26200_body25_e36325 * assign26200_body25_e36334_d_n17),)
    } else {
        (locals.var_dplim__blk846, locals.var_dplim__blk846_dn0, locals.var_dplim__blk846_dn2, locals.var_dplim__blk846_dn6, locals.var_dplim__blk846_dn7, locals.var_dplim__blk846_dn10, locals.var_dplim__blk846_dn11, locals.var_dplim__blk846_dn12, locals.var_dplim__blk846_dn17,)
    }
};
            locals.var_dplim__blk846 = assign26200_body25_e36338;
            locals.var_dplim__blk846_dn0 = assign26200_body25_e36338_d_n0;
            locals.var_dplim__blk846_dn2 = assign26200_body25_e36338_d_n2;
            locals.var_dplim__blk846_dn6 = assign26200_body25_e36338_d_n6;
            locals.var_dplim__blk846_dn7 = assign26200_body25_e36338_d_n7;
            locals.var_dplim__blk846_dn10 = assign26200_body25_e36338_d_n10;
            locals.var_dplim__blk846_dn11 = assign26200_body25_e36338_d_n11;
            locals.var_dplim__blk846_dn12 = assign26200_body25_e36338_d_n12;
            locals.var_dplim__blk846_dn17 = assign26200_body25_e36338_d_n17;
            let assign26200_body26_e36340: f64 = (locals.var_dps0).abs();
            let assign26200_body26_e36342: f64 = if assign26200_body26_e36340 > locals.var_dplim__blk846 { 1.0 } else { 0.0 };
            locals.var_guard863 = assign26200_body26_e36342;
            let (assign26200_body27_e36368, assign26200_body27_e36368_d_n0, assign26200_body27_e36368_d_n2, assign26200_body27_e36368_d_n6, assign26200_body27_e36368_d_n7, assign26200_body27_e36368_d_n10, assign26200_body27_e36368_d_n11, assign26200_body27_e36368_d_n12, assign26200_body27_e36368_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let (assign26200_body27_e36365,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign26200_body27_e36364: f64 = (-1.0);
                (assign26200_body27_e36364,)
            }
        };
        let assign26200_body27_e36366: f64 = (locals.var_dplim__blk846 * assign26200_body27_e36365);
        (assign26200_body27_e36366, (locals.var_dplim__blk846_dn0 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn2 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn6 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn7 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn10 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn11 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn12 * assign26200_body27_e36365), (locals.var_dplim__blk846_dn17 * assign26200_body27_e36365),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26200_body27_e36368;
            locals.var_dps0_dn0 = assign26200_body27_e36368_d_n0;
            locals.var_dps0_dn2 = assign26200_body27_e36368_d_n2;
            locals.var_dps0_dn6 = assign26200_body27_e36368_d_n6;
            locals.var_dps0_dn7 = assign26200_body27_e36368_d_n7;
            locals.var_dps0_dn10 = assign26200_body27_e36368_d_n10;
            locals.var_dps0_dn11 = assign26200_body27_e36368_d_n11;
            locals.var_dps0_dn12 = assign26200_body27_e36368_d_n12;
            locals.var_dps0_dn17 = assign26200_body27_e36368_d_n17;
            let (assign26200_body28_e36386, assign26200_body28_e36386_d_n0, assign26200_body28_e36386_d_n2, assign26200_body28_e36386_d_n6, assign26200_body28_e36386_d_n7, assign26200_body28_e36386_d_n10, assign26200_body28_e36386_d_n11, assign26200_body28_e36386_d_n12, assign26200_body28_e36386_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 == 0.0)) {
        let assign26200_body28_e36384: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign26200_body28_e36384, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign26200_body28_e36386;
            locals.var_ps0ld_dn0 = assign26200_body28_e36386_d_n0;
            locals.var_ps0ld_dn2 = assign26200_body28_e36386_d_n2;
            locals.var_ps0ld_dn6 = assign26200_body28_e36386_d_n6;
            locals.var_ps0ld_dn7 = assign26200_body28_e36386_d_n7;
            locals.var_ps0ld_dn10 = assign26200_body28_e36386_d_n10;
            locals.var_ps0ld_dn11 = assign26200_body28_e36386_d_n11;
            locals.var_ps0ld_dn12 = assign26200_body28_e36386_d_n12;
            locals.var_ps0ld_dn17 = assign26200_body28_e36386_d_n17;
            let assign26200_body29_e36388: f64 = (locals.var_dps0).abs();
            let assign26200_body29_e36392: f64 = (locals.var_fs0__blk844).abs();
            let assign26200_body29_e36395: f64 = if ((assign26200_body29_e36388 <= 5e-12) && (assign26200_body29_e36392 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard864 = assign26200_body29_e36395;
            let (assign26200_body30_e36413,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard864 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk787,)
    }
};
            locals.var_flg_conv__blk787 = assign26200_body30_e36413;
            let (assign26200_body31_e36428,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26200_body31_e36426: f64 = (locals.var_lp_s0 + 1.0);
        (assign26200_body31_e36426,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26200_body31_e36428;
        }

    }

    pub(super) fn stamp_transient_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign26220_e36434: f64 = if locals.var_chi__blk814 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard866 = assign26220_e36434;

        let (assign26260_e36493, assign26260_e36493_d_n0, assign26260_e36493_d_n2, assign26260_e36493_d_n6, assign26260_e36493_d_n7, assign26260_e36493_d_n10, assign26260_e36493_d_n11, assign26260_e36493_d_n12, assign26260_e36493_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard866 != 0.0)) {
        let assign26260_e36487: f64 = (locals.var_fb__blk838 * locals.var_fb__blk838);
        let assign26260_e36490: f64 = (10.0 * 2.220446049250313e-16);
        let assign26260_e36491: f64 = (assign26260_e36487 + assign26260_e36490);
        (assign26260_e36491, ((locals.var_fb__blk838_dn0 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn0)), ((locals.var_fb__blk838_dn2 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn2)), ((locals.var_fb__blk838_dn6 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn6)), ((locals.var_fb__blk838_dn7 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn7)), ((locals.var_fb__blk838_dn10 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn10)), ((locals.var_fb__blk838_dn11 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn11)), ((locals.var_fb__blk838_dn12 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn12)), ((locals.var_fb__blk838_dn17 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn17)),)
    } else {
        (locals.var_xi0__blk847, locals.var_xi0__blk847_dn0, locals.var_xi0__blk847_dn2, locals.var_xi0__blk847_dn6, locals.var_xi0__blk847_dn7, locals.var_xi0__blk847_dn10, locals.var_xi0__blk847_dn11, locals.var_xi0__blk847_dn12, locals.var_xi0__blk847_dn17,)
    }
};
        locals.var_xi0__blk847 = assign26260_e36493;
        locals.var_xi0__blk847_dn0 = assign26260_e36493_d_n0;
        locals.var_xi0__blk847_dn2 = assign26260_e36493_d_n2;
        locals.var_xi0__blk847_dn6 = assign26260_e36493_d_n6;
        locals.var_xi0__blk847_dn7 = assign26260_e36493_d_n7;
        locals.var_xi0__blk847_dn10 = assign26260_e36493_d_n10;
        locals.var_xi0__blk847_dn11 = assign26260_e36493_d_n11;
        locals.var_xi0__blk847_dn12 = assign26260_e36493_d_n12;
        locals.var_xi0__blk847_dn17 = assign26260_e36493_d_n17;

        let (assign26270_e36512, assign26270_e36512_d_n0, assign26270_e36512_d_n2, assign26270_e36512_d_n6, assign26270_e36512_d_n7, assign26270_e36512_d_n10, assign26270_e36512_d_n11, assign26270_e36512_d_n12, assign26270_e36512_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard866 != 0.0)) {
        let assign26270_e36509: f64 = (10.0 * 2.220446049250313e-16);
        let assign26270_e36510: f64 = (locals.var_fb__blk838 + assign26270_e36509);
        (assign26270_e36510, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    } else {
        (locals.var_xi0p12__blk848, locals.var_xi0p12__blk848_dn0, locals.var_xi0p12__blk848_dn2, locals.var_xi0p12__blk848_dn6, locals.var_xi0p12__blk848_dn7, locals.var_xi0p12__blk848_dn10, locals.var_xi0p12__blk848_dn11, locals.var_xi0p12__blk848_dn12, locals.var_xi0p12__blk848_dn17,)
    }
};
        locals.var_xi0p12__blk848 = assign26270_e36512;
        locals.var_xi0p12__blk848_dn0 = assign26270_e36512_d_n0;
        locals.var_xi0p12__blk848_dn2 = assign26270_e36512_d_n2;
        locals.var_xi0p12__blk848_dn6 = assign26270_e36512_d_n6;
        locals.var_xi0p12__blk848_dn7 = assign26270_e36512_d_n7;
        locals.var_xi0p12__blk848_dn10 = assign26270_e36512_d_n10;
        locals.var_xi0p12__blk848_dn11 = assign26270_e36512_d_n11;
        locals.var_xi0p12__blk848_dn12 = assign26270_e36512_d_n12;
        locals.var_xi0p12__blk848_dn17 = assign26270_e36512_d_n17;

        let (assign26290_e36546, assign26290_e36546_d_n0, assign26290_e36546_d_n2, assign26290_e36546_d_n6, assign26290_e36546_d_n7, assign26290_e36546_d_n10, assign26290_e36546_d_n11, assign26290_e36546_d_n12, assign26290_e36546_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard866 == 0.0)) {
        let assign26290_e36544: f64 = (locals.var_chi__blk814 - 1.0);
        (assign26290_e36544, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    } else {
        (locals.var_xi0__blk847, locals.var_xi0__blk847_dn0, locals.var_xi0__blk847_dn2, locals.var_xi0__blk847_dn6, locals.var_xi0__blk847_dn7, locals.var_xi0__blk847_dn10, locals.var_xi0__blk847_dn11, locals.var_xi0__blk847_dn12, locals.var_xi0__blk847_dn17,)
    }
};
        locals.var_xi0__blk847 = assign26290_e36546;
        locals.var_xi0__blk847_dn0 = assign26290_e36546_d_n0;
        locals.var_xi0__blk847_dn2 = assign26290_e36546_d_n2;
        locals.var_xi0__blk847_dn6 = assign26290_e36546_d_n6;
        locals.var_xi0__blk847_dn7 = assign26290_e36546_d_n7;
        locals.var_xi0__blk847_dn10 = assign26290_e36546_d_n10;
        locals.var_xi0__blk847_dn11 = assign26290_e36546_d_n11;
        locals.var_xi0__blk847_dn12 = assign26290_e36546_d_n12;
        locals.var_xi0__blk847_dn17 = assign26290_e36546_d_n17;

        let (assign26300_e36563, assign26300_e36563_d_n0, assign26300_e36563_d_n2, assign26300_e36563_d_n6, assign26300_e36563_d_n7, assign26300_e36563_d_n10, assign26300_e36563_d_n11, assign26300_e36563_d_n12, assign26300_e36563_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) && (locals.var_guard866 == 0.0)) {
        let assign26300_e36561: f64 = (locals.var_xi0__blk847).sqrt();
        (assign26300_e36561, (locals.var_xi0__blk847_dn0 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn2 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn6 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn7 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn10 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn11 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn12 / (2.0 * assign26300_e36561)), (locals.var_xi0__blk847_dn17 / (2.0 * assign26300_e36561)),)
    } else {
        (locals.var_xi0p12__blk848, locals.var_xi0p12__blk848_dn0, locals.var_xi0p12__blk848_dn2, locals.var_xi0p12__blk848_dn6, locals.var_xi0p12__blk848_dn7, locals.var_xi0p12__blk848_dn10, locals.var_xi0p12__blk848_dn11, locals.var_xi0p12__blk848_dn12, locals.var_xi0p12__blk848_dn17,)
    }
};
        locals.var_xi0p12__blk848 = assign26300_e36563;
        locals.var_xi0p12__blk848_dn0 = assign26300_e36563_d_n0;
        locals.var_xi0p12__blk848_dn2 = assign26300_e36563_d_n2;
        locals.var_xi0p12__blk848_dn6 = assign26300_e36563_d_n6;
        locals.var_xi0p12__blk848_dn7 = assign26300_e36563_d_n7;
        locals.var_xi0p12__blk848_dn10 = assign26300_e36563_d_n10;
        locals.var_xi0p12__blk848_dn11 = assign26300_e36563_d_n11;
        locals.var_xi0p12__blk848_dn12 = assign26300_e36563_d_n12;
        locals.var_xi0p12__blk848_dn17 = assign26300_e36563_d_n17;

        let (assign26310_e36578, assign26310_e36578_d_n0, assign26310_e36578_d_n2, assign26310_e36578_d_n6, assign26310_e36578_d_n7, assign26310_e36578_d_n10, assign26310_e36578_d_n11, assign26310_e36578_d_n12, assign26310_e36578_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26310_e36576: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk848);
        (assign26310_e36576, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26310_e36578;
        locals.var_qbuld_dn0 = assign26310_e36578_d_n0;
        locals.var_qbuld_dn2 = assign26310_e36578_d_n2;
        locals.var_qbuld_dn6 = assign26310_e36578_d_n6;
        locals.var_qbuld_dn7 = assign26310_e36578_d_n7;
        locals.var_qbuld_dn10 = assign26310_e36578_d_n10;
        locals.var_qbuld_dn11 = assign26310_e36578_d_n11;
        locals.var_qbuld_dn12 = assign26310_e36578_d_n12;
        locals.var_qbuld_dn17 = assign26310_e36578_d_n17;

        let (assign26320_e36595, assign26320_e36595_d_n0, assign26320_e36595_d_n2, assign26320_e36595_d_n6, assign26320_e36595_d_n7, assign26320_e36595_d_n10, assign26320_e36595_d_n11, assign26320_e36595_d_n12, assign26320_e36595_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26320_e36592: f64 = (locals.var_fs02__blk840 + locals.var_xi0p12__blk848);
        let assign26320_e36593: f64 = (1.0 / assign26320_e36592);
        (assign26320_e36593, (-((locals.var_fs02__blk840_dn0 + locals.var_xi0p12__blk848_dn0) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn2 + locals.var_xi0p12__blk848_dn2) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn6 + locals.var_xi0p12__blk848_dn6) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn7 + locals.var_xi0p12__blk848_dn7) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn10 + locals.var_xi0p12__blk848_dn10) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn11 + locals.var_xi0p12__blk848_dn11) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn12 + locals.var_xi0p12__blk848_dn12) / (assign26320_e36592 * assign26320_e36592))), (-((locals.var_fs02__blk840_dn17 + locals.var_xi0p12__blk848_dn17) / (assign26320_e36592 * assign26320_e36592))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26320_e36595;
        locals.var_t1__blk771_dn0 = assign26320_e36595_d_n0;
        locals.var_t1__blk771_dn2 = assign26320_e36595_d_n2;
        locals.var_t1__blk771_dn6 = assign26320_e36595_d_n6;
        locals.var_t1__blk771_dn7 = assign26320_e36595_d_n7;
        locals.var_t1__blk771_dn10 = assign26320_e36595_d_n10;
        locals.var_t1__blk771_dn11 = assign26320_e36595_d_n11;
        locals.var_t1__blk771_dn12 = assign26320_e36595_d_n12;
        locals.var_t1__blk771_dn17 = assign26320_e36595_d_n17;

        let (assign26330_e36612, assign26330_e36612_d_n0, assign26330_e36612_d_n2, assign26330_e36612_d_n6, assign26330_e36612_d_n7, assign26330_e36612_d_n10, assign26330_e36612_d_n11, assign26330_e36612_d_n12, assign26330_e36612_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26330_e36608: f64 = (locals.var_cnst0over * locals.var_fs01__blk836);
        let assign26330_e36610: f64 = (assign26330_e36608 * locals.var_t1__blk771);
        (assign26330_e36610, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn0)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn2)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn6)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn7)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn10)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn11)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn12)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn17)) * locals.var_t1__blk771) + (assign26330_e36608 * locals.var_t1__blk771_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26330_e36612;
        locals.var_qiuld_dn0 = assign26330_e36612_d_n0;
        locals.var_qiuld_dn2 = assign26330_e36612_d_n2;
        locals.var_qiuld_dn6 = assign26330_e36612_d_n6;
        locals.var_qiuld_dn7 = assign26330_e36612_d_n7;
        locals.var_qiuld_dn10 = assign26330_e36612_d_n10;
        locals.var_qiuld_dn11 = assign26330_e36612_d_n11;
        locals.var_qiuld_dn12 = assign26330_e36612_d_n12;
        locals.var_qiuld_dn17 = assign26330_e36612_d_n17;

        let (assign26340_e36627, assign26340_e36627_d_n0, assign26340_e36627_d_n2, assign26340_e36627_d_n6, assign26340_e36627_d_n7, assign26340_e36627_d_n10, assign26340_e36627_d_n11, assign26340_e36627_d_n12, assign26340_e36627_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard853 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign26340_e36625: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign26340_e36625, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26340_e36627;
        locals.var_qsuld_dn0 = assign26340_e36627_d_n0;
        locals.var_qsuld_dn2 = assign26340_e36627_d_n2;
        locals.var_qsuld_dn6 = assign26340_e36627_d_n6;
        locals.var_qsuld_dn7 = assign26340_e36627_d_n7;
        locals.var_qsuld_dn10 = assign26340_e36627_d_n10;
        locals.var_qsuld_dn11 = assign26340_e36627_d_n11;
        locals.var_qsuld_dn12 = assign26340_e36627_d_n12;
        locals.var_qsuld_dn17 = assign26340_e36627_d_n17;

        let (assign26350_e36637, assign26350_e36637_d_n0, assign26350_e36637_d_n2, assign26350_e36637_d_n6, assign26350_e36637_d_n7, assign26350_e36637_d_n10, assign26350_e36637_d_n11, assign26350_e36637_d_n12, assign26350_e36637_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26350_e36635: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign26350_e36635, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26350_e36637;
        locals.var_qiuld_dn0 = assign26350_e36637_d_n0;
        locals.var_qiuld_dn2 = assign26350_e36637_d_n2;
        locals.var_qiuld_dn6 = assign26350_e36637_d_n6;
        locals.var_qiuld_dn7 = assign26350_e36637_d_n7;
        locals.var_qiuld_dn10 = assign26350_e36637_d_n10;
        locals.var_qiuld_dn11 = assign26350_e36637_d_n11;
        locals.var_qiuld_dn12 = assign26350_e36637_d_n12;
        locals.var_qiuld_dn17 = assign26350_e36637_d_n17;

        let assign26360_e36640: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard868 = assign26360_e36640;

        let assign26370_e36643: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard869 = assign26370_e36643;

        let (assign26380_e36658, assign26380_e36658_d_n0, assign26380_e36658_d_n2, assign26380_e36658_d_n6, assign26380_e36658_d_n7, assign26380_e36658_d_n10, assign26380_e36658_d_n11, assign26380_e36658_d_n12, assign26380_e36658_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26380_e36654: f64 = (-locals.var_uc_areabt);
        let assign26380_e36656: f64 = (assign26380_e36654 * locals.var_qsuld);
        (assign26380_e36656, (assign26380_e36654 * locals.var_qsuld_dn0), (assign26380_e36654 * locals.var_qsuld_dn2), (assign26380_e36654 * locals.var_qsuld_dn6), (assign26380_e36654 * locals.var_qsuld_dn7), (assign26380_e36654 * locals.var_qsuld_dn10), (assign26380_e36654 * locals.var_qsuld_dn11), (assign26380_e36654 * locals.var_qsuld_dn12), (assign26380_e36654 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign26380_e36658;
        locals.var_qbody_bt_p_sus_dn0 = assign26380_e36658_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign26380_e36658_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign26380_e36658_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign26380_e36658_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign26380_e36658_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign26380_e36658_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign26380_e36658_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign26380_e36658_d_n17;

        let (assign26390_e36673, assign26390_e36673_d_n0, assign26390_e36673_d_n2, assign26390_e36673_d_n6, assign26390_e36673_d_n7, assign26390_e36673_d_n10, assign26390_e36673_d_n11, assign26390_e36673_d_n12, assign26390_e36673_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26390_e36669: f64 = (-locals.var_uc_areabt);
        let assign26390_e36671: f64 = (assign26390_e36669 * locals.var_qiuld);
        (assign26390_e36671, (assign26390_e36669 * locals.var_qiuld_dn0), (assign26390_e36669 * locals.var_qiuld_dn2), (assign26390_e36669 * locals.var_qiuld_dn6), (assign26390_e36669 * locals.var_qiuld_dn7), (assign26390_e36669 * locals.var_qiuld_dn10), (assign26390_e36669 * locals.var_qiuld_dn11), (assign26390_e36669 * locals.var_qiuld_dn12), (assign26390_e36669 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign26390_e36673;
        locals.var_qbody_bt_p_ius_dn0 = assign26390_e36673_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign26390_e36673_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign26390_e36673_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign26390_e36673_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign26390_e36673_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign26390_e36673_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign26390_e36673_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign26390_e36673_d_n17;

        let (assign26400_e36688, assign26400_e36688_d_n0, assign26400_e36688_d_n2, assign26400_e36688_d_n6, assign26400_e36688_d_n7, assign26400_e36688_d_n10, assign26400_e36688_d_n11, assign26400_e36688_d_n12, assign26400_e36688_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26400_e36684: f64 = (-locals.var_uc_areabt);
        let assign26400_e36686: f64 = (assign26400_e36684 * locals.var_qsuld);
        (assign26400_e36686, (assign26400_e36684 * locals.var_qsuld_dn0), (assign26400_e36684 * locals.var_qsuld_dn2), (assign26400_e36684 * locals.var_qsuld_dn6), (assign26400_e36684 * locals.var_qsuld_dn7), (assign26400_e36684 * locals.var_qsuld_dn10), (assign26400_e36684 * locals.var_qsuld_dn11), (assign26400_e36684 * locals.var_qsuld_dn12), (assign26400_e36684 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign26400_e36688;
        locals.var_qbody_bt_p_sud_dn0 = assign26400_e36688_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign26400_e36688_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign26400_e36688_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign26400_e36688_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign26400_e36688_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign26400_e36688_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign26400_e36688_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign26400_e36688_d_n17;

        let (assign26410_e36703, assign26410_e36703_d_n0, assign26410_e36703_d_n2, assign26410_e36703_d_n6, assign26410_e36703_d_n7, assign26410_e36703_d_n10, assign26410_e36703_d_n11, assign26410_e36703_d_n12, assign26410_e36703_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26410_e36699: f64 = (-locals.var_uc_areabt);
        let assign26410_e36701: f64 = (assign26410_e36699 * locals.var_qiuld);
        (assign26410_e36701, (assign26410_e36699 * locals.var_qiuld_dn0), (assign26410_e36699 * locals.var_qiuld_dn2), (assign26410_e36699 * locals.var_qiuld_dn6), (assign26410_e36699 * locals.var_qiuld_dn7), (assign26410_e36699 * locals.var_qiuld_dn10), (assign26410_e36699 * locals.var_qiuld_dn11), (assign26410_e36699 * locals.var_qiuld_dn12), (assign26410_e36699 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign26410_e36703;
        locals.var_qbody_bt_p_iud_dn0 = assign26410_e36703_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign26410_e36703_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign26410_e36703_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign26410_e36703_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign26410_e36703_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign26410_e36703_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign26410_e36703_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign26410_e36703_d_n17;

        let (assign26420_e36721, assign26420_e36721_d_n0, assign26420_e36721_d_n2, assign26420_e36721_d_n6, assign26420_e36721_d_n7, assign26420_e36721_d_n10, assign26420_e36721_d_n11, assign26420_e36721_d_n12, assign26420_e36721_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard869 != 0.0) && (locals.var_guard868 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26420_e36717: f64 = (-locals.var_uc_areabt);
        let assign26420_e36719: f64 = (assign26420_e36717 * locals.var_qsuld);
        (assign26420_e36719, (assign26420_e36717 * locals.var_qsuld_dn0), (assign26420_e36717 * locals.var_qsuld_dn2), (assign26420_e36717 * locals.var_qsuld_dn6), (assign26420_e36717 * locals.var_qsuld_dn7), (assign26420_e36717 * locals.var_qsuld_dn10), (assign26420_e36717 * locals.var_qsuld_dn11), (assign26420_e36717 * locals.var_qsuld_dn12), (assign26420_e36717 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign26420_e36721;
        locals.var_qbody_bt_n_sus_dn0 = assign26420_e36721_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign26420_e36721_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign26420_e36721_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign26420_e36721_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign26420_e36721_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign26420_e36721_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign26420_e36721_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign26420_e36721_d_n17;

        let (assign26430_e36739, assign26430_e36739_d_n0, assign26430_e36739_d_n2, assign26430_e36739_d_n6, assign26430_e36739_d_n7, assign26430_e36739_d_n10, assign26430_e36739_d_n11, assign26430_e36739_d_n12, assign26430_e36739_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard869 != 0.0) && (locals.var_guard868 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26430_e36735: f64 = (-locals.var_uc_areabt);
        let assign26430_e36737: f64 = (assign26430_e36735 * locals.var_qiuld);
        (assign26430_e36737, (assign26430_e36735 * locals.var_qiuld_dn0), (assign26430_e36735 * locals.var_qiuld_dn2), (assign26430_e36735 * locals.var_qiuld_dn6), (assign26430_e36735 * locals.var_qiuld_dn7), (assign26430_e36735 * locals.var_qiuld_dn10), (assign26430_e36735 * locals.var_qiuld_dn11), (assign26430_e36735 * locals.var_qiuld_dn12), (assign26430_e36735 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign26430_e36739;
        locals.var_qbody_bt_n_ius_dn0 = assign26430_e36739_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign26430_e36739_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign26430_e36739_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign26430_e36739_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign26430_e36739_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign26430_e36739_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign26430_e36739_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign26430_e36739_d_n17;

        let (assign26440_e36757, assign26440_e36757_d_n0, assign26440_e36757_d_n2, assign26440_e36757_d_n6, assign26440_e36757_d_n7, assign26440_e36757_d_n10, assign26440_e36757_d_n11, assign26440_e36757_d_n12, assign26440_e36757_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard869 != 0.0) && (locals.var_guard868 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26440_e36753: f64 = (-locals.var_uc_areabt);
        let assign26440_e36755: f64 = (assign26440_e36753 * locals.var_qsuld);
        (assign26440_e36755, (assign26440_e36753 * locals.var_qsuld_dn0), (assign26440_e36753 * locals.var_qsuld_dn2), (assign26440_e36753 * locals.var_qsuld_dn6), (assign26440_e36753 * locals.var_qsuld_dn7), (assign26440_e36753 * locals.var_qsuld_dn10), (assign26440_e36753 * locals.var_qsuld_dn11), (assign26440_e36753 * locals.var_qsuld_dn12), (assign26440_e36753 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign26440_e36757;
        locals.var_qbody_bt_n_sud_dn0 = assign26440_e36757_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign26440_e36757_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign26440_e36757_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign26440_e36757_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign26440_e36757_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign26440_e36757_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign26440_e36757_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign26440_e36757_d_n17;

        let (assign26450_e36775, assign26450_e36775_d_n0, assign26450_e36775_d_n2, assign26450_e36775_d_n6, assign26450_e36775_d_n7, assign26450_e36775_d_n10, assign26450_e36775_d_n11, assign26450_e36775_d_n12, assign26450_e36775_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard869 != 0.0) && (locals.var_guard868 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26450_e36771: f64 = (-locals.var_uc_areabt);
        let assign26450_e36773: f64 = (assign26450_e36771 * locals.var_qiuld);
        (assign26450_e36773, (assign26450_e36771 * locals.var_qiuld_dn0), (assign26450_e36771 * locals.var_qiuld_dn2), (assign26450_e36771 * locals.var_qiuld_dn6), (assign26450_e36771 * locals.var_qiuld_dn7), (assign26450_e36771 * locals.var_qiuld_dn10), (assign26450_e36771 * locals.var_qiuld_dn11), (assign26450_e36771 * locals.var_qiuld_dn12), (assign26450_e36771 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign26450_e36775;
        locals.var_qbody_bt_n_iud_dn0 = assign26450_e36775_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign26450_e36775_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign26450_e36775_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign26450_e36775_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign26450_e36775_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign26450_e36775_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign26450_e36775_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign26450_e36775_d_n17;

        let (assign26460_e36787,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26460_e36783: f64 = (1.0 - 1.0);
        let assign26460_e36785: f64 = (assign26460_e36783 / 2.0);
        (assign26460_e36785,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign26460_e36787;

        let (assign26470_e36799,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26470_e36795: f64 = (1.0 + 1.0);
        let assign26470_e36797: f64 = (assign26470_e36795 / 2.0);
        (assign26470_e36797,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign26470_e36799;

        let (assign26480_e36815, assign26480_e36815_d_n0, assign26480_e36815_d_n2, assign26480_e36815_d_n6, assign26480_e36815_d_n7, assign26480_e36815_d_n10, assign26480_e36815_d_n11, assign26480_e36815_d_n12, assign26480_e36815_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26480_e36807: f64 = (locals.var_modenml * locals.var_vbs);
        let assign26480_e36811: f64 = (locals.var_vbs - locals.var_vds);
        let assign26480_e36812: f64 = (locals.var_modervs * assign26480_e36811);
        let assign26480_e36813: f64 = (assign26480_e36807 + assign26480_e36812);
        (assign26480_e36813, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign26480_e36815;
        locals.var_vbsgmt_dn0 = assign26480_e36815_d_n0;
        locals.var_vbsgmt_dn2 = assign26480_e36815_d_n2;
        locals.var_vbsgmt_dn6 = assign26480_e36815_d_n6;
        locals.var_vbsgmt_dn7 = assign26480_e36815_d_n7;
        locals.var_vbsgmt_dn10 = assign26480_e36815_d_n10;
        locals.var_vbsgmt_dn11 = assign26480_e36815_d_n11;
        locals.var_vbsgmt_dn12 = assign26480_e36815_d_n12;
        locals.var_vbsgmt_dn17 = assign26480_e36815_d_n17;

        let (assign26490_e36830, assign26490_e36830_d_n0, assign26490_e36830_d_n2, assign26490_e36830_d_n6, assign26490_e36830_d_n7, assign26490_e36830_d_n10, assign26490_e36830_d_n11, assign26490_e36830_d_n12, assign26490_e36830_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26490_e36823: f64 = (locals.var_modenml * locals.var_vds);
        let assign26490_e36826: f64 = (-locals.var_vds);
        let assign26490_e36827: f64 = (locals.var_modervs * assign26490_e36826);
        let assign26490_e36828: f64 = (assign26490_e36823 + assign26490_e36827);
        (assign26490_e36828, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign26490_e36830;
        locals.var_vdsgmt_dn0 = assign26490_e36830_d_n0;
        locals.var_vdsgmt_dn2 = assign26490_e36830_d_n2;
        locals.var_vdsgmt_dn6 = assign26490_e36830_d_n6;
        locals.var_vdsgmt_dn7 = assign26490_e36830_d_n7;
        locals.var_vdsgmt_dn10 = assign26490_e36830_d_n10;
        locals.var_vdsgmt_dn11 = assign26490_e36830_d_n11;
        locals.var_vdsgmt_dn12 = assign26490_e36830_d_n12;
        locals.var_vdsgmt_dn17 = assign26490_e36830_d_n17;

        let (assign26500_e36846, assign26500_e36846_d_n0, assign26500_e36846_d_n2, assign26500_e36846_d_n6, assign26500_e36846_d_n7, assign26500_e36846_d_n10, assign26500_e36846_d_n11, assign26500_e36846_d_n12, assign26500_e36846_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26500_e36838: f64 = (locals.var_modenml * locals.var_vgs);
        let assign26500_e36842: f64 = (locals.var_vgs - locals.var_vds);
        let assign26500_e36843: f64 = (locals.var_modervs * assign26500_e36842);
        let assign26500_e36844: f64 = (assign26500_e36838 + assign26500_e36843);
        (assign26500_e36844, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign26500_e36846;
        locals.var_vgsgmt_dn0 = assign26500_e36846_d_n0;
        locals.var_vgsgmt_dn2 = assign26500_e36846_d_n2;
        locals.var_vgsgmt_dn6 = assign26500_e36846_d_n6;
        locals.var_vgsgmt_dn7 = assign26500_e36846_d_n7;
        locals.var_vgsgmt_dn10 = assign26500_e36846_d_n10;
        locals.var_vgsgmt_dn11 = assign26500_e36846_d_n11;
        locals.var_vgsgmt_dn12 = assign26500_e36846_d_n12;
        locals.var_vgsgmt_dn17 = assign26500_e36846_d_n17;

        let (assign26510_e36862, assign26510_e36862_d_n0, assign26510_e36862_d_n2, assign26510_e36862_d_n6, assign26510_e36862_d_n7, assign26510_e36862_d_n10, assign26510_e36862_d_n11, assign26510_e36862_d_n12, assign26510_e36862_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26510_e36854: f64 = (locals.var_modervs * locals.var_vgs);
        let assign26510_e36858: f64 = (locals.var_vgs - locals.var_vds);
        let assign26510_e36859: f64 = (locals.var_modenml * assign26510_e36858);
        let assign26510_e36860: f64 = (assign26510_e36854 + assign26510_e36859);
        (assign26510_e36860, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign26510_e36862;
        locals.var_vgdgmt_dn0 = assign26510_e36862_d_n0;
        locals.var_vgdgmt_dn2 = assign26510_e36862_d_n2;
        locals.var_vgdgmt_dn6 = assign26510_e36862_d_n6;
        locals.var_vgdgmt_dn7 = assign26510_e36862_d_n7;
        locals.var_vgdgmt_dn10 = assign26510_e36862_d_n10;
        locals.var_vgdgmt_dn11 = assign26510_e36862_d_n11;
        locals.var_vgdgmt_dn12 = assign26510_e36862_d_n12;
        locals.var_vgdgmt_dn17 = assign26510_e36862_d_n17;

        let (assign26520_e36872, assign26520_e36872_d_n0, assign26520_e36872_d_n2, assign26520_e36872_d_n6, assign26520_e36872_d_n7, assign26520_e36872_d_n10, assign26520_e36872_d_n11, assign26520_e36872_d_n12, assign26520_e36872_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26520_e36870: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign26520_e36870, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign26520_e36872;
        locals.var_vdbgmt_dn0 = assign26520_e36872_d_n0;
        locals.var_vdbgmt_dn2 = assign26520_e36872_d_n2;
        locals.var_vdbgmt_dn6 = assign26520_e36872_d_n6;
        locals.var_vdbgmt_dn7 = assign26520_e36872_d_n7;
        locals.var_vdbgmt_dn10 = assign26520_e36872_d_n10;
        locals.var_vdbgmt_dn11 = assign26520_e36872_d_n11;
        locals.var_vdbgmt_dn12 = assign26520_e36872_d_n12;
        locals.var_vdbgmt_dn17 = assign26520_e36872_d_n17;

        let (assign26530_e36881, assign26530_e36881_d_n0, assign26530_e36881_d_n2, assign26530_e36881_d_n6, assign26530_e36881_d_n7, assign26530_e36881_d_n10, assign26530_e36881_d_n11, assign26530_e36881_d_n12, assign26530_e36881_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26530_e36879: f64 = (-locals.var_vbsgmt);
        (assign26530_e36879, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign26530_e36881;
        locals.var_vsbgmt_dn0 = assign26530_e36881_d_n0;
        locals.var_vsbgmt_dn2 = assign26530_e36881_d_n2;
        locals.var_vsbgmt_dn6 = assign26530_e36881_d_n6;
        locals.var_vsbgmt_dn7 = assign26530_e36881_d_n7;
        locals.var_vsbgmt_dn10 = assign26530_e36881_d_n10;
        locals.var_vsbgmt_dn11 = assign26530_e36881_d_n11;
        locals.var_vsbgmt_dn12 = assign26530_e36881_d_n12;
        locals.var_vsbgmt_dn17 = assign26530_e36881_d_n17;

        let (assign26540_e36895,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26540_e36889: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign26540_e36892: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign26540_e36893: f64 = (assign26540_e36889 + assign26540_e36892);
        (assign26540_e36893,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign26540_e36895;

        let (assign26550_e36909,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26550_e36903: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign26550_e36906: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign26550_e36907: f64 = (assign26550_e36903 + assign26550_e36906);
        (assign26550_e36907,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign26550_e36909;

    }

    pub(super) fn stamp_transient_block_91(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26560_e36923, assign26560_e36923_d_n0, assign26560_e36923_d_n2, assign26560_e36923_d_n6, assign26560_e36923_d_n7, assign26560_e36923_d_n10, assign26560_e36923_d_n11, assign26560_e36923_d_n12, assign26560_e36923_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26560_e36917: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign26560_e36920: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign26560_e36921: f64 = (assign26560_e36917 + assign26560_e36920);
        (assign26560_e36921, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign26560_e36923;
        locals.var_vgbgmt_dn0 = assign26560_e36923_d_n0;
        locals.var_vgbgmt_dn2 = assign26560_e36923_d_n2;
        locals.var_vgbgmt_dn6 = assign26560_e36923_d_n6;
        locals.var_vgbgmt_dn7 = assign26560_e36923_d_n7;
        locals.var_vgbgmt_dn10 = assign26560_e36923_d_n10;
        locals.var_vgbgmt_dn11 = assign26560_e36923_d_n11;
        locals.var_vgbgmt_dn12 = assign26560_e36923_d_n12;
        locals.var_vgbgmt_dn17 = assign26560_e36923_d_n17;

        let (assign26570_e36941, assign26570_e36941_d_n0, assign26570_e36941_d_n2, assign26570_e36941_d_n6, assign26570_e36941_d_n7, assign26570_e36941_d_n10, assign26570_e36941_d_n11, assign26570_e36941_d_n12, assign26570_e36941_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26570_e36931: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign26570_e36934: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign26570_e36935: f64 = (assign26570_e36931 + assign26570_e36934);
        let assign26570_e36938: f64 = (10.0 * 2.220446049250313e-16);
        let assign26570_e36939: f64 = (assign26570_e36935 + assign26570_e36938);
        (assign26570_e36939, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign26570_e36941;
        locals.var_vxbgmt_dn0 = assign26570_e36941_d_n0;
        locals.var_vxbgmt_dn2 = assign26570_e36941_d_n2;
        locals.var_vxbgmt_dn6 = assign26570_e36941_d_n6;
        locals.var_vxbgmt_dn7 = assign26570_e36941_d_n7;
        locals.var_vxbgmt_dn10 = assign26570_e36941_d_n10;
        locals.var_vxbgmt_dn11 = assign26570_e36941_d_n11;
        locals.var_vxbgmt_dn12 = assign26570_e36941_d_n12;
        locals.var_vxbgmt_dn17 = assign26570_e36941_d_n17;

        let (assign26580_e36950, assign26580_e36950_d_n0, assign26580_e36950_d_n2, assign26580_e36950_d_n6, assign26580_e36950_d_n7, assign26580_e36950_d_n10, assign26580_e36950_d_n11, assign26580_e36950_d_n12, assign26580_e36950_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26580_e36948: f64 = (-locals.var_vxbgmt);
        (assign26580_e36948, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign26580_e36950;
        locals.var_t0__blk770_dn0 = assign26580_e36950_d_n0;
        locals.var_t0__blk770_dn2 = assign26580_e36950_d_n2;
        locals.var_t0__blk770_dn6 = assign26580_e36950_d_n6;
        locals.var_t0__blk770_dn7 = assign26580_e36950_d_n7;
        locals.var_t0__blk770_dn10 = assign26580_e36950_d_n10;
        locals.var_t0__blk770_dn11 = assign26580_e36950_d_n11;
        locals.var_t0__blk770_dn12 = assign26580_e36950_d_n12;
        locals.var_t0__blk770_dn17 = assign26580_e36950_d_n17;

        let assign26590_e36953: f64 = if locals.var_t0__blk770 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard870 = assign26590_e36953;

        let (assign26600_e36965, assign26600_e36965_d_n0, assign26600_e36965_d_n2, assign26600_e36965_d_n6, assign26600_e36965_d_n7, assign26600_e36965_d_n10, assign26600_e36965_d_n11, assign26600_e36965_d_n12, assign26600_e36965_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26600_e36963: f64 = (locals.var_t0__blk770 - locals.var_vbs_bnd);
        (assign26600_e36963, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26600_e36965;
        locals.var_t1__blk771_dn0 = assign26600_e36965_d_n0;
        locals.var_t1__blk771_dn2 = assign26600_e36965_d_n2;
        locals.var_t1__blk771_dn6 = assign26600_e36965_d_n6;
        locals.var_t1__blk771_dn7 = assign26600_e36965_d_n7;
        locals.var_t1__blk771_dn10 = assign26600_e36965_d_n10;
        locals.var_t1__blk771_dn11 = assign26600_e36965_d_n11;
        locals.var_t1__blk771_dn12 = assign26600_e36965_d_n12;
        locals.var_t1__blk771_dn17 = assign26600_e36965_d_n17;

        let (assign26610_e36977, assign26610_e36977_d_n0, assign26610_e36977_d_n2, assign26610_e36977_d_n6, assign26610_e36977_d_n7, assign26610_e36977_d_n10, assign26610_e36977_d_n11, assign26610_e36977_d_n12, assign26610_e36977_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26610_e36975: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign26610_e36975, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign26610_e36977;
        locals.var_t2__blk772_dn0 = assign26610_e36977_d_n0;
        locals.var_t2__blk772_dn2 = assign26610_e36977_d_n2;
        locals.var_t2__blk772_dn6 = assign26610_e36977_d_n6;
        locals.var_t2__blk772_dn7 = assign26610_e36977_d_n7;
        locals.var_t2__blk772_dn10 = assign26610_e36977_d_n10;
        locals.var_t2__blk772_dn11 = assign26610_e36977_d_n11;
        locals.var_t2__blk772_dn12 = assign26610_e36977_d_n12;
        locals.var_t2__blk772_dn17 = assign26610_e36977_d_n17;

        let (assign26620_e36989, assign26620_e36989_d_n0, assign26620_e36989_d_n2, assign26620_e36989_d_n6, assign26620_e36989_d_n7, assign26620_e36989_d_n10, assign26620_e36989_d_n11, assign26620_e36989_d_n12, assign26620_e36989_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26620_e36987: f64 = (locals.var_t1__blk771 / locals.var_t2__blk772);
        (assign26620_e36987, (((locals.var_t1__blk771_dn0 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn0)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn2 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn2)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn6 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn6)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn7 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn7)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn10 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn10)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn11 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn11)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn12 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn12)) / (locals.var_t2__blk772 * locals.var_t2__blk772)), (((locals.var_t1__blk771_dn17 * locals.var_t2__blk772) - (locals.var_t1__blk771 * locals.var_t2__blk772_dn17)) / (locals.var_t2__blk772 * locals.var_t2__blk772)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign26620_e36989;
        locals.var_tmf1_dn0 = assign26620_e36989_d_n0;
        locals.var_tmf1_dn2 = assign26620_e36989_d_n2;
        locals.var_tmf1_dn6 = assign26620_e36989_d_n6;
        locals.var_tmf1_dn7 = assign26620_e36989_d_n7;
        locals.var_tmf1_dn10 = assign26620_e36989_d_n10;
        locals.var_tmf1_dn11 = assign26620_e36989_d_n11;
        locals.var_tmf1_dn12 = assign26620_e36989_d_n12;
        locals.var_tmf1_dn17 = assign26620_e36989_d_n17;

        let (assign26630_e37001, assign26630_e37001_d_n0, assign26630_e37001_d_n2, assign26630_e37001_d_n6, assign26630_e37001_d_n7, assign26630_e37001_d_n10, assign26630_e37001_d_n11, assign26630_e37001_d_n12, assign26630_e37001_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26630_e36999: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26630_e36999, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26630_e37001;
        locals.var_tmf2_dn0 = assign26630_e37001_d_n0;
        locals.var_tmf2_dn2 = assign26630_e37001_d_n2;
        locals.var_tmf2_dn6 = assign26630_e37001_d_n6;
        locals.var_tmf2_dn7 = assign26630_e37001_d_n7;
        locals.var_tmf2_dn10 = assign26630_e37001_d_n10;
        locals.var_tmf2_dn11 = assign26630_e37001_d_n11;
        locals.var_tmf2_dn12 = assign26630_e37001_d_n12;
        locals.var_tmf2_dn17 = assign26630_e37001_d_n17;

        let (assign26640_e37013, assign26640_e37013_d_n0, assign26640_e37013_d_n2, assign26640_e37013_d_n6, assign26640_e37013_d_n7, assign26640_e37013_d_n10, assign26640_e37013_d_n11, assign26640_e37013_d_n12, assign26640_e37013_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26640_e37011: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign26640_e37011, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign26640_e37013;
        locals.var_tmf3_dn0 = assign26640_e37013_d_n0;
        locals.var_tmf3_dn2 = assign26640_e37013_d_n2;
        locals.var_tmf3_dn6 = assign26640_e37013_d_n6;
        locals.var_tmf3_dn7 = assign26640_e37013_d_n7;
        locals.var_tmf3_dn10 = assign26640_e37013_d_n10;
        locals.var_tmf3_dn11 = assign26640_e37013_d_n11;
        locals.var_tmf3_dn12 = assign26640_e37013_d_n12;
        locals.var_tmf3_dn17 = assign26640_e37013_d_n17;

        let (assign26650_e37025, assign26650_e37025_d_n0, assign26650_e37025_d_n2, assign26650_e37025_d_n6, assign26650_e37025_d_n7, assign26650_e37025_d_n10, assign26650_e37025_d_n11, assign26650_e37025_d_n12, assign26650_e37025_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26650_e37023: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign26650_e37023, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign26650_e37025;
        locals.var_tmf4_dn0 = assign26650_e37025_d_n0;
        locals.var_tmf4_dn2 = assign26650_e37025_d_n2;
        locals.var_tmf4_dn6 = assign26650_e37025_d_n6;
        locals.var_tmf4_dn7 = assign26650_e37025_d_n7;
        locals.var_tmf4_dn10 = assign26650_e37025_d_n10;
        locals.var_tmf4_dn11 = assign26650_e37025_d_n11;
        locals.var_tmf4_dn12 = assign26650_e37025_d_n12;
        locals.var_tmf4_dn17 = assign26650_e37025_d_n17;

        let (assign26660_e37045, assign26660_e37045_d_n0, assign26660_e37045_d_n2, assign26660_e37045_d_n6, assign26660_e37045_d_n7, assign26660_e37045_d_n10, assign26660_e37045_d_n11, assign26660_e37045_d_n12, assign26660_e37045_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26660_e37036: f64 = (1.0 + locals.var_tmf1);
        let assign26660_e37038: f64 = (assign26660_e37036 + locals.var_tmf2);
        let assign26660_e37040: f64 = (assign26660_e37038 + locals.var_tmf3);
        let assign26660_e37042: f64 = (assign26660_e37040 + locals.var_tmf4);
        let assign26660_e37043: f64 = (1.0 / assign26660_e37042);
        (assign26660_e37043, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign26660_e37042 * assign26660_e37042))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign26660_e37042 * assign26660_e37042))),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign26660_e37045;
        locals.var_ty__blk778_dn0 = assign26660_e37045_d_n0;
        locals.var_ty__blk778_dn2 = assign26660_e37045_d_n2;
        locals.var_ty__blk778_dn6 = assign26660_e37045_d_n6;
        locals.var_ty__blk778_dn7 = assign26660_e37045_d_n7;
        locals.var_ty__blk778_dn10 = assign26660_e37045_d_n10;
        locals.var_ty__blk778_dn11 = assign26660_e37045_d_n11;
        locals.var_ty__blk778_dn12 = assign26660_e37045_d_n12;
        locals.var_ty__blk778_dn17 = assign26660_e37045_d_n17;

        let (assign26680_e37086, assign26680_e37086_d_n0, assign26680_e37086_d_n2, assign26680_e37086_d_n6, assign26680_e37086_d_n7, assign26680_e37086_d_n10, assign26680_e37086_d_n11, assign26680_e37086_d_n12, assign26680_e37086_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26680_e37083: f64 = (1.0 - locals.var_ty__blk778);
        let assign26680_e37084: f64 = (locals.var_t2__blk772 * assign26680_e37083);
        (assign26680_e37084, ((locals.var_t2__blk772_dn0 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn0))), ((locals.var_t2__blk772_dn2 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn2))), ((locals.var_t2__blk772_dn6 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn6))), ((locals.var_t2__blk772_dn7 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn7))), ((locals.var_t2__blk772_dn10 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn10))), ((locals.var_t2__blk772_dn11 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn11))), ((locals.var_t2__blk772_dn12 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn12))), ((locals.var_t2__blk772_dn17 * assign26680_e37083) + (locals.var_t2__blk772 * (-locals.var_ty__blk778_dn17))),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign26680_e37086;
        locals.var_ty__blk778_dn0 = assign26680_e37086_d_n0;
        locals.var_ty__blk778_dn2 = assign26680_e37086_d_n2;
        locals.var_ty__blk778_dn6 = assign26680_e37086_d_n6;
        locals.var_ty__blk778_dn7 = assign26680_e37086_d_n7;
        locals.var_ty__blk778_dn10 = assign26680_e37086_d_n10;
        locals.var_ty__blk778_dn11 = assign26680_e37086_d_n11;
        locals.var_ty__blk778_dn12 = assign26680_e37086_d_n12;
        locals.var_ty__blk778_dn17 = assign26680_e37086_d_n17;

        let (assign26700_e37109, assign26700_e37109_d_n0, assign26700_e37109_d_n2, assign26700_e37109_d_n6, assign26700_e37109_d_n7, assign26700_e37109_d_n10, assign26700_e37109_d_n11, assign26700_e37109_d_n12, assign26700_e37109_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26700_e37107: f64 = (locals.var_vbs_bnd + locals.var_ty__blk778);
        (assign26700_e37107, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    } else {
        (locals.var_t10__blk775, locals.var_t10__blk775_dn0, locals.var_t10__blk775_dn2, locals.var_t10__blk775_dn6, locals.var_t10__blk775_dn7, locals.var_t10__blk775_dn10, locals.var_t10__blk775_dn11, locals.var_t10__blk775_dn12, locals.var_t10__blk775_dn17,)
    }
};
        locals.var_t10__blk775 = assign26700_e37109;
        locals.var_t10__blk775_dn0 = assign26700_e37109_d_n0;
        locals.var_t10__blk775_dn2 = assign26700_e37109_d_n2;
        locals.var_t10__blk775_dn6 = assign26700_e37109_d_n6;
        locals.var_t10__blk775_dn7 = assign26700_e37109_d_n7;
        locals.var_t10__blk775_dn10 = assign26700_e37109_d_n10;
        locals.var_t10__blk775_dn11 = assign26700_e37109_d_n11;
        locals.var_t10__blk775_dn12 = assign26700_e37109_d_n12;
        locals.var_t10__blk775_dn17 = assign26700_e37109_d_n17;

        let (assign26710_e37120, assign26710_e37120_d_n0, assign26710_e37120_d_n2, assign26710_e37120_d_n6, assign26710_e37120_d_n7, assign26710_e37120_d_n10, assign26710_e37120_d_n11, assign26710_e37120_d_n12, assign26710_e37120_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard870 == 0.0)) {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    } else {
        (locals.var_t10__blk775, locals.var_t10__blk775_dn0, locals.var_t10__blk775_dn2, locals.var_t10__blk775_dn6, locals.var_t10__blk775_dn7, locals.var_t10__blk775_dn10, locals.var_t10__blk775_dn11, locals.var_t10__blk775_dn12, locals.var_t10__blk775_dn17,)
    }
};
        locals.var_t10__blk775 = assign26710_e37120;
        locals.var_t10__blk775_dn0 = assign26710_e37120_d_n0;
        locals.var_t10__blk775_dn2 = assign26710_e37120_d_n2;
        locals.var_t10__blk775_dn6 = assign26710_e37120_d_n6;
        locals.var_t10__blk775_dn7 = assign26710_e37120_d_n7;
        locals.var_t10__blk775_dn10 = assign26710_e37120_d_n10;
        locals.var_t10__blk775_dn11 = assign26710_e37120_d_n11;
        locals.var_t10__blk775_dn12 = assign26710_e37120_d_n12;
        locals.var_t10__blk775_dn17 = assign26710_e37120_d_n17;

        let (assign26730_e37142, assign26730_e37142_d_n0, assign26730_e37142_d_n2, assign26730_e37142_d_n6, assign26730_e37142_d_n7, assign26730_e37142_d_n10, assign26730_e37142_d_n11, assign26730_e37142_d_n12, assign26730_e37142_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26730_e37138: f64 = (-locals.var_t10__blk775);
        let assign26730_e37140: f64 = (assign26730_e37138 - 1e-12);
        (assign26730_e37140, (-locals.var_t10__blk775_dn0), (-locals.var_t10__blk775_dn2), (-locals.var_t10__blk775_dn6), (-locals.var_t10__blk775_dn7), (-locals.var_t10__blk775_dn10), (-locals.var_t10__blk775_dn11), (-locals.var_t10__blk775_dn12), (-locals.var_t10__blk775_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign26730_e37142;
        locals.var_vxbgmtcl_dn0 = assign26730_e37142_d_n0;
        locals.var_vxbgmtcl_dn2 = assign26730_e37142_d_n2;
        locals.var_vxbgmtcl_dn6 = assign26730_e37142_d_n6;
        locals.var_vxbgmtcl_dn7 = assign26730_e37142_d_n7;
        locals.var_vxbgmtcl_dn10 = assign26730_e37142_d_n10;
        locals.var_vxbgmtcl_dn11 = assign26730_e37142_d_n11;
        locals.var_vxbgmtcl_dn12 = assign26730_e37142_d_n12;
        locals.var_vxbgmtcl_dn17 = assign26730_e37142_d_n17;

        let (assign26740_e37152, assign26740_e37152_d_n0, assign26740_e37152_d_n2, assign26740_e37152_d_n6, assign26740_e37152_d_n7, assign26740_e37152_d_n10, assign26740_e37152_d_n11, assign26740_e37152_d_n12, assign26740_e37152_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26740_e37150: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign26740_e37150, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk800, locals.var_fac1__blk800_dn0, locals.var_fac1__blk800_dn2, locals.var_fac1__blk800_dn6, locals.var_fac1__blk800_dn7, locals.var_fac1__blk800_dn10, locals.var_fac1__blk800_dn11, locals.var_fac1__blk800_dn12, locals.var_fac1__blk800_dn17,)
    }
};
        locals.var_fac1__blk800 = assign26740_e37152;
        locals.var_fac1__blk800_dn0 = assign26740_e37152_d_n0;
        locals.var_fac1__blk800_dn2 = assign26740_e37152_d_n2;
        locals.var_fac1__blk800_dn6 = assign26740_e37152_d_n6;
        locals.var_fac1__blk800_dn7 = assign26740_e37152_d_n7;
        locals.var_fac1__blk800_dn10 = assign26740_e37152_d_n10;
        locals.var_fac1__blk800_dn11 = assign26740_e37152_d_n11;
        locals.var_fac1__blk800_dn12 = assign26740_e37152_d_n12;
        locals.var_fac1__blk800_dn17 = assign26740_e37152_d_n17;

        let (assign26750_e37162, assign26750_e37162_d_n0, assign26750_e37162_d_n2, assign26750_e37162_d_n6, assign26750_e37162_d_n7, assign26750_e37162_d_n10, assign26750_e37162_d_n11, assign26750_e37162_d_n12, assign26750_e37162_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26750_e37160: f64 = (locals.var_fac1__blk800 * locals.var_fac1__blk800);
        (assign26750_e37160, ((locals.var_fac1__blk800_dn0 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn0)), ((locals.var_fac1__blk800_dn2 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn2)), ((locals.var_fac1__blk800_dn6 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn6)), ((locals.var_fac1__blk800_dn7 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn7)), ((locals.var_fac1__blk800_dn10 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn10)), ((locals.var_fac1__blk800_dn11 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn11)), ((locals.var_fac1__blk800_dn12 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn12)), ((locals.var_fac1__blk800_dn17 * locals.var_fac1__blk800) + (locals.var_fac1__blk800 * locals.var_fac1__blk800_dn17)),)
    } else {
        (locals.var_fac1p2__blk801, locals.var_fac1p2__blk801_dn0, locals.var_fac1p2__blk801_dn2, locals.var_fac1p2__blk801_dn6, locals.var_fac1p2__blk801_dn7, locals.var_fac1p2__blk801_dn10, locals.var_fac1p2__blk801_dn11, locals.var_fac1p2__blk801_dn12, locals.var_fac1p2__blk801_dn17,)
    }
};
        locals.var_fac1p2__blk801 = assign26750_e37162;
        locals.var_fac1p2__blk801_dn0 = assign26750_e37162_d_n0;
        locals.var_fac1p2__blk801_dn2 = assign26750_e37162_d_n2;
        locals.var_fac1p2__blk801_dn6 = assign26750_e37162_d_n6;
        locals.var_fac1p2__blk801_dn7 = assign26750_e37162_d_n7;
        locals.var_fac1p2__blk801_dn10 = assign26750_e37162_d_n10;
        locals.var_fac1p2__blk801_dn11 = assign26750_e37162_d_n11;
        locals.var_fac1p2__blk801_dn12 = assign26750_e37162_d_n12;
        locals.var_fac1p2__blk801_dn17 = assign26750_e37162_d_n17;

        let (assign26760_e37172, assign26760_e37172_d_n0, assign26760_e37172_d_n2, assign26760_e37172_d_n6, assign26760_e37172_d_n7, assign26760_e37172_d_n10, assign26760_e37172_d_n11, assign26760_e37172_d_n12, assign26760_e37172_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26760_e37170: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign26760_e37170, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign26760_e37172;
        locals.var_vgpld_dn0 = assign26760_e37172_d_n0;
        locals.var_vgpld_dn2 = assign26760_e37172_d_n2;
        locals.var_vgpld_dn6 = assign26760_e37172_d_n6;
        locals.var_vgpld_dn7 = assign26760_e37172_d_n7;
        locals.var_vgpld_dn10 = assign26760_e37172_d_n10;
        locals.var_vgpld_dn11 = assign26760_e37172_d_n11;
        locals.var_vgpld_dn12 = assign26760_e37172_d_n12;
        locals.var_vgpld_dn17 = assign26760_e37172_d_n17;

        let (assign26770_e37182, assign26770_e37182_d_n0, assign26770_e37182_d_n2, assign26770_e37182_d_n6, assign26770_e37182_d_n7, assign26770_e37182_d_n10, assign26770_e37182_d_n11, assign26770_e37182_d_n12, assign26770_e37182_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26770_e37180: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign26770_e37180, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign26770_e37182;
        locals.var_t0__blk770_dn0 = assign26770_e37182_d_n0;
        locals.var_t0__blk770_dn2 = assign26770_e37182_d_n2;
        locals.var_t0__blk770_dn6 = assign26770_e37182_d_n6;
        locals.var_t0__blk770_dn7 = assign26770_e37182_d_n7;
        locals.var_t0__blk770_dn10 = assign26770_e37182_d_n10;
        locals.var_t0__blk770_dn11 = assign26770_e37182_d_n11;
        locals.var_t0__blk770_dn12 = assign26770_e37182_d_n12;
        locals.var_t0__blk770_dn17 = assign26770_e37182_d_n17;

        let (assign26780_e37195, assign26780_e37195_d_n0, assign26780_e37195_d_n2, assign26780_e37195_d_n6, assign26780_e37195_d_n7, assign26780_e37195_d_n10, assign26780_e37195_d_n11, assign26780_e37195_d_n12, assign26780_e37195_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26780_e37190: f64 = (2.0 / locals.var_beta);
        let assign26780_e37192: f64 = (locals.var_t0__blk770).ln();
        let assign26780_e37193: f64 = (assign26780_e37190 * assign26780_e37192);
        (assign26780_e37193, (assign26780_e37190 * (locals.var_t0__blk770_dn0 / locals.var_t0__blk770)), (assign26780_e37190 * (locals.var_t0__blk770_dn2 / locals.var_t0__blk770)), (assign26780_e37190 * (locals.var_t0__blk770_dn6 / locals.var_t0__blk770)), (assign26780_e37190 * (locals.var_t0__blk770_dn7 / locals.var_t0__blk770)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign26780_e37192) + (assign26780_e37190 * (locals.var_t0__blk770_dn10 / locals.var_t0__blk770))), (assign26780_e37190 * (locals.var_t0__blk770_dn11 / locals.var_t0__blk770)), (assign26780_e37190 * (locals.var_t0__blk770_dn12 / locals.var_t0__blk770)), (assign26780_e37190 * (locals.var_t0__blk770_dn17 / locals.var_t0__blk770)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign26780_e37195;
        locals.var_pb2over_dn0 = assign26780_e37195_d_n0;
        locals.var_pb2over_dn2 = assign26780_e37195_d_n2;
        locals.var_pb2over_dn6 = assign26780_e37195_d_n6;
        locals.var_pb2over_dn7 = assign26780_e37195_d_n7;
        locals.var_pb2over_dn10 = assign26780_e37195_d_n10;
        locals.var_pb2over_dn11 = assign26780_e37195_d_n11;
        locals.var_pb2over_dn12 = assign26780_e37195_d_n12;
        locals.var_pb2over_dn17 = assign26780_e37195_d_n17;

        let (assign26790_e37204,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign26790_e37202: f64 = (-locals.var_vxbgmtcl);
        (assign26790_e37202,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign26790_e37204;

        let assign26800_e37207: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard871 = assign26800_e37207;

        let (assign26820_e37232, assign26820_e37232_d_n0, assign26820_e37232_d_n2, assign26820_e37232_d_n6, assign26820_e37232_d_n7, assign26820_e37232_d_n10, assign26820_e37232_d_n11, assign26820_e37232_d_n12, assign26820_e37232_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26820_e37229: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign26820_e37230: f64 = (1.0 / assign26820_e37229);
        (assign26820_e37230, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign26820_e37229 * assign26820_e37229))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign26820_e37229 * assign26820_e37229))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign26820_e37229 * assign26820_e37229))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26820_e37232;
        locals.var_t1__blk771_dn0 = assign26820_e37232_d_n0;
        locals.var_t1__blk771_dn2 = assign26820_e37232_d_n2;
        locals.var_t1__blk771_dn6 = assign26820_e37232_d_n6;
        locals.var_t1__blk771_dn7 = assign26820_e37232_d_n7;
        locals.var_t1__blk771_dn10 = assign26820_e37232_d_n10;
        locals.var_t1__blk771_dn11 = assign26820_e37232_d_n11;
        locals.var_t1__blk771_dn12 = assign26820_e37232_d_n12;
        locals.var_t1__blk771_dn17 = assign26820_e37232_d_n17;

        let (assign26830_e37244, assign26830_e37244_d_n0, assign26830_e37244_d_n2, assign26830_e37244_d_n6, assign26830_e37244_d_n7, assign26830_e37244_d_n10, assign26830_e37244_d_n11, assign26830_e37244_d_n12, assign26830_e37244_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26830_e37242: f64 = (locals.var_t1__blk771 * locals.var_cox0);
        (assign26830_e37242, (locals.var_t1__blk771_dn0 * locals.var_cox0), (locals.var_t1__blk771_dn2 * locals.var_cox0), (locals.var_t1__blk771_dn6 * locals.var_cox0), (locals.var_t1__blk771_dn7 * locals.var_cox0), (locals.var_t1__blk771_dn10 * locals.var_cox0), (locals.var_t1__blk771_dn11 * locals.var_cox0), (locals.var_t1__blk771_dn12 * locals.var_cox0), (locals.var_t1__blk771_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign26830_e37244;
        locals.var_ty__blk778_dn0 = assign26830_e37244_d_n0;
        locals.var_ty__blk778_dn2 = assign26830_e37244_d_n2;
        locals.var_ty__blk778_dn6 = assign26830_e37244_d_n6;
        locals.var_ty__blk778_dn7 = assign26830_e37244_d_n7;
        locals.var_ty__blk778_dn10 = assign26830_e37244_d_n10;
        locals.var_ty__blk778_dn11 = assign26830_e37244_d_n11;
        locals.var_ty__blk778_dn12 = assign26830_e37244_d_n12;
        locals.var_ty__blk778_dn17 = assign26830_e37244_d_n17;

        let (assign26840_e37260, assign26840_e37260_d_n0, assign26840_e37260_d_n2, assign26840_e37260_d_n6, assign26840_e37260_d_n7, assign26840_e37260_d_n10, assign26840_e37260_d_n11, assign26840_e37260_d_n12, assign26840_e37260_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26840_e37255: f64 = (3.0 * 1.414213562373095);
        let assign26840_e37257: f64 = (assign26840_e37255 * locals.var_ty__blk778);
        let assign26840_e37258: f64 = (2.0 + assign26840_e37257);
        (assign26840_e37258, (assign26840_e37255 * locals.var_ty__blk778_dn0), (assign26840_e37255 * locals.var_ty__blk778_dn2), (assign26840_e37255 * locals.var_ty__blk778_dn6), (assign26840_e37255 * locals.var_ty__blk778_dn7), (assign26840_e37255 * locals.var_ty__blk778_dn10), (assign26840_e37255 * locals.var_ty__blk778_dn11), (assign26840_e37255 * locals.var_ty__blk778_dn12), (assign26840_e37255 * locals.var_ty__blk778_dn17),)
    } else {
        (locals.var_ac41__blk805, locals.var_ac41__blk805_dn0, locals.var_ac41__blk805_dn2, locals.var_ac41__blk805_dn6, locals.var_ac41__blk805_dn7, locals.var_ac41__blk805_dn10, locals.var_ac41__blk805_dn11, locals.var_ac41__blk805_dn12, locals.var_ac41__blk805_dn17,)
    }
};
        locals.var_ac41__blk805 = assign26840_e37260;
        locals.var_ac41__blk805_dn0 = assign26840_e37260_d_n0;
        locals.var_ac41__blk805_dn2 = assign26840_e37260_d_n2;
        locals.var_ac41__blk805_dn6 = assign26840_e37260_d_n6;
        locals.var_ac41__blk805_dn7 = assign26840_e37260_d_n7;
        locals.var_ac41__blk805_dn10 = assign26840_e37260_d_n10;
        locals.var_ac41__blk805_dn11 = assign26840_e37260_d_n11;
        locals.var_ac41__blk805_dn12 = assign26840_e37260_d_n12;
        locals.var_ac41__blk805_dn17 = assign26840_e37260_d_n17;

        let (assign26850_e37276, assign26850_e37276_d_n0, assign26850_e37276_d_n2, assign26850_e37276_d_n6, assign26850_e37276_d_n7, assign26850_e37276_d_n10, assign26850_e37276_d_n11, assign26850_e37276_d_n12, assign26850_e37276_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26850_e37270: f64 = (8.0 * locals.var_ac41__blk805);
        let assign26850_e37272: f64 = (assign26850_e37270 * locals.var_ac41__blk805);
        let assign26850_e37274: f64 = (assign26850_e37272 * locals.var_ac41__blk805);
        (assign26850_e37274, (((((8.0 * locals.var_ac41__blk805_dn0) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn0)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn0)), (((((8.0 * locals.var_ac41__blk805_dn2) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn2)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn2)), (((((8.0 * locals.var_ac41__blk805_dn6) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn6)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn6)), (((((8.0 * locals.var_ac41__blk805_dn7) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn7)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn7)), (((((8.0 * locals.var_ac41__blk805_dn10) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn10)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn10)), (((((8.0 * locals.var_ac41__blk805_dn11) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn11)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn11)), (((((8.0 * locals.var_ac41__blk805_dn12) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn12)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn12)), (((((8.0 * locals.var_ac41__blk805_dn17) * locals.var_ac41__blk805) + (assign26850_e37270 * locals.var_ac41__blk805_dn17)) * locals.var_ac41__blk805) + (assign26850_e37272 * locals.var_ac41__blk805_dn17)),)
    } else {
        (locals.var_ac4__blk806, locals.var_ac4__blk806_dn0, locals.var_ac4__blk806_dn2, locals.var_ac4__blk806_dn6, locals.var_ac4__blk806_dn7, locals.var_ac4__blk806_dn10, locals.var_ac4__blk806_dn11, locals.var_ac4__blk806_dn12, locals.var_ac4__blk806_dn17,)
    }
};
        locals.var_ac4__blk806 = assign26850_e37276;
        locals.var_ac4__blk806_dn0 = assign26850_e37276_d_n0;
        locals.var_ac4__blk806_dn2 = assign26850_e37276_d_n2;
        locals.var_ac4__blk806_dn6 = assign26850_e37276_d_n6;
        locals.var_ac4__blk806_dn7 = assign26850_e37276_d_n7;
        locals.var_ac4__blk806_dn10 = assign26850_e37276_d_n10;
        locals.var_ac4__blk806_dn11 = assign26850_e37276_d_n11;
        locals.var_ac4__blk806_dn12 = assign26850_e37276_d_n12;
        locals.var_ac4__blk806_dn17 = assign26850_e37276_d_n17;

        let (assign26860_e37288, assign26860_e37288_d_n0, assign26860_e37288_d_n2, assign26860_e37288_d_n6, assign26860_e37288_d_n7, assign26860_e37288_d_n10, assign26860_e37288_d_n11, assign26860_e37288_d_n12, assign26860_e37288_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26860_e37286: f64 = (locals.var_eg - locals.var_pb2over);
        (assign26860_e37286, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk807, locals.var_ps0_min__blk807_dn0, locals.var_ps0_min__blk807_dn2, locals.var_ps0_min__blk807_dn6, locals.var_ps0_min__blk807_dn7, locals.var_ps0_min__blk807_dn10, locals.var_ps0_min__blk807_dn11, locals.var_ps0_min__blk807_dn12, locals.var_ps0_min__blk807_dn17,)
    }
};
        locals.var_ps0_min__blk807 = assign26860_e37288;
        locals.var_ps0_min__blk807_dn0 = assign26860_e37288_d_n0;
        locals.var_ps0_min__blk807_dn2 = assign26860_e37288_d_n2;
        locals.var_ps0_min__blk807_dn6 = assign26860_e37288_d_n6;
        locals.var_ps0_min__blk807_dn7 = assign26860_e37288_d_n7;
        locals.var_ps0_min__blk807_dn10 = assign26860_e37288_d_n10;
        locals.var_ps0_min__blk807_dn11 = assign26860_e37288_d_n11;
        locals.var_ps0_min__blk807_dn12 = assign26860_e37288_d_n12;
        locals.var_ps0_min__blk807_dn17 = assign26860_e37288_d_n17;

        let (assign26870_e37302, assign26870_e37302_d_n0, assign26870_e37302_d_n2, assign26870_e37302_d_n6, assign26870_e37302_d_n7, assign26870_e37302_d_n10, assign26870_e37302_d_n11, assign26870_e37302_d_n12, assign26870_e37302_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26870_e37299: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign26870_e37300: f64 = (locals.var_beta * assign26870_e37299);
        (assign26870_e37300, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26870_e37299) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign26870_e37302;
        locals.var_tx__blk777_dn0 = assign26870_e37302_d_n0;
        locals.var_tx__blk777_dn2 = assign26870_e37302_d_n2;
        locals.var_tx__blk777_dn6 = assign26870_e37302_d_n6;
        locals.var_tx__blk777_dn7 = assign26870_e37302_d_n7;
        locals.var_tx__blk777_dn10 = assign26870_e37302_d_n10;
        locals.var_tx__blk777_dn11 = assign26870_e37302_d_n11;
        locals.var_tx__blk777_dn12 = assign26870_e37302_d_n12;
        locals.var_tx__blk777_dn17 = assign26870_e37302_d_n17;

        let (assign26880_e37322, assign26880_e37322_d_n0, assign26880_e37322_d_n2, assign26880_e37322_d_n6, assign26880_e37322_d_n7, assign26880_e37322_d_n10, assign26880_e37322_d_n11, assign26880_e37322_d_n12, assign26880_e37322_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26880_e37312: f64 = (7.0 * 1.414213562373095);
        let assign26880_e37315: f64 = (9.0 * locals.var_ty__blk778);
        let assign26880_e37318: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26880_e37319: f64 = (assign26880_e37315 * assign26880_e37318);
        let assign26880_e37320: f64 = (assign26880_e37312 - assign26880_e37319);
        (assign26880_e37320, (-(((9.0 * locals.var_ty__blk778_dn0) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn0))), (-(((9.0 * locals.var_ty__blk778_dn2) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn2))), (-(((9.0 * locals.var_ty__blk778_dn6) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn6))), (-(((9.0 * locals.var_ty__blk778_dn7) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn7))), (-(((9.0 * locals.var_ty__blk778_dn10) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn10))), (-(((9.0 * locals.var_ty__blk778_dn11) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn11))), (-(((9.0 * locals.var_ty__blk778_dn12) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn12))), (-(((9.0 * locals.var_ty__blk778_dn17) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac31__blk808, locals.var_ac31__blk808_dn0, locals.var_ac31__blk808_dn2, locals.var_ac31__blk808_dn6, locals.var_ac31__blk808_dn7, locals.var_ac31__blk808_dn10, locals.var_ac31__blk808_dn11, locals.var_ac31__blk808_dn12, locals.var_ac31__blk808_dn17,)
    }
};
        locals.var_ac31__blk808 = assign26880_e37322;
        locals.var_ac31__blk808_dn0 = assign26880_e37322_d_n0;
        locals.var_ac31__blk808_dn2 = assign26880_e37322_d_n2;
        locals.var_ac31__blk808_dn6 = assign26880_e37322_d_n6;
        locals.var_ac31__blk808_dn7 = assign26880_e37322_d_n7;
        locals.var_ac31__blk808_dn10 = assign26880_e37322_d_n10;
        locals.var_ac31__blk808_dn11 = assign26880_e37322_d_n11;
        locals.var_ac31__blk808_dn12 = assign26880_e37322_d_n12;
        locals.var_ac31__blk808_dn17 = assign26880_e37322_d_n17;

    }

    pub(super) fn stamp_transient_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26890_e37334, assign26890_e37334_d_n0, assign26890_e37334_d_n2, assign26890_e37334_d_n6, assign26890_e37334_d_n7, assign26890_e37334_d_n10, assign26890_e37334_d_n11, assign26890_e37334_d_n12, assign26890_e37334_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26890_e37332: f64 = (locals.var_ac31__blk808 * locals.var_ac31__blk808);
        (assign26890_e37332, ((locals.var_ac31__blk808_dn0 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn0)), ((locals.var_ac31__blk808_dn2 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn2)), ((locals.var_ac31__blk808_dn6 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn6)), ((locals.var_ac31__blk808_dn7 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn7)), ((locals.var_ac31__blk808_dn10 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn10)), ((locals.var_ac31__blk808_dn11 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn11)), ((locals.var_ac31__blk808_dn12 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn12)), ((locals.var_ac31__blk808_dn17 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn17)),)
    } else {
        (locals.var_ac3__blk809, locals.var_ac3__blk809_dn0, locals.var_ac3__blk809_dn2, locals.var_ac3__blk809_dn6, locals.var_ac3__blk809_dn7, locals.var_ac3__blk809_dn10, locals.var_ac3__blk809_dn11, locals.var_ac3__blk809_dn12, locals.var_ac3__blk809_dn17,)
    }
};
        locals.var_ac3__blk809 = assign26890_e37334;
        locals.var_ac3__blk809_dn0 = assign26890_e37334_d_n0;
        locals.var_ac3__blk809_dn2 = assign26890_e37334_d_n2;
        locals.var_ac3__blk809_dn6 = assign26890_e37334_d_n6;
        locals.var_ac3__blk809_dn7 = assign26890_e37334_d_n7;
        locals.var_ac3__blk809_dn10 = assign26890_e37334_d_n10;
        locals.var_ac3__blk809_dn11 = assign26890_e37334_d_n11;
        locals.var_ac3__blk809_dn12 = assign26890_e37334_d_n12;
        locals.var_ac3__blk809_dn17 = assign26890_e37334_d_n17;

        let assign26900_e37338: f64 = (locals.var_ac3__blk809 * 1e-8);
        let assign26900_e37339: f64 = if locals.var_ac4__blk806 < assign26900_e37338 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign26900_e37339;

        let (assign26910_e37370, assign26910_e37370_d_n0, assign26910_e37370_d_n2, assign26910_e37370_d_n6, assign26910_e37370_d_n7, assign26910_e37370_d_n10, assign26910_e37370_d_n11, assign26910_e37370_d_n12, assign26910_e37370_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26910_e37350: f64 = (-7.0);
        let assign26910_e37352: f64 = (assign26910_e37350 * 1.414213562373095);
        let assign26910_e37354: f64 = (assign26910_e37352 + locals.var_ac31__blk808);
        let assign26910_e37357: f64 = (0.5 * locals.var_ac4__blk806);
        let assign26910_e37359: f64 = (assign26910_e37357 / locals.var_ac31__blk808);
        let assign26910_e37360: f64 = (assign26910_e37354 + assign26910_e37359);
        let assign26910_e37363: f64 = (9.0 * locals.var_ty__blk778);
        let assign26910_e37366: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26910_e37367: f64 = (assign26910_e37363 * assign26910_e37366);
        let assign26910_e37368: f64 = (assign26910_e37360 + assign26910_e37367);
        (assign26910_e37368, ((locals.var_ac31__blk808_dn0 + ((((0.5 * locals.var_ac4__blk806_dn0) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn0)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn0) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn0))), ((locals.var_ac31__blk808_dn2 + ((((0.5 * locals.var_ac4__blk806_dn2) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn2)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn2) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn2))), ((locals.var_ac31__blk808_dn6 + ((((0.5 * locals.var_ac4__blk806_dn6) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn6)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn6) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn6))), ((locals.var_ac31__blk808_dn7 + ((((0.5 * locals.var_ac4__blk806_dn7) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn7)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn7) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn7))), ((locals.var_ac31__blk808_dn10 + ((((0.5 * locals.var_ac4__blk806_dn10) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn10)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn10) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn10))), ((locals.var_ac31__blk808_dn11 + ((((0.5 * locals.var_ac4__blk806_dn11) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn11)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn11) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn11))), ((locals.var_ac31__blk808_dn12 + ((((0.5 * locals.var_ac4__blk806_dn12) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn12)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn12) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn12))), ((locals.var_ac31__blk808_dn17 + ((((0.5 * locals.var_ac4__blk806_dn17) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn17)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn17) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign26910_e37370;
        locals.var_ac1__blk811_dn0 = assign26910_e37370_d_n0;
        locals.var_ac1__blk811_dn2 = assign26910_e37370_d_n2;
        locals.var_ac1__blk811_dn6 = assign26910_e37370_d_n6;
        locals.var_ac1__blk811_dn7 = assign26910_e37370_d_n7;
        locals.var_ac1__blk811_dn10 = assign26910_e37370_d_n10;
        locals.var_ac1__blk811_dn11 = assign26910_e37370_d_n11;
        locals.var_ac1__blk811_dn12 = assign26910_e37370_d_n12;
        locals.var_ac1__blk811_dn17 = assign26910_e37370_d_n17;

        let (assign26920_e37386, assign26920_e37386_d_n0, assign26920_e37386_d_n2, assign26920_e37386_d_n6, assign26920_e37386_d_n7, assign26920_e37386_d_n10, assign26920_e37386_d_n11, assign26920_e37386_d_n12, assign26920_e37386_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) {
        let assign26920_e37383: f64 = (locals.var_ac4__blk806 + locals.var_ac3__blk809);
        let assign26920_e37384: f64 = (assign26920_e37383).sqrt();
        (assign26920_e37384, ((locals.var_ac4__blk806_dn0 + locals.var_ac3__blk809_dn0) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn2 + locals.var_ac3__blk809_dn2) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn6 + locals.var_ac3__blk809_dn6) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn7 + locals.var_ac3__blk809_dn7) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn10 + locals.var_ac3__blk809_dn10) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn11 + locals.var_ac3__blk809_dn11) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn12 + locals.var_ac3__blk809_dn12) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn17 + locals.var_ac3__blk809_dn17) / (2.0 * assign26920_e37384)),)
    } else {
        (locals.var_ac2__blk810, locals.var_ac2__blk810_dn0, locals.var_ac2__blk810_dn2, locals.var_ac2__blk810_dn6, locals.var_ac2__blk810_dn7, locals.var_ac2__blk810_dn10, locals.var_ac2__blk810_dn11, locals.var_ac2__blk810_dn12, locals.var_ac2__blk810_dn17,)
    }
};
        locals.var_ac2__blk810 = assign26920_e37386;
        locals.var_ac2__blk810_dn0 = assign26920_e37386_d_n0;
        locals.var_ac2__blk810_dn2 = assign26920_e37386_d_n2;
        locals.var_ac2__blk810_dn6 = assign26920_e37386_d_n6;
        locals.var_ac2__blk810_dn7 = assign26920_e37386_d_n7;
        locals.var_ac2__blk810_dn10 = assign26920_e37386_d_n10;
        locals.var_ac2__blk810_dn11 = assign26920_e37386_d_n11;
        locals.var_ac2__blk810_dn12 = assign26920_e37386_d_n12;
        locals.var_ac2__blk810_dn17 = assign26920_e37386_d_n17;

        let (assign26930_e37412, assign26930_e37412_d_n0, assign26930_e37412_d_n2, assign26930_e37412_d_n6, assign26930_e37412_d_n7, assign26930_e37412_d_n10, assign26930_e37412_d_n11, assign26930_e37412_d_n12, assign26930_e37412_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) {
        let assign26930_e37398: f64 = (-7.0);
        let assign26930_e37400: f64 = (assign26930_e37398 * 1.414213562373095);
        let assign26930_e37402: f64 = (assign26930_e37400 + locals.var_ac2__blk810);
        let assign26930_e37405: f64 = (9.0 * locals.var_ty__blk778);
        let assign26930_e37408: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26930_e37409: f64 = (assign26930_e37405 * assign26930_e37408);
        let assign26930_e37410: f64 = (assign26930_e37402 + assign26930_e37409);
        (assign26930_e37410, (locals.var_ac2__blk810_dn0 + (((9.0 * locals.var_ty__blk778_dn0) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn0))), (locals.var_ac2__blk810_dn2 + (((9.0 * locals.var_ty__blk778_dn2) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn2))), (locals.var_ac2__blk810_dn6 + (((9.0 * locals.var_ty__blk778_dn6) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn6))), (locals.var_ac2__blk810_dn7 + (((9.0 * locals.var_ty__blk778_dn7) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn7))), (locals.var_ac2__blk810_dn10 + (((9.0 * locals.var_ty__blk778_dn10) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn10))), (locals.var_ac2__blk810_dn11 + (((9.0 * locals.var_ty__blk778_dn11) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn11))), (locals.var_ac2__blk810_dn12 + (((9.0 * locals.var_ty__blk778_dn12) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn12))), (locals.var_ac2__blk810_dn17 + (((9.0 * locals.var_ty__blk778_dn17) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign26930_e37412;
        locals.var_ac1__blk811_dn0 = assign26930_e37412_d_n0;
        locals.var_ac1__blk811_dn2 = assign26930_e37412_d_n2;
        locals.var_ac1__blk811_dn6 = assign26930_e37412_d_n6;
        locals.var_ac1__blk811_dn7 = assign26930_e37412_d_n7;
        locals.var_ac1__blk811_dn10 = assign26930_e37412_d_n10;
        locals.var_ac1__blk811_dn11 = assign26930_e37412_d_n11;
        locals.var_ac1__blk811_dn12 = assign26930_e37412_d_n12;
        locals.var_ac1__blk811_dn17 = assign26930_e37412_d_n17;

        let (assign26940_e37424, assign26940_e37424_d_n0, assign26940_e37424_d_n2, assign26940_e37424_d_n6, assign26940_e37424_d_n7, assign26940_e37424_d_n10, assign26940_e37424_d_n11, assign26940_e37424_d_n12, assign26940_e37424_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26940_e37422: f64 = (locals.var_ac1__blk811).powf(0.3333333333333333);
        (assign26940_e37422, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn0)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn0 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn2)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn2 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn6)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn6 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn7)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn7 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn10)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn10 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn11)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn11 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn12)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn12 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn17)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn17 / locals.var_ac1__blk811))) },)
    } else {
        (locals.var_acd__blk812, locals.var_acd__blk812_dn0, locals.var_acd__blk812_dn2, locals.var_acd__blk812_dn6, locals.var_acd__blk812_dn7, locals.var_acd__blk812_dn10, locals.var_acd__blk812_dn11, locals.var_acd__blk812_dn12, locals.var_acd__blk812_dn17,)
    }
};
        locals.var_acd__blk812 = assign26940_e37424;
        locals.var_acd__blk812_dn0 = assign26940_e37424_d_n0;
        locals.var_acd__blk812_dn2 = assign26940_e37424_d_n2;
        locals.var_acd__blk812_dn6 = assign26940_e37424_d_n6;
        locals.var_acd__blk812_dn7 = assign26940_e37424_d_n7;
        locals.var_acd__blk812_dn10 = assign26940_e37424_d_n10;
        locals.var_acd__blk812_dn11 = assign26940_e37424_d_n11;
        locals.var_acd__blk812_dn12 = assign26940_e37424_d_n12;
        locals.var_acd__blk812_dn17 = assign26940_e37424_d_n17;

        let (assign26950_e37451, assign26950_e37451_d_n0, assign26950_e37451_d_n2, assign26950_e37451_d_n6, assign26950_e37451_d_n7, assign26950_e37451_d_n10, assign26950_e37451_d_n11, assign26950_e37451_d_n12, assign26950_e37451_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26950_e37433: f64 = (-4.0);
        let assign26950_e37435: f64 = (assign26950_e37433 * 1.414213562373095);
        let assign26950_e37438: f64 = (12.0 * locals.var_ty__blk778);
        let assign26950_e37439: f64 = (assign26950_e37435 - assign26950_e37438);
        let assign26950_e37442: f64 = (2.0 * locals.var_acd__blk812);
        let assign26950_e37443: f64 = (assign26950_e37439 + assign26950_e37442);
        let assign26950_e37446: f64 = (1.414213562373095 * locals.var_acd__blk812);
        let assign26950_e37448: f64 = (assign26950_e37446 * locals.var_acd__blk812);
        let assign26950_e37449: f64 = (assign26950_e37443 + assign26950_e37448);
        (assign26950_e37449, (((-(12.0 * locals.var_ty__blk778_dn0)) + (2.0 * locals.var_acd__blk812_dn0)) + (((1.414213562373095 * locals.var_acd__blk812_dn0) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn0))), (((-(12.0 * locals.var_ty__blk778_dn2)) + (2.0 * locals.var_acd__blk812_dn2)) + (((1.414213562373095 * locals.var_acd__blk812_dn2) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn2))), (((-(12.0 * locals.var_ty__blk778_dn6)) + (2.0 * locals.var_acd__blk812_dn6)) + (((1.414213562373095 * locals.var_acd__blk812_dn6) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn6))), (((-(12.0 * locals.var_ty__blk778_dn7)) + (2.0 * locals.var_acd__blk812_dn7)) + (((1.414213562373095 * locals.var_acd__blk812_dn7) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn7))), (((-(12.0 * locals.var_ty__blk778_dn10)) + (2.0 * locals.var_acd__blk812_dn10)) + (((1.414213562373095 * locals.var_acd__blk812_dn10) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn10))), (((-(12.0 * locals.var_ty__blk778_dn11)) + (2.0 * locals.var_acd__blk812_dn11)) + (((1.414213562373095 * locals.var_acd__blk812_dn11) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn11))), (((-(12.0 * locals.var_ty__blk778_dn12)) + (2.0 * locals.var_acd__blk812_dn12)) + (((1.414213562373095 * locals.var_acd__blk812_dn12) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn12))), (((-(12.0 * locals.var_ty__blk778_dn17)) + (2.0 * locals.var_acd__blk812_dn17)) + (((1.414213562373095 * locals.var_acd__blk812_dn17) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn17))),)
    } else {
        (locals.var_acn__blk813, locals.var_acn__blk813_dn0, locals.var_acn__blk813_dn2, locals.var_acn__blk813_dn6, locals.var_acn__blk813_dn7, locals.var_acn__blk813_dn10, locals.var_acn__blk813_dn11, locals.var_acn__blk813_dn12, locals.var_acn__blk813_dn17,)
    }
};
        locals.var_acn__blk813 = assign26950_e37451;
        locals.var_acn__blk813_dn0 = assign26950_e37451_d_n0;
        locals.var_acn__blk813_dn2 = assign26950_e37451_d_n2;
        locals.var_acn__blk813_dn6 = assign26950_e37451_d_n6;
        locals.var_acn__blk813_dn7 = assign26950_e37451_d_n7;
        locals.var_acn__blk813_dn10 = assign26950_e37451_d_n10;
        locals.var_acn__blk813_dn11 = assign26950_e37451_d_n11;
        locals.var_acn__blk813_dn12 = assign26950_e37451_d_n12;
        locals.var_acn__blk813_dn17 = assign26950_e37451_d_n17;

        let (assign26960_e37463, assign26960_e37463_d_n0, assign26960_e37463_d_n2, assign26960_e37463_d_n6, assign26960_e37463_d_n7, assign26960_e37463_d_n10, assign26960_e37463_d_n11, assign26960_e37463_d_n12, assign26960_e37463_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26960_e37461: f64 = (locals.var_acn__blk813 / locals.var_acd__blk812);
        (assign26960_e37461, (((locals.var_acn__blk813_dn0 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn0)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn2 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn2)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn6 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn6)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn7 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn7)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn10 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn10)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn11 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn11)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn12 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn12)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn17 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn17)) / (locals.var_acd__blk812 * locals.var_acd__blk812)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign26960_e37463;
        locals.var_chi__blk814_dn0 = assign26960_e37463_d_n0;
        locals.var_chi__blk814_dn2 = assign26960_e37463_d_n2;
        locals.var_chi__blk814_dn6 = assign26960_e37463_d_n6;
        locals.var_chi__blk814_dn7 = assign26960_e37463_d_n7;
        locals.var_chi__blk814_dn10 = assign26960_e37463_d_n10;
        locals.var_chi__blk814_dn11 = assign26960_e37463_d_n11;
        locals.var_chi__blk814_dn12 = assign26960_e37463_d_n12;
        locals.var_chi__blk814_dn17 = assign26960_e37463_d_n17;

        let (assign26970_e37477, assign26970_e37477_d_n0, assign26970_e37477_d_n2, assign26970_e37477_d_n6, assign26970_e37477_d_n7, assign26970_e37477_d_n10, assign26970_e37477_d_n11, assign26970_e37477_d_n12, assign26970_e37477_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26970_e37473: f64 = (locals.var_chi__blk814 * locals.var_beta_inv);
        let assign26970_e37475: f64 = (assign26970_e37473 - locals.var_vxbgmtcl);
        (assign26970_e37475, ((locals.var_chi__blk814_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk814_dn10 * locals.var_beta_inv) + (locals.var_chi__blk814 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk815, locals.var_psa__blk815_dn0, locals.var_psa__blk815_dn2, locals.var_psa__blk815_dn6, locals.var_psa__blk815_dn7, locals.var_psa__blk815_dn10, locals.var_psa__blk815_dn11, locals.var_psa__blk815_dn12, locals.var_psa__blk815_dn17,)
    }
};
        locals.var_psa__blk815 = assign26970_e37477;
        locals.var_psa__blk815_dn0 = assign26970_e37477_d_n0;
        locals.var_psa__blk815_dn2 = assign26970_e37477_d_n2;
        locals.var_psa__blk815_dn6 = assign26970_e37477_d_n6;
        locals.var_psa__blk815_dn7 = assign26970_e37477_d_n7;
        locals.var_psa__blk815_dn10 = assign26970_e37477_d_n10;
        locals.var_psa__blk815_dn11 = assign26970_e37477_d_n11;
        locals.var_psa__blk815_dn12 = assign26970_e37477_d_n12;
        locals.var_psa__blk815_dn17 = assign26970_e37477_d_n17;

        let (assign26980_e37489, assign26980_e37489_d_n0, assign26980_e37489_d_n2, assign26980_e37489_d_n6, assign26980_e37489_d_n7, assign26980_e37489_d_n10, assign26980_e37489_d_n11, assign26980_e37489_d_n12, assign26980_e37489_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26980_e37487: f64 = (locals.var_psa__blk815 + locals.var_vxbgmtcl);
        (assign26980_e37487, (locals.var_psa__blk815_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk815_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk815_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk815_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk815_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk815_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk815_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk815_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26980_e37489;
        locals.var_t1__blk771_dn0 = assign26980_e37489_d_n0;
        locals.var_t1__blk771_dn2 = assign26980_e37489_d_n2;
        locals.var_t1__blk771_dn6 = assign26980_e37489_d_n6;
        locals.var_t1__blk771_dn7 = assign26980_e37489_d_n7;
        locals.var_t1__blk771_dn10 = assign26980_e37489_d_n10;
        locals.var_t1__blk771_dn11 = assign26980_e37489_d_n11;
        locals.var_t1__blk771_dn12 = assign26980_e37489_d_n12;
        locals.var_t1__blk771_dn17 = assign26980_e37489_d_n17;

        let (assign26990_e37501, assign26990_e37501_d_n0, assign26990_e37501_d_n2, assign26990_e37501_d_n6, assign26990_e37501_d_n7, assign26990_e37501_d_n10, assign26990_e37501_d_n11, assign26990_e37501_d_n12, assign26990_e37501_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26990_e37499: f64 = (locals.var_t1__blk771 / locals.var_ps0_min__blk807);
        (assign26990_e37499, (((locals.var_t1__blk771_dn0 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn0)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn2 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn2)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn6 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn6)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn7 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn7)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn10 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn10)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn11 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn11)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn12 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn12)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn17 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn17)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign26990_e37501;
        locals.var_t2__blk772_dn0 = assign26990_e37501_d_n0;
        locals.var_t2__blk772_dn2 = assign26990_e37501_d_n2;
        locals.var_t2__blk772_dn6 = assign26990_e37501_d_n6;
        locals.var_t2__blk772_dn7 = assign26990_e37501_d_n7;
        locals.var_t2__blk772_dn10 = assign26990_e37501_d_n10;
        locals.var_t2__blk772_dn11 = assign26990_e37501_d_n11;
        locals.var_t2__blk772_dn12 = assign26990_e37501_d_n12;
        locals.var_t2__blk772_dn17 = assign26990_e37501_d_n17;

        let (assign27000_e37516, assign27000_e37516_d_n0, assign27000_e37516_d_n2, assign27000_e37516_d_n6, assign27000_e37516_d_n7, assign27000_e37516_d_n10, assign27000_e37516_d_n11, assign27000_e37516_d_n12, assign27000_e37516_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27000_e37512: f64 = (locals.var_t2__blk772 * locals.var_t2__blk772);
        let assign27000_e37513: f64 = (1.0 + assign27000_e37512);
        let assign27000_e37514: f64 = (assign27000_e37513).sqrt();
        (assign27000_e37514, (((locals.var_t2__blk772_dn0 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn0)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn2 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn2)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn6 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn6)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn7 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn7)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn10 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn10)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn11 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn11)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn12 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn12)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn17 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn17)) / (2.0 * assign27000_e37514)),)
    } else {
        (locals.var_t3__blk773, locals.var_t3__blk773_dn0, locals.var_t3__blk773_dn2, locals.var_t3__blk773_dn6, locals.var_t3__blk773_dn7, locals.var_t3__blk773_dn10, locals.var_t3__blk773_dn11, locals.var_t3__blk773_dn12, locals.var_t3__blk773_dn17,)
    }
};
        locals.var_t3__blk773 = assign27000_e37516;
        locals.var_t3__blk773_dn0 = assign27000_e37516_d_n0;
        locals.var_t3__blk773_dn2 = assign27000_e37516_d_n2;
        locals.var_t3__blk773_dn6 = assign27000_e37516_d_n6;
        locals.var_t3__blk773_dn7 = assign27000_e37516_d_n7;
        locals.var_t3__blk773_dn10 = assign27000_e37516_d_n10;
        locals.var_t3__blk773_dn11 = assign27000_e37516_d_n11;
        locals.var_t3__blk773_dn12 = assign27000_e37516_d_n12;
        locals.var_t3__blk773_dn17 = assign27000_e37516_d_n17;

        let (assign27010_e37530, assign27010_e37530_d_n0, assign27010_e37530_d_n2, assign27010_e37530_d_n6, assign27010_e37530_d_n7, assign27010_e37530_d_n10, assign27010_e37530_d_n11, assign27010_e37530_d_n12, assign27010_e37530_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27010_e37526: f64 = (locals.var_t1__blk771 / locals.var_t3__blk773);
        let assign27010_e37528: f64 = (assign27010_e37526 - locals.var_vxbgmtcl);
        (assign27010_e37528, ((((locals.var_t1__blk771_dn0 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn0)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk771_dn2 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn2)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk771_dn6 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn6)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk771_dn7 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn7)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk771_dn10 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn10)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk771_dn11 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn11)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk771_dn12 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn12)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk771_dn17 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn17)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27010_e37530;
        locals.var_ps0ld_dn0 = assign27010_e37530_d_n0;
        locals.var_ps0ld_dn2 = assign27010_e37530_d_n2;
        locals.var_ps0ld_dn6 = assign27010_e37530_d_n6;
        locals.var_ps0ld_dn7 = assign27010_e37530_d_n7;
        locals.var_ps0ld_dn10 = assign27010_e37530_d_n10;
        locals.var_ps0ld_dn11 = assign27010_e37530_d_n11;
        locals.var_ps0ld_dn12 = assign27010_e37530_d_n12;
        locals.var_ps0ld_dn17 = assign27010_e37530_d_n17;

        let (assign27020_e37542, assign27020_e37542_d_n0, assign27020_e37542_d_n2, assign27020_e37542_d_n6, assign27020_e37542_d_n7, assign27020_e37542_d_n10, assign27020_e37542_d_n11, assign27020_e37542_d_n12, assign27020_e37542_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27020_e37540: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign27020_e37540, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27020_e37542;
        locals.var_t2__blk772_dn0 = assign27020_e37542_d_n0;
        locals.var_t2__blk772_dn2 = assign27020_e37542_d_n2;
        locals.var_t2__blk772_dn6 = assign27020_e37542_d_n6;
        locals.var_t2__blk772_dn7 = assign27020_e37542_d_n7;
        locals.var_t2__blk772_dn10 = assign27020_e37542_d_n10;
        locals.var_t2__blk772_dn11 = assign27020_e37542_d_n11;
        locals.var_t2__blk772_dn12 = assign27020_e37542_d_n12;
        locals.var_t2__blk772_dn17 = assign27020_e37542_d_n17;

        let (assign27030_e37554, assign27030_e37554_d_n0, assign27030_e37554_d_n2, assign27030_e37554_d_n6, assign27030_e37554_d_n7, assign27030_e37554_d_n10, assign27030_e37554_d_n11, assign27030_e37554_d_n12, assign27030_e37554_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27030_e37552: f64 = (locals.var_cox0 * locals.var_t2__blk772);
        (assign27030_e37552, (locals.var_cox0 * locals.var_t2__blk772_dn0), (locals.var_cox0 * locals.var_t2__blk772_dn2), (locals.var_cox0 * locals.var_t2__blk772_dn6), (locals.var_cox0 * locals.var_t2__blk772_dn7), (locals.var_cox0 * locals.var_t2__blk772_dn10), (locals.var_cox0 * locals.var_t2__blk772_dn11), (locals.var_cox0 * locals.var_t2__blk772_dn12), (locals.var_cox0 * locals.var_t2__blk772_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27030_e37554;
        locals.var_qsuld_dn0 = assign27030_e37554_d_n0;
        locals.var_qsuld_dn2 = assign27030_e37554_d_n2;
        locals.var_qsuld_dn6 = assign27030_e37554_d_n6;
        locals.var_qsuld_dn7 = assign27030_e37554_d_n7;
        locals.var_qsuld_dn10 = assign27030_e37554_d_n10;
        locals.var_qsuld_dn11 = assign27030_e37554_d_n11;
        locals.var_qsuld_dn12 = assign27030_e37554_d_n12;
        locals.var_qsuld_dn17 = assign27030_e37554_d_n17;

        let (assign27040_e37564, assign27040_e37564_d_n0, assign27040_e37564_d_n2, assign27040_e37564_d_n6, assign27040_e37564_d_n7, assign27040_e37564_d_n10, assign27040_e37564_d_n11, assign27040_e37564_d_n12, assign27040_e37564_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27040_e37564;
        locals.var_qbuld_dn0 = assign27040_e37564_d_n0;
        locals.var_qbuld_dn2 = assign27040_e37564_d_n2;
        locals.var_qbuld_dn6 = assign27040_e37564_d_n6;
        locals.var_qbuld_dn7 = assign27040_e37564_d_n7;
        locals.var_qbuld_dn10 = assign27040_e37564_d_n10;
        locals.var_qbuld_dn11 = assign27040_e37564_d_n11;
        locals.var_qbuld_dn12 = assign27040_e37564_d_n12;
        locals.var_qbuld_dn17 = assign27040_e37564_d_n17;

        let (assign27060_e37586, assign27060_e37586_d_n0, assign27060_e37586_d_n2, assign27060_e37586_d_n6, assign27060_e37586_d_n7, assign27060_e37586_d_n10, assign27060_e37586_d_n11, assign27060_e37586_d_n12, assign27060_e37586_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27060_e37586;
        locals.var_chi__blk814_dn0 = assign27060_e37586_d_n0;
        locals.var_chi__blk814_dn2 = assign27060_e37586_d_n2;
        locals.var_chi__blk814_dn6 = assign27060_e37586_d_n6;
        locals.var_chi__blk814_dn7 = assign27060_e37586_d_n7;
        locals.var_chi__blk814_dn10 = assign27060_e37586_d_n10;
        locals.var_chi__blk814_dn11 = assign27060_e37586_d_n11;
        locals.var_chi__blk814_dn12 = assign27060_e37586_d_n12;
        locals.var_chi__blk814_dn17 = assign27060_e37586_d_n17;

        let (assign27070_e37601, assign27070_e37601_d_n0, assign27070_e37601_d_n2, assign27070_e37601_d_n6, assign27070_e37601_d_n7, assign27070_e37601_d_n10, assign27070_e37601_d_n11, assign27070_e37601_d_n12, assign27070_e37601_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27070_e37597: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign27070_e37599: f64 = (assign27070_e37597 - locals.var_vxbgmtcl);
        (assign27070_e37599, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27070_e37601;
        locals.var_ps0_inia__blk817_dn0 = assign27070_e37601_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27070_e37601_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27070_e37601_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27070_e37601_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27070_e37601_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27070_e37601_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27070_e37601_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27070_e37601_d_n17;

        let (assign27080_e37614, assign27080_e37614_d_n0, assign27080_e37614_d_n2, assign27080_e37614_d_n6, assign27080_e37614_d_n7, assign27080_e37614_d_n10, assign27080_e37614_d_n11, assign27080_e37614_d_n12, assign27080_e37614_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27080_e37611: f64 = (-locals.var_chi__blk814);
        let assign27080_e37612: f64 = (assign27080_e37611).exp();
        (assign27080_e37612, (assign27080_e37612 * (-locals.var_chi__blk814_dn0)), (assign27080_e37612 * (-locals.var_chi__blk814_dn2)), (assign27080_e37612 * (-locals.var_chi__blk814_dn6)), (assign27080_e37612 * (-locals.var_chi__blk814_dn7)), (assign27080_e37612 * (-locals.var_chi__blk814_dn10)), (assign27080_e37612 * (-locals.var_chi__blk814_dn11)), (assign27080_e37612 * (-locals.var_chi__blk814_dn12)), (assign27080_e37612 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign27080_e37614;
        locals.var_ty__blk778_dn0 = assign27080_e37614_d_n0;
        locals.var_ty__blk778_dn2 = assign27080_e37614_d_n2;
        locals.var_ty__blk778_dn6 = assign27080_e37614_d_n6;
        locals.var_ty__blk778_dn7 = assign27080_e37614_d_n7;
        locals.var_ty__blk778_dn10 = assign27080_e37614_d_n10;
        locals.var_ty__blk778_dn11 = assign27080_e37614_d_n11;
        locals.var_ty__blk778_dn12 = assign27080_e37614_d_n12;
        locals.var_ty__blk778_dn17 = assign27080_e37614_d_n17;

        let (assign27090_e37641, assign27090_e37641_d_n0, assign27090_e37641_d_n2, assign27090_e37641_d_n6, assign27090_e37641_d_n7, assign27090_e37641_d_n10, assign27090_e37641_d_n11, assign27090_e37641_d_n12, assign27090_e37641_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27090_e37628: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27090_e37629: f64 = (locals.var_beta * assign27090_e37628);
        let assign27090_e37631: f64 = (assign27090_e37629 - 1.0);
        let assign27090_e37633: f64 = (assign27090_e37631 + locals.var_ty__blk778);
        let assign27090_e37634: f64 = (4.0 * assign27090_e37633);
        let assign27090_e37637: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign27090_e37638: f64 = (assign27090_e37634 / assign27090_e37637);
        let assign27090_e37639: f64 = (1.0 + assign27090_e37638);
        (assign27090_e37639, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * (((locals.var_beta_dn10 * assign27090_e37628) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign27090_e37637) - (assign27090_e37634 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27090_e37641;
        locals.var_tx__blk777_dn0 = assign27090_e37641_d_n0;
        locals.var_tx__blk777_dn2 = assign27090_e37641_d_n2;
        locals.var_tx__blk777_dn6 = assign27090_e37641_d_n6;
        locals.var_tx__blk777_dn7 = assign27090_e37641_d_n7;
        locals.var_tx__blk777_dn10 = assign27090_e37641_d_n10;
        locals.var_tx__blk777_dn11 = assign27090_e37641_d_n11;
        locals.var_tx__blk777_dn12 = assign27090_e37641_d_n12;
        locals.var_tx__blk777_dn17 = assign27090_e37641_d_n17;

        let assign27100_e37645: f64 = (10.0 * 2.220446049250313e-16);
        let assign27100_e37646: f64 = if locals.var_tx__blk777 < assign27100_e37645 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign27100_e37646;

        let (assign27110_e37661, assign27110_e37661_d_n0, assign27110_e37661_d_n2, assign27110_e37661_d_n6, assign27110_e37661_d_n7, assign27110_e37661_d_n10, assign27110_e37661_d_n11, assign27110_e37661_d_n12, assign27110_e37661_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27110_e37659: f64 = (10.0 * 2.220446049250313e-16);
        (assign27110_e37659, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27110_e37661;
        locals.var_tx__blk777_dn0 = assign27110_e37661_d_n0;
        locals.var_tx__blk777_dn2 = assign27110_e37661_d_n2;
        locals.var_tx__blk777_dn6 = assign27110_e37661_d_n6;
        locals.var_tx__blk777_dn7 = assign27110_e37661_d_n7;
        locals.var_tx__blk777_dn10 = assign27110_e37661_d_n10;
        locals.var_tx__blk777_dn11 = assign27110_e37661_d_n11;
        locals.var_tx__blk777_dn12 = assign27110_e37661_d_n12;
        locals.var_tx__blk777_dn17 = assign27110_e37661_d_n17;

        let (assign27120_e37683, assign27120_e37683_d_n0, assign27120_e37683_d_n2, assign27120_e37683_d_n6, assign27120_e37683_d_n7, assign27120_e37683_d_n10, assign27120_e37683_d_n11, assign27120_e37683_d_n12, assign27120_e37683_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27120_e37673: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign27120_e37675: f64 = (assign27120_e37673 / 2.0);
        let assign27120_e37678: f64 = (locals.var_tx__blk777).sqrt();
        let assign27120_e37679: f64 = (1.0 - assign27120_e37678);
        let assign27120_e37680: f64 = (assign27120_e37675 * assign27120_e37679);
        let assign27120_e37681: f64 = (locals.var_vgpld + assign27120_e37680);
        (assign27120_e37681, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign27120_e37678)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27120_e37683;
        locals.var_ps0_inia__blk817_dn0 = assign27120_e37683_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27120_e37683_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27120_e37683_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27120_e37683_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27120_e37683_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27120_e37683_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27120_e37683_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27120_e37683_d_n17;

        let (assign27130_e37698, assign27130_e37698_d_n0, assign27130_e37698_d_n2, assign27130_e37698_d_n6, assign27130_e37698_d_n7, assign27130_e37698_d_n10, assign27130_e37698_d_n11, assign27130_e37698_d_n12, assign27130_e37698_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27130_e37695: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27130_e37696: f64 = (locals.var_beta * assign27130_e37695);
        (assign27130_e37696, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27130_e37695) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27130_e37698;
        locals.var_chi__blk814_dn0 = assign27130_e37698_d_n0;
        locals.var_chi__blk814_dn2 = assign27130_e37698_d_n2;
        locals.var_chi__blk814_dn6 = assign27130_e37698_d_n6;
        locals.var_chi__blk814_dn7 = assign27130_e37698_d_n7;
        locals.var_chi__blk814_dn10 = assign27130_e37698_d_n10;
        locals.var_chi__blk814_dn11 = assign27130_e37698_d_n11;
        locals.var_chi__blk814_dn12 = assign27130_e37698_d_n12;
        locals.var_chi__blk814_dn17 = assign27130_e37698_d_n17;

        let (assign27140_e37711, assign27140_e37711_d_n0, assign27140_e37711_d_n2, assign27140_e37711_d_n6, assign27140_e37711_d_n7, assign27140_e37711_d_n10, assign27140_e37711_d_n11, assign27140_e37711_d_n12, assign27140_e37711_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27140_e37708: f64 = (-locals.var_chi__blk814);
        let assign27140_e37709: f64 = (assign27140_e37708).exp();
        (assign27140_e37709, (assign27140_e37709 * (-locals.var_chi__blk814_dn0)), (assign27140_e37709 * (-locals.var_chi__blk814_dn2)), (assign27140_e37709 * (-locals.var_chi__blk814_dn6)), (assign27140_e37709 * (-locals.var_chi__blk814_dn7)), (assign27140_e37709 * (-locals.var_chi__blk814_dn10)), (assign27140_e37709 * (-locals.var_chi__blk814_dn11)), (assign27140_e37709 * (-locals.var_chi__blk814_dn12)), (assign27140_e37709 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign27140_e37711;
        locals.var_ty__blk778_dn0 = assign27140_e37711_d_n0;
        locals.var_ty__blk778_dn2 = assign27140_e37711_d_n2;
        locals.var_ty__blk778_dn6 = assign27140_e37711_d_n6;
        locals.var_ty__blk778_dn7 = assign27140_e37711_d_n7;
        locals.var_ty__blk778_dn10 = assign27140_e37711_d_n10;
        locals.var_ty__blk778_dn11 = assign27140_e37711_d_n11;
        locals.var_ty__blk778_dn12 = assign27140_e37711_d_n12;
        locals.var_ty__blk778_dn17 = assign27140_e37711_d_n17;

        let (assign27150_e37738, assign27150_e37738_d_n0, assign27150_e37738_d_n2, assign27150_e37738_d_n6, assign27150_e37738_d_n7, assign27150_e37738_d_n10, assign27150_e37738_d_n11, assign27150_e37738_d_n12, assign27150_e37738_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27150_e37725: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27150_e37726: f64 = (locals.var_beta * assign27150_e37725);
        let assign27150_e37728: f64 = (assign27150_e37726 - 1.0);
        let assign27150_e37730: f64 = (assign27150_e37728 + locals.var_ty__blk778);
        let assign27150_e37731: f64 = (4.0 * assign27150_e37730);
        let assign27150_e37734: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign27150_e37735: f64 = (assign27150_e37731 / assign27150_e37734);
        let assign27150_e37736: f64 = (1.0 + assign27150_e37735);
        (assign27150_e37736, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * (((locals.var_beta_dn10 * assign27150_e37725) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign27150_e37734) - (assign27150_e37731 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27150_e37738;
        locals.var_tx__blk777_dn0 = assign27150_e37738_d_n0;
        locals.var_tx__blk777_dn2 = assign27150_e37738_d_n2;
        locals.var_tx__blk777_dn6 = assign27150_e37738_d_n6;
        locals.var_tx__blk777_dn7 = assign27150_e37738_d_n7;
        locals.var_tx__blk777_dn10 = assign27150_e37738_d_n10;
        locals.var_tx__blk777_dn11 = assign27150_e37738_d_n11;
        locals.var_tx__blk777_dn12 = assign27150_e37738_d_n12;
        locals.var_tx__blk777_dn17 = assign27150_e37738_d_n17;

        let assign27160_e37742: f64 = (10.0 * 2.220446049250313e-16);
        let assign27160_e37743: f64 = if locals.var_tx__blk777 < assign27160_e37742 { 1.0 } else { 0.0 };
        locals.var_guard874 = assign27160_e37743;

        let (assign27170_e37758, assign27170_e37758_d_n0, assign27170_e37758_d_n2, assign27170_e37758_d_n6, assign27170_e37758_d_n7, assign27170_e37758_d_n10, assign27170_e37758_d_n11, assign27170_e37758_d_n12, assign27170_e37758_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign27170_e37756: f64 = (10.0 * 2.220446049250313e-16);
        (assign27170_e37756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27170_e37758;
        locals.var_tx__blk777_dn0 = assign27170_e37758_d_n0;
        locals.var_tx__blk777_dn2 = assign27170_e37758_d_n2;
        locals.var_tx__blk777_dn6 = assign27170_e37758_d_n6;
        locals.var_tx__blk777_dn7 = assign27170_e37758_d_n7;
        locals.var_tx__blk777_dn10 = assign27170_e37758_d_n10;
        locals.var_tx__blk777_dn11 = assign27170_e37758_d_n11;
        locals.var_tx__blk777_dn12 = assign27170_e37758_d_n12;
        locals.var_tx__blk777_dn17 = assign27170_e37758_d_n17;

    }

    pub(super) fn stamp_transient_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27180_e37780, assign27180_e37780_d_n0, assign27180_e37780_d_n2, assign27180_e37780_d_n6, assign27180_e37780_d_n7, assign27180_e37780_d_n10, assign27180_e37780_d_n11, assign27180_e37780_d_n12, assign27180_e37780_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27180_e37770: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign27180_e37772: f64 = (assign27180_e37770 / 2.0);
        let assign27180_e37775: f64 = (locals.var_tx__blk777).sqrt();
        let assign27180_e37776: f64 = (1.0 - assign27180_e37775);
        let assign27180_e37777: f64 = (assign27180_e37772 * assign27180_e37776);
        let assign27180_e37778: f64 = (locals.var_vgpld + assign27180_e37777);
        (assign27180_e37778, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign27180_e37775)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27180_e37780;
        locals.var_ps0_inia__blk817_dn0 = assign27180_e37780_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27180_e37780_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27180_e37780_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27180_e37780_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27180_e37780_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27180_e37780_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27180_e37780_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27180_e37780_d_n17;

        let (assign27190_e37795, assign27190_e37795_d_n0, assign27190_e37795_d_n2, assign27190_e37795_d_n6, assign27190_e37795_d_n7, assign27190_e37795_d_n10, assign27190_e37795_d_n11, assign27190_e37795_d_n12, assign27190_e37795_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27190_e37792: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27190_e37793: f64 = (locals.var_beta * assign27190_e37792);
        (assign27190_e37793, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27190_e37792) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27190_e37795;
        locals.var_chi__blk814_dn0 = assign27190_e37795_d_n0;
        locals.var_chi__blk814_dn2 = assign27190_e37795_d_n2;
        locals.var_chi__blk814_dn6 = assign27190_e37795_d_n6;
        locals.var_chi__blk814_dn7 = assign27190_e37795_d_n7;
        locals.var_chi__blk814_dn10 = assign27190_e37795_d_n10;
        locals.var_chi__blk814_dn11 = assign27190_e37795_d_n11;
        locals.var_chi__blk814_dn12 = assign27190_e37795_d_n12;
        locals.var_chi__blk814_dn17 = assign27190_e37795_d_n17;

        let assign27200_e37798: f64 = if locals.var_chi__blk814 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard875 = assign27200_e37798;

        let (assign27220_e37841,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27220_e37825: f64 = (9.0 * 1.414213562373095);
        let assign27220_e37826: f64 = (1.0 / assign27220_e37825);
        let assign27220_e37830: f64 = (7.0 * 0.049787068367863944);
        let assign27220_e37831: f64 = (5.0 + assign27220_e37830);
        let assign27220_e37835: f64 = (2.0 + 0.049787068367863944);
        let assign27220_e37836: f64 = (assign27220_e37835).sqrt();
        let assign27220_e37837: f64 = (54.0 * assign27220_e37836);
        let assign27220_e37838: f64 = (assign27220_e37831 / assign27220_e37837);
        let assign27220_e37839: f64 = (assign27220_e37826 - assign27220_e37838);
        (assign27220_e37839,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign27220_e37841;

        let (assign27230_e37867,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27230_e37854: f64 = (1.0 + 0.049787068367863944);
        let assign27230_e37858: f64 = (2.0 + 0.049787068367863944);
        let assign27230_e37859: f64 = (assign27230_e37858).sqrt();
        let assign27230_e37860: f64 = (2.0 * assign27230_e37859);
        let assign27230_e37861: f64 = (assign27230_e37854 / assign27230_e37860);
        let assign27230_e37864: f64 = (1.414213562373095 / 3.0);
        let assign27230_e37865: f64 = (assign27230_e37861 - assign27230_e37864);
        (assign27230_e37865,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign27230_e37867;

        let (assign27240_e37888, assign27240_e37888_d_n0, assign27240_e37888_d_n2, assign27240_e37888_d_n6, assign27240_e37888_d_n7, assign27240_e37888_d_n10, assign27240_e37888_d_n11, assign27240_e37888_d_n12, assign27240_e37888_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27240_e37880: f64 = (1.0 / 1.414213562373095);
        let assign27240_e37884: f64 = (locals.var_beta * locals.var_fac1__blk800);
        let assign27240_e37885: f64 = (1.0 / assign27240_e37884);
        let assign27240_e37886: f64 = (assign27240_e37880 + assign27240_e37885);
        (assign27240_e37886, (-((locals.var_beta * locals.var_fac1__blk800_dn0) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn2) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn6) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn7) / (assign27240_e37884 * assign27240_e37884))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk800) + (locals.var_beta * locals.var_fac1__blk800_dn10)) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn11) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn12) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn17) / (assign27240_e37884 * assign27240_e37884))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign27240_e37888;
        locals.var_tc_dn0 = assign27240_e37888_d_n0;
        locals.var_tc_dn2 = assign27240_e37888_d_n2;
        locals.var_tc_dn6 = assign27240_e37888_d_n6;
        locals.var_tc_dn7 = assign27240_e37888_d_n7;
        locals.var_tc_dn10 = assign27240_e37888_d_n10;
        locals.var_tc_dn11 = assign27240_e37888_d_n11;
        locals.var_tc_dn12 = assign27240_e37888_d_n12;
        locals.var_tc_dn17 = assign27240_e37888_d_n17;

        let (assign27250_e37906, assign27250_e37906_d_n0, assign27250_e37906_d_n2, assign27250_e37906_d_n6, assign27250_e37906_d_n7, assign27250_e37906_d_n10, assign27250_e37906_d_n11, assign27250_e37906_d_n12, assign27250_e37906_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27250_e37901: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27250_e37902: f64 = (-assign27250_e37901);
        let assign27250_e37904: f64 = (assign27250_e37902 / locals.var_fac1__blk800);
        (assign27250_e37904, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn0)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn2)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn6)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn7)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn10)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn11)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn12)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn17)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign27250_e37906;
        locals.var_td_dn0 = assign27250_e37906_d_n0;
        locals.var_td_dn2 = assign27250_e37906_d_n2;
        locals.var_td_dn6 = assign27250_e37906_d_n6;
        locals.var_td_dn7 = assign27250_e37906_d_n7;
        locals.var_td_dn10 = assign27250_e37906_d_n10;
        locals.var_td_dn11 = assign27250_e37906_d_n11;
        locals.var_td_dn12 = assign27250_e37906_d_n12;
        locals.var_td_dn17 = assign27250_e37906_d_n17;

        let (assign27260_e37947, assign27260_e37947_d_n0, assign27260_e37947_d_n2, assign27260_e37947_d_n6, assign27260_e37947_d_n7, assign27260_e37947_d_n10, assign27260_e37947_d_n11, assign27260_e37947_d_n12, assign27260_e37947_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27260_e37919: f64 = (locals.var_tb * locals.var_tb);
        let assign27260_e37921: f64 = (assign27260_e37919 * locals.var_tb);
        let assign27260_e37924: f64 = (27.0 * locals.var_ta);
        let assign27260_e37926: f64 = (assign27260_e37924 * locals.var_ta);
        let assign27260_e37928: f64 = (assign27260_e37926 * locals.var_ta);
        let assign27260_e37929: f64 = (assign27260_e37921 / assign27260_e37928);
        let assign27260_e37932: f64 = (locals.var_tb * locals.var_tc);
        let assign27260_e37935: f64 = (6.0 * locals.var_ta);
        let assign27260_e37937: f64 = (assign27260_e37935 * locals.var_ta);
        let assign27260_e37938: f64 = (assign27260_e37932 / assign27260_e37937);
        let assign27260_e37939: f64 = (assign27260_e37929 - assign27260_e37938);
        let assign27260_e37943: f64 = (2.0 * locals.var_ta);
        let assign27260_e37944: f64 = (locals.var_td / assign27260_e37943);
        let assign27260_e37945: f64 = (assign27260_e37939 + assign27260_e37944);
        (assign27260_e37945, ((-((locals.var_tb * locals.var_tc_dn0) / assign27260_e37937)) + (locals.var_td_dn0 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn2) / assign27260_e37937)) + (locals.var_td_dn2 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn6) / assign27260_e37937)) + (locals.var_td_dn6 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn7) / assign27260_e37937)) + (locals.var_td_dn7 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn10) / assign27260_e37937)) + (locals.var_td_dn10 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn11) / assign27260_e37937)) + (locals.var_td_dn11 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn12) / assign27260_e37937)) + (locals.var_td_dn12 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn17) / assign27260_e37937)) + (locals.var_td_dn17 / assign27260_e37943)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign27260_e37947;
        locals.var_tq_dn0 = assign27260_e37947_d_n0;
        locals.var_tq_dn2 = assign27260_e37947_d_n2;
        locals.var_tq_dn6 = assign27260_e37947_d_n6;
        locals.var_tq_dn7 = assign27260_e37947_d_n7;
        locals.var_tq_dn10 = assign27260_e37947_d_n10;
        locals.var_tq_dn11 = assign27260_e37947_d_n11;
        locals.var_tq_dn12 = assign27260_e37947_d_n12;
        locals.var_tq_dn17 = assign27260_e37947_d_n17;

        let (assign27270_e37974, assign27270_e37974_d_n0, assign27270_e37974_d_n2, assign27270_e37974_d_n6, assign27270_e37974_d_n7, assign27270_e37974_d_n10, assign27270_e37974_d_n11, assign27270_e37974_d_n12, assign27270_e37974_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27270_e37960: f64 = (3.0 * locals.var_ta);
        let assign27270_e37962: f64 = (assign27270_e37960 * locals.var_tc);
        let assign27270_e37965: f64 = (locals.var_tb * locals.var_tb);
        let assign27270_e37966: f64 = (assign27270_e37962 - assign27270_e37965);
        let assign27270_e37969: f64 = (9.0 * locals.var_ta);
        let assign27270_e37971: f64 = (assign27270_e37969 * locals.var_ta);
        let assign27270_e37972: f64 = (assign27270_e37966 / assign27270_e37971);
        (assign27270_e37972, ((assign27270_e37960 * locals.var_tc_dn0) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn2) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn6) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn7) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn10) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn11) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn12) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn17) / assign27270_e37971),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign27270_e37974;
        locals.var_tp_dn0 = assign27270_e37974_d_n0;
        locals.var_tp_dn2 = assign27270_e37974_d_n2;
        locals.var_tp_dn6 = assign27270_e37974_d_n6;
        locals.var_tp_dn7 = assign27270_e37974_d_n7;
        locals.var_tp_dn10 = assign27270_e37974_d_n10;
        locals.var_tp_dn11 = assign27270_e37974_d_n11;
        locals.var_tp_dn12 = assign27270_e37974_d_n12;
        locals.var_tp_dn17 = assign27270_e37974_d_n17;

        let (assign27280_e37996, assign27280_e37996_d_n0, assign27280_e37996_d_n2, assign27280_e37996_d_n6, assign27280_e37996_d_n7, assign27280_e37996_d_n10, assign27280_e37996_d_n11, assign27280_e37996_d_n12, assign27280_e37996_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27280_e37987: f64 = (locals.var_tq * locals.var_tq);
        let assign27280_e37990: f64 = (locals.var_tp * locals.var_tp);
        let assign27280_e37992: f64 = (assign27280_e37990 * locals.var_tp);
        let assign27280_e37993: f64 = (assign27280_e37987 + assign27280_e37992);
        let assign27280_e37994: f64 = (assign27280_e37993).sqrt();
        (assign27280_e37994, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn0))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn2))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn6))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn7))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn10))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn11))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn12))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn17))) / (2.0 * assign27280_e37994)),)
    } else {
        (locals.var_t5__blk774, locals.var_t5__blk774_dn0, locals.var_t5__blk774_dn2, locals.var_t5__blk774_dn6, locals.var_t5__blk774_dn7, locals.var_t5__blk774_dn10, locals.var_t5__blk774_dn11, locals.var_t5__blk774_dn12, locals.var_t5__blk774_dn17,)
    }
};
        locals.var_t5__blk774 = assign27280_e37996;
        locals.var_t5__blk774_dn0 = assign27280_e37996_d_n0;
        locals.var_t5__blk774_dn2 = assign27280_e37996_d_n2;
        locals.var_t5__blk774_dn6 = assign27280_e37996_d_n6;
        locals.var_t5__blk774_dn7 = assign27280_e37996_d_n7;
        locals.var_t5__blk774_dn10 = assign27280_e37996_d_n10;
        locals.var_t5__blk774_dn11 = assign27280_e37996_d_n11;
        locals.var_t5__blk774_dn12 = assign27280_e37996_d_n12;
        locals.var_t5__blk774_dn17 = assign27280_e37996_d_n17;

        let (assign27290_e38014, assign27290_e38014_d_n0, assign27290_e38014_d_n2, assign27290_e38014_d_n6, assign27290_e38014_d_n7, assign27290_e38014_d_n10, assign27290_e38014_d_n11, assign27290_e38014_d_n12, assign27290_e38014_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27290_e38008: f64 = (-locals.var_tq);
        let assign27290_e38010: f64 = (assign27290_e38008 + locals.var_t5__blk774);
        let assign27290_e38012: f64 = (assign27290_e38010).powf(0.3333333333333333);
        (assign27290_e38012, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17) / assign27290_e38010))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign27290_e38014;
        locals.var_tu_dn0 = assign27290_e38014_d_n0;
        locals.var_tu_dn2 = assign27290_e38014_d_n2;
        locals.var_tu_dn6 = assign27290_e38014_d_n6;
        locals.var_tu_dn7 = assign27290_e38014_d_n7;
        locals.var_tu_dn10 = assign27290_e38014_d_n10;
        locals.var_tu_dn11 = assign27290_e38014_d_n11;
        locals.var_tu_dn12 = assign27290_e38014_d_n12;
        locals.var_tu_dn17 = assign27290_e38014_d_n17;

        let (assign27300_e38032, assign27300_e38032_d_n0, assign27300_e38032_d_n2, assign27300_e38032_d_n6, assign27300_e38032_d_n7, assign27300_e38032_d_n10, assign27300_e38032_d_n11, assign27300_e38032_d_n12, assign27300_e38032_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27300_e38027: f64 = (locals.var_tq + locals.var_t5__blk774);
        let assign27300_e38029: f64 = (assign27300_e38027).powf(0.3333333333333333);
        let assign27300_e38030: f64 = (-assign27300_e38029);
        (assign27300_e38030, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk774_dn0))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk774_dn0) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk774_dn2))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk774_dn2) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk774_dn6))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk774_dn6) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk774_dn7))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk774_dn7) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk774_dn10))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk774_dn10) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk774_dn11))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk774_dn11) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk774_dn12))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk774_dn12) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk774_dn17))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk774_dn17) / assign27300_e38027))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign27300_e38032;
        locals.var_tv_dn0 = assign27300_e38032_d_n0;
        locals.var_tv_dn2 = assign27300_e38032_d_n2;
        locals.var_tv_dn6 = assign27300_e38032_d_n6;
        locals.var_tv_dn7 = assign27300_e38032_d_n7;
        locals.var_tv_dn10 = assign27300_e38032_d_n10;
        locals.var_tv_dn11 = assign27300_e38032_d_n11;
        locals.var_tv_dn12 = assign27300_e38032_d_n12;
        locals.var_tv_dn17 = assign27300_e38032_d_n17;

        let (assign27310_e38053, assign27310_e38053_d_n0, assign27310_e38053_d_n2, assign27310_e38053_d_n6, assign27310_e38053_d_n7, assign27310_e38053_d_n10, assign27310_e38053_d_n11, assign27310_e38053_d_n12, assign27310_e38053_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27310_e38045: f64 = (locals.var_tu + locals.var_tv);
        let assign27310_e38049: f64 = (3.0 * locals.var_ta);
        let assign27310_e38050: f64 = (locals.var_tb / assign27310_e38049);
        let assign27310_e38051: f64 = (assign27310_e38045 - assign27310_e38050);
        (assign27310_e38051, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27310_e38053;
        locals.var_tx__blk777_dn0 = assign27310_e38053_d_n0;
        locals.var_tx__blk777_dn2 = assign27310_e38053_d_n2;
        locals.var_tx__blk777_dn6 = assign27310_e38053_d_n6;
        locals.var_tx__blk777_dn7 = assign27310_e38053_d_n7;
        locals.var_tx__blk777_dn10 = assign27310_e38053_d_n10;
        locals.var_tx__blk777_dn11 = assign27310_e38053_d_n11;
        locals.var_tx__blk777_dn12 = assign27310_e38053_d_n12;
        locals.var_tx__blk777_dn17 = assign27310_e38053_d_n17;

        let (assign27320_e38070, assign27320_e38070_d_n0, assign27320_e38070_d_n2, assign27320_e38070_d_n6, assign27320_e38070_d_n7, assign27320_e38070_d_n10, assign27320_e38070_d_n11, assign27320_e38070_d_n12, assign27320_e38070_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27320_e38066: f64 = (locals.var_tx__blk777 * locals.var_beta_inv);
        let assign27320_e38068: f64 = (assign27320_e38066 - locals.var_vxbgmtcl);
        (assign27320_e38068, ((locals.var_tx__blk777_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk777_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk777_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk777_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk777_dn10 * locals.var_beta_inv) + (locals.var_tx__blk777 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk777_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk777_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk777_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27320_e38070;
        locals.var_ps0_inia__blk817_dn0 = assign27320_e38070_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27320_e38070_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27320_e38070_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27320_e38070_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27320_e38070_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27320_e38070_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27320_e38070_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27320_e38070_d_n17;

        let (assign27330_e38087, assign27330_e38087_d_n0, assign27330_e38087_d_n2, assign27330_e38087_d_n6, assign27330_e38087_d_n7, assign27330_e38087_d_n10, assign27330_e38087_d_n11, assign27330_e38087_d_n12, assign27330_e38087_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27330_e38084: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27330_e38085: f64 = (locals.var_beta * assign27330_e38084);
        (assign27330_e38085, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27330_e38084) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27330_e38087;
        locals.var_chi__blk814_dn0 = assign27330_e38087_d_n0;
        locals.var_chi__blk814_dn2 = assign27330_e38087_d_n2;
        locals.var_chi__blk814_dn6 = assign27330_e38087_d_n6;
        locals.var_chi__blk814_dn7 = assign27330_e38087_d_n7;
        locals.var_chi__blk814_dn10 = assign27330_e38087_d_n10;
        locals.var_chi__blk814_dn11 = assign27330_e38087_d_n11;
        locals.var_chi__blk814_dn12 = assign27330_e38087_d_n12;
        locals.var_chi__blk814_dn17 = assign27330_e38087_d_n17;

        let (assign27350_e38115, assign27350_e38115_d_n0, assign27350_e38115_d_n2, assign27350_e38115_d_n6, assign27350_e38115_d_n7, assign27350_e38115_d_n10, assign27350_e38115_d_n11, assign27350_e38115_d_n12, assign27350_e38115_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27350_e38111: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27350_e38113: f64 = (assign27350_e38111 + 0.1);
        (assign27350_e38113, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign27350_e38115;
        locals.var_vgpld_shift_dn0 = assign27350_e38115_d_n0;
        locals.var_vgpld_shift_dn2 = assign27350_e38115_d_n2;
        locals.var_vgpld_shift_dn6 = assign27350_e38115_d_n6;
        locals.var_vgpld_shift_dn7 = assign27350_e38115_d_n7;
        locals.var_vgpld_shift_dn10 = assign27350_e38115_d_n10;
        locals.var_vgpld_shift_dn11 = assign27350_e38115_d_n11;
        locals.var_vgpld_shift_dn12 = assign27350_e38115_d_n12;
        locals.var_vgpld_shift_dn17 = assign27350_e38115_d_n17;

        let (assign27360_e38132, assign27360_e38132_d_n0, assign27360_e38132_d_n2, assign27360_e38132_d_n6, assign27360_e38132_d_n7, assign27360_e38132_d_n10, assign27360_e38132_d_n11, assign27360_e38132_d_n12, assign27360_e38132_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27360_e38126: f64 = (-locals.var_vxbgmtcl);
        let assign27360_e38127: f64 = (locals.var_beta * assign27360_e38126);
        let assign27360_e38128: f64 = (assign27360_e38127).exp();
        let assign27360_e38130: f64 = (assign27360_e38128 + 1e-50);
        (assign27360_e38130, (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27360_e38128 * ((locals.var_beta_dn10 * assign27360_e38126) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign27360_e38132;
        locals.var_exp_bvbs__blk833_dn0 = assign27360_e38132_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign27360_e38132_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign27360_e38132_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign27360_e38132_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign27360_e38132_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign27360_e38132_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign27360_e38132_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign27360_e38132_d_n17;

        let (assign27370_e38145, assign27370_e38145_d_n0, assign27370_e38145_d_n2, assign27370_e38145_d_n6, assign27370_e38145_d_n7, assign27370_e38145_d_n10, assign27370_e38145_d_n11, assign27370_e38145_d_n12, assign27370_e38145_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27370_e38143: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27370_e38143, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27370_e38145;
        locals.var_t0__blk770_dn0 = assign27370_e38145_d_n0;
        locals.var_t0__blk770_dn2 = assign27370_e38145_d_n2;
        locals.var_t0__blk770_dn6 = assign27370_e38145_d_n6;
        locals.var_t0__blk770_dn7 = assign27370_e38145_d_n7;
        locals.var_t0__blk770_dn10 = assign27370_e38145_d_n10;
        locals.var_t0__blk770_dn11 = assign27370_e38145_d_n11;
        locals.var_t0__blk770_dn12 = assign27370_e38145_d_n12;
        locals.var_t0__blk770_dn17 = assign27370_e38145_d_n17;

        let (assign27380_e38158, assign27380_e38158_d_n0, assign27380_e38158_d_n2, assign27380_e38158_d_n6, assign27380_e38158_d_n7, assign27380_e38158_d_n10, assign27380_e38158_d_n11, assign27380_e38158_d_n12, assign27380_e38158_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27380_e38156: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign27380_e38156, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27380_e38158;
        locals.var_cnst1over_dn0 = assign27380_e38158_d_n0;
        locals.var_cnst1over_dn2 = assign27380_e38158_d_n2;
        locals.var_cnst1over_dn6 = assign27380_e38158_d_n6;
        locals.var_cnst1over_dn7 = assign27380_e38158_d_n7;
        locals.var_cnst1over_dn10 = assign27380_e38158_d_n10;
        locals.var_cnst1over_dn11 = assign27380_e38158_d_n11;
        locals.var_cnst1over_dn12 = assign27380_e38158_d_n12;
        locals.var_cnst1over_dn17 = assign27380_e38158_d_n17;

        let (assign27390_e38171, assign27390_e38171_d_n0, assign27390_e38171_d_n2, assign27390_e38171_d_n6, assign27390_e38171_d_n7, assign27390_e38171_d_n10, assign27390_e38171_d_n11, assign27390_e38171_d_n12, assign27390_e38171_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27390_e38169: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign27390_e38169, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign27390_e38171;
        locals.var_gammachi_dn0 = assign27390_e38171_d_n0;
        locals.var_gammachi_dn2 = assign27390_e38171_d_n2;
        locals.var_gammachi_dn6 = assign27390_e38171_d_n6;
        locals.var_gammachi_dn7 = assign27390_e38171_d_n7;
        locals.var_gammachi_dn10 = assign27390_e38171_d_n10;
        locals.var_gammachi_dn11 = assign27390_e38171_d_n11;
        locals.var_gammachi_dn12 = assign27390_e38171_d_n12;
        locals.var_gammachi_dn17 = assign27390_e38171_d_n17;

        let (assign27400_e38184, assign27400_e38184_d_n0, assign27400_e38184_d_n2, assign27400_e38184_d_n6, assign27400_e38184_d_n7, assign27400_e38184_d_n10, assign27400_e38184_d_n11, assign27400_e38184_d_n12, assign27400_e38184_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27400_e38182: f64 = (locals.var_beta2 * locals.var_fac1p2__blk801);
        (assign27400_e38182, (locals.var_beta2 * locals.var_fac1p2__blk801_dn0), (locals.var_beta2 * locals.var_fac1p2__blk801_dn2), (locals.var_beta2 * locals.var_fac1p2__blk801_dn6), (locals.var_beta2 * locals.var_fac1p2__blk801_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk801) + (locals.var_beta2 * locals.var_fac1p2__blk801_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk801_dn11), (locals.var_beta2 * locals.var_fac1p2__blk801_dn12), (locals.var_beta2 * locals.var_fac1p2__blk801_dn17),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27400_e38184;
        locals.var_t0__blk770_dn0 = assign27400_e38184_d_n0;
        locals.var_t0__blk770_dn2 = assign27400_e38184_d_n2;
        locals.var_t0__blk770_dn6 = assign27400_e38184_d_n6;
        locals.var_t0__blk770_dn7 = assign27400_e38184_d_n7;
        locals.var_t0__blk770_dn10 = assign27400_e38184_d_n10;
        locals.var_t0__blk770_dn11 = assign27400_e38184_d_n11;
        locals.var_t0__blk770_dn12 = assign27400_e38184_d_n12;
        locals.var_t0__blk770_dn17 = assign27400_e38184_d_n17;

        let (assign27410_e38197, assign27410_e38197_d_n0, assign27410_e38197_d_n2, assign27410_e38197_d_n6, assign27410_e38197_d_n7, assign27410_e38197_d_n10, assign27410_e38197_d_n11, assign27410_e38197_d_n12, assign27410_e38197_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27410_e38195: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign27410_e38195, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27410_e38197;
        locals.var_psi_dn0 = assign27410_e38197_d_n0;
        locals.var_psi_dn2 = assign27410_e38197_d_n2;
        locals.var_psi_dn6 = assign27410_e38197_d_n6;
        locals.var_psi_dn7 = assign27410_e38197_d_n7;
        locals.var_psi_dn10 = assign27410_e38197_d_n10;
        locals.var_psi_dn11 = assign27410_e38197_d_n11;
        locals.var_psi_dn12 = assign27410_e38197_d_n12;
        locals.var_psi_dn17 = assign27410_e38197_d_n17;

        let (assign27420_e38224, assign27420_e38224_d_n0, assign27420_e38224_d_n2, assign27420_e38224_d_n6, assign27420_e38224_d_n7, assign27420_e38224_d_n10, assign27420_e38224_d_n11, assign27420_e38224_d_n12, assign27420_e38224_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27420_e38208: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign27420_e38211: f64 = (locals.var_psi * locals.var_psi);
        let assign27420_e38212: f64 = (assign27420_e38208 + assign27420_e38211);
        let assign27420_e38213: f64 = (assign27420_e38212).ln();
        let assign27420_e38216: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign27420_e38217: f64 = (assign27420_e38216).ln();
        let assign27420_e38218: f64 = (assign27420_e38213 - assign27420_e38217);
        let assign27420_e38221: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27420_e38222: f64 = (assign27420_e38218 + assign27420_e38221);
        (assign27420_e38222, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27420_e38212) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27420_e38212) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27420_e38212) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27420_e38212) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27420_e38212) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign27420_e38216)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27420_e38212) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27420_e38212) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27420_e38212) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27420_e38224;
        locals.var_chi_1_dn0 = assign27420_e38224_d_n0;
        locals.var_chi_1_dn2 = assign27420_e38224_d_n2;
        locals.var_chi_1_dn6 = assign27420_e38224_d_n6;
        locals.var_chi_1_dn7 = assign27420_e38224_d_n7;
        locals.var_chi_1_dn10 = assign27420_e38224_d_n10;
        locals.var_chi_1_dn11 = assign27420_e38224_d_n11;
        locals.var_chi_1_dn12 = assign27420_e38224_d_n12;
        locals.var_chi_1_dn17 = assign27420_e38224_d_n17;

        let (assign27430_e38239, assign27430_e38239_d_n0, assign27430_e38239_d_n2, assign27430_e38239_d_n6, assign27430_e38239_d_n7, assign27430_e38239_d_n10, assign27430_e38239_d_n11, assign27430_e38239_d_n12, assign27430_e38239_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27430_e38235: f64 = (locals.var_psi - locals.var_chi_1);
        let assign27430_e38237: f64 = (assign27430_e38235 - 1.0);
        (assign27430_e38237, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27430_e38239;
        locals.var_tmf1_dn0 = assign27430_e38239_d_n0;
        locals.var_tmf1_dn2 = assign27430_e38239_d_n2;
        locals.var_tmf1_dn6 = assign27430_e38239_d_n6;
        locals.var_tmf1_dn7 = assign27430_e38239_d_n7;
        locals.var_tmf1_dn10 = assign27430_e38239_d_n10;
        locals.var_tmf1_dn11 = assign27430_e38239_d_n11;
        locals.var_tmf1_dn12 = assign27430_e38239_d_n12;
        locals.var_tmf1_dn17 = assign27430_e38239_d_n17;

        let (assign27440_e38254, assign27440_e38254_d_n0, assign27440_e38254_d_n2, assign27440_e38254_d_n6, assign27440_e38254_d_n7, assign27440_e38254_d_n10, assign27440_e38254_d_n11, assign27440_e38254_d_n12, assign27440_e38254_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27440_e38250: f64 = (4.0 * locals.var_psi);
        let assign27440_e38252: f64 = assign27440_e38250;
        (assign27440_e38252, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27440_e38254;
        locals.var_tmf2_dn0 = assign27440_e38254_d_n0;
        locals.var_tmf2_dn2 = assign27440_e38254_d_n2;
        locals.var_tmf2_dn6 = assign27440_e38254_d_n6;
        locals.var_tmf2_dn7 = assign27440_e38254_d_n7;
        locals.var_tmf2_dn10 = assign27440_e38254_d_n10;
        locals.var_tmf2_dn11 = assign27440_e38254_d_n11;
        locals.var_tmf2_dn12 = assign27440_e38254_d_n12;
        locals.var_tmf2_dn17 = assign27440_e38254_d_n17;

    }

    pub(super) fn stamp_transient_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27450_e38271, assign27450_e38271_d_n0, assign27450_e38271_d_n2, assign27450_e38271_d_n6, assign27450_e38271_d_n7, assign27450_e38271_d_n10, assign27450_e38271_d_n11, assign27450_e38271_d_n12, assign27450_e38271_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let (assign27450_e38269, assign27450_e38269_d_n0, assign27450_e38269_d_n2, assign27450_e38269_d_n6, assign27450_e38269_d_n7, assign27450_e38269_d_n10, assign27450_e38269_d_n11, assign27450_e38269_d_n12, assign27450_e38269_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27450_e38268: f64 = (-locals.var_tmf2);
                (assign27450_e38268, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27450_e38269, assign27450_e38269_d_n0, assign27450_e38269_d_n2, assign27450_e38269_d_n6, assign27450_e38269_d_n7, assign27450_e38269_d_n10, assign27450_e38269_d_n11, assign27450_e38269_d_n12, assign27450_e38269_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27450_e38271;
        locals.var_tmf2_dn0 = assign27450_e38271_d_n0;
        locals.var_tmf2_dn2 = assign27450_e38271_d_n2;
        locals.var_tmf2_dn6 = assign27450_e38271_d_n6;
        locals.var_tmf2_dn7 = assign27450_e38271_d_n7;
        locals.var_tmf2_dn10 = assign27450_e38271_d_n10;
        locals.var_tmf2_dn11 = assign27450_e38271_d_n11;
        locals.var_tmf2_dn12 = assign27450_e38271_d_n12;
        locals.var_tmf2_dn17 = assign27450_e38271_d_n17;

        let (assign27460_e38287, assign27460_e38287_d_n0, assign27460_e38287_d_n2, assign27460_e38287_d_n6, assign27460_e38287_d_n7, assign27460_e38287_d_n10, assign27460_e38287_d_n11, assign27460_e38287_d_n12, assign27460_e38287_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27460_e38282: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27460_e38284: f64 = (assign27460_e38282 + locals.var_tmf2);
        let assign27460_e38285: f64 = (assign27460_e38284).sqrt();
        (assign27460_e38285, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27460_e38285)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27460_e38287;
        locals.var_tmf2_dn0 = assign27460_e38287_d_n0;
        locals.var_tmf2_dn2 = assign27460_e38287_d_n2;
        locals.var_tmf2_dn6 = assign27460_e38287_d_n6;
        locals.var_tmf2_dn7 = assign27460_e38287_d_n7;
        locals.var_tmf2_dn10 = assign27460_e38287_d_n10;
        locals.var_tmf2_dn11 = assign27460_e38287_d_n11;
        locals.var_tmf2_dn12 = assign27460_e38287_d_n12;
        locals.var_tmf2_dn17 = assign27460_e38287_d_n17;

        let (assign27470_e38304, assign27470_e38304_d_n0, assign27470_e38304_d_n2, assign27470_e38304_d_n6, assign27470_e38304_d_n7, assign27470_e38304_d_n10, assign27470_e38304_d_n11, assign27470_e38304_d_n12, assign27470_e38304_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27470_e38300: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27470_e38301: f64 = (1.0 + assign27470_e38300);
        let assign27470_e38302: f64 = (0.5 * assign27470_e38301);
        (assign27470_e38302, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27470_e38304;
        locals.var_t1__blk771_dn0 = assign27470_e38304_d_n0;
        locals.var_t1__blk771_dn2 = assign27470_e38304_d_n2;
        locals.var_t1__blk771_dn6 = assign27470_e38304_d_n6;
        locals.var_t1__blk771_dn7 = assign27470_e38304_d_n7;
        locals.var_t1__blk771_dn10 = assign27470_e38304_d_n10;
        locals.var_t1__blk771_dn11 = assign27470_e38304_d_n11;
        locals.var_t1__blk771_dn12 = assign27470_e38304_d_n12;
        locals.var_t1__blk771_dn17 = assign27470_e38304_d_n17;

        let (assign27480_e38325, assign27480_e38325_d_n0, assign27480_e38325_d_n2, assign27480_e38325_d_n6, assign27480_e38325_d_n7, assign27480_e38325_d_n10, assign27480_e38325_d_n11, assign27480_e38325_d_n12, assign27480_e38325_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27480_e38318: f64 = 2.0;
        let assign27480_e38319: f64 = (locals.var_tmf1 + assign27480_e38318);
        let assign27480_e38321: f64 = (assign27480_e38319 / locals.var_tmf2);
        let assign27480_e38322: f64 = (1.0 - assign27480_e38321);
        let assign27480_e38323: f64 = (0.5 * assign27480_e38322);
        (assign27480_e38323, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27480_e38325;
        locals.var_t2__blk772_dn0 = assign27480_e38325_d_n0;
        locals.var_t2__blk772_dn2 = assign27480_e38325_d_n2;
        locals.var_t2__blk772_dn6 = assign27480_e38325_d_n6;
        locals.var_t2__blk772_dn7 = assign27480_e38325_d_n7;
        locals.var_t2__blk772_dn10 = assign27480_e38325_d_n10;
        locals.var_t2__blk772_dn11 = assign27480_e38325_d_n11;
        locals.var_t2__blk772_dn12 = assign27480_e38325_d_n12;
        locals.var_t2__blk772_dn17 = assign27480_e38325_d_n17;

        let (assign27490_e38342, assign27490_e38342_d_n0, assign27490_e38342_d_n2, assign27490_e38342_d_n6, assign27490_e38342_d_n7, assign27490_e38342_d_n10, assign27490_e38342_d_n11, assign27490_e38342_d_n12, assign27490_e38342_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27490_e38338: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27490_e38339: f64 = (0.5 * assign27490_e38338);
        let assign27490_e38340: f64 = (locals.var_psi - assign27490_e38339);
        (assign27490_e38340, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27490_e38342;
        locals.var_chi_1_dn0 = assign27490_e38342_d_n0;
        locals.var_chi_1_dn2 = assign27490_e38342_d_n2;
        locals.var_chi_1_dn6 = assign27490_e38342_d_n6;
        locals.var_chi_1_dn7 = assign27490_e38342_d_n7;
        locals.var_chi_1_dn10 = assign27490_e38342_d_n10;
        locals.var_chi_1_dn11 = assign27490_e38342_d_n11;
        locals.var_chi_1_dn12 = assign27490_e38342_d_n12;
        locals.var_chi_1_dn17 = assign27490_e38342_d_n17;

        let (assign27500_e38355, assign27500_e38355_d_n0, assign27500_e38355_d_n2, assign27500_e38355_d_n6, assign27500_e38355_d_n7, assign27500_e38355_d_n10, assign27500_e38355_d_n11, assign27500_e38355_d_n12, assign27500_e38355_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27500_e38353: f64 = (locals.var_psi - locals.var_chi_1);
        (assign27500_e38353, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27500_e38355;
        locals.var_psi_dn0 = assign27500_e38355_d_n0;
        locals.var_psi_dn2 = assign27500_e38355_d_n2;
        locals.var_psi_dn6 = assign27500_e38355_d_n6;
        locals.var_psi_dn7 = assign27500_e38355_d_n7;
        locals.var_psi_dn10 = assign27500_e38355_d_n10;
        locals.var_psi_dn11 = assign27500_e38355_d_n11;
        locals.var_psi_dn12 = assign27500_e38355_d_n12;
        locals.var_psi_dn17 = assign27500_e38355_d_n17;

        let (assign27510_e38370, assign27510_e38370_d_n0, assign27510_e38370_d_n2, assign27510_e38370_d_n6, assign27510_e38370_d_n7, assign27510_e38370_d_n10, assign27510_e38370_d_n11, assign27510_e38370_d_n12, assign27510_e38370_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27510_e38367: f64 = (locals.var_beta * 0.1);
        let assign27510_e38368: f64 = (locals.var_psi + assign27510_e38367);
        (assign27510_e38368, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27510_e38370;
        locals.var_psi_dn0 = assign27510_e38370_d_n0;
        locals.var_psi_dn2 = assign27510_e38370_d_n2;
        locals.var_psi_dn6 = assign27510_e38370_d_n6;
        locals.var_psi_dn7 = assign27510_e38370_d_n7;
        locals.var_psi_dn10 = assign27510_e38370_d_n10;
        locals.var_psi_dn11 = assign27510_e38370_d_n11;
        locals.var_psi_dn12 = assign27510_e38370_d_n12;
        locals.var_psi_dn17 = assign27510_e38370_d_n17;

        let (assign27520_e38397, assign27520_e38397_d_n0, assign27520_e38397_d_n2, assign27520_e38397_d_n6, assign27520_e38397_d_n7, assign27520_e38397_d_n10, assign27520_e38397_d_n11, assign27520_e38397_d_n12, assign27520_e38397_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27520_e38381: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign27520_e38384: f64 = (locals.var_psi * locals.var_psi);
        let assign27520_e38385: f64 = (assign27520_e38381 + assign27520_e38384);
        let assign27520_e38386: f64 = (assign27520_e38385).ln();
        let assign27520_e38389: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign27520_e38390: f64 = (assign27520_e38389).ln();
        let assign27520_e38391: f64 = (assign27520_e38386 - assign27520_e38390);
        let assign27520_e38394: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27520_e38395: f64 = (assign27520_e38391 + assign27520_e38394);
        (assign27520_e38395, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27520_e38385) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27520_e38385) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27520_e38385) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27520_e38385) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27520_e38385) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign27520_e38389)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27520_e38385) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27520_e38385) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27520_e38385) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign27520_e38397;
        locals.var_chi_b_dn0 = assign27520_e38397_d_n0;
        locals.var_chi_b_dn2 = assign27520_e38397_d_n2;
        locals.var_chi_b_dn6 = assign27520_e38397_d_n6;
        locals.var_chi_b_dn7 = assign27520_e38397_d_n7;
        locals.var_chi_b_dn10 = assign27520_e38397_d_n10;
        locals.var_chi_b_dn11 = assign27520_e38397_d_n11;
        locals.var_chi_b_dn12 = assign27520_e38397_d_n12;
        locals.var_chi_b_dn17 = assign27520_e38397_d_n17;

        let (assign27530_e38408, assign27530_e38408_d_n0, assign27530_e38408_d_n2, assign27530_e38408_d_n6, assign27530_e38408_d_n7, assign27530_e38408_d_n10, assign27530_e38408_d_n11, assign27530_e38408_d_n12, assign27530_e38408_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign27530_e38408;
        locals.var_chi_a_dn0 = assign27530_e38408_d_n0;
        locals.var_chi_a_dn2 = assign27530_e38408_d_n2;
        locals.var_chi_a_dn6 = assign27530_e38408_d_n6;
        locals.var_chi_a_dn7 = assign27530_e38408_d_n7;
        locals.var_chi_a_dn10 = assign27530_e38408_d_n10;
        locals.var_chi_a_dn11 = assign27530_e38408_d_n11;
        locals.var_chi_a_dn12 = assign27530_e38408_d_n12;
        locals.var_chi_a_dn17 = assign27530_e38408_d_n17;

        let (assign27540_e38425, assign27540_e38425_d_n0, assign27540_e38425_d_n2, assign27540_e38425_d_n6, assign27540_e38425_d_n7, assign27540_e38425_d_n10, assign27540_e38425_d_n11, assign27540_e38425_d_n12, assign27540_e38425_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27540_e38419: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign27540_e38422: f64 = (0.0008 * 75.0);
        let assign27540_e38423: f64 = (assign27540_e38419 - assign27540_e38422);
        (assign27540_e38423, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27540_e38425;
        locals.var_tmf1_dn0 = assign27540_e38425_d_n0;
        locals.var_tmf1_dn2 = assign27540_e38425_d_n2;
        locals.var_tmf1_dn6 = assign27540_e38425_d_n6;
        locals.var_tmf1_dn7 = assign27540_e38425_d_n7;
        locals.var_tmf1_dn10 = assign27540_e38425_d_n10;
        locals.var_tmf1_dn11 = assign27540_e38425_d_n11;
        locals.var_tmf1_dn12 = assign27540_e38425_d_n12;
        locals.var_tmf1_dn17 = assign27540_e38425_d_n17;

        let (assign27550_e38442, assign27550_e38442_d_n0, assign27550_e38442_d_n2, assign27550_e38442_d_n6, assign27550_e38442_d_n7, assign27550_e38442_d_n10, assign27550_e38442_d_n11, assign27550_e38442_d_n12, assign27550_e38442_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27550_e38436: f64 = (4.0 * locals.var_chi_b);
        let assign27550_e38439: f64 = (0.0008 * 75.0);
        let assign27550_e38440: f64 = (assign27550_e38436 * assign27550_e38439);
        (assign27550_e38440, ((4.0 * locals.var_chi_b_dn0) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn2) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn6) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn7) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn10) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn11) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn12) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn17) * assign27550_e38439),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27550_e38442;
        locals.var_tmf2_dn0 = assign27550_e38442_d_n0;
        locals.var_tmf2_dn2 = assign27550_e38442_d_n2;
        locals.var_tmf2_dn6 = assign27550_e38442_d_n6;
        locals.var_tmf2_dn7 = assign27550_e38442_d_n7;
        locals.var_tmf2_dn10 = assign27550_e38442_d_n10;
        locals.var_tmf2_dn11 = assign27550_e38442_d_n11;
        locals.var_tmf2_dn12 = assign27550_e38442_d_n12;
        locals.var_tmf2_dn17 = assign27550_e38442_d_n17;

        let (assign27560_e38459, assign27560_e38459_d_n0, assign27560_e38459_d_n2, assign27560_e38459_d_n6, assign27560_e38459_d_n7, assign27560_e38459_d_n10, assign27560_e38459_d_n11, assign27560_e38459_d_n12, assign27560_e38459_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let (assign27560_e38457, assign27560_e38457_d_n0, assign27560_e38457_d_n2, assign27560_e38457_d_n6, assign27560_e38457_d_n7, assign27560_e38457_d_n10, assign27560_e38457_d_n11, assign27560_e38457_d_n12, assign27560_e38457_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27560_e38456: f64 = (-locals.var_tmf2);
                (assign27560_e38456, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27560_e38457, assign27560_e38457_d_n0, assign27560_e38457_d_n2, assign27560_e38457_d_n6, assign27560_e38457_d_n7, assign27560_e38457_d_n10, assign27560_e38457_d_n11, assign27560_e38457_d_n12, assign27560_e38457_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27560_e38459;
        locals.var_tmf2_dn0 = assign27560_e38459_d_n0;
        locals.var_tmf2_dn2 = assign27560_e38459_d_n2;
        locals.var_tmf2_dn6 = assign27560_e38459_d_n6;
        locals.var_tmf2_dn7 = assign27560_e38459_d_n7;
        locals.var_tmf2_dn10 = assign27560_e38459_d_n10;
        locals.var_tmf2_dn11 = assign27560_e38459_d_n11;
        locals.var_tmf2_dn12 = assign27560_e38459_d_n12;
        locals.var_tmf2_dn17 = assign27560_e38459_d_n17;

        let (assign27570_e38475, assign27570_e38475_d_n0, assign27570_e38475_d_n2, assign27570_e38475_d_n6, assign27570_e38475_d_n7, assign27570_e38475_d_n10, assign27570_e38475_d_n11, assign27570_e38475_d_n12, assign27570_e38475_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27570_e38470: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27570_e38472: f64 = (assign27570_e38470 + locals.var_tmf2);
        let assign27570_e38473: f64 = (assign27570_e38472).sqrt();
        (assign27570_e38473, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27570_e38473)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27570_e38475;
        locals.var_tmf2_dn0 = assign27570_e38475_d_n0;
        locals.var_tmf2_dn2 = assign27570_e38475_d_n2;
        locals.var_tmf2_dn6 = assign27570_e38475_d_n6;
        locals.var_tmf2_dn7 = assign27570_e38475_d_n7;
        locals.var_tmf2_dn10 = assign27570_e38475_d_n10;
        locals.var_tmf2_dn11 = assign27570_e38475_d_n11;
        locals.var_tmf2_dn12 = assign27570_e38475_d_n12;
        locals.var_tmf2_dn17 = assign27570_e38475_d_n17;

        let (assign27580_e38492, assign27580_e38492_d_n0, assign27580_e38492_d_n2, assign27580_e38492_d_n6, assign27580_e38492_d_n7, assign27580_e38492_d_n10, assign27580_e38492_d_n11, assign27580_e38492_d_n12, assign27580_e38492_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27580_e38488: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27580_e38489: f64 = (1.0 + assign27580_e38488);
        let assign27580_e38490: f64 = (0.5 * assign27580_e38489);
        (assign27580_e38490, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27580_e38492;
        locals.var_t1__blk771_dn0 = assign27580_e38492_d_n0;
        locals.var_t1__blk771_dn2 = assign27580_e38492_d_n2;
        locals.var_t1__blk771_dn6 = assign27580_e38492_d_n6;
        locals.var_t1__blk771_dn7 = assign27580_e38492_d_n7;
        locals.var_t1__blk771_dn10 = assign27580_e38492_d_n10;
        locals.var_t1__blk771_dn11 = assign27580_e38492_d_n11;
        locals.var_t1__blk771_dn12 = assign27580_e38492_d_n12;
        locals.var_t1__blk771_dn17 = assign27580_e38492_d_n17;

        let (assign27590_e38515, assign27590_e38515_d_n0, assign27590_e38515_d_n2, assign27590_e38515_d_n6, assign27590_e38515_d_n7, assign27590_e38515_d_n10, assign27590_e38515_d_n11, assign27590_e38515_d_n12, assign27590_e38515_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27590_e38506: f64 = (2.0 * 0.0008);
        let assign27590_e38508: f64 = (assign27590_e38506 * 75.0);
        let assign27590_e38509: f64 = (locals.var_tmf1 + assign27590_e38508);
        let assign27590_e38511: f64 = (assign27590_e38509 / locals.var_tmf2);
        let assign27590_e38512: f64 = (1.0 - assign27590_e38511);
        let assign27590_e38513: f64 = (0.5 * assign27590_e38512);
        (assign27590_e38513, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27590_e38515;
        locals.var_t2__blk772_dn0 = assign27590_e38515_d_n0;
        locals.var_t2__blk772_dn2 = assign27590_e38515_d_n2;
        locals.var_t2__blk772_dn6 = assign27590_e38515_d_n6;
        locals.var_t2__blk772_dn7 = assign27590_e38515_d_n7;
        locals.var_t2__blk772_dn10 = assign27590_e38515_d_n10;
        locals.var_t2__blk772_dn11 = assign27590_e38515_d_n11;
        locals.var_t2__blk772_dn12 = assign27590_e38515_d_n12;
        locals.var_t2__blk772_dn17 = assign27590_e38515_d_n17;

        let (assign27600_e38532, assign27600_e38532_d_n0, assign27600_e38532_d_n2, assign27600_e38532_d_n6, assign27600_e38532_d_n7, assign27600_e38532_d_n10, assign27600_e38532_d_n11, assign27600_e38532_d_n12, assign27600_e38532_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27600_e38528: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27600_e38529: f64 = (0.5 * assign27600_e38528);
        let assign27600_e38530: f64 = (locals.var_chi_b - assign27600_e38529);
        (assign27600_e38530, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27600_e38532;
        locals.var_chi__blk814_dn0 = assign27600_e38532_d_n0;
        locals.var_chi__blk814_dn2 = assign27600_e38532_d_n2;
        locals.var_chi__blk814_dn6 = assign27600_e38532_d_n6;
        locals.var_chi__blk814_dn7 = assign27600_e38532_d_n7;
        locals.var_chi__blk814_dn10 = assign27600_e38532_d_n10;
        locals.var_chi__blk814_dn11 = assign27600_e38532_d_n11;
        locals.var_chi__blk814_dn12 = assign27600_e38532_d_n12;
        locals.var_chi__blk814_dn17 = assign27600_e38532_d_n17;

        let (assign27610_e38547, assign27610_e38547_d_n0, assign27610_e38547_d_n2, assign27610_e38547_d_n6, assign27610_e38547_d_n7, assign27610_e38547_d_n10, assign27610_e38547_d_n11, assign27610_e38547_d_n12, assign27610_e38547_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27610_e38543: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign27610_e38545: f64 = (assign27610_e38543 - locals.var_vxbgmtcl);
        (assign27610_e38545, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27610_e38547;
        locals.var_ps0ld_dn0 = assign27610_e38547_d_n0;
        locals.var_ps0ld_dn2 = assign27610_e38547_d_n2;
        locals.var_ps0ld_dn6 = assign27610_e38547_d_n6;
        locals.var_ps0ld_dn7 = assign27610_e38547_d_n7;
        locals.var_ps0ld_dn10 = assign27610_e38547_d_n10;
        locals.var_ps0ld_dn11 = assign27610_e38547_d_n11;
        locals.var_ps0ld_dn12 = assign27610_e38547_d_n12;
        locals.var_ps0ld_dn17 = assign27610_e38547_d_n17;

        let (assign27620_e38564, assign27620_e38564_d_n0, assign27620_e38564_d_n2, assign27620_e38564_d_n6, assign27620_e38564_d_n7, assign27620_e38564_d_n10, assign27620_e38564_d_n11, assign27620_e38564_d_n12, assign27620_e38564_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27620_e38558: f64 = (locals.var_chi__blk814 - 1.0);
        let assign27620_e38560: f64 = (-locals.var_chi__blk814);
        let assign27620_e38561: f64 = (assign27620_e38560).exp();
        let assign27620_e38562: f64 = (assign27620_e38558 + assign27620_e38561);
        (assign27620_e38562, (locals.var_chi__blk814_dn0 + (assign27620_e38561 * (-locals.var_chi__blk814_dn0))), (locals.var_chi__blk814_dn2 + (assign27620_e38561 * (-locals.var_chi__blk814_dn2))), (locals.var_chi__blk814_dn6 + (assign27620_e38561 * (-locals.var_chi__blk814_dn6))), (locals.var_chi__blk814_dn7 + (assign27620_e38561 * (-locals.var_chi__blk814_dn7))), (locals.var_chi__blk814_dn10 + (assign27620_e38561 * (-locals.var_chi__blk814_dn10))), (locals.var_chi__blk814_dn11 + (assign27620_e38561 * (-locals.var_chi__blk814_dn11))), (locals.var_chi__blk814_dn12 + (assign27620_e38561 * (-locals.var_chi__blk814_dn12))), (locals.var_chi__blk814_dn17 + (assign27620_e38561 * (-locals.var_chi__blk814_dn17))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27620_e38564;
        locals.var_t1__blk771_dn0 = assign27620_e38564_d_n0;
        locals.var_t1__blk771_dn2 = assign27620_e38564_d_n2;
        locals.var_t1__blk771_dn6 = assign27620_e38564_d_n6;
        locals.var_t1__blk771_dn7 = assign27620_e38564_d_n7;
        locals.var_t1__blk771_dn10 = assign27620_e38564_d_n10;
        locals.var_t1__blk771_dn11 = assign27620_e38564_d_n11;
        locals.var_t1__blk771_dn12 = assign27620_e38564_d_n12;
        locals.var_t1__blk771_dn17 = assign27620_e38564_d_n17;

        let assign27630_e38568: f64 = (10.0 * 2.220446049250313e-16);
        let assign27630_e38569: f64 = if locals.var_t1__blk771 < assign27630_e38568 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign27630_e38569;

        let (assign27640_e38584, assign27640_e38584_d_n0, assign27640_e38584_d_n2, assign27640_e38584_d_n6, assign27640_e38584_d_n7, assign27640_e38584_d_n10, assign27640_e38584_d_n11, assign27640_e38584_d_n12, assign27640_e38584_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign27640_e38582: f64 = (10.0 * 2.220446049250313e-16);
        (assign27640_e38582, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27640_e38584;
        locals.var_t1__blk771_dn0 = assign27640_e38584_d_n0;
        locals.var_t1__blk771_dn2 = assign27640_e38584_d_n2;
        locals.var_t1__blk771_dn6 = assign27640_e38584_d_n6;
        locals.var_t1__blk771_dn7 = assign27640_e38584_d_n7;
        locals.var_t1__blk771_dn10 = assign27640_e38584_d_n10;
        locals.var_t1__blk771_dn11 = assign27640_e38584_d_n11;
        locals.var_t1__blk771_dn12 = assign27640_e38584_d_n12;
        locals.var_t1__blk771_dn17 = assign27640_e38584_d_n17;

        let (assign27650_e38596, assign27650_e38596_d_n0, assign27650_e38596_d_n2, assign27650_e38596_d_n6, assign27650_e38596_d_n7, assign27650_e38596_d_n10, assign27650_e38596_d_n11, assign27650_e38596_d_n12, assign27650_e38596_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27650_e38594: f64 = (locals.var_t1__blk771).sqrt();
        (assign27650_e38594, (locals.var_t1__blk771_dn0 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn2 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn6 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn7 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn10 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn11 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn12 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn17 / (2.0 * assign27650_e38594)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27650_e38596;
        locals.var_t2__blk772_dn0 = assign27650_e38596_d_n0;
        locals.var_t2__blk772_dn2 = assign27650_e38596_d_n2;
        locals.var_t2__blk772_dn6 = assign27650_e38596_d_n6;
        locals.var_t2__blk772_dn7 = assign27650_e38596_d_n7;
        locals.var_t2__blk772_dn10 = assign27650_e38596_d_n10;
        locals.var_t2__blk772_dn11 = assign27650_e38596_d_n11;
        locals.var_t2__blk772_dn12 = assign27650_e38596_d_n12;
        locals.var_t2__blk772_dn17 = assign27650_e38596_d_n17;

        let (assign27660_e38609, assign27660_e38609_d_n0, assign27660_e38609_d_n2, assign27660_e38609_d_n6, assign27660_e38609_d_n7, assign27660_e38609_d_n10, assign27660_e38609_d_n11, assign27660_e38609_d_n12, assign27660_e38609_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27660_e38607: f64 = (locals.var_cnst0over * locals.var_t2__blk772);
        (assign27660_e38607, ((locals.var_cnst0over_dn0 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27660_e38609;
        locals.var_qbuld_dn0 = assign27660_e38609_d_n0;
        locals.var_qbuld_dn2 = assign27660_e38609_d_n2;
        locals.var_qbuld_dn6 = assign27660_e38609_d_n6;
        locals.var_qbuld_dn7 = assign27660_e38609_d_n7;
        locals.var_qbuld_dn10 = assign27660_e38609_d_n10;
        locals.var_qbuld_dn11 = assign27660_e38609_d_n11;
        locals.var_qbuld_dn12 = assign27660_e38609_d_n12;
        locals.var_qbuld_dn17 = assign27660_e38609_d_n17;

        let (assign27670_e38624, assign27670_e38624_d_n0, assign27670_e38624_d_n2, assign27670_e38624_d_n6, assign27670_e38624_d_n7, assign27670_e38624_d_n10, assign27670_e38624_d_n11, assign27670_e38624_d_n12, assign27670_e38624_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27670_e38621: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27670_e38622: f64 = (locals.var_cox0 * assign27670_e38621);
        (assign27670_e38622, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27670_e38624;
        locals.var_qsuld_dn0 = assign27670_e38624_d_n0;
        locals.var_qsuld_dn2 = assign27670_e38624_d_n2;
        locals.var_qsuld_dn6 = assign27670_e38624_d_n6;
        locals.var_qsuld_dn7 = assign27670_e38624_d_n7;
        locals.var_qsuld_dn10 = assign27670_e38624_d_n10;
        locals.var_qsuld_dn11 = assign27670_e38624_d_n11;
        locals.var_qsuld_dn12 = assign27670_e38624_d_n12;
        locals.var_qsuld_dn17 = assign27670_e38624_d_n17;

        let assign27680_e38627: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign27680_e38627;

        let (assign27690_e38644, assign27690_e38644_d_n0, assign27690_e38644_d_n2, assign27690_e38644_d_n6, assign27690_e38644_d_n7, assign27690_e38644_d_n10, assign27690_e38644_d_n11, assign27690_e38644_d_n12, assign27690_e38644_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27690_e38640: f64 = (-locals.var_vxbgmtcl);
        let assign27690_e38641: f64 = (locals.var_beta * assign27690_e38640);
        let assign27690_e38642: f64 = (assign27690_e38641).exp();
        (assign27690_e38642, (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27690_e38642 * ((locals.var_beta_dn10 * assign27690_e38640) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign27690_e38644;
        locals.var_exp_bvbs__blk833_dn0 = assign27690_e38644_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign27690_e38644_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign27690_e38644_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign27690_e38644_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign27690_e38644_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign27690_e38644_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign27690_e38644_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign27690_e38644_d_n17;

        let (assign27700_e38659, assign27700_e38659_d_n0, assign27700_e38659_d_n2, assign27700_e38659_d_n6, assign27700_e38659_d_n7, assign27700_e38659_d_n10, assign27700_e38659_d_n11, assign27700_e38659_d_n12, assign27700_e38659_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27700_e38657: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27700_e38657, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27700_e38659;
        locals.var_t0__blk770_dn0 = assign27700_e38659_d_n0;
        locals.var_t0__blk770_dn2 = assign27700_e38659_d_n2;
        locals.var_t0__blk770_dn6 = assign27700_e38659_d_n6;
        locals.var_t0__blk770_dn7 = assign27700_e38659_d_n7;
        locals.var_t0__blk770_dn10 = assign27700_e38659_d_n10;
        locals.var_t0__blk770_dn11 = assign27700_e38659_d_n11;
        locals.var_t0__blk770_dn12 = assign27700_e38659_d_n12;
        locals.var_t0__blk770_dn17 = assign27700_e38659_d_n17;

        let (assign27710_e38674, assign27710_e38674_d_n0, assign27710_e38674_d_n2, assign27710_e38674_d_n6, assign27710_e38674_d_n7, assign27710_e38674_d_n10, assign27710_e38674_d_n11, assign27710_e38674_d_n12, assign27710_e38674_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27710_e38672: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign27710_e38672, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27710_e38674;
        locals.var_cnst1over_dn0 = assign27710_e38674_d_n0;
        locals.var_cnst1over_dn2 = assign27710_e38674_d_n2;
        locals.var_cnst1over_dn6 = assign27710_e38674_d_n6;
        locals.var_cnst1over_dn7 = assign27710_e38674_d_n7;
        locals.var_cnst1over_dn10 = assign27710_e38674_d_n10;
        locals.var_cnst1over_dn11 = assign27710_e38674_d_n11;
        locals.var_cnst1over_dn12 = assign27710_e38674_d_n12;
        locals.var_cnst1over_dn17 = assign27710_e38674_d_n17;

    }

    pub(super) fn stamp_transient_block_95(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27720_e38689, assign27720_e38689_d_n0, assign27720_e38689_d_n2, assign27720_e38689_d_n6, assign27720_e38689_d_n7, assign27720_e38689_d_n10, assign27720_e38689_d_n11, assign27720_e38689_d_n12, assign27720_e38689_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27720_e38687: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign27720_e38687, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_cfs1__blk842, locals.var_cfs1__blk842_dn0, locals.var_cfs1__blk842_dn2, locals.var_cfs1__blk842_dn6, locals.var_cfs1__blk842_dn7, locals.var_cfs1__blk842_dn10, locals.var_cfs1__blk842_dn11, locals.var_cfs1__blk842_dn12, locals.var_cfs1__blk842_dn17,)
    }
};
        locals.var_cfs1__blk842 = assign27720_e38689;
        locals.var_cfs1__blk842_dn0 = assign27720_e38689_d_n0;
        locals.var_cfs1__blk842_dn2 = assign27720_e38689_d_n2;
        locals.var_cfs1__blk842_dn6 = assign27720_e38689_d_n6;
        locals.var_cfs1__blk842_dn7 = assign27720_e38689_d_n7;
        locals.var_cfs1__blk842_dn10 = assign27720_e38689_d_n10;
        locals.var_cfs1__blk842_dn11 = assign27720_e38689_d_n11;
        locals.var_cfs1__blk842_dn12 = assign27720_e38689_d_n12;
        locals.var_cfs1__blk842_dn17 = assign27720_e38689_d_n17;

        let (assign27730_e38702,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk787,)
    }
};
        locals.var_flg_conv__blk787 = assign27730_e38702;

        let (assign27740_e38715,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign27740_e38715;

    }
}
