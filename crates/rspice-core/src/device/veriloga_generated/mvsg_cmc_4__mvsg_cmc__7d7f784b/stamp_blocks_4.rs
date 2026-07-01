#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31510_e28661: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd);
        locals.var_fn382_calc_iq__vdx = assign31510_e28661;
        locals.var_fn382_calc_iq__vdx_dn4 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn4);
        locals.var_fn382_calc_iq__vdx_dn5 = ((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__fsd) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn5));
        locals.var_fn382_calc_iq__vdx_dn8 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn8);
        locals.var_fn382_calc_iq__vdx_dn9 = ((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__fsd) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn9));

        let (assign31520_e28725, assign31520_e28725_d_n4, assign31520_e28725_d_n5, assign31520_e28725_d_n8, assign31520_e28725_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31520_e28671: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28673: f64 = (assign31520_e28671 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28674: f64 = assign31520_e28673;
        let assign31520_e28677: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28679: f64 = (assign31520_e28677 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28680: f64 = (-assign31520_e28679);
        let assign31520_e28683: f64 = (0.001 / p.p53);
        let assign31520_e28686: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28688: f64 = (assign31520_e28686 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28689: f64 = (-assign31520_e28688);
        let assign31520_e28690: f64 = (assign31520_e28683 * assign31520_e28689);
        let assign31520_e28691: f64 = (assign31520_e28690).tanh();
        let assign31520_e28692: f64 = (assign31520_e28680 * assign31520_e28691);
        let assign31520_e28693: f64 = (assign31520_e28674 + assign31520_e28692);
        let assign31520_e28694: f64 = (0.5 * assign31520_e28693);
        (assign31520_e28694, (0.5 * ((-((assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-(-((assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * ((-((assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-(-((assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))),)
    } else {
        let (assign31520_e28724, assign31520_e28724_d_n4, assign31520_e28724_d_n5, assign31520_e28724_d_n8, assign31520_e28724_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31520_e28701: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28703: f64 = (assign31520_e28701 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28704: f64 = assign31520_e28703;
                let assign31520_e28707: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28709: f64 = (assign31520_e28707 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28710: f64 = (-assign31520_e28709);
                let assign31520_e28713: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28715: f64 = (assign31520_e28713 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28716: f64 = (-assign31520_e28715);
                let assign31520_e28717: f64 = (assign31520_e28710 * assign31520_e28716);
                let assign31520_e28719: f64 = (assign31520_e28717 + p.p53);
                let assign31520_e28720: f64 = (assign31520_e28719).sqrt();
                let assign31520_e28721: f64 = (assign31520_e28704 + assign31520_e28720);
                let assign31520_e28722: f64 = (0.5 * assign31520_e28721);
                (assign31520_e28722, (0.5 * ((-((assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28716) + (assign31520_e28710 * (-(-((assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31520_e28720)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28716) + (assign31520_e28710 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31520_e28720)))), (0.5 * ((-((assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28716) + (assign31520_e28710 * (-(-((assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31520_e28720)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28716) + (assign31520_e28710 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31520_e28720)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31520_e28724, assign31520_e28724_d_n4, assign31520_e28724_d_n5, assign31520_e28724_d_n8, assign31520_e28724_d_n9,)
    }
};
        let assign31520_e28727: f64 = (assign31520_e28725).powf(locals.var_fn382_calc_iq__beta);
        let assign31520_e28728: f64 = (1.0 + assign31520_e28727);
        let assign31520_e28731: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31520_e28732: f64 = (assign31520_e28728).powf(assign31520_e28731);
        let assign31520_e28733: f64 = (1.0 / assign31520_e28732);
        locals.var_fn382_calc_iq__fds = assign31520_e28733;
        locals.var_fn382_calc_iq__fds_dn4 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n4)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n4 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n4)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n4 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn5 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n5)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n5 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n5)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n5 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn8 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n8)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n8 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n8)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n8 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn9 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n9)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n9 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n9)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n9 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));

        let assign31530_e28735: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31530_e28737: f64 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds);
        locals.var_fn382_calc_iq__vsx = assign31530_e28737;
        locals.var_fn382_calc_iq__vsx_dn4 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn4);
        locals.var_fn382_calc_iq__vsx_dn5 = (((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__fds) + (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn5));
        locals.var_fn382_calc_iq__vsx_dn8 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn8);
        locals.var_fn382_calc_iq__vsx_dn9 = (((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__fds) + (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn9));

        let assign31540_e28740: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__myarg);
        let assign31540_e28742: f64 = (assign31540_e28740 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31540_e28742;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31540_e28740 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((-locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);

        let assign31550_e28745: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign31550_e28745;

        let (assign31560_e28749, assign31560_e28749_d_n4, assign31560_e28749_d_n5, assign31560_e28749_d_n8, assign31560_e28749_d_n9,) = {
    if (locals.var_guard388 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31560_e28749;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31560_e28749_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31560_e28749_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31560_e28749_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31560_e28749_d_n9;

        let assign31570_e28752: f64 = (-50.0);
        let assign31570_e28753: f64 = if locals.var_fn382_calc_iq__exparg < assign31570_e28752 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign31570_e28753;

        let (assign31580_e28760, assign31580_e28760_d_n4, assign31580_e28760_d_n5, assign31580_e28760_d_n8, assign31580_e28760_d_n9,) = {
    if ((locals.var_guard388 == 0.0) && (locals.var_guard389 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31580_e28760;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31580_e28760_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31580_e28760_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31580_e28760_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31580_e28760_d_n9;

        let (assign31590_e28773, assign31590_e28773_d_n4, assign31590_e28773_d_n5, assign31590_e28773_d_n8, assign31590_e28773_d_n9,) = {
    if ((locals.var_guard388 == 0.0) && (locals.var_guard389 == 0.0)) {
        let assign31590_e28769: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31590_e28770: f64 = (1.0 + assign31590_e28769);
        let assign31590_e28771: f64 = (1.0 / assign31590_e28770);
        (assign31590_e28771, (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31590_e28770 * assign31590_e28770))),)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31590_e28773;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31590_e28773_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31590_e28773_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31590_e28773_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31590_e28773_d_n9;

        let assign31600_e28776: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__vsx);
        let assign31600_e28780: f64 = (p.p51 * 0.1);
        let assign31600_e28782: f64 = (assign31600_e28780 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31600_e28784: f64 = (assign31600_e28782 * locals.var_fn382_calc_iq__ffs);
        let assign31600_e28785: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31600_e28784);
        let assign31600_e28786: f64 = (assign31600_e28776 - assign31600_e28785);
        let assign31600_e28788: f64 = (assign31600_e28786 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etas = assign31600_e28788;
        locals.var_fn382_calc_iq__etas_dn4 = (((((-locals.var_fn382_calc_iq__vsx_dn4) - (locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31600_e28780 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffs) + (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etas_dn5 = (((((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__vsx_dn5) - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etas_dn8 = (((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__vsx_dn8) - (-(assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etas_dn9 = (((((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__vsx_dn9) - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));

        let assign31610_e28791: f64 = if locals.var_fn382_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign31610_e28791;

        let (assign31620_e28797, assign31620_e28797_d_n4, assign31620_e28797_d_n5, assign31620_e28797_d_n8, assign31620_e28797_d_n9,) = {
    if (locals.var_guard390 != 0.0) {
        let assign31620_e28795: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas);
        (assign31620_e28795, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31620_e28797;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31620_e28797_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31620_e28797_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31620_e28797_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31620_e28797_d_n9;

        let assign31630_e28800: f64 = (-50.0);
        let assign31630_e28801: f64 = if locals.var_fn382_calc_iq__etas < assign31630_e28800 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign31630_e28801;

        let (assign31640_e28811, assign31640_e28811_d_n4, assign31640_e28811_d_n5, assign31640_e28811_d_n8, assign31640_e28811_d_n9,) = {
    if ((locals.var_guard390 == 0.0) && (locals.var_guard391 != 0.0)) {
        let assign31640_e28808: f64 = (locals.var_fn382_calc_iq__etas).exp();
        let assign31640_e28809: f64 = (locals.var_fn382_calc_iq__qref * assign31640_e28808);
        (assign31640_e28809, ((locals.var_fn382_calc_iq__qref_dn4 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn5))), (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31640_e28811;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31640_e28811_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31640_e28811_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31640_e28811_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31640_e28811_d_n9;

        let (assign31650_e28825, assign31650_e28825_d_n4, assign31650_e28825_d_n5, assign31650_e28825_d_n8, assign31650_e28825_d_n9,) = {
    if ((locals.var_guard390 == 0.0) && (locals.var_guard391 == 0.0)) {
        let assign31650_e28820: f64 = (locals.var_fn382_calc_iq__etas).exp();
        let assign31650_e28821: f64 = (1.0 + assign31650_e28820);
        let assign31650_e28822: f64 = (assign31650_e28821).ln();
        let assign31650_e28823: f64 = (locals.var_fn382_calc_iq__qref * assign31650_e28822);
        (assign31650_e28823, ((locals.var_fn382_calc_iq__qref_dn4 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn4) / assign31650_e28821))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn5) / assign31650_e28821))), (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn8) / assign31650_e28821)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn9) / assign31650_e28821))),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31650_e28825;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31650_e28825_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31650_e28825_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31650_e28825_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31650_e28825_d_n9;

        let assign31660_e28828: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__myarg);
        let assign31660_e28830: f64 = (assign31660_e28828 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31660_e28830;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31660_e28828 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);

        let assign31670_e28833: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard392 = assign31670_e28833;

        let (assign31680_e28837, assign31680_e28837_d_n4, assign31680_e28837_d_n5, assign31680_e28837_d_n8, assign31680_e28837_d_n9,) = {
    if (locals.var_guard392 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31680_e28837;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31680_e28837_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31680_e28837_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31680_e28837_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31680_e28837_d_n9;

        let assign31690_e28840: f64 = (-50.0);
        let assign31690_e28841: f64 = if locals.var_fn382_calc_iq__exparg < assign31690_e28840 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign31690_e28841;

        let (assign31700_e28848, assign31700_e28848_d_n4, assign31700_e28848_d_n5, assign31700_e28848_d_n8, assign31700_e28848_d_n9,) = {
    if ((locals.var_guard392 == 0.0) && (locals.var_guard393 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31700_e28848;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31700_e28848_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31700_e28848_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31700_e28848_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31700_e28848_d_n9;

        let (assign31710_e28861, assign31710_e28861_d_n4, assign31710_e28861_d_n5, assign31710_e28861_d_n8, assign31710_e28861_d_n9,) = {
    if ((locals.var_guard392 == 0.0) && (locals.var_guard393 == 0.0)) {
        let assign31710_e28857: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31710_e28858: f64 = (1.0 + assign31710_e28857);
        let assign31710_e28859: f64 = (1.0 / assign31710_e28858);
        (assign31710_e28859, (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31710_e28858 * assign31710_e28858))),)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31710_e28861;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31710_e28861_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31710_e28861_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31710_e28861_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31710_e28861_d_n9;

        let assign31720_e28864: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdx);
        let assign31720_e28868: f64 = (p.p51 * 0.1);
        let assign31720_e28870: f64 = (assign31720_e28868 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31720_e28872: f64 = (assign31720_e28870 * locals.var_fn382_calc_iq__ffd);
        let assign31720_e28873: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31720_e28872);
        let assign31720_e28874: f64 = (assign31720_e28864 - assign31720_e28873);
        let assign31720_e28876: f64 = (assign31720_e28874 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etad = assign31720_e28876;
        locals.var_fn382_calc_iq__etad_dn4 = (((((-locals.var_fn382_calc_iq__vdx_dn4) - (locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31720_e28868 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffd) + (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etad_dn5 = (((((-locals.var_fn382_calc_iq__vdx_dn5) - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etad_dn8 = (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vdx_dn8) - (-(assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etad_dn9 = (((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdx_dn9) - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));

        let assign31730_e28879: f64 = if locals.var_fn382_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign31730_e28879;

        let (assign31740_e28885, assign31740_e28885_d_n4, assign31740_e28885_d_n5, assign31740_e28885_d_n8, assign31740_e28885_d_n9,) = {
    if (locals.var_guard394 != 0.0) {
        let assign31740_e28883: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad);
        (assign31740_e28883, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31740_e28885;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31740_e28885_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31740_e28885_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31740_e28885_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31740_e28885_d_n9;

        let assign31750_e28888: f64 = (-50.0);
        let assign31750_e28889: f64 = if locals.var_fn382_calc_iq__etad < assign31750_e28888 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign31750_e28889;

        let (assign31760_e28899, assign31760_e28899_d_n4, assign31760_e28899_d_n5, assign31760_e28899_d_n8, assign31760_e28899_d_n9,) = {
    if ((locals.var_guard394 == 0.0) && (locals.var_guard395 != 0.0)) {
        let assign31760_e28896: f64 = (locals.var_fn382_calc_iq__etad).exp();
        let assign31760_e28897: f64 = (locals.var_fn382_calc_iq__qref * assign31760_e28896);
        (assign31760_e28897, ((locals.var_fn382_calc_iq__qref_dn4 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn5))), (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31760_e28899;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31760_e28899_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31760_e28899_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31760_e28899_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31760_e28899_d_n9;

        let (assign31770_e28913, assign31770_e28913_d_n4, assign31770_e28913_d_n5, assign31770_e28913_d_n8, assign31770_e28913_d_n9,) = {
    if ((locals.var_guard394 == 0.0) && (locals.var_guard395 == 0.0)) {
        let assign31770_e28908: f64 = (locals.var_fn382_calc_iq__etad).exp();
        let assign31770_e28909: f64 = (1.0 + assign31770_e28908);
        let assign31770_e28910: f64 = (assign31770_e28909).ln();
        let assign31770_e28911: f64 = (locals.var_fn382_calc_iq__qref * assign31770_e28910);
        (assign31770_e28911, ((locals.var_fn382_calc_iq__qref_dn4 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn4) / assign31770_e28909))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn5) / assign31770_e28909))), (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn8) / assign31770_e28909)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn9) / assign31770_e28909))),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31770_e28913;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31770_e28913_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31770_e28913_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31770_e28913_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31770_e28913_d_n9;

        let assign31780_e28916: f64 = (locals.var_fn382_calc_iq__qinvs - locals.var_fn382_calc_iq__qinvd);
        let assign31780_e28918: f64 = (assign31780_e28916 / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc = assign31780_e28918;
        locals.var_fn382_calc_iq__vdsc_dn4 = ((((locals.var_fn382_calc_iq__qinvs_dn4 - locals.var_fn382_calc_iq__qinvd_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31780_e28916 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin));
        locals.var_fn382_calc_iq__vdsc_dn5 = ((locals.var_fn382_calc_iq__qinvs_dn5 - locals.var_fn382_calc_iq__qinvd_dn5) / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc_dn8 = ((locals.var_fn382_calc_iq__qinvs_dn8 - locals.var_fn382_calc_iq__qinvd_dn8) / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc_dn9 = ((locals.var_fn382_calc_iq__qinvs_dn9 - locals.var_fn382_calc_iq__qinvd_dn9) / locals.var_fn382_calc_iq__cgin);

        let assign31790_e28921: f64 = (locals.var_fn382_calc_iq__vdsc / locals.var_fn382_calc_iq__vdsat);
        locals.var_fn382_calc_iq__myarg = assign31790_e28921;
        locals.var_fn382_calc_iq__myarg_dn4 = (((locals.var_fn382_calc_iq__vdsc_dn4 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn4)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn5 = (((locals.var_fn382_calc_iq__vdsc_dn5 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn5)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn8 = (((locals.var_fn382_calc_iq__vdsc_dn8 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn8)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn9 = (((locals.var_fn382_calc_iq__vdsc_dn9 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn9)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));

        let (assign31800_e28947, assign31800_e28947_d_n4, assign31800_e28947_d_n5, assign31800_e28947_d_n8, assign31800_e28947_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31800_e28931: f64 = (0.001 / p.p53);
        let assign31800_e28933: f64 = (assign31800_e28931 * locals.var_fn382_calc_iq__myarg);
        let assign31800_e28934: f64 = (assign31800_e28933).tanh();
        let assign31800_e28935: f64 = (locals.var_fn382_calc_iq__myarg * assign31800_e28934);
        (assign31800_e28935, ((locals.var_fn382_calc_iq__myarg_dn4 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn4) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn5 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn5) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn8 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn8) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn9 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn9) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))),)
    } else {
        let (assign31800_e28946, assign31800_e28946_d_n4, assign31800_e28946_d_n5, assign31800_e28946_d_n8, assign31800_e28946_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31800_e28941: f64 = (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg);
                let assign31800_e28943: f64 = (assign31800_e28941 + p.p53);
                let assign31800_e28944: f64 = (assign31800_e28943).sqrt();
                (assign31800_e28944, (((locals.var_fn382_calc_iq__myarg_dn4 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn4)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn5 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn5)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn8 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn8)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn9 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn9)) / (2.0 * assign31800_e28944)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31800_e28946, assign31800_e28946_d_n4, assign31800_e28946_d_n5, assign31800_e28946_d_n8, assign31800_e28946_d_n9,)
    }
};
        let assign31800_e28949: f64 = (assign31800_e28947).powf(locals.var_fn382_calc_iq__beta);
        let assign31800_e28950: f64 = (1.0 + assign31800_e28949);
        let assign31800_e28953: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31800_e28954: f64 = (assign31800_e28950).powf(assign31800_e28953);
        let assign31800_e28955: f64 = (locals.var_fn382_calc_iq__myarg / assign31800_e28954);
        locals.var_fn382_calc_iq__fsat = assign31800_e28955;
        locals.var_fn382_calc_iq__fsat_dn4 = (((locals.var_fn382_calc_iq__myarg_dn4 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n4)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n4 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n4)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n4 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn5 = (((locals.var_fn382_calc_iq__myarg_dn5 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n5)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n5 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n5)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n5 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn8 = (((locals.var_fn382_calc_iq__myarg_dn8 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n8)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n8 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n8)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n8 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn9 = (((locals.var_fn382_calc_iq__myarg_dn9 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n9)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n9 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n9)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n9 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));

        let assign31810_e28958: f64 = (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat);
        locals.var_fn382_calc_iq__vel = assign31810_e28958;
        locals.var_fn382_calc_iq__vel_dn4 = ((locals.var_fn382_calc_iq__vxf_dn4 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn4));
        locals.var_fn382_calc_iq__vel_dn5 = ((locals.var_fn382_calc_iq__vxf_dn5 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn5));
        locals.var_fn382_calc_iq__vel_dn8 = ((locals.var_fn382_calc_iq__vxf_dn8 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn8));
        locals.var_fn382_calc_iq__vel_dn9 = ((locals.var_fn382_calc_iq__vxf_dn9 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn9));

        let assign31820_e28961: f64 = (locals.var_fn382_calc_iq__type * locals.var_fn382_calc_iq__w);
        let assign31820_e28963: f64 = (assign31820_e28961 * locals.var_fn382_calc_iq__ngf);
        let assign31820_e28965: f64 = (assign31820_e28963 * 0.5);
        let assign31820_e28968: f64 = (locals.var_fn382_calc_iq__qinvs + locals.var_fn382_calc_iq__qinvd);
        let assign31820_e28969: f64 = (assign31820_e28965 * assign31820_e28968);
        let assign31820_e28971: f64 = (assign31820_e28969 * locals.var_fn382_calc_iq__vel);
        let assign31820_e28973: f64 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout = assign31820_e28973;
        locals.var_fn382_calc_iq__idsout_dn4 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn4 + locals.var_fn382_calc_iq__qinvd_dn4)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn4)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn5 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn5 + locals.var_fn382_calc_iq__qinvd_dn5)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn5)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn8 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn8 + locals.var_fn382_calc_iq__qinvd_dn8)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn8)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn9 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn9 + locals.var_fn382_calc_iq__qinvd_dn9)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn9)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn22 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__idsout_dn23 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__idsout_dn25 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__idsout_dn26 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn26);

        let assign31830_e28977: f64 = (2.302585092994046 * locals.var_fn382_calc_iq__phitin);
        let assign31830_e28978: f64 = (locals.var_fn382_calc_iq__ss / assign31830_e28977);
        locals.var_fn382_calc_iq__n0 = assign31830_e28978;
        locals.var_fn382_calc_iq__n0_dn4 = (-((locals.var_fn382_calc_iq__ss * (2.302585092994046 * locals.var_fn382_calc_iq__phitin_dn4)) / (assign31830_e28977 * assign31830_e28977)));

        let assign31840_e28981: f64 = (2.0 * locals.var_fn382_calc_iq__n0);
        let assign31840_e28983: f64 = (assign31840_e28981 * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit0 = assign31840_e28983;
        locals.var_fn382_calc_iq__two_n_phit0_dn4 = (((2.0 * locals.var_fn382_calc_iq__n0_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31840_e28981 * locals.var_fn382_calc_iq__phitin_dn4));

        let assign31850_e28986: f64 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__qref0 = assign31850_e28986;
        locals.var_fn382_calc_iq__qref0_dn4 = ((locals.var_fn382_calc_iq__cgin_dn4 * locals.var_fn382_calc_iq__two_n_phit0) + (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit0_dn4));

        let assign31860_e28990: f64 = (p.p51 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31860_e28992: f64 = (assign31860_e28990 / 2.0);
        let assign31860_e28993: f64 = (locals.var_fn382_calc_iq__vtof - assign31860_e28992);
        locals.var_fn382_calc_iq__myarg0 = assign31860_e28993;
        locals.var_fn382_calc_iq__myarg0_dn4 = (locals.var_fn382_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn382_calc_iq__alpha_phit_dn4) / 2.0));

        let (assign31870_e29037, assign31870_e29037_d_n5, assign31870_e29037_d_n8, assign31870_e29037_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31870_e29001: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29004: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29007: f64 = (0.001 / p.p53);
        let assign31870_e29010: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29011: f64 = (assign31870_e29007 * assign31870_e29010);
        let assign31870_e29012: f64 = (assign31870_e29011).tanh();
        let assign31870_e29013: f64 = (assign31870_e29004 * assign31870_e29012);
        let assign31870_e29014: f64 = (assign31870_e29001 + assign31870_e29013);
        let assign31870_e29015: f64 = (0.5 * assign31870_e29014);
        (assign31870_e29015, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))),)
    } else {
        let (assign31870_e29036, assign31870_e29036_d_n5, assign31870_e29036_d_n8, assign31870_e29036_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31870_e29022: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29025: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29028: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29029: f64 = (assign31870_e29025 * assign31870_e29028);
                let assign31870_e29031: f64 = (assign31870_e29029 + p.p53);
                let assign31870_e29032: f64 = (assign31870_e29031).sqrt();
                let assign31870_e29033: f64 = (assign31870_e29022 + assign31870_e29032);
                let assign31870_e29034: f64 = (0.5 * assign31870_e29033);
                (assign31870_e29034, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31870_e29028) + (assign31870_e29025 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31870_e29032)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31870_e29028) + (assign31870_e29025 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31870_e29032)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31870_e29028) + (assign31870_e29025 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31870_e29032)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31870_e29036, assign31870_e29036_d_n5, assign31870_e29036_d_n8, assign31870_e29036_d_n9,)
    }
};
        let assign31870_e29039: f64 = (assign31870_e29037 - locals.var_fn382_calc_iq__myarg0);
        let assign31870_e29041: f64 = (assign31870_e29039 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign31870_e29041;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31870_e29039 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = (assign31870_e29037_d_n5 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn8 = (assign31870_e29037_d_n8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (assign31870_e29037_d_n9 / locals.var_fn382_calc_iq__alpha_phit);

        let assign31880_e29044: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign31880_e29044;

        let (assign31890_e29048, assign31890_e29048_d_n4, assign31890_e29048_d_n5, assign31890_e29048_d_n8, assign31890_e29048_d_n9,) = {
    if (locals.var_guard396 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31890_e29048;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31890_e29048_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31890_e29048_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31890_e29048_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31890_e29048_d_n9;

        let assign31900_e29051: f64 = (-50.0);
        let assign31900_e29052: f64 = if locals.var_fn382_calc_iq__exparg0 < assign31900_e29051 { 1.0 } else { 0.0 };
        locals.var_guard397 = assign31900_e29052;

        let (assign31910_e29059, assign31910_e29059_d_n4, assign31910_e29059_d_n5, assign31910_e29059_d_n8, assign31910_e29059_d_n9,) = {
    if ((locals.var_guard396 == 0.0) && (locals.var_guard397 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31910_e29059;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31910_e29059_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31910_e29059_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31910_e29059_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31910_e29059_d_n9;

        let (assign31920_e29072, assign31920_e29072_d_n4, assign31920_e29072_d_n5, assign31920_e29072_d_n8, assign31920_e29072_d_n9,) = {
    if ((locals.var_guard396 == 0.0) && (locals.var_guard397 == 0.0)) {
        let assign31920_e29068: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign31920_e29069: f64 = (1.0 + assign31920_e29068);
        let assign31920_e29070: f64 = (1.0 / assign31920_e29069);
        (assign31920_e29070, (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign31920_e29069 * assign31920_e29069))),)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31920_e29072;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31920_e29072_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31920_e29072_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31920_e29072_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31920_e29072_d_n9;

    }

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31930_e29116, assign31930_e29116_d_n5, assign31930_e29116_d_n8, assign31930_e29116_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31930_e29080: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29083: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29086: f64 = (0.001 / p.p53);
        let assign31930_e29089: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29090: f64 = (assign31930_e29086 * assign31930_e29089);
        let assign31930_e29091: f64 = (assign31930_e29090).tanh();
        let assign31930_e29092: f64 = (assign31930_e29083 * assign31930_e29091);
        let assign31930_e29093: f64 = (assign31930_e29080 + assign31930_e29092);
        let assign31930_e29094: f64 = (0.5 * assign31930_e29093);
        (assign31930_e29094, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))),)
    } else {
        let (assign31930_e29115, assign31930_e29115_d_n5, assign31930_e29115_d_n8, assign31930_e29115_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31930_e29101: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29104: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29107: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29108: f64 = (assign31930_e29104 * assign31930_e29107);
                let assign31930_e29110: f64 = (assign31930_e29108 + p.p53);
                let assign31930_e29111: f64 = (assign31930_e29110).sqrt();
                let assign31930_e29112: f64 = (assign31930_e29101 + assign31930_e29111);
                let assign31930_e29113: f64 = (0.5 * assign31930_e29112);
                (assign31930_e29113, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31930_e29107) + (assign31930_e29104 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31930_e29111)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31930_e29107) + (assign31930_e29104 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31930_e29111)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31930_e29107) + (assign31930_e29104 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31930_e29111)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31930_e29115, assign31930_e29115_d_n5, assign31930_e29115_d_n8, assign31930_e29115_d_n9,)
    }
};
        let assign31930_e29120: f64 = (p.p51 * 0.1);
        let assign31930_e29122: f64 = (assign31930_e29120 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31930_e29124: f64 = (assign31930_e29122 * locals.var_fn382_calc_iq__ff0);
        let assign31930_e29125: f64 = (locals.var_fn382_calc_iq__vtof - assign31930_e29124);
        let assign31930_e29126: f64 = (assign31930_e29116 - assign31930_e29125);
        let assign31930_e29128: f64 = (assign31930_e29126 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0 = assign31930_e29128;
        locals.var_fn382_calc_iq__eta0_dn4 = ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (((assign31930_e29120 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ff0) + (assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign31930_e29126 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__eta0_dn5 = ((assign31930_e29116_d_n5 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0_dn8 = ((assign31930_e29116_d_n8 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0_dn9 = ((assign31930_e29116_d_n9 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);

        let assign31940_e29131: f64 = if locals.var_fn382_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign31940_e29131;

        let (assign31950_e29137, assign31950_e29137_d_n4, assign31950_e29137_d_n5, assign31950_e29137_d_n8, assign31950_e29137_d_n9,) = {
    if (locals.var_guard398 != 0.0) {
        let assign31950_e29135: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0);
        (assign31950_e29135, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__eta0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31950_e29137;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31950_e29137_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31950_e29137_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31950_e29137_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31950_e29137_d_n9;

        let assign31960_e29140: f64 = (-50.0);
        let assign31960_e29141: f64 = if locals.var_fn382_calc_iq__eta0 < assign31960_e29140 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign31960_e29141;

        let (assign31970_e29151, assign31970_e29151_d_n4, assign31970_e29151_d_n5, assign31970_e29151_d_n8, assign31970_e29151_d_n9,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 != 0.0)) {
        let assign31970_e29148: f64 = (locals.var_fn382_calc_iq__eta0).exp();
        let assign31970_e29149: f64 = (locals.var_fn382_calc_iq__qref0 * assign31970_e29148);
        (assign31970_e29149, ((locals.var_fn382_calc_iq__qref0_dn4 * assign31970_e29148) + (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31970_e29151;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31970_e29151_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31970_e29151_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31970_e29151_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31970_e29151_d_n9;

        let (assign31980_e29165, assign31980_e29165_d_n4, assign31980_e29165_d_n5, assign31980_e29165_d_n8, assign31980_e29165_d_n9,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 == 0.0)) {
        let assign31980_e29160: f64 = (locals.var_fn382_calc_iq__eta0).exp();
        let assign31980_e29161: f64 = (1.0 + assign31980_e29160);
        let assign31980_e29162: f64 = (assign31980_e29161).ln();
        let assign31980_e29163: f64 = (locals.var_fn382_calc_iq__qref0 * assign31980_e29162);
        (assign31980_e29163, ((locals.var_fn382_calc_iq__qref0_dn4 * assign31980_e29162) + (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn4) / assign31980_e29161))), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn5) / assign31980_e29161)), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn8) / assign31980_e29161)), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn9) / assign31980_e29161)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31980_e29165;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31980_e29165_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31980_e29165_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31980_e29165_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31980_e29165_d_n9;

        let assign31990_e29168: f64 = (locals.var_fn382_calc_iq__mu0 / locals.var_fn382_calc_iq__tfacmobin);
        locals.var_fn382_calc_iq__muf0 = assign31990_e29168;
        locals.var_fn382_calc_iq__muf0_dn4 = (-((locals.var_fn382_calc_iq__mu0 * locals.var_fn382_calc_iq__tfacmobin_dn4) / (locals.var_fn382_calc_iq__tfacmobin * locals.var_fn382_calc_iq__tfacmobin)));

        let assign32000_e29173: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tnomin);
        let assign32000_e29174: f64 = (1.0 + assign32000_e29173);
        let assign32000_e29178: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin);
        let assign32000_e29179: f64 = (1.0 + assign32000_e29178);
        let assign32000_e29180: f64 = (assign32000_e29174 / assign32000_e29179);
        let assign32000_e29181: f64 = (locals.var_fn382_calc_iq__vel0 * assign32000_e29180);
        locals.var_fn382_calc_iq__vx0 = assign32000_e29181;
        locals.var_fn382_calc_iq__vx0_dn4 = (locals.var_fn382_calc_iq__vel0 * (-((assign32000_e29174 * (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin_dn4)) / (assign32000_e29179 * assign32000_e29179))));

        let assign32010_e29184: f64 = (locals.var_fn382_calc_iq__vx0 * locals.var_fn382_calc_iq__lin);
        let assign32010_e29186: f64 = (assign32010_e29184 / locals.var_fn382_calc_iq__muf0);
        locals.var_fn382_calc_iq__vdsats0 = assign32010_e29186;
        locals.var_fn382_calc_iq__vdsats0_dn4 = ((((locals.var_fn382_calc_iq__vx0_dn4 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf0) - (assign32010_e29184 * locals.var_fn382_calc_iq__muf0_dn4)) / (locals.var_fn382_calc_iq__muf0 * locals.var_fn382_calc_iq__muf0));

        let assign32020_e29191: f64 = (2.0 * locals.var_fn382_calc_iq__qinvv0);
        let assign32020_e29193: f64 = (assign32020_e29191 / locals.var_fn382_calc_iq__cgin);
        let assign32020_e29195: f64 = (assign32020_e29193 / locals.var_fn382_calc_iq__vdsats0);
        let assign32020_e29196: f64 = (1.0 + assign32020_e29195);
        let assign32020_e29197: f64 = (assign32020_e29196).sqrt();
        let assign32020_e29198: f64 = (locals.var_fn382_calc_iq__vdsats0 * assign32020_e29197);
        let assign32020_e29200: f64 = (assign32020_e29198 - locals.var_fn382_calc_iq__vdsats0);
        locals.var_fn382_calc_iq__vdsats10 = assign32020_e29200;
        locals.var_fn382_calc_iq__vdsats10_dn4 = (((locals.var_fn382_calc_iq__vdsats0_dn4 * assign32020_e29197) + (locals.var_fn382_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn4) * locals.var_fn382_calc_iq__cgin) - (assign32020_e29191 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)) * locals.var_fn382_calc_iq__vdsats0) - (assign32020_e29193 * locals.var_fn382_calc_iq__vdsats0_dn4)) / (locals.var_fn382_calc_iq__vdsats0 * locals.var_fn382_calc_iq__vdsats0)) / (2.0 * assign32020_e29197)))) - locals.var_fn382_calc_iq__vdsats0_dn4);
        locals.var_fn382_calc_iq__vdsats10_dn5 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn5) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));
        locals.var_fn382_calc_iq__vdsats10_dn8 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn8) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));
        locals.var_fn382_calc_iq__vdsats10_dn9 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn9) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));

        let assign32030_e29204: f64 = (1.0 - locals.var_fn382_calc_iq__ff0);
        let assign32030_e29205: f64 = (locals.var_fn382_calc_iq__vdsats10 * assign32030_e29204);
        let assign32030_e29208: f64 = (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0);
        let assign32030_e29209: f64 = (assign32030_e29205 + assign32030_e29208);
        locals.var_fn382_calc_iq__vdsat10 = assign32030_e29209;
        locals.var_fn382_calc_iq__vdsat10_dn4 = (((locals.var_fn382_calc_iq__vdsats10_dn4 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit0_dn4 * locals.var_fn382_calc_iq__ff0) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn4)));
        locals.var_fn382_calc_iq__vdsat10_dn5 = (((locals.var_fn382_calc_iq__vdsats10_dn5 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn5))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn5));
        locals.var_fn382_calc_iq__vdsat10_dn8 = (((locals.var_fn382_calc_iq__vdsats10_dn8 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn8))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn8));
        locals.var_fn382_calc_iq__vdsat10_dn9 = (((locals.var_fn382_calc_iq__vdsats10_dn9 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn9))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn9));

        let (assign32040_e29267, assign32040_e29267_d_n4, assign32040_e29267_d_n5, assign32040_e29267_d_n8, assign32040_e29267_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign32040_e29220: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29221: f64 = assign32040_e29220;
        let assign32040_e29225: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29226: f64 = (-assign32040_e29225);
        let assign32040_e29229: f64 = (0.001 / p.p53);
        let assign32040_e29233: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29234: f64 = (-assign32040_e29233);
        let assign32040_e29235: f64 = (assign32040_e29229 * assign32040_e29234);
        let assign32040_e29236: f64 = (assign32040_e29235).tanh();
        let assign32040_e29237: f64 = (assign32040_e29226 * assign32040_e29236);
        let assign32040_e29238: f64 = (assign32040_e29221 + assign32040_e29237);
        let assign32040_e29239: f64 = (0.5 * assign32040_e29238);
        (assign32040_e29239, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))),)
    } else {
        let (assign32040_e29266, assign32040_e29266_d_n4, assign32040_e29266_d_n5, assign32040_e29266_d_n8, assign32040_e29266_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign32040_e29247: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29248: f64 = assign32040_e29247;
                let assign32040_e29252: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29253: f64 = (-assign32040_e29252);
                let assign32040_e29257: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29258: f64 = (-assign32040_e29257);
                let assign32040_e29259: f64 = (assign32040_e29253 * assign32040_e29258);
                let assign32040_e29261: f64 = (assign32040_e29259 + p.p53);
                let assign32040_e29262: f64 = (assign32040_e29261).sqrt();
                let assign32040_e29263: f64 = (assign32040_e29248 + assign32040_e29262);
                let assign32040_e29264: f64 = (0.5 * assign32040_e29263);
                (assign32040_e29264, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29258) + (assign32040_e29253 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32040_e29262)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29258) + (assign32040_e29253 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32040_e29262)))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29258) + (assign32040_e29253 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32040_e29262)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29258) + (assign32040_e29253 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32040_e29262)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign32040_e29266, assign32040_e29266_d_n4, assign32040_e29266_d_n5, assign32040_e29266_d_n8, assign32040_e29266_d_n9,)
    }
};
        let assign32040_e29269: f64 = (assign32040_e29267).powf(locals.var_fn382_calc_iq__beta);
        let assign32040_e29270: f64 = (1.0 + assign32040_e29269);
        let assign32040_e29273: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign32040_e29274: f64 = (assign32040_e29270).powf(assign32040_e29273);
        let assign32040_e29275: f64 = (1.0 / assign32040_e29274);
        locals.var_fn382_calc_iq__fsd0 = assign32040_e29275;
        locals.var_fn382_calc_iq__fsd0_dn4 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n4)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n4 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n4)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n4 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn5 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n5)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n5 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n5)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n5 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn8 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n8)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n8 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n8)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n8 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn9 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n9)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n9 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n9)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n9 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));

        let assign32050_e29278: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0);
        locals.var_fn382_calc_iq__vdx0 = assign32050_e29278;
        locals.var_fn382_calc_iq__vdx0_dn4 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn4);
        locals.var_fn382_calc_iq__vdx0_dn5 = ((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__fsd0) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn5));
        locals.var_fn382_calc_iq__vdx0_dn8 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn8);
        locals.var_fn382_calc_iq__vdx0_dn9 = ((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__fsd0) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn9));

        let (assign32060_e29342, assign32060_e29342_d_n4, assign32060_e29342_d_n5, assign32060_e29342_d_n8, assign32060_e29342_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign32060_e29288: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29290: f64 = (assign32060_e29288 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29291: f64 = assign32060_e29290;
        let assign32060_e29294: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29296: f64 = (assign32060_e29294 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29297: f64 = (-assign32060_e29296);
        let assign32060_e29300: f64 = (0.001 / p.p53);
        let assign32060_e29303: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29305: f64 = (assign32060_e29303 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29306: f64 = (-assign32060_e29305);
        let assign32060_e29307: f64 = (assign32060_e29300 * assign32060_e29306);
        let assign32060_e29308: f64 = (assign32060_e29307).tanh();
        let assign32060_e29309: f64 = (assign32060_e29297 * assign32060_e29308);
        let assign32060_e29310: f64 = (assign32060_e29291 + assign32060_e29309);
        let assign32060_e29311: f64 = (0.5 * assign32060_e29310);
        (assign32060_e29311, (0.5 * ((-((assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-(-((assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * ((-((assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-(-((assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))),)
    } else {
        let (assign32060_e29341, assign32060_e29341_d_n4, assign32060_e29341_d_n5, assign32060_e29341_d_n8, assign32060_e29341_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign32060_e29318: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29320: f64 = (assign32060_e29318 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29321: f64 = assign32060_e29320;
                let assign32060_e29324: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29326: f64 = (assign32060_e29324 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29327: f64 = (-assign32060_e29326);
                let assign32060_e29330: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29332: f64 = (assign32060_e29330 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29333: f64 = (-assign32060_e29332);
                let assign32060_e29334: f64 = (assign32060_e29327 * assign32060_e29333);
                let assign32060_e29336: f64 = (assign32060_e29334 + p.p53);
                let assign32060_e29337: f64 = (assign32060_e29336).sqrt();
                let assign32060_e29338: f64 = (assign32060_e29321 + assign32060_e29337);
                let assign32060_e29339: f64 = (0.5 * assign32060_e29338);
                (assign32060_e29339, (0.5 * ((-((assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29333) + (assign32060_e29327 * (-(-((assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32060_e29337)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29333) + (assign32060_e29327 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32060_e29337)))), (0.5 * ((-((assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29333) + (assign32060_e29327 * (-(-((assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32060_e29337)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29333) + (assign32060_e29327 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32060_e29337)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign32060_e29341, assign32060_e29341_d_n4, assign32060_e29341_d_n5, assign32060_e29341_d_n8, assign32060_e29341_d_n9,)
    }
};
        let assign32060_e29344: f64 = (assign32060_e29342).powf(locals.var_fn382_calc_iq__beta);
        let assign32060_e29345: f64 = (1.0 + assign32060_e29344);
        let assign32060_e29348: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign32060_e29349: f64 = (assign32060_e29345).powf(assign32060_e29348);
        let assign32060_e29350: f64 = (1.0 / assign32060_e29349);
        locals.var_fn382_calc_iq__fds0 = assign32060_e29350;
        locals.var_fn382_calc_iq__fds0_dn4 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n4)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n4 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n4)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n4 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn5 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n5)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n5 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n5)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n5 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn8 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n8)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n8 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n8)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n8 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn9 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n9)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n9 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n9)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n9 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));

        let assign32070_e29352: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32070_e29354: f64 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0);
        locals.var_fn382_calc_iq__vsx0 = assign32070_e29354;
        locals.var_fn382_calc_iq__vsx0_dn4 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn4);
        locals.var_fn382_calc_iq__vsx0_dn5 = (((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__fds0) + (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn5));
        locals.var_fn382_calc_iq__vsx0_dn8 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn8);
        locals.var_fn382_calc_iq__vsx0_dn9 = (((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__fds0) + (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn9));

        let assign32080_e29357: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__myarg0);
        let assign32080_e29359: f64 = (assign32080_e29357 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign32080_e29359;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign32080_e29357 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn8 = (locals.var_fn382_calc_iq__vgsin_dn8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (locals.var_fn382_calc_iq__vgsin_dn9 / locals.var_fn382_calc_iq__alpha_phit);

        let assign32090_e29362: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign32090_e29362;

        let (assign32100_e29366, assign32100_e29366_d_n4, assign32100_e29366_d_n5, assign32100_e29366_d_n8, assign32100_e29366_d_n9,) = {
    if (locals.var_guard400 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32100_e29366;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32100_e29366_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32100_e29366_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32100_e29366_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32100_e29366_d_n9;

        let assign32110_e29369: f64 = (-50.0);
        let assign32110_e29370: f64 = if locals.var_fn382_calc_iq__exparg0 < assign32110_e29369 { 1.0 } else { 0.0 };
        locals.var_guard401 = assign32110_e29370;

        let (assign32120_e29377, assign32120_e29377_d_n4, assign32120_e29377_d_n5, assign32120_e29377_d_n8, assign32120_e29377_d_n9,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32120_e29377;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32120_e29377_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32120_e29377_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32120_e29377_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32120_e29377_d_n9;

        let (assign32130_e29390, assign32130_e29390_d_n4, assign32130_e29390_d_n5, assign32130_e29390_d_n8, assign32130_e29390_d_n9,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 == 0.0)) {
        let assign32130_e29386: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign32130_e29387: f64 = (1.0 + assign32130_e29386);
        let assign32130_e29388: f64 = (1.0 / assign32130_e29387);
        (assign32130_e29388, (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign32130_e29387 * assign32130_e29387))),)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32130_e29390;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32130_e29390_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32130_e29390_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32130_e29390_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32130_e29390_d_n9;

        let assign32140_e29393: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__vsx0);
        let assign32140_e29397: f64 = (p.p51 * 0.1);
        let assign32140_e29399: f64 = (assign32140_e29397 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32140_e29401: f64 = (assign32140_e29399 * locals.var_fn382_calc_iq__ffs0);
        let assign32140_e29402: f64 = (locals.var_fn382_calc_iq__vtof - assign32140_e29401);
        let assign32140_e29403: f64 = (assign32140_e29393 - assign32140_e29402);
        let assign32140_e29405: f64 = (assign32140_e29403 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0 = assign32140_e29405;
        locals.var_fn382_calc_iq__etas0_dn4 = (((((-locals.var_fn382_calc_iq__vsx0_dn4) - (locals.var_fn382_calc_iq__vtof_dn4 - (((assign32140_e29397 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffs0) + (assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32140_e29403 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__etas0_dn5 = (((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__vsx0_dn5) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0_dn8 = (((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__vsx0_dn8) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0_dn9 = (((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__vsx0_dn9) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);

        let assign32150_e29408: f64 = if locals.var_fn382_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign32150_e29408;

        let (assign32160_e29414, assign32160_e29414_d_n4, assign32160_e29414_d_n5, assign32160_e29414_d_n8, assign32160_e29414_d_n9,) = {
    if (locals.var_guard402 != 0.0) {
        let assign32160_e29412: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0);
        (assign32160_e29412, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__etas0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32160_e29414;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32160_e29414_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32160_e29414_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32160_e29414_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32160_e29414_d_n9;

        let assign32170_e29417: f64 = (-50.0);
        let assign32170_e29418: f64 = if locals.var_fn382_calc_iq__etas0 < assign32170_e29417 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign32170_e29418;

        let (assign32180_e29428, assign32180_e29428_d_n4, assign32180_e29428_d_n5, assign32180_e29428_d_n8, assign32180_e29428_d_n9,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 != 0.0)) {
        let assign32180_e29425: f64 = (locals.var_fn382_calc_iq__etas0).exp();
        let assign32180_e29426: f64 = (locals.var_fn382_calc_iq__qref0 * assign32180_e29425);
        (assign32180_e29426, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32180_e29425) + (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32180_e29428;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32180_e29428_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32180_e29428_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32180_e29428_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32180_e29428_d_n9;

        let (assign32190_e29442, assign32190_e29442_d_n4, assign32190_e29442_d_n5, assign32190_e29442_d_n8, assign32190_e29442_d_n9,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 == 0.0)) {
        let assign32190_e29437: f64 = (locals.var_fn382_calc_iq__etas0).exp();
        let assign32190_e29438: f64 = (1.0 + assign32190_e29437);
        let assign32190_e29439: f64 = (assign32190_e29438).ln();
        let assign32190_e29440: f64 = (locals.var_fn382_calc_iq__qref0 * assign32190_e29439);
        (assign32190_e29440, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32190_e29439) + (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn4) / assign32190_e29438))), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn5) / assign32190_e29438)), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn8) / assign32190_e29438)), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn9) / assign32190_e29438)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32190_e29442;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32190_e29442_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32190_e29442_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32190_e29442_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32190_e29442_d_n9;

        let assign32200_e29445: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__myarg0);
        let assign32200_e29447: f64 = (assign32200_e29445 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign32200_e29447;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign32200_e29445 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = (locals.var_fn382_calc_iq__vgdin_dn5 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn8 = (locals.var_fn382_calc_iq__vgdin_dn8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (locals.var_fn382_calc_iq__vgdin_dn9 / locals.var_fn382_calc_iq__alpha_phit);

        let assign32210_e29450: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign32210_e29450;

        let (assign32220_e29454, assign32220_e29454_d_n4, assign32220_e29454_d_n5, assign32220_e29454_d_n8, assign32220_e29454_d_n9,) = {
    if (locals.var_guard404 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32220_e29454;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32220_e29454_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32220_e29454_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32220_e29454_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32220_e29454_d_n9;

        let assign32230_e29457: f64 = (-50.0);
        let assign32230_e29458: f64 = if locals.var_fn382_calc_iq__exparg0 < assign32230_e29457 { 1.0 } else { 0.0 };
        locals.var_guard405 = assign32230_e29458;

        let (assign32240_e29465, assign32240_e29465_d_n4, assign32240_e29465_d_n5, assign32240_e29465_d_n8, assign32240_e29465_d_n9,) = {
    if ((locals.var_guard404 == 0.0) && (locals.var_guard405 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32240_e29465;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32240_e29465_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32240_e29465_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32240_e29465_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32240_e29465_d_n9;

        let (assign32250_e29478, assign32250_e29478_d_n4, assign32250_e29478_d_n5, assign32250_e29478_d_n8, assign32250_e29478_d_n9,) = {
    if ((locals.var_guard404 == 0.0) && (locals.var_guard405 == 0.0)) {
        let assign32250_e29474: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign32250_e29475: f64 = (1.0 + assign32250_e29474);
        let assign32250_e29476: f64 = (1.0 / assign32250_e29475);
        (assign32250_e29476, (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign32250_e29475 * assign32250_e29475))),)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32250_e29478;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32250_e29478_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32250_e29478_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32250_e29478_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32250_e29478_d_n9;

        let assign32260_e29481: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdx0);
        let assign32260_e29485: f64 = (p.p51 * 0.1);
        let assign32260_e29487: f64 = (assign32260_e29485 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32260_e29489: f64 = (assign32260_e29487 * locals.var_fn382_calc_iq__ffd0);
        let assign32260_e29490: f64 = (locals.var_fn382_calc_iq__vtof - assign32260_e29489);
        let assign32260_e29491: f64 = (assign32260_e29481 - assign32260_e29490);
        let assign32260_e29493: f64 = (assign32260_e29491 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0 = assign32260_e29493;
        locals.var_fn382_calc_iq__etad0_dn4 = (((((-locals.var_fn382_calc_iq__vdx0_dn4) - (locals.var_fn382_calc_iq__vtof_dn4 - (((assign32260_e29485 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffd0) + (assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32260_e29491 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__etad0_dn5 = (((-locals.var_fn382_calc_iq__vdx0_dn5) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0_dn8 = (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vdx0_dn8) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0_dn9 = (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdx0_dn9) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);

        let assign32270_e29496: f64 = if locals.var_fn382_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign32270_e29496;

        let (assign32280_e29502, assign32280_e29502_d_n4, assign32280_e29502_d_n5, assign32280_e29502_d_n8, assign32280_e29502_d_n9,) = {
    if (locals.var_guard406 != 0.0) {
        let assign32280_e29500: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0);
        (assign32280_e29500, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__etad0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32280_e29502;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32280_e29502_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32280_e29502_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32280_e29502_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32280_e29502_d_n9;

        let assign32290_e29505: f64 = (-50.0);
        let assign32290_e29506: f64 = if locals.var_fn382_calc_iq__etad0 < assign32290_e29505 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign32290_e29506;

        let (assign32300_e29516, assign32300_e29516_d_n4, assign32300_e29516_d_n5, assign32300_e29516_d_n8, assign32300_e29516_d_n9,) = {
    if ((locals.var_guard406 == 0.0) && (locals.var_guard407 != 0.0)) {
        let assign32300_e29513: f64 = (locals.var_fn382_calc_iq__etad0).exp();
        let assign32300_e29514: f64 = (locals.var_fn382_calc_iq__qref0 * assign32300_e29513);
        (assign32300_e29514, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32300_e29513) + (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32300_e29516;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32300_e29516_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32300_e29516_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32300_e29516_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32300_e29516_d_n9;

        let (assign32310_e29530, assign32310_e29530_d_n4, assign32310_e29530_d_n5, assign32310_e29530_d_n8, assign32310_e29530_d_n9,) = {
    if ((locals.var_guard406 == 0.0) && (locals.var_guard407 == 0.0)) {
        let assign32310_e29525: f64 = (locals.var_fn382_calc_iq__etad0).exp();
        let assign32310_e29526: f64 = (1.0 + assign32310_e29525);
        let assign32310_e29527: f64 = (assign32310_e29526).ln();
        let assign32310_e29528: f64 = (locals.var_fn382_calc_iq__qref0 * assign32310_e29527);
        (assign32310_e29528, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32310_e29527) + (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn4) / assign32310_e29526))), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn5) / assign32310_e29526)), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn8) / assign32310_e29526)), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn9) / assign32310_e29526)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32310_e29530;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32310_e29530_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32310_e29530_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32310_e29530_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32310_e29530_d_n9;

        let assign32320_e29533: f64 = (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0);
        let assign32320_e29535: f64 = (assign32320_e29533 + 1e-38);
        locals.var_fn382_calc_iq__qs2 = assign32320_e29535;
        locals.var_fn382_calc_iq__qs2_dn4 = ((locals.var_fn382_calc_iq__qinvs0_dn4 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn4));
        locals.var_fn382_calc_iq__qs2_dn5 = ((locals.var_fn382_calc_iq__qinvs0_dn5 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn5));
        locals.var_fn382_calc_iq__qs2_dn8 = ((locals.var_fn382_calc_iq__qinvs0_dn8 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn8));
        locals.var_fn382_calc_iq__qs2_dn9 = ((locals.var_fn382_calc_iq__qinvs0_dn9 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn9));

    }

    pub(super) fn stamp_transient_block_66(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let assign32330_e29538: f64 = (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0);
        let assign32330_e29540: f64 = (assign32330_e29538 + 1e-57);
        locals.var_fn382_calc_iq__qs3 = assign32330_e29540;
        locals.var_fn382_calc_iq__qs3_dn4 = ((locals.var_fn382_calc_iq__qs2_dn4 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn4));
        locals.var_fn382_calc_iq__qs3_dn5 = ((locals.var_fn382_calc_iq__qs2_dn5 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn5));
        locals.var_fn382_calc_iq__qs3_dn8 = ((locals.var_fn382_calc_iq__qs2_dn8 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn8));
        locals.var_fn382_calc_iq__qs3_dn9 = ((locals.var_fn382_calc_iq__qs2_dn9 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn9));

        let assign32340_e29543: f64 = (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0);
        let assign32340_e29545: f64 = (assign32340_e29543 + 1e-38);
        locals.var_fn382_calc_iq__qd2 = assign32340_e29545;
        locals.var_fn382_calc_iq__qd2_dn4 = ((locals.var_fn382_calc_iq__qinvd0_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qd2_dn5 = ((locals.var_fn382_calc_iq__qinvd0_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qd2_dn8 = ((locals.var_fn382_calc_iq__qinvd0_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qd2_dn9 = ((locals.var_fn382_calc_iq__qinvd0_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn9));

        let assign32350_e29548: f64 = (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0);
        let assign32350_e29550: f64 = (assign32350_e29548 + 1e-57);
        locals.var_fn382_calc_iq__qd3 = assign32350_e29550;
        locals.var_fn382_calc_iq__qd3_dn4 = ((locals.var_fn382_calc_iq__qd2_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qd3_dn5 = ((locals.var_fn382_calc_iq__qd2_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qd3_dn8 = ((locals.var_fn382_calc_iq__qd2_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qd3_dn9 = ((locals.var_fn382_calc_iq__qd2_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn9));

        let assign32360_e29553: f64 = (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0);
        let assign32360_e29555: f64 = (assign32360_e29553 + 1e-38);
        locals.var_fn382_calc_iq__qsqd = assign32360_e29555;
        locals.var_fn382_calc_iq__qsqd_dn4 = ((locals.var_fn382_calc_iq__qinvs0_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qsqd_dn5 = ((locals.var_fn382_calc_iq__qinvs0_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qsqd_dn8 = ((locals.var_fn382_calc_iq__qinvs0_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qsqd_dn9 = ((locals.var_fn382_calc_iq__qinvs0_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn9));

        let assign32370_e29558: f64 = (2.0 / 3.0);
        let assign32370_e29561: f64 = (locals.var_fn382_calc_iq__qs2 + locals.var_fn382_calc_iq__qd2);
        let assign32370_e29563: f64 = (assign32370_e29561 + locals.var_fn382_calc_iq__qsqd);
        let assign32370_e29564: f64 = (assign32370_e29558 * assign32370_e29563);
        let assign32370_e29567: f64 = (locals.var_fn382_calc_iq__qinvs0 + locals.var_fn382_calc_iq__qinvd0);
        let assign32370_e29569: f64 = (assign32370_e29567 + 2e-19);
        let assign32370_e29570: f64 = (assign32370_e29564 / assign32370_e29569);
        locals.var_fn382_calc_iq__qinvdd = assign32370_e29570;
        locals.var_fn382_calc_iq__qinvdd_dn4 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn4 + locals.var_fn382_calc_iq__qd2_dn4) + locals.var_fn382_calc_iq__qsqd_dn4)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn4 + locals.var_fn382_calc_iq__qinvd0_dn4))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn5 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn5 + locals.var_fn382_calc_iq__qd2_dn5) + locals.var_fn382_calc_iq__qsqd_dn5)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn5 + locals.var_fn382_calc_iq__qinvd0_dn5))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn8 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn8 + locals.var_fn382_calc_iq__qd2_dn8) + locals.var_fn382_calc_iq__qsqd_dn8)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn8 + locals.var_fn382_calc_iq__qinvd0_dn8))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn9 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn9 + locals.var_fn382_calc_iq__qd2_dn9) + locals.var_fn382_calc_iq__qsqd_dn9)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn9 + locals.var_fn382_calc_iq__qinvd0_dn9))) / (assign32370_e29569 * assign32370_e29569));

        let assign32380_e29574: f64 = (2.0 * locals.var_fn382_calc_iq__qs3);
        let assign32380_e29577: f64 = (3.0 * locals.var_fn382_calc_iq__qd3);
        let assign32380_e29578: f64 = (assign32380_e29574 + assign32380_e29577);
        let assign32380_e29581: f64 = (4.0 * locals.var_fn382_calc_iq__qs2);
        let assign32380_e29583: f64 = (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0);
        let assign32380_e29584: f64 = (assign32380_e29578 + assign32380_e29583);
        let assign32380_e29587: f64 = (6.0 * locals.var_fn382_calc_iq__qd2);
        let assign32380_e29589: f64 = (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0);
        let assign32380_e29590: f64 = (assign32380_e29584 + assign32380_e29589);
        let assign32380_e29591: f64 = (2.0 * assign32380_e29590);
        let assign32380_e29595: f64 = (locals.var_fn382_calc_iq__qs2 + locals.var_fn382_calc_iq__qd2);
        let assign32380_e29598: f64 = (2.0 * locals.var_fn382_calc_iq__qsqd);
        let assign32380_e29599: f64 = (assign32380_e29595 + assign32380_e29598);
        let assign32380_e29600: f64 = (15.0 * assign32380_e29599);
        let assign32380_e29601: f64 = (assign32380_e29591 / assign32380_e29600);
        locals.var_fn382_calc_iq__qd1 = assign32380_e29601;
        locals.var_fn382_calc_iq__qd1_dn4 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn4) + (3.0 * locals.var_fn382_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn4) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn4) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn4)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn4 + locals.var_fn382_calc_iq__qd2_dn4) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn4))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn5 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn5) + (3.0 * locals.var_fn382_calc_iq__qd3_dn5)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn5) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn5))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn5) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn5)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn5 + locals.var_fn382_calc_iq__qd2_dn5) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn5))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn8 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn8) + (3.0 * locals.var_fn382_calc_iq__qd3_dn8)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn8) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn8))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn8) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn8)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn8 + locals.var_fn382_calc_iq__qd2_dn8) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn8))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn9 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn9) + (3.0 * locals.var_fn382_calc_iq__qd3_dn9)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn9) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn9))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn9) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn9)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn9 + locals.var_fn382_calc_iq__qd2_dn9) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn9))))) / (assign32380_e29600 * assign32380_e29600));

        let assign32390_e29604: f64 = (locals.var_fn382_calc_iq__qinvdd - locals.var_fn382_calc_iq__qd1);
        locals.var_fn382_calc_iq__qs = assign32390_e29604;
        locals.var_fn382_calc_iq__qs_dn4 = (locals.var_fn382_calc_iq__qinvdd_dn4 - locals.var_fn382_calc_iq__qd1_dn4);
        locals.var_fn382_calc_iq__qs_dn5 = (locals.var_fn382_calc_iq__qinvdd_dn5 - locals.var_fn382_calc_iq__qd1_dn5);
        locals.var_fn382_calc_iq__qs_dn8 = (locals.var_fn382_calc_iq__qinvdd_dn8 - locals.var_fn382_calc_iq__qd1_dn8);
        locals.var_fn382_calc_iq__qs_dn9 = (locals.var_fn382_calc_iq__qinvdd_dn9 - locals.var_fn382_calc_iq__qd1_dn9);

        locals.var_fn382_calc_iq__qd = locals.var_fn382_calc_iq__qd1;
        locals.var_fn382_calc_iq__qd_dn4 = locals.var_fn382_calc_iq__qd1_dn4;
        locals.var_fn382_calc_iq__qd_dn5 = locals.var_fn382_calc_iq__qd1_dn5;
        locals.var_fn382_calc_iq__qd_dn8 = locals.var_fn382_calc_iq__qd1_dn8;
        locals.var_fn382_calc_iq__qd_dn9 = locals.var_fn382_calc_iq__qd1_dn9;

        let assign32410_e29608: f64 = (locals.var_fn382_calc_iq__w * locals.var_fn382_calc_iq__ngf);
        let assign32410_e29610: f64 = (assign32410_e29608 * locals.var_fn382_calc_iq__lin);
        let assign32410_e29612: f64 = (assign32410_e29610 * locals.var_fn382_calc_iq__type);
        let assign32410_e29614: f64 = (assign32410_e29612 * locals.var_fn382_calc_iq__qs);
        let assign32410_e29616: f64 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout = assign32410_e29616;
        locals.var_fn382_calc_iq__qgsout_dn4 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn4) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn5 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn5) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn8 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn8) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn9 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn9) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn22 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__qgsout_dn23 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__qgsout_dn25 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__qgsout_dn26 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn26);

        let assign32420_e29619: f64 = (locals.var_fn382_calc_iq__w * locals.var_fn382_calc_iq__ngf);
        let assign32420_e29621: f64 = (assign32420_e29619 * locals.var_fn382_calc_iq__lin);
        let assign32420_e29623: f64 = (assign32420_e29621 * locals.var_fn382_calc_iq__type);
        let assign32420_e29625: f64 = (assign32420_e29623 * locals.var_fn382_calc_iq__qd);
        let assign32420_e29627: f64 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout = assign32420_e29627;
        locals.var_fn382_calc_iq__qgdout_dn4 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn4) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn5 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn5) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn8 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn8) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn9 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn9) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn22 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__qgdout_dn23 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__qgdout_dn25 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__qgdout_dn26 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn26);

        let assign32430_e29630: f64 = if locals.var_fn382_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign32430_e29630;

        let (assign32440_e29644, assign32440_e29644_d_n4,) = {
    if (locals.var_guard408 != 0.0) {
        let assign32440_e29636: f64 = (p.p51 * 0.5);
        let assign32440_e29638: f64 = (assign32440_e29636 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32440_e29639: f64 = (locals.var_fn382_calc_iq__vtof - assign32440_e29638);
        let assign32440_e29640: f64 = (locals.var_fn382_calc_iq__vcin - assign32440_e29639);
        let assign32440_e29642: f64 = (assign32440_e29640 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32440_e29642, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32440_e29636 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32440_e29640 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (locals.var_fn382_calc_iq__etac, locals.var_fn382_calc_iq__etac_dn4,)
    }
};
        locals.var_fn382_calc_iq__etac = assign32440_e29644;
        locals.var_fn382_calc_iq__etac_dn4 = assign32440_e29644_d_n4;

        let assign32450_e29647: f64 = if locals.var_fn382_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign32450_e29647;

        let (assign32460_e29653, assign32460_e29653_d_n4, assign32460_e29653_d_n5, assign32460_e29653_d_n8, assign32460_e29653_d_n9,) = {
    if ((locals.var_guard408 != 0.0) && (locals.var_guard409 != 0.0)) {
        (locals.var_fn382_calc_iq__etac, locals.var_fn382_calc_iq__etac_dn4, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32460_e29653;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32460_e29653_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32460_e29653_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32460_e29653_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32460_e29653_d_n9;

        let assign32470_e29656: f64 = (-50.0);
        let assign32470_e29657: f64 = if locals.var_fn382_calc_iq__etac < assign32470_e29656 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign32470_e29657;

        let (assign32480_e29667, assign32480_e29667_d_n4, assign32480_e29667_d_n5, assign32480_e29667_d_n8, assign32480_e29667_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        let assign32480_e29665: f64 = (locals.var_fn382_calc_iq__etac).exp();
        (assign32480_e29665, (assign32480_e29665 * locals.var_fn382_calc_iq__etac_dn4), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32480_e29667;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32480_e29667_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32480_e29667_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32480_e29667_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32480_e29667_d_n9;

        let (assign32490_e29681, assign32490_e29681_d_n4, assign32490_e29681_d_n5, assign32490_e29681_d_n8, assign32490_e29681_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
        let assign32490_e29677: f64 = (locals.var_fn382_calc_iq__etac).exp();
        let assign32490_e29678: f64 = (1.0 + assign32490_e29677);
        let assign32490_e29679: f64 = (assign32490_e29678).ln();
        (assign32490_e29679, ((assign32490_e29677 * locals.var_fn382_calc_iq__etac_dn4) / assign32490_e29678), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32490_e29681;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32490_e29681_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32490_e29681_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32490_e29681_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32490_e29681_d_n9;

        let (assign32510_e29711, assign32510_e29711_d_n4,) = {
    if (locals.var_guard408 != 0.0) {
        let assign32510_e29703: f64 = (p.p51 * 0.5);
        let assign32510_e29705: f64 = (assign32510_e29703 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32510_e29706: f64 = (locals.var_fn382_calc_iq__vtof - assign32510_e29705);
        let assign32510_e29707: f64 = (locals.var_fn382_calc_iq__vbin - assign32510_e29706);
        let assign32510_e29709: f64 = (assign32510_e29707 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32510_e29709, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32510_e29703 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32510_e29707 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (locals.var_fn382_calc_iq__etab, locals.var_fn382_calc_iq__etab_dn4,)
    }
};
        locals.var_fn382_calc_iq__etab = assign32510_e29711;
        locals.var_fn382_calc_iq__etab_dn4 = assign32510_e29711_d_n4;

        let assign32520_e29714: f64 = if locals.var_fn382_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign32520_e29714;

        let (assign32530_e29720, assign32530_e29720_d_n4, assign32530_e29720_d_n5, assign32530_e29720_d_n8, assign32530_e29720_d_n9,) = {
    if ((locals.var_guard408 != 0.0) && (locals.var_guard411 != 0.0)) {
        (locals.var_fn382_calc_iq__etab, locals.var_fn382_calc_iq__etab_dn4, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32530_e29720;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32530_e29720_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32530_e29720_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32530_e29720_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32530_e29720_d_n9;

        let assign32540_e29723: f64 = (-50.0);
        let assign32540_e29724: f64 = if locals.var_fn382_calc_iq__etab < assign32540_e29723 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign32540_e29724;

        let (assign32550_e29734, assign32550_e29734_d_n4, assign32550_e29734_d_n5, assign32550_e29734_d_n8, assign32550_e29734_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        let assign32550_e29732: f64 = (locals.var_fn382_calc_iq__etab).exp();
        (assign32550_e29732, (assign32550_e29732 * locals.var_fn382_calc_iq__etab_dn4), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32550_e29734;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32550_e29734_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32550_e29734_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32550_e29734_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32550_e29734_d_n9;

        let (assign32560_e29748, assign32560_e29748_d_n4, assign32560_e29748_d_n5, assign32560_e29748_d_n8, assign32560_e29748_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
        let assign32560_e29744: f64 = (locals.var_fn382_calc_iq__etab).exp();
        let assign32560_e29745: f64 = (1.0 + assign32560_e29744);
        let assign32560_e29746: f64 = (assign32560_e29745).ln();
        (assign32560_e29746, ((assign32560_e29744 * locals.var_fn382_calc_iq__etab_dn4) / assign32560_e29745), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32560_e29748;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32560_e29748_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32560_e29748_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32560_e29748_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32560_e29748_d_n9;

        let assign32600_e29777: f64 = if locals.var_fn382_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign32600_e29777;

        let (assign32610_e29791, assign32610_e29791_d_n4, assign32610_e29791_d_n8, assign32610_e29791_d_n9,) = {
    if (locals.var_guard413 != 0.0) {
        let assign32610_e29783: f64 = (p.p51 * 0.5);
        let assign32610_e29785: f64 = (assign32610_e29783 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32610_e29786: f64 = (locals.var_fn382_calc_iq__vtof - assign32610_e29785);
        let assign32610_e29787: f64 = (locals.var_fn382_calc_iq__vgsin - assign32610_e29786);
        let assign32610_e29789: f64 = (assign32610_e29787 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32610_e29789, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32610_e29783 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32610_e29787 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)), (locals.var_fn382_calc_iq__vgsin_dn8 / locals.var_fn382_calc_iq__two_n_phit0), (locals.var_fn382_calc_iq__vgsin_dn9 / locals.var_fn382_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn382_calc_iq__etags, locals.var_fn382_calc_iq__etags_dn4, locals.var_fn382_calc_iq__etags_dn8, locals.var_fn382_calc_iq__etags_dn9,)
    }
};
        locals.var_fn382_calc_iq__etags = assign32610_e29791;
        locals.var_fn382_calc_iq__etags_dn4 = assign32610_e29791_d_n4;
        locals.var_fn382_calc_iq__etags_dn8 = assign32610_e29791_d_n8;
        locals.var_fn382_calc_iq__etags_dn9 = assign32610_e29791_d_n9;

        let assign32620_e29794: f64 = if locals.var_fn382_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign32620_e29794;

        let (assign32630_e29800, assign32630_e29800_d_n4, assign32630_e29800_d_n5, assign32630_e29800_d_n8, assign32630_e29800_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_fn382_calc_iq__etags, locals.var_fn382_calc_iq__etags_dn4, 0.0, locals.var_fn382_calc_iq__etags_dn8, locals.var_fn382_calc_iq__etags_dn9,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32630_e29800;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32630_e29800_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32630_e29800_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32630_e29800_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32630_e29800_d_n9;

        let assign32640_e29803: f64 = (-50.0);
        let assign32640_e29804: f64 = if locals.var_fn382_calc_iq__etags < assign32640_e29803 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign32640_e29804;

        let (assign32650_e29814, assign32650_e29814_d_n4, assign32650_e29814_d_n5, assign32650_e29814_d_n8, assign32650_e29814_d_n9,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign32650_e29812: f64 = (locals.var_fn382_calc_iq__etags).exp();
        (assign32650_e29812, (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn4), 0.0, (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn8), (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn9),)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32650_e29814;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32650_e29814_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32650_e29814_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32650_e29814_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32650_e29814_d_n9;

        let (assign32660_e29828, assign32660_e29828_d_n4, assign32660_e29828_d_n5, assign32660_e29828_d_n8, assign32660_e29828_d_n9,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) && (locals.var_guard415 == 0.0)) {
        let assign32660_e29824: f64 = (locals.var_fn382_calc_iq__etags).exp();
        let assign32660_e29825: f64 = (1.0 + assign32660_e29824);
        let assign32660_e29826: f64 = (assign32660_e29825).ln();
        (assign32660_e29826, ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn4) / assign32660_e29825), 0.0, ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn8) / assign32660_e29825), ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn9) / assign32660_e29825),)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32660_e29828;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32660_e29828_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32660_e29828_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32660_e29828_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32660_e29828_d_n9;

        locals.var_fn382_calc_iq__return = locals.var_fn382_calc_iq__idsout;
        locals.var_fn382_calc_iq__return_dn4 = locals.var_fn382_calc_iq__idsout_dn4;
        locals.var_fn382_calc_iq__return_dn5 = locals.var_fn382_calc_iq__idsout_dn5;
        locals.var_fn382_calc_iq__return_dn8 = locals.var_fn382_calc_iq__idsout_dn8;
        locals.var_fn382_calc_iq__return_dn9 = locals.var_fn382_calc_iq__idsout_dn9;
        locals.var_fn382_calc_iq__return_dn22 = locals.var_fn382_calc_iq__idsout_dn22;
        locals.var_fn382_calc_iq__return_dn23 = locals.var_fn382_calc_iq__idsout_dn23;
        locals.var_fn382_calc_iq__return_dn25 = locals.var_fn382_calc_iq__idsout_dn25;
        locals.var_fn382_calc_iq__return_dn26 = locals.var_fn382_calc_iq__idsout_dn26;

        locals.var_ids = locals.var_fn382_calc_iq__idsout;
        locals.var_ids_dn4 = locals.var_fn382_calc_iq__idsout_dn4;
        locals.var_ids_dn5 = locals.var_fn382_calc_iq__idsout_dn5;
        locals.var_ids_dn8 = locals.var_fn382_calc_iq__idsout_dn8;
        locals.var_ids_dn9 = locals.var_fn382_calc_iq__idsout_dn9;
        locals.var_ids_dn22 = locals.var_fn382_calc_iq__idsout_dn22;
        locals.var_ids_dn23 = locals.var_fn382_calc_iq__idsout_dn23;
        locals.var_ids_dn25 = locals.var_fn382_calc_iq__idsout_dn25;
        locals.var_ids_dn26 = locals.var_fn382_calc_iq__idsout_dn26;

        locals.var_qgs = locals.var_fn382_calc_iq__qgsout;
        locals.var_qgs_dn4 = locals.var_fn382_calc_iq__qgsout_dn4;
        locals.var_qgs_dn5 = locals.var_fn382_calc_iq__qgsout_dn5;
        locals.var_qgs_dn8 = locals.var_fn382_calc_iq__qgsout_dn8;
        locals.var_qgs_dn9 = locals.var_fn382_calc_iq__qgsout_dn9;
        locals.var_qgs_dn22 = locals.var_fn382_calc_iq__qgsout_dn22;
        locals.var_qgs_dn23 = locals.var_fn382_calc_iq__qgsout_dn23;
        locals.var_qgs_dn25 = locals.var_fn382_calc_iq__qgsout_dn25;
        locals.var_qgs_dn26 = locals.var_fn382_calc_iq__qgsout_dn26;

        locals.var_qgd = locals.var_fn382_calc_iq__qgdout;
        locals.var_qgd_dn4 = locals.var_fn382_calc_iq__qgdout_dn4;
        locals.var_qgd_dn5 = locals.var_fn382_calc_iq__qgdout_dn5;
        locals.var_qgd_dn8 = locals.var_fn382_calc_iq__qgdout_dn8;
        locals.var_qgd_dn9 = locals.var_fn382_calc_iq__qgdout_dn9;
        locals.var_qgd_dn22 = locals.var_fn382_calc_iq__qgdout_dn22;
        locals.var_qgd_dn23 = locals.var_fn382_calc_iq__qgdout_dn23;
        locals.var_qgd_dn25 = locals.var_fn382_calc_iq__qgdout_dn25;
        locals.var_qgd_dn26 = locals.var_fn382_calc_iq__qgdout_dn26;

        locals.var_ids = locals.var_fn382_calc_iq__return;
        locals.var_ids_dn4 = locals.var_fn382_calc_iq__return_dn4;
        locals.var_ids_dn5 = locals.var_fn382_calc_iq__return_dn5;
        locals.var_ids_dn8 = locals.var_fn382_calc_iq__return_dn8;
        locals.var_ids_dn9 = locals.var_fn382_calc_iq__return_dn9;
        locals.var_ids_dn22 = locals.var_fn382_calc_iq__return_dn22;
        locals.var_ids_dn23 = locals.var_fn382_calc_iq__return_dn23;
        locals.var_ids_dn25 = locals.var_fn382_calc_iq__return_dn25;
        locals.var_ids_dn26 = locals.var_fn382_calc_iq__return_dn26;

        let assign32800_e29863: f64 = if p.p322 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign32800_e29863;

        locals.var_vsch = 0.0;
        locals.var_vsch_dn7 = 0.0;
        locals.var_vsch_dn8 = 0.0;

        locals.var_qsch = 0.0;
        locals.var_qsch_dn7 = 0.0;
        locals.var_qsch_dn8 = 0.0;

        locals.var_qsch0 = 0.0;

        locals.var_qsch1 = 0.0;
        locals.var_qsch1_dn7 = 0.0;
        locals.var_qsch1_dn8 = 0.0;

        locals.var_qsch2 = 0.0;
        locals.var_qsch2_dn7 = 0.0;
        locals.var_qsch2_dn8 = 0.0;

        locals.var_qsch3 = 0.0;
        locals.var_qsch3_dn7 = 0.0;
        locals.var_qsch3_dn8 = 0.0;

        locals.var_qsch4 = 0.0;
        locals.var_qsch4_dn7 = 0.0;
        locals.var_qsch4_dn8 = 0.0;

        locals.var_qsch5 = 0.0;
        locals.var_qsch5_dn7 = 0.0;
        locals.var_qsch5_dn8 = 0.0;

        locals.var_vschfc1 = 0.0;
        locals.var_vschfc1_dn7 = 0.0;
        locals.var_vschfc1_dn8 = 0.0;

        locals.var_vschfc2 = 0.0;
        locals.var_vschfc2_dn7 = 0.0;
        locals.var_vschfc2_dn8 = 0.0;

        locals.var_vschfc3 = 0.0;
        locals.var_vschfc3_dn7 = 0.0;
        locals.var_vschfc3_dn8 = 0.0;

        locals.var_vschfc4 = 0.0;
        locals.var_vschfc4_dn7 = 0.0;
        locals.var_vschfc4_dn8 = 0.0;

        locals.var_vschfc5 = 0.0;
        locals.var_vschfc5_dn7 = 0.0;
        locals.var_vschfc5_dn8 = 0.0;

        let assign41530_e39902: f64 = if p.p291 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign41530_e39902;

        let (assign41540_e39908, assign41540_e39908_d_n7, assign41540_e39908_d_n8,) = {
    if (locals.var_guard461 != 0.0) {
        let assign41540_e39906: f64 = (p.p6 * (nv8 - nv7));
        (assign41540_e39906, (-p.p6), p.p6,)
    } else {
        (locals.var_vsch, locals.var_vsch_dn7, locals.var_vsch_dn8,)
    }
};
        locals.var_vsch = assign41540_e39908;
        locals.var_vsch_dn7 = assign41540_e39908_d_n7;
        locals.var_vsch_dn8 = assign41540_e39908_d_n8;

        let assign43620_e42207: f64 = (p.p308 * p.p306);
        let assign43620_e42208: f64 = if locals.var_vsch <= assign43620_e42207 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign43620_e42208;

        let (assign43630_e42237, assign43630_e42237_d_n7, assign43630_e42237_d_n8,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 != 0.0)) {
        let assign43630_e42214: f64 = (p.p6 * 2.0);
        let assign43630_e42216: f64 = (assign43630_e42214 * p.p307);
        let assign43630_e42218: f64 = (assign43630_e42216 * p.p0);
        let assign43630_e42221: f64 = (1.0 - p.p311);
        let assign43630_e42222: f64 = (assign43630_e42218 * assign43630_e42221);
        let assign43630_e42224: f64 = (assign43630_e42222 * p.p2);
        let assign43630_e42226: f64 = (assign43630_e42224 * p.p306);
        let assign43630_e42231: f64 = (locals.var_vsch / p.p306);
        let assign43630_e42232: f64 = (1.0 - assign43630_e42231);
        let assign43630_e42233: f64 = (assign43630_e42232).sqrt();
        let assign43630_e42234: f64 = (1.0 - assign43630_e42233);
        let assign43630_e42235: f64 = (assign43630_e42226 * assign43630_e42234);
        (assign43630_e42235, (assign43630_e42226 * (-((-(locals.var_vsch_dn7 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(locals.var_vsch_dn8 / p.p306)) / (2.0 * assign43630_e42233)))),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn7, locals.var_qsch_dn8,)
    }
};
        locals.var_qsch = assign43630_e42237;
        locals.var_qsch_dn7 = assign43630_e42237_d_n7;
        locals.var_qsch_dn8 = assign43630_e42237_d_n8;

        let (assign43640_e42249,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) {
        let assign43640_e42245: f64 = (1.0 - p.p308);
        let assign43640_e42246: f64 = (assign43640_e42245).sqrt();
        let assign43640_e42247: f64 = (1.0 - assign43640_e42246);
        (assign43640_e42247,)
    } else {
        (locals.var_qsch0,)
    }
};
        locals.var_qsch0 = assign43640_e42249;

        let assign43650_e42252: f64 = if p.p309 >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign43650_e42252;

        let (assign43660_e42270,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43660_e42262: f64 = (2.0 * p.p306);
        let assign43660_e42265: f64 = (1.0 - p.p308);
        let assign43660_e42266: f64 = (assign43660_e42265).sqrt();
        let assign43660_e42267: f64 = (assign43660_e42262 * assign43660_e42266);
        let assign43660_e42268: f64 = (1.0 / assign43660_e42267);
        (assign43660_e42268,)
    } else {
        (locals.var_qsch1c,)
    }
};
        locals.var_qsch1c = assign43660_e42270;

        let (assign43670_e42283, assign43670_e42283_d_n7, assign43670_e42283_d_n8,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43670_e42280: f64 = (p.p308 * p.p306);
        let assign43670_e42281: f64 = (locals.var_vsch - assign43670_e42280);
        (assign43670_e42281, locals.var_vsch_dn7, locals.var_vsch_dn8,)
    } else {
        (locals.var_vschfc1, locals.var_vschfc1_dn7, locals.var_vschfc1_dn8,)
    }
};
        locals.var_vschfc1 = assign43670_e42283;
        locals.var_vschfc1_dn7 = assign43670_e42283_d_n7;
        locals.var_vschfc1_dn8 = assign43670_e42283_d_n8;

        let (assign43680_e42294, assign43680_e42294_d_n7, assign43680_e42294_d_n8,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43680_e42292: f64 = (locals.var_qsch1c * locals.var_vschfc1);
        (assign43680_e42292, (locals.var_qsch1c * locals.var_vschfc1_dn7), (locals.var_qsch1c * locals.var_vschfc1_dn8),)
    } else {
        (locals.var_qsch1, locals.var_qsch1_dn7, locals.var_qsch1_dn8,)
    }
};
        locals.var_qsch1 = assign43680_e42294;
        locals.var_qsch1_dn7 = assign43680_e42294_d_n7;
        locals.var_qsch1_dn8 = assign43680_e42294_d_n8;

    }

    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign43690_e42297: f64 = if p.p309 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign43690_e42297;

        let (assign43700_e42316,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43700_e42309: f64 = (4.0 * p.p306);
        let assign43700_e42312: f64 = (1.0 - p.p308);
        let assign43700_e42313: f64 = (assign43700_e42309 * assign43700_e42312);
        let assign43700_e42314: f64 = (locals.var_qsch1c / assign43700_e42313);
        (assign43700_e42314,)
    } else {
        (locals.var_qsch2c,)
    }
};
        locals.var_qsch2c = assign43700_e42316;

        let (assign43710_e42329, assign43710_e42329_d_n7, assign43710_e42329_d_n8,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43710_e42327: f64 = (locals.var_vschfc1 * locals.var_vschfc1);
        (assign43710_e42327, ((locals.var_vschfc1_dn7 * locals.var_vschfc1) + (locals.var_vschfc1 * locals.var_vschfc1_dn7)), ((locals.var_vschfc1_dn8 * locals.var_vschfc1) + (locals.var_vschfc1 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc2, locals.var_vschfc2_dn7, locals.var_vschfc2_dn8,)
    }
};
        locals.var_vschfc2 = assign43710_e42329;
        locals.var_vschfc2_dn7 = assign43710_e42329_d_n7;
        locals.var_vschfc2_dn8 = assign43710_e42329_d_n8;

        let (assign43720_e42342, assign43720_e42342_d_n7, assign43720_e42342_d_n8,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43720_e42340: f64 = (locals.var_qsch2c * locals.var_vschfc2);
        (assign43720_e42340, (locals.var_qsch2c * locals.var_vschfc2_dn7), (locals.var_qsch2c * locals.var_vschfc2_dn8),)
    } else {
        (locals.var_qsch2, locals.var_qsch2_dn7, locals.var_qsch2_dn8,)
    }
};
        locals.var_qsch2 = assign43720_e42342;
        locals.var_qsch2_dn7 = assign43720_e42342_d_n7;
        locals.var_qsch2_dn8 = assign43720_e42342_d_n8;

        let assign43730_e42345: f64 = if p.p309 >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign43730_e42345;

        let (assign43740_e42366,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43740_e42359: f64 = (2.0 * p.p306);
        let assign43740_e42362: f64 = (1.0 - p.p308);
        let assign43740_e42363: f64 = (assign43740_e42359 * assign43740_e42362);
        let assign43740_e42364: f64 = (locals.var_qsch2c / assign43740_e42363);
        (assign43740_e42364,)
    } else {
        (locals.var_qsch3c,)
    }
};
        locals.var_qsch3c = assign43740_e42366;

        let (assign43750_e42381, assign43750_e42381_d_n7, assign43750_e42381_d_n8,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43750_e42379: f64 = (locals.var_vschfc2 * locals.var_vschfc1);
        (assign43750_e42379, ((locals.var_vschfc2_dn7 * locals.var_vschfc1) + (locals.var_vschfc2 * locals.var_vschfc1_dn7)), ((locals.var_vschfc2_dn8 * locals.var_vschfc1) + (locals.var_vschfc2 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc3, locals.var_vschfc3_dn7, locals.var_vschfc3_dn8,)
    }
};
        locals.var_vschfc3 = assign43750_e42381;
        locals.var_vschfc3_dn7 = assign43750_e42381_d_n7;
        locals.var_vschfc3_dn8 = assign43750_e42381_d_n8;

        let (assign43760_e42396, assign43760_e42396_d_n7, assign43760_e42396_d_n8,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43760_e42394: f64 = (locals.var_qsch3c * locals.var_vschfc3);
        (assign43760_e42394, (locals.var_qsch3c * locals.var_vschfc3_dn7), (locals.var_qsch3c * locals.var_vschfc3_dn8),)
    } else {
        (locals.var_qsch3, locals.var_qsch3_dn7, locals.var_qsch3_dn8,)
    }
};
        locals.var_qsch3 = assign43760_e42396;
        locals.var_qsch3_dn7 = assign43760_e42396_d_n7;
        locals.var_qsch3_dn8 = assign43760_e42396_d_n8;

        let assign43770_e42399: f64 = if p.p309 >= 4.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign43770_e42399;

        let (assign43780_e42424,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43780_e42414: f64 = (5.0 * locals.var_qsch3c);
        let assign43780_e42417: f64 = (8.0 * p.p306);
        let assign43780_e42420: f64 = (1.0 - p.p308);
        let assign43780_e42421: f64 = (assign43780_e42417 * assign43780_e42420);
        let assign43780_e42422: f64 = (assign43780_e42414 / assign43780_e42421);
        (assign43780_e42422,)
    } else {
        (locals.var_qsch4c,)
    }
};
        locals.var_qsch4c = assign43780_e42424;

        let (assign43790_e42441, assign43790_e42441_d_n7, assign43790_e42441_d_n8,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43790_e42439: f64 = (locals.var_vschfc3 * locals.var_vschfc1);
        (assign43790_e42439, ((locals.var_vschfc3_dn7 * locals.var_vschfc1) + (locals.var_vschfc3 * locals.var_vschfc1_dn7)), ((locals.var_vschfc3_dn8 * locals.var_vschfc1) + (locals.var_vschfc3 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc4, locals.var_vschfc4_dn7, locals.var_vschfc4_dn8,)
    }
};
        locals.var_vschfc4 = assign43790_e42441;
        locals.var_vschfc4_dn7 = assign43790_e42441_d_n7;
        locals.var_vschfc4_dn8 = assign43790_e42441_d_n8;

        let (assign43800_e42458, assign43800_e42458_d_n7, assign43800_e42458_d_n8,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43800_e42456: f64 = (locals.var_qsch4c * locals.var_vschfc4);
        (assign43800_e42456, (locals.var_qsch4c * locals.var_vschfc4_dn7), (locals.var_qsch4c * locals.var_vschfc4_dn8),)
    } else {
        (locals.var_qsch4, locals.var_qsch4_dn7, locals.var_qsch4_dn8,)
    }
};
        locals.var_qsch4 = assign43800_e42458;
        locals.var_qsch4_dn7 = assign43800_e42458_d_n7;
        locals.var_qsch4_dn8 = assign43800_e42458_d_n8;

        let assign43810_e42461: f64 = if p.p309 >= 5.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign43810_e42461;

        let (assign43820_e42488,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43820_e42478: f64 = (7.0 * locals.var_qsch4c);
        let assign43820_e42481: f64 = (10.0 * p.p306);
        let assign43820_e42484: f64 = (1.0 - p.p308);
        let assign43820_e42485: f64 = (assign43820_e42481 * assign43820_e42484);
        let assign43820_e42486: f64 = (assign43820_e42478 / assign43820_e42485);
        (assign43820_e42486,)
    } else {
        (locals.var_qsch5c,)
    }
};
        locals.var_qsch5c = assign43820_e42488;

        let (assign43830_e42507, assign43830_e42507_d_n7, assign43830_e42507_d_n8,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43830_e42505: f64 = (locals.var_vschfc4 * locals.var_vschfc1);
        (assign43830_e42505, ((locals.var_vschfc4_dn7 * locals.var_vschfc1) + (locals.var_vschfc4 * locals.var_vschfc1_dn7)), ((locals.var_vschfc4_dn8 * locals.var_vschfc1) + (locals.var_vschfc4 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc5, locals.var_vschfc5_dn7, locals.var_vschfc5_dn8,)
    }
};
        locals.var_vschfc5 = assign43830_e42507;
        locals.var_vschfc5_dn7 = assign43830_e42507_d_n7;
        locals.var_vschfc5_dn8 = assign43830_e42507_d_n8;

        let (assign43840_e42526, assign43840_e42526_d_n7, assign43840_e42526_d_n8,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43840_e42524: f64 = (locals.var_qsch5c * locals.var_vschfc5);
        (assign43840_e42524, (locals.var_qsch5c * locals.var_vschfc5_dn7), (locals.var_qsch5c * locals.var_vschfc5_dn8),)
    } else {
        (locals.var_qsch5, locals.var_qsch5_dn7, locals.var_qsch5_dn8,)
    }
};
        locals.var_qsch5 = assign43840_e42526;
        locals.var_qsch5_dn7 = assign43840_e42526_d_n7;
        locals.var_qsch5_dn8 = assign43840_e42526_d_n8;

        let (assign43850_e42544,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch5c,)
    }
};
        locals.var_qsch5c = assign43850_e42544;

        let (assign43860_e42560,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch4c,)
    }
};
        locals.var_qsch4c = assign43860_e42560;

        let (assign43870_e42574,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch3c,)
    }
};
        locals.var_qsch3c = assign43870_e42574;

        let (assign43880_e42586,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch2c,)
    }
};
        locals.var_qsch2c = assign43880_e42586;

        let (assign43890_e42596,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch1c,)
    }
};
        locals.var_qsch1c = assign43890_e42596;

        let (assign43900_e42629, assign43900_e42629_d_n7, assign43900_e42629_d_n8,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) {
        let assign43900_e42603: f64 = (p.p6 * 2.0);
        let assign43900_e42605: f64 = (assign43900_e42603 * p.p307);
        let assign43900_e42607: f64 = (assign43900_e42605 * p.p0);
        let assign43900_e42610: f64 = (1.0 - p.p311);
        let assign43900_e42611: f64 = (assign43900_e42607 * assign43900_e42610);
        let assign43900_e42613: f64 = (assign43900_e42611 * p.p2);
        let assign43900_e42615: f64 = (assign43900_e42613 * p.p306);
        let assign43900_e42618: f64 = (locals.var_qsch0 + locals.var_qsch1);
        let assign43900_e42620: f64 = (assign43900_e42618 + locals.var_qsch2);
        let assign43900_e42622: f64 = (assign43900_e42620 + locals.var_qsch3);
        let assign43900_e42624: f64 = (assign43900_e42622 + locals.var_qsch4);
        let assign43900_e42626: f64 = (assign43900_e42624 + locals.var_qsch5);
        let assign43900_e42627: f64 = (assign43900_e42615 * assign43900_e42626);
        (assign43900_e42627, (assign43900_e42615 * ((((locals.var_qsch1_dn7 + locals.var_qsch2_dn7) + locals.var_qsch3_dn7) + locals.var_qsch4_dn7) + locals.var_qsch5_dn7)), (assign43900_e42615 * ((((locals.var_qsch1_dn8 + locals.var_qsch2_dn8) + locals.var_qsch3_dn8) + locals.var_qsch4_dn8) + locals.var_qsch5_dn8)),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn7, locals.var_qsch_dn8,)
    }
};
        locals.var_qsch = assign43900_e42629;
        locals.var_qsch_dn7 = assign43900_e42629_d_n7;
        locals.var_qsch_dn8 = assign43900_e42629_d_n8;

        let assign46690_e45519: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign46690_e45519;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign10_e2189: f64 = (p.p5 + 273.15);
        locals.var_tnomk = assign10_e2189;
        locals.var_tnomk_rv = 0.0;

        let assign20_e2190: f64 = ctx_temp;
        locals.var_tambk = assign20_e2190;
        locals.var_tambk_rv = 0.0;

        locals.var_tsh = (nv4 - 0.0);
        locals.var_tsh_dn4 = 1.0;
        locals.var_tsh_rv = 0.0;

        let assign50_e2198: f64 = (locals.var_tambk + p.p3);
        let assign50_e2200: f64 = (assign50_e2198 + locals.var_tsh);
        locals.var_tdut = assign50_e2200;
        locals.var_tdut_dn4 = locals.var_tsh_dn4;
        locals.var_tdut_rv = 0.0;

        let assign60_e2203: f64 = (-270.0);
        let assign60_e2205: f64 = (assign60_e2203 + 273.15);
        let assign60_e2206: f64 = if locals.var_tdut < assign60_e2205 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign60_e2206;
        locals.var_guard2_rv = 0.0;

        let (assign70_e2213, assign70_e2213_d_n4,) = {
    if (locals.var_guard2 != 0.0) {
        let assign70_e2209: f64 = (-270.0);
        let assign70_e2211: f64 = (assign70_e2209 + 273.15);
        (assign70_e2211, 0.0,)
    } else {
        (locals.var_tdut, locals.var_tdut_dn4,)
    }
};
        locals.var_tdut = assign70_e2213;
        locals.var_tdut_dn4 = assign70_e2213_d_n4;
        locals.var_tdut_rv = 0.0;

        let assign80_e2217: f64 = (1500.0 + 273.15);
        let assign80_e2218: f64 = if locals.var_tdut > assign80_e2217 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign80_e2218;
        locals.var_guard3_rv = 0.0;

        let (assign90_e2227, assign90_e2227_d_n4,) = {
    if ((locals.var_guard2 == 0.0) && (locals.var_guard3 != 0.0)) {
        let assign90_e2225: f64 = (1500.0 + 273.15);
        (assign90_e2225, 0.0,)
    } else {
        (locals.var_tdut, locals.var_tdut_dn4,)
    }
};
        locals.var_tdut = assign90_e2227;
        locals.var_tdut_dn4 = assign90_e2227_d_n4;
        locals.var_tdut_rv = 0.0;

        let assign290_e2401: f64 = (1.38062e-23 * locals.var_tdut);
        let assign290_e2403: f64 = (assign290_e2401 / 1.60219e-19);
        locals.var_phit = assign290_e2403;
        locals.var_phit_dn4 = ((1.38062e-23 * locals.var_tdut_dn4) / 1.60219e-19);
        locals.var_phit_rv = 0.0;

        let assign460_e2668: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign460_e2669: f64 = (p.p8 * assign460_e2668);
        let assign460_e2670: f64 = (1.0 + assign460_e2669);
        let (assign460_e2681, assign460_e2681_d_n4,) = {
    if (assign460_e2670 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign460_e2678: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign460_e2679: f64 = (p.p8 * assign460_e2678);
        let assign460_e2680: f64 = (1.0 + assign460_e2679);
        (assign460_e2680, (p.p8 * locals.var_tdut_dn4),)
    }
};
        let assign460_e2682: f64 = (p.p7 * assign460_e2681);
        locals.var_cgt = assign460_e2682;
        locals.var_cgt_dn4 = (p.p7 * assign460_e2681_d_n4);
        locals.var_cgt_rv = 0.0;

        let assign470_e2688: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign470_e2689: f64 = (p.p82 * assign470_e2688);
        let assign470_e2690: f64 = (1.0 + assign470_e2689);
        let (assign470_e2701, assign470_e2701_d_n4,) = {
    if (assign470_e2690 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign470_e2698: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign470_e2699: f64 = (p.p82 * assign470_e2698);
        let assign470_e2700: f64 = (1.0 + assign470_e2699);
        (assign470_e2700, (p.p82 * locals.var_tdut_dn4),)
    }
};
        let assign470_e2702: f64 = (p.p81 * assign470_e2701);
        locals.var_cgfps1t = assign470_e2702;
        locals.var_cgfps1t_dn4 = (p.p81 * assign470_e2701_d_n4);
        locals.var_cgfps1t_rv = 0.0;

        let assign480_e2708: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign480_e2709: f64 = (p.p104 * assign480_e2708);
        let assign480_e2710: f64 = (1.0 + assign480_e2709);
        let (assign480_e2721, assign480_e2721_d_n4,) = {
    if (assign480_e2710 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign480_e2718: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign480_e2719: f64 = (p.p104 * assign480_e2718);
        let assign480_e2720: f64 = (1.0 + assign480_e2719);
        (assign480_e2720, (p.p104 * locals.var_tdut_dn4),)
    }
};
        let assign480_e2722: f64 = (p.p103 * assign480_e2721);
        locals.var_cgfps2t = assign480_e2722;
        locals.var_cgfps2t_dn4 = (p.p103 * assign480_e2721_d_n4);
        locals.var_cgfps2t_rv = 0.0;

        let assign490_e2728: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign490_e2729: f64 = (p.p126 * assign490_e2728);
        let assign490_e2730: f64 = (1.0 + assign490_e2729);
        let (assign490_e2741, assign490_e2741_d_n4,) = {
    if (assign490_e2730 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign490_e2738: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign490_e2739: f64 = (p.p126 * assign490_e2738);
        let assign490_e2740: f64 = (1.0 + assign490_e2739);
        (assign490_e2740, (p.p126 * locals.var_tdut_dn4),)
    }
};
        let assign490_e2742: f64 = (p.p125 * assign490_e2741);
        locals.var_cgfps3t = assign490_e2742;
        locals.var_cgfps3t_dn4 = (p.p125 * assign490_e2741_d_n4);
        locals.var_cgfps3t_rv = 0.0;

        let assign500_e2748: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign500_e2749: f64 = (p.p148 * assign500_e2748);
        let assign500_e2750: f64 = (1.0 + assign500_e2749);
        let (assign500_e2761, assign500_e2761_d_n4,) = {
    if (assign500_e2750 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign500_e2758: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign500_e2759: f64 = (p.p148 * assign500_e2758);
        let assign500_e2760: f64 = (1.0 + assign500_e2759);
        (assign500_e2760, (p.p148 * locals.var_tdut_dn4),)
    }
};
        let assign500_e2762: f64 = (p.p147 * assign500_e2761);
        locals.var_cgfps4t = assign500_e2762;
        locals.var_cgfps4t_dn4 = (p.p147 * assign500_e2761_d_n4);
        locals.var_cgfps4t_rv = 0.0;

        let assign510_e2768: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign510_e2769: f64 = (p.p87 * assign510_e2768);
        let assign510_e2770: f64 = (1.0 + assign510_e2769);
        let (assign510_e2781, assign510_e2781_d_n4,) = {
    if (assign510_e2770 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign510_e2778: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign510_e2779: f64 = (p.p87 * assign510_e2778);
        let assign510_e2780: f64 = (1.0 + assign510_e2779);
        (assign510_e2780, (p.p87 * locals.var_tdut_dn4),)
    }
};
        let assign510_e2782: f64 = (p.p86 * assign510_e2781);
        locals.var_ccfps1t = assign510_e2782;
        locals.var_ccfps1t_dn4 = (p.p86 * assign510_e2781_d_n4);
        locals.var_ccfps1t_rv = 0.0;

        let assign520_e2788: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign520_e2789: f64 = (p.p109 * assign520_e2788);
        let assign520_e2790: f64 = (1.0 + assign520_e2789);
        let (assign520_e2801, assign520_e2801_d_n4,) = {
    if (assign520_e2790 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign520_e2798: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign520_e2799: f64 = (p.p109 * assign520_e2798);
        let assign520_e2800: f64 = (1.0 + assign520_e2799);
        (assign520_e2800, (p.p109 * locals.var_tdut_dn4),)
    }
};
        let assign520_e2802: f64 = (p.p108 * assign520_e2801);
        locals.var_ccfps2t = assign520_e2802;
        locals.var_ccfps2t_dn4 = (p.p108 * assign520_e2801_d_n4);
        locals.var_ccfps2t_rv = 0.0;

        let assign530_e2808: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign530_e2809: f64 = (p.p131 * assign530_e2808);
        let assign530_e2810: f64 = (1.0 + assign530_e2809);
        let (assign530_e2821, assign530_e2821_d_n4,) = {
    if (assign530_e2810 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign530_e2818: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign530_e2819: f64 = (p.p131 * assign530_e2818);
        let assign530_e2820: f64 = (1.0 + assign530_e2819);
        (assign530_e2820, (p.p131 * locals.var_tdut_dn4),)
    }
};
        let assign530_e2822: f64 = (p.p130 * assign530_e2821);
        locals.var_ccfps3t = assign530_e2822;
        locals.var_ccfps3t_dn4 = (p.p130 * assign530_e2821_d_n4);
        locals.var_ccfps3t_rv = 0.0;

        let assign540_e2828: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign540_e2829: f64 = (p.p153 * assign540_e2828);
        let assign540_e2830: f64 = (1.0 + assign540_e2829);
        let (assign540_e2841, assign540_e2841_d_n4,) = {
    if (assign540_e2830 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign540_e2838: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign540_e2839: f64 = (p.p153 * assign540_e2838);
        let assign540_e2840: f64 = (1.0 + assign540_e2839);
        (assign540_e2840, (p.p153 * locals.var_tdut_dn4),)
    }
};
        let assign540_e2842: f64 = (p.p152 * assign540_e2841);
        locals.var_ccfps4t = assign540_e2842;
        locals.var_ccfps4t_dn4 = (p.p152 * assign540_e2841_d_n4);
        locals.var_ccfps4t_rv = 0.0;

        let assign550_e2848: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign550_e2849: f64 = (p.p89 * assign550_e2848);
        let assign550_e2850: f64 = (1.0 + assign550_e2849);
        let (assign550_e2861, assign550_e2861_d_n4,) = {
    if (assign550_e2850 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign550_e2858: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign550_e2859: f64 = (p.p89 * assign550_e2858);
        let assign550_e2860: f64 = (1.0 + assign550_e2859);
        (assign550_e2860, (p.p89 * locals.var_tdut_dn4),)
    }
};
        let assign550_e2862: f64 = (p.p88 * assign550_e2861);
        locals.var_cbfps1t = assign550_e2862;
        locals.var_cbfps1t_dn4 = (p.p88 * assign550_e2861_d_n4);
        locals.var_cbfps1t_rv = 0.0;

        let assign560_e2868: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign560_e2869: f64 = (p.p111 * assign560_e2868);
        let assign560_e2870: f64 = (1.0 + assign560_e2869);
        let (assign560_e2881, assign560_e2881_d_n4,) = {
    if (assign560_e2870 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign560_e2878: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign560_e2879: f64 = (p.p111 * assign560_e2878);
        let assign560_e2880: f64 = (1.0 + assign560_e2879);
        (assign560_e2880, (p.p111 * locals.var_tdut_dn4),)
    }
};
        let assign560_e2882: f64 = (p.p110 * assign560_e2881);
        locals.var_cbfps2t = assign560_e2882;
        locals.var_cbfps2t_dn4 = (p.p110 * assign560_e2881_d_n4);
        locals.var_cbfps2t_rv = 0.0;

        let assign570_e2888: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign570_e2889: f64 = (p.p133 * assign570_e2888);
        let assign570_e2890: f64 = (1.0 + assign570_e2889);
        let (assign570_e2901, assign570_e2901_d_n4,) = {
    if (assign570_e2890 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign570_e2898: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign570_e2899: f64 = (p.p133 * assign570_e2898);
        let assign570_e2900: f64 = (1.0 + assign570_e2899);
        (assign570_e2900, (p.p133 * locals.var_tdut_dn4),)
    }
};
        let assign570_e2902: f64 = (p.p132 * assign570_e2901);
        locals.var_cbfps3t = assign570_e2902;
        locals.var_cbfps3t_dn4 = (p.p132 * assign570_e2901_d_n4);
        locals.var_cbfps3t_rv = 0.0;

        let assign580_e2908: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign580_e2909: f64 = (p.p155 * assign580_e2908);
        let assign580_e2910: f64 = (1.0 + assign580_e2909);
        let (assign580_e2921, assign580_e2921_d_n4,) = {
    if (assign580_e2910 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign580_e2918: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign580_e2919: f64 = (p.p155 * assign580_e2918);
        let assign580_e2920: f64 = (1.0 + assign580_e2919);
        (assign580_e2920, (p.p155 * locals.var_tdut_dn4),)
    }
};
        let assign580_e2922: f64 = (p.p154 * assign580_e2921);
        locals.var_cbfps4t = assign580_e2922;
        locals.var_cbfps4t_dn4 = (p.p154 * assign580_e2921_d_n4);
        locals.var_cbfps4t_rv = 0.0;

        let assign590_e2928: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign590_e2929: f64 = (p.p170 * assign590_e2928);
        let assign590_e2930: f64 = (1.0 + assign590_e2929);
        let (assign590_e2941, assign590_e2941_d_n4,) = {
    if (assign590_e2930 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign590_e2938: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign590_e2939: f64 = (p.p170 * assign590_e2938);
        let assign590_e2940: f64 = (1.0 + assign590_e2939);
        (assign590_e2940, (p.p170 * locals.var_tdut_dn4),)
    }
};
        let assign590_e2942: f64 = (p.p169 * assign590_e2941);
        locals.var_cgfp1t = assign590_e2942;
        locals.var_cgfp1t_dn4 = (p.p169 * assign590_e2941_d_n4);
        locals.var_cgfp1t_rv = 0.0;

        let assign600_e2948: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign600_e2949: f64 = (p.p192 * assign600_e2948);
        let assign600_e2950: f64 = (1.0 + assign600_e2949);
        let (assign600_e2961, assign600_e2961_d_n4,) = {
    if (assign600_e2950 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign600_e2958: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign600_e2959: f64 = (p.p192 * assign600_e2958);
        let assign600_e2960: f64 = (1.0 + assign600_e2959);
        (assign600_e2960, (p.p192 * locals.var_tdut_dn4),)
    }
};
        let assign600_e2962: f64 = (p.p191 * assign600_e2961);
        locals.var_cgfp2t = assign600_e2962;
        locals.var_cgfp2t_dn4 = (p.p191 * assign600_e2961_d_n4);
        locals.var_cgfp2t_rv = 0.0;

        let assign610_e2968: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign610_e2969: f64 = (p.p214 * assign610_e2968);
        let assign610_e2970: f64 = (1.0 + assign610_e2969);
        let (assign610_e2981, assign610_e2981_d_n4,) = {
    if (assign610_e2970 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign610_e2978: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign610_e2979: f64 = (p.p214 * assign610_e2978);
        let assign610_e2980: f64 = (1.0 + assign610_e2979);
        (assign610_e2980, (p.p214 * locals.var_tdut_dn4),)
    }
};
        let assign610_e2982: f64 = (p.p213 * assign610_e2981);
        locals.var_cgfp3t = assign610_e2982;
        locals.var_cgfp3t_dn4 = (p.p213 * assign610_e2981_d_n4);
        locals.var_cgfp3t_rv = 0.0;

        let assign620_e2988: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign620_e2989: f64 = (p.p236 * assign620_e2988);
        let assign620_e2990: f64 = (1.0 + assign620_e2989);
        let (assign620_e3001, assign620_e3001_d_n4,) = {
    if (assign620_e2990 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign620_e2998: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign620_e2999: f64 = (p.p236 * assign620_e2998);
        let assign620_e3000: f64 = (1.0 + assign620_e2999);
        (assign620_e3000, (p.p236 * locals.var_tdut_dn4),)
    }
};
        let assign620_e3002: f64 = (p.p235 * assign620_e3001);
        locals.var_cgfp4t = assign620_e3002;
        locals.var_cgfp4t_dn4 = (p.p235 * assign620_e3001_d_n4);
        locals.var_cgfp4t_rv = 0.0;

        let assign630_e3008: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign630_e3009: f64 = (p.p175 * assign630_e3008);
        let assign630_e3010: f64 = (1.0 + assign630_e3009);
        let (assign630_e3021, assign630_e3021_d_n4,) = {
    if (assign630_e3010 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign630_e3018: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign630_e3019: f64 = (p.p175 * assign630_e3018);
        let assign630_e3020: f64 = (1.0 + assign630_e3019);
        (assign630_e3020, (p.p175 * locals.var_tdut_dn4),)
    }
};
        let assign630_e3022: f64 = (p.p174 * assign630_e3021);
        locals.var_ccfp1t = assign630_e3022;
        locals.var_ccfp1t_dn4 = (p.p174 * assign630_e3021_d_n4);
        locals.var_ccfp1t_rv = 0.0;

        let assign640_e3028: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign640_e3029: f64 = (p.p197 * assign640_e3028);
        let assign640_e3030: f64 = (1.0 + assign640_e3029);
        let (assign640_e3041, assign640_e3041_d_n4,) = {
    if (assign640_e3030 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign640_e3038: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign640_e3039: f64 = (p.p197 * assign640_e3038);
        let assign640_e3040: f64 = (1.0 + assign640_e3039);
        (assign640_e3040, (p.p197 * locals.var_tdut_dn4),)
    }
};
        let assign640_e3042: f64 = (p.p196 * assign640_e3041);
        locals.var_ccfp2t = assign640_e3042;
        locals.var_ccfp2t_dn4 = (p.p196 * assign640_e3041_d_n4);
        locals.var_ccfp2t_rv = 0.0;

        let assign650_e3048: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign650_e3049: f64 = (p.p219 * assign650_e3048);
        let assign650_e3050: f64 = (1.0 + assign650_e3049);
        let (assign650_e3061, assign650_e3061_d_n4,) = {
    if (assign650_e3050 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign650_e3058: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign650_e3059: f64 = (p.p219 * assign650_e3058);
        let assign650_e3060: f64 = (1.0 + assign650_e3059);
        (assign650_e3060, (p.p219 * locals.var_tdut_dn4),)
    }
};
        let assign650_e3062: f64 = (p.p218 * assign650_e3061);
        locals.var_ccfp3t = assign650_e3062;
        locals.var_ccfp3t_dn4 = (p.p218 * assign650_e3061_d_n4);
        locals.var_ccfp3t_rv = 0.0;

        let assign660_e3068: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign660_e3069: f64 = (p.p241 * assign660_e3068);
        let assign660_e3070: f64 = (1.0 + assign660_e3069);
        let (assign660_e3081, assign660_e3081_d_n4,) = {
    if (assign660_e3070 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign660_e3078: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign660_e3079: f64 = (p.p241 * assign660_e3078);
        let assign660_e3080: f64 = (1.0 + assign660_e3079);
        (assign660_e3080, (p.p241 * locals.var_tdut_dn4),)
    }
};
        let assign660_e3082: f64 = (p.p240 * assign660_e3081);
        locals.var_ccfp4t = assign660_e3082;
        locals.var_ccfp4t_dn4 = (p.p240 * assign660_e3081_d_n4);
        locals.var_ccfp4t_rv = 0.0;

        let assign670_e3088: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign670_e3089: f64 = (p.p177 * assign670_e3088);
        let assign670_e3090: f64 = (1.0 + assign670_e3089);
        let (assign670_e3101, assign670_e3101_d_n4,) = {
    if (assign670_e3090 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign670_e3098: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign670_e3099: f64 = (p.p177 * assign670_e3098);
        let assign670_e3100: f64 = (1.0 + assign670_e3099);
        (assign670_e3100, (p.p177 * locals.var_tdut_dn4),)
    }
};
        let assign670_e3102: f64 = (p.p176 * assign670_e3101);
        locals.var_cbfp1t = assign670_e3102;
        locals.var_cbfp1t_dn4 = (p.p176 * assign670_e3101_d_n4);
        locals.var_cbfp1t_rv = 0.0;

        let assign680_e3108: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign680_e3109: f64 = (p.p199 * assign680_e3108);
        let assign680_e3110: f64 = (1.0 + assign680_e3109);
        let (assign680_e3121, assign680_e3121_d_n4,) = {
    if (assign680_e3110 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign680_e3118: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign680_e3119: f64 = (p.p199 * assign680_e3118);
        let assign680_e3120: f64 = (1.0 + assign680_e3119);
        (assign680_e3120, (p.p199 * locals.var_tdut_dn4),)
    }
};
        let assign680_e3122: f64 = (p.p198 * assign680_e3121);
        locals.var_cbfp2t = assign680_e3122;
        locals.var_cbfp2t_dn4 = (p.p198 * assign680_e3121_d_n4);
        locals.var_cbfp2t_rv = 0.0;

        let assign690_e3128: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign690_e3129: f64 = (p.p221 * assign690_e3128);
        let assign690_e3130: f64 = (1.0 + assign690_e3129);
        let (assign690_e3141, assign690_e3141_d_n4,) = {
    if (assign690_e3130 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign690_e3138: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign690_e3139: f64 = (p.p221 * assign690_e3138);
        let assign690_e3140: f64 = (1.0 + assign690_e3139);
        (assign690_e3140, (p.p221 * locals.var_tdut_dn4),)
    }
};
        let assign690_e3142: f64 = (p.p220 * assign690_e3141);
        locals.var_cbfp3t = assign690_e3142;
        locals.var_cbfp3t_dn4 = (p.p220 * assign690_e3141_d_n4);
        locals.var_cbfp3t_rv = 0.0;

        let assign700_e3148: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign700_e3149: f64 = (p.p243 * assign700_e3148);
        let assign700_e3150: f64 = (1.0 + assign700_e3149);
        let (assign700_e3161, assign700_e3161_d_n4,) = {
    if (assign700_e3150 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign700_e3158: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign700_e3159: f64 = (p.p243 * assign700_e3158);
        let assign700_e3160: f64 = (1.0 + assign700_e3159);
        (assign700_e3160, (p.p243 * locals.var_tdut_dn4),)
    }
};
        let assign700_e3162: f64 = (p.p242 * assign700_e3161);
        locals.var_cbfp4t = assign700_e3162;
        locals.var_cbfp4t_dn4 = (p.p242 * assign700_e3161_d_n4);
        locals.var_cbfp4t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let assign710_e3165: f64 = (p.p6 * (nv5 - nv9));
        locals.var_vdsi = assign710_e3165;
        locals.var_vdsi_dn5 = p.p6;
        locals.var_vdsi_dn9 = (-p.p6);
        locals.var_vdsi_rv = 0.0;

        let assign720_e3168: f64 = (p.p6 * (nv8 - nv9));
        locals.var_vgsi = assign720_e3168;
        locals.var_vgsi_dn8 = p.p6;
        locals.var_vgsi_dn9 = (-p.p6);
        locals.var_vgsi_rv = 0.0;

        locals.var_vdlinput = 0.0;
        locals.var_vdlinput_dn22 = 0.0;
        locals.var_vdlinput_rv = 0.0;

        locals.var_vglinput = 0.0;
        locals.var_vglinput_dn25 = 0.0;
        locals.var_vglinput_rv = 0.0;

        locals.var_vdloutput = 0.0;
        locals.var_vdloutput_dn23 = 0.0;
        locals.var_vdloutput_rv = 0.0;

        locals.var_vgloutput = 0.0;
        locals.var_vgloutput_dn26 = 0.0;
        locals.var_vgloutput_rv = 0.0;

        locals.var_chargefracd = 0.0;
        locals.var_chargefracd_dn22 = 0.0;
        locals.var_chargefracd_dn23 = 0.0;
        locals.var_chargefracd_rv = 0.0;

        locals.var_chargefracg = 0.0;
        locals.var_chargefracg_dn25 = 0.0;
        locals.var_chargefracg_dn26 = 0.0;
        locals.var_chargefracg_rv = 0.0;

        locals.var_chargefrac = 1.0;
        locals.var_chargefrac_dn22 = 0.0;
        locals.var_chargefrac_dn23 = 0.0;
        locals.var_chargefrac_dn25 = 0.0;
        locals.var_chargefrac_dn26 = 0.0;
        locals.var_chargefrac_rv = 0.0;

        let assign910_e3295: f64 = if p.p328 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign910_e3295;
        locals.var_guard12_rv = 0.0;

        let assign950_e3413: f64 = if p.p328 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign950_e3413;
        locals.var_guard13_rv = 0.0;

        let (assign960_e3420, assign960_e3420_d_n22,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv22 - 0.0), 1.0,)
    } else {
        (locals.var_vdlinput, locals.var_vdlinput_dn22,)
    }
};
        locals.var_vdlinput = assign960_e3420;
        locals.var_vdlinput_dn22 = assign960_e3420_d_n22;
        locals.var_vdlinput_rv = 0.0;

        let (assign970_e3427, assign970_e3427_d_n23,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv23 - 0.0), 1.0,)
    } else {
        (locals.var_vdloutput, locals.var_vdloutput_dn23,)
    }
};
        locals.var_vdloutput = assign970_e3427;
        locals.var_vdloutput_dn23 = assign970_e3427_d_n23;
        locals.var_vdloutput_rv = 0.0;

        let (assign980_e3439, assign980_e3439_d_n22, assign980_e3439_d_n23,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign980_e3434: f64 = (locals.var_vdloutput - locals.var_vdlinput);
        let assign980_e3435: f64 = (assign980_e3434).abs();
        let assign980_e3437: f64 = (assign980_e3435 / p.p338);
        (assign980_e3437, (if assign980_e3434 >= 0.0 { (-locals.var_vdlinput_dn22) } else { (-(-locals.var_vdlinput_dn22)) } / p.p338), (if assign980_e3434 >= 0.0 { locals.var_vdloutput_dn23 } else { (-locals.var_vdloutput_dn23) } / p.p338),)
    } else {
        (locals.var_chargefracd, locals.var_chargefracd_dn22, locals.var_chargefracd_dn23,)
    }
};
        locals.var_chargefracd = assign980_e3439;
        locals.var_chargefracd_dn22 = assign980_e3439_d_n22;
        locals.var_chargefracd_dn23 = assign980_e3439_d_n23;
        locals.var_chargefracd_rv = 0.0;

        let (assign990_e3446, assign990_e3446_d_n25,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv25 - 0.0), 1.0,)
    } else {
        (locals.var_vglinput, locals.var_vglinput_dn25,)
    }
};
        locals.var_vglinput = assign990_e3446;
        locals.var_vglinput_dn25 = assign990_e3446_d_n25;
        locals.var_vglinput_rv = 0.0;

        let (assign1000_e3453, assign1000_e3453_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv26 - 0.0), 1.0,)
    } else {
        (locals.var_vgloutput, locals.var_vgloutput_dn26,)
    }
};
        locals.var_vgloutput = assign1000_e3453;
        locals.var_vgloutput_dn26 = assign1000_e3453_d_n26;
        locals.var_vgloutput_rv = 0.0;

        let (assign1010_e3465, assign1010_e3465_d_n25, assign1010_e3465_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign1010_e3460: f64 = (locals.var_vgloutput - locals.var_vglinput);
        let assign1010_e3461: f64 = (assign1010_e3460).abs();
        let assign1010_e3463: f64 = (assign1010_e3461 / p.p337);
        (assign1010_e3463, (if assign1010_e3460 >= 0.0 { (-locals.var_vglinput_dn25) } else { (-(-locals.var_vglinput_dn25)) } / p.p337), (if assign1010_e3460 >= 0.0 { locals.var_vgloutput_dn26 } else { (-locals.var_vgloutput_dn26) } / p.p337),)
    } else {
        (locals.var_chargefracg, locals.var_chargefracg_dn25, locals.var_chargefracg_dn26,)
    }
};
        locals.var_chargefracg = assign1010_e3465;
        locals.var_chargefracg_dn25 = assign1010_e3465_d_n25;
        locals.var_chargefracg_dn26 = assign1010_e3465_d_n26;
        locals.var_chargefracg_rv = 0.0;

        let (assign1020_e3478, assign1020_e3478_d_n22, assign1020_e3478_d_n23, assign1020_e3478_d_n25, assign1020_e3478_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign1020_e3473: f64 = (1.0 + locals.var_chargefracd);
        let assign1020_e3475: f64 = (assign1020_e3473 + locals.var_chargefracg);
        let assign1020_e3476: f64 = (1.0 / assign1020_e3475);
        (assign1020_e3476, (-(locals.var_chargefracd_dn22 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracd_dn23 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracg_dn25 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracg_dn26 / (assign1020_e3475 * assign1020_e3475))),)
    } else {
        (locals.var_chargefrac, locals.var_chargefrac_dn22, locals.var_chargefrac_dn23, locals.var_chargefrac_dn25, locals.var_chargefrac_dn26,)
    }
};
        locals.var_chargefrac = assign1020_e3478;
        locals.var_chargefrac_dn22 = assign1020_e3478_d_n22;
        locals.var_chargefrac_dn23 = assign1020_e3478_d_n23;
        locals.var_chargefrac_dn25 = assign1020_e3478_d_n25;
        locals.var_chargefrac_dn26 = assign1020_e3478_d_n26;
        locals.var_chargefrac_rv = 0.0;

        let assign1110_e3597: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1110_e3597;
        locals.var_guard16_rv = 0.0;

        let (assign1120_e3603, assign1120_e3603_d_n2, assign1120_e3603_d_n7, assign1120_e3603_d_n10,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1120_e3601: f64 = (p.p6 * (nv7 - nv10));
        (assign1120_e3601, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    }
};
        locals.var_vgsfps1 = assign1120_e3603;
        locals.var_vgsfps1_dn2 = assign1120_e3603_d_n2;
        locals.var_vgsfps1_dn7 = assign1120_e3603_d_n7;
        locals.var_vgsfps1_dn10 = assign1120_e3603_d_n10;
        locals.var_vgsfps1_rv = 0.0;

        let (assign1130_e3609, assign1130_e3609_d_n2, assign1130_e3609_d_n7, assign1130_e3609_d_n10,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1130_e3607: f64 = (p.p6 * (nv2 - nv10));
        (assign1130_e3607, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    }
};
        locals.var_vcfps1 = assign1130_e3609;
        locals.var_vcfps1_dn2 = assign1130_e3609_d_n2;
        locals.var_vcfps1_dn7 = assign1130_e3609_d_n7;
        locals.var_vcfps1_dn10 = assign1130_e3609_d_n10;
        locals.var_vcfps1_rv = 0.0;

        let (assign1140_e3616, assign1140_e3616_d_n2, assign1140_e3616_d_n7, assign1140_e3616_d_n10,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1140_e3614: f64 = (p.p6 * (nv2 - nv10));
        (assign1140_e3614, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    }
};
        locals.var_vgsfps1 = assign1140_e3616;
        locals.var_vgsfps1_dn2 = assign1140_e3616_d_n2;
        locals.var_vgsfps1_dn7 = assign1140_e3616_d_n7;
        locals.var_vgsfps1_dn10 = assign1140_e3616_d_n10;
        locals.var_vgsfps1_rv = 0.0;

        let (assign1150_e3623, assign1150_e3623_d_n2, assign1150_e3623_d_n7, assign1150_e3623_d_n10,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1150_e3621: f64 = (p.p6 * (nv7 - nv10));
        (assign1150_e3621, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    }
};
        locals.var_vcfps1 = assign1150_e3623;
        locals.var_vcfps1_dn2 = assign1150_e3623_d_n2;
        locals.var_vcfps1_dn7 = assign1150_e3623_d_n7;
        locals.var_vcfps1_dn10 = assign1150_e3623_d_n10;
        locals.var_vcfps1_rv = 0.0;

        let assign1160_e3626: f64 = (p.p6 * (nv9 - nv10));
        locals.var_vdsfps1 = assign1160_e3626;
        locals.var_vdsfps1_dn9 = p.p6;
        locals.var_vdsfps1_dn10 = (-p.p6);
        locals.var_vdsfps1_rv = 0.0;

        let assign1170_e3629: f64 = (p.p6 * (nv3 - nv10));
        locals.var_vbfps1 = assign1170_e3629;
        locals.var_vbfps1_dn3 = p.p6;
        locals.var_vbfps1_dn10 = (-p.p6);
        locals.var_vbfps1_rv = 0.0;

        let assign1180_e3632: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1180_e3632;
        locals.var_guard17_rv = 0.0;

        let (assign1190_e3638, assign1190_e3638_d_n2, assign1190_e3638_d_n7, assign1190_e3638_d_n11,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1190_e3636: f64 = (p.p6 * (nv7 - nv11));
        (assign1190_e3636, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    }
};
        locals.var_vgsfps2 = assign1190_e3638;
        locals.var_vgsfps2_dn2 = assign1190_e3638_d_n2;
        locals.var_vgsfps2_dn7 = assign1190_e3638_d_n7;
        locals.var_vgsfps2_dn11 = assign1190_e3638_d_n11;
        locals.var_vgsfps2_rv = 0.0;

        let (assign1200_e3644, assign1200_e3644_d_n2, assign1200_e3644_d_n7, assign1200_e3644_d_n11,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1200_e3642: f64 = (p.p6 * (nv2 - nv11));
        (assign1200_e3642, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    }
};
        locals.var_vcfps2 = assign1200_e3644;
        locals.var_vcfps2_dn2 = assign1200_e3644_d_n2;
        locals.var_vcfps2_dn7 = assign1200_e3644_d_n7;
        locals.var_vcfps2_dn11 = assign1200_e3644_d_n11;
        locals.var_vcfps2_rv = 0.0;

        let (assign1210_e3651, assign1210_e3651_d_n2, assign1210_e3651_d_n7, assign1210_e3651_d_n11,) = {
    if (locals.var_guard17 == 0.0) {
        let assign1210_e3649: f64 = (p.p6 * (nv2 - nv11));
        (assign1210_e3649, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    }
};
        locals.var_vgsfps2 = assign1210_e3651;
        locals.var_vgsfps2_dn2 = assign1210_e3651_d_n2;
        locals.var_vgsfps2_dn7 = assign1210_e3651_d_n7;
        locals.var_vgsfps2_dn11 = assign1210_e3651_d_n11;
        locals.var_vgsfps2_rv = 0.0;

        let (assign1220_e3658, assign1220_e3658_d_n2, assign1220_e3658_d_n7, assign1220_e3658_d_n11,) = {
    if (locals.var_guard17 == 0.0) {
        let assign1220_e3656: f64 = (p.p6 * (nv7 - nv11));
        (assign1220_e3656, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    }
};
        locals.var_vcfps2 = assign1220_e3658;
        locals.var_vcfps2_dn2 = assign1220_e3658_d_n2;
        locals.var_vcfps2_dn7 = assign1220_e3658_d_n7;
        locals.var_vcfps2_dn11 = assign1220_e3658_d_n11;
        locals.var_vcfps2_rv = 0.0;

        let assign1230_e3661: f64 = (p.p6 * (nv10 - nv11));
        locals.var_vdsfps2 = assign1230_e3661;
        locals.var_vdsfps2_dn10 = p.p6;
        locals.var_vdsfps2_dn11 = (-p.p6);
        locals.var_vdsfps2_rv = 0.0;

        let assign1240_e3664: f64 = (p.p6 * (nv3 - nv11));
        locals.var_vbfps2 = assign1240_e3664;
        locals.var_vbfps2_dn3 = p.p6;
        locals.var_vbfps2_dn11 = (-p.p6);
        locals.var_vbfps2_rv = 0.0;

        let assign1250_e3667: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1250_e3667;
        locals.var_guard18_rv = 0.0;

        let (assign1260_e3673, assign1260_e3673_d_n2, assign1260_e3673_d_n7, assign1260_e3673_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1260_e3671: f64 = (p.p6 * (nv7 - nv12));
        (assign1260_e3671, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    }
};
        locals.var_vgsfps3 = assign1260_e3673;
        locals.var_vgsfps3_dn2 = assign1260_e3673_d_n2;
        locals.var_vgsfps3_dn7 = assign1260_e3673_d_n7;
        locals.var_vgsfps3_dn12 = assign1260_e3673_d_n12;
        locals.var_vgsfps3_rv = 0.0;

        let (assign1270_e3679, assign1270_e3679_d_n2, assign1270_e3679_d_n7, assign1270_e3679_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1270_e3677: f64 = (p.p6 * (nv2 - nv12));
        (assign1270_e3677, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    }
};
        locals.var_vcfps3 = assign1270_e3679;
        locals.var_vcfps3_dn2 = assign1270_e3679_d_n2;
        locals.var_vcfps3_dn7 = assign1270_e3679_d_n7;
        locals.var_vcfps3_dn12 = assign1270_e3679_d_n12;
        locals.var_vcfps3_rv = 0.0;

        let (assign1280_e3686, assign1280_e3686_d_n2, assign1280_e3686_d_n7, assign1280_e3686_d_n12,) = {
    if (locals.var_guard18 == 0.0) {
        let assign1280_e3684: f64 = (p.p6 * (nv2 - nv12));
        (assign1280_e3684, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    }
};
        locals.var_vgsfps3 = assign1280_e3686;
        locals.var_vgsfps3_dn2 = assign1280_e3686_d_n2;
        locals.var_vgsfps3_dn7 = assign1280_e3686_d_n7;
        locals.var_vgsfps3_dn12 = assign1280_e3686_d_n12;
        locals.var_vgsfps3_rv = 0.0;

        let (assign1290_e3693, assign1290_e3693_d_n2, assign1290_e3693_d_n7, assign1290_e3693_d_n12,) = {
    if (locals.var_guard18 == 0.0) {
        let assign1290_e3691: f64 = (p.p6 * (nv7 - nv12));
        (assign1290_e3691, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    }
};
        locals.var_vcfps3 = assign1290_e3693;
        locals.var_vcfps3_dn2 = assign1290_e3693_d_n2;
        locals.var_vcfps3_dn7 = assign1290_e3693_d_n7;
        locals.var_vcfps3_dn12 = assign1290_e3693_d_n12;
        locals.var_vcfps3_rv = 0.0;

        let assign1300_e3696: f64 = (p.p6 * (nv11 - nv12));
        locals.var_vdsfps3 = assign1300_e3696;
        locals.var_vdsfps3_dn11 = p.p6;
        locals.var_vdsfps3_dn12 = (-p.p6);
        locals.var_vdsfps3_rv = 0.0;

        let assign1310_e3699: f64 = (p.p6 * (nv3 - nv12));
        locals.var_vbfps3 = assign1310_e3699;
        locals.var_vbfps3_dn3 = p.p6;
        locals.var_vbfps3_dn12 = (-p.p6);
        locals.var_vbfps3_rv = 0.0;

        let assign1320_e3702: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1320_e3702;
        locals.var_guard19_rv = 0.0;

        let (assign1330_e3708, assign1330_e3708_d_n2, assign1330_e3708_d_n7, assign1330_e3708_d_n13,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1330_e3706: f64 = (p.p6 * (nv7 - nv13));
        (assign1330_e3706, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    }
};
        locals.var_vgsfps4 = assign1330_e3708;
        locals.var_vgsfps4_dn2 = assign1330_e3708_d_n2;
        locals.var_vgsfps4_dn7 = assign1330_e3708_d_n7;
        locals.var_vgsfps4_dn13 = assign1330_e3708_d_n13;
        locals.var_vgsfps4_rv = 0.0;

        let (assign1340_e3714, assign1340_e3714_d_n2, assign1340_e3714_d_n7, assign1340_e3714_d_n13,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1340_e3712: f64 = (p.p6 * (nv2 - nv13));
        (assign1340_e3712, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    }
};
        locals.var_vcfps4 = assign1340_e3714;
        locals.var_vcfps4_dn2 = assign1340_e3714_d_n2;
        locals.var_vcfps4_dn7 = assign1340_e3714_d_n7;
        locals.var_vcfps4_dn13 = assign1340_e3714_d_n13;
        locals.var_vcfps4_rv = 0.0;

        let (assign1350_e3721, assign1350_e3721_d_n2, assign1350_e3721_d_n7, assign1350_e3721_d_n13,) = {
    if (locals.var_guard19 == 0.0) {
        let assign1350_e3719: f64 = (p.p6 * (nv2 - nv13));
        (assign1350_e3719, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    }
};
        locals.var_vgsfps4 = assign1350_e3721;
        locals.var_vgsfps4_dn2 = assign1350_e3721_d_n2;
        locals.var_vgsfps4_dn7 = assign1350_e3721_d_n7;
        locals.var_vgsfps4_dn13 = assign1350_e3721_d_n13;
        locals.var_vgsfps4_rv = 0.0;

        let (assign1360_e3728, assign1360_e3728_d_n2, assign1360_e3728_d_n7, assign1360_e3728_d_n13,) = {
    if (locals.var_guard19 == 0.0) {
        let assign1360_e3726: f64 = (p.p6 * (nv7 - nv13));
        (assign1360_e3726, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    }
};
        locals.var_vcfps4 = assign1360_e3728;
        locals.var_vcfps4_dn2 = assign1360_e3728_d_n2;
        locals.var_vcfps4_dn7 = assign1360_e3728_d_n7;
        locals.var_vcfps4_dn13 = assign1360_e3728_d_n13;
        locals.var_vcfps4_rv = 0.0;

        let assign1370_e3731: f64 = (p.p6 * (nv12 - nv13));
        locals.var_vdsfps4 = assign1370_e3731;
        locals.var_vdsfps4_dn12 = p.p6;
        locals.var_vdsfps4_dn13 = (-p.p6);
        locals.var_vdsfps4_rv = 0.0;

        let assign1380_e3734: f64 = (p.p6 * (nv3 - nv13));
        locals.var_vbfps4 = assign1380_e3734;
        locals.var_vbfps4_dn3 = p.p6;
        locals.var_vbfps4_dn13 = (-p.p6);
        locals.var_vbfps4_rv = 0.0;

        let assign1390_e3737: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1390_e3737;
        locals.var_guard20_rv = 0.0;

        let (assign1400_e3743, assign1400_e3743_d_n2, assign1400_e3743_d_n5, assign1400_e3743_d_n7,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1400_e3741: f64 = (p.p6 * (nv7 - nv5));
        (assign1400_e3741, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    }
};
        locals.var_vgsfp1 = assign1400_e3743;
        locals.var_vgsfp1_dn2 = assign1400_e3743_d_n2;
        locals.var_vgsfp1_dn5 = assign1400_e3743_d_n5;
        locals.var_vgsfp1_dn7 = assign1400_e3743_d_n7;
        locals.var_vgsfp1_rv = 0.0;

        let (assign1410_e3749, assign1410_e3749_d_n2, assign1410_e3749_d_n5, assign1410_e3749_d_n7,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1410_e3747: f64 = (p.p6 * (nv2 - nv5));
        (assign1410_e3747, p.p6, (-p.p6), 0.0,)
    } else {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    }
};
        locals.var_vcfp1 = assign1410_e3749;
        locals.var_vcfp1_dn2 = assign1410_e3749_d_n2;
        locals.var_vcfp1_dn5 = assign1410_e3749_d_n5;
        locals.var_vcfp1_dn7 = assign1410_e3749_d_n7;
        locals.var_vcfp1_rv = 0.0;

        let (assign1420_e3756, assign1420_e3756_d_n2, assign1420_e3756_d_n5, assign1420_e3756_d_n7,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1420_e3754: f64 = (p.p6 * (nv2 - nv5));
        (assign1420_e3754, p.p6, (-p.p6), 0.0,)
    } else {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    }
};
        locals.var_vgsfp1 = assign1420_e3756;
        locals.var_vgsfp1_dn2 = assign1420_e3756_d_n2;
        locals.var_vgsfp1_dn5 = assign1420_e3756_d_n5;
        locals.var_vgsfp1_dn7 = assign1420_e3756_d_n7;
        locals.var_vgsfp1_rv = 0.0;

        let (assign1430_e3763, assign1430_e3763_d_n2, assign1430_e3763_d_n5, assign1430_e3763_d_n7,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1430_e3761: f64 = (p.p6 * (nv7 - nv5));
        (assign1430_e3761, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    }
};
        locals.var_vcfp1 = assign1430_e3763;
        locals.var_vcfp1_dn2 = assign1430_e3763_d_n2;
        locals.var_vcfp1_dn5 = assign1430_e3763_d_n5;
        locals.var_vcfp1_dn7 = assign1430_e3763_d_n7;
        locals.var_vcfp1_rv = 0.0;

        let assign1440_e3766: f64 = (p.p6 * (nv14 - nv5));
        locals.var_vdsfp1 = assign1440_e3766;
        locals.var_vdsfp1_dn5 = (-p.p6);
        locals.var_vdsfp1_dn14 = p.p6;
        locals.var_vdsfp1_rv = 0.0;

        let assign1450_e3769: f64 = (p.p6 * (nv3 - nv5));
        locals.var_vbfp1 = assign1450_e3769;
        locals.var_vbfp1_dn3 = p.p6;
        locals.var_vbfp1_dn5 = (-p.p6);
        locals.var_vbfp1_rv = 0.0;

        let assign1460_e3772: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1460_e3772;
        locals.var_guard21_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign1470_e3778, assign1470_e3778_d_n2, assign1470_e3778_d_n7, assign1470_e3778_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1470_e3776: f64 = (p.p6 * (nv7 - nv14));
        (assign1470_e3776, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    }
};
        locals.var_vgsfp2 = assign1470_e3778;
        locals.var_vgsfp2_dn2 = assign1470_e3778_d_n2;
        locals.var_vgsfp2_dn7 = assign1470_e3778_d_n7;
        locals.var_vgsfp2_dn14 = assign1470_e3778_d_n14;
        locals.var_vgsfp2_rv = 0.0;

        let (assign1480_e3784, assign1480_e3784_d_n2, assign1480_e3784_d_n7, assign1480_e3784_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1480_e3782: f64 = (p.p6 * (nv2 - nv14));
        (assign1480_e3782, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    }
};
        locals.var_vcfp2 = assign1480_e3784;
        locals.var_vcfp2_dn2 = assign1480_e3784_d_n2;
        locals.var_vcfp2_dn7 = assign1480_e3784_d_n7;
        locals.var_vcfp2_dn14 = assign1480_e3784_d_n14;
        locals.var_vcfp2_rv = 0.0;

        let (assign1490_e3791, assign1490_e3791_d_n2, assign1490_e3791_d_n7, assign1490_e3791_d_n14,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1490_e3789: f64 = (p.p6 * (nv2 - nv14));
        (assign1490_e3789, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    }
};
        locals.var_vgsfp2 = assign1490_e3791;
        locals.var_vgsfp2_dn2 = assign1490_e3791_d_n2;
        locals.var_vgsfp2_dn7 = assign1490_e3791_d_n7;
        locals.var_vgsfp2_dn14 = assign1490_e3791_d_n14;
        locals.var_vgsfp2_rv = 0.0;

        let (assign1500_e3798, assign1500_e3798_d_n2, assign1500_e3798_d_n7, assign1500_e3798_d_n14,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1500_e3796: f64 = (p.p6 * (nv7 - nv14));
        (assign1500_e3796, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    }
};
        locals.var_vcfp2 = assign1500_e3798;
        locals.var_vcfp2_dn2 = assign1500_e3798_d_n2;
        locals.var_vcfp2_dn7 = assign1500_e3798_d_n7;
        locals.var_vcfp2_dn14 = assign1500_e3798_d_n14;
        locals.var_vcfp2_rv = 0.0;

        let assign1510_e3801: f64 = (p.p6 * (nv15 - nv14));
        locals.var_vdsfp2 = assign1510_e3801;
        locals.var_vdsfp2_dn14 = (-p.p6);
        locals.var_vdsfp2_dn15 = p.p6;
        locals.var_vdsfp2_rv = 0.0;

        let assign1520_e3804: f64 = (p.p6 * (nv3 - nv14));
        locals.var_vbfp2 = assign1520_e3804;
        locals.var_vbfp2_dn3 = p.p6;
        locals.var_vbfp2_dn14 = (-p.p6);
        locals.var_vbfp2_rv = 0.0;

        let assign1530_e3807: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1530_e3807;
        locals.var_guard22_rv = 0.0;

        let (assign1540_e3813, assign1540_e3813_d_n2, assign1540_e3813_d_n7, assign1540_e3813_d_n15,) = {
    if (locals.var_guard22 != 0.0) {
        let assign1540_e3811: f64 = (p.p6 * (nv7 - nv15));
        (assign1540_e3811, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    }
};
        locals.var_vgsfp3 = assign1540_e3813;
        locals.var_vgsfp3_dn2 = assign1540_e3813_d_n2;
        locals.var_vgsfp3_dn7 = assign1540_e3813_d_n7;
        locals.var_vgsfp3_dn15 = assign1540_e3813_d_n15;
        locals.var_vgsfp3_rv = 0.0;

        let (assign1550_e3819, assign1550_e3819_d_n2, assign1550_e3819_d_n7, assign1550_e3819_d_n15,) = {
    if (locals.var_guard22 != 0.0) {
        let assign1550_e3817: f64 = (p.p6 * (nv2 - nv15));
        (assign1550_e3817, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    }
};
        locals.var_vcfp3 = assign1550_e3819;
        locals.var_vcfp3_dn2 = assign1550_e3819_d_n2;
        locals.var_vcfp3_dn7 = assign1550_e3819_d_n7;
        locals.var_vcfp3_dn15 = assign1550_e3819_d_n15;
        locals.var_vcfp3_rv = 0.0;

        let (assign1560_e3826, assign1560_e3826_d_n2, assign1560_e3826_d_n7, assign1560_e3826_d_n15,) = {
    if (locals.var_guard22 == 0.0) {
        let assign1560_e3824: f64 = (p.p6 * (nv2 - nv15));
        (assign1560_e3824, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    }
};
        locals.var_vgsfp3 = assign1560_e3826;
        locals.var_vgsfp3_dn2 = assign1560_e3826_d_n2;
        locals.var_vgsfp3_dn7 = assign1560_e3826_d_n7;
        locals.var_vgsfp3_dn15 = assign1560_e3826_d_n15;
        locals.var_vgsfp3_rv = 0.0;

        let (assign1570_e3833, assign1570_e3833_d_n2, assign1570_e3833_d_n7, assign1570_e3833_d_n15,) = {
    if (locals.var_guard22 == 0.0) {
        let assign1570_e3831: f64 = (p.p6 * (nv7 - nv15));
        (assign1570_e3831, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    }
};
        locals.var_vcfp3 = assign1570_e3833;
        locals.var_vcfp3_dn2 = assign1570_e3833_d_n2;
        locals.var_vcfp3_dn7 = assign1570_e3833_d_n7;
        locals.var_vcfp3_dn15 = assign1570_e3833_d_n15;
        locals.var_vcfp3_rv = 0.0;

        let assign1580_e3836: f64 = (p.p6 * (nv16 - nv15));
        locals.var_vdsfp3 = assign1580_e3836;
        locals.var_vdsfp3_dn15 = (-p.p6);
        locals.var_vdsfp3_dn16 = p.p6;
        locals.var_vdsfp3_rv = 0.0;

        let assign1590_e3839: f64 = (p.p6 * (nv3 - nv15));
        locals.var_vbfp3 = assign1590_e3839;
        locals.var_vbfp3_dn3 = p.p6;
        locals.var_vbfp3_dn15 = (-p.p6);
        locals.var_vbfp3_rv = 0.0;

        let assign1600_e3842: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1600_e3842;
        locals.var_guard23_rv = 0.0;

        let (assign1610_e3848, assign1610_e3848_d_n2, assign1610_e3848_d_n7, assign1610_e3848_d_n16,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1610_e3846: f64 = (p.p6 * (nv7 - nv16));
        (assign1610_e3846, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    }
};
        locals.var_vgsfp4 = assign1610_e3848;
        locals.var_vgsfp4_dn2 = assign1610_e3848_d_n2;
        locals.var_vgsfp4_dn7 = assign1610_e3848_d_n7;
        locals.var_vgsfp4_dn16 = assign1610_e3848_d_n16;
        locals.var_vgsfp4_rv = 0.0;

        let (assign1620_e3854, assign1620_e3854_d_n2, assign1620_e3854_d_n7, assign1620_e3854_d_n16,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1620_e3852: f64 = (p.p6 * (nv2 - nv16));
        (assign1620_e3852, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    }
};
        locals.var_vcfp4 = assign1620_e3854;
        locals.var_vcfp4_dn2 = assign1620_e3854_d_n2;
        locals.var_vcfp4_dn7 = assign1620_e3854_d_n7;
        locals.var_vcfp4_dn16 = assign1620_e3854_d_n16;
        locals.var_vcfp4_rv = 0.0;

        let (assign1630_e3861, assign1630_e3861_d_n2, assign1630_e3861_d_n7, assign1630_e3861_d_n16,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1630_e3859: f64 = (p.p6 * (nv2 - nv16));
        (assign1630_e3859, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    }
};
        locals.var_vgsfp4 = assign1630_e3861;
        locals.var_vgsfp4_dn2 = assign1630_e3861_d_n2;
        locals.var_vgsfp4_dn7 = assign1630_e3861_d_n7;
        locals.var_vgsfp4_dn16 = assign1630_e3861_d_n16;
        locals.var_vgsfp4_rv = 0.0;

        let (assign1640_e3868, assign1640_e3868_d_n2, assign1640_e3868_d_n7, assign1640_e3868_d_n16,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1640_e3866: f64 = (p.p6 * (nv7 - nv16));
        (assign1640_e3866, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    }
};
        locals.var_vcfp4 = assign1640_e3868;
        locals.var_vcfp4_dn2 = assign1640_e3868_d_n2;
        locals.var_vcfp4_dn7 = assign1640_e3868_d_n7;
        locals.var_vcfp4_dn16 = assign1640_e3868_d_n16;
        locals.var_vcfp4_rv = 0.0;

        let assign1650_e3871: f64 = (p.p6 * (nv17 - nv16));
        locals.var_vdsfp4 = assign1650_e3871;
        locals.var_vdsfp4_dn16 = (-p.p6);
        locals.var_vdsfp4_dn17 = p.p6;
        locals.var_vdsfp4_rv = 0.0;

        let assign1660_e3874: f64 = (p.p6 * (nv3 - nv16));
        locals.var_vbfp4 = assign1660_e3874;
        locals.var_vbfp4_dn3 = p.p6;
        locals.var_vbfp4_dn16 = (-p.p6);
        locals.var_vbfp4_rv = 0.0;

        locals.var_qgsfp4 = 0.0;
        locals.var_qgsfp4_dn2 = 0.0;
        locals.var_qgsfp4_dn4 = 0.0;
        locals.var_qgsfp4_dn7 = 0.0;
        locals.var_qgsfp4_dn16 = 0.0;
        locals.var_qgsfp4_dn17 = 0.0;
        locals.var_qgsfp4_rv = 0.0;

        locals.var_qgdfp4 = 0.0;
        locals.var_qgdfp4_dn2 = 0.0;
        locals.var_qgdfp4_dn4 = 0.0;
        locals.var_qgdfp4_dn7 = 0.0;
        locals.var_qgdfp4_dn16 = 0.0;
        locals.var_qgdfp4_dn17 = 0.0;
        locals.var_qgdfp4_rv = 0.0;

        locals.var_qcfp4 = 0.0;
        locals.var_qcfp4_dn2 = 0.0;
        locals.var_qcfp4_dn3 = 0.0;
        locals.var_qcfp4_dn4 = 0.0;
        locals.var_qcfp4_dn7 = 0.0;
        locals.var_qcfp4_dn16 = 0.0;
        locals.var_qcfp4_dn17 = 0.0;
        locals.var_qcfp4_rv = 0.0;

        locals.var_qbfp4 = 0.0;
        locals.var_qbfp4_dn2 = 0.0;
        locals.var_qbfp4_dn3 = 0.0;
        locals.var_qbfp4_dn4 = 0.0;
        locals.var_qbfp4_dn7 = 0.0;
        locals.var_qbfp4_dn16 = 0.0;
        locals.var_qbfp4_dn17 = 0.0;
        locals.var_qbfp4_rv = 0.0;

        locals.var_qsfp4 = 0.0;
        locals.var_qsfp4_dn2 = 0.0;
        locals.var_qsfp4_dn3 = 0.0;
        locals.var_qsfp4_dn4 = 0.0;
        locals.var_qsfp4_dn7 = 0.0;
        locals.var_qsfp4_dn16 = 0.0;
        locals.var_qsfp4_dn17 = 0.0;
        locals.var_qsfp4_rv = 0.0;

        let assign1750_e3885: f64 = if p.p233 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign1750_e3885;
        locals.var_guard24_rv = 0.0;

        let (assign1780_e3897, assign1780_e3897_d_n2, assign1780_e3897_d_n4, assign1780_e3897_d_n7, assign1780_e3897_d_n16, assign1780_e3897_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgsout = assign1780_e3897;
        locals.var_fn25_calc_iq__qgsout_dn2 = assign1780_e3897_d_n2;
        locals.var_fn25_calc_iq__qgsout_dn4 = assign1780_e3897_d_n4;
        locals.var_fn25_calc_iq__qgsout_dn7 = assign1780_e3897_d_n7;
        locals.var_fn25_calc_iq__qgsout_dn16 = assign1780_e3897_d_n16;
        locals.var_fn25_calc_iq__qgsout_dn17 = assign1780_e3897_d_n17;
        locals.var_fn25_calc_iq__qgsout_rv = 0.0;

        let (assign1790_e3901, assign1790_e3901_d_n2, assign1790_e3901_d_n4, assign1790_e3901_d_n7, assign1790_e3901_d_n16, assign1790_e3901_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgdout = assign1790_e3901;
        locals.var_fn25_calc_iq__qgdout_dn2 = assign1790_e3901_d_n2;
        locals.var_fn25_calc_iq__qgdout_dn4 = assign1790_e3901_d_n4;
        locals.var_fn25_calc_iq__qgdout_dn7 = assign1790_e3901_d_n7;
        locals.var_fn25_calc_iq__qgdout_dn16 = assign1790_e3901_d_n16;
        locals.var_fn25_calc_iq__qgdout_dn17 = assign1790_e3901_d_n17;
        locals.var_fn25_calc_iq__qgdout_rv = 0.0;

        let (assign1800_e3905, assign1800_e3905_d_n2, assign1800_e3905_d_n3, assign1800_e3905_d_n4, assign1800_e3905_d_n7, assign1800_e3905_d_n16, assign1800_e3905_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign1800_e3905;
        locals.var_fn25_calc_iq__qcout_dn2 = assign1800_e3905_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign1800_e3905_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign1800_e3905_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign1800_e3905_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign1800_e3905_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign1800_e3905_d_n17;
        locals.var_fn25_calc_iq__qcout_rv = 0.0;

        let (assign1810_e3909, assign1810_e3909_d_n2, assign1810_e3909_d_n3, assign1810_e3909_d_n4, assign1810_e3909_d_n7, assign1810_e3909_d_n16, assign1810_e3909_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign1810_e3909;
        locals.var_fn25_calc_iq__qbout_dn2 = assign1810_e3909_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign1810_e3909_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign1810_e3909_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign1810_e3909_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign1810_e3909_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign1810_e3909_d_n17;
        locals.var_fn25_calc_iq__qbout_rv = 0.0;

        let (assign1820_e3913, assign1820_e3913_d_n2, assign1820_e3913_d_n3, assign1820_e3913_d_n4, assign1820_e3913_d_n7, assign1820_e3913_d_n16, assign1820_e3913_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign1820_e3913;
        locals.var_fn25_calc_iq__qsout_dn2 = assign1820_e3913_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign1820_e3913_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign1820_e3913_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign1820_e3913_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign1820_e3913_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign1820_e3913_d_n17;
        locals.var_fn25_calc_iq__qsout_rv = 0.0;

        let (assign1830_e3917, assign1830_e3917_d_n4, assign1830_e3917_d_n16, assign1830_e3917_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vtdibl, locals.var_fn25_calc_iq__vtdibl_dn4, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vtdibl = assign1830_e3917;
        locals.var_fn25_calc_iq__vtdibl_dn4 = assign1830_e3917_d_n4;
        locals.var_fn25_calc_iq__vtdibl_dn16 = assign1830_e3917_d_n16;
        locals.var_fn25_calc_iq__vtdibl_dn17 = assign1830_e3917_d_n17;
        locals.var_fn25_calc_iq__vtdibl_rv = 0.0;

        let (assign1840_e3921, assign1840_e3921_d_n2, assign1840_e3921_d_n3, assign1840_e3921_d_n4, assign1840_e3921_d_n7, assign1840_e3921_d_n16, assign1840_e3921_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat1, locals.var_fn25_calc_iq__vdsat1_dn2, locals.var_fn25_calc_iq__vdsat1_dn3, locals.var_fn25_calc_iq__vdsat1_dn4, locals.var_fn25_calc_iq__vdsat1_dn7, locals.var_fn25_calc_iq__vdsat1_dn16, locals.var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat1 = assign1840_e3921;
        locals.var_fn25_calc_iq__vdsat1_dn2 = assign1840_e3921_d_n2;
        locals.var_fn25_calc_iq__vdsat1_dn3 = assign1840_e3921_d_n3;
        locals.var_fn25_calc_iq__vdsat1_dn4 = assign1840_e3921_d_n4;
        locals.var_fn25_calc_iq__vdsat1_dn7 = assign1840_e3921_d_n7;
        locals.var_fn25_calc_iq__vdsat1_dn16 = assign1840_e3921_d_n16;
        locals.var_fn25_calc_iq__vdsat1_dn17 = assign1840_e3921_d_n17;
        locals.var_fn25_calc_iq__vdsat1_rv = 0.0;

        let (assign1850_e3925, assign1850_e3925_d_n2, assign1850_e3925_d_n7, assign1850_e3925_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vgsin, locals.var_fn25_calc_iq__vgsin_dn2, locals.var_fn25_calc_iq__vgsin_dn7, locals.var_fn25_calc_iq__vgsin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vgsin = assign1850_e3925;
        locals.var_fn25_calc_iq__vgsin_dn2 = assign1850_e3925_d_n2;
        locals.var_fn25_calc_iq__vgsin_dn7 = assign1850_e3925_d_n7;
        locals.var_fn25_calc_iq__vgsin_dn16 = assign1850_e3925_d_n16;
        locals.var_fn25_calc_iq__vgsin_rv = 0.0;

        let (assign1860_e3929, assign1860_e3929_d_n16, assign1860_e3929_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vdsfp4, locals.var_vdsfp4_dn16, locals.var_vdsfp4_dn17,)
    } else {
        (locals.var_fn25_calc_iq__vdsin, locals.var_fn25_calc_iq__vdsin_dn16, locals.var_fn25_calc_iq__vdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsin = assign1860_e3929;
        locals.var_fn25_calc_iq__vdsin_dn16 = assign1860_e3929_d_n16;
        locals.var_fn25_calc_iq__vdsin_dn17 = assign1860_e3929_d_n17;
        locals.var_fn25_calc_iq__vdsin_rv = 0.0;

        let (assign1870_e3933,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_fn25_calc_iq__qcbflag,)
    }
};
        locals.var_fn25_calc_iq__qcbflag = assign1870_e3933;
        locals.var_fn25_calc_iq__qcbflag_rv = 0.0;

        let (assign1880_e3937, assign1880_e3937_d_n2, assign1880_e3937_d_n7, assign1880_e3937_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vcin, locals.var_fn25_calc_iq__vcin_dn2, locals.var_fn25_calc_iq__vcin_dn7, locals.var_fn25_calc_iq__vcin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vcin = assign1880_e3937;
        locals.var_fn25_calc_iq__vcin_dn2 = assign1880_e3937_d_n2;
        locals.var_fn25_calc_iq__vcin_dn7 = assign1880_e3937_d_n7;
        locals.var_fn25_calc_iq__vcin_dn16 = assign1880_e3937_d_n16;
        locals.var_fn25_calc_iq__vcin_rv = 0.0;

        let (assign1890_e3941, assign1890_e3941_d_n3, assign1890_e3941_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vbfp4, locals.var_vbfp4_dn3, locals.var_vbfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vbin, locals.var_fn25_calc_iq__vbin_dn3, locals.var_fn25_calc_iq__vbin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vbin = assign1890_e3941;
        locals.var_fn25_calc_iq__vbin_dn3 = assign1890_e3941_d_n3;
        locals.var_fn25_calc_iq__vbin_dn16 = assign1890_e3941_d_n16;
        locals.var_fn25_calc_iq__vbin_rv = 0.0;

        let (assign1900_e3945,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_fn25_calc_iq__qgsflag,)
    }
};
        locals.var_fn25_calc_iq__qgsflag = assign1900_e3945;
        locals.var_fn25_calc_iq__qgsflag_rv = 0.0;

        let (assign1910_e3949, assign1910_e3949_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn25_calc_iq__tambin, locals.var_fn25_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tambin = assign1910_e3949;
        locals.var_fn25_calc_iq__tambin_dn4 = assign1910_e3949_d_n4;
        locals.var_fn25_calc_iq__tambin_rv = 0.0;

        let (assign1920_e3953,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn25_calc_iq__tnomin,)
    }
};
        locals.var_fn25_calc_iq__tnomin = assign1920_e3953;
        locals.var_fn25_calc_iq__tnomin_rv = 0.0;

        let (assign1930_e3957, assign1930_e3957_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn25_calc_iq__phitin, locals.var_fn25_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn25_calc_iq__phitin = assign1930_e3957;
        locals.var_fn25_calc_iq__phitin_dn4 = assign1930_e3957_d_n4;
        locals.var_fn25_calc_iq__phitin_rv = 0.0;

        let (assign1940_e3961,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn25_calc_iq__w,)
    }
};
        locals.var_fn25_calc_iq__w = assign1940_e3961;
        locals.var_fn25_calc_iq__w_rv = 0.0;

        let (assign1950_e3965,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p233,)
    } else {
        (locals.var_fn25_calc_iq__lin,)
    }
};
        locals.var_fn25_calc_iq__lin = assign1950_e3965;
        locals.var_fn25_calc_iq__lin_rv = 0.0;

        let (assign1960_e3969, assign1960_e3969_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_cgfp4t, locals.var_cgfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cgin, locals.var_fn25_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn25_calc_iq__cgin = assign1960_e3969;
        locals.var_fn25_calc_iq__cgin_dn4 = assign1960_e3969_d_n4;
        locals.var_fn25_calc_iq__cgin_rv = 0.0;

        let (assign1970_e3973,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p238,)
    } else {
        (locals.var_fn25_calc_iq__cs,)
    }
};
        locals.var_fn25_calc_iq__cs = assign1970_e3973;
        locals.var_fn25_calc_iq__cs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1980_e3977, assign1980_e3977_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_ccfp4t, locals.var_ccfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cc, locals.var_fn25_calc_iq__cc_dn4,)
    }
};
        locals.var_fn25_calc_iq__cc = assign1980_e3977;
        locals.var_fn25_calc_iq__cc_dn4 = assign1980_e3977_d_n4;
        locals.var_fn25_calc_iq__cc_rv = 0.0;

        let (assign1990_e3981, assign1990_e3981_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_cbfp4t, locals.var_cbfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cb, locals.var_fn25_calc_iq__cb_dn4,)
    }
};
        locals.var_fn25_calc_iq__cb = assign1990_e3981;
        locals.var_fn25_calc_iq__cb_dn4 = assign1990_e3981_d_n4;
        locals.var_fn25_calc_iq__cb_rv = 0.0;

        let (assign2000_e3985,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p234,)
    } else {
        (locals.var_fn25_calc_iq__vto,)
    }
};
        locals.var_fn25_calc_iq__vto = assign2000_e3985;
        locals.var_fn25_calc_iq__vto_rv = 0.0;

        let (assign2010_e3989,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p248,)
    } else {
        (locals.var_fn25_calc_iq__ss,)
    }
};
        locals.var_fn25_calc_iq__ss = assign2010_e3989;
        locals.var_fn25_calc_iq__ss_rv = 0.0;

        let (assign2020_e3993,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_fn25_calc_iq__delta1,)
    }
};
        locals.var_fn25_calc_iq__delta1 = assign2020_e3993;
        locals.var_fn25_calc_iq__delta1_rv = 0.0;

        let (assign2030_e3997,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn25_calc_iq__delta2,)
    }
};
        locals.var_fn25_calc_iq__delta2 = assign2030_e3997;
        locals.var_fn25_calc_iq__delta2_rv = 0.0;

        let (assign2040_e4001,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p249,)
    } else {
        (locals.var_fn25_calc_iq__nd,)
    }
};
        locals.var_fn25_calc_iq__nd = assign2040_e4001;
        locals.var_fn25_calc_iq__nd_rv = 0.0;

        let (assign2050_e4005,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p253,)
    } else {
        (locals.var_fn25_calc_iq__alpha,)
    }
};
        locals.var_fn25_calc_iq__alpha = assign2050_e4005;
        locals.var_fn25_calc_iq__alpha_rv = 0.0;

        let (assign2060_e4009,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p244,)
    } else {
        (locals.var_fn25_calc_iq__vel0,)
    }
};
        locals.var_fn25_calc_iq__vel0 = assign2060_e4009;
        locals.var_fn25_calc_iq__vel0_rv = 0.0;

        let (assign2070_e4013,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_fn25_calc_iq__mu0,)
    }
};
        locals.var_fn25_calc_iq__mu0 = assign2070_e4013;
        locals.var_fn25_calc_iq__mu0_rv = 0.0;

        let (assign2080_e4017,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_fn25_calc_iq__beta,)
    }
};
        locals.var_fn25_calc_iq__beta = assign2080_e4017;
        locals.var_fn25_calc_iq__beta_rv = 0.0;

        let (assign2090_e4021,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_fn25_calc_iq__mtheta,)
    }
};
        locals.var_fn25_calc_iq__mtheta = assign2090_e4021;
        locals.var_fn25_calc_iq__mtheta_rv = 0.0;

        let (assign2100_e4025,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p251,)
    } else {
        (locals.var_fn25_calc_iq__vtheta,)
    }
};
        locals.var_fn25_calc_iq__vtheta = assign2100_e4025;
        locals.var_fn25_calc_iq__vtheta_rv = 0.0;

        let (assign2110_e4029,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p250,)
    } else {
        (locals.var_fn25_calc_iq__vtzeta,)
    }
};
        locals.var_fn25_calc_iq__vtzeta = assign2110_e4029;
        locals.var_fn25_calc_iq__vtzeta_rv = 0.0;

        let (assign2120_e4033,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn25_calc_iq__dibsat,)
    }
};
        locals.var_fn25_calc_iq__dibsat = assign2120_e4033;
        locals.var_fn25_calc_iq__dibsat_rv = 0.0;

        let (assign2130_e4037,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn25_calc_iq__epsilon,)
    }
};
        locals.var_fn25_calc_iq__epsilon = assign2130_e4037;
        locals.var_fn25_calc_iq__epsilon_rv = 0.0;

        let (assign2140_e4041,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn25_calc_iq__vzeta,)
    }
};
        locals.var_fn25_calc_iq__vzeta = assign2140_e4041;
        locals.var_fn25_calc_iq__vzeta_rv = 0.0;

        let (assign2150_e4045,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn25_calc_iq__lambda,)
    }
};
        locals.var_fn25_calc_iq__lambda = assign2150_e4045;
        locals.var_fn25_calc_iq__lambda_rv = 0.0;

        let (assign2160_e4049,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn25_calc_iq__ngf,)
    }
};
        locals.var_fn25_calc_iq__ngf = assign2160_e4049;
        locals.var_fn25_calc_iq__ngf_rv = 0.0;

        let (assign2170_e4053,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn25_calc_iq__type,)
    }
};
        locals.var_fn25_calc_iq__type = assign2170_e4053;
        locals.var_fn25_calc_iq__type_rv = 0.0;

        let (assign2180_e4057,) = {
    if (locals.var_guard24 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn25_calc_iq__trapfracdl,)
    }
};
        locals.var_fn25_calc_iq__trapfracdl = assign2180_e4057;
        locals.var_fn25_calc_iq__trapfracdl_rv = 0.0;

        let (assign2190_e4061, assign2190_e4061_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__alpha_phit, locals.var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn25_calc_iq__alpha_phit = assign2190_e4061;
        locals.var_fn25_calc_iq__alpha_phit_dn4 = assign2190_e4061_d_n4;
        locals.var_fn25_calc_iq__alpha_phit_rv = 0.0;

        let (assign2200_e4065, assign2200_e4065_d_n16, assign2200_e4065_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__delta, locals.var_fn25_calc_iq__delta_dn16, locals.var_fn25_calc_iq__delta_dn17,)
    }
};
        locals.var_fn25_calc_iq__delta = assign2200_e4065;
        locals.var_fn25_calc_iq__delta_dn16 = assign2200_e4065_d_n16;
        locals.var_fn25_calc_iq__delta_dn17 = assign2200_e4065_d_n17;
        locals.var_fn25_calc_iq__delta_rv = 0.0;

        let (assign2210_e4069, assign2210_e4069_d_n4, assign2210_e4069_d_n16, assign2210_e4069_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__n, locals.var_fn25_calc_iq__n_dn4, locals.var_fn25_calc_iq__n_dn16, locals.var_fn25_calc_iq__n_dn17,)
    }
};
        locals.var_fn25_calc_iq__n = assign2210_e4069;
        locals.var_fn25_calc_iq__n_dn4 = assign2210_e4069_d_n4;
        locals.var_fn25_calc_iq__n_dn16 = assign2210_e4069_d_n16;
        locals.var_fn25_calc_iq__n_dn17 = assign2210_e4069_d_n17;
        locals.var_fn25_calc_iq__n_rv = 0.0;

        let (assign2220_e4073, assign2220_e4073_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vtof, locals.var_fn25_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn25_calc_iq__vtof = assign2220_e4073;
        locals.var_fn25_calc_iq__vtof_dn4 = assign2220_e4073_d_n4;
        locals.var_fn25_calc_iq__vtof_rv = 0.0;

        let (assign2230_e4077, assign2230_e4077_d_n16, assign2230_e4077_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2230_e4077;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2230_e4077_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2230_e4077_d_n17;
        locals.var_fn25_calc_iq__vsatdibl_rv = 0.0;

        let (assign2240_e4081, assign2240_e4081_d_n2, assign2240_e4081_d_n3, assign2240_e4081_d_n4, assign2240_e4081_d_n7, assign2240_e4081_d_n16, assign2240_e4081_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign2240_e4081;
        locals.var_fn25_calc_iq__ffs_dn2 = assign2240_e4081_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign2240_e4081_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign2240_e4081_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign2240_e4081_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign2240_e4081_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign2240_e4081_d_n17;
        locals.var_fn25_calc_iq__ffs_rv = 0.0;

        let (assign2250_e4085, assign2250_e4085_d_n4, assign2250_e4085_d_n16, assign2250_e4085_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit, locals.var_fn25_calc_iq__two_n_phit_dn4, locals.var_fn25_calc_iq__two_n_phit_dn16, locals.var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit = assign2250_e4085;
        locals.var_fn25_calc_iq__two_n_phit_dn4 = assign2250_e4085_d_n4;
        locals.var_fn25_calc_iq__two_n_phit_dn16 = assign2250_e4085_d_n16;
        locals.var_fn25_calc_iq__two_n_phit_dn17 = assign2250_e4085_d_n17;
        locals.var_fn25_calc_iq__two_n_phit_rv = 0.0;

        let (assign2260_e4089, assign2260_e4089_d_n4, assign2260_e4089_d_n16, assign2260_e4089_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qref, locals.var_fn25_calc_iq__qref_dn4, locals.var_fn25_calc_iq__qref_dn16, locals.var_fn25_calc_iq__qref_dn17,)
    }
};
        locals.var_fn25_calc_iq__qref = assign2260_e4089;
        locals.var_fn25_calc_iq__qref_dn4 = assign2260_e4089_d_n4;
        locals.var_fn25_calc_iq__qref_dn16 = assign2260_e4089_d_n16;
        locals.var_fn25_calc_iq__qref_dn17 = assign2260_e4089_d_n17;
        locals.var_fn25_calc_iq__qref_rv = 0.0;

        let (assign2270_e4093, assign2270_e4093_d_n2, assign2270_e4093_d_n3, assign2270_e4093_d_n4, assign2270_e4093_d_n7, assign2270_e4093_d_n16, assign2270_e4093_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etas, locals.var_fn25_calc_iq__etas_dn2, locals.var_fn25_calc_iq__etas_dn3, locals.var_fn25_calc_iq__etas_dn4, locals.var_fn25_calc_iq__etas_dn7, locals.var_fn25_calc_iq__etas_dn16, locals.var_fn25_calc_iq__etas_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas = assign2270_e4093;
        locals.var_fn25_calc_iq__etas_dn2 = assign2270_e4093_d_n2;
        locals.var_fn25_calc_iq__etas_dn3 = assign2270_e4093_d_n3;
        locals.var_fn25_calc_iq__etas_dn4 = assign2270_e4093_d_n4;
        locals.var_fn25_calc_iq__etas_dn7 = assign2270_e4093_d_n7;
        locals.var_fn25_calc_iq__etas_dn16 = assign2270_e4093_d_n16;
        locals.var_fn25_calc_iq__etas_dn17 = assign2270_e4093_d_n17;
        locals.var_fn25_calc_iq__etas_rv = 0.0;

        let (assign2280_e4097, assign2280_e4097_d_n2, assign2280_e4097_d_n3, assign2280_e4097_d_n4, assign2280_e4097_d_n7, assign2280_e4097_d_n16, assign2280_e4097_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign2280_e4097;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign2280_e4097_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign2280_e4097_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign2280_e4097_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign2280_e4097_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign2280_e4097_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign2280_e4097_d_n17;
        locals.var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign2290_e4101, assign2290_e4101_d_n2, assign2290_e4101_d_n3, assign2290_e4101_d_n4, assign2290_e4101_d_n7, assign2290_e4101_d_n16, assign2290_e4101_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__muf, locals.var_fn25_calc_iq__muf_dn2, locals.var_fn25_calc_iq__muf_dn3, locals.var_fn25_calc_iq__muf_dn4, locals.var_fn25_calc_iq__muf_dn7, locals.var_fn25_calc_iq__muf_dn16, locals.var_fn25_calc_iq__muf_dn17,)
    }
};
        locals.var_fn25_calc_iq__muf = assign2290_e4101;
        locals.var_fn25_calc_iq__muf_dn2 = assign2290_e4101_d_n2;
        locals.var_fn25_calc_iq__muf_dn3 = assign2290_e4101_d_n3;
        locals.var_fn25_calc_iq__muf_dn4 = assign2290_e4101_d_n4;
        locals.var_fn25_calc_iq__muf_dn7 = assign2290_e4101_d_n7;
        locals.var_fn25_calc_iq__muf_dn16 = assign2290_e4101_d_n16;
        locals.var_fn25_calc_iq__muf_dn17 = assign2290_e4101_d_n17;
        locals.var_fn25_calc_iq__muf_rv = 0.0;

        let (assign2300_e4105, assign2300_e4105_d_n2, assign2300_e4105_d_n3, assign2300_e4105_d_n4, assign2300_e4105_d_n7, assign2300_e4105_d_n16, assign2300_e4105_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vx, locals.var_fn25_calc_iq__vx_dn2, locals.var_fn25_calc_iq__vx_dn3, locals.var_fn25_calc_iq__vx_dn4, locals.var_fn25_calc_iq__vx_dn7, locals.var_fn25_calc_iq__vx_dn16, locals.var_fn25_calc_iq__vx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vx = assign2300_e4105;
        locals.var_fn25_calc_iq__vx_dn2 = assign2300_e4105_d_n2;
        locals.var_fn25_calc_iq__vx_dn3 = assign2300_e4105_d_n3;
        locals.var_fn25_calc_iq__vx_dn4 = assign2300_e4105_d_n4;
        locals.var_fn25_calc_iq__vx_dn7 = assign2300_e4105_d_n7;
        locals.var_fn25_calc_iq__vx_dn16 = assign2300_e4105_d_n16;
        locals.var_fn25_calc_iq__vx_dn17 = assign2300_e4105_d_n17;
        locals.var_fn25_calc_iq__vx_rv = 0.0;

        let (assign2320_e4113, assign2320_e4113_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__n0, locals.var_fn25_calc_iq__n0_dn4,)
    }
};
        locals.var_fn25_calc_iq__n0 = assign2320_e4113;
        locals.var_fn25_calc_iq__n0_dn4 = assign2320_e4113_d_n4;
        locals.var_fn25_calc_iq__n0_rv = 0.0;

        let (assign2330_e4117, assign2330_e4117_d_n2, assign2330_e4117_d_n4, assign2330_e4117_d_n7, assign2330_e4117_d_n16, assign2330_e4117_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign2330_e4117;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign2330_e4117_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign2330_e4117_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign2330_e4117_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign2330_e4117_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign2330_e4117_d_n17;
        locals.var_fn25_calc_iq__ffs0_rv = 0.0;

        let (assign2340_e4121, assign2340_e4121_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit0, locals.var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit0 = assign2340_e4121;
        locals.var_fn25_calc_iq__two_n_phit0_dn4 = assign2340_e4121_d_n4;
        locals.var_fn25_calc_iq__two_n_phit0_rv = 0.0;

        let (assign2350_e4125, assign2350_e4125_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qref0, locals.var_fn25_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn25_calc_iq__qref0 = assign2350_e4125;
        locals.var_fn25_calc_iq__qref0_dn4 = assign2350_e4125_d_n4;
        locals.var_fn25_calc_iq__qref0_rv = 0.0;

        let (assign2360_e4129, assign2360_e4129_d_n2, assign2360_e4129_d_n4, assign2360_e4129_d_n7, assign2360_e4129_d_n16, assign2360_e4129_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etas0, locals.var_fn25_calc_iq__etas0_dn2, locals.var_fn25_calc_iq__etas0_dn4, locals.var_fn25_calc_iq__etas0_dn7, locals.var_fn25_calc_iq__etas0_dn16, locals.var_fn25_calc_iq__etas0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas0 = assign2360_e4129;
        locals.var_fn25_calc_iq__etas0_dn2 = assign2360_e4129_d_n2;
        locals.var_fn25_calc_iq__etas0_dn4 = assign2360_e4129_d_n4;
        locals.var_fn25_calc_iq__etas0_dn7 = assign2360_e4129_d_n7;
        locals.var_fn25_calc_iq__etas0_dn16 = assign2360_e4129_d_n16;
        locals.var_fn25_calc_iq__etas0_dn17 = assign2360_e4129_d_n17;
        locals.var_fn25_calc_iq__etas0_rv = 0.0;

        let (assign2370_e4133, assign2370_e4133_d_n2, assign2370_e4133_d_n4, assign2370_e4133_d_n7, assign2370_e4133_d_n16, assign2370_e4133_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign2370_e4133;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign2370_e4133_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign2370_e4133_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign2370_e4133_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign2370_e4133_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign2370_e4133_d_n17;
        locals.var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign2380_e4137, assign2380_e4137_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__muf0, locals.var_fn25_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn25_calc_iq__muf0 = assign2380_e4137;
        locals.var_fn25_calc_iq__muf0_dn4 = assign2380_e4137_d_n4;
        locals.var_fn25_calc_iq__muf0_rv = 0.0;

        let (assign2390_e4141, assign2390_e4141_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vx0, locals.var_fn25_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vx0 = assign2390_e4141;
        locals.var_fn25_calc_iq__vx0_dn4 = assign2390_e4141_d_n4;
        locals.var_fn25_calc_iq__vx0_rv = 0.0;

        let (assign2400_e4145, assign2400_e4145_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__tfacmobin, locals.var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tfacmobin = assign2400_e4145;
        locals.var_fn25_calc_iq__tfacmobin_dn4 = assign2400_e4145_d_n4;
        locals.var_fn25_calc_iq__tfacmobin_rv = 0.0;

        let (assign2410_e4149, assign2410_e4149_d_n2, assign2410_e4149_d_n3, assign2410_e4149_d_n4, assign2410_e4149_d_n7, assign2410_e4149_d_n16, assign2410_e4149_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign2410_e4149;
        locals.var_fn25_calc_iq__ff_dn2 = assign2410_e4149_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign2410_e4149_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign2410_e4149_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign2410_e4149_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign2410_e4149_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign2410_e4149_d_n17;
        locals.var_fn25_calc_iq__ff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        locals: &mut StampLocals,
    ) {
        let (assign2420_e4153, assign2420_e4153_d_n2, assign2420_e4153_d_n3, assign2420_e4153_d_n4, assign2420_e4153_d_n7, assign2420_e4153_d_n16, assign2420_e4153_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__eta, locals.var_fn25_calc_iq__eta_dn2, locals.var_fn25_calc_iq__eta_dn3, locals.var_fn25_calc_iq__eta_dn4, locals.var_fn25_calc_iq__eta_dn7, locals.var_fn25_calc_iq__eta_dn16, locals.var_fn25_calc_iq__eta_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta = assign2420_e4153;
        locals.var_fn25_calc_iq__eta_dn2 = assign2420_e4153_d_n2;
        locals.var_fn25_calc_iq__eta_dn3 = assign2420_e4153_d_n3;
        locals.var_fn25_calc_iq__eta_dn4 = assign2420_e4153_d_n4;
        locals.var_fn25_calc_iq__eta_dn7 = assign2420_e4153_d_n7;
        locals.var_fn25_calc_iq__eta_dn16 = assign2420_e4153_d_n16;
        locals.var_fn25_calc_iq__eta_dn17 = assign2420_e4153_d_n17;
        locals.var_fn25_calc_iq__eta_rv = 0.0;

        let (assign2430_e4157, assign2430_e4157_d_n2, assign2430_e4157_d_n3, assign2430_e4157_d_n4, assign2430_e4157_d_n7, assign2430_e4157_d_n16, assign2430_e4157_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign2430_e4157;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign2430_e4157_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign2430_e4157_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign2430_e4157_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign2430_e4157_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign2430_e4157_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign2430_e4157_d_n17;
        locals.var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign2440_e4161, assign2440_e4161_d_n2, assign2440_e4161_d_n4, assign2440_e4161_d_n7, assign2440_e4161_d_n16, assign2440_e4161_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign2440_e4161;
        locals.var_fn25_calc_iq__ff0_dn2 = assign2440_e4161_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign2440_e4161_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign2440_e4161_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign2440_e4161_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign2440_e4161_d_n17;
        locals.var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign2450_e4165, assign2450_e4165_d_n2, assign2450_e4165_d_n4, assign2450_e4165_d_n7, assign2450_e4165_d_n16, assign2450_e4165_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__eta0, locals.var_fn25_calc_iq__eta0_dn2, locals.var_fn25_calc_iq__eta0_dn4, locals.var_fn25_calc_iq__eta0_dn7, locals.var_fn25_calc_iq__eta0_dn16, locals.var_fn25_calc_iq__eta0_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta0 = assign2450_e4165;
        locals.var_fn25_calc_iq__eta0_dn2 = assign2450_e4165_d_n2;
        locals.var_fn25_calc_iq__eta0_dn4 = assign2450_e4165_d_n4;
        locals.var_fn25_calc_iq__eta0_dn7 = assign2450_e4165_d_n7;
        locals.var_fn25_calc_iq__eta0_dn16 = assign2450_e4165_d_n16;
        locals.var_fn25_calc_iq__eta0_dn17 = assign2450_e4165_d_n17;
        locals.var_fn25_calc_iq__eta0_rv = 0.0;

        let (assign2460_e4169, assign2460_e4169_d_n2, assign2460_e4169_d_n4, assign2460_e4169_d_n7, assign2460_e4169_d_n16, assign2460_e4169_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign2460_e4169;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign2460_e4169_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign2460_e4169_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign2460_e4169_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign2460_e4169_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign2460_e4169_d_n17;
        locals.var_fn25_calc_iq__qinvv0_rv = 0.0;

        let (assign2470_e4173, assign2470_e4173_d_n2, assign2470_e4173_d_n3, assign2470_e4173_d_n4, assign2470_e4173_d_n7, assign2470_e4173_d_n16, assign2470_e4173_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats, locals.var_fn25_calc_iq__vdsats_dn2, locals.var_fn25_calc_iq__vdsats_dn3, locals.var_fn25_calc_iq__vdsats_dn4, locals.var_fn25_calc_iq__vdsats_dn7, locals.var_fn25_calc_iq__vdsats_dn16, locals.var_fn25_calc_iq__vdsats_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats = assign2470_e4173;
        locals.var_fn25_calc_iq__vdsats_dn2 = assign2470_e4173_d_n2;
        locals.var_fn25_calc_iq__vdsats_dn3 = assign2470_e4173_d_n3;
        locals.var_fn25_calc_iq__vdsats_dn4 = assign2470_e4173_d_n4;
        locals.var_fn25_calc_iq__vdsats_dn7 = assign2470_e4173_d_n7;
        locals.var_fn25_calc_iq__vdsats_dn16 = assign2470_e4173_d_n16;
        locals.var_fn25_calc_iq__vdsats_dn17 = assign2470_e4173_d_n17;
        locals.var_fn25_calc_iq__vdsats_rv = 0.0;

        let (assign2480_e4177, assign2480_e4177_d_n2, assign2480_e4177_d_n3, assign2480_e4177_d_n4, assign2480_e4177_d_n7, assign2480_e4177_d_n16, assign2480_e4177_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats1, locals.var_fn25_calc_iq__vdsats1_dn2, locals.var_fn25_calc_iq__vdsats1_dn3, locals.var_fn25_calc_iq__vdsats1_dn4, locals.var_fn25_calc_iq__vdsats1_dn7, locals.var_fn25_calc_iq__vdsats1_dn16, locals.var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats1 = assign2480_e4177;
        locals.var_fn25_calc_iq__vdsats1_dn2 = assign2480_e4177_d_n2;
        locals.var_fn25_calc_iq__vdsats1_dn3 = assign2480_e4177_d_n3;
        locals.var_fn25_calc_iq__vdsats1_dn4 = assign2480_e4177_d_n4;
        locals.var_fn25_calc_iq__vdsats1_dn7 = assign2480_e4177_d_n7;
        locals.var_fn25_calc_iq__vdsats1_dn16 = assign2480_e4177_d_n16;
        locals.var_fn25_calc_iq__vdsats1_dn17 = assign2480_e4177_d_n17;
        locals.var_fn25_calc_iq__vdsats1_rv = 0.0;

        let (assign2490_e4181, assign2490_e4181_d_n2, assign2490_e4181_d_n3, assign2490_e4181_d_n4, assign2490_e4181_d_n7, assign2490_e4181_d_n16, assign2490_e4181_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat, locals.var_fn25_calc_iq__vdsat_dn2, locals.var_fn25_calc_iq__vdsat_dn3, locals.var_fn25_calc_iq__vdsat_dn4, locals.var_fn25_calc_iq__vdsat_dn7, locals.var_fn25_calc_iq__vdsat_dn16, locals.var_fn25_calc_iq__vdsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat = assign2490_e4181;
        locals.var_fn25_calc_iq__vdsat_dn2 = assign2490_e4181_d_n2;
        locals.var_fn25_calc_iq__vdsat_dn3 = assign2490_e4181_d_n3;
        locals.var_fn25_calc_iq__vdsat_dn4 = assign2490_e4181_d_n4;
        locals.var_fn25_calc_iq__vdsat_dn7 = assign2490_e4181_d_n7;
        locals.var_fn25_calc_iq__vdsat_dn16 = assign2490_e4181_d_n16;
        locals.var_fn25_calc_iq__vdsat_dn17 = assign2490_e4181_d_n17;
        locals.var_fn25_calc_iq__vdsat_rv = 0.0;

        let (assign2500_e4185, assign2500_e4185_d_n2, assign2500_e4185_d_n3, assign2500_e4185_d_n4, assign2500_e4185_d_n7, assign2500_e4185_d_n16, assign2500_e4185_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fsd, locals.var_fn25_calc_iq__fsd_dn2, locals.var_fn25_calc_iq__fsd_dn3, locals.var_fn25_calc_iq__fsd_dn4, locals.var_fn25_calc_iq__fsd_dn7, locals.var_fn25_calc_iq__fsd_dn16, locals.var_fn25_calc_iq__fsd_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd = assign2500_e4185;
        locals.var_fn25_calc_iq__fsd_dn2 = assign2500_e4185_d_n2;
        locals.var_fn25_calc_iq__fsd_dn3 = assign2500_e4185_d_n3;
        locals.var_fn25_calc_iq__fsd_dn4 = assign2500_e4185_d_n4;
        locals.var_fn25_calc_iq__fsd_dn7 = assign2500_e4185_d_n7;
        locals.var_fn25_calc_iq__fsd_dn16 = assign2500_e4185_d_n16;
        locals.var_fn25_calc_iq__fsd_dn17 = assign2500_e4185_d_n17;
        locals.var_fn25_calc_iq__fsd_rv = 0.0;

        let (assign2510_e4189, assign2510_e4189_d_n2, assign2510_e4189_d_n3, assign2510_e4189_d_n4, assign2510_e4189_d_n7, assign2510_e4189_d_n16, assign2510_e4189_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdx, locals.var_fn25_calc_iq__vdx_dn2, locals.var_fn25_calc_iq__vdx_dn3, locals.var_fn25_calc_iq__vdx_dn4, locals.var_fn25_calc_iq__vdx_dn7, locals.var_fn25_calc_iq__vdx_dn16, locals.var_fn25_calc_iq__vdx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx = assign2510_e4189;
        locals.var_fn25_calc_iq__vdx_dn2 = assign2510_e4189_d_n2;
        locals.var_fn25_calc_iq__vdx_dn3 = assign2510_e4189_d_n3;
        locals.var_fn25_calc_iq__vdx_dn4 = assign2510_e4189_d_n4;
        locals.var_fn25_calc_iq__vdx_dn7 = assign2510_e4189_d_n7;
        locals.var_fn25_calc_iq__vdx_dn16 = assign2510_e4189_d_n16;
        locals.var_fn25_calc_iq__vdx_dn17 = assign2510_e4189_d_n17;
        locals.var_fn25_calc_iq__vdx_rv = 0.0;

        let (assign2520_e4193, assign2520_e4193_d_n2, assign2520_e4193_d_n3, assign2520_e4193_d_n4, assign2520_e4193_d_n7, assign2520_e4193_d_n16, assign2520_e4193_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fds, locals.var_fn25_calc_iq__fds_dn2, locals.var_fn25_calc_iq__fds_dn3, locals.var_fn25_calc_iq__fds_dn4, locals.var_fn25_calc_iq__fds_dn7, locals.var_fn25_calc_iq__fds_dn16, locals.var_fn25_calc_iq__fds_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds = assign2520_e4193;
        locals.var_fn25_calc_iq__fds_dn2 = assign2520_e4193_d_n2;
        locals.var_fn25_calc_iq__fds_dn3 = assign2520_e4193_d_n3;
        locals.var_fn25_calc_iq__fds_dn4 = assign2520_e4193_d_n4;
        locals.var_fn25_calc_iq__fds_dn7 = assign2520_e4193_d_n7;
        locals.var_fn25_calc_iq__fds_dn16 = assign2520_e4193_d_n16;
        locals.var_fn25_calc_iq__fds_dn17 = assign2520_e4193_d_n17;
        locals.var_fn25_calc_iq__fds_rv = 0.0;

        let (assign2530_e4197, assign2530_e4197_d_n2, assign2530_e4197_d_n3, assign2530_e4197_d_n4, assign2530_e4197_d_n7, assign2530_e4197_d_n16, assign2530_e4197_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsx, locals.var_fn25_calc_iq__vsx_dn2, locals.var_fn25_calc_iq__vsx_dn3, locals.var_fn25_calc_iq__vsx_dn4, locals.var_fn25_calc_iq__vsx_dn7, locals.var_fn25_calc_iq__vsx_dn16, locals.var_fn25_calc_iq__vsx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx = assign2530_e4197;
        locals.var_fn25_calc_iq__vsx_dn2 = assign2530_e4197_d_n2;
        locals.var_fn25_calc_iq__vsx_dn3 = assign2530_e4197_d_n3;
        locals.var_fn25_calc_iq__vsx_dn4 = assign2530_e4197_d_n4;
        locals.var_fn25_calc_iq__vsx_dn7 = assign2530_e4197_d_n7;
        locals.var_fn25_calc_iq__vsx_dn16 = assign2530_e4197_d_n16;
        locals.var_fn25_calc_iq__vsx_dn17 = assign2530_e4197_d_n17;
        locals.var_fn25_calc_iq__vsx_rv = 0.0;

        let (assign2540_e4201, assign2540_e4201_d_n2, assign2540_e4201_d_n3, assign2540_e4201_d_n4, assign2540_e4201_d_n7, assign2540_e4201_d_n16, assign2540_e4201_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign2540_e4201;
        locals.var_fn25_calc_iq__ffd_dn2 = assign2540_e4201_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign2540_e4201_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign2540_e4201_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign2540_e4201_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign2540_e4201_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign2540_e4201_d_n17;
        locals.var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign2550_e4205, assign2550_e4205_d_n2, assign2550_e4205_d_n3, assign2550_e4205_d_n4, assign2550_e4205_d_n7, assign2550_e4205_d_n16, assign2550_e4205_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etad, locals.var_fn25_calc_iq__etad_dn2, locals.var_fn25_calc_iq__etad_dn3, locals.var_fn25_calc_iq__etad_dn4, locals.var_fn25_calc_iq__etad_dn7, locals.var_fn25_calc_iq__etad_dn16, locals.var_fn25_calc_iq__etad_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad = assign2550_e4205;
        locals.var_fn25_calc_iq__etad_dn2 = assign2550_e4205_d_n2;
        locals.var_fn25_calc_iq__etad_dn3 = assign2550_e4205_d_n3;
        locals.var_fn25_calc_iq__etad_dn4 = assign2550_e4205_d_n4;
        locals.var_fn25_calc_iq__etad_dn7 = assign2550_e4205_d_n7;
        locals.var_fn25_calc_iq__etad_dn16 = assign2550_e4205_d_n16;
        locals.var_fn25_calc_iq__etad_dn17 = assign2550_e4205_d_n17;
        locals.var_fn25_calc_iq__etad_rv = 0.0;

        let (assign2560_e4209, assign2560_e4209_d_n2, assign2560_e4209_d_n3, assign2560_e4209_d_n4, assign2560_e4209_d_n7, assign2560_e4209_d_n16, assign2560_e4209_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign2560_e4209;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign2560_e4209_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign2560_e4209_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign2560_e4209_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign2560_e4209_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign2560_e4209_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign2560_e4209_d_n17;
        locals.var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign2570_e4213, assign2570_e4213_d_n2, assign2570_e4213_d_n3, assign2570_e4213_d_n4, assign2570_e4213_d_n7, assign2570_e4213_d_n16, assign2570_e4213_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsc, locals.var_fn25_calc_iq__vdsc_dn2, locals.var_fn25_calc_iq__vdsc_dn3, locals.var_fn25_calc_iq__vdsc_dn4, locals.var_fn25_calc_iq__vdsc_dn7, locals.var_fn25_calc_iq__vdsc_dn16, locals.var_fn25_calc_iq__vdsc_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsc = assign2570_e4213;
        locals.var_fn25_calc_iq__vdsc_dn2 = assign2570_e4213_d_n2;
        locals.var_fn25_calc_iq__vdsc_dn3 = assign2570_e4213_d_n3;
        locals.var_fn25_calc_iq__vdsc_dn4 = assign2570_e4213_d_n4;
        locals.var_fn25_calc_iq__vdsc_dn7 = assign2570_e4213_d_n7;
        locals.var_fn25_calc_iq__vdsc_dn16 = assign2570_e4213_d_n16;
        locals.var_fn25_calc_iq__vdsc_dn17 = assign2570_e4213_d_n17;
        locals.var_fn25_calc_iq__vdsc_rv = 0.0;

        let (assign2600_e4225, assign2600_e4225_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats0, locals.var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vdsats0 = assign2600_e4225;
        locals.var_fn25_calc_iq__vdsats0_dn4 = assign2600_e4225_d_n4;
        locals.var_fn25_calc_iq__vdsats0_rv = 0.0;

        let (assign2610_e4229, assign2610_e4229_d_n2, assign2610_e4229_d_n4, assign2610_e4229_d_n7, assign2610_e4229_d_n16, assign2610_e4229_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats10, locals.var_fn25_calc_iq__vdsats10_dn2, locals.var_fn25_calc_iq__vdsats10_dn4, locals.var_fn25_calc_iq__vdsats10_dn7, locals.var_fn25_calc_iq__vdsats10_dn16, locals.var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats10 = assign2610_e4229;
        locals.var_fn25_calc_iq__vdsats10_dn2 = assign2610_e4229_d_n2;
        locals.var_fn25_calc_iq__vdsats10_dn4 = assign2610_e4229_d_n4;
        locals.var_fn25_calc_iq__vdsats10_dn7 = assign2610_e4229_d_n7;
        locals.var_fn25_calc_iq__vdsats10_dn16 = assign2610_e4229_d_n16;
        locals.var_fn25_calc_iq__vdsats10_dn17 = assign2610_e4229_d_n17;
        locals.var_fn25_calc_iq__vdsats10_rv = 0.0;

        let (assign2620_e4233, assign2620_e4233_d_n2, assign2620_e4233_d_n4, assign2620_e4233_d_n7, assign2620_e4233_d_n16, assign2620_e4233_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat10, locals.var_fn25_calc_iq__vdsat10_dn2, locals.var_fn25_calc_iq__vdsat10_dn4, locals.var_fn25_calc_iq__vdsat10_dn7, locals.var_fn25_calc_iq__vdsat10_dn16, locals.var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat10 = assign2620_e4233;
        locals.var_fn25_calc_iq__vdsat10_dn2 = assign2620_e4233_d_n2;
        locals.var_fn25_calc_iq__vdsat10_dn4 = assign2620_e4233_d_n4;
        locals.var_fn25_calc_iq__vdsat10_dn7 = assign2620_e4233_d_n7;
        locals.var_fn25_calc_iq__vdsat10_dn16 = assign2620_e4233_d_n16;
        locals.var_fn25_calc_iq__vdsat10_dn17 = assign2620_e4233_d_n17;
        locals.var_fn25_calc_iq__vdsat10_rv = 0.0;

        let (assign2630_e4237, assign2630_e4237_d_n2, assign2630_e4237_d_n4, assign2630_e4237_d_n7, assign2630_e4237_d_n16, assign2630_e4237_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fsd0, locals.var_fn25_calc_iq__fsd0_dn2, locals.var_fn25_calc_iq__fsd0_dn4, locals.var_fn25_calc_iq__fsd0_dn7, locals.var_fn25_calc_iq__fsd0_dn16, locals.var_fn25_calc_iq__fsd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd0 = assign2630_e4237;
        locals.var_fn25_calc_iq__fsd0_dn2 = assign2630_e4237_d_n2;
        locals.var_fn25_calc_iq__fsd0_dn4 = assign2630_e4237_d_n4;
        locals.var_fn25_calc_iq__fsd0_dn7 = assign2630_e4237_d_n7;
        locals.var_fn25_calc_iq__fsd0_dn16 = assign2630_e4237_d_n16;
        locals.var_fn25_calc_iq__fsd0_dn17 = assign2630_e4237_d_n17;
        locals.var_fn25_calc_iq__fsd0_rv = 0.0;

        let (assign2640_e4241, assign2640_e4241_d_n2, assign2640_e4241_d_n4, assign2640_e4241_d_n7, assign2640_e4241_d_n16, assign2640_e4241_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdx0, locals.var_fn25_calc_iq__vdx0_dn2, locals.var_fn25_calc_iq__vdx0_dn4, locals.var_fn25_calc_iq__vdx0_dn7, locals.var_fn25_calc_iq__vdx0_dn16, locals.var_fn25_calc_iq__vdx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx0 = assign2640_e4241;
        locals.var_fn25_calc_iq__vdx0_dn2 = assign2640_e4241_d_n2;
        locals.var_fn25_calc_iq__vdx0_dn4 = assign2640_e4241_d_n4;
        locals.var_fn25_calc_iq__vdx0_dn7 = assign2640_e4241_d_n7;
        locals.var_fn25_calc_iq__vdx0_dn16 = assign2640_e4241_d_n16;
        locals.var_fn25_calc_iq__vdx0_dn17 = assign2640_e4241_d_n17;
        locals.var_fn25_calc_iq__vdx0_rv = 0.0;

        let (assign2650_e4245, assign2650_e4245_d_n2, assign2650_e4245_d_n4, assign2650_e4245_d_n7, assign2650_e4245_d_n16, assign2650_e4245_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fds0, locals.var_fn25_calc_iq__fds0_dn2, locals.var_fn25_calc_iq__fds0_dn4, locals.var_fn25_calc_iq__fds0_dn7, locals.var_fn25_calc_iq__fds0_dn16, locals.var_fn25_calc_iq__fds0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds0 = assign2650_e4245;
        locals.var_fn25_calc_iq__fds0_dn2 = assign2650_e4245_d_n2;
        locals.var_fn25_calc_iq__fds0_dn4 = assign2650_e4245_d_n4;
        locals.var_fn25_calc_iq__fds0_dn7 = assign2650_e4245_d_n7;
        locals.var_fn25_calc_iq__fds0_dn16 = assign2650_e4245_d_n16;
        locals.var_fn25_calc_iq__fds0_dn17 = assign2650_e4245_d_n17;
        locals.var_fn25_calc_iq__fds0_rv = 0.0;

        let (assign2660_e4249, assign2660_e4249_d_n2, assign2660_e4249_d_n4, assign2660_e4249_d_n7, assign2660_e4249_d_n16, assign2660_e4249_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsx0, locals.var_fn25_calc_iq__vsx0_dn2, locals.var_fn25_calc_iq__vsx0_dn4, locals.var_fn25_calc_iq__vsx0_dn7, locals.var_fn25_calc_iq__vsx0_dn16, locals.var_fn25_calc_iq__vsx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx0 = assign2660_e4249;
        locals.var_fn25_calc_iq__vsx0_dn2 = assign2660_e4249_d_n2;
        locals.var_fn25_calc_iq__vsx0_dn4 = assign2660_e4249_d_n4;
        locals.var_fn25_calc_iq__vsx0_dn7 = assign2660_e4249_d_n7;
        locals.var_fn25_calc_iq__vsx0_dn16 = assign2660_e4249_d_n16;
        locals.var_fn25_calc_iq__vsx0_dn17 = assign2660_e4249_d_n17;
        locals.var_fn25_calc_iq__vsx0_rv = 0.0;

        let (assign2670_e4253, assign2670_e4253_d_n2, assign2670_e4253_d_n4, assign2670_e4253_d_n7, assign2670_e4253_d_n16, assign2670_e4253_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign2670_e4253;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign2670_e4253_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign2670_e4253_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign2670_e4253_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign2670_e4253_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign2670_e4253_d_n17;
        locals.var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign2680_e4257, assign2680_e4257_d_n2, assign2680_e4257_d_n4, assign2680_e4257_d_n7, assign2680_e4257_d_n16, assign2680_e4257_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etad0, locals.var_fn25_calc_iq__etad0_dn2, locals.var_fn25_calc_iq__etad0_dn4, locals.var_fn25_calc_iq__etad0_dn7, locals.var_fn25_calc_iq__etad0_dn16, locals.var_fn25_calc_iq__etad0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad0 = assign2680_e4257;
        locals.var_fn25_calc_iq__etad0_dn2 = assign2680_e4257_d_n2;
        locals.var_fn25_calc_iq__etad0_dn4 = assign2680_e4257_d_n4;
        locals.var_fn25_calc_iq__etad0_dn7 = assign2680_e4257_d_n7;
        locals.var_fn25_calc_iq__etad0_dn16 = assign2680_e4257_d_n16;
        locals.var_fn25_calc_iq__etad0_dn17 = assign2680_e4257_d_n17;
        locals.var_fn25_calc_iq__etad0_rv = 0.0;

        let (assign2690_e4261, assign2690_e4261_d_n2, assign2690_e4261_d_n4, assign2690_e4261_d_n7, assign2690_e4261_d_n16, assign2690_e4261_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign2690_e4261;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign2690_e4261_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign2690_e4261_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign2690_e4261_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign2690_e4261_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign2690_e4261_d_n17;
        locals.var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign2700_e4265, assign2700_e4265_d_n2, assign2700_e4265_d_n4, assign2700_e4265_d_n7, assign2700_e4265_d_n16, assign2700_e4265_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs2, locals.var_fn25_calc_iq__qs2_dn2, locals.var_fn25_calc_iq__qs2_dn4, locals.var_fn25_calc_iq__qs2_dn7, locals.var_fn25_calc_iq__qs2_dn16, locals.var_fn25_calc_iq__qs2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs2 = assign2700_e4265;
        locals.var_fn25_calc_iq__qs2_dn2 = assign2700_e4265_d_n2;
        locals.var_fn25_calc_iq__qs2_dn4 = assign2700_e4265_d_n4;
        locals.var_fn25_calc_iq__qs2_dn7 = assign2700_e4265_d_n7;
        locals.var_fn25_calc_iq__qs2_dn16 = assign2700_e4265_d_n16;
        locals.var_fn25_calc_iq__qs2_dn17 = assign2700_e4265_d_n17;
        locals.var_fn25_calc_iq__qs2_rv = 0.0;

        let (assign2710_e4269, assign2710_e4269_d_n2, assign2710_e4269_d_n4, assign2710_e4269_d_n7, assign2710_e4269_d_n16, assign2710_e4269_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs3, locals.var_fn25_calc_iq__qs3_dn2, locals.var_fn25_calc_iq__qs3_dn4, locals.var_fn25_calc_iq__qs3_dn7, locals.var_fn25_calc_iq__qs3_dn16, locals.var_fn25_calc_iq__qs3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs3 = assign2710_e4269;
        locals.var_fn25_calc_iq__qs3_dn2 = assign2710_e4269_d_n2;
        locals.var_fn25_calc_iq__qs3_dn4 = assign2710_e4269_d_n4;
        locals.var_fn25_calc_iq__qs3_dn7 = assign2710_e4269_d_n7;
        locals.var_fn25_calc_iq__qs3_dn16 = assign2710_e4269_d_n16;
        locals.var_fn25_calc_iq__qs3_dn17 = assign2710_e4269_d_n17;
        locals.var_fn25_calc_iq__qs3_rv = 0.0;

        let (assign2720_e4273, assign2720_e4273_d_n2, assign2720_e4273_d_n4, assign2720_e4273_d_n7, assign2720_e4273_d_n16, assign2720_e4273_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd2, locals.var_fn25_calc_iq__qd2_dn2, locals.var_fn25_calc_iq__qd2_dn4, locals.var_fn25_calc_iq__qd2_dn7, locals.var_fn25_calc_iq__qd2_dn16, locals.var_fn25_calc_iq__qd2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd2 = assign2720_e4273;
        locals.var_fn25_calc_iq__qd2_dn2 = assign2720_e4273_d_n2;
        locals.var_fn25_calc_iq__qd2_dn4 = assign2720_e4273_d_n4;
        locals.var_fn25_calc_iq__qd2_dn7 = assign2720_e4273_d_n7;
        locals.var_fn25_calc_iq__qd2_dn16 = assign2720_e4273_d_n16;
        locals.var_fn25_calc_iq__qd2_dn17 = assign2720_e4273_d_n17;
        locals.var_fn25_calc_iq__qd2_rv = 0.0;

        let (assign2730_e4277, assign2730_e4277_d_n2, assign2730_e4277_d_n4, assign2730_e4277_d_n7, assign2730_e4277_d_n16, assign2730_e4277_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd3, locals.var_fn25_calc_iq__qd3_dn2, locals.var_fn25_calc_iq__qd3_dn4, locals.var_fn25_calc_iq__qd3_dn7, locals.var_fn25_calc_iq__qd3_dn16, locals.var_fn25_calc_iq__qd3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd3 = assign2730_e4277;
        locals.var_fn25_calc_iq__qd3_dn2 = assign2730_e4277_d_n2;
        locals.var_fn25_calc_iq__qd3_dn4 = assign2730_e4277_d_n4;
        locals.var_fn25_calc_iq__qd3_dn7 = assign2730_e4277_d_n7;
        locals.var_fn25_calc_iq__qd3_dn16 = assign2730_e4277_d_n16;
        locals.var_fn25_calc_iq__qd3_dn17 = assign2730_e4277_d_n17;
        locals.var_fn25_calc_iq__qd3_rv = 0.0;

        let (assign2740_e4281, assign2740_e4281_d_n2, assign2740_e4281_d_n4, assign2740_e4281_d_n7, assign2740_e4281_d_n16, assign2740_e4281_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsqd, locals.var_fn25_calc_iq__qsqd_dn2, locals.var_fn25_calc_iq__qsqd_dn4, locals.var_fn25_calc_iq__qsqd_dn7, locals.var_fn25_calc_iq__qsqd_dn16, locals.var_fn25_calc_iq__qsqd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsqd = assign2740_e4281;
        locals.var_fn25_calc_iq__qsqd_dn2 = assign2740_e4281_d_n2;
        locals.var_fn25_calc_iq__qsqd_dn4 = assign2740_e4281_d_n4;
        locals.var_fn25_calc_iq__qsqd_dn7 = assign2740_e4281_d_n7;
        locals.var_fn25_calc_iq__qsqd_dn16 = assign2740_e4281_d_n16;
        locals.var_fn25_calc_iq__qsqd_dn17 = assign2740_e4281_d_n17;
        locals.var_fn25_calc_iq__qsqd_rv = 0.0;

        let (assign2750_e4285, assign2750_e4285_d_n2, assign2750_e4285_d_n4, assign2750_e4285_d_n7, assign2750_e4285_d_n16, assign2750_e4285_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvdd, locals.var_fn25_calc_iq__qinvdd_dn2, locals.var_fn25_calc_iq__qinvdd_dn4, locals.var_fn25_calc_iq__qinvdd_dn7, locals.var_fn25_calc_iq__qinvdd_dn16, locals.var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvdd = assign2750_e4285;
        locals.var_fn25_calc_iq__qinvdd_dn2 = assign2750_e4285_d_n2;
        locals.var_fn25_calc_iq__qinvdd_dn4 = assign2750_e4285_d_n4;
        locals.var_fn25_calc_iq__qinvdd_dn7 = assign2750_e4285_d_n7;
        locals.var_fn25_calc_iq__qinvdd_dn16 = assign2750_e4285_d_n16;
        locals.var_fn25_calc_iq__qinvdd_dn17 = assign2750_e4285_d_n17;
        locals.var_fn25_calc_iq__qinvdd_rv = 0.0;

        let (assign2760_e4289, assign2760_e4289_d_n2, assign2760_e4289_d_n4, assign2760_e4289_d_n7, assign2760_e4289_d_n16, assign2760_e4289_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd1 = assign2760_e4289;
        locals.var_fn25_calc_iq__qd1_dn2 = assign2760_e4289_d_n2;
        locals.var_fn25_calc_iq__qd1_dn4 = assign2760_e4289_d_n4;
        locals.var_fn25_calc_iq__qd1_dn7 = assign2760_e4289_d_n7;
        locals.var_fn25_calc_iq__qd1_dn16 = assign2760_e4289_d_n16;
        locals.var_fn25_calc_iq__qd1_dn17 = assign2760_e4289_d_n17;
        locals.var_fn25_calc_iq__qd1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2770_e4293, assign2770_e4293_d_n2, assign2770_e4293_d_n4, assign2770_e4293_d_n7, assign2770_e4293_d_n16, assign2770_e4293_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs, locals.var_fn25_calc_iq__qs_dn2, locals.var_fn25_calc_iq__qs_dn4, locals.var_fn25_calc_iq__qs_dn7, locals.var_fn25_calc_iq__qs_dn16, locals.var_fn25_calc_iq__qs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs = assign2770_e4293;
        locals.var_fn25_calc_iq__qs_dn2 = assign2770_e4293_d_n2;
        locals.var_fn25_calc_iq__qs_dn4 = assign2770_e4293_d_n4;
        locals.var_fn25_calc_iq__qs_dn7 = assign2770_e4293_d_n7;
        locals.var_fn25_calc_iq__qs_dn16 = assign2770_e4293_d_n16;
        locals.var_fn25_calc_iq__qs_dn17 = assign2770_e4293_d_n17;
        locals.var_fn25_calc_iq__qs_rv = 0.0;

        let (assign2780_e4297, assign2780_e4297_d_n2, assign2780_e4297_d_n4, assign2780_e4297_d_n7, assign2780_e4297_d_n16, assign2780_e4297_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd, locals.var_fn25_calc_iq__qd_dn2, locals.var_fn25_calc_iq__qd_dn4, locals.var_fn25_calc_iq__qd_dn7, locals.var_fn25_calc_iq__qd_dn16, locals.var_fn25_calc_iq__qd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd = assign2780_e4297;
        locals.var_fn25_calc_iq__qd_dn2 = assign2780_e4297_d_n2;
        locals.var_fn25_calc_iq__qd_dn4 = assign2780_e4297_d_n4;
        locals.var_fn25_calc_iq__qd_dn7 = assign2780_e4297_d_n7;
        locals.var_fn25_calc_iq__qd_dn16 = assign2780_e4297_d_n16;
        locals.var_fn25_calc_iq__qd_dn17 = assign2780_e4297_d_n17;
        locals.var_fn25_calc_iq__qd_rv = 0.0;

        let (assign2790_e4301, assign2790_e4301_d_n2, assign2790_e4301_d_n4, assign2790_e4301_d_n7, assign2790_e4301_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16,)
    }
};
        locals.var_fn25_calc_iq__etac = assign2790_e4301;
        locals.var_fn25_calc_iq__etac_dn2 = assign2790_e4301_d_n2;
        locals.var_fn25_calc_iq__etac_dn4 = assign2790_e4301_d_n4;
        locals.var_fn25_calc_iq__etac_dn7 = assign2790_e4301_d_n7;
        locals.var_fn25_calc_iq__etac_dn16 = assign2790_e4301_d_n16;
        locals.var_fn25_calc_iq__etac_rv = 0.0;

        let (assign2800_e4305, assign2800_e4305_d_n3, assign2800_e4305_d_n4, assign2800_e4305_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etab, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, locals.var_fn25_calc_iq__etab_dn16,)
    }
};
        locals.var_fn25_calc_iq__etab = assign2800_e4305;
        locals.var_fn25_calc_iq__etab_dn3 = assign2800_e4305_d_n3;
        locals.var_fn25_calc_iq__etab_dn4 = assign2800_e4305_d_n4;
        locals.var_fn25_calc_iq__etab_dn16 = assign2800_e4305_d_n16;
        locals.var_fn25_calc_iq__etab_rv = 0.0;

        let (assign2810_e4309, assign2810_e4309_d_n2, assign2810_e4309_d_n4, assign2810_e4309_d_n7, assign2810_e4309_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16,)
    }
};
        locals.var_fn25_calc_iq__etags = assign2810_e4309;
        locals.var_fn25_calc_iq__etags_dn2 = assign2810_e4309_d_n2;
        locals.var_fn25_calc_iq__etags_dn4 = assign2810_e4309_d_n4;
        locals.var_fn25_calc_iq__etags_dn7 = assign2810_e4309_d_n7;
        locals.var_fn25_calc_iq__etags_dn16 = assign2810_e4309_d_n16;
        locals.var_fn25_calc_iq__etags_rv = 0.0;

        let (assign2820_e4313, assign2820_e4313_d_n2, assign2820_e4313_d_n3, assign2820_e4313_d_n4, assign2820_e4313_d_n7, assign2820_e4313_d_n16, assign2820_e4313_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign2820_e4313;
        locals.var_fn25_calc_iq__exparg_dn2 = assign2820_e4313_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign2820_e4313_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign2820_e4313_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign2820_e4313_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign2820_e4313_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign2820_e4313_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign2830_e4317, assign2830_e4317_d_n2, assign2830_e4317_d_n3, assign2830_e4317_d_n4, assign2830_e4317_d_n7, assign2830_e4317_d_n16, assign2830_e4317_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign2830_e4317;
        locals.var_fn25_calc_iq__myarg_dn2 = assign2830_e4317_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign2830_e4317_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign2830_e4317_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign2830_e4317_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign2830_e4317_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign2830_e4317_d_n17;
        locals.var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign2840_e4321, assign2840_e4321_d_n16, assign2840_e4321_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__absvdsin, locals.var_fn25_calc_iq__absvdsin_dn16, locals.var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__absvdsin = assign2840_e4321;
        locals.var_fn25_calc_iq__absvdsin_dn16 = assign2840_e4321_d_n16;
        locals.var_fn25_calc_iq__absvdsin_dn17 = assign2840_e4321_d_n17;
        locals.var_fn25_calc_iq__absvdsin_rv = 0.0;

        let (assign2850_e4325, assign2850_e4325_d_n2, assign2850_e4325_d_n7, assign2850_e4325_d_n16, assign2850_e4325_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vgdin, locals.var_fn25_calc_iq__vgdin_dn2, locals.var_fn25_calc_iq__vgdin_dn7, locals.var_fn25_calc_iq__vgdin_dn16, locals.var_fn25_calc_iq__vgdin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vgdin = assign2850_e4325;
        locals.var_fn25_calc_iq__vgdin_dn2 = assign2850_e4325_d_n2;
        locals.var_fn25_calc_iq__vgdin_dn7 = assign2850_e4325_d_n7;
        locals.var_fn25_calc_iq__vgdin_dn16 = assign2850_e4325_d_n16;
        locals.var_fn25_calc_iq__vgdin_dn17 = assign2850_e4325_d_n17;
        locals.var_fn25_calc_iq__vgdin_rv = 0.0;

        let (assign2860_e4329, assign2860_e4329_d_n2, assign2860_e4329_d_n4, assign2860_e4329_d_n7, assign2860_e4329_d_n16, assign2860_e4329_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign2860_e4329;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign2860_e4329_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign2860_e4329_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign2860_e4329_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign2860_e4329_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign2860_e4329_d_n17;
        locals.var_fn25_calc_iq__exparg0_rv = 0.0;

        let (assign2870_e4333, assign2870_e4333_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__myarg0, locals.var_fn25_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn25_calc_iq__myarg0 = assign2870_e4333;
        locals.var_fn25_calc_iq__myarg0_dn4 = assign2870_e4333_d_n4;
        locals.var_fn25_calc_iq__myarg0_rv = 0.0;

        let (assign2880_e4360, assign2880_e4360_d_n16, assign2880_e4360_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign2880_e4342: f64 = (0.001 / p.p53);
                let assign2880_e4344: f64 = (assign2880_e4342 * locals.var_fn25_calc_iq__vdsin);
                let assign2880_e4345: f64 = (assign2880_e4344).tanh();
                let assign2880_e4346: f64 = (locals.var_fn25_calc_iq__vdsin * assign2880_e4345);
                (assign2880_e4346, ((locals.var_fn25_calc_iq__vdsin_dn16 * assign2880_e4345) + (locals.var_fn25_calc_iq__vdsin * ((assign2880_e4342 * locals.var_fn25_calc_iq__vdsin_dn16) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))), ((locals.var_fn25_calc_iq__vdsin_dn17 * assign2880_e4345) + (locals.var_fn25_calc_iq__vdsin * ((assign2880_e4342 * locals.var_fn25_calc_iq__vdsin_dn17) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))),)
            } else {
                let (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign2880_e4352: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin);
                        let assign2880_e4354: f64 = (assign2880_e4352 + p.p53);
                        let assign2880_e4355: f64 = (assign2880_e4354).sqrt();
                        (assign2880_e4355, (((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsin) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin_dn16)) / (2.0 * assign2880_e4355)), (((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsin) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin_dn17)) / (2.0 * assign2880_e4355)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,)
            }
        };
        (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,)
    } else {
        (locals.var_fn25_calc_iq__absvdsin, locals.var_fn25_calc_iq__absvdsin_dn16, locals.var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__absvdsin = assign2880_e4360;
        locals.var_fn25_calc_iq__absvdsin_dn16 = assign2880_e4360_d_n16;
        locals.var_fn25_calc_iq__absvdsin_dn17 = assign2880_e4360_d_n17;
        locals.var_fn25_calc_iq__absvdsin_rv = 0.0;

        let (assign2890_e4366, assign2890_e4366_d_n2, assign2890_e4366_d_n7, assign2890_e4366_d_n16, assign2890_e4366_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2890_e4364: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdsin);
        (assign2890_e4364, locals.var_fn25_calc_iq__vgsin_dn2, locals.var_fn25_calc_iq__vgsin_dn7, (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdsin_dn16), (-locals.var_fn25_calc_iq__vdsin_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vgdin, locals.var_fn25_calc_iq__vgdin_dn2, locals.var_fn25_calc_iq__vgdin_dn7, locals.var_fn25_calc_iq__vgdin_dn16, locals.var_fn25_calc_iq__vgdin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vgdin = assign2890_e4366;
        locals.var_fn25_calc_iq__vgdin_dn2 = assign2890_e4366_d_n2;
        locals.var_fn25_calc_iq__vgdin_dn7 = assign2890_e4366_d_n7;
        locals.var_fn25_calc_iq__vgdin_dn16 = assign2890_e4366_d_n16;
        locals.var_fn25_calc_iq__vgdin_dn17 = assign2890_e4366_d_n17;
        locals.var_fn25_calc_iq__vgdin_rv = 0.0;

        let (assign2900_e4372, assign2900_e4372_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2900_e4370: f64 = (locals.var_fn25_calc_iq__alpha * locals.var_fn25_calc_iq__phitin);
        (assign2900_e4370, (locals.var_fn25_calc_iq__alpha * locals.var_fn25_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn25_calc_iq__alpha_phit, locals.var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn25_calc_iq__alpha_phit = assign2900_e4372;
        locals.var_fn25_calc_iq__alpha_phit_dn4 = assign2900_e4372_d_n4;
        locals.var_fn25_calc_iq__alpha_phit_rv = 0.0;

        let (assign2910_e4384, assign2910_e4384_d_n4, assign2910_e4384_d_n16, assign2910_e4384_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2910_e4377: f64 = (2.302585092994046 * locals.var_fn25_calc_iq__phitin);
        let assign2910_e4378: f64 = (locals.var_fn25_calc_iq__ss / assign2910_e4377);
        let assign2910_e4381: f64 = (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin);
        let assign2910_e4382: f64 = (assign2910_e4378 + assign2910_e4381);
        (assign2910_e4382, (-((locals.var_fn25_calc_iq__ss * (2.302585092994046 * locals.var_fn25_calc_iq__phitin_dn4)) / (assign2910_e4377 * assign2910_e4377))), (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin_dn16), (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin_dn17),)
    } else {
        (locals.var_fn25_calc_iq__n, locals.var_fn25_calc_iq__n_dn4, locals.var_fn25_calc_iq__n_dn16, locals.var_fn25_calc_iq__n_dn17,)
    }
};
        locals.var_fn25_calc_iq__n = assign2910_e4384;
        locals.var_fn25_calc_iq__n_dn4 = assign2910_e4384_d_n4;
        locals.var_fn25_calc_iq__n_dn16 = assign2910_e4384_d_n16;
        locals.var_fn25_calc_iq__n_dn17 = assign2910_e4384_d_n17;
        locals.var_fn25_calc_iq__n_rv = 0.0;

        let (assign2920_e4394, assign2920_e4394_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2920_e4390: f64 = (locals.var_fn25_calc_iq__tambin - locals.var_fn25_calc_iq__tnomin);
        let assign2920_e4391: f64 = (locals.var_fn25_calc_iq__vtzeta * assign2920_e4390);
        let assign2920_e4392: f64 = (locals.var_fn25_calc_iq__vto + assign2920_e4391);
        (assign2920_e4392, (locals.var_fn25_calc_iq__vtzeta * locals.var_fn25_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn25_calc_iq__vtof, locals.var_fn25_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn25_calc_iq__vtof = assign2920_e4394;
        locals.var_fn25_calc_iq__vtof_dn4 = assign2920_e4394_d_n4;
        locals.var_fn25_calc_iq__vtof_rv = 0.0;

        let (assign2930_e4402, assign2930_e4402_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2930_e4398: f64 = (locals.var_fn25_calc_iq__tambin / locals.var_fn25_calc_iq__tnomin);
        let assign2930_e4400: f64 = (assign2930_e4398).powf(locals.var_fn25_calc_iq__epsilon);
        (assign2930_e4400, if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn25_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__epsilon * ((assign2930_e4398).powf(locals.var_fn25_calc_iq__epsilon - 1.0) * (locals.var_fn25_calc_iq__tambin_dn4 / locals.var_fn25_calc_iq__tnomin))) } } else { (assign2930_e4400 * (locals.var_fn25_calc_iq__epsilon * ((locals.var_fn25_calc_iq__tambin_dn4 / locals.var_fn25_calc_iq__tnomin) / assign2930_e4398))) },)
    } else {
        (locals.var_fn25_calc_iq__tfacmobin, locals.var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tfacmobin = assign2930_e4402;
        locals.var_fn25_calc_iq__tfacmobin_dn4 = assign2930_e4402_d_n4;
        locals.var_fn25_calc_iq__tfacmobin_rv = 0.0;

        let assign2940_e4405: f64 = if locals.var_fn25_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign2940_e4405;
        locals.var_guard26_rv = 0.0;

        let (assign2950_e4423, assign2950_e4423_d_n16, assign2950_e4423_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard26 != 0.0)) {
        let assign2950_e4413: f64 = (locals.var_fn25_calc_iq__absvdsin / locals.var_fn25_calc_iq__dibsat);
        let assign2950_e4415: f64 = (assign2950_e4413).powf(locals.var_fn25_calc_iq__beta);
        let assign2950_e4416: f64 = (1.0 + assign2950_e4415);
        let assign2950_e4419: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign2950_e4420: f64 = (assign2950_e4416).powf(assign2950_e4419);
        let assign2950_e4421: f64 = (locals.var_fn25_calc_iq__absvdsin / assign2950_e4420);
        (assign2950_e4421, (((locals.var_fn25_calc_iq__absvdsin_dn16 * assign2950_e4420) - (locals.var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)), (((locals.var_fn25_calc_iq__absvdsin_dn17 * assign2950_e4420) - (locals.var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)),)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2950_e4423;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2950_e4423_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2950_e4423_d_n17;
        locals.var_fn25_calc_iq__vsatdibl_rv = 0.0;

        let (assign2960_e4430, assign2960_e4430_d_n16, assign2960_e4430_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard26 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2960_e4430;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2960_e4430_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2960_e4430_d_n17;
        locals.var_fn25_calc_iq__vsatdibl_rv = 0.0;

        let (assign2970_e4440, assign2970_e4440_d_n16, assign2970_e4440_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2970_e4435: f64 = (locals.var_fn25_calc_iq__vsatdibl * locals.var_fn25_calc_iq__delta2);
        let assign2970_e4436: f64 = (locals.var_fn25_calc_iq__delta1 - assign2970_e4435);
        let assign2970_e4438: f64 = (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin);
        (assign2970_e4438, (((-(locals.var_fn25_calc_iq__vsatdibl_dn16 * locals.var_fn25_calc_iq__delta2)) * locals.var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin_dn16)), (((-(locals.var_fn25_calc_iq__vsatdibl_dn17 * locals.var_fn25_calc_iq__delta2)) * locals.var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__delta, locals.var_fn25_calc_iq__delta_dn16, locals.var_fn25_calc_iq__delta_dn17,)
    }
};
        locals.var_fn25_calc_iq__delta = assign2970_e4440;
        locals.var_fn25_calc_iq__delta_dn16 = assign2970_e4440_d_n16;
        locals.var_fn25_calc_iq__delta_dn17 = assign2970_e4440_d_n17;
        locals.var_fn25_calc_iq__delta_rv = 0.0;

        let (assign2980_e4446, assign2980_e4446_d_n4, assign2980_e4446_d_n16, assign2980_e4446_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2980_e4444: f64 = (locals.var_fn25_calc_iq__vtof - locals.var_fn25_calc_iq__delta);
        (assign2980_e4444, locals.var_fn25_calc_iq__vtof_dn4, (-locals.var_fn25_calc_iq__delta_dn16), (-locals.var_fn25_calc_iq__delta_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vtdibl, locals.var_fn25_calc_iq__vtdibl_dn4, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vtdibl = assign2980_e4446;
        locals.var_fn25_calc_iq__vtdibl_dn4 = assign2980_e4446_d_n4;
        locals.var_fn25_calc_iq__vtdibl_dn16 = assign2980_e4446_d_n16;
        locals.var_fn25_calc_iq__vtdibl_dn17 = assign2980_e4446_d_n17;
        locals.var_fn25_calc_iq__vtdibl_rv = 0.0;

        let (assign2990_e4454, assign2990_e4454_d_n4, assign2990_e4454_d_n16, assign2990_e4454_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2990_e4450: f64 = (2.0 * locals.var_fn25_calc_iq__n);
        let assign2990_e4452: f64 = (assign2990_e4450 * locals.var_fn25_calc_iq__phitin);
        (assign2990_e4452, (((2.0 * locals.var_fn25_calc_iq__n_dn4) * locals.var_fn25_calc_iq__phitin) + (assign2990_e4450 * locals.var_fn25_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn25_calc_iq__n_dn16) * locals.var_fn25_calc_iq__phitin), ((2.0 * locals.var_fn25_calc_iq__n_dn17) * locals.var_fn25_calc_iq__phitin),)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit, locals.var_fn25_calc_iq__two_n_phit_dn4, locals.var_fn25_calc_iq__two_n_phit_dn16, locals.var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit = assign2990_e4454;
        locals.var_fn25_calc_iq__two_n_phit_dn4 = assign2990_e4454_d_n4;
        locals.var_fn25_calc_iq__two_n_phit_dn16 = assign2990_e4454_d_n16;
        locals.var_fn25_calc_iq__two_n_phit_dn17 = assign2990_e4454_d_n17;
        locals.var_fn25_calc_iq__two_n_phit_rv = 0.0;

        let (assign3000_e4460, assign3000_e4460_d_n4, assign3000_e4460_d_n16, assign3000_e4460_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3000_e4458: f64 = (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit);
        (assign3000_e4458, ((locals.var_fn25_calc_iq__cgin_dn4 * locals.var_fn25_calc_iq__two_n_phit) + (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn4)), (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn16), (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qref, locals.var_fn25_calc_iq__qref_dn4, locals.var_fn25_calc_iq__qref_dn16, locals.var_fn25_calc_iq__qref_dn17,)
    }
};
        locals.var_fn25_calc_iq__qref = assign3000_e4460;
        locals.var_fn25_calc_iq__qref_dn4 = assign3000_e4460_d_n4;
        locals.var_fn25_calc_iq__qref_dn16 = assign3000_e4460_d_n16;
        locals.var_fn25_calc_iq__qref_dn17 = assign3000_e4460_d_n17;
        locals.var_fn25_calc_iq__qref_rv = 0.0;

        let (assign3010_e4470, assign3010_e4470_d_n2, assign3010_e4470_d_n3, assign3010_e4470_d_n4, assign3010_e4470_d_n7, assign3010_e4470_d_n16, assign3010_e4470_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3010_e4465: f64 = (p.p51 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3010_e4467: f64 = (assign3010_e4465 / 2.0);
        let assign3010_e4468: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3010_e4467);
        (assign3010_e4468, 0.0, 0.0, (locals.var_fn25_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn25_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign3010_e4470;
        locals.var_fn25_calc_iq__myarg_dn2 = assign3010_e4470_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign3010_e4470_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign3010_e4470_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign3010_e4470_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign3010_e4470_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign3010_e4470_d_n17;
        locals.var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign3020_e4521, assign3020_e4521_d_n2, assign3020_e4521_d_n3, assign3020_e4521_d_n4, assign3020_e4521_d_n7, assign3020_e4521_d_n16, assign3020_e4521_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3020_e4515, assign3020_e4515_d_n2, assign3020_e4515_d_n7, assign3020_e4515_d_n16, assign3020_e4515_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3020_e4479: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4482: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4485: f64 = (0.001 / p.p53);
                let assign3020_e4488: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4489: f64 = (assign3020_e4485 * assign3020_e4488);
                let assign3020_e4490: f64 = (assign3020_e4489).tanh();
                let assign3020_e4491: f64 = (assign3020_e4482 * assign3020_e4490);
                let assign3020_e4492: f64 = (assign3020_e4479 + assign3020_e4491);
                let assign3020_e4493: f64 = (0.5 * assign3020_e4492);
                (assign3020_e4493, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))),)
            } else {
                let (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3020_e4500: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4503: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4506: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4507: f64 = (assign3020_e4503 * assign3020_e4506);
                        let assign3020_e4509: f64 = (assign3020_e4507 + p.p53);
                        let assign3020_e4510: f64 = (assign3020_e4509).sqrt();
                        let assign3020_e4511: f64 = (assign3020_e4500 + assign3020_e4510);
                        let assign3020_e4512: f64 = (0.5 * assign3020_e4511);
                        (assign3020_e4512, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3020_e4510)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3020_e4510)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3020_e4510)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3020_e4506) + (assign3020_e4503 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3020_e4510)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,)
            }
        };
        let assign3020_e4517: f64 = (assign3020_e4515 - locals.var_fn25_calc_iq__myarg);
        let assign3020_e4519: f64 = (assign3020_e4517 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3020_e4519, ((assign3020_e4515_d_n2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3020_e4517 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((assign3020_e4515_d_n7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n17 - locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3020_e4521;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3020_e4521_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3020_e4521_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3020_e4521_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3020_e4521_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3020_e4521_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3020_e4521_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3030_e4524: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3030_e4524;
        locals.var_guard27_rv = 0.0;

        let (assign3040_e4530, assign3040_e4530_d_n2, assign3040_e4530_d_n3, assign3040_e4530_d_n4, assign3040_e4530_d_n7, assign3040_e4530_d_n16, assign3040_e4530_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard27 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3040_e4530;
        locals.var_fn25_calc_iq__ff_dn2 = assign3040_e4530_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3040_e4530_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3040_e4530_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3040_e4530_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3040_e4530_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3040_e4530_d_n17;
        locals.var_fn25_calc_iq__ff_rv = 0.0;

        let assign3050_e4533: f64 = (-50.0);
        let assign3050_e4534: f64 = if locals.var_fn25_calc_iq__exparg < assign3050_e4533 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3050_e4534;
        locals.var_guard28_rv = 0.0;

        let (assign3060_e4543, assign3060_e4543_d_n2, assign3060_e4543_d_n3, assign3060_e4543_d_n4, assign3060_e4543_d_n7, assign3060_e4543_d_n16, assign3060_e4543_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard27 == 0.0)) && (locals.var_guard28 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3060_e4543;
        locals.var_fn25_calc_iq__ff_dn2 = assign3060_e4543_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3060_e4543_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3060_e4543_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3060_e4543_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3060_e4543_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3060_e4543_d_n17;
        locals.var_fn25_calc_iq__ff_rv = 0.0;

        let (assign3070_e4558, assign3070_e4558_d_n2, assign3070_e4558_d_n3, assign3070_e4558_d_n4, assign3070_e4558_d_n7, assign3070_e4558_d_n16, assign3070_e4558_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard27 == 0.0)) && (locals.var_guard28 == 0.0)) {
        let assign3070_e4554: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3070_e4555: f64 = (1.0 + assign3070_e4554);
        let assign3070_e4556: f64 = (1.0 / assign3070_e4555);
        (assign3070_e4556, (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3070_e4555 * assign3070_e4555))),)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3070_e4558;
        locals.var_fn25_calc_iq__ff_dn2 = assign3070_e4558_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3070_e4558_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3070_e4558_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3070_e4558_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3070_e4558_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3070_e4558_d_n17;
        locals.var_fn25_calc_iq__ff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3080_e4617, assign3080_e4617_d_n2, assign3080_e4617_d_n3, assign3080_e4617_d_n4, assign3080_e4617_d_n7, assign3080_e4617_d_n16, assign3080_e4617_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3080_e4603, assign3080_e4603_d_n2, assign3080_e4603_d_n7, assign3080_e4603_d_n16, assign3080_e4603_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3080_e4567: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4570: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4573: f64 = (0.001 / p.p53);
                let assign3080_e4576: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4577: f64 = (assign3080_e4573 * assign3080_e4576);
                let assign3080_e4578: f64 = (assign3080_e4577).tanh();
                let assign3080_e4579: f64 = (assign3080_e4570 * assign3080_e4578);
                let assign3080_e4580: f64 = (assign3080_e4567 + assign3080_e4579);
                let assign3080_e4581: f64 = (0.5 * assign3080_e4580);
                (assign3080_e4581, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))),)
            } else {
                let (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3080_e4588: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4591: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4594: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4595: f64 = (assign3080_e4591 * assign3080_e4594);
                        let assign3080_e4597: f64 = (assign3080_e4595 + p.p53);
                        let assign3080_e4598: f64 = (assign3080_e4597).sqrt();
                        let assign3080_e4599: f64 = (assign3080_e4588 + assign3080_e4598);
                        let assign3080_e4600: f64 = (0.5 * assign3080_e4599);
                        (assign3080_e4600, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3080_e4598)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3080_e4598)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3080_e4598)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3080_e4594) + (assign3080_e4591 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3080_e4598)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,)
            }
        };
        let assign3080_e4607: f64 = (p.p51 * 0.1);
        let assign3080_e4609: f64 = (assign3080_e4607 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3080_e4611: f64 = (assign3080_e4609 * locals.var_fn25_calc_iq__ff);
        let assign3080_e4612: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3080_e4611);
        let assign3080_e4613: f64 = (assign3080_e4603 - assign3080_e4612);
        let assign3080_e4615: f64 = (assign3080_e4613 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3080_e4615, ((assign3080_e4603_d_n2 - (-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn2))) / locals.var_fn25_calc_iq__two_n_phit), ((-(-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn3))) / locals.var_fn25_calc_iq__two_n_phit), ((((-(locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3080_e4607 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ff) + (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), ((assign3080_e4603_d_n7 - (-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn7))) / locals.var_fn25_calc_iq__two_n_phit), ((((assign3080_e4603_d_n16 - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), ((((assign3080_e4603_d_n17 - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__eta, locals.var_fn25_calc_iq__eta_dn2, locals.var_fn25_calc_iq__eta_dn3, locals.var_fn25_calc_iq__eta_dn4, locals.var_fn25_calc_iq__eta_dn7, locals.var_fn25_calc_iq__eta_dn16, locals.var_fn25_calc_iq__eta_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta = assign3080_e4617;
        locals.var_fn25_calc_iq__eta_dn2 = assign3080_e4617_d_n2;
        locals.var_fn25_calc_iq__eta_dn3 = assign3080_e4617_d_n3;
        locals.var_fn25_calc_iq__eta_dn4 = assign3080_e4617_d_n4;
        locals.var_fn25_calc_iq__eta_dn7 = assign3080_e4617_d_n7;
        locals.var_fn25_calc_iq__eta_dn16 = assign3080_e4617_d_n16;
        locals.var_fn25_calc_iq__eta_dn17 = assign3080_e4617_d_n17;
        locals.var_fn25_calc_iq__eta_rv = 0.0;

        let assign3090_e4620: f64 = if locals.var_fn25_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3090_e4620;
        locals.var_guard29_rv = 0.0;

        let (assign3100_e4628, assign3100_e4628_d_n2, assign3100_e4628_d_n3, assign3100_e4628_d_n4, assign3100_e4628_d_n7, assign3100_e4628_d_n16, assign3100_e4628_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard29 != 0.0)) {
        let assign3100_e4626: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta);
        (assign3100_e4626, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3100_e4628;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3100_e4628_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3100_e4628_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3100_e4628_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3100_e4628_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3100_e4628_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3100_e4628_d_n17;
        locals.var_fn25_calc_iq__qinvv_rv = 0.0;

        let assign3110_e4631: f64 = (-50.0);
        let assign3110_e4632: f64 = if locals.var_fn25_calc_iq__eta < assign3110_e4631 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3110_e4632;
        locals.var_guard30_rv = 0.0;

        let (assign3120_e4644, assign3120_e4644_d_n2, assign3120_e4644_d_n3, assign3120_e4644_d_n4, assign3120_e4644_d_n7, assign3120_e4644_d_n16, assign3120_e4644_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign3120_e4641: f64 = (locals.var_fn25_calc_iq__eta).exp();
        let assign3120_e4642: f64 = (locals.var_fn25_calc_iq__qref * assign3120_e4641);
        (assign3120_e4642, (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn2)), (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn4))), (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3120_e4644;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3120_e4644_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3120_e4644_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3120_e4644_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3120_e4644_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3120_e4644_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3120_e4644_d_n17;
        locals.var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign3130_e4660, assign3130_e4660_d_n2, assign3130_e4660_d_n3, assign3130_e4660_d_n4, assign3130_e4660_d_n7, assign3130_e4660_d_n16, assign3130_e4660_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 == 0.0)) {
        let assign3130_e4655: f64 = (locals.var_fn25_calc_iq__eta).exp();
        let assign3130_e4656: f64 = (1.0 + assign3130_e4655);
        let assign3130_e4657: f64 = (assign3130_e4656).ln();
        let assign3130_e4658: f64 = (locals.var_fn25_calc_iq__qref * assign3130_e4657);
        (assign3130_e4658, (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn2) / assign3130_e4656)), (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn3) / assign3130_e4656)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn4) / assign3130_e4656))), (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn7) / assign3130_e4656)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn16) / assign3130_e4656))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn17) / assign3130_e4656))),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3130_e4660;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3130_e4660_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3130_e4660_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3130_e4660_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3130_e4660_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3130_e4660_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3130_e4660_d_n17;
        locals.var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign3140_e4674, assign3140_e4674_d_n2, assign3140_e4674_d_n3, assign3140_e4674_d_n4, assign3140_e4674_d_n7, assign3140_e4674_d_n16, assign3140_e4674_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3140_e4667: f64 = (locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv);
        let assign3140_e4669: f64 = (assign3140_e4667 / locals.var_fn25_calc_iq__cgin);
        let assign3140_e4670: f64 = (1.0 + assign3140_e4669);
        let assign3140_e4671: f64 = (locals.var_fn25_calc_iq__tfacmobin * assign3140_e4670);
        let assign3140_e4672: f64 = (locals.var_fn25_calc_iq__mu0 / assign3140_e4671);
        (assign3140_e4672, (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * ((locals.var_fn25_calc_iq__tfacmobin_dn4 * assign3140_e4670) + (locals.var_fn25_calc_iq__tfacmobin * ((((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3140_e4667 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin))))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))),)
    } else {
        (locals.var_fn25_calc_iq__muf, locals.var_fn25_calc_iq__muf_dn2, locals.var_fn25_calc_iq__muf_dn3, locals.var_fn25_calc_iq__muf_dn4, locals.var_fn25_calc_iq__muf_dn7, locals.var_fn25_calc_iq__muf_dn16, locals.var_fn25_calc_iq__muf_dn17,)
    }
};
        locals.var_fn25_calc_iq__muf = assign3140_e4674;
        locals.var_fn25_calc_iq__muf_dn2 = assign3140_e4674_d_n2;
        locals.var_fn25_calc_iq__muf_dn3 = assign3140_e4674_d_n3;
        locals.var_fn25_calc_iq__muf_dn4 = assign3140_e4674_d_n4;
        locals.var_fn25_calc_iq__muf_dn7 = assign3140_e4674_d_n7;
        locals.var_fn25_calc_iq__muf_dn16 = assign3140_e4674_d_n16;
        locals.var_fn25_calc_iq__muf_dn17 = assign3140_e4674_d_n17;
        locals.var_fn25_calc_iq__muf_rv = 0.0;

        let (assign3150_e4706, assign3150_e4706_d_n2, assign3150_e4706_d_n3, assign3150_e4706_d_n4, assign3150_e4706_d_n7, assign3150_e4706_d_n16, assign3150_e4706_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3150_e4680: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tnomin);
        let assign3150_e4681: f64 = (1.0 + assign3150_e4680);
        let assign3150_e4685: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin);
        let assign3150_e4686: f64 = (1.0 + assign3150_e4685);
        let assign3150_e4687: f64 = (assign3150_e4681 / assign3150_e4686);
        let assign3150_e4688: f64 = (locals.var_fn25_calc_iq__vel0 * assign3150_e4687);
        let assign3150_e4692: f64 = (locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin);
        let assign3150_e4694: f64 = (assign3150_e4692 / locals.var_fn25_calc_iq__lin);
        let assign3150_e4695: f64 = (1.0 + assign3150_e4694);
        let assign3150_e4696: f64 = (assign3150_e4688 * assign3150_e4695);
        let assign3150_e4700: f64 = (locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv);
        let assign3150_e4702: f64 = (assign3150_e4700 / locals.var_fn25_calc_iq__cgin);
        let assign3150_e4703: f64 = (1.0 + assign3150_e4702);
        let assign3150_e4704: f64 = (assign3150_e4696 / assign3150_e4703);
        (assign3150_e4704, (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (((((locals.var_fn25_calc_iq__vel0 * (-((assign3150_e4681 * (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin_dn4)) / (assign3150_e4686 * assign3150_e4686)))) * assign3150_e4695) * assign3150_e4703) - (assign3150_e4696 * ((((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3150_e4700 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)))) / (assign3150_e4703 * assign3150_e4703)), (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), ((((assign3150_e4688 * ((locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin_dn16) / locals.var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)), ((((assign3150_e4688 * ((locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin_dn17) / locals.var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)),)
    } else {
        (locals.var_fn25_calc_iq__vx, locals.var_fn25_calc_iq__vx_dn2, locals.var_fn25_calc_iq__vx_dn3, locals.var_fn25_calc_iq__vx_dn4, locals.var_fn25_calc_iq__vx_dn7, locals.var_fn25_calc_iq__vx_dn16, locals.var_fn25_calc_iq__vx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vx = assign3150_e4706;
        locals.var_fn25_calc_iq__vx_dn2 = assign3150_e4706_d_n2;
        locals.var_fn25_calc_iq__vx_dn3 = assign3150_e4706_d_n3;
        locals.var_fn25_calc_iq__vx_dn4 = assign3150_e4706_d_n4;
        locals.var_fn25_calc_iq__vx_dn7 = assign3150_e4706_d_n7;
        locals.var_fn25_calc_iq__vx_dn16 = assign3150_e4706_d_n16;
        locals.var_fn25_calc_iq__vx_dn17 = assign3150_e4706_d_n17;
        locals.var_fn25_calc_iq__vx_rv = 0.0;

        let (assign3170_e4732, assign3170_e4732_d_n2, assign3170_e4732_d_n3, assign3170_e4732_d_n4, assign3170_e4732_d_n7, assign3170_e4732_d_n16, assign3170_e4732_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3170_e4728: f64 = (locals.var_fn25_calc_iq__vx * locals.var_fn25_calc_iq__lin);
        let assign3170_e4730: f64 = (assign3170_e4728 / locals.var_fn25_calc_iq__muf);
        (assign3170_e4730, ((((locals.var_fn25_calc_iq__vx_dn2 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn2)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn3 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn3)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn4 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn4)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn7 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn7)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn16 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn16)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn17 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn17)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)),)
    } else {
        (locals.var_fn25_calc_iq__vdsats, locals.var_fn25_calc_iq__vdsats_dn2, locals.var_fn25_calc_iq__vdsats_dn3, locals.var_fn25_calc_iq__vdsats_dn4, locals.var_fn25_calc_iq__vdsats_dn7, locals.var_fn25_calc_iq__vdsats_dn16, locals.var_fn25_calc_iq__vdsats_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats = assign3170_e4732;
        locals.var_fn25_calc_iq__vdsats_dn2 = assign3170_e4732_d_n2;
        locals.var_fn25_calc_iq__vdsats_dn3 = assign3170_e4732_d_n3;
        locals.var_fn25_calc_iq__vdsats_dn4 = assign3170_e4732_d_n4;
        locals.var_fn25_calc_iq__vdsats_dn7 = assign3170_e4732_d_n7;
        locals.var_fn25_calc_iq__vdsats_dn16 = assign3170_e4732_d_n16;
        locals.var_fn25_calc_iq__vdsats_dn17 = assign3170_e4732_d_n17;
        locals.var_fn25_calc_iq__vdsats_rv = 0.0;

        let (assign3180_e4749, assign3180_e4749_d_n2, assign3180_e4749_d_n3, assign3180_e4749_d_n4, assign3180_e4749_d_n7, assign3180_e4749_d_n16, assign3180_e4749_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3180_e4738: f64 = (2.0 * locals.var_fn25_calc_iq__qinvv);
        let assign3180_e4740: f64 = (assign3180_e4738 / locals.var_fn25_calc_iq__cgin);
        let assign3180_e4742: f64 = (assign3180_e4740 / locals.var_fn25_calc_iq__vdsats);
        let assign3180_e4743: f64 = (1.0 + assign3180_e4742);
        let assign3180_e4744: f64 = (assign3180_e4743).sqrt();
        let assign3180_e4745: f64 = (locals.var_fn25_calc_iq__vdsats * assign3180_e4744);
        let assign3180_e4747: f64 = (assign3180_e4745 - locals.var_fn25_calc_iq__vdsats);
        (assign3180_e4747, (((locals.var_fn25_calc_iq__vdsats_dn2 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn2)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn2), (((locals.var_fn25_calc_iq__vdsats_dn3 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn3)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn3), (((locals.var_fn25_calc_iq__vdsats_dn4 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3180_e4738 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn4)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn4), (((locals.var_fn25_calc_iq__vdsats_dn7 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn7)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn7), (((locals.var_fn25_calc_iq__vdsats_dn16 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn16)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn16), (((locals.var_fn25_calc_iq__vdsats_dn17 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn17)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vdsats1, locals.var_fn25_calc_iq__vdsats1_dn2, locals.var_fn25_calc_iq__vdsats1_dn3, locals.var_fn25_calc_iq__vdsats1_dn4, locals.var_fn25_calc_iq__vdsats1_dn7, locals.var_fn25_calc_iq__vdsats1_dn16, locals.var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats1 = assign3180_e4749;
        locals.var_fn25_calc_iq__vdsats1_dn2 = assign3180_e4749_d_n2;
        locals.var_fn25_calc_iq__vdsats1_dn3 = assign3180_e4749_d_n3;
        locals.var_fn25_calc_iq__vdsats1_dn4 = assign3180_e4749_d_n4;
        locals.var_fn25_calc_iq__vdsats1_dn7 = assign3180_e4749_d_n7;
        locals.var_fn25_calc_iq__vdsats1_dn16 = assign3180_e4749_d_n16;
        locals.var_fn25_calc_iq__vdsats1_dn17 = assign3180_e4749_d_n17;
        locals.var_fn25_calc_iq__vdsats1_rv = 0.0;

        let (assign3190_e4761, assign3190_e4761_d_n2, assign3190_e4761_d_n3, assign3190_e4761_d_n4, assign3190_e4761_d_n7, assign3190_e4761_d_n16, assign3190_e4761_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3190_e4754: f64 = (1.0 - locals.var_fn25_calc_iq__ff);
        let assign3190_e4755: f64 = (locals.var_fn25_calc_iq__vdsats * assign3190_e4754);
        let assign3190_e4758: f64 = (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff);
        let assign3190_e4759: f64 = (assign3190_e4755 + assign3190_e4758);
        (assign3190_e4759, (((locals.var_fn25_calc_iq__vdsats_dn2 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn2))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn2)), (((locals.var_fn25_calc_iq__vdsats_dn3 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn3))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn3)), (((locals.var_fn25_calc_iq__vdsats_dn4 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit_dn4 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn4))), (((locals.var_fn25_calc_iq__vdsats_dn7 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn7))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn7)), (((locals.var_fn25_calc_iq__vdsats_dn16 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn16))) + ((locals.var_fn25_calc_iq__two_n_phit_dn16 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn16))), (((locals.var_fn25_calc_iq__vdsats_dn17 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn17))) + ((locals.var_fn25_calc_iq__two_n_phit_dn17 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__vdsat, locals.var_fn25_calc_iq__vdsat_dn2, locals.var_fn25_calc_iq__vdsat_dn3, locals.var_fn25_calc_iq__vdsat_dn4, locals.var_fn25_calc_iq__vdsat_dn7, locals.var_fn25_calc_iq__vdsat_dn16, locals.var_fn25_calc_iq__vdsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat = assign3190_e4761;
        locals.var_fn25_calc_iq__vdsat_dn2 = assign3190_e4761_d_n2;
        locals.var_fn25_calc_iq__vdsat_dn3 = assign3190_e4761_d_n3;
        locals.var_fn25_calc_iq__vdsat_dn4 = assign3190_e4761_d_n4;
        locals.var_fn25_calc_iq__vdsat_dn7 = assign3190_e4761_d_n7;
        locals.var_fn25_calc_iq__vdsat_dn16 = assign3190_e4761_d_n16;
        locals.var_fn25_calc_iq__vdsat_dn17 = assign3190_e4761_d_n17;
        locals.var_fn25_calc_iq__vdsat_rv = 0.0;

        let (assign3200_e4773, assign3200_e4773_d_n2, assign3200_e4773_d_n3, assign3200_e4773_d_n4, assign3200_e4773_d_n7, assign3200_e4773_d_n16, assign3200_e4773_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3200_e4766: f64 = (1.0 - locals.var_fn25_calc_iq__ff);
        let assign3200_e4767: f64 = (locals.var_fn25_calc_iq__vdsats1 * assign3200_e4766);
        let assign3200_e4770: f64 = (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff);
        let assign3200_e4771: f64 = (assign3200_e4767 + assign3200_e4770);
        (assign3200_e4771, (((locals.var_fn25_calc_iq__vdsats1_dn2 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn2))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn2)), (((locals.var_fn25_calc_iq__vdsats1_dn3 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn3))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn3)), (((locals.var_fn25_calc_iq__vdsats1_dn4 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit_dn4 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn4))), (((locals.var_fn25_calc_iq__vdsats1_dn7 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn7))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn7)), (((locals.var_fn25_calc_iq__vdsats1_dn16 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn16))) + ((locals.var_fn25_calc_iq__two_n_phit_dn16 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn16))), (((locals.var_fn25_calc_iq__vdsats1_dn17 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn17))) + ((locals.var_fn25_calc_iq__two_n_phit_dn17 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__vdsat1, locals.var_fn25_calc_iq__vdsat1_dn2, locals.var_fn25_calc_iq__vdsat1_dn3, locals.var_fn25_calc_iq__vdsat1_dn4, locals.var_fn25_calc_iq__vdsat1_dn7, locals.var_fn25_calc_iq__vdsat1_dn16, locals.var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat1 = assign3200_e4773;
        locals.var_fn25_calc_iq__vdsat1_dn2 = assign3200_e4773_d_n2;
        locals.var_fn25_calc_iq__vdsat1_dn3 = assign3200_e4773_d_n3;
        locals.var_fn25_calc_iq__vdsat1_dn4 = assign3200_e4773_d_n4;
        locals.var_fn25_calc_iq__vdsat1_dn7 = assign3200_e4773_d_n7;
        locals.var_fn25_calc_iq__vdsat1_dn16 = assign3200_e4773_d_n16;
        locals.var_fn25_calc_iq__vdsat1_dn17 = assign3200_e4773_d_n17;
        locals.var_fn25_calc_iq__vdsat1_rv = 0.0;

        let (assign3210_e4842, assign3210_e4842_d_n2, assign3210_e4842_d_n3, assign3210_e4842_d_n4, assign3210_e4842_d_n7, assign3210_e4842_d_n16, assign3210_e4842_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3210_e4832, assign3210_e4832_d_n2, assign3210_e4832_d_n3, assign3210_e4832_d_n4, assign3210_e4832_d_n7, assign3210_e4832_d_n16, assign3210_e4832_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3210_e4785: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4786: f64 = assign3210_e4785;
                let assign3210_e4790: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4791: f64 = (-assign3210_e4790);
                let assign3210_e4794: f64 = (0.001 / p.p53);
                let assign3210_e4798: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4799: f64 = (-assign3210_e4798);
                let assign3210_e4800: f64 = (assign3210_e4794 * assign3210_e4799);
                let assign3210_e4801: f64 = (assign3210_e4800).tanh();
                let assign3210_e4802: f64 = (assign3210_e4791 * assign3210_e4801);
                let assign3210_e4803: f64 = (assign3210_e4786 + assign3210_e4802);
                let assign3210_e4804: f64 = (0.5 * assign3210_e4803);
                (assign3210_e4804, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))),)
            } else {
                let (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3210_e4812: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4813: f64 = assign3210_e4812;
                        let assign3210_e4817: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4818: f64 = (-assign3210_e4817);
                        let assign3210_e4822: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4823: f64 = (-assign3210_e4822);
                        let assign3210_e4824: f64 = (assign3210_e4818 * assign3210_e4823);
                        let assign3210_e4826: f64 = (assign3210_e4824 + p.p53);
                        let assign3210_e4827: f64 = (assign3210_e4826).sqrt();
                        let assign3210_e4828: f64 = (assign3210_e4813 + assign3210_e4827);
                        let assign3210_e4829: f64 = (0.5 * assign3210_e4828);
                        (assign3210_e4829, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,)
            }
        };
        let assign3210_e4834: f64 = (assign3210_e4832).powf(locals.var_fn25_calc_iq__beta);
        let assign3210_e4835: f64 = (1.0 + assign3210_e4834);
        let assign3210_e4838: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3210_e4839: f64 = (assign3210_e4835).powf(assign3210_e4838);
        let assign3210_e4840: f64 = (1.0 / assign3210_e4839);
        (assign3210_e4840, (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))),)
    } else {
        (locals.var_fn25_calc_iq__fsd, locals.var_fn25_calc_iq__fsd_dn2, locals.var_fn25_calc_iq__fsd_dn3, locals.var_fn25_calc_iq__fsd_dn4, locals.var_fn25_calc_iq__fsd_dn7, locals.var_fn25_calc_iq__fsd_dn16, locals.var_fn25_calc_iq__fsd_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd = assign3210_e4842;
        locals.var_fn25_calc_iq__fsd_dn2 = assign3210_e4842_d_n2;
        locals.var_fn25_calc_iq__fsd_dn3 = assign3210_e4842_d_n3;
        locals.var_fn25_calc_iq__fsd_dn4 = assign3210_e4842_d_n4;
        locals.var_fn25_calc_iq__fsd_dn7 = assign3210_e4842_d_n7;
        locals.var_fn25_calc_iq__fsd_dn16 = assign3210_e4842_d_n16;
        locals.var_fn25_calc_iq__fsd_dn17 = assign3210_e4842_d_n17;
        locals.var_fn25_calc_iq__fsd_rv = 0.0;

        let (assign3220_e4848, assign3220_e4848_d_n2, assign3220_e4848_d_n3, assign3220_e4848_d_n4, assign3220_e4848_d_n7, assign3220_e4848_d_n16, assign3220_e4848_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3220_e4846: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd);
        (assign3220_e4846, (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn2), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn3), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn4), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn7), ((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__fsd) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn16)), ((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__fsd) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdx, locals.var_fn25_calc_iq__vdx_dn2, locals.var_fn25_calc_iq__vdx_dn3, locals.var_fn25_calc_iq__vdx_dn4, locals.var_fn25_calc_iq__vdx_dn7, locals.var_fn25_calc_iq__vdx_dn16, locals.var_fn25_calc_iq__vdx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx = assign3220_e4848;
        locals.var_fn25_calc_iq__vdx_dn2 = assign3220_e4848_d_n2;
        locals.var_fn25_calc_iq__vdx_dn3 = assign3220_e4848_d_n3;
        locals.var_fn25_calc_iq__vdx_dn4 = assign3220_e4848_d_n4;
        locals.var_fn25_calc_iq__vdx_dn7 = assign3220_e4848_d_n7;
        locals.var_fn25_calc_iq__vdx_dn16 = assign3220_e4848_d_n16;
        locals.var_fn25_calc_iq__vdx_dn17 = assign3220_e4848_d_n17;
        locals.var_fn25_calc_iq__vdx_rv = 0.0;

        let (assign3230_e4923, assign3230_e4923_d_n2, assign3230_e4923_d_n3, assign3230_e4923_d_n4, assign3230_e4923_d_n7, assign3230_e4923_d_n16, assign3230_e4923_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3230_e4913, assign3230_e4913_d_n2, assign3230_e4913_d_n3, assign3230_e4913_d_n4, assign3230_e4913_d_n7, assign3230_e4913_d_n16, assign3230_e4913_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3230_e4859: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4861: f64 = (assign3230_e4859 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4862: f64 = assign3230_e4861;
                let assign3230_e4865: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4867: f64 = (assign3230_e4865 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4868: f64 = (-assign3230_e4867);
                let assign3230_e4871: f64 = (0.001 / p.p53);
                let assign3230_e4874: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4876: f64 = (assign3230_e4874 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4877: f64 = (-assign3230_e4876);
                let assign3230_e4878: f64 = (assign3230_e4871 * assign3230_e4877);
                let assign3230_e4879: f64 = (assign3230_e4878).tanh();
                let assign3230_e4880: f64 = (assign3230_e4868 * assign3230_e4879);
                let assign3230_e4881: f64 = (assign3230_e4862 + assign3230_e4880);
                let assign3230_e4882: f64 = (0.5 * assign3230_e4881);
                (assign3230_e4882, (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))),)
            } else {
                let (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3230_e4889: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4891: f64 = (assign3230_e4889 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4892: f64 = assign3230_e4891;
                        let assign3230_e4895: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4897: f64 = (assign3230_e4895 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4898: f64 = (-assign3230_e4897);
                        let assign3230_e4901: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4903: f64 = (assign3230_e4901 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4904: f64 = (-assign3230_e4903);
                        let assign3230_e4905: f64 = (assign3230_e4898 * assign3230_e4904);
                        let assign3230_e4907: f64 = (assign3230_e4905 + p.p53);
                        let assign3230_e4908: f64 = (assign3230_e4907).sqrt();
                        let assign3230_e4909: f64 = (assign3230_e4892 + assign3230_e4908);
                        let assign3230_e4910: f64 = (0.5 * assign3230_e4909);
                        (assign3230_e4910, (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,)
            }
        };
        let assign3230_e4915: f64 = (assign3230_e4913).powf(locals.var_fn25_calc_iq__beta);
        let assign3230_e4916: f64 = (1.0 + assign3230_e4915);
        let assign3230_e4919: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3230_e4920: f64 = (assign3230_e4916).powf(assign3230_e4919);
        let assign3230_e4921: f64 = (1.0 / assign3230_e4920);
        (assign3230_e4921, (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))),)
    } else {
        (locals.var_fn25_calc_iq__fds, locals.var_fn25_calc_iq__fds_dn2, locals.var_fn25_calc_iq__fds_dn3, locals.var_fn25_calc_iq__fds_dn4, locals.var_fn25_calc_iq__fds_dn7, locals.var_fn25_calc_iq__fds_dn16, locals.var_fn25_calc_iq__fds_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds = assign3230_e4923;
        locals.var_fn25_calc_iq__fds_dn2 = assign3230_e4923_d_n2;
        locals.var_fn25_calc_iq__fds_dn3 = assign3230_e4923_d_n3;
        locals.var_fn25_calc_iq__fds_dn4 = assign3230_e4923_d_n4;
        locals.var_fn25_calc_iq__fds_dn7 = assign3230_e4923_d_n7;
        locals.var_fn25_calc_iq__fds_dn16 = assign3230_e4923_d_n16;
        locals.var_fn25_calc_iq__fds_dn17 = assign3230_e4923_d_n17;
        locals.var_fn25_calc_iq__fds_rv = 0.0;

        let (assign3240_e4930, assign3240_e4930_d_n2, assign3240_e4930_d_n3, assign3240_e4930_d_n4, assign3240_e4930_d_n7, assign3240_e4930_d_n16, assign3240_e4930_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3240_e4926: f64 = (-locals.var_fn25_calc_iq__vdsin);
        let assign3240_e4928: f64 = (assign3240_e4926 * locals.var_fn25_calc_iq__fds);
        (assign3240_e4928, (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn2), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn3), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn4), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn7), (((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__fds) + (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn16)), (((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__fds) + (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vsx, locals.var_fn25_calc_iq__vsx_dn2, locals.var_fn25_calc_iq__vsx_dn3, locals.var_fn25_calc_iq__vsx_dn4, locals.var_fn25_calc_iq__vsx_dn7, locals.var_fn25_calc_iq__vsx_dn16, locals.var_fn25_calc_iq__vsx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx = assign3240_e4930;
        locals.var_fn25_calc_iq__vsx_dn2 = assign3240_e4930_d_n2;
        locals.var_fn25_calc_iq__vsx_dn3 = assign3240_e4930_d_n3;
        locals.var_fn25_calc_iq__vsx_dn4 = assign3240_e4930_d_n4;
        locals.var_fn25_calc_iq__vsx_dn7 = assign3240_e4930_d_n7;
        locals.var_fn25_calc_iq__vsx_dn16 = assign3240_e4930_d_n16;
        locals.var_fn25_calc_iq__vsx_dn17 = assign3240_e4930_d_n17;
        locals.var_fn25_calc_iq__vsx_rv = 0.0;

        let (assign3250_e4938, assign3250_e4938_d_n2, assign3250_e4938_d_n3, assign3250_e4938_d_n4, assign3250_e4938_d_n7, assign3250_e4938_d_n16, assign3250_e4938_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3250_e4934: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__myarg);
        let assign3250_e4936: f64 = (assign3250_e4934 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3250_e4936, ((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3250_e4934 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3250_e4938;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3250_e4938_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3250_e4938_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3250_e4938_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3250_e4938_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3250_e4938_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3250_e4938_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3260_e4941: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3260_e4941;
        locals.var_guard31_rv = 0.0;

        let (assign3270_e4947, assign3270_e4947_d_n2, assign3270_e4947_d_n3, assign3270_e4947_d_n4, assign3270_e4947_d_n7, assign3270_e4947_d_n16, assign3270_e4947_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard31 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3270_e4947;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3270_e4947_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3270_e4947_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3270_e4947_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3270_e4947_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3270_e4947_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3270_e4947_d_n17;
        locals.var_fn25_calc_iq__ffs_rv = 0.0;

        let assign3280_e4950: f64 = (-50.0);
        let assign3280_e4951: f64 = if locals.var_fn25_calc_iq__exparg < assign3280_e4950 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3280_e4951;
        locals.var_guard32_rv = 0.0;

        let (assign3290_e4960, assign3290_e4960_d_n2, assign3290_e4960_d_n3, assign3290_e4960_d_n4, assign3290_e4960_d_n7, assign3290_e4960_d_n16, assign3290_e4960_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard31 == 0.0)) && (locals.var_guard32 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3290_e4960;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3290_e4960_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3290_e4960_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3290_e4960_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3290_e4960_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3290_e4960_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3290_e4960_d_n17;
        locals.var_fn25_calc_iq__ffs_rv = 0.0;

        let (assign3300_e4975, assign3300_e4975_d_n2, assign3300_e4975_d_n3, assign3300_e4975_d_n4, assign3300_e4975_d_n7, assign3300_e4975_d_n16, assign3300_e4975_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard31 == 0.0)) && (locals.var_guard32 == 0.0)) {
        let assign3300_e4971: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3300_e4972: f64 = (1.0 + assign3300_e4971);
        let assign3300_e4973: f64 = (1.0 / assign3300_e4972);
        (assign3300_e4973, (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3300_e4972 * assign3300_e4972))),)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3300_e4975;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3300_e4975_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3300_e4975_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3300_e4975_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3300_e4975_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3300_e4975_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3300_e4975_d_n17;
        locals.var_fn25_calc_iq__ffs_rv = 0.0;

        let (assign3310_e4993, assign3310_e4993_d_n2, assign3310_e4993_d_n3, assign3310_e4993_d_n4, assign3310_e4993_d_n7, assign3310_e4993_d_n16, assign3310_e4993_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3310_e4979: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__vsx);
        let assign3310_e4983: f64 = (p.p51 * 0.1);
        let assign3310_e4985: f64 = (assign3310_e4983 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3310_e4987: f64 = (assign3310_e4985 * locals.var_fn25_calc_iq__ffs);
        let assign3310_e4988: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3310_e4987);
        let assign3310_e4989: f64 = (assign3310_e4979 - assign3310_e4988);
        let assign3310_e4991: f64 = (assign3310_e4989 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3310_e4991, (((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__vsx_dn2) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn2))) / locals.var_fn25_calc_iq__two_n_phit), (((-locals.var_fn25_calc_iq__vsx_dn3) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn3))) / locals.var_fn25_calc_iq__two_n_phit), (((((-locals.var_fn25_calc_iq__vsx_dn4) - (locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3310_e4983 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffs) + (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__vsx_dn7) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn7))) / locals.var_fn25_calc_iq__two_n_phit), (((((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__vsx_dn16) - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__vsx_dn17) - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__etas, locals.var_fn25_calc_iq__etas_dn2, locals.var_fn25_calc_iq__etas_dn3, locals.var_fn25_calc_iq__etas_dn4, locals.var_fn25_calc_iq__etas_dn7, locals.var_fn25_calc_iq__etas_dn16, locals.var_fn25_calc_iq__etas_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas = assign3310_e4993;
        locals.var_fn25_calc_iq__etas_dn2 = assign3310_e4993_d_n2;
        locals.var_fn25_calc_iq__etas_dn3 = assign3310_e4993_d_n3;
        locals.var_fn25_calc_iq__etas_dn4 = assign3310_e4993_d_n4;
        locals.var_fn25_calc_iq__etas_dn7 = assign3310_e4993_d_n7;
        locals.var_fn25_calc_iq__etas_dn16 = assign3310_e4993_d_n16;
        locals.var_fn25_calc_iq__etas_dn17 = assign3310_e4993_d_n17;
        locals.var_fn25_calc_iq__etas_rv = 0.0;

        let assign3320_e4996: f64 = if locals.var_fn25_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3320_e4996;
        locals.var_guard33_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3330_e5004, assign3330_e5004_d_n2, assign3330_e5004_d_n3, assign3330_e5004_d_n4, assign3330_e5004_d_n7, assign3330_e5004_d_n16, assign3330_e5004_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard33 != 0.0)) {
        let assign3330_e5002: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas);
        (assign3330_e5002, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3330_e5004;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3330_e5004_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3330_e5004_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3330_e5004_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3330_e5004_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3330_e5004_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3330_e5004_d_n17;
        locals.var_fn25_calc_iq__qinvs_rv = 0.0;

        let assign3340_e5007: f64 = (-50.0);
        let assign3340_e5008: f64 = if locals.var_fn25_calc_iq__etas < assign3340_e5007 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3340_e5008;
        locals.var_guard34_rv = 0.0;

        let (assign3350_e5020, assign3350_e5020_d_n2, assign3350_e5020_d_n3, assign3350_e5020_d_n4, assign3350_e5020_d_n7, assign3350_e5020_d_n16, assign3350_e5020_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard33 == 0.0)) && (locals.var_guard34 != 0.0)) {
        let assign3350_e5017: f64 = (locals.var_fn25_calc_iq__etas).exp();
        let assign3350_e5018: f64 = (locals.var_fn25_calc_iq__qref * assign3350_e5017);
        (assign3350_e5018, (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn2)), (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn4))), (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3350_e5020;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3350_e5020_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3350_e5020_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3350_e5020_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3350_e5020_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3350_e5020_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3350_e5020_d_n17;
        locals.var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign3360_e5036, assign3360_e5036_d_n2, assign3360_e5036_d_n3, assign3360_e5036_d_n4, assign3360_e5036_d_n7, assign3360_e5036_d_n16, assign3360_e5036_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard33 == 0.0)) && (locals.var_guard34 == 0.0)) {
        let assign3360_e5031: f64 = (locals.var_fn25_calc_iq__etas).exp();
        let assign3360_e5032: f64 = (1.0 + assign3360_e5031);
        let assign3360_e5033: f64 = (assign3360_e5032).ln();
        let assign3360_e5034: f64 = (locals.var_fn25_calc_iq__qref * assign3360_e5033);
        (assign3360_e5034, (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn2) / assign3360_e5032)), (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn3) / assign3360_e5032)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn4) / assign3360_e5032))), (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn7) / assign3360_e5032)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn16) / assign3360_e5032))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn17) / assign3360_e5032))),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3360_e5036;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3360_e5036_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3360_e5036_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3360_e5036_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3360_e5036_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3360_e5036_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3360_e5036_d_n17;
        locals.var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign3370_e5044, assign3370_e5044_d_n2, assign3370_e5044_d_n3, assign3370_e5044_d_n4, assign3370_e5044_d_n7, assign3370_e5044_d_n16, assign3370_e5044_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3370_e5040: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__myarg);
        let assign3370_e5042: f64 = (assign3370_e5040 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3370_e5042, ((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3370_e5040 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3370_e5044;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3370_e5044_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3370_e5044_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3370_e5044_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3370_e5044_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3370_e5044_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3370_e5044_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3380_e5047: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign3380_e5047;
        locals.var_guard35_rv = 0.0;

        let (assign3390_e5053, assign3390_e5053_d_n2, assign3390_e5053_d_n3, assign3390_e5053_d_n4, assign3390_e5053_d_n7, assign3390_e5053_d_n16, assign3390_e5053_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard35 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3390_e5053;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3390_e5053_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3390_e5053_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3390_e5053_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3390_e5053_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3390_e5053_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3390_e5053_d_n17;
        locals.var_fn25_calc_iq__ffd_rv = 0.0;

        let assign3400_e5056: f64 = (-50.0);
        let assign3400_e5057: f64 = if locals.var_fn25_calc_iq__exparg < assign3400_e5056 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3400_e5057;
        locals.var_guard36_rv = 0.0;

        let (assign3410_e5066, assign3410_e5066_d_n2, assign3410_e5066_d_n3, assign3410_e5066_d_n4, assign3410_e5066_d_n7, assign3410_e5066_d_n16, assign3410_e5066_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard35 == 0.0)) && (locals.var_guard36 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3410_e5066;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3410_e5066_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3410_e5066_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3410_e5066_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3410_e5066_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3410_e5066_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3410_e5066_d_n17;
        locals.var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign3420_e5081, assign3420_e5081_d_n2, assign3420_e5081_d_n3, assign3420_e5081_d_n4, assign3420_e5081_d_n7, assign3420_e5081_d_n16, assign3420_e5081_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard35 == 0.0)) && (locals.var_guard36 == 0.0)) {
        let assign3420_e5077: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3420_e5078: f64 = (1.0 + assign3420_e5077);
        let assign3420_e5079: f64 = (1.0 / assign3420_e5078);
        (assign3420_e5079, (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3420_e5078 * assign3420_e5078))),)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3420_e5081;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3420_e5081_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3420_e5081_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3420_e5081_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3420_e5081_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3420_e5081_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3420_e5081_d_n17;
        locals.var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign3430_e5099, assign3430_e5099_d_n2, assign3430_e5099_d_n3, assign3430_e5099_d_n4, assign3430_e5099_d_n7, assign3430_e5099_d_n16, assign3430_e5099_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3430_e5085: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdx);
        let assign3430_e5089: f64 = (p.p51 * 0.1);
        let assign3430_e5091: f64 = (assign3430_e5089 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3430_e5093: f64 = (assign3430_e5091 * locals.var_fn25_calc_iq__ffd);
        let assign3430_e5094: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3430_e5093);
        let assign3430_e5095: f64 = (assign3430_e5085 - assign3430_e5094);
        let assign3430_e5097: f64 = (assign3430_e5095 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3430_e5097, (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vdx_dn2) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn2))) / locals.var_fn25_calc_iq__two_n_phit), (((-locals.var_fn25_calc_iq__vdx_dn3) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn3))) / locals.var_fn25_calc_iq__two_n_phit), (((((-locals.var_fn25_calc_iq__vdx_dn4) - (locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3430_e5089 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffd) + (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vdx_dn7) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn7))) / locals.var_fn25_calc_iq__two_n_phit), (((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdx_dn16) - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((((-locals.var_fn25_calc_iq__vdx_dn17) - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__etad, locals.var_fn25_calc_iq__etad_dn2, locals.var_fn25_calc_iq__etad_dn3, locals.var_fn25_calc_iq__etad_dn4, locals.var_fn25_calc_iq__etad_dn7, locals.var_fn25_calc_iq__etad_dn16, locals.var_fn25_calc_iq__etad_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad = assign3430_e5099;
        locals.var_fn25_calc_iq__etad_dn2 = assign3430_e5099_d_n2;
        locals.var_fn25_calc_iq__etad_dn3 = assign3430_e5099_d_n3;
        locals.var_fn25_calc_iq__etad_dn4 = assign3430_e5099_d_n4;
        locals.var_fn25_calc_iq__etad_dn7 = assign3430_e5099_d_n7;
        locals.var_fn25_calc_iq__etad_dn16 = assign3430_e5099_d_n16;
        locals.var_fn25_calc_iq__etad_dn17 = assign3430_e5099_d_n17;
        locals.var_fn25_calc_iq__etad_rv = 0.0;

        let assign3440_e5102: f64 = if locals.var_fn25_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign3440_e5102;
        locals.var_guard37_rv = 0.0;

        let (assign3450_e5110, assign3450_e5110_d_n2, assign3450_e5110_d_n3, assign3450_e5110_d_n4, assign3450_e5110_d_n7, assign3450_e5110_d_n16, assign3450_e5110_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign3450_e5108: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad);
        (assign3450_e5108, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3450_e5110;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3450_e5110_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3450_e5110_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3450_e5110_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3450_e5110_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3450_e5110_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3450_e5110_d_n17;
        locals.var_fn25_calc_iq__qinvd_rv = 0.0;

        let assign3460_e5113: f64 = (-50.0);
        let assign3460_e5114: f64 = if locals.var_fn25_calc_iq__etad < assign3460_e5113 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign3460_e5114;
        locals.var_guard38_rv = 0.0;

        let (assign3470_e5126, assign3470_e5126_d_n2, assign3470_e5126_d_n3, assign3470_e5126_d_n4, assign3470_e5126_d_n7, assign3470_e5126_d_n16, assign3470_e5126_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign3470_e5123: f64 = (locals.var_fn25_calc_iq__etad).exp();
        let assign3470_e5124: f64 = (locals.var_fn25_calc_iq__qref * assign3470_e5123);
        (assign3470_e5124, (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn2)), (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn4))), (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3470_e5126;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3470_e5126_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3470_e5126_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3470_e5126_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3470_e5126_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3470_e5126_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3470_e5126_d_n17;
        locals.var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign3480_e5142, assign3480_e5142_d_n2, assign3480_e5142_d_n3, assign3480_e5142_d_n4, assign3480_e5142_d_n7, assign3480_e5142_d_n16, assign3480_e5142_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign3480_e5137: f64 = (locals.var_fn25_calc_iq__etad).exp();
        let assign3480_e5138: f64 = (1.0 + assign3480_e5137);
        let assign3480_e5139: f64 = (assign3480_e5138).ln();
        let assign3480_e5140: f64 = (locals.var_fn25_calc_iq__qref * assign3480_e5139);
        (assign3480_e5140, (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn2) / assign3480_e5138)), (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn3) / assign3480_e5138)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn4) / assign3480_e5138))), (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn7) / assign3480_e5138)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn16) / assign3480_e5138))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn17) / assign3480_e5138))),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3480_e5142;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3480_e5142_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3480_e5142_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3480_e5142_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3480_e5142_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3480_e5142_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3480_e5142_d_n17;
        locals.var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign3490_e5150, assign3490_e5150_d_n2, assign3490_e5150_d_n3, assign3490_e5150_d_n4, assign3490_e5150_d_n7, assign3490_e5150_d_n16, assign3490_e5150_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3490_e5146: f64 = (locals.var_fn25_calc_iq__qinvs - locals.var_fn25_calc_iq__qinvd);
        let assign3490_e5148: f64 = (assign3490_e5146 / locals.var_fn25_calc_iq__cgin);
        (assign3490_e5148, ((locals.var_fn25_calc_iq__qinvs_dn2 - locals.var_fn25_calc_iq__qinvd_dn2) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn3 - locals.var_fn25_calc_iq__qinvd_dn3) / locals.var_fn25_calc_iq__cgin), ((((locals.var_fn25_calc_iq__qinvs_dn4 - locals.var_fn25_calc_iq__qinvd_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3490_e5146 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)), ((locals.var_fn25_calc_iq__qinvs_dn7 - locals.var_fn25_calc_iq__qinvd_dn7) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn16 - locals.var_fn25_calc_iq__qinvd_dn16) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn17 - locals.var_fn25_calc_iq__qinvd_dn17) / locals.var_fn25_calc_iq__cgin),)
    } else {
        (locals.var_fn25_calc_iq__vdsc, locals.var_fn25_calc_iq__vdsc_dn2, locals.var_fn25_calc_iq__vdsc_dn3, locals.var_fn25_calc_iq__vdsc_dn4, locals.var_fn25_calc_iq__vdsc_dn7, locals.var_fn25_calc_iq__vdsc_dn16, locals.var_fn25_calc_iq__vdsc_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsc = assign3490_e5150;
        locals.var_fn25_calc_iq__vdsc_dn2 = assign3490_e5150_d_n2;
        locals.var_fn25_calc_iq__vdsc_dn3 = assign3490_e5150_d_n3;
        locals.var_fn25_calc_iq__vdsc_dn4 = assign3490_e5150_d_n4;
        locals.var_fn25_calc_iq__vdsc_dn7 = assign3490_e5150_d_n7;
        locals.var_fn25_calc_iq__vdsc_dn16 = assign3490_e5150_d_n16;
        locals.var_fn25_calc_iq__vdsc_dn17 = assign3490_e5150_d_n17;
        locals.var_fn25_calc_iq__vdsc_rv = 0.0;

        let (assign3500_e5156, assign3500_e5156_d_n2, assign3500_e5156_d_n3, assign3500_e5156_d_n4, assign3500_e5156_d_n7, assign3500_e5156_d_n16, assign3500_e5156_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3500_e5154: f64 = (locals.var_fn25_calc_iq__vdsc / locals.var_fn25_calc_iq__vdsat);
        (assign3500_e5154, (((locals.var_fn25_calc_iq__vdsc_dn2 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn2)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn3 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn3)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn4 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn4)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn7 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn7)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn16 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn16)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn17 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn17)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)),)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign3500_e5156;
        locals.var_fn25_calc_iq__myarg_dn2 = assign3500_e5156_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign3500_e5156_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign3500_e5156_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign3500_e5156_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign3500_e5156_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign3500_e5156_d_n17;
        locals.var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign3540_e5225, assign3540_e5225_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3540_e5222: f64 = (2.302585092994046 * locals.var_fn25_calc_iq__phitin);
        let assign3540_e5223: f64 = (locals.var_fn25_calc_iq__ss / assign3540_e5222);
        (assign3540_e5223, (-((locals.var_fn25_calc_iq__ss * (2.302585092994046 * locals.var_fn25_calc_iq__phitin_dn4)) / (assign3540_e5222 * assign3540_e5222))),)
    } else {
        (locals.var_fn25_calc_iq__n0, locals.var_fn25_calc_iq__n0_dn4,)
    }
};
        locals.var_fn25_calc_iq__n0 = assign3540_e5225;
        locals.var_fn25_calc_iq__n0_dn4 = assign3540_e5225_d_n4;
        locals.var_fn25_calc_iq__n0_rv = 0.0;

        let (assign3550_e5233, assign3550_e5233_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3550_e5229: f64 = (2.0 * locals.var_fn25_calc_iq__n0);
        let assign3550_e5231: f64 = (assign3550_e5229 * locals.var_fn25_calc_iq__phitin);
        (assign3550_e5231, (((2.0 * locals.var_fn25_calc_iq__n0_dn4) * locals.var_fn25_calc_iq__phitin) + (assign3550_e5229 * locals.var_fn25_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit0, locals.var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit0 = assign3550_e5233;
        locals.var_fn25_calc_iq__two_n_phit0_dn4 = assign3550_e5233_d_n4;
        locals.var_fn25_calc_iq__two_n_phit0_rv = 0.0;

        let (assign3560_e5239, assign3560_e5239_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3560_e5237: f64 = (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit0);
        (assign3560_e5237, ((locals.var_fn25_calc_iq__cgin_dn4 * locals.var_fn25_calc_iq__two_n_phit0) + (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn25_calc_iq__qref0, locals.var_fn25_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn25_calc_iq__qref0 = assign3560_e5239;
        locals.var_fn25_calc_iq__qref0_dn4 = assign3560_e5239_d_n4;
        locals.var_fn25_calc_iq__qref0_rv = 0.0;

        let (assign3570_e5249, assign3570_e5249_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3570_e5244: f64 = (p.p51 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3570_e5246: f64 = (assign3570_e5244 / 2.0);
        let assign3570_e5247: f64 = (locals.var_fn25_calc_iq__vtof - assign3570_e5246);
        (assign3570_e5247, (locals.var_fn25_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn25_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn25_calc_iq__myarg0, locals.var_fn25_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn25_calc_iq__myarg0 = assign3570_e5249;
        locals.var_fn25_calc_iq__myarg0_dn4 = assign3570_e5249_d_n4;
        locals.var_fn25_calc_iq__myarg0_rv = 0.0;

        let (assign3580_e5300, assign3580_e5300_d_n2, assign3580_e5300_d_n4, assign3580_e5300_d_n7, assign3580_e5300_d_n16, assign3580_e5300_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3580_e5294, assign3580_e5294_d_n2, assign3580_e5294_d_n7, assign3580_e5294_d_n16, assign3580_e5294_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3580_e5258: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5261: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5264: f64 = (0.001 / p.p53);
                let assign3580_e5267: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5268: f64 = (assign3580_e5264 * assign3580_e5267);
                let assign3580_e5269: f64 = (assign3580_e5268).tanh();
                let assign3580_e5270: f64 = (assign3580_e5261 * assign3580_e5269);
                let assign3580_e5271: f64 = (assign3580_e5258 + assign3580_e5270);
                let assign3580_e5272: f64 = (0.5 * assign3580_e5271);
                (assign3580_e5272, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))),)
            } else {
                let (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3580_e5279: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5282: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5285: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5286: f64 = (assign3580_e5282 * assign3580_e5285);
                        let assign3580_e5288: f64 = (assign3580_e5286 + p.p53);
                        let assign3580_e5289: f64 = (assign3580_e5288).sqrt();
                        let assign3580_e5290: f64 = (assign3580_e5279 + assign3580_e5289);
                        let assign3580_e5291: f64 = (0.5 * assign3580_e5290);
                        (assign3580_e5291, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3580_e5289)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3580_e5289)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3580_e5289)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3580_e5285) + (assign3580_e5282 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3580_e5289)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,)
            }
        };
        let assign3580_e5296: f64 = (assign3580_e5294 - locals.var_fn25_calc_iq__myarg0);
        let assign3580_e5298: f64 = (assign3580_e5296 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3580_e5298, (assign3580_e5294_d_n2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3580_e5296 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (assign3580_e5294_d_n7 / locals.var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n16 / locals.var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n17 / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3580_e5300;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3580_e5300_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3580_e5300_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3580_e5300_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3580_e5300_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3580_e5300_d_n17;
        locals.var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3590_e5303: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign3590_e5303;
        locals.var_guard39_rv = 0.0;

        let (assign3600_e5309, assign3600_e5309_d_n2, assign3600_e5309_d_n4, assign3600_e5309_d_n7, assign3600_e5309_d_n16, assign3600_e5309_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard39 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3600_e5309;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3600_e5309_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3600_e5309_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3600_e5309_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3600_e5309_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3600_e5309_d_n17;
        locals.var_fn25_calc_iq__ff0_rv = 0.0;

        let assign3610_e5312: f64 = (-50.0);
        let assign3610_e5313: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3610_e5312 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign3610_e5313;
        locals.var_guard40_rv = 0.0;

        let (assign3620_e5322, assign3620_e5322_d_n2, assign3620_e5322_d_n4, assign3620_e5322_d_n7, assign3620_e5322_d_n16, assign3620_e5322_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard39 == 0.0)) && (locals.var_guard40 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3620_e5322;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3620_e5322_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3620_e5322_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3620_e5322_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3620_e5322_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3620_e5322_d_n17;
        locals.var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign3630_e5337, assign3630_e5337_d_n2, assign3630_e5337_d_n4, assign3630_e5337_d_n7, assign3630_e5337_d_n16, assign3630_e5337_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard39 == 0.0)) && (locals.var_guard40 == 0.0)) {
        let assign3630_e5333: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3630_e5334: f64 = (1.0 + assign3630_e5333);
        let assign3630_e5335: f64 = (1.0 / assign3630_e5334);
        (assign3630_e5335, (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3630_e5334 * assign3630_e5334))),)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3630_e5337;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3630_e5337_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3630_e5337_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3630_e5337_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3630_e5337_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3630_e5337_d_n17;
        locals.var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign3640_e5396, assign3640_e5396_d_n2, assign3640_e5396_d_n4, assign3640_e5396_d_n7, assign3640_e5396_d_n16, assign3640_e5396_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3640_e5382, assign3640_e5382_d_n2, assign3640_e5382_d_n7, assign3640_e5382_d_n16, assign3640_e5382_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3640_e5346: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5349: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5352: f64 = (0.001 / p.p53);
                let assign3640_e5355: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5356: f64 = (assign3640_e5352 * assign3640_e5355);
                let assign3640_e5357: f64 = (assign3640_e5356).tanh();
                let assign3640_e5358: f64 = (assign3640_e5349 * assign3640_e5357);
                let assign3640_e5359: f64 = (assign3640_e5346 + assign3640_e5358);
                let assign3640_e5360: f64 = (0.5 * assign3640_e5359);
                (assign3640_e5360, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))),)
            } else {
                let (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3640_e5367: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5370: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5373: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5374: f64 = (assign3640_e5370 * assign3640_e5373);
                        let assign3640_e5376: f64 = (assign3640_e5374 + p.p53);
                        let assign3640_e5377: f64 = (assign3640_e5376).sqrt();
                        let assign3640_e5378: f64 = (assign3640_e5367 + assign3640_e5377);
                        let assign3640_e5379: f64 = (0.5 * assign3640_e5378);
                        (assign3640_e5379, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3640_e5377)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3640_e5377)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3640_e5377)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3640_e5373) + (assign3640_e5370 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3640_e5377)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,)
            }
        };
        let assign3640_e5386: f64 = (p.p51 * 0.1);
        let assign3640_e5388: f64 = (assign3640_e5386 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3640_e5390: f64 = (assign3640_e5388 * locals.var_fn25_calc_iq__ff0);
        let assign3640_e5391: f64 = (locals.var_fn25_calc_iq__vtof - assign3640_e5390);
        let assign3640_e5392: f64 = (assign3640_e5382 - assign3640_e5391);
        let assign3640_e5394: f64 = (assign3640_e5392 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3640_e5394, ((assign3640_e5382_d_n2 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (((assign3640_e5386 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ff0) + (assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3640_e5392 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), ((assign3640_e5382_d_n7 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n16 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n17 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__eta0, locals.var_fn25_calc_iq__eta0_dn2, locals.var_fn25_calc_iq__eta0_dn4, locals.var_fn25_calc_iq__eta0_dn7, locals.var_fn25_calc_iq__eta0_dn16, locals.var_fn25_calc_iq__eta0_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta0 = assign3640_e5396;
        locals.var_fn25_calc_iq__eta0_dn2 = assign3640_e5396_d_n2;
        locals.var_fn25_calc_iq__eta0_dn4 = assign3640_e5396_d_n4;
        locals.var_fn25_calc_iq__eta0_dn7 = assign3640_e5396_d_n7;
        locals.var_fn25_calc_iq__eta0_dn16 = assign3640_e5396_d_n16;
        locals.var_fn25_calc_iq__eta0_dn17 = assign3640_e5396_d_n17;
        locals.var_fn25_calc_iq__eta0_rv = 0.0;

        let assign3650_e5399: f64 = if locals.var_fn25_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign3650_e5399;
        locals.var_guard41_rv = 0.0;

        let (assign3660_e5407, assign3660_e5407_d_n2, assign3660_e5407_d_n4, assign3660_e5407_d_n7, assign3660_e5407_d_n16, assign3660_e5407_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard41 != 0.0)) {
        let assign3660_e5405: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0);
        (assign3660_e5405, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__eta0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3660_e5407;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3660_e5407_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3660_e5407_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3660_e5407_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3660_e5407_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3660_e5407_d_n17;
        locals.var_fn25_calc_iq__qinvv0_rv = 0.0;

        let assign3670_e5410: f64 = (-50.0);
        let assign3670_e5411: f64 = if locals.var_fn25_calc_iq__eta0 < assign3670_e5410 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign3670_e5411;
        locals.var_guard42_rv = 0.0;

        let (assign3680_e5423, assign3680_e5423_d_n2, assign3680_e5423_d_n4, assign3680_e5423_d_n7, assign3680_e5423_d_n16, assign3680_e5423_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 != 0.0)) {
        let assign3680_e5420: f64 = (locals.var_fn25_calc_iq__eta0).exp();
        let assign3680_e5421: f64 = (locals.var_fn25_calc_iq__qref0 * assign3680_e5420);
        (assign3680_e5421, (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3680_e5420) + (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3680_e5423;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3680_e5423_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3680_e5423_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3680_e5423_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3680_e5423_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3680_e5423_d_n17;
        locals.var_fn25_calc_iq__qinvv0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3690_e5439, assign3690_e5439_d_n2, assign3690_e5439_d_n4, assign3690_e5439_d_n7, assign3690_e5439_d_n16, assign3690_e5439_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 == 0.0)) {
        let assign3690_e5434: f64 = (locals.var_fn25_calc_iq__eta0).exp();
        let assign3690_e5435: f64 = (1.0 + assign3690_e5434);
        let assign3690_e5436: f64 = (assign3690_e5435).ln();
        let assign3690_e5437: f64 = (locals.var_fn25_calc_iq__qref0 * assign3690_e5436);
        (assign3690_e5437, (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn2) / assign3690_e5435)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3690_e5436) + (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn4) / assign3690_e5435))), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn7) / assign3690_e5435)), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn16) / assign3690_e5435)), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn17) / assign3690_e5435)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3690_e5439;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3690_e5439_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3690_e5439_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3690_e5439_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3690_e5439_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3690_e5439_d_n17;
        locals.var_fn25_calc_iq__qinvv0_rv = 0.0;

        let (assign3700_e5445, assign3700_e5445_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3700_e5443: f64 = (locals.var_fn25_calc_iq__mu0 / locals.var_fn25_calc_iq__tfacmobin);
        (assign3700_e5443, (-((locals.var_fn25_calc_iq__mu0 * locals.var_fn25_calc_iq__tfacmobin_dn4) / (locals.var_fn25_calc_iq__tfacmobin * locals.var_fn25_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn25_calc_iq__muf0, locals.var_fn25_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn25_calc_iq__muf0 = assign3700_e5445;
        locals.var_fn25_calc_iq__muf0_dn4 = assign3700_e5445_d_n4;
        locals.var_fn25_calc_iq__muf0_rv = 0.0;

        let (assign3710_e5461, assign3710_e5461_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3710_e5451: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tnomin);
        let assign3710_e5452: f64 = (1.0 + assign3710_e5451);
        let assign3710_e5456: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin);
        let assign3710_e5457: f64 = (1.0 + assign3710_e5456);
        let assign3710_e5458: f64 = (assign3710_e5452 / assign3710_e5457);
        let assign3710_e5459: f64 = (locals.var_fn25_calc_iq__vel0 * assign3710_e5458);
        (assign3710_e5459, (locals.var_fn25_calc_iq__vel0 * (-((assign3710_e5452 * (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin_dn4)) / (assign3710_e5457 * assign3710_e5457)))),)
    } else {
        (locals.var_fn25_calc_iq__vx0, locals.var_fn25_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vx0 = assign3710_e5461;
        locals.var_fn25_calc_iq__vx0_dn4 = assign3710_e5461_d_n4;
        locals.var_fn25_calc_iq__vx0_rv = 0.0;

        let (assign3720_e5469, assign3720_e5469_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3720_e5465: f64 = (locals.var_fn25_calc_iq__vx0 * locals.var_fn25_calc_iq__lin);
        let assign3720_e5467: f64 = (assign3720_e5465 / locals.var_fn25_calc_iq__muf0);
        (assign3720_e5467, ((((locals.var_fn25_calc_iq__vx0_dn4 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf0) - (assign3720_e5465 * locals.var_fn25_calc_iq__muf0_dn4)) / (locals.var_fn25_calc_iq__muf0 * locals.var_fn25_calc_iq__muf0)),)
    } else {
        (locals.var_fn25_calc_iq__vdsats0, locals.var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vdsats0 = assign3720_e5469;
        locals.var_fn25_calc_iq__vdsats0_dn4 = assign3720_e5469_d_n4;
        locals.var_fn25_calc_iq__vdsats0_rv = 0.0;

        let (assign3730_e5486, assign3730_e5486_d_n2, assign3730_e5486_d_n4, assign3730_e5486_d_n7, assign3730_e5486_d_n16, assign3730_e5486_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3730_e5475: f64 = (2.0 * locals.var_fn25_calc_iq__qinvv0);
        let assign3730_e5477: f64 = (assign3730_e5475 / locals.var_fn25_calc_iq__cgin);
        let assign3730_e5479: f64 = (assign3730_e5477 / locals.var_fn25_calc_iq__vdsats0);
        let assign3730_e5480: f64 = (1.0 + assign3730_e5479);
        let assign3730_e5481: f64 = (assign3730_e5480).sqrt();
        let assign3730_e5482: f64 = (locals.var_fn25_calc_iq__vdsats0 * assign3730_e5481);
        let assign3730_e5484: f64 = (assign3730_e5482 - locals.var_fn25_calc_iq__vdsats0);
        (assign3730_e5484, (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn2) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (((locals.var_fn25_calc_iq__vdsats0_dn4 * assign3730_e5481) + (locals.var_fn25_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3730_e5475 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)) * locals.var_fn25_calc_iq__vdsats0) - (assign3730_e5477 * locals.var_fn25_calc_iq__vdsats0_dn4)) / (locals.var_fn25_calc_iq__vdsats0 * locals.var_fn25_calc_iq__vdsats0)) / (2.0 * assign3730_e5481)))) - locals.var_fn25_calc_iq__vdsats0_dn4), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn7) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn16) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn17) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))),)
    } else {
        (locals.var_fn25_calc_iq__vdsats10, locals.var_fn25_calc_iq__vdsats10_dn2, locals.var_fn25_calc_iq__vdsats10_dn4, locals.var_fn25_calc_iq__vdsats10_dn7, locals.var_fn25_calc_iq__vdsats10_dn16, locals.var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats10 = assign3730_e5486;
        locals.var_fn25_calc_iq__vdsats10_dn2 = assign3730_e5486_d_n2;
        locals.var_fn25_calc_iq__vdsats10_dn4 = assign3730_e5486_d_n4;
        locals.var_fn25_calc_iq__vdsats10_dn7 = assign3730_e5486_d_n7;
        locals.var_fn25_calc_iq__vdsats10_dn16 = assign3730_e5486_d_n16;
        locals.var_fn25_calc_iq__vdsats10_dn17 = assign3730_e5486_d_n17;
        locals.var_fn25_calc_iq__vdsats10_rv = 0.0;

        let (assign3740_e5498, assign3740_e5498_d_n2, assign3740_e5498_d_n4, assign3740_e5498_d_n7, assign3740_e5498_d_n16, assign3740_e5498_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3740_e5491: f64 = (1.0 - locals.var_fn25_calc_iq__ff0);
        let assign3740_e5492: f64 = (locals.var_fn25_calc_iq__vdsats10 * assign3740_e5491);
        let assign3740_e5495: f64 = (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0);
        let assign3740_e5496: f64 = (assign3740_e5492 + assign3740_e5495);
        (assign3740_e5496, (((locals.var_fn25_calc_iq__vdsats10_dn2 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn2))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn2)), (((locals.var_fn25_calc_iq__vdsats10_dn4 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit0_dn4 * locals.var_fn25_calc_iq__ff0) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn4))), (((locals.var_fn25_calc_iq__vdsats10_dn7 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn7))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn7)), (((locals.var_fn25_calc_iq__vdsats10_dn16 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn16))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn16)), (((locals.var_fn25_calc_iq__vdsats10_dn17 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn17))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdsat10, locals.var_fn25_calc_iq__vdsat10_dn2, locals.var_fn25_calc_iq__vdsat10_dn4, locals.var_fn25_calc_iq__vdsat10_dn7, locals.var_fn25_calc_iq__vdsat10_dn16, locals.var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat10 = assign3740_e5498;
        locals.var_fn25_calc_iq__vdsat10_dn2 = assign3740_e5498_d_n2;
        locals.var_fn25_calc_iq__vdsat10_dn4 = assign3740_e5498_d_n4;
        locals.var_fn25_calc_iq__vdsat10_dn7 = assign3740_e5498_d_n7;
        locals.var_fn25_calc_iq__vdsat10_dn16 = assign3740_e5498_d_n16;
        locals.var_fn25_calc_iq__vdsat10_dn17 = assign3740_e5498_d_n17;
        locals.var_fn25_calc_iq__vdsat10_rv = 0.0;

        let (assign3750_e5567, assign3750_e5567_d_n2, assign3750_e5567_d_n4, assign3750_e5567_d_n7, assign3750_e5567_d_n16, assign3750_e5567_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3750_e5557, assign3750_e5557_d_n2, assign3750_e5557_d_n4, assign3750_e5557_d_n7, assign3750_e5557_d_n16, assign3750_e5557_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3750_e5510: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5511: f64 = assign3750_e5510;
                let assign3750_e5515: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5516: f64 = (-assign3750_e5515);
                let assign3750_e5519: f64 = (0.001 / p.p53);
                let assign3750_e5523: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5524: f64 = (-assign3750_e5523);
                let assign3750_e5525: f64 = (assign3750_e5519 * assign3750_e5524);
                let assign3750_e5526: f64 = (assign3750_e5525).tanh();
                let assign3750_e5527: f64 = (assign3750_e5516 * assign3750_e5526);
                let assign3750_e5528: f64 = (assign3750_e5511 + assign3750_e5527);
                let assign3750_e5529: f64 = (0.5 * assign3750_e5528);
                (assign3750_e5529, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))),)
            } else {
                let (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3750_e5537: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5538: f64 = assign3750_e5537;
                        let assign3750_e5542: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5543: f64 = (-assign3750_e5542);
                        let assign3750_e5547: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5548: f64 = (-assign3750_e5547);
                        let assign3750_e5549: f64 = (assign3750_e5543 * assign3750_e5548);
                        let assign3750_e5551: f64 = (assign3750_e5549 + p.p53);
                        let assign3750_e5552: f64 = (assign3750_e5551).sqrt();
                        let assign3750_e5553: f64 = (assign3750_e5538 + assign3750_e5552);
                        let assign3750_e5554: f64 = (0.5 * assign3750_e5553);
                        (assign3750_e5554, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,)
            }
        };
        let assign3750_e5559: f64 = (assign3750_e5557).powf(locals.var_fn25_calc_iq__beta);
        let assign3750_e5560: f64 = (1.0 + assign3750_e5559);
        let assign3750_e5563: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3750_e5564: f64 = (assign3750_e5560).powf(assign3750_e5563);
        let assign3750_e5565: f64 = (1.0 / assign3750_e5564);
        (assign3750_e5565, (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))),)
    } else {
        (locals.var_fn25_calc_iq__fsd0, locals.var_fn25_calc_iq__fsd0_dn2, locals.var_fn25_calc_iq__fsd0_dn4, locals.var_fn25_calc_iq__fsd0_dn7, locals.var_fn25_calc_iq__fsd0_dn16, locals.var_fn25_calc_iq__fsd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd0 = assign3750_e5567;
        locals.var_fn25_calc_iq__fsd0_dn2 = assign3750_e5567_d_n2;
        locals.var_fn25_calc_iq__fsd0_dn4 = assign3750_e5567_d_n4;
        locals.var_fn25_calc_iq__fsd0_dn7 = assign3750_e5567_d_n7;
        locals.var_fn25_calc_iq__fsd0_dn16 = assign3750_e5567_d_n16;
        locals.var_fn25_calc_iq__fsd0_dn17 = assign3750_e5567_d_n17;
        locals.var_fn25_calc_iq__fsd0_rv = 0.0;

        let (assign3760_e5573, assign3760_e5573_d_n2, assign3760_e5573_d_n4, assign3760_e5573_d_n7, assign3760_e5573_d_n16, assign3760_e5573_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3760_e5571: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0);
        (assign3760_e5571, (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn2), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn4), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn7), ((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__fsd0) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn16)), ((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__fsd0) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdx0, locals.var_fn25_calc_iq__vdx0_dn2, locals.var_fn25_calc_iq__vdx0_dn4, locals.var_fn25_calc_iq__vdx0_dn7, locals.var_fn25_calc_iq__vdx0_dn16, locals.var_fn25_calc_iq__vdx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx0 = assign3760_e5573;
        locals.var_fn25_calc_iq__vdx0_dn2 = assign3760_e5573_d_n2;
        locals.var_fn25_calc_iq__vdx0_dn4 = assign3760_e5573_d_n4;
        locals.var_fn25_calc_iq__vdx0_dn7 = assign3760_e5573_d_n7;
        locals.var_fn25_calc_iq__vdx0_dn16 = assign3760_e5573_d_n16;
        locals.var_fn25_calc_iq__vdx0_dn17 = assign3760_e5573_d_n17;
        locals.var_fn25_calc_iq__vdx0_rv = 0.0;

        let (assign3770_e5648, assign3770_e5648_d_n2, assign3770_e5648_d_n4, assign3770_e5648_d_n7, assign3770_e5648_d_n16, assign3770_e5648_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3770_e5638, assign3770_e5638_d_n2, assign3770_e5638_d_n4, assign3770_e5638_d_n7, assign3770_e5638_d_n16, assign3770_e5638_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3770_e5584: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5586: f64 = (assign3770_e5584 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5587: f64 = assign3770_e5586;
                let assign3770_e5590: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5592: f64 = (assign3770_e5590 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5593: f64 = (-assign3770_e5592);
                let assign3770_e5596: f64 = (0.001 / p.p53);
                let assign3770_e5599: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5601: f64 = (assign3770_e5599 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5602: f64 = (-assign3770_e5601);
                let assign3770_e5603: f64 = (assign3770_e5596 * assign3770_e5602);
                let assign3770_e5604: f64 = (assign3770_e5603).tanh();
                let assign3770_e5605: f64 = (assign3770_e5593 * assign3770_e5604);
                let assign3770_e5606: f64 = (assign3770_e5587 + assign3770_e5605);
                let assign3770_e5607: f64 = (0.5 * assign3770_e5606);
                (assign3770_e5607, (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))),)
            } else {
                let (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3770_e5614: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5616: f64 = (assign3770_e5614 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5617: f64 = assign3770_e5616;
                        let assign3770_e5620: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5622: f64 = (assign3770_e5620 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5623: f64 = (-assign3770_e5622);
                        let assign3770_e5626: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5628: f64 = (assign3770_e5626 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5629: f64 = (-assign3770_e5628);
                        let assign3770_e5630: f64 = (assign3770_e5623 * assign3770_e5629);
                        let assign3770_e5632: f64 = (assign3770_e5630 + p.p53);
                        let assign3770_e5633: f64 = (assign3770_e5632).sqrt();
                        let assign3770_e5634: f64 = (assign3770_e5617 + assign3770_e5633);
                        let assign3770_e5635: f64 = (0.5 * assign3770_e5634);
                        (assign3770_e5635, (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,)
            }
        };
        let assign3770_e5640: f64 = (assign3770_e5638).powf(locals.var_fn25_calc_iq__beta);
        let assign3770_e5641: f64 = (1.0 + assign3770_e5640);
        let assign3770_e5644: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3770_e5645: f64 = (assign3770_e5641).powf(assign3770_e5644);
        let assign3770_e5646: f64 = (1.0 / assign3770_e5645);
        (assign3770_e5646, (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))),)
    } else {
        (locals.var_fn25_calc_iq__fds0, locals.var_fn25_calc_iq__fds0_dn2, locals.var_fn25_calc_iq__fds0_dn4, locals.var_fn25_calc_iq__fds0_dn7, locals.var_fn25_calc_iq__fds0_dn16, locals.var_fn25_calc_iq__fds0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds0 = assign3770_e5648;
        locals.var_fn25_calc_iq__fds0_dn2 = assign3770_e5648_d_n2;
        locals.var_fn25_calc_iq__fds0_dn4 = assign3770_e5648_d_n4;
        locals.var_fn25_calc_iq__fds0_dn7 = assign3770_e5648_d_n7;
        locals.var_fn25_calc_iq__fds0_dn16 = assign3770_e5648_d_n16;
        locals.var_fn25_calc_iq__fds0_dn17 = assign3770_e5648_d_n17;
        locals.var_fn25_calc_iq__fds0_rv = 0.0;

        let (assign3780_e5655, assign3780_e5655_d_n2, assign3780_e5655_d_n4, assign3780_e5655_d_n7, assign3780_e5655_d_n16, assign3780_e5655_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3780_e5651: f64 = (-locals.var_fn25_calc_iq__vdsin);
        let assign3780_e5653: f64 = (assign3780_e5651 * locals.var_fn25_calc_iq__fds0);
        (assign3780_e5653, (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn2), (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn4), (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn7), (((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__fds0) + (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn16)), (((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__fds0) + (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vsx0, locals.var_fn25_calc_iq__vsx0_dn2, locals.var_fn25_calc_iq__vsx0_dn4, locals.var_fn25_calc_iq__vsx0_dn7, locals.var_fn25_calc_iq__vsx0_dn16, locals.var_fn25_calc_iq__vsx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx0 = assign3780_e5655;
        locals.var_fn25_calc_iq__vsx0_dn2 = assign3780_e5655_d_n2;
        locals.var_fn25_calc_iq__vsx0_dn4 = assign3780_e5655_d_n4;
        locals.var_fn25_calc_iq__vsx0_dn7 = assign3780_e5655_d_n7;
        locals.var_fn25_calc_iq__vsx0_dn16 = assign3780_e5655_d_n16;
        locals.var_fn25_calc_iq__vsx0_dn17 = assign3780_e5655_d_n17;
        locals.var_fn25_calc_iq__vsx0_rv = 0.0;

        let (assign3790_e5663, assign3790_e5663_d_n2, assign3790_e5663_d_n4, assign3790_e5663_d_n7, assign3790_e5663_d_n16, assign3790_e5663_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3790_e5659: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__myarg0);
        let assign3790_e5661: f64 = (assign3790_e5659 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3790_e5661, (locals.var_fn25_calc_iq__vgsin_dn2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3790_e5659 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (locals.var_fn25_calc_iq__vgsin_dn7 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgsin_dn16 / locals.var_fn25_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3790_e5663;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3790_e5663_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3790_e5663_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3790_e5663_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3790_e5663_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3790_e5663_d_n17;
        locals.var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3800_e5666: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign3800_e5666;
        locals.var_guard43_rv = 0.0;

        let (assign3810_e5672, assign3810_e5672_d_n2, assign3810_e5672_d_n4, assign3810_e5672_d_n7, assign3810_e5672_d_n16, assign3810_e5672_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard43 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3810_e5672;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3810_e5672_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3810_e5672_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3810_e5672_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3810_e5672_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3810_e5672_d_n17;
        locals.var_fn25_calc_iq__ffs0_rv = 0.0;

        let assign3820_e5675: f64 = (-50.0);
        let assign3820_e5676: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3820_e5675 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign3820_e5676;
        locals.var_guard44_rv = 0.0;

        let (assign3830_e5685, assign3830_e5685_d_n2, assign3830_e5685_d_n4, assign3830_e5685_d_n7, assign3830_e5685_d_n16, assign3830_e5685_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard43 == 0.0)) && (locals.var_guard44 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3830_e5685;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3830_e5685_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3830_e5685_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3830_e5685_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3830_e5685_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3830_e5685_d_n17;
        locals.var_fn25_calc_iq__ffs0_rv = 0.0;

        let (assign3840_e5700, assign3840_e5700_d_n2, assign3840_e5700_d_n4, assign3840_e5700_d_n7, assign3840_e5700_d_n16, assign3840_e5700_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard43 == 0.0)) && (locals.var_guard44 == 0.0)) {
        let assign3840_e5696: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3840_e5697: f64 = (1.0 + assign3840_e5696);
        let assign3840_e5698: f64 = (1.0 / assign3840_e5697);
        (assign3840_e5698, (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3840_e5697 * assign3840_e5697))),)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3840_e5700;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3840_e5700_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3840_e5700_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3840_e5700_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3840_e5700_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3840_e5700_d_n17;
        locals.var_fn25_calc_iq__ffs0_rv = 0.0;

        let (assign3850_e5718, assign3850_e5718_d_n2, assign3850_e5718_d_n4, assign3850_e5718_d_n7, assign3850_e5718_d_n16, assign3850_e5718_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3850_e5704: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__vsx0);
        let assign3850_e5708: f64 = (p.p51 * 0.1);
        let assign3850_e5710: f64 = (assign3850_e5708 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3850_e5712: f64 = (assign3850_e5710 * locals.var_fn25_calc_iq__ffs0);
        let assign3850_e5713: f64 = (locals.var_fn25_calc_iq__vtof - assign3850_e5712);
        let assign3850_e5714: f64 = (assign3850_e5704 - assign3850_e5713);
        let assign3850_e5716: f64 = (assign3850_e5714 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3850_e5716, (((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__vsx0_dn2) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), (((((-locals.var_fn25_calc_iq__vsx0_dn4) - (locals.var_fn25_calc_iq__vtof_dn4 - (((assign3850_e5708 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffs0) + (assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3850_e5714 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__vsx0_dn7) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__vsx0_dn16) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__vsx0_dn17) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etas0, locals.var_fn25_calc_iq__etas0_dn2, locals.var_fn25_calc_iq__etas0_dn4, locals.var_fn25_calc_iq__etas0_dn7, locals.var_fn25_calc_iq__etas0_dn16, locals.var_fn25_calc_iq__etas0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas0 = assign3850_e5718;
        locals.var_fn25_calc_iq__etas0_dn2 = assign3850_e5718_d_n2;
        locals.var_fn25_calc_iq__etas0_dn4 = assign3850_e5718_d_n4;
        locals.var_fn25_calc_iq__etas0_dn7 = assign3850_e5718_d_n7;
        locals.var_fn25_calc_iq__etas0_dn16 = assign3850_e5718_d_n16;
        locals.var_fn25_calc_iq__etas0_dn17 = assign3850_e5718_d_n17;
        locals.var_fn25_calc_iq__etas0_rv = 0.0;

        let assign3860_e5721: f64 = if locals.var_fn25_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign3860_e5721;
        locals.var_guard45_rv = 0.0;

        let (assign3870_e5729, assign3870_e5729_d_n2, assign3870_e5729_d_n4, assign3870_e5729_d_n7, assign3870_e5729_d_n16, assign3870_e5729_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign3870_e5727: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0);
        (assign3870_e5727, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__etas0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3870_e5729;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3870_e5729_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3870_e5729_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3870_e5729_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3870_e5729_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3870_e5729_d_n17;
        locals.var_fn25_calc_iq__qinvs0_rv = 0.0;

        let assign3880_e5732: f64 = (-50.0);
        let assign3880_e5733: f64 = if locals.var_fn25_calc_iq__etas0 < assign3880_e5732 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign3880_e5733;
        locals.var_guard46_rv = 0.0;

        let (assign3890_e5745, assign3890_e5745_d_n2, assign3890_e5745_d_n4, assign3890_e5745_d_n7, assign3890_e5745_d_n16, assign3890_e5745_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard45 == 0.0)) && (locals.var_guard46 != 0.0)) {
        let assign3890_e5742: f64 = (locals.var_fn25_calc_iq__etas0).exp();
        let assign3890_e5743: f64 = (locals.var_fn25_calc_iq__qref0 * assign3890_e5742);
        (assign3890_e5743, (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3890_e5742) + (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3890_e5745;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3890_e5745_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3890_e5745_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3890_e5745_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3890_e5745_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3890_e5745_d_n17;
        locals.var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign3900_e5761, assign3900_e5761_d_n2, assign3900_e5761_d_n4, assign3900_e5761_d_n7, assign3900_e5761_d_n16, assign3900_e5761_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard45 == 0.0)) && (locals.var_guard46 == 0.0)) {
        let assign3900_e5756: f64 = (locals.var_fn25_calc_iq__etas0).exp();
        let assign3900_e5757: f64 = (1.0 + assign3900_e5756);
        let assign3900_e5758: f64 = (assign3900_e5757).ln();
        let assign3900_e5759: f64 = (locals.var_fn25_calc_iq__qref0 * assign3900_e5758);
        (assign3900_e5759, (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn2) / assign3900_e5757)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3900_e5758) + (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn4) / assign3900_e5757))), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn7) / assign3900_e5757)), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn16) / assign3900_e5757)), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn17) / assign3900_e5757)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3900_e5761;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3900_e5761_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3900_e5761_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3900_e5761_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3900_e5761_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3900_e5761_d_n17;
        locals.var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign3910_e5769, assign3910_e5769_d_n2, assign3910_e5769_d_n4, assign3910_e5769_d_n7, assign3910_e5769_d_n16, assign3910_e5769_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3910_e5765: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__myarg0);
        let assign3910_e5767: f64 = (assign3910_e5765 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3910_e5767, (locals.var_fn25_calc_iq__vgdin_dn2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3910_e5765 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (locals.var_fn25_calc_iq__vgdin_dn7 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgdin_dn16 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgdin_dn17 / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3910_e5769;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3910_e5769_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3910_e5769_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3910_e5769_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3910_e5769_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3910_e5769_d_n17;
        locals.var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3920_e5772: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign3920_e5772;
        locals.var_guard47_rv = 0.0;

        let (assign3930_e5778, assign3930_e5778_d_n2, assign3930_e5778_d_n4, assign3930_e5778_d_n7, assign3930_e5778_d_n16, assign3930_e5778_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard47 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3930_e5778;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3930_e5778_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3930_e5778_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3930_e5778_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3930_e5778_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3930_e5778_d_n17;
        locals.var_fn25_calc_iq__ffd0_rv = 0.0;

        let assign3940_e5781: f64 = (-50.0);
        let assign3940_e5782: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3940_e5781 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign3940_e5782;
        locals.var_guard48_rv = 0.0;

        let (assign3950_e5791, assign3950_e5791_d_n2, assign3950_e5791_d_n4, assign3950_e5791_d_n7, assign3950_e5791_d_n16, assign3950_e5791_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard47 == 0.0)) && (locals.var_guard48 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3950_e5791;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3950_e5791_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3950_e5791_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3950_e5791_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3950_e5791_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3950_e5791_d_n17;
        locals.var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign3960_e5806, assign3960_e5806_d_n2, assign3960_e5806_d_n4, assign3960_e5806_d_n7, assign3960_e5806_d_n16, assign3960_e5806_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard47 == 0.0)) && (locals.var_guard48 == 0.0)) {
        let assign3960_e5802: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3960_e5803: f64 = (1.0 + assign3960_e5802);
        let assign3960_e5804: f64 = (1.0 / assign3960_e5803);
        (assign3960_e5804, (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3960_e5803 * assign3960_e5803))),)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3960_e5806;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3960_e5806_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3960_e5806_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3960_e5806_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3960_e5806_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3960_e5806_d_n17;
        locals.var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign3970_e5824, assign3970_e5824_d_n2, assign3970_e5824_d_n4, assign3970_e5824_d_n7, assign3970_e5824_d_n16, assign3970_e5824_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3970_e5810: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdx0);
        let assign3970_e5814: f64 = (p.p51 * 0.1);
        let assign3970_e5816: f64 = (assign3970_e5814 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3970_e5818: f64 = (assign3970_e5816 * locals.var_fn25_calc_iq__ffd0);
        let assign3970_e5819: f64 = (locals.var_fn25_calc_iq__vtof - assign3970_e5818);
        let assign3970_e5820: f64 = (assign3970_e5810 - assign3970_e5819);
        let assign3970_e5822: f64 = (assign3970_e5820 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3970_e5822, (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vdx0_dn2) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), (((((-locals.var_fn25_calc_iq__vdx0_dn4) - (locals.var_fn25_calc_iq__vtof_dn4 - (((assign3970_e5814 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffd0) + (assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3970_e5820 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vdx0_dn7) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdx0_dn16) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), (((-locals.var_fn25_calc_iq__vdx0_dn17) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etad0, locals.var_fn25_calc_iq__etad0_dn2, locals.var_fn25_calc_iq__etad0_dn4, locals.var_fn25_calc_iq__etad0_dn7, locals.var_fn25_calc_iq__etad0_dn16, locals.var_fn25_calc_iq__etad0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad0 = assign3970_e5824;
        locals.var_fn25_calc_iq__etad0_dn2 = assign3970_e5824_d_n2;
        locals.var_fn25_calc_iq__etad0_dn4 = assign3970_e5824_d_n4;
        locals.var_fn25_calc_iq__etad0_dn7 = assign3970_e5824_d_n7;
        locals.var_fn25_calc_iq__etad0_dn16 = assign3970_e5824_d_n16;
        locals.var_fn25_calc_iq__etad0_dn17 = assign3970_e5824_d_n17;
        locals.var_fn25_calc_iq__etad0_rv = 0.0;

        let assign3980_e5827: f64 = if locals.var_fn25_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3980_e5827;
        locals.var_guard49_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3990_e5835, assign3990_e5835_d_n2, assign3990_e5835_d_n4, assign3990_e5835_d_n7, assign3990_e5835_d_n16, assign3990_e5835_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign3990_e5833: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0);
        (assign3990_e5833, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__etad0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign3990_e5835;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign3990_e5835_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign3990_e5835_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign3990_e5835_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign3990_e5835_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign3990_e5835_d_n17;
        locals.var_fn25_calc_iq__qinvd0_rv = 0.0;

        let assign4000_e5838: f64 = (-50.0);
        let assign4000_e5839: f64 = if locals.var_fn25_calc_iq__etad0 < assign4000_e5838 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign4000_e5839;
        locals.var_guard50_rv = 0.0;

        let (assign4010_e5851, assign4010_e5851_d_n2, assign4010_e5851_d_n4, assign4010_e5851_d_n7, assign4010_e5851_d_n16, assign4010_e5851_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard50 != 0.0)) {
        let assign4010_e5848: f64 = (locals.var_fn25_calc_iq__etad0).exp();
        let assign4010_e5849: f64 = (locals.var_fn25_calc_iq__qref0 * assign4010_e5848);
        (assign4010_e5849, (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign4010_e5848) + (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign4010_e5851;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign4010_e5851_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign4010_e5851_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign4010_e5851_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign4010_e5851_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign4010_e5851_d_n17;
        locals.var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign4020_e5867, assign4020_e5867_d_n2, assign4020_e5867_d_n4, assign4020_e5867_d_n7, assign4020_e5867_d_n16, assign4020_e5867_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard50 == 0.0)) {
        let assign4020_e5862: f64 = (locals.var_fn25_calc_iq__etad0).exp();
        let assign4020_e5863: f64 = (1.0 + assign4020_e5862);
        let assign4020_e5864: f64 = (assign4020_e5863).ln();
        let assign4020_e5865: f64 = (locals.var_fn25_calc_iq__qref0 * assign4020_e5864);
        (assign4020_e5865, (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn2) / assign4020_e5863)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign4020_e5864) + (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn4) / assign4020_e5863))), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn7) / assign4020_e5863)), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn16) / assign4020_e5863)), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn17) / assign4020_e5863)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign4020_e5867;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign4020_e5867_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign4020_e5867_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign4020_e5867_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign4020_e5867_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign4020_e5867_d_n17;
        locals.var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign4030_e5875, assign4030_e5875_d_n2, assign4030_e5875_d_n4, assign4030_e5875_d_n7, assign4030_e5875_d_n16, assign4030_e5875_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4030_e5871: f64 = (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0);
        let assign4030_e5873: f64 = (assign4030_e5871 + 1e-38);
        (assign4030_e5873, ((locals.var_fn25_calc_iq__qinvs0_dn2 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn2)), ((locals.var_fn25_calc_iq__qinvs0_dn4 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn4)), ((locals.var_fn25_calc_iq__qinvs0_dn7 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn7)), ((locals.var_fn25_calc_iq__qinvs0_dn16 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn16)), ((locals.var_fn25_calc_iq__qinvs0_dn17 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qs2, locals.var_fn25_calc_iq__qs2_dn2, locals.var_fn25_calc_iq__qs2_dn4, locals.var_fn25_calc_iq__qs2_dn7, locals.var_fn25_calc_iq__qs2_dn16, locals.var_fn25_calc_iq__qs2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs2 = assign4030_e5875;
        locals.var_fn25_calc_iq__qs2_dn2 = assign4030_e5875_d_n2;
        locals.var_fn25_calc_iq__qs2_dn4 = assign4030_e5875_d_n4;
        locals.var_fn25_calc_iq__qs2_dn7 = assign4030_e5875_d_n7;
        locals.var_fn25_calc_iq__qs2_dn16 = assign4030_e5875_d_n16;
        locals.var_fn25_calc_iq__qs2_dn17 = assign4030_e5875_d_n17;
        locals.var_fn25_calc_iq__qs2_rv = 0.0;

        let (assign4040_e5883, assign4040_e5883_d_n2, assign4040_e5883_d_n4, assign4040_e5883_d_n7, assign4040_e5883_d_n16, assign4040_e5883_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4040_e5879: f64 = (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0);
        let assign4040_e5881: f64 = (assign4040_e5879 + 1e-57);
        (assign4040_e5881, ((locals.var_fn25_calc_iq__qs2_dn2 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn2)), ((locals.var_fn25_calc_iq__qs2_dn4 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn4)), ((locals.var_fn25_calc_iq__qs2_dn7 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn7)), ((locals.var_fn25_calc_iq__qs2_dn16 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn16)), ((locals.var_fn25_calc_iq__qs2_dn17 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qs3, locals.var_fn25_calc_iq__qs3_dn2, locals.var_fn25_calc_iq__qs3_dn4, locals.var_fn25_calc_iq__qs3_dn7, locals.var_fn25_calc_iq__qs3_dn16, locals.var_fn25_calc_iq__qs3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs3 = assign4040_e5883;
        locals.var_fn25_calc_iq__qs3_dn2 = assign4040_e5883_d_n2;
        locals.var_fn25_calc_iq__qs3_dn4 = assign4040_e5883_d_n4;
        locals.var_fn25_calc_iq__qs3_dn7 = assign4040_e5883_d_n7;
        locals.var_fn25_calc_iq__qs3_dn16 = assign4040_e5883_d_n16;
        locals.var_fn25_calc_iq__qs3_dn17 = assign4040_e5883_d_n17;
        locals.var_fn25_calc_iq__qs3_rv = 0.0;

        let (assign4050_e5891, assign4050_e5891_d_n2, assign4050_e5891_d_n4, assign4050_e5891_d_n7, assign4050_e5891_d_n16, assign4050_e5891_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4050_e5887: f64 = (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0);
        let assign4050_e5889: f64 = (assign4050_e5887 + 1e-38);
        (assign4050_e5889, ((locals.var_fn25_calc_iq__qinvd0_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qinvd0_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qinvd0_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qinvd0_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qinvd0_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qd2, locals.var_fn25_calc_iq__qd2_dn2, locals.var_fn25_calc_iq__qd2_dn4, locals.var_fn25_calc_iq__qd2_dn7, locals.var_fn25_calc_iq__qd2_dn16, locals.var_fn25_calc_iq__qd2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd2 = assign4050_e5891;
        locals.var_fn25_calc_iq__qd2_dn2 = assign4050_e5891_d_n2;
        locals.var_fn25_calc_iq__qd2_dn4 = assign4050_e5891_d_n4;
        locals.var_fn25_calc_iq__qd2_dn7 = assign4050_e5891_d_n7;
        locals.var_fn25_calc_iq__qd2_dn16 = assign4050_e5891_d_n16;
        locals.var_fn25_calc_iq__qd2_dn17 = assign4050_e5891_d_n17;
        locals.var_fn25_calc_iq__qd2_rv = 0.0;

        let (assign4060_e5899, assign4060_e5899_d_n2, assign4060_e5899_d_n4, assign4060_e5899_d_n7, assign4060_e5899_d_n16, assign4060_e5899_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4060_e5895: f64 = (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0);
        let assign4060_e5897: f64 = (assign4060_e5895 + 1e-57);
        (assign4060_e5897, ((locals.var_fn25_calc_iq__qd2_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qd2_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qd2_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qd2_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qd2_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qd3, locals.var_fn25_calc_iq__qd3_dn2, locals.var_fn25_calc_iq__qd3_dn4, locals.var_fn25_calc_iq__qd3_dn7, locals.var_fn25_calc_iq__qd3_dn16, locals.var_fn25_calc_iq__qd3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd3 = assign4060_e5899;
        locals.var_fn25_calc_iq__qd3_dn2 = assign4060_e5899_d_n2;
        locals.var_fn25_calc_iq__qd3_dn4 = assign4060_e5899_d_n4;
        locals.var_fn25_calc_iq__qd3_dn7 = assign4060_e5899_d_n7;
        locals.var_fn25_calc_iq__qd3_dn16 = assign4060_e5899_d_n16;
        locals.var_fn25_calc_iq__qd3_dn17 = assign4060_e5899_d_n17;
        locals.var_fn25_calc_iq__qd3_rv = 0.0;

        let (assign4070_e5907, assign4070_e5907_d_n2, assign4070_e5907_d_n4, assign4070_e5907_d_n7, assign4070_e5907_d_n16, assign4070_e5907_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4070_e5903: f64 = (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0);
        let assign4070_e5905: f64 = (assign4070_e5903 + 1e-38);
        (assign4070_e5905, ((locals.var_fn25_calc_iq__qinvs0_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qinvs0_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qinvs0_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qinvs0_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qinvs0_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qsqd, locals.var_fn25_calc_iq__qsqd_dn2, locals.var_fn25_calc_iq__qsqd_dn4, locals.var_fn25_calc_iq__qsqd_dn7, locals.var_fn25_calc_iq__qsqd_dn16, locals.var_fn25_calc_iq__qsqd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsqd = assign4070_e5907;
        locals.var_fn25_calc_iq__qsqd_dn2 = assign4070_e5907_d_n2;
        locals.var_fn25_calc_iq__qsqd_dn4 = assign4070_e5907_d_n4;
        locals.var_fn25_calc_iq__qsqd_dn7 = assign4070_e5907_d_n7;
        locals.var_fn25_calc_iq__qsqd_dn16 = assign4070_e5907_d_n16;
        locals.var_fn25_calc_iq__qsqd_dn17 = assign4070_e5907_d_n17;
        locals.var_fn25_calc_iq__qsqd_rv = 0.0;

        let (assign4080_e5925, assign4080_e5925_d_n2, assign4080_e5925_d_n4, assign4080_e5925_d_n7, assign4080_e5925_d_n16, assign4080_e5925_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4080_e5911: f64 = (2.0 / 3.0);
        let assign4080_e5914: f64 = (locals.var_fn25_calc_iq__qs2 + locals.var_fn25_calc_iq__qd2);
        let assign4080_e5916: f64 = (assign4080_e5914 + locals.var_fn25_calc_iq__qsqd);
        let assign4080_e5917: f64 = (assign4080_e5911 * assign4080_e5916);
        let assign4080_e5920: f64 = (locals.var_fn25_calc_iq__qinvs0 + locals.var_fn25_calc_iq__qinvd0);
        let assign4080_e5922: f64 = (assign4080_e5920 + 2e-19);
        let assign4080_e5923: f64 = (assign4080_e5917 / assign4080_e5922);
        (assign4080_e5923, ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn2 + locals.var_fn25_calc_iq__qd2_dn2) + locals.var_fn25_calc_iq__qsqd_dn2)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn2 + locals.var_fn25_calc_iq__qinvd0_dn2))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn4 + locals.var_fn25_calc_iq__qd2_dn4) + locals.var_fn25_calc_iq__qsqd_dn4)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn4 + locals.var_fn25_calc_iq__qinvd0_dn4))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn7 + locals.var_fn25_calc_iq__qd2_dn7) + locals.var_fn25_calc_iq__qsqd_dn7)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn7 + locals.var_fn25_calc_iq__qinvd0_dn7))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn16 + locals.var_fn25_calc_iq__qd2_dn16) + locals.var_fn25_calc_iq__qsqd_dn16)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn16 + locals.var_fn25_calc_iq__qinvd0_dn16))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn17 + locals.var_fn25_calc_iq__qd2_dn17) + locals.var_fn25_calc_iq__qsqd_dn17)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn17 + locals.var_fn25_calc_iq__qinvd0_dn17))) / (assign4080_e5922 * assign4080_e5922)),)
    } else {
        (locals.var_fn25_calc_iq__qinvdd, locals.var_fn25_calc_iq__qinvdd_dn2, locals.var_fn25_calc_iq__qinvdd_dn4, locals.var_fn25_calc_iq__qinvdd_dn7, locals.var_fn25_calc_iq__qinvdd_dn16, locals.var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvdd = assign4080_e5925;
        locals.var_fn25_calc_iq__qinvdd_dn2 = assign4080_e5925_d_n2;
        locals.var_fn25_calc_iq__qinvdd_dn4 = assign4080_e5925_d_n4;
        locals.var_fn25_calc_iq__qinvdd_dn7 = assign4080_e5925_d_n7;
        locals.var_fn25_calc_iq__qinvdd_dn16 = assign4080_e5925_d_n16;
        locals.var_fn25_calc_iq__qinvdd_dn17 = assign4080_e5925_d_n17;
        locals.var_fn25_calc_iq__qinvdd_rv = 0.0;

        let (assign4090_e5959, assign4090_e5959_d_n2, assign4090_e5959_d_n4, assign4090_e5959_d_n7, assign4090_e5959_d_n16, assign4090_e5959_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4090_e5930: f64 = (2.0 * locals.var_fn25_calc_iq__qs3);
        let assign4090_e5933: f64 = (3.0 * locals.var_fn25_calc_iq__qd3);
        let assign4090_e5934: f64 = (assign4090_e5930 + assign4090_e5933);
        let assign4090_e5937: f64 = (4.0 * locals.var_fn25_calc_iq__qs2);
        let assign4090_e5939: f64 = (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0);
        let assign4090_e5940: f64 = (assign4090_e5934 + assign4090_e5939);
        let assign4090_e5943: f64 = (6.0 * locals.var_fn25_calc_iq__qd2);
        let assign4090_e5945: f64 = (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0);
        let assign4090_e5946: f64 = (assign4090_e5940 + assign4090_e5945);
        let assign4090_e5947: f64 = (2.0 * assign4090_e5946);
        let assign4090_e5951: f64 = (locals.var_fn25_calc_iq__qs2 + locals.var_fn25_calc_iq__qd2);
        let assign4090_e5954: f64 = (2.0 * locals.var_fn25_calc_iq__qsqd);
        let assign4090_e5955: f64 = (assign4090_e5951 + assign4090_e5954);
        let assign4090_e5956: f64 = (15.0 * assign4090_e5955);
        let assign4090_e5957: f64 = (assign4090_e5947 / assign4090_e5956);
        (assign4090_e5957, ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn2) + (3.0 * locals.var_fn25_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn2) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn2) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn2)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn2 + locals.var_fn25_calc_iq__qd2_dn2) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn2))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn4) + (3.0 * locals.var_fn25_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn4) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn4) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn4)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn4 + locals.var_fn25_calc_iq__qd2_dn4) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn4))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn7) + (3.0 * locals.var_fn25_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn7) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn7) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn7)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn7 + locals.var_fn25_calc_iq__qd2_dn7) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn7))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn16) + (3.0 * locals.var_fn25_calc_iq__qd3_dn16)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn16) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn16))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn16) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn16)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn16 + locals.var_fn25_calc_iq__qd2_dn16) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn16))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn17) + (3.0 * locals.var_fn25_calc_iq__qd3_dn17)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn17) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn17))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn17) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn17)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn17 + locals.var_fn25_calc_iq__qd2_dn17) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn17))))) / (assign4090_e5956 * assign4090_e5956)),)
    } else {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd1 = assign4090_e5959;
        locals.var_fn25_calc_iq__qd1_dn2 = assign4090_e5959_d_n2;
        locals.var_fn25_calc_iq__qd1_dn4 = assign4090_e5959_d_n4;
        locals.var_fn25_calc_iq__qd1_dn7 = assign4090_e5959_d_n7;
        locals.var_fn25_calc_iq__qd1_dn16 = assign4090_e5959_d_n16;
        locals.var_fn25_calc_iq__qd1_dn17 = assign4090_e5959_d_n17;
        locals.var_fn25_calc_iq__qd1_rv = 0.0;

        let (assign4100_e5965, assign4100_e5965_d_n2, assign4100_e5965_d_n4, assign4100_e5965_d_n7, assign4100_e5965_d_n16, assign4100_e5965_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4100_e5963: f64 = (locals.var_fn25_calc_iq__qinvdd - locals.var_fn25_calc_iq__qd1);
        (assign4100_e5963, (locals.var_fn25_calc_iq__qinvdd_dn2 - locals.var_fn25_calc_iq__qd1_dn2), (locals.var_fn25_calc_iq__qinvdd_dn4 - locals.var_fn25_calc_iq__qd1_dn4), (locals.var_fn25_calc_iq__qinvdd_dn7 - locals.var_fn25_calc_iq__qd1_dn7), (locals.var_fn25_calc_iq__qinvdd_dn16 - locals.var_fn25_calc_iq__qd1_dn16), (locals.var_fn25_calc_iq__qinvdd_dn17 - locals.var_fn25_calc_iq__qd1_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qs, locals.var_fn25_calc_iq__qs_dn2, locals.var_fn25_calc_iq__qs_dn4, locals.var_fn25_calc_iq__qs_dn7, locals.var_fn25_calc_iq__qs_dn16, locals.var_fn25_calc_iq__qs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs = assign4100_e5965;
        locals.var_fn25_calc_iq__qs_dn2 = assign4100_e5965_d_n2;
        locals.var_fn25_calc_iq__qs_dn4 = assign4100_e5965_d_n4;
        locals.var_fn25_calc_iq__qs_dn7 = assign4100_e5965_d_n7;
        locals.var_fn25_calc_iq__qs_dn16 = assign4100_e5965_d_n16;
        locals.var_fn25_calc_iq__qs_dn17 = assign4100_e5965_d_n17;
        locals.var_fn25_calc_iq__qs_rv = 0.0;

        let (assign4110_e5969, assign4110_e5969_d_n2, assign4110_e5969_d_n4, assign4110_e5969_d_n7, assign4110_e5969_d_n16, assign4110_e5969_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    } else {
        (locals.var_fn25_calc_iq__qd, locals.var_fn25_calc_iq__qd_dn2, locals.var_fn25_calc_iq__qd_dn4, locals.var_fn25_calc_iq__qd_dn7, locals.var_fn25_calc_iq__qd_dn16, locals.var_fn25_calc_iq__qd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd = assign4110_e5969;
        locals.var_fn25_calc_iq__qd_dn2 = assign4110_e5969_d_n2;
        locals.var_fn25_calc_iq__qd_dn4 = assign4110_e5969_d_n4;
        locals.var_fn25_calc_iq__qd_dn7 = assign4110_e5969_d_n7;
        locals.var_fn25_calc_iq__qd_dn16 = assign4110_e5969_d_n16;
        locals.var_fn25_calc_iq__qd_dn17 = assign4110_e5969_d_n17;
        locals.var_fn25_calc_iq__qd_rv = 0.0;

        let (assign4120_e5983, assign4120_e5983_d_n2, assign4120_e5983_d_n4, assign4120_e5983_d_n7, assign4120_e5983_d_n16, assign4120_e5983_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4120_e5973: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4120_e5975: f64 = (assign4120_e5973 * locals.var_fn25_calc_iq__lin);
        let assign4120_e5977: f64 = (assign4120_e5975 * locals.var_fn25_calc_iq__type);
        let assign4120_e5979: f64 = (assign4120_e5977 * locals.var_fn25_calc_iq__qs);
        let assign4120_e5981: f64 = (assign4120_e5979 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4120_e5981, ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn4) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgsout = assign4120_e5983;
        locals.var_fn25_calc_iq__qgsout_dn2 = assign4120_e5983_d_n2;
        locals.var_fn25_calc_iq__qgsout_dn4 = assign4120_e5983_d_n4;
        locals.var_fn25_calc_iq__qgsout_dn7 = assign4120_e5983_d_n7;
        locals.var_fn25_calc_iq__qgsout_dn16 = assign4120_e5983_d_n16;
        locals.var_fn25_calc_iq__qgsout_dn17 = assign4120_e5983_d_n17;
        locals.var_fn25_calc_iq__qgsout_rv = 0.0;

        let (assign4130_e5997, assign4130_e5997_d_n2, assign4130_e5997_d_n4, assign4130_e5997_d_n7, assign4130_e5997_d_n16, assign4130_e5997_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4130_e5987: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4130_e5989: f64 = (assign4130_e5987 * locals.var_fn25_calc_iq__lin);
        let assign4130_e5991: f64 = (assign4130_e5989 * locals.var_fn25_calc_iq__type);
        let assign4130_e5993: f64 = (assign4130_e5991 * locals.var_fn25_calc_iq__qd);
        let assign4130_e5995: f64 = (assign4130_e5993 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4130_e5995, ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn4) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgdout = assign4130_e5997;
        locals.var_fn25_calc_iq__qgdout_dn2 = assign4130_e5997_d_n2;
        locals.var_fn25_calc_iq__qgdout_dn4 = assign4130_e5997_d_n4;
        locals.var_fn25_calc_iq__qgdout_dn7 = assign4130_e5997_d_n7;
        locals.var_fn25_calc_iq__qgdout_dn16 = assign4130_e5997_d_n16;
        locals.var_fn25_calc_iq__qgdout_dn17 = assign4130_e5997_d_n17;
        locals.var_fn25_calc_iq__qgdout_rv = 0.0;

        let assign4140_e6000: f64 = if locals.var_fn25_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign4140_e6000;
        locals.var_guard51_rv = 0.0;

        let (assign4150_e6016, assign4150_e6016_d_n2, assign4150_e6016_d_n4, assign4150_e6016_d_n7, assign4150_e6016_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4150_e6008: f64 = (p.p51 * 0.5);
        let assign4150_e6010: f64 = (assign4150_e6008 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4150_e6011: f64 = (locals.var_fn25_calc_iq__vtof - assign4150_e6010);
        let assign4150_e6012: f64 = (locals.var_fn25_calc_iq__vcin - assign4150_e6011);
        let assign4150_e6014: f64 = (assign4150_e6012 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4150_e6014, (locals.var_fn25_calc_iq__vcin_dn2 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4150_e6008 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4150_e6012 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vcin_dn7 / locals.var_fn25_calc_iq__two_n_phit0), (locals.var_fn25_calc_iq__vcin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16,)
    }
};
        locals.var_fn25_calc_iq__etac = assign4150_e6016;
        locals.var_fn25_calc_iq__etac_dn2 = assign4150_e6016_d_n2;
        locals.var_fn25_calc_iq__etac_dn4 = assign4150_e6016_d_n4;
        locals.var_fn25_calc_iq__etac_dn7 = assign4150_e6016_d_n7;
        locals.var_fn25_calc_iq__etac_dn16 = assign4150_e6016_d_n16;
        locals.var_fn25_calc_iq__etac_rv = 0.0;

        let assign4160_e6019: f64 = if locals.var_fn25_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign4160_e6019;
        locals.var_guard52_rv = 0.0;

        let (assign4170_e6027, assign4170_e6027_d_n2, assign4170_e6027_d_n3, assign4170_e6027_d_n4, assign4170_e6027_d_n7, assign4170_e6027_d_n16, assign4170_e6027_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 != 0.0)) {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, 0.0, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4170_e6027;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4170_e6027_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4170_e6027_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4170_e6027_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4170_e6027_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4170_e6027_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4170_e6027_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign4180_e6030: f64 = (-50.0);
        let assign4180_e6031: f64 = if locals.var_fn25_calc_iq__etac < assign4180_e6030 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign4180_e6031;
        locals.var_guard53_rv = 0.0;

        let (assign4190_e6043, assign4190_e6043_d_n2, assign4190_e6043_d_n3, assign4190_e6043_d_n4, assign4190_e6043_d_n7, assign4190_e6043_d_n16, assign4190_e6043_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 != 0.0)) {
        let assign4190_e6041: f64 = (locals.var_fn25_calc_iq__etac).exp();
        (assign4190_e6041, (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn2), 0.0, (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn4), (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn7), (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4190_e6043;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4190_e6043_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4190_e6043_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4190_e6043_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4190_e6043_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4190_e6043_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4190_e6043_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4200_e6059, assign4200_e6059_d_n2, assign4200_e6059_d_n3, assign4200_e6059_d_n4, assign4200_e6059_d_n7, assign4200_e6059_d_n16, assign4200_e6059_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 == 0.0)) {
        let assign4200_e6055: f64 = (locals.var_fn25_calc_iq__etac).exp();
        let assign4200_e6056: f64 = (1.0 + assign4200_e6055);
        let assign4200_e6057: f64 = (assign4200_e6056).ln();
        (assign4200_e6057, ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn2) / assign4200_e6056), 0.0, ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn4) / assign4200_e6056), ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn7) / assign4200_e6056), ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn16) / assign4200_e6056), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4200_e6059;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4200_e6059_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4200_e6059_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4200_e6059_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4200_e6059_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4200_e6059_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4200_e6059_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4210_e6077, assign4210_e6077_d_n2, assign4210_e6077_d_n3, assign4210_e6077_d_n4, assign4210_e6077_d_n7, assign4210_e6077_d_n16, assign4210_e6077_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4210_e6065: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4210_e6067: f64 = (assign4210_e6065 * locals.var_fn25_calc_iq__type);
        let assign4210_e6069: f64 = (assign4210_e6067 * locals.var_fn25_calc_iq__cc);
        let assign4210_e6071: f64 = (assign4210_e6069 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4210_e6073: f64 = (assign4210_e6071 * locals.var_fn25_calc_iq__exparg);
        let assign4210_e6075: f64 = (assign4210_e6073 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4210_e6075, ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((((assign4210_e6067 * locals.var_fn25_calc_iq__cc_dn4) * locals.var_fn25_calc_iq__two_n_phit0) + (assign4210_e6069 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) * locals.var_fn25_calc_iq__exparg) + (assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign4210_e6077;
        locals.var_fn25_calc_iq__qcout_dn2 = assign4210_e6077_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign4210_e6077_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign4210_e6077_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign4210_e6077_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign4210_e6077_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign4210_e6077_d_n17;
        locals.var_fn25_calc_iq__qcout_rv = 0.0;

        let (assign4220_e6093, assign4220_e6093_d_n3, assign4220_e6093_d_n4, assign4220_e6093_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4220_e6085: f64 = (p.p51 * 0.5);
        let assign4220_e6087: f64 = (assign4220_e6085 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4220_e6088: f64 = (locals.var_fn25_calc_iq__vtof - assign4220_e6087);
        let assign4220_e6089: f64 = (locals.var_fn25_calc_iq__vbin - assign4220_e6088);
        let assign4220_e6091: f64 = (assign4220_e6089 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4220_e6091, (locals.var_fn25_calc_iq__vbin_dn3 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4220_e6085 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4220_e6089 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vbin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etab, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, locals.var_fn25_calc_iq__etab_dn16,)
    }
};
        locals.var_fn25_calc_iq__etab = assign4220_e6093;
        locals.var_fn25_calc_iq__etab_dn3 = assign4220_e6093_d_n3;
        locals.var_fn25_calc_iq__etab_dn4 = assign4220_e6093_d_n4;
        locals.var_fn25_calc_iq__etab_dn16 = assign4220_e6093_d_n16;
        locals.var_fn25_calc_iq__etab_rv = 0.0;

        let assign4230_e6096: f64 = if locals.var_fn25_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign4230_e6096;
        locals.var_guard54_rv = 0.0;

        let (assign4240_e6104, assign4240_e6104_d_n2, assign4240_e6104_d_n3, assign4240_e6104_d_n4, assign4240_e6104_d_n7, assign4240_e6104_d_n16, assign4240_e6104_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (locals.var_fn25_calc_iq__etab, 0.0, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, 0.0, locals.var_fn25_calc_iq__etab_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4240_e6104;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4240_e6104_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4240_e6104_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4240_e6104_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4240_e6104_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4240_e6104_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4240_e6104_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign4250_e6107: f64 = (-50.0);
        let assign4250_e6108: f64 = if locals.var_fn25_calc_iq__etab < assign4250_e6107 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign4250_e6108;
        locals.var_guard55_rv = 0.0;

        let (assign4260_e6120, assign4260_e6120_d_n2, assign4260_e6120_d_n3, assign4260_e6120_d_n4, assign4260_e6120_d_n7, assign4260_e6120_d_n16, assign4260_e6120_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign4260_e6118: f64 = (locals.var_fn25_calc_iq__etab).exp();
        (assign4260_e6118, 0.0, (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn3), (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn4), 0.0, (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4260_e6120;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4260_e6120_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4260_e6120_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4260_e6120_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4260_e6120_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4260_e6120_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4260_e6120_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4270_e6136, assign4270_e6136_d_n2, assign4270_e6136_d_n3, assign4270_e6136_d_n4, assign4270_e6136_d_n7, assign4270_e6136_d_n16, assign4270_e6136_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign4270_e6132: f64 = (locals.var_fn25_calc_iq__etab).exp();
        let assign4270_e6133: f64 = (1.0 + assign4270_e6132);
        let assign4270_e6134: f64 = (assign4270_e6133).ln();
        (assign4270_e6134, 0.0, ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn3) / assign4270_e6133), ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn4) / assign4270_e6133), 0.0, ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn16) / assign4270_e6133), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4270_e6136;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4270_e6136_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4270_e6136_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4270_e6136_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4270_e6136_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4270_e6136_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4270_e6136_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4280_e6154, assign4280_e6154_d_n2, assign4280_e6154_d_n3, assign4280_e6154_d_n4, assign4280_e6154_d_n7, assign4280_e6154_d_n16, assign4280_e6154_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4280_e6142: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4280_e6144: f64 = (assign4280_e6142 * locals.var_fn25_calc_iq__type);
        let assign4280_e6146: f64 = (assign4280_e6144 * locals.var_fn25_calc_iq__cb);
        let assign4280_e6148: f64 = (assign4280_e6146 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4280_e6150: f64 = (assign4280_e6148 * locals.var_fn25_calc_iq__exparg);
        let assign4280_e6152: f64 = (assign4280_e6150 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4280_e6152, ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((((assign4280_e6144 * locals.var_fn25_calc_iq__cb_dn4) * locals.var_fn25_calc_iq__two_n_phit0) + (assign4280_e6146 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) * locals.var_fn25_calc_iq__exparg) + (assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign4280_e6154;
        locals.var_fn25_calc_iq__qbout_dn2 = assign4280_e6154_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign4280_e6154_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign4280_e6154_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign4280_e6154_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign4280_e6154_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign4280_e6154_d_n17;
        locals.var_fn25_calc_iq__qbout_rv = 0.0;

        let (assign4290_e6161, assign4290_e6161_d_n2, assign4290_e6161_d_n3, assign4290_e6161_d_n4, assign4290_e6161_d_n7, assign4290_e6161_d_n16, assign4290_e6161_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign4290_e6161;
        locals.var_fn25_calc_iq__qcout_dn2 = assign4290_e6161_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign4290_e6161_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign4290_e6161_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign4290_e6161_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign4290_e6161_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign4290_e6161_d_n17;
        locals.var_fn25_calc_iq__qcout_rv = 0.0;

        let (assign4300_e6168, assign4300_e6168_d_n2, assign4300_e6168_d_n3, assign4300_e6168_d_n4, assign4300_e6168_d_n7, assign4300_e6168_d_n16, assign4300_e6168_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign4300_e6168;
        locals.var_fn25_calc_iq__qbout_dn2 = assign4300_e6168_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign4300_e6168_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign4300_e6168_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign4300_e6168_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign4300_e6168_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign4300_e6168_d_n17;
        locals.var_fn25_calc_iq__qbout_rv = 0.0;

        let assign4310_e6171: f64 = if locals.var_fn25_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign4310_e6171;
        locals.var_guard56_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4320_e6187, assign4320_e6187_d_n2, assign4320_e6187_d_n4, assign4320_e6187_d_n7, assign4320_e6187_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign4320_e6179: f64 = (p.p51 * 0.5);
        let assign4320_e6181: f64 = (assign4320_e6179 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4320_e6182: f64 = (locals.var_fn25_calc_iq__vtof - assign4320_e6181);
        let assign4320_e6183: f64 = (locals.var_fn25_calc_iq__vgsin - assign4320_e6182);
        let assign4320_e6185: f64 = (assign4320_e6183 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4320_e6185, (locals.var_fn25_calc_iq__vgsin_dn2 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4320_e6179 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4320_e6183 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vgsin_dn7 / locals.var_fn25_calc_iq__two_n_phit0), (locals.var_fn25_calc_iq__vgsin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16,)
    }
};
        locals.var_fn25_calc_iq__etags = assign4320_e6187;
        locals.var_fn25_calc_iq__etags_dn2 = assign4320_e6187_d_n2;
        locals.var_fn25_calc_iq__etags_dn4 = assign4320_e6187_d_n4;
        locals.var_fn25_calc_iq__etags_dn7 = assign4320_e6187_d_n7;
        locals.var_fn25_calc_iq__etags_dn16 = assign4320_e6187_d_n16;
        locals.var_fn25_calc_iq__etags_rv = 0.0;

        let assign4330_e6190: f64 = if locals.var_fn25_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign4330_e6190;
        locals.var_guard57_rv = 0.0;

        let (assign4340_e6198, assign4340_e6198_d_n2, assign4340_e6198_d_n3, assign4340_e6198_d_n4, assign4340_e6198_d_n7, assign4340_e6198_d_n16, assign4340_e6198_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 != 0.0)) {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, 0.0, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4340_e6198;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4340_e6198_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4340_e6198_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4340_e6198_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4340_e6198_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4340_e6198_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4340_e6198_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let assign4350_e6201: f64 = (-50.0);
        let assign4350_e6202: f64 = if locals.var_fn25_calc_iq__etags < assign4350_e6201 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign4350_e6202;
        locals.var_guard58_rv = 0.0;

        let (assign4360_e6214, assign4360_e6214_d_n2, assign4360_e6214_d_n3, assign4360_e6214_d_n4, assign4360_e6214_d_n7, assign4360_e6214_d_n16, assign4360_e6214_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 == 0.0)) && (locals.var_guard58 != 0.0)) {
        let assign4360_e6212: f64 = (locals.var_fn25_calc_iq__etags).exp();
        (assign4360_e6212, (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn2), 0.0, (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn4), (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn7), (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4360_e6214;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4360_e6214_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4360_e6214_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4360_e6214_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4360_e6214_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4360_e6214_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4360_e6214_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4370_e6230, assign4370_e6230_d_n2, assign4370_e6230_d_n3, assign4370_e6230_d_n4, assign4370_e6230_d_n7, assign4370_e6230_d_n16, assign4370_e6230_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 == 0.0)) && (locals.var_guard58 == 0.0)) {
        let assign4370_e6226: f64 = (locals.var_fn25_calc_iq__etags).exp();
        let assign4370_e6227: f64 = (1.0 + assign4370_e6226);
        let assign4370_e6228: f64 = (assign4370_e6227).ln();
        (assign4370_e6228, ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn2) / assign4370_e6227), 0.0, ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn4) / assign4370_e6227), ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn7) / assign4370_e6227), ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn16) / assign4370_e6227), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4370_e6230;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4370_e6230_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4370_e6230_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4370_e6230_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4370_e6230_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4370_e6230_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4370_e6230_d_n17;
        locals.var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign4380_e6248, assign4380_e6248_d_n2, assign4380_e6248_d_n3, assign4380_e6248_d_n4, assign4380_e6248_d_n7, assign4380_e6248_d_n16, assign4380_e6248_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign4380_e6236: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4380_e6238: f64 = (assign4380_e6236 * locals.var_fn25_calc_iq__type);
        let assign4380_e6240: f64 = (assign4380_e6238 * locals.var_fn25_calc_iq__cs);
        let assign4380_e6242: f64 = (assign4380_e6240 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4380_e6244: f64 = (assign4380_e6242 * locals.var_fn25_calc_iq__exparg);
        let assign4380_e6246: f64 = (assign4380_e6244 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4380_e6246, ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((assign4380_e6240 * locals.var_fn25_calc_iq__two_n_phit0_dn4) * locals.var_fn25_calc_iq__exparg) + (assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign4380_e6248;
        locals.var_fn25_calc_iq__qsout_dn2 = assign4380_e6248_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign4380_e6248_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign4380_e6248_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign4380_e6248_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign4380_e6248_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign4380_e6248_d_n17;
        locals.var_fn25_calc_iq__qsout_rv = 0.0;

        let (assign4390_e6255, assign4390_e6255_d_n2, assign4390_e6255_d_n3, assign4390_e6255_d_n4, assign4390_e6255_d_n7, assign4390_e6255_d_n16, assign4390_e6255_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign4390_e6255;
        locals.var_fn25_calc_iq__qsout_dn2 = assign4390_e6255_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign4390_e6255_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign4390_e6255_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign4390_e6255_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign4390_e6255_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign4390_e6255_d_n17;
        locals.var_fn25_calc_iq__qsout_rv = 0.0;

        let (assign4420_e6267, assign4420_e6267_d_n2, assign4420_e6267_d_n4, assign4420_e6267_d_n7, assign4420_e6267_d_n16, assign4420_e6267_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    } else {
        (locals.var_qgsfp4, locals.var_qgsfp4_dn2, locals.var_qgsfp4_dn4, locals.var_qgsfp4_dn7, locals.var_qgsfp4_dn16, locals.var_qgsfp4_dn17,)
    }
};
        locals.var_qgsfp4 = assign4420_e6267;
        locals.var_qgsfp4_dn2 = assign4420_e6267_d_n2;
        locals.var_qgsfp4_dn4 = assign4420_e6267_d_n4;
        locals.var_qgsfp4_dn7 = assign4420_e6267_d_n7;
        locals.var_qgsfp4_dn16 = assign4420_e6267_d_n16;
        locals.var_qgsfp4_dn17 = assign4420_e6267_d_n17;
        locals.var_qgsfp4_rv = 0.0;

        let (assign4430_e6271, assign4430_e6271_d_n2, assign4430_e6271_d_n4, assign4430_e6271_d_n7, assign4430_e6271_d_n16, assign4430_e6271_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    } else {
        (locals.var_qgdfp4, locals.var_qgdfp4_dn2, locals.var_qgdfp4_dn4, locals.var_qgdfp4_dn7, locals.var_qgdfp4_dn16, locals.var_qgdfp4_dn17,)
    }
};
        locals.var_qgdfp4 = assign4430_e6271;
        locals.var_qgdfp4_dn2 = assign4430_e6271_d_n2;
        locals.var_qgdfp4_dn4 = assign4430_e6271_d_n4;
        locals.var_qgdfp4_dn7 = assign4430_e6271_d_n7;
        locals.var_qgdfp4_dn16 = assign4430_e6271_d_n16;
        locals.var_qgdfp4_dn17 = assign4430_e6271_d_n17;
        locals.var_qgdfp4_rv = 0.0;

        let (assign4440_e6275, assign4440_e6275_d_n2, assign4440_e6275_d_n3, assign4440_e6275_d_n4, assign4440_e6275_d_n7, assign4440_e6275_d_n16, assign4440_e6275_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    } else {
        (locals.var_qcfp4, locals.var_qcfp4_dn2, locals.var_qcfp4_dn3, locals.var_qcfp4_dn4, locals.var_qcfp4_dn7, locals.var_qcfp4_dn16, locals.var_qcfp4_dn17,)
    }
};
        locals.var_qcfp4 = assign4440_e6275;
        locals.var_qcfp4_dn2 = assign4440_e6275_d_n2;
        locals.var_qcfp4_dn3 = assign4440_e6275_d_n3;
        locals.var_qcfp4_dn4 = assign4440_e6275_d_n4;
        locals.var_qcfp4_dn7 = assign4440_e6275_d_n7;
        locals.var_qcfp4_dn16 = assign4440_e6275_d_n16;
        locals.var_qcfp4_dn17 = assign4440_e6275_d_n17;
        locals.var_qcfp4_rv = 0.0;

        let (assign4450_e6279, assign4450_e6279_d_n2, assign4450_e6279_d_n3, assign4450_e6279_d_n4, assign4450_e6279_d_n7, assign4450_e6279_d_n16, assign4450_e6279_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    } else {
        (locals.var_qbfp4, locals.var_qbfp4_dn2, locals.var_qbfp4_dn3, locals.var_qbfp4_dn4, locals.var_qbfp4_dn7, locals.var_qbfp4_dn16, locals.var_qbfp4_dn17,)
    }
};
        locals.var_qbfp4 = assign4450_e6279;
        locals.var_qbfp4_dn2 = assign4450_e6279_d_n2;
        locals.var_qbfp4_dn3 = assign4450_e6279_d_n3;
        locals.var_qbfp4_dn4 = assign4450_e6279_d_n4;
        locals.var_qbfp4_dn7 = assign4450_e6279_d_n7;
        locals.var_qbfp4_dn16 = assign4450_e6279_d_n16;
        locals.var_qbfp4_dn17 = assign4450_e6279_d_n17;
        locals.var_qbfp4_rv = 0.0;

        let (assign4460_e6283, assign4460_e6283_d_n2, assign4460_e6283_d_n3, assign4460_e6283_d_n4, assign4460_e6283_d_n7, assign4460_e6283_d_n16, assign4460_e6283_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    } else {
        (locals.var_qsfp4, locals.var_qsfp4_dn2, locals.var_qsfp4_dn3, locals.var_qsfp4_dn4, locals.var_qsfp4_dn7, locals.var_qsfp4_dn16, locals.var_qsfp4_dn17,)
    }
};
        locals.var_qsfp4 = assign4460_e6283;
        locals.var_qsfp4_dn2 = assign4460_e6283_d_n2;
        locals.var_qsfp4_dn3 = assign4460_e6283_d_n3;
        locals.var_qsfp4_dn4 = assign4460_e6283_d_n4;
        locals.var_qsfp4_dn7 = assign4460_e6283_d_n7;
        locals.var_qsfp4_dn16 = assign4460_e6283_d_n16;
        locals.var_qsfp4_dn17 = assign4460_e6283_d_n17;
        locals.var_qsfp4_rv = 0.0;

        let assign4500_e6298: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign4500_e6298;
        locals.var_guard59_rv = 0.0;

        locals.var_qgsfp3 = 0.0;
        locals.var_qgsfp3_dn2 = 0.0;
        locals.var_qgsfp3_dn4 = 0.0;
        locals.var_qgsfp3_dn7 = 0.0;
        locals.var_qgsfp3_dn15 = 0.0;
        locals.var_qgsfp3_dn16 = 0.0;
        locals.var_qgsfp3_rv = 0.0;

        locals.var_qgdfp3 = 0.0;
        locals.var_qgdfp3_dn2 = 0.0;
        locals.var_qgdfp3_dn4 = 0.0;
        locals.var_qgdfp3_dn7 = 0.0;
        locals.var_qgdfp3_dn15 = 0.0;
        locals.var_qgdfp3_dn16 = 0.0;
        locals.var_qgdfp3_rv = 0.0;

        locals.var_qcfp3 = 0.0;
        locals.var_qcfp3_dn2 = 0.0;
        locals.var_qcfp3_dn3 = 0.0;
        locals.var_qcfp3_dn4 = 0.0;
        locals.var_qcfp3_dn7 = 0.0;
        locals.var_qcfp3_dn15 = 0.0;
        locals.var_qcfp3_dn16 = 0.0;
        locals.var_qcfp3_rv = 0.0;

        locals.var_qbfp3 = 0.0;
        locals.var_qbfp3_dn2 = 0.0;
        locals.var_qbfp3_dn3 = 0.0;
        locals.var_qbfp3_dn4 = 0.0;
        locals.var_qbfp3_dn7 = 0.0;
        locals.var_qbfp3_dn15 = 0.0;
        locals.var_qbfp3_dn16 = 0.0;
        locals.var_qbfp3_rv = 0.0;

        locals.var_qsfp3 = 0.0;
        locals.var_qsfp3_dn2 = 0.0;
        locals.var_qsfp3_dn3 = 0.0;
        locals.var_qsfp3_dn4 = 0.0;
        locals.var_qsfp3_dn7 = 0.0;
        locals.var_qsfp3_dn15 = 0.0;
        locals.var_qsfp3_dn16 = 0.0;
        locals.var_qsfp3_rv = 0.0;

        let assign4590_e6309: f64 = if p.p211 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign4590_e6309;
        locals.var_guard60_rv = 0.0;

        let (assign4620_e6321, assign4620_e6321_d_n2, assign4620_e6321_d_n4, assign4620_e6321_d_n7, assign4620_e6321_d_n15, assign4620_e6321_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgsout = assign4620_e6321;
        locals.var_fn61_calc_iq__qgsout_dn2 = assign4620_e6321_d_n2;
        locals.var_fn61_calc_iq__qgsout_dn4 = assign4620_e6321_d_n4;
        locals.var_fn61_calc_iq__qgsout_dn7 = assign4620_e6321_d_n7;
        locals.var_fn61_calc_iq__qgsout_dn15 = assign4620_e6321_d_n15;
        locals.var_fn61_calc_iq__qgsout_dn16 = assign4620_e6321_d_n16;
        locals.var_fn61_calc_iq__qgsout_rv = 0.0;

        let (assign4630_e6325, assign4630_e6325_d_n2, assign4630_e6325_d_n4, assign4630_e6325_d_n7, assign4630_e6325_d_n15, assign4630_e6325_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgdout = assign4630_e6325;
        locals.var_fn61_calc_iq__qgdout_dn2 = assign4630_e6325_d_n2;
        locals.var_fn61_calc_iq__qgdout_dn4 = assign4630_e6325_d_n4;
        locals.var_fn61_calc_iq__qgdout_dn7 = assign4630_e6325_d_n7;
        locals.var_fn61_calc_iq__qgdout_dn15 = assign4630_e6325_d_n15;
        locals.var_fn61_calc_iq__qgdout_dn16 = assign4630_e6325_d_n16;
        locals.var_fn61_calc_iq__qgdout_rv = 0.0;

        let (assign4640_e6329, assign4640_e6329_d_n2, assign4640_e6329_d_n3, assign4640_e6329_d_n4, assign4640_e6329_d_n7, assign4640_e6329_d_n15, assign4640_e6329_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign4640_e6329;
        locals.var_fn61_calc_iq__qcout_dn2 = assign4640_e6329_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign4640_e6329_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign4640_e6329_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign4640_e6329_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign4640_e6329_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign4640_e6329_d_n16;
        locals.var_fn61_calc_iq__qcout_rv = 0.0;

        let (assign4650_e6333, assign4650_e6333_d_n2, assign4650_e6333_d_n3, assign4650_e6333_d_n4, assign4650_e6333_d_n7, assign4650_e6333_d_n15, assign4650_e6333_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign4650_e6333;
        locals.var_fn61_calc_iq__qbout_dn2 = assign4650_e6333_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign4650_e6333_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign4650_e6333_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign4650_e6333_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign4650_e6333_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign4650_e6333_d_n16;
        locals.var_fn61_calc_iq__qbout_rv = 0.0;

        let (assign4660_e6337, assign4660_e6337_d_n2, assign4660_e6337_d_n3, assign4660_e6337_d_n4, assign4660_e6337_d_n7, assign4660_e6337_d_n15, assign4660_e6337_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign4660_e6337;
        locals.var_fn61_calc_iq__qsout_dn2 = assign4660_e6337_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign4660_e6337_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign4660_e6337_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign4660_e6337_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign4660_e6337_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign4660_e6337_d_n16;
        locals.var_fn61_calc_iq__qsout_rv = 0.0;

        let (assign4670_e6341, assign4670_e6341_d_n4, assign4670_e6341_d_n15, assign4670_e6341_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vtdibl, locals.var_fn61_calc_iq__vtdibl_dn4, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vtdibl = assign4670_e6341;
        locals.var_fn61_calc_iq__vtdibl_dn4 = assign4670_e6341_d_n4;
        locals.var_fn61_calc_iq__vtdibl_dn15 = assign4670_e6341_d_n15;
        locals.var_fn61_calc_iq__vtdibl_dn16 = assign4670_e6341_d_n16;
        locals.var_fn61_calc_iq__vtdibl_rv = 0.0;

        let (assign4680_e6345, assign4680_e6345_d_n2, assign4680_e6345_d_n3, assign4680_e6345_d_n4, assign4680_e6345_d_n7, assign4680_e6345_d_n15, assign4680_e6345_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat1, locals.var_fn61_calc_iq__vdsat1_dn2, locals.var_fn61_calc_iq__vdsat1_dn3, locals.var_fn61_calc_iq__vdsat1_dn4, locals.var_fn61_calc_iq__vdsat1_dn7, locals.var_fn61_calc_iq__vdsat1_dn15, locals.var_fn61_calc_iq__vdsat1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat1 = assign4680_e6345;
        locals.var_fn61_calc_iq__vdsat1_dn2 = assign4680_e6345_d_n2;
        locals.var_fn61_calc_iq__vdsat1_dn3 = assign4680_e6345_d_n3;
        locals.var_fn61_calc_iq__vdsat1_dn4 = assign4680_e6345_d_n4;
        locals.var_fn61_calc_iq__vdsat1_dn7 = assign4680_e6345_d_n7;
        locals.var_fn61_calc_iq__vdsat1_dn15 = assign4680_e6345_d_n15;
        locals.var_fn61_calc_iq__vdsat1_dn16 = assign4680_e6345_d_n16;
        locals.var_fn61_calc_iq__vdsat1_rv = 0.0;

        let (assign4690_e6349, assign4690_e6349_d_n2, assign4690_e6349_d_n7, assign4690_e6349_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vgsin, locals.var_fn61_calc_iq__vgsin_dn2, locals.var_fn61_calc_iq__vgsin_dn7, locals.var_fn61_calc_iq__vgsin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vgsin = assign4690_e6349;
        locals.var_fn61_calc_iq__vgsin_dn2 = assign4690_e6349_d_n2;
        locals.var_fn61_calc_iq__vgsin_dn7 = assign4690_e6349_d_n7;
        locals.var_fn61_calc_iq__vgsin_dn15 = assign4690_e6349_d_n15;
        locals.var_fn61_calc_iq__vgsin_rv = 0.0;

        let (assign4700_e6353, assign4700_e6353_d_n15, assign4700_e6353_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vdsfp3, locals.var_vdsfp3_dn15, locals.var_vdsfp3_dn16,)
    } else {
        (locals.var_fn61_calc_iq__vdsin, locals.var_fn61_calc_iq__vdsin_dn15, locals.var_fn61_calc_iq__vdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsin = assign4700_e6353;
        locals.var_fn61_calc_iq__vdsin_dn15 = assign4700_e6353_d_n15;
        locals.var_fn61_calc_iq__vdsin_dn16 = assign4700_e6353_d_n16;
        locals.var_fn61_calc_iq__vdsin_rv = 0.0;

        let (assign4710_e6357,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p217,)
    } else {
        (locals.var_fn61_calc_iq__qcbflag,)
    }
};
        locals.var_fn61_calc_iq__qcbflag = assign4710_e6357;
        locals.var_fn61_calc_iq__qcbflag_rv = 0.0;

        let (assign4720_e6361, assign4720_e6361_d_n2, assign4720_e6361_d_n7, assign4720_e6361_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vcin, locals.var_fn61_calc_iq__vcin_dn2, locals.var_fn61_calc_iq__vcin_dn7, locals.var_fn61_calc_iq__vcin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vcin = assign4720_e6361;
        locals.var_fn61_calc_iq__vcin_dn2 = assign4720_e6361_d_n2;
        locals.var_fn61_calc_iq__vcin_dn7 = assign4720_e6361_d_n7;
        locals.var_fn61_calc_iq__vcin_dn15 = assign4720_e6361_d_n15;
        locals.var_fn61_calc_iq__vcin_rv = 0.0;

        let (assign4730_e6365, assign4730_e6365_d_n3, assign4730_e6365_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vbfp3, locals.var_vbfp3_dn3, locals.var_vbfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vbin, locals.var_fn61_calc_iq__vbin_dn3, locals.var_fn61_calc_iq__vbin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vbin = assign4730_e6365;
        locals.var_fn61_calc_iq__vbin_dn3 = assign4730_e6365_d_n3;
        locals.var_fn61_calc_iq__vbin_dn15 = assign4730_e6365_d_n15;
        locals.var_fn61_calc_iq__vbin_rv = 0.0;

        let (assign4740_e6369,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p215,)
    } else {
        (locals.var_fn61_calc_iq__qgsflag,)
    }
};
        locals.var_fn61_calc_iq__qgsflag = assign4740_e6369;
        locals.var_fn61_calc_iq__qgsflag_rv = 0.0;

        let (assign4750_e6373, assign4750_e6373_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn61_calc_iq__tambin, locals.var_fn61_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tambin = assign4750_e6373;
        locals.var_fn61_calc_iq__tambin_dn4 = assign4750_e6373_d_n4;
        locals.var_fn61_calc_iq__tambin_rv = 0.0;

        let (assign4760_e6377,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn61_calc_iq__tnomin,)
    }
};
        locals.var_fn61_calc_iq__tnomin = assign4760_e6377;
        locals.var_fn61_calc_iq__tnomin_rv = 0.0;

        let (assign4770_e6381, assign4770_e6381_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn61_calc_iq__phitin, locals.var_fn61_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn61_calc_iq__phitin = assign4770_e6381;
        locals.var_fn61_calc_iq__phitin_dn4 = assign4770_e6381_d_n4;
        locals.var_fn61_calc_iq__phitin_rv = 0.0;

        let (assign4780_e6385,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn61_calc_iq__w,)
    }
};
        locals.var_fn61_calc_iq__w = assign4780_e6385;
        locals.var_fn61_calc_iq__w_rv = 0.0;

        let (assign4790_e6389,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p211,)
    } else {
        (locals.var_fn61_calc_iq__lin,)
    }
};
        locals.var_fn61_calc_iq__lin = assign4790_e6389;
        locals.var_fn61_calc_iq__lin_rv = 0.0;

        let (assign4800_e6393, assign4800_e6393_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_cgfp3t, locals.var_cgfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cgin, locals.var_fn61_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn61_calc_iq__cgin = assign4800_e6393;
        locals.var_fn61_calc_iq__cgin_dn4 = assign4800_e6393_d_n4;
        locals.var_fn61_calc_iq__cgin_rv = 0.0;

        let (assign4810_e6397,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p216,)
    } else {
        (locals.var_fn61_calc_iq__cs,)
    }
};
        locals.var_fn61_calc_iq__cs = assign4810_e6397;
        locals.var_fn61_calc_iq__cs_rv = 0.0;

        let (assign4820_e6401, assign4820_e6401_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_ccfp3t, locals.var_ccfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cc, locals.var_fn61_calc_iq__cc_dn4,)
    }
};
        locals.var_fn61_calc_iq__cc = assign4820_e6401;
        locals.var_fn61_calc_iq__cc_dn4 = assign4820_e6401_d_n4;
        locals.var_fn61_calc_iq__cc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4830_e6405, assign4830_e6405_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_cbfp3t, locals.var_cbfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cb, locals.var_fn61_calc_iq__cb_dn4,)
    }
};
        locals.var_fn61_calc_iq__cb = assign4830_e6405;
        locals.var_fn61_calc_iq__cb_dn4 = assign4830_e6405_d_n4;
        locals.var_fn61_calc_iq__cb_rv = 0.0;

        let (assign4840_e6409,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p212,)
    } else {
        (locals.var_fn61_calc_iq__vto,)
    }
};
        locals.var_fn61_calc_iq__vto = assign4840_e6409;
        locals.var_fn61_calc_iq__vto_rv = 0.0;

        let (assign4850_e6413,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_fn61_calc_iq__ss,)
    }
};
        locals.var_fn61_calc_iq__ss = assign4850_e6413;
        locals.var_fn61_calc_iq__ss_rv = 0.0;

        let (assign4860_e6417,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p225,)
    } else {
        (locals.var_fn61_calc_iq__delta1,)
    }
};
        locals.var_fn61_calc_iq__delta1 = assign4860_e6417;
        locals.var_fn61_calc_iq__delta1_rv = 0.0;

        let (assign4870_e6421,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn61_calc_iq__delta2,)
    }
};
        locals.var_fn61_calc_iq__delta2 = assign4870_e6421;
        locals.var_fn61_calc_iq__delta2_rv = 0.0;

        let (assign4880_e6425,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p227,)
    } else {
        (locals.var_fn61_calc_iq__nd,)
    }
};
        locals.var_fn61_calc_iq__nd = assign4880_e6425;
        locals.var_fn61_calc_iq__nd_rv = 0.0;

        let (assign4890_e6429,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p231,)
    } else {
        (locals.var_fn61_calc_iq__alpha,)
    }
};
        locals.var_fn61_calc_iq__alpha = assign4890_e6429;
        locals.var_fn61_calc_iq__alpha_rv = 0.0;

        let (assign4900_e6433,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p222,)
    } else {
        (locals.var_fn61_calc_iq__vel0,)
    }
};
        locals.var_fn61_calc_iq__vel0 = assign4900_e6433;
        locals.var_fn61_calc_iq__vel0_rv = 0.0;

        let (assign4910_e6437,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p223,)
    } else {
        (locals.var_fn61_calc_iq__mu0,)
    }
};
        locals.var_fn61_calc_iq__mu0 = assign4910_e6437;
        locals.var_fn61_calc_iq__mu0_rv = 0.0;

        let (assign4920_e6441,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p224,)
    } else {
        (locals.var_fn61_calc_iq__beta,)
    }
};
        locals.var_fn61_calc_iq__beta = assign4920_e6441;
        locals.var_fn61_calc_iq__beta_rv = 0.0;

        let (assign4930_e6445,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p230,)
    } else {
        (locals.var_fn61_calc_iq__mtheta,)
    }
};
        locals.var_fn61_calc_iq__mtheta = assign4930_e6445;
        locals.var_fn61_calc_iq__mtheta_rv = 0.0;

        let (assign4940_e6449,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p229,)
    } else {
        (locals.var_fn61_calc_iq__vtheta,)
    }
};
        locals.var_fn61_calc_iq__vtheta = assign4940_e6449;
        locals.var_fn61_calc_iq__vtheta_rv = 0.0;

        let (assign4950_e6453,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p228,)
    } else {
        (locals.var_fn61_calc_iq__vtzeta,)
    }
};
        locals.var_fn61_calc_iq__vtzeta = assign4950_e6453;
        locals.var_fn61_calc_iq__vtzeta_rv = 0.0;

        let (assign4960_e6457,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn61_calc_iq__dibsat,)
    }
};
        locals.var_fn61_calc_iq__dibsat = assign4960_e6457;
        locals.var_fn61_calc_iq__dibsat_rv = 0.0;

        let (assign4970_e6461,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn61_calc_iq__epsilon,)
    }
};
        locals.var_fn61_calc_iq__epsilon = assign4970_e6461;
        locals.var_fn61_calc_iq__epsilon_rv = 0.0;

        let (assign4980_e6465,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn61_calc_iq__vzeta,)
    }
};
        locals.var_fn61_calc_iq__vzeta = assign4980_e6465;
        locals.var_fn61_calc_iq__vzeta_rv = 0.0;

        let (assign4990_e6469,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn61_calc_iq__lambda,)
    }
};
        locals.var_fn61_calc_iq__lambda = assign4990_e6469;
        locals.var_fn61_calc_iq__lambda_rv = 0.0;

        let (assign5000_e6473,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn61_calc_iq__ngf,)
    }
};
        locals.var_fn61_calc_iq__ngf = assign5000_e6473;
        locals.var_fn61_calc_iq__ngf_rv = 0.0;

        let (assign5010_e6477,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn61_calc_iq__type,)
    }
};
        locals.var_fn61_calc_iq__type = assign5010_e6477;
        locals.var_fn61_calc_iq__type_rv = 0.0;

        let (assign5020_e6481,) = {
    if (locals.var_guard60 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn61_calc_iq__trapfracdl,)
    }
};
        locals.var_fn61_calc_iq__trapfracdl = assign5020_e6481;
        locals.var_fn61_calc_iq__trapfracdl_rv = 0.0;

        let (assign5030_e6485, assign5030_e6485_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__alpha_phit, locals.var_fn61_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn61_calc_iq__alpha_phit = assign5030_e6485;
        locals.var_fn61_calc_iq__alpha_phit_dn4 = assign5030_e6485_d_n4;
        locals.var_fn61_calc_iq__alpha_phit_rv = 0.0;

        let (assign5040_e6489, assign5040_e6489_d_n15, assign5040_e6489_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__delta, locals.var_fn61_calc_iq__delta_dn15, locals.var_fn61_calc_iq__delta_dn16,)
    }
};
        locals.var_fn61_calc_iq__delta = assign5040_e6489;
        locals.var_fn61_calc_iq__delta_dn15 = assign5040_e6489_d_n15;
        locals.var_fn61_calc_iq__delta_dn16 = assign5040_e6489_d_n16;
        locals.var_fn61_calc_iq__delta_rv = 0.0;

        let (assign5050_e6493, assign5050_e6493_d_n4, assign5050_e6493_d_n15, assign5050_e6493_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__n, locals.var_fn61_calc_iq__n_dn4, locals.var_fn61_calc_iq__n_dn15, locals.var_fn61_calc_iq__n_dn16,)
    }
};
        locals.var_fn61_calc_iq__n = assign5050_e6493;
        locals.var_fn61_calc_iq__n_dn4 = assign5050_e6493_d_n4;
        locals.var_fn61_calc_iq__n_dn15 = assign5050_e6493_d_n15;
        locals.var_fn61_calc_iq__n_dn16 = assign5050_e6493_d_n16;
        locals.var_fn61_calc_iq__n_rv = 0.0;

        let (assign5060_e6497, assign5060_e6497_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vtof, locals.var_fn61_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn61_calc_iq__vtof = assign5060_e6497;
        locals.var_fn61_calc_iq__vtof_dn4 = assign5060_e6497_d_n4;
        locals.var_fn61_calc_iq__vtof_rv = 0.0;

        let (assign5070_e6501, assign5070_e6501_d_n15, assign5070_e6501_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5070_e6501;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5070_e6501_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5070_e6501_d_n16;
        locals.var_fn61_calc_iq__vsatdibl_rv = 0.0;

        let (assign5080_e6505, assign5080_e6505_d_n2, assign5080_e6505_d_n3, assign5080_e6505_d_n4, assign5080_e6505_d_n7, assign5080_e6505_d_n15, assign5080_e6505_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign5080_e6505;
        locals.var_fn61_calc_iq__ffs_dn2 = assign5080_e6505_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign5080_e6505_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign5080_e6505_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign5080_e6505_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign5080_e6505_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign5080_e6505_d_n16;
        locals.var_fn61_calc_iq__ffs_rv = 0.0;

        let (assign5090_e6509, assign5090_e6509_d_n4, assign5090_e6509_d_n15, assign5090_e6509_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit, locals.var_fn61_calc_iq__two_n_phit_dn4, locals.var_fn61_calc_iq__two_n_phit_dn15, locals.var_fn61_calc_iq__two_n_phit_dn16,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit = assign5090_e6509;
        locals.var_fn61_calc_iq__two_n_phit_dn4 = assign5090_e6509_d_n4;
        locals.var_fn61_calc_iq__two_n_phit_dn15 = assign5090_e6509_d_n15;
        locals.var_fn61_calc_iq__two_n_phit_dn16 = assign5090_e6509_d_n16;
        locals.var_fn61_calc_iq__two_n_phit_rv = 0.0;

        let (assign5100_e6513, assign5100_e6513_d_n4, assign5100_e6513_d_n15, assign5100_e6513_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qref, locals.var_fn61_calc_iq__qref_dn4, locals.var_fn61_calc_iq__qref_dn15, locals.var_fn61_calc_iq__qref_dn16,)
    }
};
        locals.var_fn61_calc_iq__qref = assign5100_e6513;
        locals.var_fn61_calc_iq__qref_dn4 = assign5100_e6513_d_n4;
        locals.var_fn61_calc_iq__qref_dn15 = assign5100_e6513_d_n15;
        locals.var_fn61_calc_iq__qref_dn16 = assign5100_e6513_d_n16;
        locals.var_fn61_calc_iq__qref_rv = 0.0;

        let (assign5110_e6517, assign5110_e6517_d_n2, assign5110_e6517_d_n3, assign5110_e6517_d_n4, assign5110_e6517_d_n7, assign5110_e6517_d_n15, assign5110_e6517_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etas, locals.var_fn61_calc_iq__etas_dn2, locals.var_fn61_calc_iq__etas_dn3, locals.var_fn61_calc_iq__etas_dn4, locals.var_fn61_calc_iq__etas_dn7, locals.var_fn61_calc_iq__etas_dn15, locals.var_fn61_calc_iq__etas_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas = assign5110_e6517;
        locals.var_fn61_calc_iq__etas_dn2 = assign5110_e6517_d_n2;
        locals.var_fn61_calc_iq__etas_dn3 = assign5110_e6517_d_n3;
        locals.var_fn61_calc_iq__etas_dn4 = assign5110_e6517_d_n4;
        locals.var_fn61_calc_iq__etas_dn7 = assign5110_e6517_d_n7;
        locals.var_fn61_calc_iq__etas_dn15 = assign5110_e6517_d_n15;
        locals.var_fn61_calc_iq__etas_dn16 = assign5110_e6517_d_n16;
        locals.var_fn61_calc_iq__etas_rv = 0.0;

        let (assign5120_e6521, assign5120_e6521_d_n2, assign5120_e6521_d_n3, assign5120_e6521_d_n4, assign5120_e6521_d_n7, assign5120_e6521_d_n15, assign5120_e6521_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign5120_e6521;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign5120_e6521_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign5120_e6521_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign5120_e6521_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign5120_e6521_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign5120_e6521_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign5120_e6521_d_n16;
        locals.var_fn61_calc_iq__qinvs_rv = 0.0;

        let (assign5130_e6525, assign5130_e6525_d_n2, assign5130_e6525_d_n3, assign5130_e6525_d_n4, assign5130_e6525_d_n7, assign5130_e6525_d_n15, assign5130_e6525_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__muf, locals.var_fn61_calc_iq__muf_dn2, locals.var_fn61_calc_iq__muf_dn3, locals.var_fn61_calc_iq__muf_dn4, locals.var_fn61_calc_iq__muf_dn7, locals.var_fn61_calc_iq__muf_dn15, locals.var_fn61_calc_iq__muf_dn16,)
    }
};
        locals.var_fn61_calc_iq__muf = assign5130_e6525;
        locals.var_fn61_calc_iq__muf_dn2 = assign5130_e6525_d_n2;
        locals.var_fn61_calc_iq__muf_dn3 = assign5130_e6525_d_n3;
        locals.var_fn61_calc_iq__muf_dn4 = assign5130_e6525_d_n4;
        locals.var_fn61_calc_iq__muf_dn7 = assign5130_e6525_d_n7;
        locals.var_fn61_calc_iq__muf_dn15 = assign5130_e6525_d_n15;
        locals.var_fn61_calc_iq__muf_dn16 = assign5130_e6525_d_n16;
        locals.var_fn61_calc_iq__muf_rv = 0.0;

        let (assign5140_e6529, assign5140_e6529_d_n2, assign5140_e6529_d_n3, assign5140_e6529_d_n4, assign5140_e6529_d_n7, assign5140_e6529_d_n15, assign5140_e6529_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vx, locals.var_fn61_calc_iq__vx_dn2, locals.var_fn61_calc_iq__vx_dn3, locals.var_fn61_calc_iq__vx_dn4, locals.var_fn61_calc_iq__vx_dn7, locals.var_fn61_calc_iq__vx_dn15, locals.var_fn61_calc_iq__vx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vx = assign5140_e6529;
        locals.var_fn61_calc_iq__vx_dn2 = assign5140_e6529_d_n2;
        locals.var_fn61_calc_iq__vx_dn3 = assign5140_e6529_d_n3;
        locals.var_fn61_calc_iq__vx_dn4 = assign5140_e6529_d_n4;
        locals.var_fn61_calc_iq__vx_dn7 = assign5140_e6529_d_n7;
        locals.var_fn61_calc_iq__vx_dn15 = assign5140_e6529_d_n15;
        locals.var_fn61_calc_iq__vx_dn16 = assign5140_e6529_d_n16;
        locals.var_fn61_calc_iq__vx_rv = 0.0;

        let (assign5160_e6537, assign5160_e6537_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__n0, locals.var_fn61_calc_iq__n0_dn4,)
    }
};
        locals.var_fn61_calc_iq__n0 = assign5160_e6537;
        locals.var_fn61_calc_iq__n0_dn4 = assign5160_e6537_d_n4;
        locals.var_fn61_calc_iq__n0_rv = 0.0;

        let (assign5170_e6541, assign5170_e6541_d_n2, assign5170_e6541_d_n4, assign5170_e6541_d_n7, assign5170_e6541_d_n15, assign5170_e6541_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs0, locals.var_fn61_calc_iq__ffs0_dn2, locals.var_fn61_calc_iq__ffs0_dn4, locals.var_fn61_calc_iq__ffs0_dn7, locals.var_fn61_calc_iq__ffs0_dn15, locals.var_fn61_calc_iq__ffs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs0 = assign5170_e6541;
        locals.var_fn61_calc_iq__ffs0_dn2 = assign5170_e6541_d_n2;
        locals.var_fn61_calc_iq__ffs0_dn4 = assign5170_e6541_d_n4;
        locals.var_fn61_calc_iq__ffs0_dn7 = assign5170_e6541_d_n7;
        locals.var_fn61_calc_iq__ffs0_dn15 = assign5170_e6541_d_n15;
        locals.var_fn61_calc_iq__ffs0_dn16 = assign5170_e6541_d_n16;
        locals.var_fn61_calc_iq__ffs0_rv = 0.0;

        let (assign5180_e6545, assign5180_e6545_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit0, locals.var_fn61_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit0 = assign5180_e6545;
        locals.var_fn61_calc_iq__two_n_phit0_dn4 = assign5180_e6545_d_n4;
        locals.var_fn61_calc_iq__two_n_phit0_rv = 0.0;

        let (assign5190_e6549, assign5190_e6549_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qref0, locals.var_fn61_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn61_calc_iq__qref0 = assign5190_e6549;
        locals.var_fn61_calc_iq__qref0_dn4 = assign5190_e6549_d_n4;
        locals.var_fn61_calc_iq__qref0_rv = 0.0;

        let (assign5200_e6553, assign5200_e6553_d_n2, assign5200_e6553_d_n4, assign5200_e6553_d_n7, assign5200_e6553_d_n15, assign5200_e6553_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etas0, locals.var_fn61_calc_iq__etas0_dn2, locals.var_fn61_calc_iq__etas0_dn4, locals.var_fn61_calc_iq__etas0_dn7, locals.var_fn61_calc_iq__etas0_dn15, locals.var_fn61_calc_iq__etas0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas0 = assign5200_e6553;
        locals.var_fn61_calc_iq__etas0_dn2 = assign5200_e6553_d_n2;
        locals.var_fn61_calc_iq__etas0_dn4 = assign5200_e6553_d_n4;
        locals.var_fn61_calc_iq__etas0_dn7 = assign5200_e6553_d_n7;
        locals.var_fn61_calc_iq__etas0_dn15 = assign5200_e6553_d_n15;
        locals.var_fn61_calc_iq__etas0_dn16 = assign5200_e6553_d_n16;
        locals.var_fn61_calc_iq__etas0_rv = 0.0;

        let (assign5210_e6557, assign5210_e6557_d_n2, assign5210_e6557_d_n4, assign5210_e6557_d_n7, assign5210_e6557_d_n15, assign5210_e6557_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvs0, locals.var_fn61_calc_iq__qinvs0_dn2, locals.var_fn61_calc_iq__qinvs0_dn4, locals.var_fn61_calc_iq__qinvs0_dn7, locals.var_fn61_calc_iq__qinvs0_dn15, locals.var_fn61_calc_iq__qinvs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs0 = assign5210_e6557;
        locals.var_fn61_calc_iq__qinvs0_dn2 = assign5210_e6557_d_n2;
        locals.var_fn61_calc_iq__qinvs0_dn4 = assign5210_e6557_d_n4;
        locals.var_fn61_calc_iq__qinvs0_dn7 = assign5210_e6557_d_n7;
        locals.var_fn61_calc_iq__qinvs0_dn15 = assign5210_e6557_d_n15;
        locals.var_fn61_calc_iq__qinvs0_dn16 = assign5210_e6557_d_n16;
        locals.var_fn61_calc_iq__qinvs0_rv = 0.0;

        let (assign5220_e6561, assign5220_e6561_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__muf0, locals.var_fn61_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn61_calc_iq__muf0 = assign5220_e6561;
        locals.var_fn61_calc_iq__muf0_dn4 = assign5220_e6561_d_n4;
        locals.var_fn61_calc_iq__muf0_rv = 0.0;

        let (assign5230_e6565, assign5230_e6565_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vx0, locals.var_fn61_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vx0 = assign5230_e6565;
        locals.var_fn61_calc_iq__vx0_dn4 = assign5230_e6565_d_n4;
        locals.var_fn61_calc_iq__vx0_rv = 0.0;

        let (assign5240_e6569, assign5240_e6569_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__tfacmobin, locals.var_fn61_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tfacmobin = assign5240_e6569;
        locals.var_fn61_calc_iq__tfacmobin_dn4 = assign5240_e6569_d_n4;
        locals.var_fn61_calc_iq__tfacmobin_rv = 0.0;

        let (assign5250_e6573, assign5250_e6573_d_n2, assign5250_e6573_d_n3, assign5250_e6573_d_n4, assign5250_e6573_d_n7, assign5250_e6573_d_n15, assign5250_e6573_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5250_e6573;
        locals.var_fn61_calc_iq__ff_dn2 = assign5250_e6573_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5250_e6573_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5250_e6573_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5250_e6573_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5250_e6573_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5250_e6573_d_n16;
        locals.var_fn61_calc_iq__ff_rv = 0.0;

        let (assign5260_e6577, assign5260_e6577_d_n2, assign5260_e6577_d_n3, assign5260_e6577_d_n4, assign5260_e6577_d_n7, assign5260_e6577_d_n15, assign5260_e6577_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__eta, locals.var_fn61_calc_iq__eta_dn2, locals.var_fn61_calc_iq__eta_dn3, locals.var_fn61_calc_iq__eta_dn4, locals.var_fn61_calc_iq__eta_dn7, locals.var_fn61_calc_iq__eta_dn15, locals.var_fn61_calc_iq__eta_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta = assign5260_e6577;
        locals.var_fn61_calc_iq__eta_dn2 = assign5260_e6577_d_n2;
        locals.var_fn61_calc_iq__eta_dn3 = assign5260_e6577_d_n3;
        locals.var_fn61_calc_iq__eta_dn4 = assign5260_e6577_d_n4;
        locals.var_fn61_calc_iq__eta_dn7 = assign5260_e6577_d_n7;
        locals.var_fn61_calc_iq__eta_dn15 = assign5260_e6577_d_n15;
        locals.var_fn61_calc_iq__eta_dn16 = assign5260_e6577_d_n16;
        locals.var_fn61_calc_iq__eta_rv = 0.0;

    }
}
