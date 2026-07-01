#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28850_e49761, assign28850_e49761_d_n0, assign28850_e49761_d_n2, assign28850_e49761_d_n3, assign28850_e49761_d_n4, assign28850_e49761_d_n5, assign28850_e49761_d_n6, assign28850_e49761_d_n7, assign28850_e49761_d_n8, assign28850_e49761_d_n9, assign28850_e49761_d_n10, assign28850_e49761_d_n11, assign28850_e49761_d_n13, assign28850_e49761_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28850_e49761;
        locals.var_t7_dn0 = assign28850_e49761_d_n0;
        locals.var_t7_dn2 = assign28850_e49761_d_n2;
        locals.var_t7_dn3 = assign28850_e49761_d_n3;
        locals.var_t7_dn4 = assign28850_e49761_d_n4;
        locals.var_t7_dn5 = assign28850_e49761_d_n5;
        locals.var_t7_dn6 = assign28850_e49761_d_n6;
        locals.var_t7_dn7 = assign28850_e49761_d_n7;
        locals.var_t7_dn8 = assign28850_e49761_d_n8;
        locals.var_t7_dn9 = assign28850_e49761_d_n9;
        locals.var_t7_dn10 = assign28850_e49761_d_n10;
        locals.var_t7_dn11 = assign28850_e49761_d_n11;
        locals.var_t7_dn13 = assign28850_e49761_d_n13;
        locals.var_t7_dn14 = assign28850_e49761_d_n14;

        let (assign28860_e49777, assign28860_e49777_d_n0, assign28860_e49777_d_n2, assign28860_e49777_d_n3, assign28860_e49777_d_n4, assign28860_e49777_d_n5, assign28860_e49777_d_n6, assign28860_e49777_d_n7, assign28860_e49777_d_n8, assign28860_e49777_d_n9, assign28860_e49777_d_n10, assign28860_e49777_d_n11, assign28860_e49777_d_n13, assign28860_e49777_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28860_e49769: f64 = (-locals.var_vgs_noswap);
        let assign28860_e49771: f64 = (assign28860_e49769 - locals.var_egislb_i);
        let assign28860_e49773: f64 = (assign28860_e49771 + locals.var_vfbsd_v);
        let assign28860_e49775: f64 = (assign28860_e49773 / locals.var_t0);
        (assign28860_e49775, (((locals.var_vfbsd_v_dn0 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn2 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn3 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn4 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn5 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn6) + locals.var_vfbsd_v_dn6) * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn7 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn8 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn9 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn10 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn11) + locals.var_vfbsd_v_dn11) * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn13 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn14 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28860_e49777;
        locals.var_t1_dn0 = assign28860_e49777_d_n0;
        locals.var_t1_dn2 = assign28860_e49777_d_n2;
        locals.var_t1_dn3 = assign28860_e49777_d_n3;
        locals.var_t1_dn4 = assign28860_e49777_d_n4;
        locals.var_t1_dn5 = assign28860_e49777_d_n5;
        locals.var_t1_dn6 = assign28860_e49777_d_n6;
        locals.var_t1_dn7 = assign28860_e49777_d_n7;
        locals.var_t1_dn8 = assign28860_e49777_d_n8;
        locals.var_t1_dn9 = assign28860_e49777_d_n9;
        locals.var_t1_dn10 = assign28860_e49777_d_n10;
        locals.var_t1_dn11 = assign28860_e49777_d_n11;
        locals.var_t1_dn13 = assign28860_e49777_d_n13;
        locals.var_t1_dn14 = assign28860_e49777_d_n14;

        let (assign28870_e49821, assign28870_e49821_d_n0, assign28870_e49821_d_n2, assign28870_e49821_d_n3, assign28870_e49821_d_n4, assign28870_e49821_d_n5, assign28870_e49821_d_n6, assign28870_e49821_d_n7, assign28870_e49821_d_n8, assign28870_e49821_d_n9, assign28870_e49821_d_n10, assign28870_e49821_d_n11, assign28870_e49821_d_n13, assign28870_e49821_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28870_e49786: f64 = (-10000.0);
        let assign28870_e49788: f64 = (assign28870_e49786 * 0.01);
        let (assign28870_e49819, assign28870_e49819_d_n0, assign28870_e49819_d_n2, assign28870_e49819_d_n3, assign28870_e49819_d_n4, assign28870_e49819_d_n5, assign28870_e49819_d_n6, assign28870_e49819_d_n7, assign28870_e49819_d_n8, assign28870_e49819_d_n9, assign28870_e49819_d_n10, assign28870_e49819_d_n11, assign28870_e49819_d_n13, assign28870_e49819_d_n14,) = {
            if (!(locals.var_t1 < assign28870_e49788)) {
                let assign28870_e49795: f64 = (locals.var_t1 * locals.var_t1);
                let assign28870_e49798: f64 = (4.0 * 0.01);
                let assign28870_e49800: f64 = (assign28870_e49798 * 0.01);
                let assign28870_e49801: f64 = (assign28870_e49795 + assign28870_e49800);
                let assign28870_e49802: f64 = (assign28870_e49801).sqrt();
                let assign28870_e49803: f64 = (locals.var_t1 + assign28870_e49802);
                let assign28870_e49804: f64 = (0.5 * assign28870_e49803);
                (assign28870_e49804, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign28870_e49802)))),)
            } else {
                let assign28870_e49807: f64 = (-10000.0);
                let assign28870_e49809: f64 = (assign28870_e49807 * 0.01);
                let (assign28870_e49818, assign28870_e49818_d_n0, assign28870_e49818_d_n2, assign28870_e49818_d_n3, assign28870_e49818_d_n4, assign28870_e49818_d_n5, assign28870_e49818_d_n6, assign28870_e49818_d_n7, assign28870_e49818_d_n8, assign28870_e49818_d_n9, assign28870_e49818_d_n10, assign28870_e49818_d_n11, assign28870_e49818_d_n13, assign28870_e49818_d_n14,) = {
                    if (locals.var_t1 < assign28870_e49809) {
                        let assign28870_e49812: f64 = (-0.01);
                        let assign28870_e49814: f64 = (assign28870_e49812 * 0.01);
                        let assign28870_e49816: f64 = (assign28870_e49814 / locals.var_t1);
                        (assign28870_e49816, (-((assign28870_e49814 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28870_e49818, assign28870_e49818_d_n0, assign28870_e49818_d_n2, assign28870_e49818_d_n3, assign28870_e49818_d_n4, assign28870_e49818_d_n5, assign28870_e49818_d_n6, assign28870_e49818_d_n7, assign28870_e49818_d_n8, assign28870_e49818_d_n9, assign28870_e49818_d_n10, assign28870_e49818_d_n11, assign28870_e49818_d_n13, assign28870_e49818_d_n14,)
            }
        };
        (assign28870_e49819, assign28870_e49819_d_n0, assign28870_e49819_d_n2, assign28870_e49819_d_n3, assign28870_e49819_d_n4, assign28870_e49819_d_n5, assign28870_e49819_d_n6, assign28870_e49819_d_n7, assign28870_e49819_d_n8, assign28870_e49819_d_n9, assign28870_e49819_d_n10, assign28870_e49819_d_n11, assign28870_e49819_d_n13, assign28870_e49819_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28870_e49821;
        locals.var_t1_dn0 = assign28870_e49821_d_n0;
        locals.var_t1_dn2 = assign28870_e49821_d_n2;
        locals.var_t1_dn3 = assign28870_e49821_d_n3;
        locals.var_t1_dn4 = assign28870_e49821_d_n4;
        locals.var_t1_dn5 = assign28870_e49821_d_n5;
        locals.var_t1_dn6 = assign28870_e49821_d_n6;
        locals.var_t1_dn7 = assign28870_e49821_d_n7;
        locals.var_t1_dn8 = assign28870_e49821_d_n8;
        locals.var_t1_dn9 = assign28870_e49821_d_n9;
        locals.var_t1_dn10 = assign28870_e49821_d_n10;
        locals.var_t1_dn11 = assign28870_e49821_d_n11;
        locals.var_t1_dn13 = assign28870_e49821_d_n13;
        locals.var_t1_dn14 = assign28870_e49821_d_n14;

        let (assign28880_e49834, assign28880_e49834_d_n0, assign28880_e49834_d_n2, assign28880_e49834_d_n3, assign28880_e49834_d_n4, assign28880_e49834_d_n5, assign28880_e49834_d_n6, assign28880_e49834_d_n7, assign28880_e49834_d_n8, assign28880_e49834_d_n9, assign28880_e49834_d_n10, assign28880_e49834_d_n11, assign28880_e49834_d_n13, assign28880_e49834_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28880_e49831: f64 = (locals.var_t1 + 0.001);
        let assign28880_e49832: f64 = (locals.var_bgislb_t / assign28880_e49831);
        (assign28880_e49832, (-((locals.var_bgislb_t * locals.var_t1_dn0) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn2) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn3) / (assign28880_e49831 * assign28880_e49831))), (((locals.var_bgislb_t_dn4 * assign28880_e49831) - (locals.var_bgislb_t * locals.var_t1_dn4)) / (assign28880_e49831 * assign28880_e49831)), (-((locals.var_bgislb_t * locals.var_t1_dn5) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn6) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn7) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn8) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn9) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn10) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn11) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn13) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn14) / (assign28880_e49831 * assign28880_e49831))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28880_e49834;
        locals.var_t2_dn0 = assign28880_e49834_d_n0;
        locals.var_t2_dn2 = assign28880_e49834_d_n2;
        locals.var_t2_dn3 = assign28880_e49834_d_n3;
        locals.var_t2_dn4 = assign28880_e49834_d_n4;
        locals.var_t2_dn5 = assign28880_e49834_d_n5;
        locals.var_t2_dn6 = assign28880_e49834_d_n6;
        locals.var_t2_dn7 = assign28880_e49834_d_n7;
        locals.var_t2_dn8 = assign28880_e49834_d_n8;
        locals.var_t2_dn9 = assign28880_e49834_d_n9;
        locals.var_t2_dn10 = assign28880_e49834_d_n10;
        locals.var_t2_dn11 = assign28880_e49834_d_n11;
        locals.var_t2_dn13 = assign28880_e49834_d_n13;
        locals.var_t2_dn14 = assign28880_e49834_d_n14;

        let (assign28890_e49845, assign28890_e49845_d_n0, assign28890_e49845_d_n2, assign28890_e49845_d_n3, assign28890_e49845_d_n4, assign28890_e49845_d_n5, assign28890_e49845_d_n6, assign28890_e49845_d_n7, assign28890_e49845_d_n8, assign28890_e49845_d_n9, assign28890_e49845_d_n10, assign28890_e49845_d_n11, assign28890_e49845_d_n13, assign28890_e49845_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28890_e49843: f64 = (locals.var_t1).powf(locals.var_pgislb_i);
        (assign28890_e49843, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn0)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn2)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn3)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn4)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn5)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn6)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn7)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn8)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn9)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn10)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn11)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn13)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn14)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28890_e49845;
        locals.var_t3_dn0 = assign28890_e49845_d_n0;
        locals.var_t3_dn2 = assign28890_e49845_d_n2;
        locals.var_t3_dn3 = assign28890_e49845_d_n3;
        locals.var_t3_dn4 = assign28890_e49845_d_n4;
        locals.var_t3_dn5 = assign28890_e49845_d_n5;
        locals.var_t3_dn6 = assign28890_e49845_d_n6;
        locals.var_t3_dn7 = assign28890_e49845_d_n7;
        locals.var_t3_dn8 = assign28890_e49845_d_n8;
        locals.var_t3_dn9 = assign28890_e49845_d_n9;
        locals.var_t3_dn10 = assign28890_e49845_d_n10;
        locals.var_t3_dn11 = assign28890_e49845_d_n11;
        locals.var_t3_dn13 = assign28890_e49845_d_n13;
        locals.var_t3_dn14 = assign28890_e49845_d_n14;

        let (assign28900_e49859, assign28900_e49859_d_n0, assign28900_e49859_d_n2, assign28900_e49859_d_n3, assign28900_e49859_d_n4, assign28900_e49859_d_n5, assign28900_e49859_d_n6, assign28900_e49859_d_n7, assign28900_e49859_d_n8, assign28900_e49859_d_n9, assign28900_e49859_d_n10, assign28900_e49859_d_n11, assign28900_e49859_d_n13, assign28900_e49859_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28900_e49853: f64 = (-locals.var_ves_jct);
        let assign28900_e49855: f64 = (assign28900_e49853 * locals.var_ves_jct);
        let assign28900_e49857: f64 = (assign28900_e49855 * locals.var_ves_jct);
        (assign28900_e49857, 0.0, 0.0, (((((-locals.var_ves_jct_dn3) * locals.var_ves_jct) + (assign28900_e49853 * locals.var_ves_jct_dn3)) * locals.var_ves_jct) + (assign28900_e49855 * locals.var_ves_jct_dn3)), 0.0, 0.0, (((((-locals.var_ves_jct_dn6) * locals.var_ves_jct) + (assign28900_e49853 * locals.var_ves_jct_dn6)) * locals.var_ves_jct) + (assign28900_e49855 * locals.var_ves_jct_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28900_e49859;
        locals.var_t4_dn0 = assign28900_e49859_d_n0;
        locals.var_t4_dn2 = assign28900_e49859_d_n2;
        locals.var_t4_dn3 = assign28900_e49859_d_n3;
        locals.var_t4_dn4 = assign28900_e49859_d_n4;
        locals.var_t4_dn5 = assign28900_e49859_d_n5;
        locals.var_t4_dn6 = assign28900_e49859_d_n6;
        locals.var_t4_dn7 = assign28900_e49859_d_n7;
        locals.var_t4_dn8 = assign28900_e49859_d_n8;
        locals.var_t4_dn9 = assign28900_e49859_d_n9;
        locals.var_t4_dn10 = assign28900_e49859_d_n10;
        locals.var_t4_dn11 = assign28900_e49859_d_n11;
        locals.var_t4_dn13 = assign28900_e49859_d_n13;
        locals.var_t4_dn14 = assign28900_e49859_d_n14;

        let (assign28910_e49873, assign28910_e49873_d_n0, assign28910_e49873_d_n2, assign28910_e49873_d_n3, assign28910_e49873_d_n4, assign28910_e49873_d_n5, assign28910_e49873_d_n6, assign28910_e49873_d_n7, assign28910_e49873_d_n8, assign28910_e49873_d_n9, assign28910_e49873_d_n10, assign28910_e49873_d_n11, assign28910_e49873_d_n13, assign28910_e49873_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28910_e49868: f64 = (locals.var_t4).abs();
        let assign28910_e49869: f64 = (locals.var_cgislb_i + assign28910_e49868);
        let assign28910_e49871: f64 = (assign28910_e49869 + 1e-5);
        (assign28910_e49871, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28910_e49873;
        locals.var_t4a_dn0 = assign28910_e49873_d_n0;
        locals.var_t4a_dn2 = assign28910_e49873_d_n2;
        locals.var_t4a_dn3 = assign28910_e49873_d_n3;
        locals.var_t4a_dn4 = assign28910_e49873_d_n4;
        locals.var_t4a_dn5 = assign28910_e49873_d_n5;
        locals.var_t4a_dn6 = assign28910_e49873_d_n6;
        locals.var_t4a_dn7 = assign28910_e49873_d_n7;
        locals.var_t4a_dn8 = assign28910_e49873_d_n8;
        locals.var_t4a_dn9 = assign28910_e49873_d_n9;
        locals.var_t4a_dn10 = assign28910_e49873_d_n10;
        locals.var_t4a_dn11 = assign28910_e49873_d_n11;
        locals.var_t4a_dn13 = assign28910_e49873_d_n13;
        locals.var_t4a_dn14 = assign28910_e49873_d_n14;

        let (assign28920_e49931, assign28920_e49931_d_n0, assign28920_e49931_d_n2, assign28920_e49931_d_n3, assign28920_e49931_d_n4, assign28920_e49931_d_n5, assign28920_e49931_d_n6, assign28920_e49931_d_n7, assign28920_e49931_d_n8, assign28920_e49931_d_n9, assign28920_e49931_d_n10, assign28920_e49931_d_n11, assign28920_e49931_d_n13, assign28920_e49931_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28920_e49882: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28920_e49884: f64 = (-10000.0);
        let assign28920_e49886: f64 = (assign28920_e49884 * 1e-6);
        let (assign28920_e49927, assign28920_e49927_d_n0, assign28920_e49927_d_n2, assign28920_e49927_d_n3, assign28920_e49927_d_n4, assign28920_e49927_d_n5, assign28920_e49927_d_n6, assign28920_e49927_d_n7, assign28920_e49927_d_n8, assign28920_e49927_d_n9, assign28920_e49927_d_n10, assign28920_e49927_d_n11, assign28920_e49927_d_n13, assign28920_e49927_d_n14,) = {
            if (!(assign28920_e49882 < assign28920_e49886)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28920_e49892: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49895: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49898: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49899: f64 = (assign28920_e49895 * assign28920_e49898);
                let assign28920_e49902: f64 = (4.0 * 1e-6);
                let assign28920_e49904: f64 = (assign28920_e49902 * 1e-6);
                let assign28920_e49905: f64 = (assign28920_e49899 + assign28920_e49904);
                let assign28920_e49906: f64 = (assign28920_e49905).sqrt();
                let assign28920_e49907: f64 = (assign28920_e49892 + assign28920_e49906);
                let assign28920_e49908: f64 = (0.5 * assign28920_e49907);
                (assign28920_e49908, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))),)
            } else {
                let assign28920_e49911: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28920_e49913: f64 = (-10000.0);
                let assign28920_e49915: f64 = (assign28920_e49913 * 1e-6);
                let (assign28920_e49926, assign28920_e49926_d_n0, assign28920_e49926_d_n2, assign28920_e49926_d_n3, assign28920_e49926_d_n4, assign28920_e49926_d_n5, assign28920_e49926_d_n6, assign28920_e49926_d_n7, assign28920_e49926_d_n8, assign28920_e49926_d_n9, assign28920_e49926_d_n10, assign28920_e49926_d_n11, assign28920_e49926_d_n13, assign28920_e49926_d_n14,) = {
                    if (assign28920_e49911 < assign28920_e49915) {
                        let assign28920_e49918: f64 = (-1e-6);
                        let assign28920_e49920: f64 = (assign28920_e49918 * 1e-6);
                        let assign28920_e49923: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28920_e49924: f64 = (assign28920_e49920 / assign28920_e49923);
                        (assign28920_e49924, (-((assign28920_e49920 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28920_e49926, assign28920_e49926_d_n0, assign28920_e49926_d_n2, assign28920_e49926_d_n3, assign28920_e49926_d_n4, assign28920_e49926_d_n5, assign28920_e49926_d_n6, assign28920_e49926_d_n7, assign28920_e49926_d_n8, assign28920_e49926_d_n9, assign28920_e49926_d_n10, assign28920_e49926_d_n11, assign28920_e49926_d_n13, assign28920_e49926_d_n14,)
            }
        };
        let assign28920_e49929: f64 = (assign28920_e49927 - 1e-6);
        (assign28920_e49929, assign28920_e49927_d_n0, assign28920_e49927_d_n2, assign28920_e49927_d_n3, assign28920_e49927_d_n4, assign28920_e49927_d_n5, assign28920_e49927_d_n6, assign28920_e49927_d_n7, assign28920_e49927_d_n8, assign28920_e49927_d_n9, assign28920_e49927_d_n10, assign28920_e49927_d_n11, assign28920_e49927_d_n13, assign28920_e49927_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28920_e49931;
        locals.var_t5_dn0 = assign28920_e49931_d_n0;
        locals.var_t5_dn2 = assign28920_e49931_d_n2;
        locals.var_t5_dn3 = assign28920_e49931_d_n3;
        locals.var_t5_dn4 = assign28920_e49931_d_n4;
        locals.var_t5_dn5 = assign28920_e49931_d_n5;
        locals.var_t5_dn6 = assign28920_e49931_d_n6;
        locals.var_t5_dn7 = assign28920_e49931_d_n7;
        locals.var_t5_dn8 = assign28920_e49931_d_n8;
        locals.var_t5_dn9 = assign28920_e49931_d_n9;
        locals.var_t5_dn10 = assign28920_e49931_d_n10;
        locals.var_t5_dn11 = assign28920_e49931_d_n11;
        locals.var_t5_dn13 = assign28920_e49931_d_n13;
        locals.var_t5_dn14 = assign28920_e49931_d_n14;

        let (assign28930_e49950, assign28930_e49950_d_n0, assign28930_e49950_d_n2, assign28930_e49950_d_n3, assign28930_e49950_d_n4, assign28930_e49950_d_n5, assign28930_e49950_d_n6, assign28930_e49950_d_n7, assign28930_e49950_d_n8, assign28930_e49950_d_n9, assign28930_e49950_d_n10, assign28930_e49950_d_n11, assign28930_e49950_d_n13, assign28930_e49950_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28930_e49940: f64 = (locals.var_agislb_i * locals.var_weffb);
        let assign28930_e49942: f64 = (assign28930_e49940 * locals.var_t3);
        let assign28930_e49944: f64 = (-locals.var_t2);
        let assign28930_e49945: f64 = { let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28930_e49946: f64 = (assign28930_e49942 * assign28930_e49945);
        let assign28930_e49948: f64 = (assign28930_e49946 * locals.var_t5);
        (assign28930_e49948, (((((assign28930_e49940 * locals.var_t3_dn0) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn0)), (((((assign28930_e49940 * locals.var_t3_dn2) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn2)), (((((assign28930_e49940 * locals.var_t3_dn3) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn3)), (((((assign28930_e49940 * locals.var_t3_dn4) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn4)), (((((assign28930_e49940 * locals.var_t3_dn5) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn5)), (((((assign28930_e49940 * locals.var_t3_dn6) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn6)), (((((assign28930_e49940 * locals.var_t3_dn7) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn7)), (((((assign28930_e49940 * locals.var_t3_dn8) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn8)), (((((assign28930_e49940 * locals.var_t3_dn9) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn9)), (((((assign28930_e49940 * locals.var_t3_dn10) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn10)), (((((assign28930_e49940 * locals.var_t3_dn11) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn11)), (((((assign28930_e49940 * locals.var_t3_dn13) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn13)), (((((assign28930_e49940 * locals.var_t3_dn14) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28930_e49950;
        locals.var_t7_dn0 = assign28930_e49950_d_n0;
        locals.var_t7_dn2 = assign28930_e49950_d_n2;
        locals.var_t7_dn3 = assign28930_e49950_d_n3;
        locals.var_t7_dn4 = assign28930_e49950_d_n4;
        locals.var_t7_dn5 = assign28930_e49950_d_n5;
        locals.var_t7_dn6 = assign28930_e49950_d_n6;
        locals.var_t7_dn7 = assign28930_e49950_d_n7;
        locals.var_t7_dn8 = assign28930_e49950_d_n8;
        locals.var_t7_dn9 = assign28930_e49950_d_n9;
        locals.var_t7_dn10 = assign28930_e49950_d_n10;
        locals.var_t7_dn11 = assign28930_e49950_d_n11;
        locals.var_t7_dn13 = assign28930_e49950_d_n13;
        locals.var_t7_dn14 = assign28930_e49950_d_n14;

        let assign28990_e49982: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign28990_e49982;

        let assign29000_e49985: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign29000_e49985;

        let assign29010_e49988: f64 = if locals.var_ves_jct < locals.var_vjsmrev { 1.0 } else { 0.0 };
        locals.var_guard471 = assign29010_e49988;

        let (assign29020_e49998, assign29020_e49998_d_n0, assign29020_e49998_d_n2, assign29020_e49998_d_n3, assign29020_e49998_d_n4, assign29020_e49998_d_n5, assign29020_e49998_d_n6, assign29020_e49998_d_n7, assign29020_e49998_d_n8, assign29020_e49998_d_n9, assign29020_e49998_d_n10, assign29020_e49998_d_n11, assign29020_e49998_d_n13, assign29020_e49998_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29020_e49996: f64 = (locals.var_ves_jct / locals.var_nvtms);
        (assign29020_e49996, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((locals.var_ves_jct * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29020_e49998;
        locals.var_t0_dn0 = assign29020_e49998_d_n0;
        locals.var_t0_dn2 = assign29020_e49998_d_n2;
        locals.var_t0_dn3 = assign29020_e49998_d_n3;
        locals.var_t0_dn4 = assign29020_e49998_d_n4;
        locals.var_t0_dn5 = assign29020_e49998_d_n5;
        locals.var_t0_dn6 = assign29020_e49998_d_n6;
        locals.var_t0_dn7 = assign29020_e49998_d_n7;
        locals.var_t0_dn8 = assign29020_e49998_d_n8;
        locals.var_t0_dn9 = assign29020_e49998_d_n9;
        locals.var_t0_dn10 = assign29020_e49998_d_n10;
        locals.var_t0_dn11 = assign29020_e49998_d_n11;
        locals.var_t0_dn13 = assign29020_e49998_d_n13;
        locals.var_t0_dn14 = assign29020_e49998_d_n14;

        let (assign29030_e50009, assign29030_e50009_d_n0, assign29030_e50009_d_n2, assign29030_e50009_d_n3, assign29030_e50009_d_n4, assign29030_e50009_d_n5, assign29030_e50009_d_n6, assign29030_e50009_d_n7, assign29030_e50009_d_n8, assign29030_e50009_d_n9, assign29030_e50009_d_n10, assign29030_e50009_d_n11, assign29030_e50009_d_n13, assign29030_e50009_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29030_e50005: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29030_e50007: f64 = (assign29030_e50005 - 1.0);
        (assign29030_e50007, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29030_e50009;
        locals.var_t1_dn0 = assign29030_e50009_d_n0;
        locals.var_t1_dn2 = assign29030_e50009_d_n2;
        locals.var_t1_dn3 = assign29030_e50009_d_n3;
        locals.var_t1_dn4 = assign29030_e50009_d_n4;
        locals.var_t1_dn5 = assign29030_e50009_d_n5;
        locals.var_t1_dn6 = assign29030_e50009_d_n6;
        locals.var_t1_dn7 = assign29030_e50009_d_n7;
        locals.var_t1_dn8 = assign29030_e50009_d_n8;
        locals.var_t1_dn9 = assign29030_e50009_d_n9;
        locals.var_t1_dn10 = assign29030_e50009_d_n10;
        locals.var_t1_dn11 = assign29030_e50009_d_n11;
        locals.var_t1_dn13 = assign29030_e50009_d_n13;
        locals.var_t1_dn14 = assign29030_e50009_d_n14;

        let (assign29040_e50023, assign29040_e50023_d_n0, assign29040_e50023_d_n2, assign29040_e50023_d_n3, assign29040_e50023_d_n4, assign29040_e50023_d_n5, assign29040_e50023_d_n6, assign29040_e50023_d_n7, assign29040_e50023_d_n8, assign29040_e50023_d_n9, assign29040_e50023_d_n10, assign29040_e50023_d_n11, assign29040_e50023_d_n13, assign29040_e50023_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29040_e50019: f64 = (locals.var_ves_jct - locals.var_vjsmrev);
        let assign29040_e50020: f64 = (locals.var_sslprev * assign29040_e50019);
        let assign29040_e50021: f64 = (locals.var_ivjsmrev + assign29040_e50020);
        (assign29040_e50021, (locals.var_ivjsmrev_dn0 + ((locals.var_sslprev_dn0 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn0)))), (locals.var_ivjsmrev_dn2 + ((locals.var_sslprev_dn2 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn2)))), (locals.var_ivjsmrev_dn3 + ((locals.var_sslprev_dn3 * assign29040_e50019) + (locals.var_sslprev * (locals.var_ves_jct_dn3 - locals.var_vjsmrev_dn3)))), (locals.var_ivjsmrev_dn4 + ((locals.var_sslprev_dn4 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn4)))), (locals.var_ivjsmrev_dn5 + ((locals.var_sslprev_dn5 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn5)))), (locals.var_ivjsmrev_dn6 + ((locals.var_sslprev_dn6 * assign29040_e50019) + (locals.var_sslprev * (locals.var_ves_jct_dn6 - locals.var_vjsmrev_dn6)))), (locals.var_ivjsmrev_dn7 + ((locals.var_sslprev_dn7 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn7)))), (locals.var_ivjsmrev_dn8 + ((locals.var_sslprev_dn8 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn8)))), (locals.var_ivjsmrev_dn9 + ((locals.var_sslprev_dn9 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn9)))), (locals.var_ivjsmrev_dn10 + ((locals.var_sslprev_dn10 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn10)))), (locals.var_ivjsmrev_dn11 + ((locals.var_sslprev_dn11 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn11)))), (locals.var_ivjsmrev_dn13 + ((locals.var_sslprev_dn13 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn13)))), (locals.var_ivjsmrev_dn14 + ((locals.var_sslprev_dn14 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn14)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29040_e50023;
        locals.var_t2_dn0 = assign29040_e50023_d_n0;
        locals.var_t2_dn2 = assign29040_e50023_d_n2;
        locals.var_t2_dn3 = assign29040_e50023_d_n3;
        locals.var_t2_dn4 = assign29040_e50023_d_n4;
        locals.var_t2_dn5 = assign29040_e50023_d_n5;
        locals.var_t2_dn6 = assign29040_e50023_d_n6;
        locals.var_t2_dn7 = assign29040_e50023_d_n7;
        locals.var_t2_dn8 = assign29040_e50023_d_n8;
        locals.var_t2_dn9 = assign29040_e50023_d_n9;
        locals.var_t2_dn10 = assign29040_e50023_d_n10;
        locals.var_t2_dn11 = assign29040_e50023_d_n11;
        locals.var_t2_dn13 = assign29040_e50023_d_n13;
        locals.var_t2_dn14 = assign29040_e50023_d_n14;

        let assign29060_e50036: f64 = if locals.var_ves_jct <= locals.var_vjsmfwd { 1.0 } else { 0.0 };
        locals.var_guard472 = assign29060_e50036;

        let (assign29070_e50049, assign29070_e50049_d_n0, assign29070_e50049_d_n2, assign29070_e50049_d_n3, assign29070_e50049_d_n4, assign29070_e50049_d_n5, assign29070_e50049_d_n6, assign29070_e50049_d_n7, assign29070_e50049_d_n8, assign29070_e50049_d_n9, assign29070_e50049_d_n10, assign29070_e50049_d_n11, assign29070_e50049_d_n13, assign29070_e50049_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29070_e50047: f64 = (locals.var_ves_jct / locals.var_nvtms);
        (assign29070_e50047, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((locals.var_ves_jct * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29070_e50049;
        locals.var_t0_dn0 = assign29070_e50049_d_n0;
        locals.var_t0_dn2 = assign29070_e50049_d_n2;
        locals.var_t0_dn3 = assign29070_e50049_d_n3;
        locals.var_t0_dn4 = assign29070_e50049_d_n4;
        locals.var_t0_dn5 = assign29070_e50049_d_n5;
        locals.var_t0_dn6 = assign29070_e50049_d_n6;
        locals.var_t0_dn7 = assign29070_e50049_d_n7;
        locals.var_t0_dn8 = assign29070_e50049_d_n8;
        locals.var_t0_dn9 = assign29070_e50049_d_n9;
        locals.var_t0_dn10 = assign29070_e50049_d_n10;
        locals.var_t0_dn11 = assign29070_e50049_d_n11;
        locals.var_t0_dn13 = assign29070_e50049_d_n13;
        locals.var_t0_dn14 = assign29070_e50049_d_n14;

        let (assign29080_e50064, assign29080_e50064_d_n0, assign29080_e50064_d_n2, assign29080_e50064_d_n3, assign29080_e50064_d_n4, assign29080_e50064_d_n5, assign29080_e50064_d_n6, assign29080_e50064_d_n7, assign29080_e50064_d_n8, assign29080_e50064_d_n9, assign29080_e50064_d_n10, assign29080_e50064_d_n11, assign29080_e50064_d_n13, assign29080_e50064_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29080_e50060: f64 = (p.p1626 + locals.var_ves_jct);
        let assign29080_e50062: f64 = (assign29080_e50060 / locals.var_nvtms);
        (assign29080_e50062, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((assign29080_e50060 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29080_e50064;
        locals.var_t1_dn0 = assign29080_e50064_d_n0;
        locals.var_t1_dn2 = assign29080_e50064_d_n2;
        locals.var_t1_dn3 = assign29080_e50064_d_n3;
        locals.var_t1_dn4 = assign29080_e50064_d_n4;
        locals.var_t1_dn5 = assign29080_e50064_d_n5;
        locals.var_t1_dn6 = assign29080_e50064_d_n6;
        locals.var_t1_dn7 = assign29080_e50064_d_n7;
        locals.var_t1_dn8 = assign29080_e50064_d_n8;
        locals.var_t1_dn9 = assign29080_e50064_d_n9;
        locals.var_t1_dn10 = assign29080_e50064_d_n10;
        locals.var_t1_dn11 = assign29080_e50064_d_n11;
        locals.var_t1_dn13 = assign29080_e50064_d_n13;
        locals.var_t1_dn14 = assign29080_e50064_d_n14;

        let (assign29090_e50077, assign29090_e50077_d_n0, assign29090_e50077_d_n2, assign29090_e50077_d_n3, assign29090_e50077_d_n4, assign29090_e50077_d_n5, assign29090_e50077_d_n6, assign29090_e50077_d_n7, assign29090_e50077_d_n8, assign29090_e50077_d_n9, assign29090_e50077_d_n10, assign29090_e50077_d_n11, assign29090_e50077_d_n13, assign29090_e50077_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29090_e50074: f64 = (-locals.var_t1);
        let assign29090_e50075: f64 = { let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign29090_e50075, ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29090_e50077;
        locals.var_t2_dn0 = assign29090_e50077_d_n0;
        locals.var_t2_dn2 = assign29090_e50077_d_n2;
        locals.var_t2_dn3 = assign29090_e50077_d_n3;
        locals.var_t2_dn4 = assign29090_e50077_d_n4;
        locals.var_t2_dn5 = assign29090_e50077_d_n5;
        locals.var_t2_dn6 = assign29090_e50077_d_n6;
        locals.var_t2_dn7 = assign29090_e50077_d_n7;
        locals.var_t2_dn8 = assign29090_e50077_d_n8;
        locals.var_t2_dn9 = assign29090_e50077_d_n9;
        locals.var_t2_dn10 = assign29090_e50077_d_n10;
        locals.var_t2_dn11 = assign29090_e50077_d_n11;
        locals.var_t2_dn13 = assign29090_e50077_d_n13;
        locals.var_t2_dn14 = assign29090_e50077_d_n14;

        let assign29130_e50127: f64 = if locals.var_jtss_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign29130_e50127;

        let assign29140_e50130: f64 = (p.p1643 - locals.var_ves_jct);
        let assign29140_e50133: f64 = (p.p1643 * 0.001);
        let assign29140_e50134: f64 = if assign29140_e50130 < assign29140_e50133 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign29140_e50134;

        let (assign29150_e50147, assign29150_e50147_d_n0, assign29150_e50147_d_n2, assign29150_e50147_d_n3, assign29150_e50147_d_n4, assign29150_e50147_d_n5, assign29150_e50147_d_n6, assign29150_e50147_d_n7, assign29150_e50147_d_n8, assign29150_e50147_d_n9, assign29150_e50147_d_n10, assign29150_e50147_d_n11, assign29150_e50147_d_n13, assign29150_e50147_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign29150_e50141: f64 = (-locals.var_ves_jct);
        let assign29150_e50143: f64 = (assign29150_e50141 / locals.var_vtm0);
        let assign29150_e50145: f64 = (assign29150_e50143 / locals.var_njts_t);
        (assign29150_e50145, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njts_t), (-((assign29150_e50143 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29150_e50147;
        locals.var_t0_dn0 = assign29150_e50147_d_n0;
        locals.var_t0_dn2 = assign29150_e50147_d_n2;
        locals.var_t0_dn3 = assign29150_e50147_d_n3;
        locals.var_t0_dn4 = assign29150_e50147_d_n4;
        locals.var_t0_dn5 = assign29150_e50147_d_n5;
        locals.var_t0_dn6 = assign29150_e50147_d_n6;
        locals.var_t0_dn7 = assign29150_e50147_d_n7;
        locals.var_t0_dn8 = assign29150_e50147_d_n8;
        locals.var_t0_dn9 = assign29150_e50147_d_n9;
        locals.var_t0_dn10 = assign29150_e50147_d_n10;
        locals.var_t0_dn11 = assign29150_e50147_d_n11;
        locals.var_t0_dn13 = assign29150_e50147_d_n13;
        locals.var_t0_dn14 = assign29150_e50147_d_n14;

        let (assign29160_e50160, assign29160_e50160_d_n0, assign29160_e50160_d_n2, assign29160_e50160_d_n3, assign29160_e50160_d_n4, assign29160_e50160_d_n5, assign29160_e50160_d_n6, assign29160_e50160_d_n7, assign29160_e50160_d_n8, assign29160_e50160_d_n9, assign29160_e50160_d_n10, assign29160_e50160_d_n11, assign29160_e50160_d_n13, assign29160_e50160_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign29160_e50155: f64 = (locals.var_t0 * 1000.0);
        let assign29160_e50156: f64 = { let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29160_e50158: f64 = (assign29160_e50156 - 1.0);
        (assign29160_e50158, ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29160_e50160;
        locals.var_t1_dn0 = assign29160_e50160_d_n0;
        locals.var_t1_dn2 = assign29160_e50160_d_n2;
        locals.var_t1_dn3 = assign29160_e50160_d_n3;
        locals.var_t1_dn4 = assign29160_e50160_d_n4;
        locals.var_t1_dn5 = assign29160_e50160_d_n5;
        locals.var_t1_dn6 = assign29160_e50160_d_n6;
        locals.var_t1_dn7 = assign29160_e50160_d_n7;
        locals.var_t1_dn8 = assign29160_e50160_d_n8;
        locals.var_t1_dn9 = assign29160_e50160_d_n9;
        locals.var_t1_dn10 = assign29160_e50160_d_n10;
        locals.var_t1_dn11 = assign29160_e50160_d_n11;
        locals.var_t1_dn13 = assign29160_e50160_d_n13;
        locals.var_t1_dn14 = assign29160_e50160_d_n14;

    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29180_e50188, assign29180_e50188_d_n0, assign29180_e50188_d_n2, assign29180_e50188_d_n3, assign29180_e50188_d_n4, assign29180_e50188_d_n5, assign29180_e50188_d_n6, assign29180_e50188_d_n7, assign29180_e50188_d_n8, assign29180_e50188_d_n9, assign29180_e50188_d_n10, assign29180_e50188_d_n11, assign29180_e50188_d_n13, assign29180_e50188_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) {
        let assign29180_e50182: f64 = (-locals.var_ves_jct);
        let assign29180_e50184: f64 = (assign29180_e50182 / locals.var_vtm0);
        let assign29180_e50186: f64 = (assign29180_e50184 / locals.var_njts_t);
        (assign29180_e50186, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njts_t), (-((assign29180_e50184 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29180_e50188;
        locals.var_t0_dn0 = assign29180_e50188_d_n0;
        locals.var_t0_dn2 = assign29180_e50188_d_n2;
        locals.var_t0_dn3 = assign29180_e50188_d_n3;
        locals.var_t0_dn4 = assign29180_e50188_d_n4;
        locals.var_t0_dn5 = assign29180_e50188_d_n5;
        locals.var_t0_dn6 = assign29180_e50188_d_n6;
        locals.var_t0_dn7 = assign29180_e50188_d_n7;
        locals.var_t0_dn8 = assign29180_e50188_d_n8;
        locals.var_t0_dn9 = assign29180_e50188_d_n9;
        locals.var_t0_dn10 = assign29180_e50188_d_n10;
        locals.var_t0_dn11 = assign29180_e50188_d_n11;
        locals.var_t0_dn13 = assign29180_e50188_d_n13;
        locals.var_t0_dn14 = assign29180_e50188_d_n14;

        let (assign29190_e50206, assign29190_e50206_d_n0, assign29190_e50206_d_n2, assign29190_e50206_d_n3, assign29190_e50206_d_n4, assign29190_e50206_d_n5, assign29190_e50206_d_n6, assign29190_e50206_d_n7, assign29190_e50206_d_n8, assign29190_e50206_d_n9, assign29190_e50206_d_n10, assign29190_e50206_d_n11, assign29190_e50206_d_n13, assign29190_e50206_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) {
        let assign29190_e50197: f64 = (locals.var_t0 * p.p1643);
        let assign29190_e50200: f64 = (p.p1643 - locals.var_ves_jct);
        let assign29190_e50201: f64 = (assign29190_e50197 / assign29190_e50200);
        let assign29190_e50202: f64 = { let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29190_e50204: f64 = (assign29190_e50202 - 1.0);
        (assign29190_e50204, ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1643) * assign29190_e50200) - (assign29190_e50197 * (-locals.var_ves_jct_dn3))) / (assign29190_e50200 * assign29190_e50200))), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1643) * assign29190_e50200) - (assign29190_e50197 * (-locals.var_ves_jct_dn6))) / (assign29190_e50200 * assign29190_e50200))), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1643) / assign29190_e50200)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29190_e50206;
        locals.var_t1_dn0 = assign29190_e50206_d_n0;
        locals.var_t1_dn2 = assign29190_e50206_d_n2;
        locals.var_t1_dn3 = assign29190_e50206_d_n3;
        locals.var_t1_dn4 = assign29190_e50206_d_n4;
        locals.var_t1_dn5 = assign29190_e50206_d_n5;
        locals.var_t1_dn6 = assign29190_e50206_d_n6;
        locals.var_t1_dn7 = assign29190_e50206_d_n7;
        locals.var_t1_dn8 = assign29190_e50206_d_n8;
        locals.var_t1_dn9 = assign29190_e50206_d_n9;
        locals.var_t1_dn10 = assign29190_e50206_d_n10;
        locals.var_t1_dn11 = assign29190_e50206_d_n11;
        locals.var_t1_dn13 = assign29190_e50206_d_n13;
        locals.var_t1_dn14 = assign29190_e50206_d_n14;

        let assign29210_e50224: f64 = if locals.var_jtssws_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign29210_e50224;

        let assign29220_e50227: f64 = (p.p1645 - locals.var_ves_jct);
        let assign29220_e50230: f64 = (p.p1645 * 0.001);
        let assign29220_e50231: f64 = if assign29220_e50227 < assign29220_e50230 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign29220_e50231;

        let (assign29230_e50244, assign29230_e50244_d_n0, assign29230_e50244_d_n2, assign29230_e50244_d_n3, assign29230_e50244_d_n4, assign29230_e50244_d_n5, assign29230_e50244_d_n6, assign29230_e50244_d_n7, assign29230_e50244_d_n8, assign29230_e50244_d_n9, assign29230_e50244_d_n10, assign29230_e50244_d_n11, assign29230_e50244_d_n13, assign29230_e50244_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign29230_e50238: f64 = (-locals.var_ves_jct);
        let assign29230_e50240: f64 = (assign29230_e50238 / locals.var_vtm0);
        let assign29230_e50242: f64 = (assign29230_e50240 / locals.var_njtssw_t);
        (assign29230_e50242, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtssw_t), (-((assign29230_e50240 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29230_e50244;
        locals.var_t0_dn0 = assign29230_e50244_d_n0;
        locals.var_t0_dn2 = assign29230_e50244_d_n2;
        locals.var_t0_dn3 = assign29230_e50244_d_n3;
        locals.var_t0_dn4 = assign29230_e50244_d_n4;
        locals.var_t0_dn5 = assign29230_e50244_d_n5;
        locals.var_t0_dn6 = assign29230_e50244_d_n6;
        locals.var_t0_dn7 = assign29230_e50244_d_n7;
        locals.var_t0_dn8 = assign29230_e50244_d_n8;
        locals.var_t0_dn9 = assign29230_e50244_d_n9;
        locals.var_t0_dn10 = assign29230_e50244_d_n10;
        locals.var_t0_dn11 = assign29230_e50244_d_n11;
        locals.var_t0_dn13 = assign29230_e50244_d_n13;
        locals.var_t0_dn14 = assign29230_e50244_d_n14;

        let (assign29240_e50257, assign29240_e50257_d_n0, assign29240_e50257_d_n2, assign29240_e50257_d_n3, assign29240_e50257_d_n4, assign29240_e50257_d_n5, assign29240_e50257_d_n6, assign29240_e50257_d_n7, assign29240_e50257_d_n8, assign29240_e50257_d_n9, assign29240_e50257_d_n10, assign29240_e50257_d_n11, assign29240_e50257_d_n13, assign29240_e50257_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign29240_e50252: f64 = (locals.var_t0 * 1000.0);
        let assign29240_e50253: f64 = { let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29240_e50255: f64 = (assign29240_e50253 - 1.0);
        (assign29240_e50255, ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29240_e50257;
        locals.var_t1_dn0 = assign29240_e50257_d_n0;
        locals.var_t1_dn2 = assign29240_e50257_d_n2;
        locals.var_t1_dn3 = assign29240_e50257_d_n3;
        locals.var_t1_dn4 = assign29240_e50257_d_n4;
        locals.var_t1_dn5 = assign29240_e50257_d_n5;
        locals.var_t1_dn6 = assign29240_e50257_d_n6;
        locals.var_t1_dn7 = assign29240_e50257_d_n7;
        locals.var_t1_dn8 = assign29240_e50257_d_n8;
        locals.var_t1_dn9 = assign29240_e50257_d_n9;
        locals.var_t1_dn10 = assign29240_e50257_d_n10;
        locals.var_t1_dn11 = assign29240_e50257_d_n11;
        locals.var_t1_dn13 = assign29240_e50257_d_n13;
        locals.var_t1_dn14 = assign29240_e50257_d_n14;

        let (assign29260_e50285, assign29260_e50285_d_n0, assign29260_e50285_d_n2, assign29260_e50285_d_n3, assign29260_e50285_d_n4, assign29260_e50285_d_n5, assign29260_e50285_d_n6, assign29260_e50285_d_n7, assign29260_e50285_d_n8, assign29260_e50285_d_n9, assign29260_e50285_d_n10, assign29260_e50285_d_n11, assign29260_e50285_d_n13, assign29260_e50285_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign29260_e50279: f64 = (-locals.var_ves_jct);
        let assign29260_e50281: f64 = (assign29260_e50279 / locals.var_vtm0);
        let assign29260_e50283: f64 = (assign29260_e50281 / locals.var_njtssw_t);
        (assign29260_e50283, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtssw_t), (-((assign29260_e50281 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29260_e50285;
        locals.var_t0_dn0 = assign29260_e50285_d_n0;
        locals.var_t0_dn2 = assign29260_e50285_d_n2;
        locals.var_t0_dn3 = assign29260_e50285_d_n3;
        locals.var_t0_dn4 = assign29260_e50285_d_n4;
        locals.var_t0_dn5 = assign29260_e50285_d_n5;
        locals.var_t0_dn6 = assign29260_e50285_d_n6;
        locals.var_t0_dn7 = assign29260_e50285_d_n7;
        locals.var_t0_dn8 = assign29260_e50285_d_n8;
        locals.var_t0_dn9 = assign29260_e50285_d_n9;
        locals.var_t0_dn10 = assign29260_e50285_d_n10;
        locals.var_t0_dn11 = assign29260_e50285_d_n11;
        locals.var_t0_dn13 = assign29260_e50285_d_n13;
        locals.var_t0_dn14 = assign29260_e50285_d_n14;

        let (assign29270_e50303, assign29270_e50303_d_n0, assign29270_e50303_d_n2, assign29270_e50303_d_n3, assign29270_e50303_d_n4, assign29270_e50303_d_n5, assign29270_e50303_d_n6, assign29270_e50303_d_n7, assign29270_e50303_d_n8, assign29270_e50303_d_n9, assign29270_e50303_d_n10, assign29270_e50303_d_n11, assign29270_e50303_d_n13, assign29270_e50303_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign29270_e50294: f64 = (locals.var_t0 * p.p1645);
        let assign29270_e50297: f64 = (p.p1645 - locals.var_ves_jct);
        let assign29270_e50298: f64 = (assign29270_e50294 / assign29270_e50297);
        let assign29270_e50299: f64 = { let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29270_e50301: f64 = (assign29270_e50299 - 1.0);
        (assign29270_e50301, ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1645) * assign29270_e50297) - (assign29270_e50294 * (-locals.var_ves_jct_dn3))) / (assign29270_e50297 * assign29270_e50297))), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1645) * assign29270_e50297) - (assign29270_e50294 * (-locals.var_ves_jct_dn6))) / (assign29270_e50297 * assign29270_e50297))), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1645) / assign29270_e50297)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29270_e50303;
        locals.var_t1_dn0 = assign29270_e50303_d_n0;
        locals.var_t1_dn2 = assign29270_e50303_d_n2;
        locals.var_t1_dn3 = assign29270_e50303_d_n3;
        locals.var_t1_dn4 = assign29270_e50303_d_n4;
        locals.var_t1_dn5 = assign29270_e50303_d_n5;
        locals.var_t1_dn6 = assign29270_e50303_d_n6;
        locals.var_t1_dn7 = assign29270_e50303_d_n7;
        locals.var_t1_dn8 = assign29270_e50303_d_n8;
        locals.var_t1_dn9 = assign29270_e50303_d_n9;
        locals.var_t1_dn10 = assign29270_e50303_d_n10;
        locals.var_t1_dn11 = assign29270_e50303_d_n11;
        locals.var_t1_dn13 = assign29270_e50303_d_n13;
        locals.var_t1_dn14 = assign29270_e50303_d_n14;

        let assign29290_e50321: f64 = if locals.var_jtsswgs_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign29290_e50321;

        let assign29300_e50324: f64 = (p.p1647 - locals.var_ves_jct);
        let assign29300_e50327: f64 = (p.p1647 * 0.001);
        let assign29300_e50328: f64 = if assign29300_e50324 < assign29300_e50327 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign29300_e50328;

        let (assign29310_e50341, assign29310_e50341_d_n0, assign29310_e50341_d_n2, assign29310_e50341_d_n3, assign29310_e50341_d_n4, assign29310_e50341_d_n5, assign29310_e50341_d_n6, assign29310_e50341_d_n7, assign29310_e50341_d_n8, assign29310_e50341_d_n9, assign29310_e50341_d_n10, assign29310_e50341_d_n11, assign29310_e50341_d_n13, assign29310_e50341_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign29310_e50335: f64 = (-locals.var_ves_jct);
        let assign29310_e50337: f64 = (assign29310_e50335 / locals.var_vtm0);
        let assign29310_e50339: f64 = (assign29310_e50337 / locals.var_njtsswg_t);
        (assign29310_e50339, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtsswg_t), (-((assign29310_e50337 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29310_e50341;
        locals.var_t0_dn0 = assign29310_e50341_d_n0;
        locals.var_t0_dn2 = assign29310_e50341_d_n2;
        locals.var_t0_dn3 = assign29310_e50341_d_n3;
        locals.var_t0_dn4 = assign29310_e50341_d_n4;
        locals.var_t0_dn5 = assign29310_e50341_d_n5;
        locals.var_t0_dn6 = assign29310_e50341_d_n6;
        locals.var_t0_dn7 = assign29310_e50341_d_n7;
        locals.var_t0_dn8 = assign29310_e50341_d_n8;
        locals.var_t0_dn9 = assign29310_e50341_d_n9;
        locals.var_t0_dn10 = assign29310_e50341_d_n10;
        locals.var_t0_dn11 = assign29310_e50341_d_n11;
        locals.var_t0_dn13 = assign29310_e50341_d_n13;
        locals.var_t0_dn14 = assign29310_e50341_d_n14;

        let (assign29320_e50354, assign29320_e50354_d_n0, assign29320_e50354_d_n2, assign29320_e50354_d_n3, assign29320_e50354_d_n4, assign29320_e50354_d_n5, assign29320_e50354_d_n6, assign29320_e50354_d_n7, assign29320_e50354_d_n8, assign29320_e50354_d_n9, assign29320_e50354_d_n10, assign29320_e50354_d_n11, assign29320_e50354_d_n13, assign29320_e50354_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign29320_e50349: f64 = (locals.var_t0 * 1000.0);
        let assign29320_e50350: f64 = { let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29320_e50352: f64 = (assign29320_e50350 - 1.0);
        (assign29320_e50352, ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29320_e50354;
        locals.var_t1_dn0 = assign29320_e50354_d_n0;
        locals.var_t1_dn2 = assign29320_e50354_d_n2;
        locals.var_t1_dn3 = assign29320_e50354_d_n3;
        locals.var_t1_dn4 = assign29320_e50354_d_n4;
        locals.var_t1_dn5 = assign29320_e50354_d_n5;
        locals.var_t1_dn6 = assign29320_e50354_d_n6;
        locals.var_t1_dn7 = assign29320_e50354_d_n7;
        locals.var_t1_dn8 = assign29320_e50354_d_n8;
        locals.var_t1_dn9 = assign29320_e50354_d_n9;
        locals.var_t1_dn10 = assign29320_e50354_d_n10;
        locals.var_t1_dn11 = assign29320_e50354_d_n11;
        locals.var_t1_dn13 = assign29320_e50354_d_n13;
        locals.var_t1_dn14 = assign29320_e50354_d_n14;

        let (assign29340_e50384, assign29340_e50384_d_n0, assign29340_e50384_d_n2, assign29340_e50384_d_n3, assign29340_e50384_d_n4, assign29340_e50384_d_n5, assign29340_e50384_d_n6, assign29340_e50384_d_n7, assign29340_e50384_d_n8, assign29340_e50384_d_n9, assign29340_e50384_d_n10, assign29340_e50384_d_n11, assign29340_e50384_d_n13, assign29340_e50384_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        let assign29340_e50378: f64 = (-locals.var_ves_jct);
        let assign29340_e50380: f64 = (assign29340_e50378 / locals.var_vtm0);
        let assign29340_e50382: f64 = (assign29340_e50380 / locals.var_njtsswg_t);
        (assign29340_e50382, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtsswg_t), (-((assign29340_e50380 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29340_e50384;
        locals.var_t0_dn0 = assign29340_e50384_d_n0;
        locals.var_t0_dn2 = assign29340_e50384_d_n2;
        locals.var_t0_dn3 = assign29340_e50384_d_n3;
        locals.var_t0_dn4 = assign29340_e50384_d_n4;
        locals.var_t0_dn5 = assign29340_e50384_d_n5;
        locals.var_t0_dn6 = assign29340_e50384_d_n6;
        locals.var_t0_dn7 = assign29340_e50384_d_n7;
        locals.var_t0_dn8 = assign29340_e50384_d_n8;
        locals.var_t0_dn9 = assign29340_e50384_d_n9;
        locals.var_t0_dn10 = assign29340_e50384_d_n10;
        locals.var_t0_dn11 = assign29340_e50384_d_n11;
        locals.var_t0_dn13 = assign29340_e50384_d_n13;
        locals.var_t0_dn14 = assign29340_e50384_d_n14;

        let (assign29350_e50402, assign29350_e50402_d_n0, assign29350_e50402_d_n2, assign29350_e50402_d_n3, assign29350_e50402_d_n4, assign29350_e50402_d_n5, assign29350_e50402_d_n6, assign29350_e50402_d_n7, assign29350_e50402_d_n8, assign29350_e50402_d_n9, assign29350_e50402_d_n10, assign29350_e50402_d_n11, assign29350_e50402_d_n13, assign29350_e50402_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        let assign29350_e50393: f64 = (locals.var_t0 * p.p1647);
        let assign29350_e50396: f64 = (p.p1647 - locals.var_ves_jct);
        let assign29350_e50397: f64 = (assign29350_e50393 / assign29350_e50396);
        let assign29350_e50398: f64 = { let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29350_e50400: f64 = (assign29350_e50398 - 1.0);
        (assign29350_e50400, ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1647) * assign29350_e50396) - (assign29350_e50393 * (-locals.var_ves_jct_dn3))) / (assign29350_e50396 * assign29350_e50396))), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1647) * assign29350_e50396) - (assign29350_e50393 * (-locals.var_ves_jct_dn6))) / (assign29350_e50396 * assign29350_e50396))), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1647) / assign29350_e50396)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29350_e50402;
        locals.var_t1_dn0 = assign29350_e50402_d_n0;
        locals.var_t1_dn2 = assign29350_e50402_d_n2;
        locals.var_t1_dn3 = assign29350_e50402_d_n3;
        locals.var_t1_dn4 = assign29350_e50402_d_n4;
        locals.var_t1_dn5 = assign29350_e50402_d_n5;
        locals.var_t1_dn6 = assign29350_e50402_d_n6;
        locals.var_t1_dn7 = assign29350_e50402_d_n7;
        locals.var_t1_dn8 = assign29350_e50402_d_n8;
        locals.var_t1_dn9 = assign29350_e50402_d_n9;
        locals.var_t1_dn10 = assign29350_e50402_d_n10;
        locals.var_t1_dn11 = assign29350_e50402_d_n11;
        locals.var_t1_dn13 = assign29350_e50402_d_n13;
        locals.var_t1_dn14 = assign29350_e50402_d_n14;

        let assign29370_e50422: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign29370_e50422;

        let assign29380_e50425: f64 = if locals.var_ved_jct < locals.var_vjdmrev { 1.0 } else { 0.0 };
        locals.var_guard480 = assign29380_e50425;

        let (assign29390_e50435, assign29390_e50435_d_n0, assign29390_e50435_d_n2, assign29390_e50435_d_n3, assign29390_e50435_d_n4, assign29390_e50435_d_n5, assign29390_e50435_d_n6, assign29390_e50435_d_n7, assign29390_e50435_d_n8, assign29390_e50435_d_n9, assign29390_e50435_d_n10, assign29390_e50435_d_n11, assign29390_e50435_d_n13, assign29390_e50435_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29390_e50433: f64 = (locals.var_ved_jct / locals.var_nvtmd);
        (assign29390_e50433, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((locals.var_ved_jct * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29390_e50435;
        locals.var_t0_dn0 = assign29390_e50435_d_n0;
        locals.var_t0_dn2 = assign29390_e50435_d_n2;
        locals.var_t0_dn3 = assign29390_e50435_d_n3;
        locals.var_t0_dn4 = assign29390_e50435_d_n4;
        locals.var_t0_dn5 = assign29390_e50435_d_n5;
        locals.var_t0_dn6 = assign29390_e50435_d_n6;
        locals.var_t0_dn7 = assign29390_e50435_d_n7;
        locals.var_t0_dn8 = assign29390_e50435_d_n8;
        locals.var_t0_dn9 = assign29390_e50435_d_n9;
        locals.var_t0_dn10 = assign29390_e50435_d_n10;
        locals.var_t0_dn11 = assign29390_e50435_d_n11;
        locals.var_t0_dn13 = assign29390_e50435_d_n13;
        locals.var_t0_dn14 = assign29390_e50435_d_n14;

        let (assign29400_e50446, assign29400_e50446_d_n0, assign29400_e50446_d_n2, assign29400_e50446_d_n3, assign29400_e50446_d_n4, assign29400_e50446_d_n5, assign29400_e50446_d_n6, assign29400_e50446_d_n7, assign29400_e50446_d_n8, assign29400_e50446_d_n9, assign29400_e50446_d_n10, assign29400_e50446_d_n11, assign29400_e50446_d_n13, assign29400_e50446_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29400_e50442: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29400_e50444: f64 = (assign29400_e50442 - 1.0);
        (assign29400_e50444, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29400_e50446;
        locals.var_t1_dn0 = assign29400_e50446_d_n0;
        locals.var_t1_dn2 = assign29400_e50446_d_n2;
        locals.var_t1_dn3 = assign29400_e50446_d_n3;
        locals.var_t1_dn4 = assign29400_e50446_d_n4;
        locals.var_t1_dn5 = assign29400_e50446_d_n5;
        locals.var_t1_dn6 = assign29400_e50446_d_n6;
        locals.var_t1_dn7 = assign29400_e50446_d_n7;
        locals.var_t1_dn8 = assign29400_e50446_d_n8;
        locals.var_t1_dn9 = assign29400_e50446_d_n9;
        locals.var_t1_dn10 = assign29400_e50446_d_n10;
        locals.var_t1_dn11 = assign29400_e50446_d_n11;
        locals.var_t1_dn13 = assign29400_e50446_d_n13;
        locals.var_t1_dn14 = assign29400_e50446_d_n14;

        let (assign29410_e50460, assign29410_e50460_d_n0, assign29410_e50460_d_n2, assign29410_e50460_d_n3, assign29410_e50460_d_n4, assign29410_e50460_d_n5, assign29410_e50460_d_n6, assign29410_e50460_d_n7, assign29410_e50460_d_n8, assign29410_e50460_d_n9, assign29410_e50460_d_n10, assign29410_e50460_d_n11, assign29410_e50460_d_n13, assign29410_e50460_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29410_e50456: f64 = (locals.var_ved_jct - locals.var_vjdmrev);
        let assign29410_e50457: f64 = (locals.var_dslprev * assign29410_e50456);
        let assign29410_e50458: f64 = (locals.var_ivjdmrev + assign29410_e50457);
        (assign29410_e50458, (locals.var_ivjdmrev_dn0 + ((locals.var_dslprev_dn0 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn0)))), (locals.var_ivjdmrev_dn2 + ((locals.var_dslprev_dn2 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn2)))), (locals.var_ivjdmrev_dn3 + ((locals.var_dslprev_dn3 * assign29410_e50456) + (locals.var_dslprev * (locals.var_ved_jct_dn3 - locals.var_vjdmrev_dn3)))), (locals.var_ivjdmrev_dn4 + ((locals.var_dslprev_dn4 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn4)))), (locals.var_ivjdmrev_dn5 + ((locals.var_dslprev_dn5 * assign29410_e50456) + (locals.var_dslprev * (locals.var_ved_jct_dn5 - locals.var_vjdmrev_dn5)))), (locals.var_ivjdmrev_dn6 + ((locals.var_dslprev_dn6 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn6)))), (locals.var_ivjdmrev_dn7 + ((locals.var_dslprev_dn7 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn7)))), (locals.var_ivjdmrev_dn8 + ((locals.var_dslprev_dn8 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn8)))), (locals.var_ivjdmrev_dn9 + ((locals.var_dslprev_dn9 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn9)))), (locals.var_ivjdmrev_dn10 + ((locals.var_dslprev_dn10 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn10)))), (locals.var_ivjdmrev_dn11 + ((locals.var_dslprev_dn11 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn11)))), (locals.var_ivjdmrev_dn13 + ((locals.var_dslprev_dn13 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn13)))), (locals.var_ivjdmrev_dn14 + ((locals.var_dslprev_dn14 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn14)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29410_e50460;
        locals.var_t2_dn0 = assign29410_e50460_d_n0;
        locals.var_t2_dn2 = assign29410_e50460_d_n2;
        locals.var_t2_dn3 = assign29410_e50460_d_n3;
        locals.var_t2_dn4 = assign29410_e50460_d_n4;
        locals.var_t2_dn5 = assign29410_e50460_d_n5;
        locals.var_t2_dn6 = assign29410_e50460_d_n6;
        locals.var_t2_dn7 = assign29410_e50460_d_n7;
        locals.var_t2_dn8 = assign29410_e50460_d_n8;
        locals.var_t2_dn9 = assign29410_e50460_d_n9;
        locals.var_t2_dn10 = assign29410_e50460_d_n10;
        locals.var_t2_dn11 = assign29410_e50460_d_n11;
        locals.var_t2_dn13 = assign29410_e50460_d_n13;
        locals.var_t2_dn14 = assign29410_e50460_d_n14;

        let assign29430_e50473: f64 = if locals.var_ved_jct <= locals.var_vjdmfwd { 1.0 } else { 0.0 };
        locals.var_guard481 = assign29430_e50473;

        let (assign29440_e50486, assign29440_e50486_d_n0, assign29440_e50486_d_n2, assign29440_e50486_d_n3, assign29440_e50486_d_n4, assign29440_e50486_d_n5, assign29440_e50486_d_n6, assign29440_e50486_d_n7, assign29440_e50486_d_n8, assign29440_e50486_d_n9, assign29440_e50486_d_n10, assign29440_e50486_d_n11, assign29440_e50486_d_n13, assign29440_e50486_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29440_e50484: f64 = (locals.var_ved_jct / locals.var_nvtmd);
        (assign29440_e50484, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((locals.var_ved_jct * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29440_e50486;
        locals.var_t0_dn0 = assign29440_e50486_d_n0;
        locals.var_t0_dn2 = assign29440_e50486_d_n2;
        locals.var_t0_dn3 = assign29440_e50486_d_n3;
        locals.var_t0_dn4 = assign29440_e50486_d_n4;
        locals.var_t0_dn5 = assign29440_e50486_d_n5;
        locals.var_t0_dn6 = assign29440_e50486_d_n6;
        locals.var_t0_dn7 = assign29440_e50486_d_n7;
        locals.var_t0_dn8 = assign29440_e50486_d_n8;
        locals.var_t0_dn9 = assign29440_e50486_d_n9;
        locals.var_t0_dn10 = assign29440_e50486_d_n10;
        locals.var_t0_dn11 = assign29440_e50486_d_n11;
        locals.var_t0_dn13 = assign29440_e50486_d_n13;
        locals.var_t0_dn14 = assign29440_e50486_d_n14;

        let (assign29450_e50501, assign29450_e50501_d_n0, assign29450_e50501_d_n2, assign29450_e50501_d_n3, assign29450_e50501_d_n4, assign29450_e50501_d_n5, assign29450_e50501_d_n6, assign29450_e50501_d_n7, assign29450_e50501_d_n8, assign29450_e50501_d_n9, assign29450_e50501_d_n10, assign29450_e50501_d_n11, assign29450_e50501_d_n13, assign29450_e50501_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29450_e50497: f64 = (p.p1627 + locals.var_ved_jct);
        let assign29450_e50499: f64 = (assign29450_e50497 / locals.var_nvtmd);
        (assign29450_e50499, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((assign29450_e50497 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29450_e50501;
        locals.var_t1_dn0 = assign29450_e50501_d_n0;
        locals.var_t1_dn2 = assign29450_e50501_d_n2;
        locals.var_t1_dn3 = assign29450_e50501_d_n3;
        locals.var_t1_dn4 = assign29450_e50501_d_n4;
        locals.var_t1_dn5 = assign29450_e50501_d_n5;
        locals.var_t1_dn6 = assign29450_e50501_d_n6;
        locals.var_t1_dn7 = assign29450_e50501_d_n7;
        locals.var_t1_dn8 = assign29450_e50501_d_n8;
        locals.var_t1_dn9 = assign29450_e50501_d_n9;
        locals.var_t1_dn10 = assign29450_e50501_d_n10;
        locals.var_t1_dn11 = assign29450_e50501_d_n11;
        locals.var_t1_dn13 = assign29450_e50501_d_n13;
        locals.var_t1_dn14 = assign29450_e50501_d_n14;

        let (assign29460_e50514, assign29460_e50514_d_n0, assign29460_e50514_d_n2, assign29460_e50514_d_n3, assign29460_e50514_d_n4, assign29460_e50514_d_n5, assign29460_e50514_d_n6, assign29460_e50514_d_n7, assign29460_e50514_d_n8, assign29460_e50514_d_n9, assign29460_e50514_d_n10, assign29460_e50514_d_n11, assign29460_e50514_d_n13, assign29460_e50514_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29460_e50511: f64 = (-locals.var_t1);
        let assign29460_e50512: f64 = { let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign29460_e50512, ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29460_e50514;
        locals.var_t2_dn0 = assign29460_e50514_d_n0;
        locals.var_t2_dn2 = assign29460_e50514_d_n2;
        locals.var_t2_dn3 = assign29460_e50514_d_n3;
        locals.var_t2_dn4 = assign29460_e50514_d_n4;
        locals.var_t2_dn5 = assign29460_e50514_d_n5;
        locals.var_t2_dn6 = assign29460_e50514_d_n6;
        locals.var_t2_dn7 = assign29460_e50514_d_n7;
        locals.var_t2_dn8 = assign29460_e50514_d_n8;
        locals.var_t2_dn9 = assign29460_e50514_d_n9;
        locals.var_t2_dn10 = assign29460_e50514_d_n10;
        locals.var_t2_dn11 = assign29460_e50514_d_n11;
        locals.var_t2_dn13 = assign29460_e50514_d_n13;
        locals.var_t2_dn14 = assign29460_e50514_d_n14;

        let assign29500_e50564: f64 = if locals.var_jtsd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign29500_e50564;

        let assign29510_e50567: f64 = (p.p1644 - locals.var_ved_jct);
        let assign29510_e50570: f64 = (p.p1644 * 0.001);
        let assign29510_e50571: f64 = if assign29510_e50567 < assign29510_e50570 { 1.0 } else { 0.0 };
        locals.var_guard483 = assign29510_e50571;

        let (assign29520_e50584, assign29520_e50584_d_n0, assign29520_e50584_d_n2, assign29520_e50584_d_n3, assign29520_e50584_d_n4, assign29520_e50584_d_n5, assign29520_e50584_d_n6, assign29520_e50584_d_n7, assign29520_e50584_d_n8, assign29520_e50584_d_n9, assign29520_e50584_d_n10, assign29520_e50584_d_n11, assign29520_e50584_d_n13, assign29520_e50584_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29520_e50578: f64 = (-locals.var_ved_jct);
        let assign29520_e50580: f64 = (assign29520_e50578 / locals.var_vtm0);
        let assign29520_e50582: f64 = (assign29520_e50580 / locals.var_njtsd_t);
        (assign29520_e50582, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsd_t), (-((assign29520_e50580 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29520_e50584;
        locals.var_t0_dn0 = assign29520_e50584_d_n0;
        locals.var_t0_dn2 = assign29520_e50584_d_n2;
        locals.var_t0_dn3 = assign29520_e50584_d_n3;
        locals.var_t0_dn4 = assign29520_e50584_d_n4;
        locals.var_t0_dn5 = assign29520_e50584_d_n5;
        locals.var_t0_dn6 = assign29520_e50584_d_n6;
        locals.var_t0_dn7 = assign29520_e50584_d_n7;
        locals.var_t0_dn8 = assign29520_e50584_d_n8;
        locals.var_t0_dn9 = assign29520_e50584_d_n9;
        locals.var_t0_dn10 = assign29520_e50584_d_n10;
        locals.var_t0_dn11 = assign29520_e50584_d_n11;
        locals.var_t0_dn13 = assign29520_e50584_d_n13;
        locals.var_t0_dn14 = assign29520_e50584_d_n14;

        let (assign29530_e50597, assign29530_e50597_d_n0, assign29530_e50597_d_n2, assign29530_e50597_d_n3, assign29530_e50597_d_n4, assign29530_e50597_d_n5, assign29530_e50597_d_n6, assign29530_e50597_d_n7, assign29530_e50597_d_n8, assign29530_e50597_d_n9, assign29530_e50597_d_n10, assign29530_e50597_d_n11, assign29530_e50597_d_n13, assign29530_e50597_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29530_e50592: f64 = (locals.var_t0 * 1000.0);
        let assign29530_e50593: f64 = { let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29530_e50595: f64 = (assign29530_e50593 - 1.0);
        (assign29530_e50595, ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29530_e50597;
        locals.var_t1_dn0 = assign29530_e50597_d_n0;
        locals.var_t1_dn2 = assign29530_e50597_d_n2;
        locals.var_t1_dn3 = assign29530_e50597_d_n3;
        locals.var_t1_dn4 = assign29530_e50597_d_n4;
        locals.var_t1_dn5 = assign29530_e50597_d_n5;
        locals.var_t1_dn6 = assign29530_e50597_d_n6;
        locals.var_t1_dn7 = assign29530_e50597_d_n7;
        locals.var_t1_dn8 = assign29530_e50597_d_n8;
        locals.var_t1_dn9 = assign29530_e50597_d_n9;
        locals.var_t1_dn10 = assign29530_e50597_d_n10;
        locals.var_t1_dn11 = assign29530_e50597_d_n11;
        locals.var_t1_dn13 = assign29530_e50597_d_n13;
        locals.var_t1_dn14 = assign29530_e50597_d_n14;

        let (assign29550_e50625, assign29550_e50625_d_n0, assign29550_e50625_d_n2, assign29550_e50625_d_n3, assign29550_e50625_d_n4, assign29550_e50625_d_n5, assign29550_e50625_d_n6, assign29550_e50625_d_n7, assign29550_e50625_d_n8, assign29550_e50625_d_n9, assign29550_e50625_d_n10, assign29550_e50625_d_n11, assign29550_e50625_d_n13, assign29550_e50625_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 == 0.0)) {
        let assign29550_e50619: f64 = (-locals.var_ved_jct);
        let assign29550_e50621: f64 = (assign29550_e50619 / locals.var_vtm0);
        let assign29550_e50623: f64 = (assign29550_e50621 / locals.var_njtsd_t);
        (assign29550_e50623, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsd_t), (-((assign29550_e50621 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29550_e50625;
        locals.var_t0_dn0 = assign29550_e50625_d_n0;
        locals.var_t0_dn2 = assign29550_e50625_d_n2;
        locals.var_t0_dn3 = assign29550_e50625_d_n3;
        locals.var_t0_dn4 = assign29550_e50625_d_n4;
        locals.var_t0_dn5 = assign29550_e50625_d_n5;
        locals.var_t0_dn6 = assign29550_e50625_d_n6;
        locals.var_t0_dn7 = assign29550_e50625_d_n7;
        locals.var_t0_dn8 = assign29550_e50625_d_n8;
        locals.var_t0_dn9 = assign29550_e50625_d_n9;
        locals.var_t0_dn10 = assign29550_e50625_d_n10;
        locals.var_t0_dn11 = assign29550_e50625_d_n11;
        locals.var_t0_dn13 = assign29550_e50625_d_n13;
        locals.var_t0_dn14 = assign29550_e50625_d_n14;

    }

    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29560_e50643, assign29560_e50643_d_n0, assign29560_e50643_d_n2, assign29560_e50643_d_n3, assign29560_e50643_d_n4, assign29560_e50643_d_n5, assign29560_e50643_d_n6, assign29560_e50643_d_n7, assign29560_e50643_d_n8, assign29560_e50643_d_n9, assign29560_e50643_d_n10, assign29560_e50643_d_n11, assign29560_e50643_d_n13, assign29560_e50643_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 == 0.0)) {
        let assign29560_e50634: f64 = (locals.var_t0 * p.p1644);
        let assign29560_e50637: f64 = (p.p1644 - locals.var_ved_jct);
        let assign29560_e50638: f64 = (assign29560_e50634 / assign29560_e50637);
        let assign29560_e50639: f64 = { let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29560_e50641: f64 = (assign29560_e50639 - 1.0);
        (assign29560_e50641, ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1644) * assign29560_e50637) - (assign29560_e50634 * (-locals.var_ved_jct_dn3))) / (assign29560_e50637 * assign29560_e50637))), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1644) * assign29560_e50637) - (assign29560_e50634 * (-locals.var_ved_jct_dn5))) / (assign29560_e50637 * assign29560_e50637))), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1644) / assign29560_e50637)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29560_e50643;
        locals.var_t1_dn0 = assign29560_e50643_d_n0;
        locals.var_t1_dn2 = assign29560_e50643_d_n2;
        locals.var_t1_dn3 = assign29560_e50643_d_n3;
        locals.var_t1_dn4 = assign29560_e50643_d_n4;
        locals.var_t1_dn5 = assign29560_e50643_d_n5;
        locals.var_t1_dn6 = assign29560_e50643_d_n6;
        locals.var_t1_dn7 = assign29560_e50643_d_n7;
        locals.var_t1_dn8 = assign29560_e50643_d_n8;
        locals.var_t1_dn9 = assign29560_e50643_d_n9;
        locals.var_t1_dn10 = assign29560_e50643_d_n10;
        locals.var_t1_dn11 = assign29560_e50643_d_n11;
        locals.var_t1_dn13 = assign29560_e50643_d_n13;
        locals.var_t1_dn14 = assign29560_e50643_d_n14;

        let assign29580_e50661: f64 = if locals.var_jtsswd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign29580_e50661;

        let assign29590_e50664: f64 = (p.p1646 - locals.var_ved_jct);
        let assign29590_e50667: f64 = (p.p1646 * 0.001);
        let assign29590_e50668: f64 = if assign29590_e50664 < assign29590_e50667 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign29590_e50668;

        let (assign29600_e50681, assign29600_e50681_d_n0, assign29600_e50681_d_n2, assign29600_e50681_d_n3, assign29600_e50681_d_n4, assign29600_e50681_d_n5, assign29600_e50681_d_n6, assign29600_e50681_d_n7, assign29600_e50681_d_n8, assign29600_e50681_d_n9, assign29600_e50681_d_n10, assign29600_e50681_d_n11, assign29600_e50681_d_n13, assign29600_e50681_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign29600_e50675: f64 = (-locals.var_ved_jct);
        let assign29600_e50677: f64 = (assign29600_e50675 / locals.var_vtm0);
        let assign29600_e50679: f64 = (assign29600_e50677 / locals.var_njtsswd_t);
        (assign29600_e50679, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswd_t), (-((assign29600_e50677 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29600_e50681;
        locals.var_t0_dn0 = assign29600_e50681_d_n0;
        locals.var_t0_dn2 = assign29600_e50681_d_n2;
        locals.var_t0_dn3 = assign29600_e50681_d_n3;
        locals.var_t0_dn4 = assign29600_e50681_d_n4;
        locals.var_t0_dn5 = assign29600_e50681_d_n5;
        locals.var_t0_dn6 = assign29600_e50681_d_n6;
        locals.var_t0_dn7 = assign29600_e50681_d_n7;
        locals.var_t0_dn8 = assign29600_e50681_d_n8;
        locals.var_t0_dn9 = assign29600_e50681_d_n9;
        locals.var_t0_dn10 = assign29600_e50681_d_n10;
        locals.var_t0_dn11 = assign29600_e50681_d_n11;
        locals.var_t0_dn13 = assign29600_e50681_d_n13;
        locals.var_t0_dn14 = assign29600_e50681_d_n14;

        let (assign29610_e50694, assign29610_e50694_d_n0, assign29610_e50694_d_n2, assign29610_e50694_d_n3, assign29610_e50694_d_n4, assign29610_e50694_d_n5, assign29610_e50694_d_n6, assign29610_e50694_d_n7, assign29610_e50694_d_n8, assign29610_e50694_d_n9, assign29610_e50694_d_n10, assign29610_e50694_d_n11, assign29610_e50694_d_n13, assign29610_e50694_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign29610_e50689: f64 = (locals.var_t0 * 1000.0);
        let assign29610_e50690: f64 = { let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29610_e50692: f64 = (assign29610_e50690 - 1.0);
        (assign29610_e50692, ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29610_e50694;
        locals.var_t1_dn0 = assign29610_e50694_d_n0;
        locals.var_t1_dn2 = assign29610_e50694_d_n2;
        locals.var_t1_dn3 = assign29610_e50694_d_n3;
        locals.var_t1_dn4 = assign29610_e50694_d_n4;
        locals.var_t1_dn5 = assign29610_e50694_d_n5;
        locals.var_t1_dn6 = assign29610_e50694_d_n6;
        locals.var_t1_dn7 = assign29610_e50694_d_n7;
        locals.var_t1_dn8 = assign29610_e50694_d_n8;
        locals.var_t1_dn9 = assign29610_e50694_d_n9;
        locals.var_t1_dn10 = assign29610_e50694_d_n10;
        locals.var_t1_dn11 = assign29610_e50694_d_n11;
        locals.var_t1_dn13 = assign29610_e50694_d_n13;
        locals.var_t1_dn14 = assign29610_e50694_d_n14;

        let (assign29630_e50722, assign29630_e50722_d_n0, assign29630_e50722_d_n2, assign29630_e50722_d_n3, assign29630_e50722_d_n4, assign29630_e50722_d_n5, assign29630_e50722_d_n6, assign29630_e50722_d_n7, assign29630_e50722_d_n8, assign29630_e50722_d_n9, assign29630_e50722_d_n10, assign29630_e50722_d_n11, assign29630_e50722_d_n13, assign29630_e50722_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign29630_e50716: f64 = (-locals.var_ved_jct);
        let assign29630_e50718: f64 = (assign29630_e50716 / locals.var_vtm0);
        let assign29630_e50720: f64 = (assign29630_e50718 / locals.var_njtsswd_t);
        (assign29630_e50720, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswd_t), (-((assign29630_e50718 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29630_e50722;
        locals.var_t0_dn0 = assign29630_e50722_d_n0;
        locals.var_t0_dn2 = assign29630_e50722_d_n2;
        locals.var_t0_dn3 = assign29630_e50722_d_n3;
        locals.var_t0_dn4 = assign29630_e50722_d_n4;
        locals.var_t0_dn5 = assign29630_e50722_d_n5;
        locals.var_t0_dn6 = assign29630_e50722_d_n6;
        locals.var_t0_dn7 = assign29630_e50722_d_n7;
        locals.var_t0_dn8 = assign29630_e50722_d_n8;
        locals.var_t0_dn9 = assign29630_e50722_d_n9;
        locals.var_t0_dn10 = assign29630_e50722_d_n10;
        locals.var_t0_dn11 = assign29630_e50722_d_n11;
        locals.var_t0_dn13 = assign29630_e50722_d_n13;
        locals.var_t0_dn14 = assign29630_e50722_d_n14;

        let (assign29640_e50740, assign29640_e50740_d_n0, assign29640_e50740_d_n2, assign29640_e50740_d_n3, assign29640_e50740_d_n4, assign29640_e50740_d_n5, assign29640_e50740_d_n6, assign29640_e50740_d_n7, assign29640_e50740_d_n8, assign29640_e50740_d_n9, assign29640_e50740_d_n10, assign29640_e50740_d_n11, assign29640_e50740_d_n13, assign29640_e50740_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign29640_e50731: f64 = (locals.var_t0 * p.p1646);
        let assign29640_e50734: f64 = (p.p1646 - locals.var_ved_jct);
        let assign29640_e50735: f64 = (assign29640_e50731 / assign29640_e50734);
        let assign29640_e50736: f64 = { let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29640_e50738: f64 = (assign29640_e50736 - 1.0);
        (assign29640_e50738, ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1646) * assign29640_e50734) - (assign29640_e50731 * (-locals.var_ved_jct_dn3))) / (assign29640_e50734 * assign29640_e50734))), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1646) * assign29640_e50734) - (assign29640_e50731 * (-locals.var_ved_jct_dn5))) / (assign29640_e50734 * assign29640_e50734))), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1646) / assign29640_e50734)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29640_e50740;
        locals.var_t1_dn0 = assign29640_e50740_d_n0;
        locals.var_t1_dn2 = assign29640_e50740_d_n2;
        locals.var_t1_dn3 = assign29640_e50740_d_n3;
        locals.var_t1_dn4 = assign29640_e50740_d_n4;
        locals.var_t1_dn5 = assign29640_e50740_d_n5;
        locals.var_t1_dn6 = assign29640_e50740_d_n6;
        locals.var_t1_dn7 = assign29640_e50740_d_n7;
        locals.var_t1_dn8 = assign29640_e50740_d_n8;
        locals.var_t1_dn9 = assign29640_e50740_d_n9;
        locals.var_t1_dn10 = assign29640_e50740_d_n10;
        locals.var_t1_dn11 = assign29640_e50740_d_n11;
        locals.var_t1_dn13 = assign29640_e50740_d_n13;
        locals.var_t1_dn14 = assign29640_e50740_d_n14;

        let assign29660_e50758: f64 = if locals.var_jtsswgd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign29660_e50758;

        let assign29670_e50761: f64 = (p.p1648 - locals.var_ved_jct);
        let assign29670_e50764: f64 = (p.p1648 * 0.001);
        let assign29670_e50765: f64 = if assign29670_e50761 < assign29670_e50764 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign29670_e50765;

        let (assign29680_e50778, assign29680_e50778_d_n0, assign29680_e50778_d_n2, assign29680_e50778_d_n3, assign29680_e50778_d_n4, assign29680_e50778_d_n5, assign29680_e50778_d_n6, assign29680_e50778_d_n7, assign29680_e50778_d_n8, assign29680_e50778_d_n9, assign29680_e50778_d_n10, assign29680_e50778_d_n11, assign29680_e50778_d_n13, assign29680_e50778_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign29680_e50772: f64 = (-locals.var_ved_jct);
        let assign29680_e50774: f64 = (assign29680_e50772 / locals.var_vtm0);
        let assign29680_e50776: f64 = (assign29680_e50774 / locals.var_njtsswgd_t);
        (assign29680_e50776, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswgd_t), (-((assign29680_e50774 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29680_e50778;
        locals.var_t0_dn0 = assign29680_e50778_d_n0;
        locals.var_t0_dn2 = assign29680_e50778_d_n2;
        locals.var_t0_dn3 = assign29680_e50778_d_n3;
        locals.var_t0_dn4 = assign29680_e50778_d_n4;
        locals.var_t0_dn5 = assign29680_e50778_d_n5;
        locals.var_t0_dn6 = assign29680_e50778_d_n6;
        locals.var_t0_dn7 = assign29680_e50778_d_n7;
        locals.var_t0_dn8 = assign29680_e50778_d_n8;
        locals.var_t0_dn9 = assign29680_e50778_d_n9;
        locals.var_t0_dn10 = assign29680_e50778_d_n10;
        locals.var_t0_dn11 = assign29680_e50778_d_n11;
        locals.var_t0_dn13 = assign29680_e50778_d_n13;
        locals.var_t0_dn14 = assign29680_e50778_d_n14;

        let (assign29690_e50791, assign29690_e50791_d_n0, assign29690_e50791_d_n2, assign29690_e50791_d_n3, assign29690_e50791_d_n4, assign29690_e50791_d_n5, assign29690_e50791_d_n6, assign29690_e50791_d_n7, assign29690_e50791_d_n8, assign29690_e50791_d_n9, assign29690_e50791_d_n10, assign29690_e50791_d_n11, assign29690_e50791_d_n13, assign29690_e50791_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign29690_e50786: f64 = (locals.var_t0 * 1000.0);
        let assign29690_e50787: f64 = { let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29690_e50789: f64 = (assign29690_e50787 - 1.0);
        (assign29690_e50789, ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29690_e50791;
        locals.var_t1_dn0 = assign29690_e50791_d_n0;
        locals.var_t1_dn2 = assign29690_e50791_d_n2;
        locals.var_t1_dn3 = assign29690_e50791_d_n3;
        locals.var_t1_dn4 = assign29690_e50791_d_n4;
        locals.var_t1_dn5 = assign29690_e50791_d_n5;
        locals.var_t1_dn6 = assign29690_e50791_d_n6;
        locals.var_t1_dn7 = assign29690_e50791_d_n7;
        locals.var_t1_dn8 = assign29690_e50791_d_n8;
        locals.var_t1_dn9 = assign29690_e50791_d_n9;
        locals.var_t1_dn10 = assign29690_e50791_d_n10;
        locals.var_t1_dn11 = assign29690_e50791_d_n11;
        locals.var_t1_dn13 = assign29690_e50791_d_n13;
        locals.var_t1_dn14 = assign29690_e50791_d_n14;

        let (assign29710_e50821, assign29710_e50821_d_n0, assign29710_e50821_d_n2, assign29710_e50821_d_n3, assign29710_e50821_d_n4, assign29710_e50821_d_n5, assign29710_e50821_d_n6, assign29710_e50821_d_n7, assign29710_e50821_d_n8, assign29710_e50821_d_n9, assign29710_e50821_d_n10, assign29710_e50821_d_n11, assign29710_e50821_d_n13, assign29710_e50821_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign29710_e50815: f64 = (-locals.var_ved_jct);
        let assign29710_e50817: f64 = (assign29710_e50815 / locals.var_vtm0);
        let assign29710_e50819: f64 = (assign29710_e50817 / locals.var_njtsswgd_t);
        (assign29710_e50819, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswgd_t), (-((assign29710_e50817 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29710_e50821;
        locals.var_t0_dn0 = assign29710_e50821_d_n0;
        locals.var_t0_dn2 = assign29710_e50821_d_n2;
        locals.var_t0_dn3 = assign29710_e50821_d_n3;
        locals.var_t0_dn4 = assign29710_e50821_d_n4;
        locals.var_t0_dn5 = assign29710_e50821_d_n5;
        locals.var_t0_dn6 = assign29710_e50821_d_n6;
        locals.var_t0_dn7 = assign29710_e50821_d_n7;
        locals.var_t0_dn8 = assign29710_e50821_d_n8;
        locals.var_t0_dn9 = assign29710_e50821_d_n9;
        locals.var_t0_dn10 = assign29710_e50821_d_n10;
        locals.var_t0_dn11 = assign29710_e50821_d_n11;
        locals.var_t0_dn13 = assign29710_e50821_d_n13;
        locals.var_t0_dn14 = assign29710_e50821_d_n14;

        let (assign29720_e50839, assign29720_e50839_d_n0, assign29720_e50839_d_n2, assign29720_e50839_d_n3, assign29720_e50839_d_n4, assign29720_e50839_d_n5, assign29720_e50839_d_n6, assign29720_e50839_d_n7, assign29720_e50839_d_n8, assign29720_e50839_d_n9, assign29720_e50839_d_n10, assign29720_e50839_d_n11, assign29720_e50839_d_n13, assign29720_e50839_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign29720_e50830: f64 = (locals.var_t0 * p.p1648);
        let assign29720_e50833: f64 = (p.p1648 - locals.var_ved_jct);
        let assign29720_e50834: f64 = (assign29720_e50830 / assign29720_e50833);
        let assign29720_e50835: f64 = { let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29720_e50837: f64 = (assign29720_e50835 - 1.0);
        (assign29720_e50837, ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1648) * assign29720_e50833) - (assign29720_e50830 * (-locals.var_ved_jct_dn3))) / (assign29720_e50833 * assign29720_e50833))), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1648) * assign29720_e50833) - (assign29720_e50830 * (-locals.var_ved_jct_dn5))) / (assign29720_e50833 * assign29720_e50833))), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1648) / assign29720_e50833)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29720_e50839;
        locals.var_t1_dn0 = assign29720_e50839_d_n0;
        locals.var_t1_dn2 = assign29720_e50839_d_n2;
        locals.var_t1_dn3 = assign29720_e50839_d_n3;
        locals.var_t1_dn4 = assign29720_e50839_d_n4;
        locals.var_t1_dn5 = assign29720_e50839_d_n5;
        locals.var_t1_dn6 = assign29720_e50839_d_n6;
        locals.var_t1_dn7 = assign29720_e50839_d_n7;
        locals.var_t1_dn8 = assign29720_e50839_d_n8;
        locals.var_t1_dn9 = assign29720_e50839_d_n9;
        locals.var_t1_dn10 = assign29720_e50839_d_n10;
        locals.var_t1_dn11 = assign29720_e50839_d_n11;
        locals.var_t1_dn13 = assign29720_e50839_d_n13;
        locals.var_t1_dn14 = assign29720_e50839_d_n14;

        let assign29740_e50859: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign29740_e50859;

        let (assign29750_e50867, assign29750_e50867_d_n3, assign29750_e50867_d_n4, assign29750_e50867_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) {
        let assign29750_e50865: f64 = (locals.var_ves_jct / locals.var_pbs_t);
        (assign29750_e50865, (locals.var_ves_jct_dn3 / locals.var_pbs_t), (-((locals.var_ves_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (locals.var_ves_jct_dn6 / locals.var_pbs_t),)
    } else {
        (locals.var_t1__blk488, locals.var_t1__blk488_dn3, locals.var_t1__blk488_dn4, locals.var_t1__blk488_dn6,)
    }
};
        locals.var_t1__blk488 = assign29750_e50867;
        locals.var_t1__blk488_dn3 = assign29750_e50867_d_n3;
        locals.var_t1__blk488_dn4 = assign29750_e50867_d_n4;
        locals.var_t1__blk488_dn6 = assign29750_e50867_d_n6;

        let assign29760_e50870: f64 = if locals.var_t1__blk488 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign29760_e50870;

        let assign29770_e50873: f64 = if p.p1602 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign29770_e50873;

        let assign29780_e50876: f64 = if locals.var_ves_jct > locals.var_vec1s { 1.0 } else { 0.0 };
        locals.var_guard499 = assign29780_e50876;

        let (assign29790_e50890, assign29790_e50890_d_n3, assign29790_e50890_d_n4, assign29790_e50890_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) {
        let assign29790_e50888: f64 = (1.0 - locals.var_t1__blk488);
        (assign29790_e50888, (-locals.var_t1__blk488_dn3), (-locals.var_t1__blk488_dn4), (-locals.var_t1__blk488_dn6),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29790_e50890;
        locals.var_arg_dn3 = assign29790_e50890_d_n3;
        locals.var_arg_dn4 = assign29790_e50890_d_n4;
        locals.var_arg_dn6 = assign29790_e50890_d_n6;

        let assign29800_e50893: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign29800_e50893;

        let assign29810_e50896: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign29810_e50896;

        let (assign29820_e50915, assign29820_e50915_d_n3, assign29820_e50915_d_n4, assign29820_e50915_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign29820_e50912: f64 = (locals.var_arg).sqrt();
        let assign29820_e50913: f64 = (1.0 / assign29820_e50912);
        (assign29820_e50913, (-((locals.var_arg_dn3 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))), (-((locals.var_arg_dn4 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))), (-((locals.var_arg_dn6 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29820_e50915;
        locals.var_sarg_dn3 = assign29820_e50915_d_n3;
        locals.var_sarg_dn4 = assign29820_e50915_d_n4;
        locals.var_sarg_dn6 = assign29820_e50915_d_n6;

        let (assign29830_e50935, assign29830_e50935_d_n3, assign29830_e50935_d_n4, assign29830_e50935_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 == 0.0)) {
        let assign29830_e50932: f64 = (-p.p1596);
        let assign29830_e50933: f64 = (locals.var_arg).powf(assign29830_e50932);
        (assign29830_e50933, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn3)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn4)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn6)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29830_e50935;
        locals.var_sarg_dn3 = assign29830_e50935_d_n3;
        locals.var_sarg_dn4 = assign29830_e50935_d_n4;
        locals.var_sarg_dn6 = assign29830_e50935_d_n6;

        let (assign29840_e50961, assign29840_e50961_d_n3, assign29840_e50961_d_n4, assign29840_e50961_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) {
        let assign29840_e50949: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign29840_e50953: f64 = (locals.var_arg * locals.var_sarg);
        let assign29840_e50954: f64 = (1.0 - assign29840_e50953);
        let assign29840_e50955: f64 = (assign29840_e50949 * assign29840_e50954);
        let assign29840_e50958: f64 = (1.0 - p.p1596);
        let assign29840_e50959: f64 = (assign29840_e50955 / assign29840_e50958);
        (assign29840_e50959, ((assign29840_e50949 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29840_e50958), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign29840_e50954) + (assign29840_e50949 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29840_e50958), ((assign29840_e50949 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29840_e50958),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29840_e50961;
        locals.var_qesj1_dn3 = assign29840_e50961_d_n3;
        locals.var_qesj1_dn4 = assign29840_e50961_d_n4;
        locals.var_qesj1_dn6 = assign29840_e50961_d_n6;

        let (assign29850_e50994, assign29850_e50994_d_n3, assign29850_e50994_d_n4, assign29850_e50994_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 == 0.0)) {
        let assign29850_e50975: f64 = (-locals.var_pbs_t);
        let assign29850_e50977: f64 = (assign29850_e50975 * locals.var_czbs);
        let (assign29850_e50991, assign29850_e50991_d_n3, assign29850_e50991_d_n4, assign29850_e50991_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29850_e50983: f64 = (-87.498233534);
                (assign29850_e50983, 0.0, 0.0, 0.0,)
            } else {
                let (assign29850_e50990, assign29850_e50990_d_n3, assign29850_e50990_d_n4, assign29850_e50990_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29850_e50988: f64 = (locals.var_arg).ln();
                        (assign29850_e50988, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29850_e50990, assign29850_e50990_d_n3, assign29850_e50990_d_n4, assign29850_e50990_d_n6,)
            }
        };
        let assign29850_e50992: f64 = (assign29850_e50977 * assign29850_e50991);
        (assign29850_e50992, (assign29850_e50977 * assign29850_e50991_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign29850_e50975 * locals.var_czbs_dn4)) * assign29850_e50991) + (assign29850_e50977 * assign29850_e50991_d_n4)), (assign29850_e50977 * assign29850_e50991_d_n6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29850_e50994;
        locals.var_qesj1_dn3 = assign29850_e50994_d_n3;
        locals.var_qesj1_dn4 = assign29850_e50994_d_n4;
        locals.var_qesj1_dn6 = assign29850_e50994_d_n6;

        let (assign29860_e51011, assign29860_e51011_d_n3, assign29860_e51011_d_n4, assign29860_e51011_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) {
        let assign29860_e51008: f64 = (locals.var_vec1s / locals.var_pbs_t);
        let assign29860_e51009: f64 = (1.0 - assign29860_e51008);
        (assign29860_e51009, 0.0, (-(((locals.var_vec1s_dn4 * locals.var_pbs_t) - (locals.var_vec1s * locals.var_pbs_t_dn4)) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29860_e51011;
        locals.var_arg_dn3 = assign29860_e51011_d_n3;
        locals.var_arg_dn4 = assign29860_e51011_d_n4;
        locals.var_arg_dn6 = assign29860_e51011_d_n6;

        let assign29870_e51014: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign29870_e51014;

        let assign29880_e51017: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign29880_e51017;

        let (assign29890_e51037, assign29890_e51037_d_n3, assign29890_e51037_d_n4, assign29890_e51037_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 != 0.0)) {
        let assign29890_e51034: f64 = (locals.var_arg).sqrt();
        let assign29890_e51035: f64 = (1.0 / assign29890_e51034);
        (assign29890_e51035, (-((locals.var_arg_dn3 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))), (-((locals.var_arg_dn4 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))), (-((locals.var_arg_dn6 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29890_e51037;
        locals.var_sarg_dn3 = assign29890_e51037_d_n3;
        locals.var_sarg_dn4 = assign29890_e51037_d_n4;
        locals.var_sarg_dn6 = assign29890_e51037_d_n6;

        let (assign29900_e51058, assign29900_e51058_d_n3, assign29900_e51058_d_n4, assign29900_e51058_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 == 0.0)) {
        let assign29900_e51055: f64 = (-p.p1596);
        let assign29900_e51056: f64 = (locals.var_arg).powf(assign29900_e51055);
        (assign29900_e51056, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn3)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn4)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn6)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29900_e51058;
        locals.var_sarg_dn3 = assign29900_e51058_d_n3;
        locals.var_sarg_dn4 = assign29900_e51058_d_n4;
        locals.var_sarg_dn6 = assign29900_e51058_d_n6;

        let (assign29910_e51085, assign29910_e51085_d_n3, assign29910_e51085_d_n4, assign29910_e51085_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) {
        let assign29910_e51073: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign29910_e51077: f64 = (locals.var_arg * locals.var_sarg);
        let assign29910_e51078: f64 = (1.0 - assign29910_e51077);
        let assign29910_e51079: f64 = (assign29910_e51073 * assign29910_e51078);
        let assign29910_e51082: f64 = (1.0 - p.p1596);
        let assign29910_e51083: f64 = (assign29910_e51079 / assign29910_e51082);
        (assign29910_e51083, ((assign29910_e51073 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29910_e51082), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign29910_e51078) + (assign29910_e51073 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29910_e51082), ((assign29910_e51073 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29910_e51082),)
    } else {
        (locals.var_qec, locals.var_qec_dn3, locals.var_qec_dn4, locals.var_qec_dn6,)
    }
};
        locals.var_qec = assign29910_e51085;
        locals.var_qec_dn3 = assign29910_e51085_d_n3;
        locals.var_qec_dn4 = assign29910_e51085_d_n4;
        locals.var_qec_dn6 = assign29910_e51085_d_n6;

        let (assign29920_e51119, assign29920_e51119_d_n3, assign29920_e51119_d_n4, assign29920_e51119_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 == 0.0)) {
        let assign29920_e51100: f64 = (-locals.var_pbs_t);
        let assign29920_e51102: f64 = (assign29920_e51100 * locals.var_czbs);
        let (assign29920_e51116, assign29920_e51116_d_n3, assign29920_e51116_d_n4, assign29920_e51116_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29920_e51108: f64 = (-87.498233534);
                (assign29920_e51108, 0.0, 0.0, 0.0,)
            } else {
                let (assign29920_e51115, assign29920_e51115_d_n3, assign29920_e51115_d_n4, assign29920_e51115_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29920_e51113: f64 = (locals.var_arg).ln();
                        (assign29920_e51113, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29920_e51115, assign29920_e51115_d_n3, assign29920_e51115_d_n4, assign29920_e51115_d_n6,)
            }
        };
        let assign29920_e51117: f64 = (assign29920_e51102 * assign29920_e51116);
        (assign29920_e51117, (assign29920_e51102 * assign29920_e51116_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign29920_e51100 * locals.var_czbs_dn4)) * assign29920_e51116) + (assign29920_e51102 * assign29920_e51116_d_n4)), (assign29920_e51102 * assign29920_e51116_d_n6),)
    } else {
        (locals.var_qec, locals.var_qec_dn3, locals.var_qec_dn4, locals.var_qec_dn6,)
    }
};
        locals.var_qec = assign29920_e51119;
        locals.var_qec_dn3 = assign29920_e51119_d_n3;
        locals.var_qec_dn4 = assign29920_e51119_d_n4;
        locals.var_qec_dn6 = assign29920_e51119_d_n6;

        let (assign29930_e51138, assign29930_e51138_d_n3, assign29930_e51138_d_n4, assign29930_e51138_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) {
        let assign29930_e51133: f64 = (locals.var_ves_jct - locals.var_vec1s);
        let assign29930_e51135: f64 = (assign29930_e51133 / locals.var_pb21s);
        let assign29930_e51136: f64 = (1.0 - assign29930_e51135);
        (assign29930_e51136, (-(locals.var_ves_jct_dn3 / locals.var_pb21s)), (-((((-locals.var_vec1s_dn4) * locals.var_pb21s) - (assign29930_e51133 * locals.var_pb21s_dn4)) / (locals.var_pb21s * locals.var_pb21s))), (-(locals.var_ves_jct_dn6 / locals.var_pb21s)),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29930_e51138;
        locals.var_arg_dn3 = assign29930_e51138_d_n3;
        locals.var_arg_dn4 = assign29930_e51138_d_n4;
        locals.var_arg_dn6 = assign29930_e51138_d_n6;

        let assign29940_e51141: f64 = if p.p1608 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard504 = assign29940_e51141;

        let assign29950_e51144: f64 = if p.p1608 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign29950_e51144;

        let (assign29960_e51164, assign29960_e51164_d_n3, assign29960_e51164_d_n4, assign29960_e51164_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 != 0.0)) {
        let assign29960_e51161: f64 = (locals.var_arg).sqrt();
        let assign29960_e51162: f64 = (1.0 / assign29960_e51161);
        (assign29960_e51162, (-((locals.var_arg_dn3 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))), (-((locals.var_arg_dn4 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))), (-((locals.var_arg_dn6 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29960_e51164;
        locals.var_sarg_dn3 = assign29960_e51164_d_n3;
        locals.var_sarg_dn4 = assign29960_e51164_d_n4;
        locals.var_sarg_dn6 = assign29960_e51164_d_n6;

    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29970_e51185, assign29970_e51185_d_n3, assign29970_e51185_d_n4, assign29970_e51185_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 == 0.0)) {
        let assign29970_e51182: f64 = (-p.p1608);
        let assign29970_e51183: f64 = (locals.var_arg).powf(assign29970_e51182);
        (assign29970_e51183, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn3)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn4)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn6)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29970_e51185;
        locals.var_sarg_dn3 = assign29970_e51185_d_n3;
        locals.var_sarg_dn4 = assign29970_e51185_d_n4;
        locals.var_sarg_dn6 = assign29970_e51185_d_n6;

        let (assign29980_e51216, assign29980_e51216_d_n3, assign29980_e51216_d_n4, assign29980_e51216_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) {
        let assign29980_e51201: f64 = (p.p1602 * locals.var_pb21s);
        let assign29980_e51203: f64 = (assign29980_e51201 * locals.var_czbs);
        let assign29980_e51207: f64 = (locals.var_arg * locals.var_sarg);
        let assign29980_e51208: f64 = (1.0 - assign29980_e51207);
        let assign29980_e51209: f64 = (assign29980_e51203 * assign29980_e51208);
        let assign29980_e51212: f64 = (1.0 - p.p1608);
        let assign29980_e51213: f64 = (assign29980_e51209 / assign29980_e51212);
        let assign29980_e51214: f64 = (locals.var_qec + assign29980_e51213);
        (assign29980_e51214, (locals.var_qec_dn3 + ((assign29980_e51203 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29980_e51212)), (locals.var_qec_dn4 + ((((((p.p1602 * locals.var_pb21s_dn4) * locals.var_czbs) + (assign29980_e51201 * locals.var_czbs_dn4)) * assign29980_e51208) + (assign29980_e51203 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29980_e51212)), (locals.var_qec_dn6 + ((assign29980_e51203 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29980_e51212)),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29980_e51216;
        locals.var_qesj1_dn3 = assign29980_e51216_d_n3;
        locals.var_qesj1_dn4 = assign29980_e51216_d_n4;
        locals.var_qesj1_dn6 = assign29980_e51216_d_n6;

        let (assign29990_e51253, assign29990_e51253_d_n3, assign29990_e51253_d_n4, assign29990_e51253_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 == 0.0)) {
        let assign29990_e51233: f64 = (p.p1602 * locals.var_pb21s);
        let assign29990_e51235: f64 = (assign29990_e51233 * locals.var_czbs);
        let (assign29990_e51249, assign29990_e51249_d_n3, assign29990_e51249_d_n4, assign29990_e51249_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29990_e51241: f64 = (-87.498233534);
                (assign29990_e51241, 0.0, 0.0, 0.0,)
            } else {
                let (assign29990_e51248, assign29990_e51248_d_n3, assign29990_e51248_d_n4, assign29990_e51248_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29990_e51246: f64 = (locals.var_arg).ln();
                        (assign29990_e51246, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29990_e51248, assign29990_e51248_d_n3, assign29990_e51248_d_n4, assign29990_e51248_d_n6,)
            }
        };
        let assign29990_e51250: f64 = (assign29990_e51235 * assign29990_e51249);
        let assign29990_e51251: f64 = (locals.var_qec - assign29990_e51250);
        (assign29990_e51251, (locals.var_qec_dn3 - (assign29990_e51235 * assign29990_e51249_d_n3)), (locals.var_qec_dn4 - (((((p.p1602 * locals.var_pb21s_dn4) * locals.var_czbs) + (assign29990_e51233 * locals.var_czbs_dn4)) * assign29990_e51249) + (assign29990_e51235 * assign29990_e51249_d_n4))), (locals.var_qec_dn6 - (assign29990_e51235 * assign29990_e51249_d_n6)),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29990_e51253;
        locals.var_qesj1_dn3 = assign29990_e51253_d_n3;
        locals.var_qesj1_dn4 = assign29990_e51253_d_n4;
        locals.var_qesj1_dn6 = assign29990_e51253_d_n6;

        let (assign30000_e51266, assign30000_e51266_d_n3, assign30000_e51266_d_n4, assign30000_e51266_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) {
        let assign30000_e51264: f64 = (1.0 - locals.var_t1__blk488);
        (assign30000_e51264, (-locals.var_t1__blk488_dn3), (-locals.var_t1__blk488_dn4), (-locals.var_t1__blk488_dn6),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign30000_e51266;
        locals.var_arg_dn3 = assign30000_e51266_d_n3;
        locals.var_arg_dn4 = assign30000_e51266_d_n4;
        locals.var_arg_dn6 = assign30000_e51266_d_n6;

        let assign30010_e51269: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign30010_e51269;

        let assign30020_e51272: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard507 = assign30020_e51272;

        let (assign30030_e51290, assign30030_e51290_d_n3, assign30030_e51290_d_n4, assign30030_e51290_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) && (locals.var_guard507 != 0.0)) {
        let assign30030_e51287: f64 = (locals.var_arg).sqrt();
        let assign30030_e51288: f64 = (1.0 / assign30030_e51287);
        (assign30030_e51288, (-((locals.var_arg_dn3 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))), (-((locals.var_arg_dn4 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))), (-((locals.var_arg_dn6 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign30030_e51290;
        locals.var_sarg_dn3 = assign30030_e51290_d_n3;
        locals.var_sarg_dn4 = assign30030_e51290_d_n4;
        locals.var_sarg_dn6 = assign30030_e51290_d_n6;

        let (assign30040_e51309, assign30040_e51309_d_n3, assign30040_e51309_d_n4, assign30040_e51309_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) && (locals.var_guard507 == 0.0)) {
        let assign30040_e51306: f64 = (-p.p1596);
        let assign30040_e51307: f64 = (locals.var_arg).powf(assign30040_e51306);
        (assign30040_e51307, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn3)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn4)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn6)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign30040_e51309;
        locals.var_sarg_dn3 = assign30040_e51309_d_n3;
        locals.var_sarg_dn4 = assign30040_e51309_d_n4;
        locals.var_sarg_dn6 = assign30040_e51309_d_n6;

        let (assign30050_e51334, assign30050_e51334_d_n3, assign30050_e51334_d_n4, assign30050_e51334_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) {
        let assign30050_e51322: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign30050_e51326: f64 = (locals.var_arg * locals.var_sarg);
        let assign30050_e51327: f64 = (1.0 - assign30050_e51326);
        let assign30050_e51328: f64 = (assign30050_e51322 * assign30050_e51327);
        let assign30050_e51331: f64 = (1.0 - p.p1596);
        let assign30050_e51332: f64 = (assign30050_e51328 / assign30050_e51331);
        (assign30050_e51332, ((assign30050_e51322 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign30050_e51331), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign30050_e51327) + (assign30050_e51322 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign30050_e51331), ((assign30050_e51322 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign30050_e51331),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30050_e51334;
        locals.var_qesj1_dn3 = assign30050_e51334_d_n3;
        locals.var_qesj1_dn4 = assign30050_e51334_d_n4;
        locals.var_qesj1_dn6 = assign30050_e51334_d_n6;

        let (assign30060_e51366, assign30060_e51366_d_n3, assign30060_e51366_d_n4, assign30060_e51366_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 == 0.0)) {
        let assign30060_e51347: f64 = (-locals.var_pbs_t);
        let assign30060_e51349: f64 = (assign30060_e51347 * locals.var_czbs);
        let (assign30060_e51363, assign30060_e51363_d_n3, assign30060_e51363_d_n4, assign30060_e51363_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign30060_e51355: f64 = (-87.498233534);
                (assign30060_e51355, 0.0, 0.0, 0.0,)
            } else {
                let (assign30060_e51362, assign30060_e51362_d_n3, assign30060_e51362_d_n4, assign30060_e51362_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign30060_e51360: f64 = (locals.var_arg).ln();
                        (assign30060_e51360, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30060_e51362, assign30060_e51362_d_n3, assign30060_e51362_d_n4, assign30060_e51362_d_n6,)
            }
        };
        let assign30060_e51364: f64 = (assign30060_e51349 * assign30060_e51363);
        (assign30060_e51364, (assign30060_e51349 * assign30060_e51363_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign30060_e51347 * locals.var_czbs_dn4)) * assign30060_e51363) + (assign30060_e51349 * assign30060_e51363_d_n4)), (assign30060_e51349 * assign30060_e51363_d_n6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30060_e51366;
        locals.var_qesj1_dn3 = assign30060_e51366_d_n3;
        locals.var_qesj1_dn4 = assign30060_e51366_d_n4;
        locals.var_qesj1_dn6 = assign30060_e51366_d_n6;

        let assign30070_e51369: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign30070_e51369;

        let assign30080_e51372: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard509 = assign30080_e51372;

        let (assign30090_e51388,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) && (locals.var_guard509 != 0.0)) {
        let assign30090_e51385: f64 = (0.1_f64).sqrt();
        let assign30090_e51386: f64 = (1.0 / assign30090_e51385);
        (assign30090_e51386,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30090_e51388;

        let (assign30100_e51405,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) && (locals.var_guard509 == 0.0)) {
        let assign30100_e51402: f64 = (-p.p1596);
        let assign30100_e51403: f64 = (0.1_f64).powf(assign30100_e51402);
        (assign30100_e51403,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30100_e51405;

        let (assign30110_e51420,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) {
        let assign30110_e51417: f64 = (1.0 - p.p1596);
        let assign30110_e51418: f64 = (1.0 / assign30110_e51417);
        (assign30110_e51418,)
    } else {
        (locals.var_t3__blk490,)
    }
};
        locals.var_t3__blk490 = assign30110_e51420;

        let (assign30120_e51443,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) {
        let assign30120_e51433: f64 = (0.05 * p.p1596);
        let assign30120_e51436: f64 = (1.0 + p.p1596);
        let assign30120_e51437: f64 = (assign30120_e51433 * assign30120_e51436);
        let assign30120_e51439: f64 = (assign30120_e51437 * locals.var_t2__blk489);
        let assign30120_e51440: f64 = (1.0 - assign30120_e51439);
        let assign30120_e51441: f64 = (locals.var_t3__blk490 * assign30120_e51440);
        (assign30120_e51441,)
    } else {
        (locals.var_t5__blk492,)
    }
};
        locals.var_t5__blk492 = assign30120_e51443;

        let (assign30130_e51455,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30130_e51455;

        let (assign30140_e51470,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 == 0.0)) {
        let assign30140_e51467: f64 = (0.1_f64).ln();
        let assign30140_e51468: f64 = (1.5 - assign30140_e51467);
        (assign30140_e51468,)
    } else {
        (locals.var_t5__blk492,)
    }
};
        locals.var_t5__blk492 = assign30140_e51470;

        let (assign30150_e51495, assign30150_e51495_d_n3, assign30150_e51495_d_n4, assign30150_e51495_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign30150_e51480: f64 = (locals.var_t1__blk488 - 1.0);
        let assign30150_e51481: f64 = (locals.var_t2__blk489 * assign30150_e51480);
        let assign30150_e51484: f64 = (5.0 * p.p1596);
        let assign30150_e51487: f64 = (locals.var_t1__blk488 - 1.0);
        let assign30150_e51488: f64 = (assign30150_e51484 * assign30150_e51487);
        let assign30150_e51491: f64 = (1.0 + p.p1596);
        let assign30150_e51492: f64 = (assign30150_e51488 + assign30150_e51491);
        let assign30150_e51493: f64 = (assign30150_e51481 * assign30150_e51492);
        (assign30150_e51493, (((locals.var_t2__blk489 * locals.var_t1__blk488_dn3) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn3))), (((locals.var_t2__blk489 * locals.var_t1__blk488_dn4) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn4))), (((locals.var_t2__blk489 * locals.var_t1__blk488_dn6) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn6))),)
    } else {
        (locals.var_t4__blk491, locals.var_t4__blk491_dn3, locals.var_t4__blk491_dn4, locals.var_t4__blk491_dn6,)
    }
};
        locals.var_t4__blk491 = assign30150_e51495;
        locals.var_t4__blk491_dn3 = assign30150_e51495_d_n3;
        locals.var_t4__blk491_dn4 = assign30150_e51495_d_n4;
        locals.var_t4__blk491_dn6 = assign30150_e51495_d_n6;

        let (assign30160_e51510, assign30160_e51510_d_n3, assign30160_e51510_d_n4, assign30160_e51510_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign30160_e51504: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign30160_e51507: f64 = (locals.var_t4__blk491 + locals.var_t5__blk492);
        let assign30160_e51508: f64 = (assign30160_e51504 * assign30160_e51507);
        (assign30160_e51508, (assign30160_e51504 * locals.var_t4__blk491_dn3), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign30160_e51507) + (assign30160_e51504 * locals.var_t4__blk491_dn4)), (assign30160_e51504 * locals.var_t4__blk491_dn6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30160_e51510;
        locals.var_qesj1_dn3 = assign30160_e51510_d_n3;
        locals.var_qesj1_dn4 = assign30160_e51510_d_n4;
        locals.var_qesj1_dn6 = assign30160_e51510_d_n6;

        let (assign30170_e51517, assign30170_e51517_d_n3, assign30170_e51517_d_n4, assign30170_e51517_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard496 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30170_e51517;
        locals.var_qesj1_dn3 = assign30170_e51517_d_n3;
        locals.var_qesj1_dn4 = assign30170_e51517_d_n4;
        locals.var_qesj1_dn6 = assign30170_e51517_d_n6;

        let assign30180_e51520: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard518 = assign30180_e51520;

        let (assign30190_e51528, assign30190_e51528_d_n3, assign30190_e51528_d_n4, assign30190_e51528_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) {
        let assign30190_e51526: f64 = (locals.var_ves_jct / locals.var_pbsws_t);
        (assign30190_e51526, (locals.var_ves_jct_dn3 / locals.var_pbsws_t), (-((locals.var_ves_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (locals.var_ves_jct_dn6 / locals.var_pbsws_t),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn3, locals.var_t1__blk510_dn4, locals.var_t1__blk510_dn6,)
    }
};
        locals.var_t1__blk510 = assign30190_e51528;
        locals.var_t1__blk510_dn3 = assign30190_e51528_d_n3;
        locals.var_t1__blk510_dn4 = assign30190_e51528_d_n4;
        locals.var_t1__blk510_dn6 = assign30190_e51528_d_n6;

        let assign30200_e51531: f64 = if locals.var_t1__blk510 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard519 = assign30200_e51531;

        let assign30210_e51534: f64 = if p.p1604 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard520 = assign30210_e51534;

        let assign30220_e51537: f64 = if locals.var_ves_jct > locals.var_vec2s { 1.0 } else { 0.0 };
        locals.var_guard521 = assign30220_e51537;

        let (assign30230_e51551, assign30230_e51551_d_n3, assign30230_e51551_d_n4, assign30230_e51551_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign30230_e51549: f64 = (1.0 - locals.var_t1__blk510);
        (assign30230_e51549, (-locals.var_t1__blk510_dn3), (-locals.var_t1__blk510_dn4), (-locals.var_t1__blk510_dn6),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30230_e51551;
        locals.var_arg__blk515_dn3 = assign30230_e51551_d_n3;
        locals.var_arg__blk515_dn4 = assign30230_e51551_d_n4;
        locals.var_arg__blk515_dn6 = assign30230_e51551_d_n6;

        let assign30240_e51554: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard522 = assign30240_e51554;

        let assign30250_e51557: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign30250_e51557;

        let (assign30260_e51576, assign30260_e51576_d_n3, assign30260_e51576_d_n4, assign30260_e51576_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign30260_e51573: f64 = (locals.var_arg__blk515).sqrt();
        let assign30260_e51574: f64 = (1.0 / assign30260_e51573);
        (assign30260_e51574, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30260_e51576;
        locals.var_sarg__blk516_dn3 = assign30260_e51576_d_n3;
        locals.var_sarg__blk516_dn4 = assign30260_e51576_d_n4;
        locals.var_sarg__blk516_dn6 = assign30260_e51576_d_n6;

        let (assign30270_e51596, assign30270_e51596_d_n3, assign30270_e51596_d_n4, assign30270_e51596_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) && (locals.var_guard523 == 0.0)) {
        let assign30270_e51593: f64 = (-p.p1598);
        let assign30270_e51594: f64 = (locals.var_arg__blk515).powf(assign30270_e51593);
        (assign30270_e51594, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30270_e51596;
        locals.var_sarg__blk516_dn3 = assign30270_e51596_d_n3;
        locals.var_sarg__blk516_dn4 = assign30270_e51596_d_n4;
        locals.var_sarg__blk516_dn6 = assign30270_e51596_d_n6;

        let (assign30280_e51622, assign30280_e51622_d_n3, assign30280_e51622_d_n4, assign30280_e51622_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) {
        let assign30280_e51610: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30280_e51614: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30280_e51615: f64 = (1.0 - assign30280_e51614);
        let assign30280_e51616: f64 = (assign30280_e51610 * assign30280_e51615);
        let assign30280_e51619: f64 = (1.0 - p.p1598);
        let assign30280_e51620: f64 = (assign30280_e51616 / assign30280_e51619);
        (assign30280_e51620, ((assign30280_e51610 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30280_e51619), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30280_e51615) + (assign30280_e51610 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30280_e51619), ((assign30280_e51610 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30280_e51619),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30280_e51622;
        locals.var_qesj2_dn3 = assign30280_e51622_d_n3;
        locals.var_qesj2_dn4 = assign30280_e51622_d_n4;
        locals.var_qesj2_dn6 = assign30280_e51622_d_n6;

        let (assign30290_e51655, assign30290_e51655_d_n3, assign30290_e51655_d_n4, assign30290_e51655_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 == 0.0)) {
        let assign30290_e51636: f64 = (-locals.var_pbsws_t);
        let assign30290_e51638: f64 = (assign30290_e51636 * locals.var_czbssw);
        let (assign30290_e51652, assign30290_e51652_d_n3, assign30290_e51652_d_n4, assign30290_e51652_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30290_e51644: f64 = (-87.498233534);
                (assign30290_e51644, 0.0, 0.0, 0.0,)
            } else {
                let (assign30290_e51651, assign30290_e51651_d_n3, assign30290_e51651_d_n4, assign30290_e51651_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30290_e51649: f64 = (locals.var_arg__blk515).ln();
                        (assign30290_e51649, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30290_e51651, assign30290_e51651_d_n3, assign30290_e51651_d_n4, assign30290_e51651_d_n6,)
            }
        };
        let assign30290_e51653: f64 = (assign30290_e51638 * assign30290_e51652);
        (assign30290_e51653, (assign30290_e51638 * assign30290_e51652_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30290_e51636 * locals.var_czbssw_dn4)) * assign30290_e51652) + (assign30290_e51638 * assign30290_e51652_d_n4)), (assign30290_e51638 * assign30290_e51652_d_n6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30290_e51655;
        locals.var_qesj2_dn3 = assign30290_e51655_d_n3;
        locals.var_qesj2_dn4 = assign30290_e51655_d_n4;
        locals.var_qesj2_dn6 = assign30290_e51655_d_n6;

        let (assign30300_e51672, assign30300_e51672_d_n3, assign30300_e51672_d_n4, assign30300_e51672_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) {
        let assign30300_e51669: f64 = (locals.var_vec2s / locals.var_pbsws_t);
        let assign30300_e51670: f64 = (1.0 - assign30300_e51669);
        (assign30300_e51670, 0.0, (-(((locals.var_vec2s_dn4 * locals.var_pbsws_t) - (locals.var_vec2s * locals.var_pbsws_t_dn4)) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0,)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30300_e51672;
        locals.var_arg__blk515_dn3 = assign30300_e51672_d_n3;
        locals.var_arg__blk515_dn4 = assign30300_e51672_d_n4;
        locals.var_arg__blk515_dn6 = assign30300_e51672_d_n6;

        let assign30310_e51675: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign30310_e51675;

        let assign30320_e51678: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign30320_e51678;

        let (assign30330_e51698, assign30330_e51698_d_n3, assign30330_e51698_d_n4, assign30330_e51698_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 != 0.0)) {
        let assign30330_e51695: f64 = (locals.var_arg__blk515).sqrt();
        let assign30330_e51696: f64 = (1.0 / assign30330_e51695);
        (assign30330_e51696, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30330_e51698;
        locals.var_sarg__blk516_dn3 = assign30330_e51698_d_n3;
        locals.var_sarg__blk516_dn4 = assign30330_e51698_d_n4;
        locals.var_sarg__blk516_dn6 = assign30330_e51698_d_n6;

        let (assign30340_e51719, assign30340_e51719_d_n3, assign30340_e51719_d_n4, assign30340_e51719_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 == 0.0)) {
        let assign30340_e51716: f64 = (-p.p1598);
        let assign30340_e51717: f64 = (locals.var_arg__blk515).powf(assign30340_e51716);
        (assign30340_e51717, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30340_e51719;
        locals.var_sarg__blk516_dn3 = assign30340_e51719_d_n3;
        locals.var_sarg__blk516_dn4 = assign30340_e51719_d_n4;
        locals.var_sarg__blk516_dn6 = assign30340_e51719_d_n6;

        let (assign30350_e51746, assign30350_e51746_d_n3, assign30350_e51746_d_n4, assign30350_e51746_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) {
        let assign30350_e51734: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30350_e51738: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30350_e51739: f64 = (1.0 - assign30350_e51738);
        let assign30350_e51740: f64 = (assign30350_e51734 * assign30350_e51739);
        let assign30350_e51743: f64 = (1.0 - p.p1598);
        let assign30350_e51744: f64 = (assign30350_e51740 / assign30350_e51743);
        (assign30350_e51744, ((assign30350_e51734 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30350_e51743), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30350_e51739) + (assign30350_e51734 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30350_e51743), ((assign30350_e51734 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30350_e51743),)
    } else {
        (locals.var_qec__blk517, locals.var_qec__blk517_dn3, locals.var_qec__blk517_dn4, locals.var_qec__blk517_dn6,)
    }
};
        locals.var_qec__blk517 = assign30350_e51746;
        locals.var_qec__blk517_dn3 = assign30350_e51746_d_n3;
        locals.var_qec__blk517_dn4 = assign30350_e51746_d_n4;
        locals.var_qec__blk517_dn6 = assign30350_e51746_d_n6;

        let (assign30360_e51780, assign30360_e51780_d_n3, assign30360_e51780_d_n4, assign30360_e51780_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign30360_e51761: f64 = (-locals.var_pbsws_t);
        let assign30360_e51763: f64 = (assign30360_e51761 * locals.var_czbssw);
        let (assign30360_e51777, assign30360_e51777_d_n3, assign30360_e51777_d_n4, assign30360_e51777_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30360_e51769: f64 = (-87.498233534);
                (assign30360_e51769, 0.0, 0.0, 0.0,)
            } else {
                let (assign30360_e51776, assign30360_e51776_d_n3, assign30360_e51776_d_n4, assign30360_e51776_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30360_e51774: f64 = (locals.var_arg__blk515).ln();
                        (assign30360_e51774, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30360_e51776, assign30360_e51776_d_n3, assign30360_e51776_d_n4, assign30360_e51776_d_n6,)
            }
        };
        let assign30360_e51778: f64 = (assign30360_e51763 * assign30360_e51777);
        (assign30360_e51778, (assign30360_e51763 * assign30360_e51777_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30360_e51761 * locals.var_czbssw_dn4)) * assign30360_e51777) + (assign30360_e51763 * assign30360_e51777_d_n4)), (assign30360_e51763 * assign30360_e51777_d_n6),)
    } else {
        (locals.var_qec__blk517, locals.var_qec__blk517_dn3, locals.var_qec__blk517_dn4, locals.var_qec__blk517_dn6,)
    }
};
        locals.var_qec__blk517 = assign30360_e51780;
        locals.var_qec__blk517_dn3 = assign30360_e51780_d_n3;
        locals.var_qec__blk517_dn4 = assign30360_e51780_d_n4;
        locals.var_qec__blk517_dn6 = assign30360_e51780_d_n6;

    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30370_e51799, assign30370_e51799_d_n3, assign30370_e51799_d_n4, assign30370_e51799_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) {
        let assign30370_e51794: f64 = (locals.var_ves_jct - locals.var_vec2s);
        let assign30370_e51796: f64 = (assign30370_e51794 / locals.var_pb22s);
        let assign30370_e51797: f64 = (1.0 - assign30370_e51796);
        (assign30370_e51797, (-(locals.var_ves_jct_dn3 / locals.var_pb22s)), (-((((-locals.var_vec2s_dn4) * locals.var_pb22s) - (assign30370_e51794 * locals.var_pb22s_dn4)) / (locals.var_pb22s * locals.var_pb22s))), (-(locals.var_ves_jct_dn6 / locals.var_pb22s)),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30370_e51799;
        locals.var_arg__blk515_dn3 = assign30370_e51799_d_n3;
        locals.var_arg__blk515_dn4 = assign30370_e51799_d_n4;
        locals.var_arg__blk515_dn6 = assign30370_e51799_d_n6;

        let assign30380_e51802: f64 = if p.p1610 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign30380_e51802;

        let assign30390_e51805: f64 = if p.p1610 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign30390_e51805;

        let (assign30400_e51825, assign30400_e51825_d_n3, assign30400_e51825_d_n4, assign30400_e51825_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
        let assign30400_e51822: f64 = (locals.var_arg__blk515).sqrt();
        let assign30400_e51823: f64 = (1.0 / assign30400_e51822);
        (assign30400_e51823, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30400_e51825;
        locals.var_sarg__blk516_dn3 = assign30400_e51825_d_n3;
        locals.var_sarg__blk516_dn4 = assign30400_e51825_d_n4;
        locals.var_sarg__blk516_dn6 = assign30400_e51825_d_n6;

        let (assign30410_e51846, assign30410_e51846_d_n3, assign30410_e51846_d_n4, assign30410_e51846_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 == 0.0)) {
        let assign30410_e51843: f64 = (-p.p1610);
        let assign30410_e51844: f64 = (locals.var_arg__blk515).powf(assign30410_e51843);
        (assign30410_e51844, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30410_e51846;
        locals.var_sarg__blk516_dn3 = assign30410_e51846_d_n3;
        locals.var_sarg__blk516_dn4 = assign30410_e51846_d_n4;
        locals.var_sarg__blk516_dn6 = assign30410_e51846_d_n6;

        let (assign30420_e51877, assign30420_e51877_d_n3, assign30420_e51877_d_n4, assign30420_e51877_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign30420_e51862: f64 = (p.p1604 * locals.var_pb22s);
        let assign30420_e51864: f64 = (assign30420_e51862 * locals.var_czbssw);
        let assign30420_e51868: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30420_e51869: f64 = (1.0 - assign30420_e51868);
        let assign30420_e51870: f64 = (assign30420_e51864 * assign30420_e51869);
        let assign30420_e51873: f64 = (1.0 - p.p1610);
        let assign30420_e51874: f64 = (assign30420_e51870 / assign30420_e51873);
        let assign30420_e51875: f64 = (locals.var_qec__blk517 + assign30420_e51874);
        (assign30420_e51875, (locals.var_qec__blk517_dn3 + ((assign30420_e51864 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30420_e51873)), (locals.var_qec__blk517_dn4 + ((((((p.p1604 * locals.var_pb22s_dn4) * locals.var_czbssw) + (assign30420_e51862 * locals.var_czbssw_dn4)) * assign30420_e51869) + (assign30420_e51864 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30420_e51873)), (locals.var_qec__blk517_dn6 + ((assign30420_e51864 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30420_e51873)),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30420_e51877;
        locals.var_qesj2_dn3 = assign30420_e51877_d_n3;
        locals.var_qesj2_dn4 = assign30420_e51877_d_n4;
        locals.var_qesj2_dn6 = assign30420_e51877_d_n6;

        let (assign30430_e51914, assign30430_e51914_d_n3, assign30430_e51914_d_n4, assign30430_e51914_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign30430_e51894: f64 = (p.p1604 * locals.var_pb22s);
        let assign30430_e51896: f64 = (assign30430_e51894 * locals.var_czbssw);
        let (assign30430_e51910, assign30430_e51910_d_n3, assign30430_e51910_d_n4, assign30430_e51910_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30430_e51902: f64 = (-87.498233534);
                (assign30430_e51902, 0.0, 0.0, 0.0,)
            } else {
                let (assign30430_e51909, assign30430_e51909_d_n3, assign30430_e51909_d_n4, assign30430_e51909_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30430_e51907: f64 = (locals.var_arg__blk515).ln();
                        (assign30430_e51907, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30430_e51909, assign30430_e51909_d_n3, assign30430_e51909_d_n4, assign30430_e51909_d_n6,)
            }
        };
        let assign30430_e51911: f64 = (assign30430_e51896 * assign30430_e51910);
        let assign30430_e51912: f64 = (locals.var_qec__blk517 - assign30430_e51911);
        (assign30430_e51912, (locals.var_qec__blk517_dn3 - (assign30430_e51896 * assign30430_e51910_d_n3)), (locals.var_qec__blk517_dn4 - (((((p.p1604 * locals.var_pb22s_dn4) * locals.var_czbssw) + (assign30430_e51894 * locals.var_czbssw_dn4)) * assign30430_e51910) + (assign30430_e51896 * assign30430_e51910_d_n4))), (locals.var_qec__blk517_dn6 - (assign30430_e51896 * assign30430_e51910_d_n6)),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30430_e51914;
        locals.var_qesj2_dn3 = assign30430_e51914_d_n3;
        locals.var_qesj2_dn4 = assign30430_e51914_d_n4;
        locals.var_qesj2_dn6 = assign30430_e51914_d_n6;

        let (assign30440_e51927, assign30440_e51927_d_n3, assign30440_e51927_d_n4, assign30440_e51927_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign30440_e51925: f64 = (1.0 - locals.var_t1__blk510);
        (assign30440_e51925, (-locals.var_t1__blk510_dn3), (-locals.var_t1__blk510_dn4), (-locals.var_t1__blk510_dn6),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30440_e51927;
        locals.var_arg__blk515_dn3 = assign30440_e51927_d_n3;
        locals.var_arg__blk515_dn4 = assign30440_e51927_d_n4;
        locals.var_arg__blk515_dn6 = assign30440_e51927_d_n6;

        let assign30450_e51930: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard528 = assign30450_e51930;

        let assign30460_e51933: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign30460_e51933;

        let (assign30470_e51951, assign30470_e51951_d_n3, assign30470_e51951_d_n4, assign30470_e51951_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) {
        let assign30470_e51948: f64 = (locals.var_arg__blk515).sqrt();
        let assign30470_e51949: f64 = (1.0 / assign30470_e51948);
        (assign30470_e51949, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30470_e51951;
        locals.var_sarg__blk516_dn3 = assign30470_e51951_d_n3;
        locals.var_sarg__blk516_dn4 = assign30470_e51951_d_n4;
        locals.var_sarg__blk516_dn6 = assign30470_e51951_d_n6;

        let (assign30480_e51970, assign30480_e51970_d_n3, assign30480_e51970_d_n4, assign30480_e51970_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 == 0.0)) {
        let assign30480_e51967: f64 = (-p.p1598);
        let assign30480_e51968: f64 = (locals.var_arg__blk515).powf(assign30480_e51967);
        (assign30480_e51968, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30480_e51970;
        locals.var_sarg__blk516_dn3 = assign30480_e51970_d_n3;
        locals.var_sarg__blk516_dn4 = assign30480_e51970_d_n4;
        locals.var_sarg__blk516_dn6 = assign30480_e51970_d_n6;

        let (assign30490_e51995, assign30490_e51995_d_n3, assign30490_e51995_d_n4, assign30490_e51995_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) {
        let assign30490_e51983: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30490_e51987: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30490_e51988: f64 = (1.0 - assign30490_e51987);
        let assign30490_e51989: f64 = (assign30490_e51983 * assign30490_e51988);
        let assign30490_e51992: f64 = (1.0 - p.p1598);
        let assign30490_e51993: f64 = (assign30490_e51989 / assign30490_e51992);
        (assign30490_e51993, ((assign30490_e51983 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30490_e51992), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30490_e51988) + (assign30490_e51983 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30490_e51992), ((assign30490_e51983 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30490_e51992),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30490_e51995;
        locals.var_qesj2_dn3 = assign30490_e51995_d_n3;
        locals.var_qesj2_dn4 = assign30490_e51995_d_n4;
        locals.var_qesj2_dn6 = assign30490_e51995_d_n6;

        let (assign30500_e52027, assign30500_e52027_d_n3, assign30500_e52027_d_n4, assign30500_e52027_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign30500_e52008: f64 = (-locals.var_pbsws_t);
        let assign30500_e52010: f64 = (assign30500_e52008 * locals.var_czbssw);
        let (assign30500_e52024, assign30500_e52024_d_n3, assign30500_e52024_d_n4, assign30500_e52024_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30500_e52016: f64 = (-87.498233534);
                (assign30500_e52016, 0.0, 0.0, 0.0,)
            } else {
                let (assign30500_e52023, assign30500_e52023_d_n3, assign30500_e52023_d_n4, assign30500_e52023_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30500_e52021: f64 = (locals.var_arg__blk515).ln();
                        (assign30500_e52021, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30500_e52023, assign30500_e52023_d_n3, assign30500_e52023_d_n4, assign30500_e52023_d_n6,)
            }
        };
        let assign30500_e52025: f64 = (assign30500_e52010 * assign30500_e52024);
        (assign30500_e52025, (assign30500_e52010 * assign30500_e52024_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30500_e52008 * locals.var_czbssw_dn4)) * assign30500_e52024) + (assign30500_e52010 * assign30500_e52024_d_n4)), (assign30500_e52010 * assign30500_e52024_d_n6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30500_e52027;
        locals.var_qesj2_dn3 = assign30500_e52027_d_n3;
        locals.var_qesj2_dn4 = assign30500_e52027_d_n4;
        locals.var_qesj2_dn6 = assign30500_e52027_d_n6;

        let assign30510_e52030: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign30510_e52030;

        let assign30520_e52033: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign30520_e52033;

        let (assign30530_e52049,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign30530_e52046: f64 = (0.1_f64).sqrt();
        let assign30530_e52047: f64 = (1.0 / assign30530_e52046);
        (assign30530_e52047,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30530_e52049;

        let (assign30540_e52066,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 == 0.0)) {
        let assign30540_e52063: f64 = (-p.p1598);
        let assign30540_e52064: f64 = (0.1_f64).powf(assign30540_e52063);
        (assign30540_e52064,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30540_e52066;

        let (assign30550_e52081,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign30550_e52078: f64 = (1.0 - p.p1598);
        let assign30550_e52079: f64 = (1.0 / assign30550_e52078);
        (assign30550_e52079,)
    } else {
        (locals.var_t3__blk512,)
    }
};
        locals.var_t3__blk512 = assign30550_e52081;

        let (assign30560_e52104,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign30560_e52094: f64 = (0.05 * p.p1598);
        let assign30560_e52097: f64 = (1.0 + p.p1598);
        let assign30560_e52098: f64 = (assign30560_e52094 * assign30560_e52097);
        let assign30560_e52100: f64 = (assign30560_e52098 * locals.var_t2__blk511);
        let assign30560_e52101: f64 = (1.0 - assign30560_e52100);
        let assign30560_e52102: f64 = (locals.var_t3__blk512 * assign30560_e52101);
        (assign30560_e52102,)
    } else {
        (locals.var_t5__blk514,)
    }
};
        locals.var_t5__blk514 = assign30560_e52104;

        let (assign30570_e52116,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30570_e52116;

        let (assign30580_e52131,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 == 0.0)) {
        let assign30580_e52128: f64 = (0.1_f64).ln();
        let assign30580_e52129: f64 = (1.5 - assign30580_e52128);
        (assign30580_e52129,)
    } else {
        (locals.var_t5__blk514,)
    }
};
        locals.var_t5__blk514 = assign30580_e52131;

        let (assign30590_e52156, assign30590_e52156_d_n3, assign30590_e52156_d_n4, assign30590_e52156_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) {
        let assign30590_e52141: f64 = (locals.var_t1__blk510 - 1.0);
        let assign30590_e52142: f64 = (locals.var_t2__blk511 * assign30590_e52141);
        let assign30590_e52145: f64 = (5.0 * p.p1598);
        let assign30590_e52148: f64 = (locals.var_t1__blk510 - 1.0);
        let assign30590_e52149: f64 = (assign30590_e52145 * assign30590_e52148);
        let assign30590_e52152: f64 = (1.0 + p.p1598);
        let assign30590_e52153: f64 = (assign30590_e52149 + assign30590_e52152);
        let assign30590_e52154: f64 = (assign30590_e52142 * assign30590_e52153);
        (assign30590_e52154, (((locals.var_t2__blk511 * locals.var_t1__blk510_dn3) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn3))), (((locals.var_t2__blk511 * locals.var_t1__blk510_dn4) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn4))), (((locals.var_t2__blk511 * locals.var_t1__blk510_dn6) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn6))),)
    } else {
        (locals.var_t4__blk513, locals.var_t4__blk513_dn3, locals.var_t4__blk513_dn4, locals.var_t4__blk513_dn6,)
    }
};
        locals.var_t4__blk513 = assign30590_e52156;
        locals.var_t4__blk513_dn3 = assign30590_e52156_d_n3;
        locals.var_t4__blk513_dn4 = assign30590_e52156_d_n4;
        locals.var_t4__blk513_dn6 = assign30590_e52156_d_n6;

        let (assign30600_e52171, assign30600_e52171_d_n3, assign30600_e52171_d_n4, assign30600_e52171_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) {
        let assign30600_e52165: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30600_e52168: f64 = (locals.var_t4__blk513 + locals.var_t5__blk514);
        let assign30600_e52169: f64 = (assign30600_e52165 * assign30600_e52168);
        (assign30600_e52169, (assign30600_e52165 * locals.var_t4__blk513_dn3), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30600_e52168) + (assign30600_e52165 * locals.var_t4__blk513_dn4)), (assign30600_e52165 * locals.var_t4__blk513_dn6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30600_e52171;
        locals.var_qesj2_dn3 = assign30600_e52171_d_n3;
        locals.var_qesj2_dn4 = assign30600_e52171_d_n4;
        locals.var_qesj2_dn6 = assign30600_e52171_d_n6;

        let (assign30610_e52178, assign30610_e52178_d_n3, assign30610_e52178_d_n4, assign30610_e52178_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard518 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30610_e52178;
        locals.var_qesj2_dn3 = assign30610_e52178_d_n3;
        locals.var_qesj2_dn4 = assign30610_e52178_d_n4;
        locals.var_qesj2_dn6 = assign30610_e52178_d_n6;

        let assign30620_e52181: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign30620_e52181;

        let (assign30630_e52189, assign30630_e52189_d_n3, assign30630_e52189_d_n4, assign30630_e52189_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) {
        let assign30630_e52187: f64 = (locals.var_ves_jct / locals.var_pbswgs_t);
        (assign30630_e52187, (locals.var_ves_jct_dn3 / locals.var_pbswgs_t), (-((locals.var_ves_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (locals.var_ves_jct_dn6 / locals.var_pbswgs_t),)
    } else {
        (locals.var_t1__blk532, locals.var_t1__blk532_dn3, locals.var_t1__blk532_dn4, locals.var_t1__blk532_dn6,)
    }
};
        locals.var_t1__blk532 = assign30630_e52189;
        locals.var_t1__blk532_dn3 = assign30630_e52189_d_n3;
        locals.var_t1__blk532_dn4 = assign30630_e52189_d_n4;
        locals.var_t1__blk532_dn6 = assign30630_e52189_d_n6;

        let assign30640_e52192: f64 = if locals.var_t1__blk532 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign30640_e52192;

        let assign30650_e52195: f64 = if p.p1606 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign30650_e52195;

        let assign30660_e52198: f64 = if locals.var_ves_jct > locals.var_vec3s { 1.0 } else { 0.0 };
        locals.var_guard543 = assign30660_e52198;

        let (assign30670_e52212, assign30670_e52212_d_n3, assign30670_e52212_d_n4, assign30670_e52212_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) {
        let assign30670_e52210: f64 = (1.0 - locals.var_t1__blk532);
        (assign30670_e52210, (-locals.var_t1__blk532_dn3), (-locals.var_t1__blk532_dn4), (-locals.var_t1__blk532_dn6),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30670_e52212;
        locals.var_arg__blk537_dn3 = assign30670_e52212_d_n3;
        locals.var_arg__blk537_dn4 = assign30670_e52212_d_n4;
        locals.var_arg__blk537_dn6 = assign30670_e52212_d_n6;

        let assign30680_e52215: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign30680_e52215;

        let assign30690_e52218: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign30690_e52218;

        let (assign30700_e52237, assign30700_e52237_d_n3, assign30700_e52237_d_n4, assign30700_e52237_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign30700_e52234: f64 = (locals.var_arg__blk537).sqrt();
        let assign30700_e52235: f64 = (1.0 / assign30700_e52234);
        (assign30700_e52235, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30700_e52237;
        locals.var_sarg__blk538_dn3 = assign30700_e52237_d_n3;
        locals.var_sarg__blk538_dn4 = assign30700_e52237_d_n4;
        locals.var_sarg__blk538_dn6 = assign30700_e52237_d_n6;

        let (assign30710_e52257, assign30710_e52257_d_n3, assign30710_e52257_d_n4, assign30710_e52257_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign30710_e52254: f64 = (-p.p1600);
        let assign30710_e52255: f64 = (locals.var_arg__blk537).powf(assign30710_e52254);
        (assign30710_e52255, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30710_e52257;
        locals.var_sarg__blk538_dn3 = assign30710_e52257_d_n3;
        locals.var_sarg__blk538_dn4 = assign30710_e52257_d_n4;
        locals.var_sarg__blk538_dn6 = assign30710_e52257_d_n6;

        let (assign30720_e52283, assign30720_e52283_d_n3, assign30720_e52283_d_n4, assign30720_e52283_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) {
        let assign30720_e52271: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30720_e52275: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30720_e52276: f64 = (1.0 - assign30720_e52275);
        let assign30720_e52277: f64 = (assign30720_e52271 * assign30720_e52276);
        let assign30720_e52280: f64 = (1.0 - p.p1600);
        let assign30720_e52281: f64 = (assign30720_e52277 / assign30720_e52280);
        (assign30720_e52281, ((assign30720_e52271 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30720_e52280), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30720_e52276) + (assign30720_e52271 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30720_e52280), ((assign30720_e52271 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30720_e52280),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30720_e52283;
        locals.var_qesj3_dn3 = assign30720_e52283_d_n3;
        locals.var_qesj3_dn4 = assign30720_e52283_d_n4;
        locals.var_qesj3_dn6 = assign30720_e52283_d_n6;

        let (assign30730_e52316, assign30730_e52316_d_n3, assign30730_e52316_d_n4, assign30730_e52316_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 == 0.0)) {
        let assign30730_e52297: f64 = (-locals.var_pbswgs_t);
        let assign30730_e52299: f64 = (assign30730_e52297 * locals.var_czbsswg);
        let (assign30730_e52313, assign30730_e52313_d_n3, assign30730_e52313_d_n4, assign30730_e52313_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30730_e52305: f64 = (-87.498233534);
                (assign30730_e52305, 0.0, 0.0, 0.0,)
            } else {
                let (assign30730_e52312, assign30730_e52312_d_n3, assign30730_e52312_d_n4, assign30730_e52312_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30730_e52310: f64 = (locals.var_arg__blk537).ln();
                        (assign30730_e52310, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30730_e52312, assign30730_e52312_d_n3, assign30730_e52312_d_n4, assign30730_e52312_d_n6,)
            }
        };
        let assign30730_e52314: f64 = (assign30730_e52299 * assign30730_e52313);
        (assign30730_e52314, (assign30730_e52299 * assign30730_e52313_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30730_e52297 * locals.var_czbsswg_dn4)) * assign30730_e52313) + (assign30730_e52299 * assign30730_e52313_d_n4)), (assign30730_e52299 * assign30730_e52313_d_n6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30730_e52316;
        locals.var_qesj3_dn3 = assign30730_e52316_d_n3;
        locals.var_qesj3_dn4 = assign30730_e52316_d_n4;
        locals.var_qesj3_dn6 = assign30730_e52316_d_n6;

        let (assign30740_e52333, assign30740_e52333_d_n3, assign30740_e52333_d_n4, assign30740_e52333_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) {
        let assign30740_e52330: f64 = (locals.var_vec3s / locals.var_pbswgs_t);
        let assign30740_e52331: f64 = (1.0 - assign30740_e52330);
        (assign30740_e52331, 0.0, (-(((locals.var_vec3s_dn4 * locals.var_pbswgs_t) - (locals.var_vec3s * locals.var_pbswgs_t_dn4)) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0,)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30740_e52333;
        locals.var_arg__blk537_dn3 = assign30740_e52333_d_n3;
        locals.var_arg__blk537_dn4 = assign30740_e52333_d_n4;
        locals.var_arg__blk537_dn6 = assign30740_e52333_d_n6;

        let assign30750_e52336: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign30750_e52336;

        let assign30760_e52339: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign30760_e52339;

        let (assign30770_e52359, assign30770_e52359_d_n3, assign30770_e52359_d_n4, assign30770_e52359_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign30770_e52356: f64 = (locals.var_arg__blk537).sqrt();
        let assign30770_e52357: f64 = (1.0 / assign30770_e52356);
        (assign30770_e52357, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30770_e52359;
        locals.var_sarg__blk538_dn3 = assign30770_e52359_d_n3;
        locals.var_sarg__blk538_dn4 = assign30770_e52359_d_n4;
        locals.var_sarg__blk538_dn6 = assign30770_e52359_d_n6;

        let (assign30780_e52380, assign30780_e52380_d_n3, assign30780_e52380_d_n4, assign30780_e52380_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign30780_e52377: f64 = (-p.p1600);
        let assign30780_e52378: f64 = (locals.var_arg__blk537).powf(assign30780_e52377);
        (assign30780_e52378, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30780_e52380;
        locals.var_sarg__blk538_dn3 = assign30780_e52380_d_n3;
        locals.var_sarg__blk538_dn4 = assign30780_e52380_d_n4;
        locals.var_sarg__blk538_dn6 = assign30780_e52380_d_n6;

        let (assign30790_e52407, assign30790_e52407_d_n3, assign30790_e52407_d_n4, assign30790_e52407_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) {
        let assign30790_e52395: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30790_e52399: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30790_e52400: f64 = (1.0 - assign30790_e52399);
        let assign30790_e52401: f64 = (assign30790_e52395 * assign30790_e52400);
        let assign30790_e52404: f64 = (1.0 - p.p1600);
        let assign30790_e52405: f64 = (assign30790_e52401 / assign30790_e52404);
        (assign30790_e52405, ((assign30790_e52395 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30790_e52404), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30790_e52400) + (assign30790_e52395 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30790_e52404), ((assign30790_e52395 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30790_e52404),)
    } else {
        (locals.var_qec__blk539, locals.var_qec__blk539_dn3, locals.var_qec__blk539_dn4, locals.var_qec__blk539_dn6,)
    }
};
        locals.var_qec__blk539 = assign30790_e52407;
        locals.var_qec__blk539_dn3 = assign30790_e52407_d_n3;
        locals.var_qec__blk539_dn4 = assign30790_e52407_d_n4;
        locals.var_qec__blk539_dn6 = assign30790_e52407_d_n6;

    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30800_e52441, assign30800_e52441_d_n3, assign30800_e52441_d_n4, assign30800_e52441_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 == 0.0)) {
        let assign30800_e52422: f64 = (-locals.var_pbswgs_t);
        let assign30800_e52424: f64 = (assign30800_e52422 * locals.var_czbsswg);
        let (assign30800_e52438, assign30800_e52438_d_n3, assign30800_e52438_d_n4, assign30800_e52438_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30800_e52430: f64 = (-87.498233534);
                (assign30800_e52430, 0.0, 0.0, 0.0,)
            } else {
                let (assign30800_e52437, assign30800_e52437_d_n3, assign30800_e52437_d_n4, assign30800_e52437_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30800_e52435: f64 = (locals.var_arg__blk537).ln();
                        (assign30800_e52435, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30800_e52437, assign30800_e52437_d_n3, assign30800_e52437_d_n4, assign30800_e52437_d_n6,)
            }
        };
        let assign30800_e52439: f64 = (assign30800_e52424 * assign30800_e52438);
        (assign30800_e52439, (assign30800_e52424 * assign30800_e52438_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30800_e52422 * locals.var_czbsswg_dn4)) * assign30800_e52438) + (assign30800_e52424 * assign30800_e52438_d_n4)), (assign30800_e52424 * assign30800_e52438_d_n6),)
    } else {
        (locals.var_qec__blk539, locals.var_qec__blk539_dn3, locals.var_qec__blk539_dn4, locals.var_qec__blk539_dn6,)
    }
};
        locals.var_qec__blk539 = assign30800_e52441;
        locals.var_qec__blk539_dn3 = assign30800_e52441_d_n3;
        locals.var_qec__blk539_dn4 = assign30800_e52441_d_n4;
        locals.var_qec__blk539_dn6 = assign30800_e52441_d_n6;

        let (assign30810_e52460, assign30810_e52460_d_n3, assign30810_e52460_d_n4, assign30810_e52460_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) {
        let assign30810_e52455: f64 = (locals.var_ves_jct - locals.var_vec3s);
        let assign30810_e52457: f64 = (assign30810_e52455 / locals.var_pb23s);
        let assign30810_e52458: f64 = (1.0 - assign30810_e52457);
        (assign30810_e52458, (-(locals.var_ves_jct_dn3 / locals.var_pb23s)), (-((((-locals.var_vec3s_dn4) * locals.var_pb23s) - (assign30810_e52455 * locals.var_pb23s_dn4)) / (locals.var_pb23s * locals.var_pb23s))), (-(locals.var_ves_jct_dn6 / locals.var_pb23s)),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30810_e52460;
        locals.var_arg__blk537_dn3 = assign30810_e52460_d_n3;
        locals.var_arg__blk537_dn4 = assign30810_e52460_d_n4;
        locals.var_arg__blk537_dn6 = assign30810_e52460_d_n6;

        let assign30820_e52463: f64 = if p.p1612 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign30820_e52463;

        let assign30830_e52466: f64 = if p.p1612 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign30830_e52466;

        let (assign30840_e52486, assign30840_e52486_d_n3, assign30840_e52486_d_n4, assign30840_e52486_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) && (locals.var_guard549 != 0.0)) {
        let assign30840_e52483: f64 = (locals.var_arg__blk537).sqrt();
        let assign30840_e52484: f64 = (1.0 / assign30840_e52483);
        (assign30840_e52484, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30840_e52486;
        locals.var_sarg__blk538_dn3 = assign30840_e52486_d_n3;
        locals.var_sarg__blk538_dn4 = assign30840_e52486_d_n4;
        locals.var_sarg__blk538_dn6 = assign30840_e52486_d_n6;

        let (assign30850_e52507, assign30850_e52507_d_n3, assign30850_e52507_d_n4, assign30850_e52507_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) && (locals.var_guard549 == 0.0)) {
        let assign30850_e52504: f64 = (-p.p1612);
        let assign30850_e52505: f64 = (locals.var_arg__blk537).powf(assign30850_e52504);
        (assign30850_e52505, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30850_e52507;
        locals.var_sarg__blk538_dn3 = assign30850_e52507_d_n3;
        locals.var_sarg__blk538_dn4 = assign30850_e52507_d_n4;
        locals.var_sarg__blk538_dn6 = assign30850_e52507_d_n6;

        let (assign30860_e52538, assign30860_e52538_d_n3, assign30860_e52538_d_n4, assign30860_e52538_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) {
        let assign30860_e52523: f64 = (p.p1606 * locals.var_pb23s);
        let assign30860_e52525: f64 = (assign30860_e52523 * locals.var_czbsswg);
        let assign30860_e52529: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30860_e52530: f64 = (1.0 - assign30860_e52529);
        let assign30860_e52531: f64 = (assign30860_e52525 * assign30860_e52530);
        let assign30860_e52534: f64 = (1.0 - p.p1612);
        let assign30860_e52535: f64 = (assign30860_e52531 / assign30860_e52534);
        let assign30860_e52536: f64 = (locals.var_qec__blk539 + assign30860_e52535);
        (assign30860_e52536, (locals.var_qec__blk539_dn3 + ((assign30860_e52525 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30860_e52534)), (locals.var_qec__blk539_dn4 + ((((((p.p1606 * locals.var_pb23s_dn4) * locals.var_czbsswg) + (assign30860_e52523 * locals.var_czbsswg_dn4)) * assign30860_e52530) + (assign30860_e52525 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30860_e52534)), (locals.var_qec__blk539_dn6 + ((assign30860_e52525 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30860_e52534)),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30860_e52538;
        locals.var_qesj3_dn3 = assign30860_e52538_d_n3;
        locals.var_qesj3_dn4 = assign30860_e52538_d_n4;
        locals.var_qesj3_dn6 = assign30860_e52538_d_n6;

        let (assign30870_e52575, assign30870_e52575_d_n3, assign30870_e52575_d_n4, assign30870_e52575_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 == 0.0)) {
        let assign30870_e52555: f64 = (p.p1606 * locals.var_pb23s);
        let assign30870_e52557: f64 = (assign30870_e52555 * locals.var_czbsswg);
        let (assign30870_e52571, assign30870_e52571_d_n3, assign30870_e52571_d_n4, assign30870_e52571_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30870_e52563: f64 = (-87.498233534);
                (assign30870_e52563, 0.0, 0.0, 0.0,)
            } else {
                let (assign30870_e52570, assign30870_e52570_d_n3, assign30870_e52570_d_n4, assign30870_e52570_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30870_e52568: f64 = (locals.var_arg__blk537).ln();
                        (assign30870_e52568, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30870_e52570, assign30870_e52570_d_n3, assign30870_e52570_d_n4, assign30870_e52570_d_n6,)
            }
        };
        let assign30870_e52572: f64 = (assign30870_e52557 * assign30870_e52571);
        let assign30870_e52573: f64 = (locals.var_qec__blk539 - assign30870_e52572);
        (assign30870_e52573, (locals.var_qec__blk539_dn3 - (assign30870_e52557 * assign30870_e52571_d_n3)), (locals.var_qec__blk539_dn4 - (((((p.p1606 * locals.var_pb23s_dn4) * locals.var_czbsswg) + (assign30870_e52555 * locals.var_czbsswg_dn4)) * assign30870_e52571) + (assign30870_e52557 * assign30870_e52571_d_n4))), (locals.var_qec__blk539_dn6 - (assign30870_e52557 * assign30870_e52571_d_n6)),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30870_e52575;
        locals.var_qesj3_dn3 = assign30870_e52575_d_n3;
        locals.var_qesj3_dn4 = assign30870_e52575_d_n4;
        locals.var_qesj3_dn6 = assign30870_e52575_d_n6;

        let (assign30880_e52588, assign30880_e52588_d_n3, assign30880_e52588_d_n4, assign30880_e52588_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign30880_e52586: f64 = (1.0 - locals.var_t1__blk532);
        (assign30880_e52586, (-locals.var_t1__blk532_dn3), (-locals.var_t1__blk532_dn4), (-locals.var_t1__blk532_dn6),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30880_e52588;
        locals.var_arg__blk537_dn3 = assign30880_e52588_d_n3;
        locals.var_arg__blk537_dn4 = assign30880_e52588_d_n4;
        locals.var_arg__blk537_dn6 = assign30880_e52588_d_n6;

        let assign30890_e52591: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign30890_e52591;

        let assign30900_e52594: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign30900_e52594;

        let (assign30910_e52612, assign30910_e52612_d_n3, assign30910_e52612_d_n4, assign30910_e52612_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) && (locals.var_guard551 != 0.0)) {
        let assign30910_e52609: f64 = (locals.var_arg__blk537).sqrt();
        let assign30910_e52610: f64 = (1.0 / assign30910_e52609);
        (assign30910_e52610, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30910_e52612;
        locals.var_sarg__blk538_dn3 = assign30910_e52612_d_n3;
        locals.var_sarg__blk538_dn4 = assign30910_e52612_d_n4;
        locals.var_sarg__blk538_dn6 = assign30910_e52612_d_n6;

        let (assign30920_e52631, assign30920_e52631_d_n3, assign30920_e52631_d_n4, assign30920_e52631_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) && (locals.var_guard551 == 0.0)) {
        let assign30920_e52628: f64 = (-p.p1600);
        let assign30920_e52629: f64 = (locals.var_arg__blk537).powf(assign30920_e52628);
        (assign30920_e52629, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30920_e52631;
        locals.var_sarg__blk538_dn3 = assign30920_e52631_d_n3;
        locals.var_sarg__blk538_dn4 = assign30920_e52631_d_n4;
        locals.var_sarg__blk538_dn6 = assign30920_e52631_d_n6;

        let (assign30930_e52656, assign30930_e52656_d_n3, assign30930_e52656_d_n4, assign30930_e52656_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) {
        let assign30930_e52644: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30930_e52648: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30930_e52649: f64 = (1.0 - assign30930_e52648);
        let assign30930_e52650: f64 = (assign30930_e52644 * assign30930_e52649);
        let assign30930_e52653: f64 = (1.0 - p.p1600);
        let assign30930_e52654: f64 = (assign30930_e52650 / assign30930_e52653);
        (assign30930_e52654, ((assign30930_e52644 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30930_e52653), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30930_e52649) + (assign30930_e52644 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30930_e52653), ((assign30930_e52644 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30930_e52653),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30930_e52656;
        locals.var_qesj3_dn3 = assign30930_e52656_d_n3;
        locals.var_qesj3_dn4 = assign30930_e52656_d_n4;
        locals.var_qesj3_dn6 = assign30930_e52656_d_n6;

        let (assign30940_e52688, assign30940_e52688_d_n3, assign30940_e52688_d_n4, assign30940_e52688_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 == 0.0)) {
        let assign30940_e52669: f64 = (-locals.var_pbswgs_t);
        let assign30940_e52671: f64 = (assign30940_e52669 * locals.var_czbsswg);
        let (assign30940_e52685, assign30940_e52685_d_n3, assign30940_e52685_d_n4, assign30940_e52685_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30940_e52677: f64 = (-87.498233534);
                (assign30940_e52677, 0.0, 0.0, 0.0,)
            } else {
                let (assign30940_e52684, assign30940_e52684_d_n3, assign30940_e52684_d_n4, assign30940_e52684_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30940_e52682: f64 = (locals.var_arg__blk537).ln();
                        (assign30940_e52682, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30940_e52684, assign30940_e52684_d_n3, assign30940_e52684_d_n4, assign30940_e52684_d_n6,)
            }
        };
        let assign30940_e52686: f64 = (assign30940_e52671 * assign30940_e52685);
        (assign30940_e52686, (assign30940_e52671 * assign30940_e52685_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30940_e52669 * locals.var_czbsswg_dn4)) * assign30940_e52685) + (assign30940_e52671 * assign30940_e52685_d_n4)), (assign30940_e52671 * assign30940_e52685_d_n6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30940_e52688;
        locals.var_qesj3_dn3 = assign30940_e52688_d_n3;
        locals.var_qesj3_dn4 = assign30940_e52688_d_n4;
        locals.var_qesj3_dn6 = assign30940_e52688_d_n6;

        let assign30950_e52691: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign30950_e52691;

        let assign30960_e52694: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign30960_e52694;

        let (assign30970_e52710,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) && (locals.var_guard553 != 0.0)) {
        let assign30970_e52707: f64 = (0.1_f64).sqrt();
        let assign30970_e52708: f64 = (1.0 / assign30970_e52707);
        (assign30970_e52708,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign30970_e52710;

        let (assign30980_e52727,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) && (locals.var_guard553 == 0.0)) {
        let assign30980_e52724: f64 = (-p.p1600);
        let assign30980_e52725: f64 = (0.1_f64).powf(assign30980_e52724);
        (assign30980_e52725,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign30980_e52727;

        let (assign30990_e52742,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) {
        let assign30990_e52739: f64 = (1.0 - p.p1600);
        let assign30990_e52740: f64 = (1.0 / assign30990_e52739);
        (assign30990_e52740,)
    } else {
        (locals.var_t3__blk534,)
    }
};
        locals.var_t3__blk534 = assign30990_e52742;

        let (assign31000_e52765,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) {
        let assign31000_e52755: f64 = (0.05 * p.p1600);
        let assign31000_e52758: f64 = (1.0 + p.p1600);
        let assign31000_e52759: f64 = (assign31000_e52755 * assign31000_e52758);
        let assign31000_e52761: f64 = (assign31000_e52759 * locals.var_t2__blk533);
        let assign31000_e52762: f64 = (1.0 - assign31000_e52761);
        let assign31000_e52763: f64 = (locals.var_t3__blk534 * assign31000_e52762);
        (assign31000_e52763,)
    } else {
        (locals.var_t5__blk536,)
    }
};
        locals.var_t5__blk536 = assign31000_e52765;

        let (assign31010_e52777,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign31010_e52777;

        let (assign31020_e52792,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign31020_e52789: f64 = (0.1_f64).ln();
        let assign31020_e52790: f64 = (1.5 - assign31020_e52789);
        (assign31020_e52790,)
    } else {
        (locals.var_t5__blk536,)
    }
};
        locals.var_t5__blk536 = assign31020_e52792;

        let (assign31030_e52817, assign31030_e52817_d_n3, assign31030_e52817_d_n4, assign31030_e52817_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) {
        let assign31030_e52802: f64 = (locals.var_t1__blk532 - 1.0);
        let assign31030_e52803: f64 = (locals.var_t2__blk533 * assign31030_e52802);
        let assign31030_e52806: f64 = (5.0 * p.p1600);
        let assign31030_e52809: f64 = (locals.var_t1__blk532 - 1.0);
        let assign31030_e52810: f64 = (assign31030_e52806 * assign31030_e52809);
        let assign31030_e52813: f64 = (1.0 + p.p1600);
        let assign31030_e52814: f64 = (assign31030_e52810 + assign31030_e52813);
        let assign31030_e52815: f64 = (assign31030_e52803 * assign31030_e52814);
        (assign31030_e52815, (((locals.var_t2__blk533 * locals.var_t1__blk532_dn3) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn3))), (((locals.var_t2__blk533 * locals.var_t1__blk532_dn4) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn4))), (((locals.var_t2__blk533 * locals.var_t1__blk532_dn6) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn6))),)
    } else {
        (locals.var_t4__blk535, locals.var_t4__blk535_dn3, locals.var_t4__blk535_dn4, locals.var_t4__blk535_dn6,)
    }
};
        locals.var_t4__blk535 = assign31030_e52817;
        locals.var_t4__blk535_dn3 = assign31030_e52817_d_n3;
        locals.var_t4__blk535_dn4 = assign31030_e52817_d_n4;
        locals.var_t4__blk535_dn6 = assign31030_e52817_d_n6;

        let (assign31040_e52832, assign31040_e52832_d_n3, assign31040_e52832_d_n4, assign31040_e52832_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) {
        let assign31040_e52826: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign31040_e52829: f64 = (locals.var_t4__blk535 + locals.var_t5__blk536);
        let assign31040_e52830: f64 = (assign31040_e52826 * assign31040_e52829);
        (assign31040_e52830, (assign31040_e52826 * locals.var_t4__blk535_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign31040_e52829) + (assign31040_e52826 * locals.var_t4__blk535_dn4)), (assign31040_e52826 * locals.var_t4__blk535_dn6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign31040_e52832;
        locals.var_qesj3_dn3 = assign31040_e52832_d_n3;
        locals.var_qesj3_dn4 = assign31040_e52832_d_n4;
        locals.var_qesj3_dn6 = assign31040_e52832_d_n6;

        let (assign31050_e52839, assign31050_e52839_d_n3, assign31050_e52839_d_n4, assign31050_e52839_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard540 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign31050_e52839;
        locals.var_qesj3_dn3 = assign31050_e52839_d_n3;
        locals.var_qesj3_dn4 = assign31050_e52839_d_n4;
        locals.var_qesj3_dn6 = assign31050_e52839_d_n6;

        let (assign31060_e52847, assign31060_e52847_d_n3, assign31060_e52847_d_n4, assign31060_e52847_d_n6,) = {
    if (locals.var_guard469 != 0.0) {
        let assign31060_e52843: f64 = (locals.var_qesj1 + locals.var_qesj2);
        let assign31060_e52845: f64 = (assign31060_e52843 + locals.var_qesj3);
        (assign31060_e52845, ((locals.var_qesj1_dn3 + locals.var_qesj2_dn3) + locals.var_qesj3_dn3), ((locals.var_qesj1_dn4 + locals.var_qesj2_dn4) + locals.var_qesj3_dn4), ((locals.var_qesj1_dn6 + locals.var_qesj2_dn6) + locals.var_qesj3_dn6),)
    } else {
        (locals.var_qesj, locals.var_qesj_dn3, locals.var_qesj_dn4, locals.var_qesj_dn6,)
    }
};
        locals.var_qesj = assign31060_e52847;
        locals.var_qesj_dn3 = assign31060_e52847_d_n3;
        locals.var_qesj_dn4 = assign31060_e52847_d_n4;
        locals.var_qesj_dn6 = assign31060_e52847_d_n6;

        let assign31070_e52850: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign31070_e52850;

        let (assign31080_e52858, assign31080_e52858_d_n3, assign31080_e52858_d_n4, assign31080_e52858_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) {
        let assign31080_e52856: f64 = (locals.var_ved_jct / locals.var_pbd_t);
        (assign31080_e52856, (locals.var_ved_jct_dn3 / locals.var_pbd_t), (-((locals.var_ved_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_ved_jct_dn5 / locals.var_pbd_t),)
    } else {
        (locals.var_t1__blk554, locals.var_t1__blk554_dn3, locals.var_t1__blk554_dn4, locals.var_t1__blk554_dn5,)
    }
};
        locals.var_t1__blk554 = assign31080_e52858;
        locals.var_t1__blk554_dn3 = assign31080_e52858_d_n3;
        locals.var_t1__blk554_dn4 = assign31080_e52858_d_n4;
        locals.var_t1__blk554_dn5 = assign31080_e52858_d_n5;

        let assign31090_e52861: f64 = if locals.var_t1__blk554 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign31090_e52861;

        let assign31100_e52864: f64 = if p.p1603 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard564 = assign31100_e52864;

        let assign31110_e52867: f64 = if locals.var_ved_jct > locals.var_vec1d { 1.0 } else { 0.0 };
        locals.var_guard565 = assign31110_e52867;

        let (assign31120_e52881, assign31120_e52881_d_n3, assign31120_e52881_d_n4, assign31120_e52881_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign31120_e52879: f64 = (1.0 - locals.var_t1__blk554);
        (assign31120_e52879, (-locals.var_t1__blk554_dn3), (-locals.var_t1__blk554_dn4), (-locals.var_t1__blk554_dn5),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31120_e52881;
        locals.var_arg__blk559_dn3 = assign31120_e52881_d_n3;
        locals.var_arg__blk559_dn4 = assign31120_e52881_d_n4;
        locals.var_arg__blk559_dn5 = assign31120_e52881_d_n5;

        let assign31130_e52884: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign31130_e52884;

        let assign31140_e52887: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign31140_e52887;

        let (assign31150_e52906, assign31150_e52906_d_n3, assign31150_e52906_d_n4, assign31150_e52906_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) {
        let assign31150_e52903: f64 = (locals.var_arg__blk559).sqrt();
        let assign31150_e52904: f64 = (1.0 / assign31150_e52903);
        (assign31150_e52904, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31150_e52906;
        locals.var_sarg__blk560_dn3 = assign31150_e52906_d_n3;
        locals.var_sarg__blk560_dn4 = assign31150_e52906_d_n4;
        locals.var_sarg__blk560_dn5 = assign31150_e52906_d_n5;

        let (assign31160_e52926, assign31160_e52926_d_n3, assign31160_e52926_d_n4, assign31160_e52926_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 == 0.0)) {
        let assign31160_e52923: f64 = (-p.p1597);
        let assign31160_e52924: f64 = (locals.var_arg__blk559).powf(assign31160_e52923);
        (assign31160_e52924, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31160_e52926;
        locals.var_sarg__blk560_dn3 = assign31160_e52926_d_n3;
        locals.var_sarg__blk560_dn4 = assign31160_e52926_d_n4;
        locals.var_sarg__blk560_dn5 = assign31160_e52926_d_n5;

        let (assign31170_e52952, assign31170_e52952_d_n3, assign31170_e52952_d_n4, assign31170_e52952_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign31170_e52940: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31170_e52944: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31170_e52945: f64 = (1.0 - assign31170_e52944);
        let assign31170_e52946: f64 = (assign31170_e52940 * assign31170_e52945);
        let assign31170_e52949: f64 = (1.0 - p.p1597);
        let assign31170_e52950: f64 = (assign31170_e52946 / assign31170_e52949);
        (assign31170_e52950, ((assign31170_e52940 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31170_e52949), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31170_e52945) + (assign31170_e52940 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31170_e52949), ((assign31170_e52940 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31170_e52949),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31170_e52952;
        locals.var_qedj1_dn3 = assign31170_e52952_d_n3;
        locals.var_qedj1_dn4 = assign31170_e52952_d_n4;
        locals.var_qedj1_dn5 = assign31170_e52952_d_n5;

        let (assign31180_e52985, assign31180_e52985_d_n3, assign31180_e52985_d_n4, assign31180_e52985_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 == 0.0)) {
        let assign31180_e52966: f64 = (-locals.var_pbd_t);
        let assign31180_e52968: f64 = (assign31180_e52966 * locals.var_czbd);
        let (assign31180_e52982, assign31180_e52982_d_n3, assign31180_e52982_d_n4, assign31180_e52982_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31180_e52974: f64 = (-87.498233534);
                (assign31180_e52974, 0.0, 0.0, 0.0,)
            } else {
                let (assign31180_e52981, assign31180_e52981_d_n3, assign31180_e52981_d_n4, assign31180_e52981_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31180_e52979: f64 = (locals.var_arg__blk559).ln();
                        (assign31180_e52979, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31180_e52981, assign31180_e52981_d_n3, assign31180_e52981_d_n4, assign31180_e52981_d_n5,)
            }
        };
        let assign31180_e52983: f64 = (assign31180_e52968 * assign31180_e52982);
        (assign31180_e52983, (assign31180_e52968 * assign31180_e52982_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31180_e52966 * locals.var_czbd_dn4)) * assign31180_e52982) + (assign31180_e52968 * assign31180_e52982_d_n4)), (assign31180_e52968 * assign31180_e52982_d_n5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31180_e52985;
        locals.var_qedj1_dn3 = assign31180_e52985_d_n3;
        locals.var_qedj1_dn4 = assign31180_e52985_d_n4;
        locals.var_qedj1_dn5 = assign31180_e52985_d_n5;

        let (assign31190_e53002, assign31190_e53002_d_n3, assign31190_e53002_d_n4, assign31190_e53002_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) {
        let assign31190_e52999: f64 = (locals.var_vec1d / locals.var_pbd_t);
        let assign31190_e53000: f64 = (1.0 - assign31190_e52999);
        (assign31190_e53000, 0.0, (-(((locals.var_vec1d_dn4 * locals.var_pbd_t) - (locals.var_vec1d * locals.var_pbd_t_dn4)) / (locals.var_pbd_t * locals.var_pbd_t))), 0.0,)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31190_e53002;
        locals.var_arg__blk559_dn3 = assign31190_e53002_d_n3;
        locals.var_arg__blk559_dn4 = assign31190_e53002_d_n4;
        locals.var_arg__blk559_dn5 = assign31190_e53002_d_n5;

        let assign31200_e53005: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign31200_e53005;

        let assign31210_e53008: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign31210_e53008;

    }

    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31220_e53028, assign31220_e53028_d_n3, assign31220_e53028_d_n4, assign31220_e53028_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        let assign31220_e53025: f64 = (locals.var_arg__blk559).sqrt();
        let assign31220_e53026: f64 = (1.0 / assign31220_e53025);
        (assign31220_e53026, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31220_e53028;
        locals.var_sarg__blk560_dn3 = assign31220_e53028_d_n3;
        locals.var_sarg__blk560_dn4 = assign31220_e53028_d_n4;
        locals.var_sarg__blk560_dn5 = assign31220_e53028_d_n5;

        let (assign31230_e53049, assign31230_e53049_d_n3, assign31230_e53049_d_n4, assign31230_e53049_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign31230_e53046: f64 = (-p.p1597);
        let assign31230_e53047: f64 = (locals.var_arg__blk559).powf(assign31230_e53046);
        (assign31230_e53047, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31230_e53049;
        locals.var_sarg__blk560_dn3 = assign31230_e53049_d_n3;
        locals.var_sarg__blk560_dn4 = assign31230_e53049_d_n4;
        locals.var_sarg__blk560_dn5 = assign31230_e53049_d_n5;

        let (assign31240_e53076, assign31240_e53076_d_n3, assign31240_e53076_d_n4, assign31240_e53076_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign31240_e53064: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31240_e53068: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31240_e53069: f64 = (1.0 - assign31240_e53068);
        let assign31240_e53070: f64 = (assign31240_e53064 * assign31240_e53069);
        let assign31240_e53073: f64 = (1.0 - p.p1597);
        let assign31240_e53074: f64 = (assign31240_e53070 / assign31240_e53073);
        (assign31240_e53074, ((assign31240_e53064 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31240_e53073), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31240_e53069) + (assign31240_e53064 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31240_e53073), ((assign31240_e53064 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31240_e53073),)
    } else {
        (locals.var_qec__blk561, locals.var_qec__blk561_dn3, locals.var_qec__blk561_dn4, locals.var_qec__blk561_dn5,)
    }
};
        locals.var_qec__blk561 = assign31240_e53076;
        locals.var_qec__blk561_dn3 = assign31240_e53076_d_n3;
        locals.var_qec__blk561_dn4 = assign31240_e53076_d_n4;
        locals.var_qec__blk561_dn5 = assign31240_e53076_d_n5;

        let (assign31250_e53110, assign31250_e53110_d_n3, assign31250_e53110_d_n4, assign31250_e53110_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 == 0.0)) {
        let assign31250_e53091: f64 = (-locals.var_pbd_t);
        let assign31250_e53093: f64 = (assign31250_e53091 * locals.var_czbd);
        let (assign31250_e53107, assign31250_e53107_d_n3, assign31250_e53107_d_n4, assign31250_e53107_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31250_e53099: f64 = (-87.498233534);
                (assign31250_e53099, 0.0, 0.0, 0.0,)
            } else {
                let (assign31250_e53106, assign31250_e53106_d_n3, assign31250_e53106_d_n4, assign31250_e53106_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31250_e53104: f64 = (locals.var_arg__blk559).ln();
                        (assign31250_e53104, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31250_e53106, assign31250_e53106_d_n3, assign31250_e53106_d_n4, assign31250_e53106_d_n5,)
            }
        };
        let assign31250_e53108: f64 = (assign31250_e53093 * assign31250_e53107);
        (assign31250_e53108, (assign31250_e53093 * assign31250_e53107_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31250_e53091 * locals.var_czbd_dn4)) * assign31250_e53107) + (assign31250_e53093 * assign31250_e53107_d_n4)), (assign31250_e53093 * assign31250_e53107_d_n5),)
    } else {
        (locals.var_qec__blk561, locals.var_qec__blk561_dn3, locals.var_qec__blk561_dn4, locals.var_qec__blk561_dn5,)
    }
};
        locals.var_qec__blk561 = assign31250_e53110;
        locals.var_qec__blk561_dn3 = assign31250_e53110_d_n3;
        locals.var_qec__blk561_dn4 = assign31250_e53110_d_n4;
        locals.var_qec__blk561_dn5 = assign31250_e53110_d_n5;

        let (assign31260_e53129, assign31260_e53129_d_n3, assign31260_e53129_d_n4, assign31260_e53129_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) {
        let assign31260_e53124: f64 = (locals.var_ved_jct - locals.var_vec1d);
        let assign31260_e53126: f64 = (assign31260_e53124 / locals.var_pb21d);
        let assign31260_e53127: f64 = (1.0 - assign31260_e53126);
        (assign31260_e53127, (-(locals.var_ved_jct_dn3 / locals.var_pb21d)), (-((((-locals.var_vec1d_dn4) * locals.var_pb21d) - (assign31260_e53124 * locals.var_pb21d_dn4)) / (locals.var_pb21d * locals.var_pb21d))), (-(locals.var_ved_jct_dn5 / locals.var_pb21d)),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31260_e53129;
        locals.var_arg__blk559_dn3 = assign31260_e53129_d_n3;
        locals.var_arg__blk559_dn4 = assign31260_e53129_d_n4;
        locals.var_arg__blk559_dn5 = assign31260_e53129_d_n5;

        let assign31270_e53132: f64 = if p.p1609 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign31270_e53132;

        let assign31280_e53135: f64 = if p.p1609 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign31280_e53135;

        let (assign31290_e53155, assign31290_e53155_d_n3, assign31290_e53155_d_n4, assign31290_e53155_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        let assign31290_e53152: f64 = (locals.var_arg__blk559).sqrt();
        let assign31290_e53153: f64 = (1.0 / assign31290_e53152);
        (assign31290_e53153, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31290_e53155;
        locals.var_sarg__blk560_dn3 = assign31290_e53155_d_n3;
        locals.var_sarg__blk560_dn4 = assign31290_e53155_d_n4;
        locals.var_sarg__blk560_dn5 = assign31290_e53155_d_n5;

        let (assign31300_e53176, assign31300_e53176_d_n3, assign31300_e53176_d_n4, assign31300_e53176_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign31300_e53173: f64 = (-p.p1609);
        let assign31300_e53174: f64 = (locals.var_arg__blk559).powf(assign31300_e53173);
        (assign31300_e53174, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31300_e53176;
        locals.var_sarg__blk560_dn3 = assign31300_e53176_d_n3;
        locals.var_sarg__blk560_dn4 = assign31300_e53176_d_n4;
        locals.var_sarg__blk560_dn5 = assign31300_e53176_d_n5;

        let (assign31310_e53207, assign31310_e53207_d_n3, assign31310_e53207_d_n4, assign31310_e53207_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign31310_e53192: f64 = (p.p1603 * locals.var_pb21d);
        let assign31310_e53194: f64 = (assign31310_e53192 * locals.var_czbd);
        let assign31310_e53198: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31310_e53199: f64 = (1.0 - assign31310_e53198);
        let assign31310_e53200: f64 = (assign31310_e53194 * assign31310_e53199);
        let assign31310_e53203: f64 = (1.0 - p.p1609);
        let assign31310_e53204: f64 = (assign31310_e53200 / assign31310_e53203);
        let assign31310_e53205: f64 = (locals.var_qec__blk561 + assign31310_e53204);
        (assign31310_e53205, (locals.var_qec__blk561_dn3 + ((assign31310_e53194 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31310_e53203)), (locals.var_qec__blk561_dn4 + ((((((p.p1603 * locals.var_pb21d_dn4) * locals.var_czbd) + (assign31310_e53192 * locals.var_czbd_dn4)) * assign31310_e53199) + (assign31310_e53194 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31310_e53203)), (locals.var_qec__blk561_dn5 + ((assign31310_e53194 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31310_e53203)),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31310_e53207;
        locals.var_qedj1_dn3 = assign31310_e53207_d_n3;
        locals.var_qedj1_dn4 = assign31310_e53207_d_n4;
        locals.var_qedj1_dn5 = assign31310_e53207_d_n5;

        let (assign31320_e53244, assign31320_e53244_d_n3, assign31320_e53244_d_n4, assign31320_e53244_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign31320_e53224: f64 = (p.p1603 * locals.var_pb21d);
        let assign31320_e53226: f64 = (assign31320_e53224 * locals.var_czbd);
        let (assign31320_e53240, assign31320_e53240_d_n3, assign31320_e53240_d_n4, assign31320_e53240_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31320_e53232: f64 = (-87.498233534);
                (assign31320_e53232, 0.0, 0.0, 0.0,)
            } else {
                let (assign31320_e53239, assign31320_e53239_d_n3, assign31320_e53239_d_n4, assign31320_e53239_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31320_e53237: f64 = (locals.var_arg__blk559).ln();
                        (assign31320_e53237, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31320_e53239, assign31320_e53239_d_n3, assign31320_e53239_d_n4, assign31320_e53239_d_n5,)
            }
        };
        let assign31320_e53241: f64 = (assign31320_e53226 * assign31320_e53240);
        let assign31320_e53242: f64 = (locals.var_qec__blk561 - assign31320_e53241);
        (assign31320_e53242, (locals.var_qec__blk561_dn3 - (assign31320_e53226 * assign31320_e53240_d_n3)), (locals.var_qec__blk561_dn4 - (((((p.p1603 * locals.var_pb21d_dn4) * locals.var_czbd) + (assign31320_e53224 * locals.var_czbd_dn4)) * assign31320_e53240) + (assign31320_e53226 * assign31320_e53240_d_n4))), (locals.var_qec__blk561_dn5 - (assign31320_e53226 * assign31320_e53240_d_n5)),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31320_e53244;
        locals.var_qedj1_dn3 = assign31320_e53244_d_n3;
        locals.var_qedj1_dn4 = assign31320_e53244_d_n4;
        locals.var_qedj1_dn5 = assign31320_e53244_d_n5;

        let (assign31330_e53257, assign31330_e53257_d_n3, assign31330_e53257_d_n4, assign31330_e53257_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign31330_e53255: f64 = (1.0 - locals.var_t1__blk554);
        (assign31330_e53255, (-locals.var_t1__blk554_dn3), (-locals.var_t1__blk554_dn4), (-locals.var_t1__blk554_dn5),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31330_e53257;
        locals.var_arg__blk559_dn3 = assign31330_e53257_d_n3;
        locals.var_arg__blk559_dn4 = assign31330_e53257_d_n4;
        locals.var_arg__blk559_dn5 = assign31330_e53257_d_n5;

        let assign31340_e53260: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign31340_e53260;

        let assign31350_e53263: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign31350_e53263;

        let (assign31360_e53281, assign31360_e53281_d_n3, assign31360_e53281_d_n4, assign31360_e53281_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 != 0.0)) {
        let assign31360_e53278: f64 = (locals.var_arg__blk559).sqrt();
        let assign31360_e53279: f64 = (1.0 / assign31360_e53278);
        (assign31360_e53279, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31360_e53281;
        locals.var_sarg__blk560_dn3 = assign31360_e53281_d_n3;
        locals.var_sarg__blk560_dn4 = assign31360_e53281_d_n4;
        locals.var_sarg__blk560_dn5 = assign31360_e53281_d_n5;

        let (assign31370_e53300, assign31370_e53300_d_n3, assign31370_e53300_d_n4, assign31370_e53300_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 == 0.0)) {
        let assign31370_e53297: f64 = (-p.p1597);
        let assign31370_e53298: f64 = (locals.var_arg__blk559).powf(assign31370_e53297);
        (assign31370_e53298, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31370_e53300;
        locals.var_sarg__blk560_dn3 = assign31370_e53300_d_n3;
        locals.var_sarg__blk560_dn4 = assign31370_e53300_d_n4;
        locals.var_sarg__blk560_dn5 = assign31370_e53300_d_n5;

        let (assign31380_e53325, assign31380_e53325_d_n3, assign31380_e53325_d_n4, assign31380_e53325_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign31380_e53313: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31380_e53317: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31380_e53318: f64 = (1.0 - assign31380_e53317);
        let assign31380_e53319: f64 = (assign31380_e53313 * assign31380_e53318);
        let assign31380_e53322: f64 = (1.0 - p.p1597);
        let assign31380_e53323: f64 = (assign31380_e53319 / assign31380_e53322);
        (assign31380_e53323, ((assign31380_e53313 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31380_e53322), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31380_e53318) + (assign31380_e53313 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31380_e53322), ((assign31380_e53313 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31380_e53322),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31380_e53325;
        locals.var_qedj1_dn3 = assign31380_e53325_d_n3;
        locals.var_qedj1_dn4 = assign31380_e53325_d_n4;
        locals.var_qedj1_dn5 = assign31380_e53325_d_n5;

        let (assign31390_e53357, assign31390_e53357_d_n3, assign31390_e53357_d_n4, assign31390_e53357_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign31390_e53338: f64 = (-locals.var_pbd_t);
        let assign31390_e53340: f64 = (assign31390_e53338 * locals.var_czbd);
        let (assign31390_e53354, assign31390_e53354_d_n3, assign31390_e53354_d_n4, assign31390_e53354_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31390_e53346: f64 = (-87.498233534);
                (assign31390_e53346, 0.0, 0.0, 0.0,)
            } else {
                let (assign31390_e53353, assign31390_e53353_d_n3, assign31390_e53353_d_n4, assign31390_e53353_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31390_e53351: f64 = (locals.var_arg__blk559).ln();
                        (assign31390_e53351, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31390_e53353, assign31390_e53353_d_n3, assign31390_e53353_d_n4, assign31390_e53353_d_n5,)
            }
        };
        let assign31390_e53355: f64 = (assign31390_e53340 * assign31390_e53354);
        (assign31390_e53355, (assign31390_e53340 * assign31390_e53354_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31390_e53338 * locals.var_czbd_dn4)) * assign31390_e53354) + (assign31390_e53340 * assign31390_e53354_d_n4)), (assign31390_e53340 * assign31390_e53354_d_n5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31390_e53357;
        locals.var_qedj1_dn3 = assign31390_e53357_d_n3;
        locals.var_qedj1_dn4 = assign31390_e53357_d_n4;
        locals.var_qedj1_dn5 = assign31390_e53357_d_n5;

        let assign31400_e53360: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign31400_e53360;

        let assign31410_e53363: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign31410_e53363;

        let (assign31420_e53379,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 != 0.0)) {
        let assign31420_e53376: f64 = (0.1_f64).sqrt();
        let assign31420_e53377: f64 = (1.0 / assign31420_e53376);
        (assign31420_e53377,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31420_e53379;

        let (assign31430_e53396,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 == 0.0)) {
        let assign31430_e53393: f64 = (-p.p1597);
        let assign31430_e53394: f64 = (0.1_f64).powf(assign31430_e53393);
        (assign31430_e53394,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31430_e53396;

        let (assign31440_e53411,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign31440_e53408: f64 = (1.0 - p.p1597);
        let assign31440_e53409: f64 = (1.0 / assign31440_e53408);
        (assign31440_e53409,)
    } else {
        (locals.var_t3__blk556,)
    }
};
        locals.var_t3__blk556 = assign31440_e53411;

        let (assign31450_e53434,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign31450_e53424: f64 = (0.05 * p.p1597);
        let assign31450_e53427: f64 = (1.0 + p.p1597);
        let assign31450_e53428: f64 = (assign31450_e53424 * assign31450_e53427);
        let assign31450_e53430: f64 = (assign31450_e53428 * locals.var_t2__blk555);
        let assign31450_e53431: f64 = (1.0 - assign31450_e53430);
        let assign31450_e53432: f64 = (locals.var_t3__blk556 * assign31450_e53431);
        (assign31450_e53432,)
    } else {
        (locals.var_t5__blk558,)
    }
};
        locals.var_t5__blk558 = assign31450_e53434;

        let (assign31460_e53446,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31460_e53446;

        let (assign31470_e53461,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 == 0.0)) {
        let assign31470_e53458: f64 = (0.1_f64).ln();
        let assign31470_e53459: f64 = (1.5 - assign31470_e53458);
        (assign31470_e53459,)
    } else {
        (locals.var_t5__blk558,)
    }
};
        locals.var_t5__blk558 = assign31470_e53461;

        let (assign31480_e53486, assign31480_e53486_d_n3, assign31480_e53486_d_n4, assign31480_e53486_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign31480_e53471: f64 = (locals.var_t1__blk554 - 1.0);
        let assign31480_e53472: f64 = (locals.var_t2__blk555 * assign31480_e53471);
        let assign31480_e53475: f64 = (5.0 * p.p1597);
        let assign31480_e53478: f64 = (locals.var_t1__blk554 - 1.0);
        let assign31480_e53479: f64 = (assign31480_e53475 * assign31480_e53478);
        let assign31480_e53482: f64 = (1.0 + p.p1597);
        let assign31480_e53483: f64 = (assign31480_e53479 + assign31480_e53482);
        let assign31480_e53484: f64 = (assign31480_e53472 * assign31480_e53483);
        (assign31480_e53484, (((locals.var_t2__blk555 * locals.var_t1__blk554_dn3) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn3))), (((locals.var_t2__blk555 * locals.var_t1__blk554_dn4) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn4))), (((locals.var_t2__blk555 * locals.var_t1__blk554_dn5) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn5))),)
    } else {
        (locals.var_t4__blk557, locals.var_t4__blk557_dn3, locals.var_t4__blk557_dn4, locals.var_t4__blk557_dn5,)
    }
};
        locals.var_t4__blk557 = assign31480_e53486;
        locals.var_t4__blk557_dn3 = assign31480_e53486_d_n3;
        locals.var_t4__blk557_dn4 = assign31480_e53486_d_n4;
        locals.var_t4__blk557_dn5 = assign31480_e53486_d_n5;

        let (assign31490_e53501, assign31490_e53501_d_n3, assign31490_e53501_d_n4, assign31490_e53501_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign31490_e53495: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31490_e53498: f64 = (locals.var_t4__blk557 + locals.var_t5__blk558);
        let assign31490_e53499: f64 = (assign31490_e53495 * assign31490_e53498);
        (assign31490_e53499, (assign31490_e53495 * locals.var_t4__blk557_dn3), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31490_e53498) + (assign31490_e53495 * locals.var_t4__blk557_dn4)), (assign31490_e53495 * locals.var_t4__blk557_dn5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31490_e53501;
        locals.var_qedj1_dn3 = assign31490_e53501_d_n3;
        locals.var_qedj1_dn4 = assign31490_e53501_d_n4;
        locals.var_qedj1_dn5 = assign31490_e53501_d_n5;

        let (assign31500_e53508, assign31500_e53508_d_n3, assign31500_e53508_d_n4, assign31500_e53508_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard562 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31500_e53508;
        locals.var_qedj1_dn3 = assign31500_e53508_d_n3;
        locals.var_qedj1_dn4 = assign31500_e53508_d_n4;
        locals.var_qedj1_dn5 = assign31500_e53508_d_n5;

        let assign31510_e53511: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign31510_e53511;

        let (assign31520_e53519, assign31520_e53519_d_n3, assign31520_e53519_d_n4, assign31520_e53519_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign31520_e53517: f64 = (locals.var_ved_jct / locals.var_pbswd_t);
        (assign31520_e53517, (locals.var_ved_jct_dn3 / locals.var_pbswd_t), (-((locals.var_ved_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_ved_jct_dn5 / locals.var_pbswd_t),)
    } else {
        (locals.var_t1__blk576, locals.var_t1__blk576_dn3, locals.var_t1__blk576_dn4, locals.var_t1__blk576_dn5,)
    }
};
        locals.var_t1__blk576 = assign31520_e53519;
        locals.var_t1__blk576_dn3 = assign31520_e53519_d_n3;
        locals.var_t1__blk576_dn4 = assign31520_e53519_d_n4;
        locals.var_t1__blk576_dn5 = assign31520_e53519_d_n5;

        let assign31530_e53522: f64 = if locals.var_t1__blk576 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign31530_e53522;

        let assign31540_e53525: f64 = if p.p1605 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign31540_e53525;

        let assign31550_e53528: f64 = if locals.var_ved_jct > locals.var_vec2d { 1.0 } else { 0.0 };
        locals.var_guard587 = assign31550_e53528;

        let (assign31560_e53542, assign31560_e53542_d_n3, assign31560_e53542_d_n4, assign31560_e53542_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign31560_e53540: f64 = (1.0 - locals.var_t1__blk576);
        (assign31560_e53540, (-locals.var_t1__blk576_dn3), (-locals.var_t1__blk576_dn4), (-locals.var_t1__blk576_dn5),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31560_e53542;
        locals.var_arg__blk581_dn3 = assign31560_e53542_d_n3;
        locals.var_arg__blk581_dn4 = assign31560_e53542_d_n4;
        locals.var_arg__blk581_dn5 = assign31560_e53542_d_n5;

        let assign31570_e53545: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign31570_e53545;

        let assign31580_e53548: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign31580_e53548;

        let (assign31590_e53567, assign31590_e53567_d_n3, assign31590_e53567_d_n4, assign31590_e53567_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign31590_e53564: f64 = (locals.var_arg__blk581).sqrt();
        let assign31590_e53565: f64 = (1.0 / assign31590_e53564);
        (assign31590_e53565, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31590_e53567;
        locals.var_sarg__blk582_dn3 = assign31590_e53567_d_n3;
        locals.var_sarg__blk582_dn4 = assign31590_e53567_d_n4;
        locals.var_sarg__blk582_dn5 = assign31590_e53567_d_n5;

        let (assign31600_e53587, assign31600_e53587_d_n3, assign31600_e53587_d_n4, assign31600_e53587_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 == 0.0)) {
        let assign31600_e53584: f64 = (-p.p1599);
        let assign31600_e53585: f64 = (locals.var_arg__blk581).powf(assign31600_e53584);
        (assign31600_e53585, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31600_e53587;
        locals.var_sarg__blk582_dn3 = assign31600_e53587_d_n3;
        locals.var_sarg__blk582_dn4 = assign31600_e53587_d_n4;
        locals.var_sarg__blk582_dn5 = assign31600_e53587_d_n5;

        let (assign31610_e53613, assign31610_e53613_d_n3, assign31610_e53613_d_n4, assign31610_e53613_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign31610_e53601: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31610_e53605: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31610_e53606: f64 = (1.0 - assign31610_e53605);
        let assign31610_e53607: f64 = (assign31610_e53601 * assign31610_e53606);
        let assign31610_e53610: f64 = (1.0 - p.p1599);
        let assign31610_e53611: f64 = (assign31610_e53607 / assign31610_e53610);
        (assign31610_e53611, ((assign31610_e53601 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31610_e53610), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31610_e53606) + (assign31610_e53601 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31610_e53610), ((assign31610_e53601 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31610_e53610),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31610_e53613;
        locals.var_qedj2_dn3 = assign31610_e53613_d_n3;
        locals.var_qedj2_dn4 = assign31610_e53613_d_n4;
        locals.var_qedj2_dn5 = assign31610_e53613_d_n5;

    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31620_e53646, assign31620_e53646_d_n3, assign31620_e53646_d_n4, assign31620_e53646_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) {
        let assign31620_e53627: f64 = (-locals.var_pbswd_t);
        let assign31620_e53629: f64 = (assign31620_e53627 * locals.var_czbdsw);
        let (assign31620_e53643, assign31620_e53643_d_n3, assign31620_e53643_d_n4, assign31620_e53643_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31620_e53635: f64 = (-87.498233534);
                (assign31620_e53635, 0.0, 0.0, 0.0,)
            } else {
                let (assign31620_e53642, assign31620_e53642_d_n3, assign31620_e53642_d_n4, assign31620_e53642_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31620_e53640: f64 = (locals.var_arg__blk581).ln();
                        (assign31620_e53640, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31620_e53642, assign31620_e53642_d_n3, assign31620_e53642_d_n4, assign31620_e53642_d_n5,)
            }
        };
        let assign31620_e53644: f64 = (assign31620_e53629 * assign31620_e53643);
        (assign31620_e53644, (assign31620_e53629 * assign31620_e53643_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31620_e53627 * locals.var_czbdsw_dn4)) * assign31620_e53643) + (assign31620_e53629 * assign31620_e53643_d_n4)), (assign31620_e53629 * assign31620_e53643_d_n5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31620_e53646;
        locals.var_qedj2_dn3 = assign31620_e53646_d_n3;
        locals.var_qedj2_dn4 = assign31620_e53646_d_n4;
        locals.var_qedj2_dn5 = assign31620_e53646_d_n5;

        let (assign31630_e53663, assign31630_e53663_d_n3, assign31630_e53663_d_n4, assign31630_e53663_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign31630_e53660: f64 = (locals.var_vec2d / locals.var_pbswd_t);
        let assign31630_e53661: f64 = (1.0 - assign31630_e53660);
        (assign31630_e53661, 0.0, (-(((locals.var_vec2d_dn4 * locals.var_pbswd_t) - (locals.var_vec2d * locals.var_pbswd_t_dn4)) / (locals.var_pbswd_t * locals.var_pbswd_t))), 0.0,)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31630_e53663;
        locals.var_arg__blk581_dn3 = assign31630_e53663_d_n3;
        locals.var_arg__blk581_dn4 = assign31630_e53663_d_n4;
        locals.var_arg__blk581_dn5 = assign31630_e53663_d_n5;

        let assign31640_e53666: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign31640_e53666;

        let assign31650_e53669: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign31650_e53669;

        let (assign31660_e53689, assign31660_e53689_d_n3, assign31660_e53689_d_n4, assign31660_e53689_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign31660_e53686: f64 = (locals.var_arg__blk581).sqrt();
        let assign31660_e53687: f64 = (1.0 / assign31660_e53686);
        (assign31660_e53687, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31660_e53689;
        locals.var_sarg__blk582_dn3 = assign31660_e53689_d_n3;
        locals.var_sarg__blk582_dn4 = assign31660_e53689_d_n4;
        locals.var_sarg__blk582_dn5 = assign31660_e53689_d_n5;

        let (assign31670_e53710, assign31670_e53710_d_n3, assign31670_e53710_d_n4, assign31670_e53710_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign31670_e53707: f64 = (-p.p1599);
        let assign31670_e53708: f64 = (locals.var_arg__blk581).powf(assign31670_e53707);
        (assign31670_e53708, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31670_e53710;
        locals.var_sarg__blk582_dn3 = assign31670_e53710_d_n3;
        locals.var_sarg__blk582_dn4 = assign31670_e53710_d_n4;
        locals.var_sarg__blk582_dn5 = assign31670_e53710_d_n5;

        let (assign31680_e53737, assign31680_e53737_d_n3, assign31680_e53737_d_n4, assign31680_e53737_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign31680_e53725: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31680_e53729: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31680_e53730: f64 = (1.0 - assign31680_e53729);
        let assign31680_e53731: f64 = (assign31680_e53725 * assign31680_e53730);
        let assign31680_e53734: f64 = (1.0 - p.p1599);
        let assign31680_e53735: f64 = (assign31680_e53731 / assign31680_e53734);
        (assign31680_e53735, ((assign31680_e53725 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31680_e53734), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31680_e53730) + (assign31680_e53725 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31680_e53734), ((assign31680_e53725 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31680_e53734),)
    } else {
        (locals.var_qec__blk583, locals.var_qec__blk583_dn3, locals.var_qec__blk583_dn4, locals.var_qec__blk583_dn5,)
    }
};
        locals.var_qec__blk583 = assign31680_e53737;
        locals.var_qec__blk583_dn3 = assign31680_e53737_d_n3;
        locals.var_qec__blk583_dn4 = assign31680_e53737_d_n4;
        locals.var_qec__blk583_dn5 = assign31680_e53737_d_n5;

        let (assign31690_e53771, assign31690_e53771_d_n3, assign31690_e53771_d_n4, assign31690_e53771_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 == 0.0)) {
        let assign31690_e53752: f64 = (-locals.var_pbswd_t);
        let assign31690_e53754: f64 = (assign31690_e53752 * locals.var_czbdsw);
        let (assign31690_e53768, assign31690_e53768_d_n3, assign31690_e53768_d_n4, assign31690_e53768_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31690_e53760: f64 = (-87.498233534);
                (assign31690_e53760, 0.0, 0.0, 0.0,)
            } else {
                let (assign31690_e53767, assign31690_e53767_d_n3, assign31690_e53767_d_n4, assign31690_e53767_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31690_e53765: f64 = (locals.var_arg__blk581).ln();
                        (assign31690_e53765, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31690_e53767, assign31690_e53767_d_n3, assign31690_e53767_d_n4, assign31690_e53767_d_n5,)
            }
        };
        let assign31690_e53769: f64 = (assign31690_e53754 * assign31690_e53768);
        (assign31690_e53769, (assign31690_e53754 * assign31690_e53768_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31690_e53752 * locals.var_czbdsw_dn4)) * assign31690_e53768) + (assign31690_e53754 * assign31690_e53768_d_n4)), (assign31690_e53754 * assign31690_e53768_d_n5),)
    } else {
        (locals.var_qec__blk583, locals.var_qec__blk583_dn3, locals.var_qec__blk583_dn4, locals.var_qec__blk583_dn5,)
    }
};
        locals.var_qec__blk583 = assign31690_e53771;
        locals.var_qec__blk583_dn3 = assign31690_e53771_d_n3;
        locals.var_qec__blk583_dn4 = assign31690_e53771_d_n4;
        locals.var_qec__blk583_dn5 = assign31690_e53771_d_n5;

        let (assign31700_e53790, assign31700_e53790_d_n3, assign31700_e53790_d_n4, assign31700_e53790_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign31700_e53785: f64 = (locals.var_ved_jct - locals.var_vec2d);
        let assign31700_e53787: f64 = (assign31700_e53785 / locals.var_pb22d);
        let assign31700_e53788: f64 = (1.0 - assign31700_e53787);
        (assign31700_e53788, (-(locals.var_ved_jct_dn3 / locals.var_pb22d)), (-((((-locals.var_vec2d_dn4) * locals.var_pb22d) - (assign31700_e53785 * locals.var_pb22d_dn4)) / (locals.var_pb22d * locals.var_pb22d))), (-(locals.var_ved_jct_dn5 / locals.var_pb22d)),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31700_e53790;
        locals.var_arg__blk581_dn3 = assign31700_e53790_d_n3;
        locals.var_arg__blk581_dn4 = assign31700_e53790_d_n4;
        locals.var_arg__blk581_dn5 = assign31700_e53790_d_n5;

        let assign31710_e53793: f64 = if p.p1611 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign31710_e53793;

        let assign31720_e53796: f64 = if p.p1611 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign31720_e53796;

        let (assign31730_e53816, assign31730_e53816_d_n3, assign31730_e53816_d_n4, assign31730_e53816_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign31730_e53813: f64 = (locals.var_arg__blk581).sqrt();
        let assign31730_e53814: f64 = (1.0 / assign31730_e53813);
        (assign31730_e53814, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31730_e53816;
        locals.var_sarg__blk582_dn3 = assign31730_e53816_d_n3;
        locals.var_sarg__blk582_dn4 = assign31730_e53816_d_n4;
        locals.var_sarg__blk582_dn5 = assign31730_e53816_d_n5;

        let (assign31740_e53837, assign31740_e53837_d_n3, assign31740_e53837_d_n4, assign31740_e53837_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign31740_e53834: f64 = (-p.p1611);
        let assign31740_e53835: f64 = (locals.var_arg__blk581).powf(assign31740_e53834);
        (assign31740_e53835, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31740_e53837;
        locals.var_sarg__blk582_dn3 = assign31740_e53837_d_n3;
        locals.var_sarg__blk582_dn4 = assign31740_e53837_d_n4;
        locals.var_sarg__blk582_dn5 = assign31740_e53837_d_n5;

        let (assign31750_e53868, assign31750_e53868_d_n3, assign31750_e53868_d_n4, assign31750_e53868_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) {
        let assign31750_e53853: f64 = (p.p1605 * locals.var_pb22d);
        let assign31750_e53855: f64 = (assign31750_e53853 * locals.var_czbdsw);
        let assign31750_e53859: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31750_e53860: f64 = (1.0 - assign31750_e53859);
        let assign31750_e53861: f64 = (assign31750_e53855 * assign31750_e53860);
        let assign31750_e53864: f64 = (1.0 - p.p1611);
        let assign31750_e53865: f64 = (assign31750_e53861 / assign31750_e53864);
        let assign31750_e53866: f64 = (locals.var_qec__blk583 + assign31750_e53865);
        (assign31750_e53866, (locals.var_qec__blk583_dn3 + ((assign31750_e53855 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31750_e53864)), (locals.var_qec__blk583_dn4 + ((((((p.p1605 * locals.var_pb22d_dn4) * locals.var_czbdsw) + (assign31750_e53853 * locals.var_czbdsw_dn4)) * assign31750_e53860) + (assign31750_e53855 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31750_e53864)), (locals.var_qec__blk583_dn5 + ((assign31750_e53855 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31750_e53864)),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31750_e53868;
        locals.var_qedj2_dn3 = assign31750_e53868_d_n3;
        locals.var_qedj2_dn4 = assign31750_e53868_d_n4;
        locals.var_qedj2_dn5 = assign31750_e53868_d_n5;

        let (assign31760_e53905, assign31760_e53905_d_n3, assign31760_e53905_d_n4, assign31760_e53905_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 == 0.0)) {
        let assign31760_e53885: f64 = (p.p1605 * locals.var_pb22d);
        let assign31760_e53887: f64 = (assign31760_e53885 * locals.var_czbdsw);
        let (assign31760_e53901, assign31760_e53901_d_n3, assign31760_e53901_d_n4, assign31760_e53901_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31760_e53893: f64 = (-87.498233534);
                (assign31760_e53893, 0.0, 0.0, 0.0,)
            } else {
                let (assign31760_e53900, assign31760_e53900_d_n3, assign31760_e53900_d_n4, assign31760_e53900_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31760_e53898: f64 = (locals.var_arg__blk581).ln();
                        (assign31760_e53898, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31760_e53900, assign31760_e53900_d_n3, assign31760_e53900_d_n4, assign31760_e53900_d_n5,)
            }
        };
        let assign31760_e53902: f64 = (assign31760_e53887 * assign31760_e53901);
        let assign31760_e53903: f64 = (locals.var_qec__blk583 - assign31760_e53902);
        (assign31760_e53903, (locals.var_qec__blk583_dn3 - (assign31760_e53887 * assign31760_e53901_d_n3)), (locals.var_qec__blk583_dn4 - (((((p.p1605 * locals.var_pb22d_dn4) * locals.var_czbdsw) + (assign31760_e53885 * locals.var_czbdsw_dn4)) * assign31760_e53901) + (assign31760_e53887 * assign31760_e53901_d_n4))), (locals.var_qec__blk583_dn5 - (assign31760_e53887 * assign31760_e53901_d_n5)),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31760_e53905;
        locals.var_qedj2_dn3 = assign31760_e53905_d_n3;
        locals.var_qedj2_dn4 = assign31760_e53905_d_n4;
        locals.var_qedj2_dn5 = assign31760_e53905_d_n5;

        let (assign31770_e53918, assign31770_e53918_d_n3, assign31770_e53918_d_n4, assign31770_e53918_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign31770_e53916: f64 = (1.0 - locals.var_t1__blk576);
        (assign31770_e53916, (-locals.var_t1__blk576_dn3), (-locals.var_t1__blk576_dn4), (-locals.var_t1__blk576_dn5),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31770_e53918;
        locals.var_arg__blk581_dn3 = assign31770_e53918_d_n3;
        locals.var_arg__blk581_dn4 = assign31770_e53918_d_n4;
        locals.var_arg__blk581_dn5 = assign31770_e53918_d_n5;

        let assign31780_e53921: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign31780_e53921;

        let assign31790_e53924: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign31790_e53924;

        let (assign31800_e53942, assign31800_e53942_d_n3, assign31800_e53942_d_n4, assign31800_e53942_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign31800_e53939: f64 = (locals.var_arg__blk581).sqrt();
        let assign31800_e53940: f64 = (1.0 / assign31800_e53939);
        (assign31800_e53940, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31800_e53942;
        locals.var_sarg__blk582_dn3 = assign31800_e53942_d_n3;
        locals.var_sarg__blk582_dn4 = assign31800_e53942_d_n4;
        locals.var_sarg__blk582_dn5 = assign31800_e53942_d_n5;

        let (assign31810_e53961, assign31810_e53961_d_n3, assign31810_e53961_d_n4, assign31810_e53961_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
        let assign31810_e53958: f64 = (-p.p1599);
        let assign31810_e53959: f64 = (locals.var_arg__blk581).powf(assign31810_e53958);
        (assign31810_e53959, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31810_e53961;
        locals.var_sarg__blk582_dn3 = assign31810_e53961_d_n3;
        locals.var_sarg__blk582_dn4 = assign31810_e53961_d_n4;
        locals.var_sarg__blk582_dn5 = assign31810_e53961_d_n5;

        let (assign31820_e53986, assign31820_e53986_d_n3, assign31820_e53986_d_n4, assign31820_e53986_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) {
        let assign31820_e53974: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31820_e53978: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31820_e53979: f64 = (1.0 - assign31820_e53978);
        let assign31820_e53980: f64 = (assign31820_e53974 * assign31820_e53979);
        let assign31820_e53983: f64 = (1.0 - p.p1599);
        let assign31820_e53984: f64 = (assign31820_e53980 / assign31820_e53983);
        (assign31820_e53984, ((assign31820_e53974 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31820_e53983), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31820_e53979) + (assign31820_e53974 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31820_e53983), ((assign31820_e53974 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31820_e53983),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31820_e53986;
        locals.var_qedj2_dn3 = assign31820_e53986_d_n3;
        locals.var_qedj2_dn4 = assign31820_e53986_d_n4;
        locals.var_qedj2_dn5 = assign31820_e53986_d_n5;

        let (assign31830_e54018, assign31830_e54018_d_n3, assign31830_e54018_d_n4, assign31830_e54018_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 == 0.0)) {
        let assign31830_e53999: f64 = (-locals.var_pbswd_t);
        let assign31830_e54001: f64 = (assign31830_e53999 * locals.var_czbdsw);
        let (assign31830_e54015, assign31830_e54015_d_n3, assign31830_e54015_d_n4, assign31830_e54015_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31830_e54007: f64 = (-87.498233534);
                (assign31830_e54007, 0.0, 0.0, 0.0,)
            } else {
                let (assign31830_e54014, assign31830_e54014_d_n3, assign31830_e54014_d_n4, assign31830_e54014_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31830_e54012: f64 = (locals.var_arg__blk581).ln();
                        (assign31830_e54012, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31830_e54014, assign31830_e54014_d_n3, assign31830_e54014_d_n4, assign31830_e54014_d_n5,)
            }
        };
        let assign31830_e54016: f64 = (assign31830_e54001 * assign31830_e54015);
        (assign31830_e54016, (assign31830_e54001 * assign31830_e54015_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31830_e53999 * locals.var_czbdsw_dn4)) * assign31830_e54015) + (assign31830_e54001 * assign31830_e54015_d_n4)), (assign31830_e54001 * assign31830_e54015_d_n5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31830_e54018;
        locals.var_qedj2_dn3 = assign31830_e54018_d_n3;
        locals.var_qedj2_dn4 = assign31830_e54018_d_n4;
        locals.var_qedj2_dn5 = assign31830_e54018_d_n5;

        let assign31840_e54021: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign31840_e54021;

        let assign31850_e54024: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign31850_e54024;

        let (assign31860_e54040,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign31860_e54037: f64 = (0.1_f64).sqrt();
        let assign31860_e54038: f64 = (1.0 / assign31860_e54037);
        (assign31860_e54038,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31860_e54040;

        let (assign31870_e54057,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign31870_e54054: f64 = (-p.p1599);
        let assign31870_e54055: f64 = (0.1_f64).powf(assign31870_e54054);
        (assign31870_e54055,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31870_e54057;

        let (assign31880_e54072,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign31880_e54069: f64 = (1.0 - p.p1599);
        let assign31880_e54070: f64 = (1.0 / assign31880_e54069);
        (assign31880_e54070,)
    } else {
        (locals.var_t3__blk578,)
    }
};
        locals.var_t3__blk578 = assign31880_e54072;

        let (assign31890_e54095,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign31890_e54085: f64 = (0.05 * p.p1599);
        let assign31890_e54088: f64 = (1.0 + p.p1599);
        let assign31890_e54089: f64 = (assign31890_e54085 * assign31890_e54088);
        let assign31890_e54091: f64 = (assign31890_e54089 * locals.var_t2__blk577);
        let assign31890_e54092: f64 = (1.0 - assign31890_e54091);
        let assign31890_e54093: f64 = (locals.var_t3__blk578 * assign31890_e54092);
        (assign31890_e54093,)
    } else {
        (locals.var_t5__blk580,)
    }
};
        locals.var_t5__blk580 = assign31890_e54095;

        let (assign31900_e54107,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31900_e54107;

        let (assign31910_e54122,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 == 0.0)) {
        let assign31910_e54119: f64 = (0.1_f64).ln();
        let assign31910_e54120: f64 = (1.5 - assign31910_e54119);
        (assign31910_e54120,)
    } else {
        (locals.var_t5__blk580,)
    }
};
        locals.var_t5__blk580 = assign31910_e54122;

        let (assign31920_e54147, assign31920_e54147_d_n3, assign31920_e54147_d_n4, assign31920_e54147_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign31920_e54132: f64 = (locals.var_t1__blk576 - 1.0);
        let assign31920_e54133: f64 = (locals.var_t2__blk577 * assign31920_e54132);
        let assign31920_e54136: f64 = (5.0 * p.p1599);
        let assign31920_e54139: f64 = (locals.var_t1__blk576 - 1.0);
        let assign31920_e54140: f64 = (assign31920_e54136 * assign31920_e54139);
        let assign31920_e54143: f64 = (1.0 + p.p1599);
        let assign31920_e54144: f64 = (assign31920_e54140 + assign31920_e54143);
        let assign31920_e54145: f64 = (assign31920_e54133 * assign31920_e54144);
        (assign31920_e54145, (((locals.var_t2__blk577 * locals.var_t1__blk576_dn3) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn3))), (((locals.var_t2__blk577 * locals.var_t1__blk576_dn4) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn4))), (((locals.var_t2__blk577 * locals.var_t1__blk576_dn5) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn5))),)
    } else {
        (locals.var_t4__blk579, locals.var_t4__blk579_dn3, locals.var_t4__blk579_dn4, locals.var_t4__blk579_dn5,)
    }
};
        locals.var_t4__blk579 = assign31920_e54147;
        locals.var_t4__blk579_dn3 = assign31920_e54147_d_n3;
        locals.var_t4__blk579_dn4 = assign31920_e54147_d_n4;
        locals.var_t4__blk579_dn5 = assign31920_e54147_d_n5;

        let (assign31930_e54162, assign31930_e54162_d_n3, assign31930_e54162_d_n4, assign31930_e54162_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign31930_e54156: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31930_e54159: f64 = (locals.var_t4__blk579 + locals.var_t5__blk580);
        let assign31930_e54160: f64 = (assign31930_e54156 * assign31930_e54159);
        (assign31930_e54160, (assign31930_e54156 * locals.var_t4__blk579_dn3), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31930_e54159) + (assign31930_e54156 * locals.var_t4__blk579_dn4)), (assign31930_e54156 * locals.var_t4__blk579_dn5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31930_e54162;
        locals.var_qedj2_dn3 = assign31930_e54162_d_n3;
        locals.var_qedj2_dn4 = assign31930_e54162_d_n4;
        locals.var_qedj2_dn5 = assign31930_e54162_d_n5;

        let (assign31940_e54169, assign31940_e54169_d_n3, assign31940_e54169_d_n4, assign31940_e54169_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard584 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31940_e54169;
        locals.var_qedj2_dn3 = assign31940_e54169_d_n3;
        locals.var_qedj2_dn4 = assign31940_e54169_d_n4;
        locals.var_qedj2_dn5 = assign31940_e54169_d_n5;

        let assign31950_e54172: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign31950_e54172;

        let (assign31960_e54180, assign31960_e54180_d_n3, assign31960_e54180_d_n4, assign31960_e54180_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) {
        let assign31960_e54178: f64 = (locals.var_ved_jct / locals.var_pbswgd_t);
        (assign31960_e54178, (locals.var_ved_jct_dn3 / locals.var_pbswgd_t), (-((locals.var_ved_jct * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_ved_jct_dn5 / locals.var_pbswgd_t),)
    } else {
        (locals.var_t1__blk598, locals.var_t1__blk598_dn3, locals.var_t1__blk598_dn4, locals.var_t1__blk598_dn5,)
    }
};
        locals.var_t1__blk598 = assign31960_e54180;
        locals.var_t1__blk598_dn3 = assign31960_e54180_d_n3;
        locals.var_t1__blk598_dn4 = assign31960_e54180_d_n4;
        locals.var_t1__blk598_dn5 = assign31960_e54180_d_n5;

        let assign31970_e54183: f64 = if locals.var_t1__blk598 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign31970_e54183;

        let assign31980_e54186: f64 = if p.p1607 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign31980_e54186;

        let assign31990_e54189: f64 = if locals.var_ved_jct > locals.var_vec3d { 1.0 } else { 0.0 };
        locals.var_guard609 = assign31990_e54189;

        let (assign32000_e54203, assign32000_e54203_d_n3, assign32000_e54203_d_n4, assign32000_e54203_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) {
        let assign32000_e54201: f64 = (1.0 - locals.var_t1__blk598);
        (assign32000_e54201, (-locals.var_t1__blk598_dn3), (-locals.var_t1__blk598_dn4), (-locals.var_t1__blk598_dn5),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32000_e54203;
        locals.var_arg__blk603_dn3 = assign32000_e54203_d_n3;
        locals.var_arg__blk603_dn4 = assign32000_e54203_d_n4;
        locals.var_arg__blk603_dn5 = assign32000_e54203_d_n5;

        let assign32010_e54206: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign32010_e54206;

        let assign32020_e54209: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign32020_e54209;

        let (assign32030_e54228, assign32030_e54228_d_n3, assign32030_e54228_d_n4, assign32030_e54228_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign32030_e54225: f64 = (locals.var_arg__blk603).sqrt();
        let assign32030_e54226: f64 = (1.0 / assign32030_e54225);
        (assign32030_e54226, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32030_e54228;
        locals.var_sarg__blk604_dn3 = assign32030_e54228_d_n3;
        locals.var_sarg__blk604_dn4 = assign32030_e54228_d_n4;
        locals.var_sarg__blk604_dn5 = assign32030_e54228_d_n5;

    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32040_e54248, assign32040_e54248_d_n3, assign32040_e54248_d_n4, assign32040_e54248_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) && (locals.var_guard611 == 0.0)) {
        let assign32040_e54245: f64 = (-p.p1601);
        let assign32040_e54246: f64 = (locals.var_arg__blk603).powf(assign32040_e54245);
        (assign32040_e54246, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32040_e54248;
        locals.var_sarg__blk604_dn3 = assign32040_e54248_d_n3;
        locals.var_sarg__blk604_dn4 = assign32040_e54248_d_n4;
        locals.var_sarg__blk604_dn5 = assign32040_e54248_d_n5;

        let (assign32050_e54274, assign32050_e54274_d_n3, assign32050_e54274_d_n4, assign32050_e54274_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) {
        let assign32050_e54262: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32050_e54266: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32050_e54267: f64 = (1.0 - assign32050_e54266);
        let assign32050_e54268: f64 = (assign32050_e54262 * assign32050_e54267);
        let assign32050_e54271: f64 = (1.0 - p.p1601);
        let assign32050_e54272: f64 = (assign32050_e54268 / assign32050_e54271);
        (assign32050_e54272, ((assign32050_e54262 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32050_e54271), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32050_e54267) + (assign32050_e54262 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32050_e54271), ((assign32050_e54262 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32050_e54271),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32050_e54274;
        locals.var_qedj3_dn3 = assign32050_e54274_d_n3;
        locals.var_qedj3_dn4 = assign32050_e54274_d_n4;
        locals.var_qedj3_dn5 = assign32050_e54274_d_n5;

        let (assign32060_e54307, assign32060_e54307_d_n3, assign32060_e54307_d_n4, assign32060_e54307_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 == 0.0)) {
        let assign32060_e54288: f64 = (-locals.var_pbswgd_t);
        let assign32060_e54290: f64 = (assign32060_e54288 * locals.var_czbdswg);
        let (assign32060_e54304, assign32060_e54304_d_n3, assign32060_e54304_d_n4, assign32060_e54304_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32060_e54296: f64 = (-87.498233534);
                (assign32060_e54296, 0.0, 0.0, 0.0,)
            } else {
                let (assign32060_e54303, assign32060_e54303_d_n3, assign32060_e54303_d_n4, assign32060_e54303_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32060_e54301: f64 = (locals.var_arg__blk603).ln();
                        (assign32060_e54301, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32060_e54303, assign32060_e54303_d_n3, assign32060_e54303_d_n4, assign32060_e54303_d_n5,)
            }
        };
        let assign32060_e54305: f64 = (assign32060_e54290 * assign32060_e54304);
        (assign32060_e54305, (assign32060_e54290 * assign32060_e54304_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32060_e54288 * locals.var_czbdswg_dn4)) * assign32060_e54304) + (assign32060_e54290 * assign32060_e54304_d_n4)), (assign32060_e54290 * assign32060_e54304_d_n5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32060_e54307;
        locals.var_qedj3_dn3 = assign32060_e54307_d_n3;
        locals.var_qedj3_dn4 = assign32060_e54307_d_n4;
        locals.var_qedj3_dn5 = assign32060_e54307_d_n5;

        let (assign32070_e54324, assign32070_e54324_d_n3, assign32070_e54324_d_n4, assign32070_e54324_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) {
        let assign32070_e54321: f64 = (locals.var_vec3d / locals.var_pbswgd_t);
        let assign32070_e54322: f64 = (1.0 - assign32070_e54321);
        (assign32070_e54322, 0.0, (-(((locals.var_vec3d_dn4 * locals.var_pbswgd_t) - (locals.var_vec3d * locals.var_pbswgd_t_dn4)) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), 0.0,)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32070_e54324;
        locals.var_arg__blk603_dn3 = assign32070_e54324_d_n3;
        locals.var_arg__blk603_dn4 = assign32070_e54324_d_n4;
        locals.var_arg__blk603_dn5 = assign32070_e54324_d_n5;

        let assign32080_e54327: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign32080_e54327;

        let assign32090_e54330: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign32090_e54330;

        let (assign32100_e54350, assign32100_e54350_d_n3, assign32100_e54350_d_n4, assign32100_e54350_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) && (locals.var_guard613 != 0.0)) {
        let assign32100_e54347: f64 = (locals.var_arg__blk603).sqrt();
        let assign32100_e54348: f64 = (1.0 / assign32100_e54347);
        (assign32100_e54348, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32100_e54350;
        locals.var_sarg__blk604_dn3 = assign32100_e54350_d_n3;
        locals.var_sarg__blk604_dn4 = assign32100_e54350_d_n4;
        locals.var_sarg__blk604_dn5 = assign32100_e54350_d_n5;

        let (assign32110_e54371, assign32110_e54371_d_n3, assign32110_e54371_d_n4, assign32110_e54371_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) && (locals.var_guard613 == 0.0)) {
        let assign32110_e54368: f64 = (-p.p1601);
        let assign32110_e54369: f64 = (locals.var_arg__blk603).powf(assign32110_e54368);
        (assign32110_e54369, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32110_e54371;
        locals.var_sarg__blk604_dn3 = assign32110_e54371_d_n3;
        locals.var_sarg__blk604_dn4 = assign32110_e54371_d_n4;
        locals.var_sarg__blk604_dn5 = assign32110_e54371_d_n5;

        let (assign32120_e54398, assign32120_e54398_d_n3, assign32120_e54398_d_n4, assign32120_e54398_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign32120_e54386: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32120_e54390: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32120_e54391: f64 = (1.0 - assign32120_e54390);
        let assign32120_e54392: f64 = (assign32120_e54386 * assign32120_e54391);
        let assign32120_e54395: f64 = (1.0 - p.p1601);
        let assign32120_e54396: f64 = (assign32120_e54392 / assign32120_e54395);
        (assign32120_e54396, ((assign32120_e54386 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32120_e54395), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32120_e54391) + (assign32120_e54386 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32120_e54395), ((assign32120_e54386 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32120_e54395),)
    } else {
        (locals.var_qec__blk605, locals.var_qec__blk605_dn3, locals.var_qec__blk605_dn4, locals.var_qec__blk605_dn5,)
    }
};
        locals.var_qec__blk605 = assign32120_e54398;
        locals.var_qec__blk605_dn3 = assign32120_e54398_d_n3;
        locals.var_qec__blk605_dn4 = assign32120_e54398_d_n4;
        locals.var_qec__blk605_dn5 = assign32120_e54398_d_n5;

        let (assign32130_e54432, assign32130_e54432_d_n3, assign32130_e54432_d_n4, assign32130_e54432_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 == 0.0)) {
        let assign32130_e54413: f64 = (-locals.var_pbswgd_t);
        let assign32130_e54415: f64 = (assign32130_e54413 * locals.var_czbdswg);
        let (assign32130_e54429, assign32130_e54429_d_n3, assign32130_e54429_d_n4, assign32130_e54429_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32130_e54421: f64 = (-87.498233534);
                (assign32130_e54421, 0.0, 0.0, 0.0,)
            } else {
                let (assign32130_e54428, assign32130_e54428_d_n3, assign32130_e54428_d_n4, assign32130_e54428_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32130_e54426: f64 = (locals.var_arg__blk603).ln();
                        (assign32130_e54426, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32130_e54428, assign32130_e54428_d_n3, assign32130_e54428_d_n4, assign32130_e54428_d_n5,)
            }
        };
        let assign32130_e54430: f64 = (assign32130_e54415 * assign32130_e54429);
        (assign32130_e54430, (assign32130_e54415 * assign32130_e54429_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32130_e54413 * locals.var_czbdswg_dn4)) * assign32130_e54429) + (assign32130_e54415 * assign32130_e54429_d_n4)), (assign32130_e54415 * assign32130_e54429_d_n5),)
    } else {
        (locals.var_qec__blk605, locals.var_qec__blk605_dn3, locals.var_qec__blk605_dn4, locals.var_qec__blk605_dn5,)
    }
};
        locals.var_qec__blk605 = assign32130_e54432;
        locals.var_qec__blk605_dn3 = assign32130_e54432_d_n3;
        locals.var_qec__blk605_dn4 = assign32130_e54432_d_n4;
        locals.var_qec__blk605_dn5 = assign32130_e54432_d_n5;

        let (assign32140_e54451, assign32140_e54451_d_n3, assign32140_e54451_d_n4, assign32140_e54451_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) {
        let assign32140_e54446: f64 = (locals.var_ved_jct - locals.var_vec3d);
        let assign32140_e54448: f64 = (assign32140_e54446 / locals.var_pb23d);
        let assign32140_e54449: f64 = (1.0 - assign32140_e54448);
        (assign32140_e54449, (-(locals.var_ved_jct_dn3 / locals.var_pb23d)), (-((((-locals.var_vec3d_dn4) * locals.var_pb23d) - (assign32140_e54446 * locals.var_pb23d_dn4)) / (locals.var_pb23d * locals.var_pb23d))), (-(locals.var_ved_jct_dn5 / locals.var_pb23d)),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32140_e54451;
        locals.var_arg__blk603_dn3 = assign32140_e54451_d_n3;
        locals.var_arg__blk603_dn4 = assign32140_e54451_d_n4;
        locals.var_arg__blk603_dn5 = assign32140_e54451_d_n5;

        let assign32150_e54454: f64 = if p.p1613 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign32150_e54454;

        let assign32160_e54457: f64 = if p.p1613 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign32160_e54457;

        let (assign32170_e54477, assign32170_e54477_d_n3, assign32170_e54477_d_n4, assign32170_e54477_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign32170_e54474: f64 = (locals.var_arg__blk603).sqrt();
        let assign32170_e54475: f64 = (1.0 / assign32170_e54474);
        (assign32170_e54475, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32170_e54477;
        locals.var_sarg__blk604_dn3 = assign32170_e54477_d_n3;
        locals.var_sarg__blk604_dn4 = assign32170_e54477_d_n4;
        locals.var_sarg__blk604_dn5 = assign32170_e54477_d_n5;

        let (assign32180_e54498, assign32180_e54498_d_n3, assign32180_e54498_d_n4, assign32180_e54498_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign32180_e54495: f64 = (-p.p1613);
        let assign32180_e54496: f64 = (locals.var_arg__blk603).powf(assign32180_e54495);
        (assign32180_e54496, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32180_e54498;
        locals.var_sarg__blk604_dn3 = assign32180_e54498_d_n3;
        locals.var_sarg__blk604_dn4 = assign32180_e54498_d_n4;
        locals.var_sarg__blk604_dn5 = assign32180_e54498_d_n5;

        let (assign32190_e54529, assign32190_e54529_d_n3, assign32190_e54529_d_n4, assign32190_e54529_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) {
        let assign32190_e54514: f64 = (p.p1607 * locals.var_pb23d);
        let assign32190_e54516: f64 = (assign32190_e54514 * locals.var_czbdswg);
        let assign32190_e54520: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32190_e54521: f64 = (1.0 - assign32190_e54520);
        let assign32190_e54522: f64 = (assign32190_e54516 * assign32190_e54521);
        let assign32190_e54525: f64 = (1.0 - p.p1613);
        let assign32190_e54526: f64 = (assign32190_e54522 / assign32190_e54525);
        let assign32190_e54527: f64 = (locals.var_qec__blk605 + assign32190_e54526);
        (assign32190_e54527, (locals.var_qec__blk605_dn3 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32190_e54525)), (locals.var_qec__blk605_dn4 + ((((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32190_e54514 * locals.var_czbdswg_dn4)) * assign32190_e54521) + (assign32190_e54516 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32190_e54525)), (locals.var_qec__blk605_dn5 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32190_e54525)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32190_e54529;
        locals.var_qedj3_dn3 = assign32190_e54529_d_n3;
        locals.var_qedj3_dn4 = assign32190_e54529_d_n4;
        locals.var_qedj3_dn5 = assign32190_e54529_d_n5;

        let (assign32200_e54566, assign32200_e54566_d_n3, assign32200_e54566_d_n4, assign32200_e54566_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 == 0.0)) {
        let assign32200_e54546: f64 = (p.p1607 * locals.var_pb23d);
        let assign32200_e54548: f64 = (assign32200_e54546 * locals.var_czbdswg);
        let (assign32200_e54562, assign32200_e54562_d_n3, assign32200_e54562_d_n4, assign32200_e54562_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32200_e54554: f64 = (-87.498233534);
                (assign32200_e54554, 0.0, 0.0, 0.0,)
            } else {
                let (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32200_e54559: f64 = (locals.var_arg__blk603).ln();
                        (assign32200_e54559, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,)
            }
        };
        let assign32200_e54563: f64 = (assign32200_e54548 * assign32200_e54562);
        let assign32200_e54564: f64 = (locals.var_qec__blk605 - assign32200_e54563);
        (assign32200_e54564, (locals.var_qec__blk605_dn3 - (assign32200_e54548 * assign32200_e54562_d_n3)), (locals.var_qec__blk605_dn4 - (((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32200_e54546 * locals.var_czbdswg_dn4)) * assign32200_e54562) + (assign32200_e54548 * assign32200_e54562_d_n4))), (locals.var_qec__blk605_dn5 - (assign32200_e54548 * assign32200_e54562_d_n5)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32200_e54566;
        locals.var_qedj3_dn3 = assign32200_e54566_d_n3;
        locals.var_qedj3_dn4 = assign32200_e54566_d_n4;
        locals.var_qedj3_dn5 = assign32200_e54566_d_n5;

        let (assign32210_e54579, assign32210_e54579_d_n3, assign32210_e54579_d_n4, assign32210_e54579_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) {
        let assign32210_e54577: f64 = (1.0 - locals.var_t1__blk598);
        (assign32210_e54577, (-locals.var_t1__blk598_dn3), (-locals.var_t1__blk598_dn4), (-locals.var_t1__blk598_dn5),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32210_e54579;
        locals.var_arg__blk603_dn3 = assign32210_e54579_d_n3;
        locals.var_arg__blk603_dn4 = assign32210_e54579_d_n4;
        locals.var_arg__blk603_dn5 = assign32210_e54579_d_n5;

        let assign32220_e54582: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign32220_e54582;

        let assign32230_e54585: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign32230_e54585;

        let (assign32240_e54603, assign32240_e54603_d_n3, assign32240_e54603_d_n4, assign32240_e54603_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 != 0.0)) {
        let assign32240_e54600: f64 = (locals.var_arg__blk603).sqrt();
        let assign32240_e54601: f64 = (1.0 / assign32240_e54600);
        (assign32240_e54601, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32240_e54603;
        locals.var_sarg__blk604_dn3 = assign32240_e54603_d_n3;
        locals.var_sarg__blk604_dn4 = assign32240_e54603_d_n4;
        locals.var_sarg__blk604_dn5 = assign32240_e54603_d_n5;

        let (assign32250_e54622, assign32250_e54622_d_n3, assign32250_e54622_d_n4, assign32250_e54622_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 == 0.0)) {
        let assign32250_e54619: f64 = (-p.p1601);
        let assign32250_e54620: f64 = (locals.var_arg__blk603).powf(assign32250_e54619);
        (assign32250_e54620, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32250_e54622;
        locals.var_sarg__blk604_dn3 = assign32250_e54622_d_n3;
        locals.var_sarg__blk604_dn4 = assign32250_e54622_d_n4;
        locals.var_sarg__blk604_dn5 = assign32250_e54622_d_n5;

        let (assign32260_e54647, assign32260_e54647_d_n3, assign32260_e54647_d_n4, assign32260_e54647_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign32260_e54635: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32260_e54639: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32260_e54640: f64 = (1.0 - assign32260_e54639);
        let assign32260_e54641: f64 = (assign32260_e54635 * assign32260_e54640);
        let assign32260_e54644: f64 = (1.0 - p.p1601);
        let assign32260_e54645: f64 = (assign32260_e54641 / assign32260_e54644);
        (assign32260_e54645, ((assign32260_e54635 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32260_e54644), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32260_e54640) + (assign32260_e54635 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32260_e54644), ((assign32260_e54635 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32260_e54644),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32260_e54647;
        locals.var_qedj3_dn3 = assign32260_e54647_d_n3;
        locals.var_qedj3_dn4 = assign32260_e54647_d_n4;
        locals.var_qedj3_dn5 = assign32260_e54647_d_n5;

        let (assign32270_e54679, assign32270_e54679_d_n3, assign32270_e54679_d_n4, assign32270_e54679_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign32270_e54660: f64 = (-locals.var_pbswgd_t);
        let assign32270_e54662: f64 = (assign32270_e54660 * locals.var_czbdswg);
        let (assign32270_e54676, assign32270_e54676_d_n3, assign32270_e54676_d_n4, assign32270_e54676_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32270_e54668: f64 = (-87.498233534);
                (assign32270_e54668, 0.0, 0.0, 0.0,)
            } else {
                let (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32270_e54673: f64 = (locals.var_arg__blk603).ln();
                        (assign32270_e54673, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,)
            }
        };
        let assign32270_e54677: f64 = (assign32270_e54662 * assign32270_e54676);
        (assign32270_e54677, (assign32270_e54662 * assign32270_e54676_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32270_e54660 * locals.var_czbdswg_dn4)) * assign32270_e54676) + (assign32270_e54662 * assign32270_e54676_d_n4)), (assign32270_e54662 * assign32270_e54676_d_n5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32270_e54679;
        locals.var_qedj3_dn3 = assign32270_e54679_d_n3;
        locals.var_qedj3_dn4 = assign32270_e54679_d_n4;
        locals.var_qedj3_dn5 = assign32270_e54679_d_n5;

        let assign32280_e54682: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign32280_e54682;

        let assign32290_e54685: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign32290_e54685;

        let (assign32300_e54701,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign32300_e54698: f64 = (0.1_f64).sqrt();
        let assign32300_e54699: f64 = (1.0 / assign32300_e54698);
        (assign32300_e54699,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32300_e54701;

        let (assign32310_e54718,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) {
        let assign32310_e54715: f64 = (-p.p1601);
        let assign32310_e54716: f64 = (0.1_f64).powf(assign32310_e54715);
        (assign32310_e54716,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32310_e54718;

        let (assign32320_e54733,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32320_e54730: f64 = (1.0 - p.p1601);
        let assign32320_e54731: f64 = (1.0 / assign32320_e54730);
        (assign32320_e54731,)
    } else {
        (locals.var_t3__blk600,)
    }
};
        locals.var_t3__blk600 = assign32320_e54733;

        let (assign32330_e54756,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32330_e54746: f64 = (0.05 * p.p1601);
        let assign32330_e54749: f64 = (1.0 + p.p1601);
        let assign32330_e54750: f64 = (assign32330_e54746 * assign32330_e54749);
        let assign32330_e54752: f64 = (assign32330_e54750 * locals.var_t2__blk599);
        let assign32330_e54753: f64 = (1.0 - assign32330_e54752);
        let assign32330_e54754: f64 = (locals.var_t3__blk600 * assign32330_e54753);
        (assign32330_e54754,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32330_e54756;

        let (assign32340_e54768,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32340_e54768;

        let (assign32350_e54783,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign32350_e54780: f64 = (0.1_f64).ln();
        let assign32350_e54781: f64 = (1.5 - assign32350_e54780);
        (assign32350_e54781,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32350_e54783;

        let (assign32360_e54808, assign32360_e54808_d_n3, assign32360_e54808_d_n4, assign32360_e54808_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32360_e54793: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54794: f64 = (locals.var_t2__blk599 * assign32360_e54793);
        let assign32360_e54797: f64 = (5.0 * p.p1601);
        let assign32360_e54800: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54801: f64 = (assign32360_e54797 * assign32360_e54800);
        let assign32360_e54804: f64 = (1.0 + p.p1601);
        let assign32360_e54805: f64 = (assign32360_e54801 + assign32360_e54804);
        let assign32360_e54806: f64 = (assign32360_e54794 * assign32360_e54805);
        (assign32360_e54806, (((locals.var_t2__blk599 * locals.var_t1__blk598_dn3) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn3))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn4) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn4))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn5) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn5))),)
    } else {
        (locals.var_t4__blk601, locals.var_t4__blk601_dn3, locals.var_t4__blk601_dn4, locals.var_t4__blk601_dn5,)
    }
};
        locals.var_t4__blk601 = assign32360_e54808;
        locals.var_t4__blk601_dn3 = assign32360_e54808_d_n3;
        locals.var_t4__blk601_dn4 = assign32360_e54808_d_n4;
        locals.var_t4__blk601_dn5 = assign32360_e54808_d_n5;

        let (assign32370_e54823, assign32370_e54823_d_n3, assign32370_e54823_d_n4, assign32370_e54823_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32370_e54817: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32370_e54820: f64 = (locals.var_t4__blk601 + locals.var_t5__blk602);
        let assign32370_e54821: f64 = (assign32370_e54817 * assign32370_e54820);
        (assign32370_e54821, (assign32370_e54817 * locals.var_t4__blk601_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32370_e54820) + (assign32370_e54817 * locals.var_t4__blk601_dn4)), (assign32370_e54817 * locals.var_t4__blk601_dn5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32370_e54823;
        locals.var_qedj3_dn3 = assign32370_e54823_d_n3;
        locals.var_qedj3_dn4 = assign32370_e54823_d_n4;
        locals.var_qedj3_dn5 = assign32370_e54823_d_n5;

        let (assign32380_e54830, assign32380_e54830_d_n3, assign32380_e54830_d_n4, assign32380_e54830_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard606 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32380_e54830;
        locals.var_qedj3_dn3 = assign32380_e54830_d_n3;
        locals.var_qedj3_dn4 = assign32380_e54830_d_n4;
        locals.var_qedj3_dn5 = assign32380_e54830_d_n5;

        let (assign32390_e54838, assign32390_e54838_d_n3, assign32390_e54838_d_n4, assign32390_e54838_d_n5,) = {
    if (locals.var_guard469 != 0.0) {
        let assign32390_e54834: f64 = (locals.var_qedj1 + locals.var_qedj2);
        let assign32390_e54836: f64 = (assign32390_e54834 + locals.var_qedj3);
        (assign32390_e54836, ((locals.var_qedj1_dn3 + locals.var_qedj2_dn3) + locals.var_qedj3_dn3), ((locals.var_qedj1_dn4 + locals.var_qedj2_dn4) + locals.var_qedj3_dn4), ((locals.var_qedj1_dn5 + locals.var_qedj2_dn5) + locals.var_qedj3_dn5),)
    } else {
        (locals.var_qedj, locals.var_qedj_dn3, locals.var_qedj_dn4, locals.var_qedj_dn5,)
    }
};
        locals.var_qedj = assign32390_e54838;
        locals.var_qedj_dn3 = assign32390_e54838_d_n3;
        locals.var_qedj_dn4 = assign32390_e54838_d_n4;
        locals.var_qedj_dn5 = assign32390_e54838_d_n5;

        let assign32400_e54842: f64 = (locals.var_csbox * locals.var_ves_jct);
        let assign32400_e54843: f64 = (locals.var_qesj + assign32400_e54842);
        locals.var_qes = assign32400_e54843;
        locals.var_qes_dn0 = (locals.var_csbox_dn0 * locals.var_ves_jct);
        locals.var_qes_dn2 = (locals.var_csbox_dn2 * locals.var_ves_jct);
        locals.var_qes_dn3 = (locals.var_qesj_dn3 + ((locals.var_csbox_dn3 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn3)));
        locals.var_qes_dn4 = (locals.var_qesj_dn4 + (locals.var_csbox_dn4 * locals.var_ves_jct));
        locals.var_qes_dn5 = (locals.var_csbox_dn5 * locals.var_ves_jct);
        locals.var_qes_dn6 = (locals.var_qesj_dn6 + ((locals.var_csbox_dn6 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn6)));
        locals.var_qes_dn7 = (locals.var_csbox_dn7 * locals.var_ves_jct);
        locals.var_qes_dn8 = (locals.var_csbox_dn8 * locals.var_ves_jct);
        locals.var_qes_dn9 = (locals.var_csbox_dn9 * locals.var_ves_jct);
        locals.var_qes_dn10 = (locals.var_csbox_dn10 * locals.var_ves_jct);
        locals.var_qes_dn11 = (locals.var_csbox_dn11 * locals.var_ves_jct);
        locals.var_qes_dn13 = (locals.var_csbox_dn13 * locals.var_ves_jct);
        locals.var_qes_dn14 = (locals.var_csbox_dn14 * locals.var_ves_jct);

    }

    pub(super) fn stamp_transient_block_121(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let assign32410_e54847: f64 = (locals.var_cdbox * locals.var_ved_jct);
        let assign32410_e54848: f64 = (locals.var_qedj + assign32410_e54847);
        locals.var_qed = assign32410_e54848;
        locals.var_qed_dn0 = (locals.var_cdbox_dn0 * locals.var_ved_jct);
        locals.var_qed_dn2 = (locals.var_cdbox_dn2 * locals.var_ved_jct);
        locals.var_qed_dn3 = (locals.var_qedj_dn3 + ((locals.var_cdbox_dn3 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn3)));
        locals.var_qed_dn4 = (locals.var_qedj_dn4 + (locals.var_cdbox_dn4 * locals.var_ved_jct));
        locals.var_qed_dn5 = (locals.var_qedj_dn5 + ((locals.var_cdbox_dn5 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn5)));
        locals.var_qed_dn6 = (locals.var_cdbox_dn6 * locals.var_ved_jct);
        locals.var_qed_dn7 = (locals.var_cdbox_dn7 * locals.var_ved_jct);
        locals.var_qed_dn8 = (locals.var_cdbox_dn8 * locals.var_ved_jct);
        locals.var_qed_dn9 = (locals.var_cdbox_dn9 * locals.var_ved_jct);
        locals.var_qed_dn10 = (locals.var_cdbox_dn10 * locals.var_ved_jct);
        locals.var_qed_dn11 = (locals.var_cdbox_dn11 * locals.var_ved_jct);
        locals.var_qed_dn13 = (locals.var_cdbox_dn13 * locals.var_ved_jct);
        locals.var_qed_dn14 = (locals.var_cdbox_dn14 * locals.var_ved_jct);

        let assign32420_e54851: f64 = (locals.var_cgbox * locals.var_devsign);
        let assign32420_e54853: f64 = (assign32420_e54851 * (nv3 - nv10));
        locals.var_qeg = assign32420_e54853;
        locals.var_qeg_dn0 = 0.0;
        locals.var_qeg_dn2 = 0.0;
        locals.var_qeg_dn3 = assign32420_e54851;
        locals.var_qeg_dn4 = 0.0;
        locals.var_qeg_dn5 = 0.0;
        locals.var_qeg_dn6 = 0.0;
        locals.var_qeg_dn7 = 0.0;
        locals.var_qeg_dn8 = 0.0;
        locals.var_qeg_dn9 = 0.0;
        locals.var_qeg_dn10 = (-assign32420_e54851);
        locals.var_qeg_dn11 = 0.0;
        locals.var_qeg_dn13 = 0.0;
        locals.var_qeg_dn14 = 0.0;

        let assign32430_e54856: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign32430_e54856;

        let (assign32440_e54862, assign32440_e54862_d_n0, assign32440_e54862_d_n2, assign32440_e54862_d_n3, assign32440_e54862_d_n4, assign32440_e54862_d_n5, assign32440_e54862_d_n6, assign32440_e54862_d_n7, assign32440_e54862_d_n8, assign32440_e54862_d_n9, assign32440_e54862_d_n10, assign32440_e54862_d_n11, assign32440_e54862_d_n13, assign32440_e54862_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32440_e54860: f64 = (locals.var_devsign * (nv10 - nv3));
        (assign32440_e54860, 0.0, 0.0, (-locals.var_devsign), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_devsign, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32440_e54862;
        locals.var_t2_dn0 = assign32440_e54862_d_n0;
        locals.var_t2_dn2 = assign32440_e54862_d_n2;
        locals.var_t2_dn3 = assign32440_e54862_d_n3;
        locals.var_t2_dn4 = assign32440_e54862_d_n4;
        locals.var_t2_dn5 = assign32440_e54862_d_n5;
        locals.var_t2_dn6 = assign32440_e54862_d_n6;
        locals.var_t2_dn7 = assign32440_e54862_d_n7;
        locals.var_t2_dn8 = assign32440_e54862_d_n8;
        locals.var_t2_dn9 = assign32440_e54862_d_n9;
        locals.var_t2_dn10 = assign32440_e54862_d_n10;
        locals.var_t2_dn11 = assign32440_e54862_d_n11;
        locals.var_t2_dn13 = assign32440_e54862_d_n13;
        locals.var_t2_dn14 = assign32440_e54862_d_n14;

        let (assign32450_e54876, assign32450_e54876_d_n0, assign32450_e54876_d_n2, assign32450_e54876_d_n3, assign32450_e54876_d_n4, assign32450_e54876_d_n5, assign32450_e54876_d_n6, assign32450_e54876_d_n7, assign32450_e54876_d_n8, assign32450_e54876_d_n9, assign32450_e54876_d_n10, assign32450_e54876_d_n11, assign32450_e54876_d_n13, assign32450_e54876_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32450_e54866: f64 = (locals.var_t2 - locals.var_deltaphi);
        let assign32450_e54869: f64 = (locals.var_eg / 2.0);
        let assign32450_e54870: f64 = (assign32450_e54866 + assign32450_e54869);
        let assign32450_e54872: f64 = (assign32450_e54870 + locals.var_phib);
        let assign32450_e54874: f64 = (assign32450_e54872 - p.p1529);
        (assign32450_e54874, ((locals.var_t2_dn0 - locals.var_deltaphi_dn0) + locals.var_phib_dn0), ((locals.var_t2_dn2 - locals.var_deltaphi_dn2) + locals.var_phib_dn2), ((locals.var_t2_dn3 - locals.var_deltaphi_dn3) + locals.var_phib_dn3), (((locals.var_t2_dn4 - locals.var_deltaphi_dn4) + (locals.var_eg_dn4 / 2.0)) + locals.var_phib_dn4), ((locals.var_t2_dn5 - locals.var_deltaphi_dn5) + locals.var_phib_dn5), ((locals.var_t2_dn6 - locals.var_deltaphi_dn6) + locals.var_phib_dn6), ((locals.var_t2_dn7 - locals.var_deltaphi_dn7) + locals.var_phib_dn7), ((locals.var_t2_dn8 - locals.var_deltaphi_dn8) + locals.var_phib_dn8), ((locals.var_t2_dn9 - locals.var_deltaphi_dn9) + locals.var_phib_dn9), ((locals.var_t2_dn10 - locals.var_deltaphi_dn10) + locals.var_phib_dn10), ((locals.var_t2_dn11 - locals.var_deltaphi_dn11) + locals.var_phib_dn11), ((locals.var_t2_dn13 - locals.var_deltaphi_dn13) + locals.var_phib_dn13), ((locals.var_t2_dn14 - locals.var_deltaphi_dn14) + locals.var_phib_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32450_e54876;
        locals.var_t3_dn0 = assign32450_e54876_d_n0;
        locals.var_t3_dn2 = assign32450_e54876_d_n2;
        locals.var_t3_dn3 = assign32450_e54876_d_n3;
        locals.var_t3_dn4 = assign32450_e54876_d_n4;
        locals.var_t3_dn5 = assign32450_e54876_d_n5;
        locals.var_t3_dn6 = assign32450_e54876_d_n6;
        locals.var_t3_dn7 = assign32450_e54876_d_n7;
        locals.var_t3_dn8 = assign32450_e54876_d_n8;
        locals.var_t3_dn9 = assign32450_e54876_d_n9;
        locals.var_t3_dn10 = assign32450_e54876_d_n10;
        locals.var_t3_dn11 = assign32450_e54876_d_n11;
        locals.var_t3_dn13 = assign32450_e54876_d_n13;
        locals.var_t3_dn14 = assign32450_e54876_d_n14;

        let (assign32460_e54882, assign32460_e54882_d_n0, assign32460_e54882_d_n2, assign32460_e54882_d_n3, assign32460_e54882_d_n4, assign32460_e54882_d_n5, assign32460_e54882_d_n6, assign32460_e54882_d_n7, assign32460_e54882_d_n8, assign32460_e54882_d_n9, assign32460_e54882_d_n10, assign32460_e54882_d_n11, assign32460_e54882_d_n13, assign32460_e54882_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32460_e54880: f64 = (locals.var_t3 + 0.02);
        (assign32460_e54880, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32460_e54882;
        locals.var_t0_dn0 = assign32460_e54882_d_n0;
        locals.var_t0_dn2 = assign32460_e54882_d_n2;
        locals.var_t0_dn3 = assign32460_e54882_d_n3;
        locals.var_t0_dn4 = assign32460_e54882_d_n4;
        locals.var_t0_dn5 = assign32460_e54882_d_n5;
        locals.var_t0_dn6 = assign32460_e54882_d_n6;
        locals.var_t0_dn7 = assign32460_e54882_d_n7;
        locals.var_t0_dn8 = assign32460_e54882_d_n8;
        locals.var_t0_dn9 = assign32460_e54882_d_n9;
        locals.var_t0_dn10 = assign32460_e54882_d_n10;
        locals.var_t0_dn11 = assign32460_e54882_d_n11;
        locals.var_t0_dn13 = assign32460_e54882_d_n13;
        locals.var_t0_dn14 = assign32460_e54882_d_n14;

        let (assign32470_e54897, assign32470_e54897_d_n0, assign32470_e54897_d_n2, assign32470_e54897_d_n3, assign32470_e54897_d_n4, assign32470_e54897_d_n5, assign32470_e54897_d_n6, assign32470_e54897_d_n7, assign32470_e54897_d_n8, assign32470_e54897_d_n9, assign32470_e54897_d_n10, assign32470_e54897_d_n11, assign32470_e54897_d_n13, assign32470_e54897_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32470_e54888: f64 = (locals.var_t0 * locals.var_t0);
        let assign32470_e54891: f64 = (4.0 * 0.02);
        let assign32470_e54892: f64 = (assign32470_e54888 + assign32470_e54891);
        let assign32470_e54893: f64 = (assign32470_e54892).sqrt();
        let assign32470_e54894: f64 = (locals.var_t0 + assign32470_e54893);
        let assign32470_e54895: f64 = (0.5 * assign32470_e54894);
        (assign32470_e54895, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign32470_e54893)))),)
    } else {
        (locals.var_vge_overlap, locals.var_vge_overlap_dn0, locals.var_vge_overlap_dn2, locals.var_vge_overlap_dn3, locals.var_vge_overlap_dn4, locals.var_vge_overlap_dn5, locals.var_vge_overlap_dn6, locals.var_vge_overlap_dn7, locals.var_vge_overlap_dn8, locals.var_vge_overlap_dn9, locals.var_vge_overlap_dn10, locals.var_vge_overlap_dn11, locals.var_vge_overlap_dn13, locals.var_vge_overlap_dn14,)
    }
};
        locals.var_vge_overlap = assign32470_e54897;
        locals.var_vge_overlap_dn0 = assign32470_e54897_d_n0;
        locals.var_vge_overlap_dn2 = assign32470_e54897_d_n2;
        locals.var_vge_overlap_dn3 = assign32470_e54897_d_n3;
        locals.var_vge_overlap_dn4 = assign32470_e54897_d_n4;
        locals.var_vge_overlap_dn5 = assign32470_e54897_d_n5;
        locals.var_vge_overlap_dn6 = assign32470_e54897_d_n6;
        locals.var_vge_overlap_dn7 = assign32470_e54897_d_n7;
        locals.var_vge_overlap_dn8 = assign32470_e54897_d_n8;
        locals.var_vge_overlap_dn9 = assign32470_e54897_d_n9;
        locals.var_vge_overlap_dn10 = assign32470_e54897_d_n10;
        locals.var_vge_overlap_dn11 = assign32470_e54897_d_n11;
        locals.var_vge_overlap_dn13 = assign32470_e54897_d_n13;
        locals.var_vge_overlap_dn14 = assign32470_e54897_d_n14;

        let (assign32480_e54926, assign32480_e54926_d_n0, assign32480_e54926_d_n2, assign32480_e54926_d_n3, assign32480_e54926_d_n4, assign32480_e54926_d_n5, assign32480_e54926_d_n6, assign32480_e54926_d_n7, assign32480_e54926_d_n8, assign32480_e54926_d_n9, assign32480_e54926_d_n10, assign32480_e54926_d_n11, assign32480_e54926_d_n13, assign32480_e54926_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32480_e54902: f64 = (locals.var_nfintotal * locals.var_leffcv_1);
        let assign32480_e54906: f64 = (locals.var_t3 - locals.var_vge_overlap);
        let assign32480_e54909: f64 = (0.5 * locals.var_ckappab_i);
        let assign32480_e54913: f64 = (4.0 * locals.var_vge_overlap);
        let assign32480_e54915: f64 = (assign32480_e54913 / locals.var_ckappab_i);
        let assign32480_e54916: f64 = (1.0 + assign32480_e54915);
        let assign32480_e54917: f64 = (assign32480_e54916).sqrt();
        let assign32480_e54919: f64 = (assign32480_e54917 - 1.0);
        let assign32480_e54920: f64 = (assign32480_e54909 * assign32480_e54919);
        let assign32480_e54921: f64 = (assign32480_e54906 + assign32480_e54920);
        let assign32480_e54922: f64 = (locals.var_cgbl_i * assign32480_e54921);
        let assign32480_e54923: f64 = (assign32480_e54902 * assign32480_e54922);
        let assign32480_e54924: f64 = (locals.var_qeg - assign32480_e54923);
        (assign32480_e54924, (locals.var_qeg_dn0 - (((locals.var_nfintotal * locals.var_leffcv_1_dn0) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn0 - locals.var_vge_overlap_dn0) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn0) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn2 - (((locals.var_nfintotal * locals.var_leffcv_1_dn2) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn2 - locals.var_vge_overlap_dn2) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn2) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn3 - (((locals.var_nfintotal * locals.var_leffcv_1_dn3) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn3 - locals.var_vge_overlap_dn3) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn3) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn4 - (((locals.var_nfintotal * locals.var_leffcv_1_dn4) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn4 - locals.var_vge_overlap_dn4) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn4) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn5 - (((locals.var_nfintotal * locals.var_leffcv_1_dn5) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn5 - locals.var_vge_overlap_dn5) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn5) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn6 - (((locals.var_nfintotal * locals.var_leffcv_1_dn6) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn6 - locals.var_vge_overlap_dn6) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn6) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn7 - (((locals.var_nfintotal * locals.var_leffcv_1_dn7) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn7 - locals.var_vge_overlap_dn7) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn7) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn8 - (((locals.var_nfintotal * locals.var_leffcv_1_dn8) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn8 - locals.var_vge_overlap_dn8) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn8) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn9 - (((locals.var_nfintotal * locals.var_leffcv_1_dn9) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn9 - locals.var_vge_overlap_dn9) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn9) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn10 - (((locals.var_nfintotal * locals.var_leffcv_1_dn10) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn10 - locals.var_vge_overlap_dn10) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn10) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn11 - (((locals.var_nfintotal * locals.var_leffcv_1_dn11) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn11 - locals.var_vge_overlap_dn11) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn11) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn13 - (((locals.var_nfintotal * locals.var_leffcv_1_dn13) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn13 - locals.var_vge_overlap_dn13) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn13) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn14 - (((locals.var_nfintotal * locals.var_leffcv_1_dn14) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn14 - locals.var_vge_overlap_dn14) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn14) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))),)
    } else {
        (locals.var_qeg, locals.var_qeg_dn0, locals.var_qeg_dn2, locals.var_qeg_dn3, locals.var_qeg_dn4, locals.var_qeg_dn5, locals.var_qeg_dn6, locals.var_qeg_dn7, locals.var_qeg_dn8, locals.var_qeg_dn9, locals.var_qeg_dn10, locals.var_qeg_dn11, locals.var_qeg_dn13, locals.var_qeg_dn14,)
    }
};
        locals.var_qeg = assign32480_e54926;
        locals.var_qeg_dn0 = assign32480_e54926_d_n0;
        locals.var_qeg_dn2 = assign32480_e54926_d_n2;
        locals.var_qeg_dn3 = assign32480_e54926_d_n3;
        locals.var_qeg_dn4 = assign32480_e54926_d_n4;
        locals.var_qeg_dn5 = assign32480_e54926_d_n5;
        locals.var_qeg_dn6 = assign32480_e54926_d_n6;
        locals.var_qeg_dn7 = assign32480_e54926_d_n7;
        locals.var_qeg_dn8 = assign32480_e54926_d_n8;
        locals.var_qeg_dn9 = assign32480_e54926_d_n9;
        locals.var_qeg_dn10 = assign32480_e54926_d_n10;
        locals.var_qeg_dn11 = assign32480_e54926_d_n11;
        locals.var_qeg_dn13 = assign32480_e54926_d_n13;
        locals.var_qeg_dn14 = assign32480_e54926_d_n14;

        let assign32490_e54931: f64 = (locals.var_bigen_i * locals.var_vds);
        let assign32490_e54933: f64 = (assign32490_e54931 * locals.var_vds);
        let assign32490_e54934: f64 = (locals.var_aigen_i + assign32490_e54933);
        let assign32490_e54935: f64 = (locals.var_vds * assign32490_e54934);
        locals.var_t1 = assign32490_e54935;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = ((locals.var_vds_dn5 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn5) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn5))));
        locals.var_t1_dn6 = ((locals.var_vds_dn6 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn6) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn6))));
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign32510_e54951: f64 = (locals.var_ueff * locals.var_coxeff);
        let assign32510_e54953: f64 = (assign32510_e54951 * locals.var_weff0);
        let assign32510_e54955: f64 = (assign32510_e54953 / locals.var_leff_1);
        locals.var_t0 = assign32510_e54955;
        locals.var_t0_dn0 = ((((((locals.var_ueff_dn0 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn0)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn0)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn2 = ((((((locals.var_ueff_dn2 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn2)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn2)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn3 = ((((((locals.var_ueff_dn3 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn3)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn3)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn4 = ((((((locals.var_ueff_dn4 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn4)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn4)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn5 = ((((((locals.var_ueff_dn5 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn5)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn5)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn6 = ((((((locals.var_ueff_dn6 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn6)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn6)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn7 = ((((((locals.var_ueff_dn7 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn7)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn7)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn8 = ((((((locals.var_ueff_dn8 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn8)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn8)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn9 = ((((((locals.var_ueff_dn9 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn9)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn9)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn10 = ((((((locals.var_ueff_dn10 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn10)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn10)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn11 = ((((((locals.var_ueff_dn11 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn11)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn11)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn13 = ((((((locals.var_ueff_dn13 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn13)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn13)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn14 = ((((((locals.var_ueff_dn14 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn14)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn14)) / (locals.var_leff_1 * locals.var_leff_1));

        let assign32730_e55066: f64 = (2.0 * locals.var_vsat_a);
        let assign32730_e55068: f64 = (assign32730_e55066 / locals.var_ueff);
        locals.var_esatnoi = assign32730_e55068;
        locals.var_esatnoi_dn0 = ((((2.0 * locals.var_vsat_a_dn0) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn0)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn2 = ((((2.0 * locals.var_vsat_a_dn2) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn2)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn3 = ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn4 = ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn5 = ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn6 = ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn7 = ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn8 = ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn9 = ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn10 = ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn11 = ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn13 = ((((2.0 * locals.var_vsat_a_dn13) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn13)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn14 = ((((2.0 * locals.var_vsat_a_dn14) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn14)) / (locals.var_ueff * locals.var_ueff));

        let assign32740_e55079: f64 = if (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard624 = assign32740_e55079;

        let (assign32750_e55087, assign32750_e55087_d_n0, assign32750_e55087_d_n2, assign32750_e55087_d_n3, assign32750_e55087_d_n4, assign32750_e55087_d_n5, assign32750_e55087_d_n6, assign32750_e55087_d_n7, assign32750_e55087_d_n8, assign32750_e55087_d_n9, assign32750_e55087_d_n10, assign32750_e55087_d_n11, assign32750_e55087_d_n13, assign32750_e55087_d_n14,) = {
    if (locals.var_guard624 != 0.0) {
        let assign32750_e55084: f64 = (2.0 * p.p1687);
        let assign32750_e55085: f64 = (locals.var_leff_1 - assign32750_e55084);
        (assign32750_e55085, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32750_e55087;
        locals.var_leffnoi_dn0 = assign32750_e55087_d_n0;
        locals.var_leffnoi_dn2 = assign32750_e55087_d_n2;
        locals.var_leffnoi_dn3 = assign32750_e55087_d_n3;
        locals.var_leffnoi_dn4 = assign32750_e55087_d_n4;
        locals.var_leffnoi_dn5 = assign32750_e55087_d_n5;
        locals.var_leffnoi_dn6 = assign32750_e55087_d_n6;
        locals.var_leffnoi_dn7 = assign32750_e55087_d_n7;
        locals.var_leffnoi_dn8 = assign32750_e55087_d_n8;
        locals.var_leffnoi_dn9 = assign32750_e55087_d_n9;
        locals.var_leffnoi_dn10 = assign32750_e55087_d_n10;
        locals.var_leffnoi_dn11 = assign32750_e55087_d_n11;
        locals.var_leffnoi_dn13 = assign32750_e55087_d_n13;
        locals.var_leffnoi_dn14 = assign32750_e55087_d_n14;

        let assign32760_e55090: f64 = if locals.var_leffnoi <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign32760_e55090;

        let (assign32770_e55096, assign32770_e55096_d_n0, assign32770_e55096_d_n2, assign32770_e55096_d_n3, assign32770_e55096_d_n4, assign32770_e55096_d_n5, assign32770_e55096_d_n6, assign32770_e55096_d_n7, assign32770_e55096_d_n8, assign32770_e55096_d_n9, assign32770_e55096_d_n10, assign32770_e55096_d_n11, assign32770_e55096_d_n13, assign32770_e55096_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 != 0.0)) {
        (locals.var_leff_1, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32770_e55096;
        locals.var_leffnoi_dn0 = assign32770_e55096_d_n0;
        locals.var_leffnoi_dn2 = assign32770_e55096_d_n2;
        locals.var_leffnoi_dn3 = assign32770_e55096_d_n3;
        locals.var_leffnoi_dn4 = assign32770_e55096_d_n4;
        locals.var_leffnoi_dn5 = assign32770_e55096_d_n5;
        locals.var_leffnoi_dn6 = assign32770_e55096_d_n6;
        locals.var_leffnoi_dn7 = assign32770_e55096_d_n7;
        locals.var_leffnoi_dn8 = assign32770_e55096_d_n8;
        locals.var_leffnoi_dn9 = assign32770_e55096_d_n9;
        locals.var_leffnoi_dn10 = assign32770_e55096_d_n10;
        locals.var_leffnoi_dn11 = assign32770_e55096_d_n11;
        locals.var_leffnoi_dn13 = assign32770_e55096_d_n13;
        locals.var_leffnoi_dn14 = assign32770_e55096_d_n14;

        let assign32780_e55103: f64 = if ((p.p79 == 1.0) || (p.p79 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard626 = assign32780_e55103;

        let (assign32790_e55111, assign32790_e55111_d_n0, assign32790_e55111_d_n2, assign32790_e55111_d_n3, assign32790_e55111_d_n4, assign32790_e55111_d_n5, assign32790_e55111_d_n6, assign32790_e55111_d_n7, assign32790_e55111_d_n8, assign32790_e55111_d_n9, assign32790_e55111_d_n10, assign32790_e55111_d_n11, assign32790_e55111_d_n13, assign32790_e55111_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32790_e55109: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign32790_e55109, ((locals.var_leffnoi_dn0 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn0)), ((locals.var_leffnoi_dn2 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn2)), ((locals.var_leffnoi_dn3 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn3)), ((locals.var_leffnoi_dn4 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn4)), ((locals.var_leffnoi_dn5 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn5)), ((locals.var_leffnoi_dn6 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn6)), ((locals.var_leffnoi_dn7 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn7)), ((locals.var_leffnoi_dn8 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn8)), ((locals.var_leffnoi_dn9 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn9)), ((locals.var_leffnoi_dn10 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn10)), ((locals.var_leffnoi_dn11 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn11)), ((locals.var_leffnoi_dn13 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn13)), ((locals.var_leffnoi_dn14 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn14)),)
    } else {
        (locals.var_leffnoisq, locals.var_leffnoisq_dn0, locals.var_leffnoisq_dn2, locals.var_leffnoisq_dn3, locals.var_leffnoisq_dn4, locals.var_leffnoisq_dn5, locals.var_leffnoisq_dn6, locals.var_leffnoisq_dn7, locals.var_leffnoisq_dn8, locals.var_leffnoisq_dn9, locals.var_leffnoisq_dn10, locals.var_leffnoisq_dn11, locals.var_leffnoisq_dn13, locals.var_leffnoisq_dn14,)
    }
};
        locals.var_leffnoisq = assign32790_e55111;
        locals.var_leffnoisq_dn0 = assign32790_e55111_d_n0;
        locals.var_leffnoisq_dn2 = assign32790_e55111_d_n2;
        locals.var_leffnoisq_dn3 = assign32790_e55111_d_n3;
        locals.var_leffnoisq_dn4 = assign32790_e55111_d_n4;
        locals.var_leffnoisq_dn5 = assign32790_e55111_d_n5;
        locals.var_leffnoisq_dn6 = assign32790_e55111_d_n6;
        locals.var_leffnoisq_dn7 = assign32790_e55111_d_n7;
        locals.var_leffnoisq_dn8 = assign32790_e55111_d_n8;
        locals.var_leffnoisq_dn9 = assign32790_e55111_d_n9;
        locals.var_leffnoisq_dn10 = assign32790_e55111_d_n10;
        locals.var_leffnoisq_dn11 = assign32790_e55111_d_n11;
        locals.var_leffnoisq_dn13 = assign32790_e55111_d_n13;
        locals.var_leffnoisq_dn14 = assign32790_e55111_d_n14;

        let assign32800_e55114: f64 = if p.p1681 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign32800_e55114;

        let (assign32810_e55128, assign32810_e55128_d_n0, assign32810_e55128_d_n2, assign32810_e55128_d_n3, assign32810_e55128_d_n4, assign32810_e55128_d_n5, assign32810_e55128_d_n6, assign32810_e55128_d_n7, assign32810_e55128_d_n8, assign32810_e55128_d_n9, assign32810_e55128_d_n10, assign32810_e55128_d_n11, assign32810_e55128_d_n13, assign32810_e55128_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign32810_e55122: f64 = (locals.var_diffvds / locals.var_litl);
        let assign32810_e55124: f64 = (assign32810_e55122 + p.p1681);
        let assign32810_e55126: f64 = (assign32810_e55124 / locals.var_esatnoi);
        (assign32810_e55126, ((((locals.var_diffvds_dn0 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn0)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn2 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn2)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn13 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn13)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn14 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn14)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32810_e55128;
        locals.var_t0_dn0 = assign32810_e55128_d_n0;
        locals.var_t0_dn2 = assign32810_e55128_d_n2;
        locals.var_t0_dn3 = assign32810_e55128_d_n3;
        locals.var_t0_dn4 = assign32810_e55128_d_n4;
        locals.var_t0_dn5 = assign32810_e55128_d_n5;
        locals.var_t0_dn6 = assign32810_e55128_d_n6;
        locals.var_t0_dn7 = assign32810_e55128_d_n7;
        locals.var_t0_dn8 = assign32810_e55128_d_n8;
        locals.var_t0_dn9 = assign32810_e55128_d_n9;
        locals.var_t0_dn10 = assign32810_e55128_d_n10;
        locals.var_t0_dn11 = assign32810_e55128_d_n11;
        locals.var_t0_dn13 = assign32810_e55128_d_n13;
        locals.var_t0_dn14 = assign32810_e55128_d_n14;

        let (assign32820_e55151, assign32820_e55151_d_n0, assign32820_e55151_d_n2, assign32820_e55151_d_n3, assign32820_e55151_d_n4, assign32820_e55151_d_n5, assign32820_e55151_d_n6, assign32820_e55151_d_n7, assign32820_e55151_d_n8, assign32820_e55151_d_n9, assign32820_e55151_d_n10, assign32820_e55151_d_n11, assign32820_e55151_d_n13, assign32820_e55151_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let (assign32820_e55148, assign32820_e55148_d_n0, assign32820_e55148_d_n2, assign32820_e55148_d_n3, assign32820_e55148_d_n4, assign32820_e55148_d_n5, assign32820_e55148_d_n6, assign32820_e55148_d_n7, assign32820_e55148_d_n8, assign32820_e55148_d_n9, assign32820_e55148_d_n10, assign32820_e55148_d_n11, assign32820_e55148_d_n13, assign32820_e55148_d_n14,) = {
            if (!(locals.var_t0 > 1e-38)) {
                let assign32820_e55140: f64 = (-87.498233534);
                (assign32820_e55140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,) = {
                    if (locals.var_t0 > 1e-38) {
                        let assign32820_e55145: f64 = (locals.var_t0).ln();
                        (assign32820_e55145, (locals.var_t0_dn0 / locals.var_t0), (locals.var_t0_dn2 / locals.var_t0), (locals.var_t0_dn3 / locals.var_t0), (locals.var_t0_dn4 / locals.var_t0), (locals.var_t0_dn5 / locals.var_t0), (locals.var_t0_dn6 / locals.var_t0), (locals.var_t0_dn7 / locals.var_t0), (locals.var_t0_dn8 / locals.var_t0), (locals.var_t0_dn9 / locals.var_t0), (locals.var_t0_dn10 / locals.var_t0), (locals.var_t0_dn11 / locals.var_t0), (locals.var_t0_dn13 / locals.var_t0), (locals.var_t0_dn14 / locals.var_t0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,)
            }
        };
        let assign32820_e55149: f64 = (locals.var_litl * assign32820_e55148);
        (assign32820_e55149, (locals.var_litl * assign32820_e55148_d_n0), (locals.var_litl * assign32820_e55148_d_n2), (locals.var_litl * assign32820_e55148_d_n3), (locals.var_litl * assign32820_e55148_d_n4), (locals.var_litl * assign32820_e55148_d_n5), (locals.var_litl * assign32820_e55148_d_n6), (locals.var_litl * assign32820_e55148_d_n7), (locals.var_litl * assign32820_e55148_d_n8), (locals.var_litl * assign32820_e55148_d_n9), (locals.var_litl * assign32820_e55148_d_n10), (locals.var_litl * assign32820_e55148_d_n11), (locals.var_litl * assign32820_e55148_d_n13), (locals.var_litl * assign32820_e55148_d_n14),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32820_e55151;
        locals.var_delclm_dn0 = assign32820_e55151_d_n0;
        locals.var_delclm_dn2 = assign32820_e55151_d_n2;
        locals.var_delclm_dn3 = assign32820_e55151_d_n3;
        locals.var_delclm_dn4 = assign32820_e55151_d_n4;
        locals.var_delclm_dn5 = assign32820_e55151_d_n5;
        locals.var_delclm_dn6 = assign32820_e55151_d_n6;
        locals.var_delclm_dn7 = assign32820_e55151_d_n7;
        locals.var_delclm_dn8 = assign32820_e55151_d_n8;
        locals.var_delclm_dn9 = assign32820_e55151_d_n9;
        locals.var_delclm_dn10 = assign32820_e55151_d_n10;
        locals.var_delclm_dn11 = assign32820_e55151_d_n11;
        locals.var_delclm_dn13 = assign32820_e55151_d_n13;
        locals.var_delclm_dn14 = assign32820_e55151_d_n14;

        let (assign32830_e55160, assign32830_e55160_d_n0, assign32830_e55160_d_n2, assign32830_e55160_d_n3, assign32830_e55160_d_n4, assign32830_e55160_d_n5, assign32830_e55160_d_n6, assign32830_e55160_d_n7, assign32830_e55160_d_n8, assign32830_e55160_d_n9, assign32830_e55160_d_n10, assign32830_e55160_d_n11, assign32830_e55160_d_n13, assign32830_e55160_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32830_e55160;
        locals.var_delclm_dn0 = assign32830_e55160_d_n0;
        locals.var_delclm_dn2 = assign32830_e55160_d_n2;
        locals.var_delclm_dn3 = assign32830_e55160_d_n3;
        locals.var_delclm_dn4 = assign32830_e55160_d_n4;
        locals.var_delclm_dn5 = assign32830_e55160_d_n5;
        locals.var_delclm_dn6 = assign32830_e55160_d_n6;
        locals.var_delclm_dn7 = assign32830_e55160_d_n7;
        locals.var_delclm_dn8 = assign32830_e55160_d_n8;
        locals.var_delclm_dn9 = assign32830_e55160_d_n9;
        locals.var_delclm_dn10 = assign32830_e55160_d_n10;
        locals.var_delclm_dn11 = assign32830_e55160_d_n11;
        locals.var_delclm_dn13 = assign32830_e55160_d_n13;
        locals.var_delclm_dn14 = assign32830_e55160_d_n14;

        let assign32840_e55163: f64 = if p.p79 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign32840_e55163;

        let (assign32850_e55173, assign32850_e55173_d_n0, assign32850_e55173_d_n2, assign32850_e55173_d_n3, assign32850_e55173_d_n4, assign32850_e55173_d_n5, assign32850_e55173_d_n6, assign32850_e55173_d_n7, assign32850_e55173_d_n8, assign32850_e55173_d_n9, assign32850_e55173_d_n10, assign32850_e55173_d_n11, assign32850_e55173_d_n13, assign32850_e55173_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32850_e55171: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign32850_e55171, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32850_e55173;
        locals.var_t1_dn0 = assign32850_e55173_d_n0;
        locals.var_t1_dn2 = assign32850_e55173_d_n2;
        locals.var_t1_dn3 = assign32850_e55173_d_n3;
        locals.var_t1_dn4 = assign32850_e55173_d_n4;
        locals.var_t1_dn5 = assign32850_e55173_d_n5;
        locals.var_t1_dn6 = assign32850_e55173_d_n6;
        locals.var_t1_dn7 = assign32850_e55173_d_n7;
        locals.var_t1_dn8 = assign32850_e55173_d_n8;
        locals.var_t1_dn9 = assign32850_e55173_d_n9;
        locals.var_t1_dn10 = assign32850_e55173_d_n10;
        locals.var_t1_dn11 = assign32850_e55173_d_n11;
        locals.var_t1_dn13 = assign32850_e55173_d_n13;
        locals.var_t1_dn14 = assign32850_e55173_d_n14;

        let (assign32860_e55185, assign32860_e55185_d_n0, assign32860_e55185_d_n2, assign32860_e55185_d_n3, assign32860_e55185_d_n4, assign32860_e55185_d_n5, assign32860_e55185_d_n6, assign32860_e55185_d_n7, assign32860_e55185_d_n8, assign32860_e55185_d_n9, assign32860_e55185_d_n10, assign32860_e55185_d_n11, assign32860_e55185_d_n13, assign32860_e55185_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32860_e55182: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign32860_e55183: f64 = (1.0 + assign32860_e55182);
        (assign32860_e55183, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32860_e55185;
        locals.var_t2_dn0 = assign32860_e55185_d_n0;
        locals.var_t2_dn2 = assign32860_e55185_d_n2;
        locals.var_t2_dn3 = assign32860_e55185_d_n3;
        locals.var_t2_dn4 = assign32860_e55185_d_n4;
        locals.var_t2_dn5 = assign32860_e55185_d_n5;
        locals.var_t2_dn6 = assign32860_e55185_d_n6;
        locals.var_t2_dn7 = assign32860_e55185_d_n7;
        locals.var_t2_dn8 = assign32860_e55185_d_n8;
        locals.var_t2_dn9 = assign32860_e55185_d_n9;
        locals.var_t2_dn10 = assign32860_e55185_d_n10;
        locals.var_t2_dn11 = assign32860_e55185_d_n11;
        locals.var_t2_dn13 = assign32860_e55185_d_n13;
        locals.var_t2_dn14 = assign32860_e55185_d_n14;

        let (assign32870_e55195, assign32870_e55195_d_n0, assign32870_e55195_d_n2, assign32870_e55195_d_n3, assign32870_e55195_d_n4, assign32870_e55195_d_n5, assign32870_e55195_d_n6, assign32870_e55195_d_n7, assign32870_e55195_d_n8, assign32870_e55195_d_n9, assign32870_e55195_d_n10, assign32870_e55195_d_n11, assign32870_e55195_d_n13, assign32870_e55195_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32870_e55193: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign32870_e55193, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32870_e55195;
        locals.var_t3_dn0 = assign32870_e55195_d_n0;
        locals.var_t3_dn2 = assign32870_e55195_d_n2;
        locals.var_t3_dn3 = assign32870_e55195_d_n3;
        locals.var_t3_dn4 = assign32870_e55195_d_n4;
        locals.var_t3_dn5 = assign32870_e55195_d_n5;
        locals.var_t3_dn6 = assign32870_e55195_d_n6;
        locals.var_t3_dn7 = assign32870_e55195_d_n7;
        locals.var_t3_dn8 = assign32870_e55195_d_n8;
        locals.var_t3_dn9 = assign32870_e55195_d_n9;
        locals.var_t3_dn10 = assign32870_e55195_d_n10;
        locals.var_t3_dn11 = assign32870_e55195_d_n11;
        locals.var_t3_dn13 = assign32870_e55195_d_n13;
        locals.var_t3_dn14 = assign32870_e55195_d_n14;

        let (assign32880_e55205, assign32880_e55205_d_n0, assign32880_e55205_d_n2, assign32880_e55205_d_n3, assign32880_e55205_d_n4, assign32880_e55205_d_n5, assign32880_e55205_d_n6, assign32880_e55205_d_n7, assign32880_e55205_d_n8, assign32880_e55205_d_n9, assign32880_e55205_d_n10, assign32880_e55205_d_n11, assign32880_e55205_d_n13, assign32880_e55205_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32880_e55203: f64 = (locals.var_t3 / p.p1682);
        (assign32880_e55203, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32880_e55205;
        locals.var_t4_dn0 = assign32880_e55205_d_n0;
        locals.var_t4_dn2 = assign32880_e55205_d_n2;
        locals.var_t4_dn3 = assign32880_e55205_d_n3;
        locals.var_t4_dn4 = assign32880_e55205_d_n4;
        locals.var_t4_dn5 = assign32880_e55205_d_n5;
        locals.var_t4_dn6 = assign32880_e55205_d_n6;
        locals.var_t4_dn7 = assign32880_e55205_d_n7;
        locals.var_t4_dn8 = assign32880_e55205_d_n8;
        locals.var_t4_dn9 = assign32880_e55205_d_n9;
        locals.var_t4_dn10 = assign32880_e55205_d_n10;
        locals.var_t4_dn11 = assign32880_e55205_d_n11;
        locals.var_t4_dn13 = assign32880_e55205_d_n13;
        locals.var_t4_dn14 = assign32880_e55205_d_n14;

    }

    pub(super) fn stamp_transient_block_122(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32890_e55232, assign32890_e55232_d_n0, assign32890_e55232_d_n2, assign32890_e55232_d_n3, assign32890_e55232_d_n4, assign32890_e55232_d_n5, assign32890_e55232_d_n6, assign32890_e55232_d_n7, assign32890_e55232_d_n8, assign32890_e55232_d_n9, assign32890_e55232_d_n10, assign32890_e55232_d_n11, assign32890_e55232_d_n13, assign32890_e55232_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32890_e55214: f64 = (locals.var_t4 + 1.0);
        let assign32890_e55217: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55220: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55221: f64 = (assign32890_e55217 * assign32890_e55220);
        let assign32890_e55224: f64 = (0.25 * p.p1688);
        let assign32890_e55226: f64 = (assign32890_e55224 * p.p1688);
        let assign32890_e55227: f64 = (assign32890_e55221 + assign32890_e55226);
        let assign32890_e55228: f64 = (assign32890_e55227).sqrt();
        let assign32890_e55229: f64 = (assign32890_e55214 + assign32890_e55228);
        let assign32890_e55230: f64 = (0.5 * assign32890_e55229);
        (assign32890_e55230, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn0)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn2)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn3)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn4)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn5)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn6)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn7)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn8)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn9)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn10)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn11)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn13)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn14)) / (2.0 * assign32890_e55228)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32890_e55232;
        locals.var_t5_dn0 = assign32890_e55232_d_n0;
        locals.var_t5_dn2 = assign32890_e55232_d_n2;
        locals.var_t5_dn3 = assign32890_e55232_d_n3;
        locals.var_t5_dn4 = assign32890_e55232_d_n4;
        locals.var_t5_dn5 = assign32890_e55232_d_n5;
        locals.var_t5_dn6 = assign32890_e55232_d_n6;
        locals.var_t5_dn7 = assign32890_e55232_d_n7;
        locals.var_t5_dn8 = assign32890_e55232_d_n8;
        locals.var_t5_dn9 = assign32890_e55232_d_n9;
        locals.var_t5_dn10 = assign32890_e55232_d_n10;
        locals.var_t5_dn11 = assign32890_e55232_d_n11;
        locals.var_t5_dn13 = assign32890_e55232_d_n13;
        locals.var_t5_dn14 = assign32890_e55232_d_n14;

        let (assign32900_e55242, assign32900_e55242_d_n0, assign32900_e55242_d_n2, assign32900_e55242_d_n3, assign32900_e55242_d_n4, assign32900_e55242_d_n5, assign32900_e55242_d_n6, assign32900_e55242_d_n7, assign32900_e55242_d_n8, assign32900_e55242_d_n9, assign32900_e55242_d_n10, assign32900_e55242_d_n11, assign32900_e55242_d_n13, assign32900_e55242_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32900_e55240: f64 = (p.p1682 * locals.var_t5);
        (assign32900_e55240, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32900_e55242;
        locals.var_noiaeff_dn0 = assign32900_e55242_d_n0;
        locals.var_noiaeff_dn2 = assign32900_e55242_d_n2;
        locals.var_noiaeff_dn3 = assign32900_e55242_d_n3;
        locals.var_noiaeff_dn4 = assign32900_e55242_d_n4;
        locals.var_noiaeff_dn5 = assign32900_e55242_d_n5;
        locals.var_noiaeff_dn6 = assign32900_e55242_d_n6;
        locals.var_noiaeff_dn7 = assign32900_e55242_d_n7;
        locals.var_noiaeff_dn8 = assign32900_e55242_d_n8;
        locals.var_noiaeff_dn9 = assign32900_e55242_d_n9;
        locals.var_noiaeff_dn10 = assign32900_e55242_d_n10;
        locals.var_noiaeff_dn11 = assign32900_e55242_d_n11;
        locals.var_noiaeff_dn13 = assign32900_e55242_d_n13;
        locals.var_noiaeff_dn14 = assign32900_e55242_d_n14;

        let (assign32910_e55251, assign32910_e55251_d_n0, assign32910_e55251_d_n2, assign32910_e55251_d_n3, assign32910_e55251_d_n4, assign32910_e55251_d_n5, assign32910_e55251_d_n6, assign32910_e55251_d_n7, assign32910_e55251_d_n8, assign32910_e55251_d_n9, assign32910_e55251_d_n10, assign32910_e55251_d_n11, assign32910_e55251_d_n13, assign32910_e55251_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 == 0.0)) {
        (p.p1682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32910_e55251;
        locals.var_noiaeff_dn0 = assign32910_e55251_d_n0;
        locals.var_noiaeff_dn2 = assign32910_e55251_d_n2;
        locals.var_noiaeff_dn3 = assign32910_e55251_d_n3;
        locals.var_noiaeff_dn4 = assign32910_e55251_d_n4;
        locals.var_noiaeff_dn5 = assign32910_e55251_d_n5;
        locals.var_noiaeff_dn6 = assign32910_e55251_d_n6;
        locals.var_noiaeff_dn7 = assign32910_e55251_d_n7;
        locals.var_noiaeff_dn8 = assign32910_e55251_d_n8;
        locals.var_noiaeff_dn9 = assign32910_e55251_d_n9;
        locals.var_noiaeff_dn10 = assign32910_e55251_d_n10;
        locals.var_noiaeff_dn11 = assign32910_e55251_d_n11;
        locals.var_noiaeff_dn13 = assign32910_e55251_d_n13;
        locals.var_noiaeff_dn14 = assign32910_e55251_d_n14;

        let (assign32920_e55268, assign32920_e55268_d_n0, assign32920_e55268_d_n2, assign32920_e55268_d_n3, assign32920_e55268_d_n4, assign32920_e55268_d_n5, assign32920_e55268_d_n6, assign32920_e55268_d_n7, assign32920_e55268_d_n8, assign32920_e55268_d_n9, assign32920_e55268_d_n10, assign32920_e55268_d_n11, assign32920_e55268_d_n13, assign32920_e55268_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32920_e55257: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32920_e55259: f64 = (assign32920_e55257 * 1.60219e-19);
        let assign32920_e55261: f64 = (assign32920_e55259 * locals.var_vtm);
        let assign32920_e55263: f64 = (locals.var_ids_v).abs();
        let assign32920_e55264: f64 = (assign32920_e55261 * assign32920_e55263);
        let assign32920_e55266: f64 = (assign32920_e55264 * locals.var_ueff);
        (assign32920_e55266, (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn0 } else { (-locals.var_ids_v_dn0) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn0)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn2 } else { (-locals.var_ids_v_dn2) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn2)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn3 } else { (-locals.var_ids_v_dn3) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn3)), (((((assign32920_e55259 * locals.var_vtm_dn4) * assign32920_e55263) + (assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn4 } else { (-locals.var_ids_v_dn4) })) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn4)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn5 } else { (-locals.var_ids_v_dn5) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn5)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn6 } else { (-locals.var_ids_v_dn6) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn6)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn7 } else { (-locals.var_ids_v_dn7) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn7)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn8 } else { (-locals.var_ids_v_dn8) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn8)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn9 } else { (-locals.var_ids_v_dn9) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn9)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn10 } else { (-locals.var_ids_v_dn10) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn10)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn11 } else { (-locals.var_ids_v_dn11) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn11)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn13 } else { (-locals.var_ids_v_dn13) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn13)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn14 } else { (-locals.var_ids_v_dn14) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e55268;
        locals.var_t1_dn0 = assign32920_e55268_d_n0;
        locals.var_t1_dn2 = assign32920_e55268_d_n2;
        locals.var_t1_dn3 = assign32920_e55268_d_n3;
        locals.var_t1_dn4 = assign32920_e55268_d_n4;
        locals.var_t1_dn5 = assign32920_e55268_d_n5;
        locals.var_t1_dn6 = assign32920_e55268_d_n6;
        locals.var_t1_dn7 = assign32920_e55268_d_n7;
        locals.var_t1_dn8 = assign32920_e55268_d_n8;
        locals.var_t1_dn9 = assign32920_e55268_d_n9;
        locals.var_t1_dn10 = assign32920_e55268_d_n10;
        locals.var_t1_dn11 = assign32920_e55268_d_n11;
        locals.var_t1_dn13 = assign32920_e55268_d_n13;
        locals.var_t1_dn14 = assign32920_e55268_d_n14;

        let (assign32930_e55278, assign32930_e55278_d_n0, assign32930_e55278_d_n2, assign32930_e55278_d_n3, assign32930_e55278_d_n4, assign32930_e55278_d_n5, assign32930_e55278_d_n6, assign32930_e55278_d_n7, assign32930_e55278_d_n8, assign32930_e55278_d_n9, assign32930_e55278_d_n10, assign32930_e55278_d_n11, assign32930_e55278_d_n13, assign32930_e55278_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32930_e55274: f64 = (10000000000.0 * locals.var_coxeff);
        let assign32930_e55276: f64 = (assign32930_e55274 * locals.var_leffnoisq);
        (assign32930_e55276, (((10000000000.0 * locals.var_coxeff_dn0) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn0)), (((10000000000.0 * locals.var_coxeff_dn2) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn2)), (((10000000000.0 * locals.var_coxeff_dn3) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn3)), (((10000000000.0 * locals.var_coxeff_dn4) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn4)), (((10000000000.0 * locals.var_coxeff_dn5) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn5)), (((10000000000.0 * locals.var_coxeff_dn6) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn6)), (((10000000000.0 * locals.var_coxeff_dn7) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn7)), (((10000000000.0 * locals.var_coxeff_dn8) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn8)), (((10000000000.0 * locals.var_coxeff_dn9) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn9)), (((10000000000.0 * locals.var_coxeff_dn10) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn10)), (((10000000000.0 * locals.var_coxeff_dn11) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn11)), (((10000000000.0 * locals.var_coxeff_dn13) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn13)), (((10000000000.0 * locals.var_coxeff_dn14) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e55278;
        locals.var_t2_dn0 = assign32930_e55278_d_n0;
        locals.var_t2_dn2 = assign32930_e55278_d_n2;
        locals.var_t2_dn3 = assign32930_e55278_d_n3;
        locals.var_t2_dn4 = assign32930_e55278_d_n4;
        locals.var_t2_dn5 = assign32930_e55278_d_n5;
        locals.var_t2_dn6 = assign32930_e55278_d_n6;
        locals.var_t2_dn7 = assign32930_e55278_d_n7;
        locals.var_t2_dn8 = assign32930_e55278_d_n8;
        locals.var_t2_dn9 = assign32930_e55278_d_n9;
        locals.var_t2_dn10 = assign32930_e55278_d_n10;
        locals.var_t2_dn11 = assign32930_e55278_d_n11;
        locals.var_t2_dn13 = assign32930_e55278_d_n13;
        locals.var_t2_dn14 = assign32930_e55278_d_n14;

        let (assign32940_e55288, assign32940_e55288_d_n0, assign32940_e55288_d_n2, assign32940_e55288_d_n3, assign32940_e55288_d_n4, assign32940_e55288_d_n5, assign32940_e55288_d_n6, assign32940_e55288_d_n7, assign32940_e55288_d_n8, assign32940_e55288_d_n9, assign32940_e55288_d_n10, assign32940_e55288_d_n11, assign32940_e55288_d_n13, assign32940_e55288_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32940_e55284: f64 = (locals.var_coxeff * locals.var_qis);
        let assign32940_e55286: f64 = (assign32940_e55284 / 1.60219e-19);
        (assign32940_e55286, (((locals.var_coxeff_dn0 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32940_e55288;
        locals.var_n0_dn0 = assign32940_e55288_d_n0;
        locals.var_n0_dn2 = assign32940_e55288_d_n2;
        locals.var_n0_dn3 = assign32940_e55288_d_n3;
        locals.var_n0_dn4 = assign32940_e55288_d_n4;
        locals.var_n0_dn5 = assign32940_e55288_d_n5;
        locals.var_n0_dn6 = assign32940_e55288_d_n6;
        locals.var_n0_dn7 = assign32940_e55288_d_n7;
        locals.var_n0_dn8 = assign32940_e55288_d_n8;
        locals.var_n0_dn9 = assign32940_e55288_d_n9;
        locals.var_n0_dn10 = assign32940_e55288_d_n10;
        locals.var_n0_dn11 = assign32940_e55288_d_n11;
        locals.var_n0_dn13 = assign32940_e55288_d_n13;
        locals.var_n0_dn14 = assign32940_e55288_d_n14;

        let (assign32950_e55298, assign32950_e55298_d_n0, assign32950_e55298_d_n2, assign32950_e55298_d_n3, assign32950_e55298_d_n4, assign32950_e55298_d_n5, assign32950_e55298_d_n6, assign32950_e55298_d_n7, assign32950_e55298_d_n8, assign32950_e55298_d_n9, assign32950_e55298_d_n10, assign32950_e55298_d_n11, assign32950_e55298_d_n13, assign32950_e55298_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32950_e55294: f64 = (locals.var_coxeff * locals.var_qid);
        let assign32950_e55296: f64 = (assign32950_e55294 / 1.60219e-19);
        (assign32950_e55296, (((locals.var_coxeff_dn0 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32950_e55298;
        locals.var_nl_dn0 = assign32950_e55298_d_n0;
        locals.var_nl_dn2 = assign32950_e55298_d_n2;
        locals.var_nl_dn3 = assign32950_e55298_d_n3;
        locals.var_nl_dn4 = assign32950_e55298_d_n4;
        locals.var_nl_dn5 = assign32950_e55298_d_n5;
        locals.var_nl_dn6 = assign32950_e55298_d_n6;
        locals.var_nl_dn7 = assign32950_e55298_d_n7;
        locals.var_nl_dn8 = assign32950_e55298_d_n8;
        locals.var_nl_dn9 = assign32950_e55298_d_n9;
        locals.var_nl_dn10 = assign32950_e55298_d_n10;
        locals.var_nl_dn11 = assign32950_e55298_d_n11;
        locals.var_nl_dn13 = assign32950_e55298_d_n13;
        locals.var_nl_dn14 = assign32950_e55298_d_n14;

        let (assign32960_e55310, assign32960_e55310_d_n0, assign32960_e55310_d_n2, assign32960_e55310_d_n3, assign32960_e55310_d_n4, assign32960_e55310_d_n5, assign32960_e55310_d_n6, assign32960_e55310_d_n7, assign32960_e55310_d_n8, assign32960_e55310_d_n9, assign32960_e55310_d_n10, assign32960_e55310_d_n11, assign32960_e55310_d_n13, assign32960_e55310_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32960_e55304: f64 = (locals.var_vtm / 1.60219e-19);
        let assign32960_e55307: f64 = (locals.var_coxeff + locals.var_cit_a);
        let assign32960_e55308: f64 = (assign32960_e55304 * assign32960_e55307);
        (assign32960_e55308, (assign32960_e55304 * (locals.var_coxeff_dn0 + locals.var_cit_a_dn0)), (assign32960_e55304 * (locals.var_coxeff_dn2 + locals.var_cit_a_dn2)), (assign32960_e55304 * (locals.var_coxeff_dn3 + locals.var_cit_a_dn3)), (((locals.var_vtm_dn4 / 1.60219e-19) * assign32960_e55307) + (assign32960_e55304 * (locals.var_coxeff_dn4 + locals.var_cit_a_dn4))), (assign32960_e55304 * (locals.var_coxeff_dn5 + locals.var_cit_a_dn5)), (assign32960_e55304 * (locals.var_coxeff_dn6 + locals.var_cit_a_dn6)), (assign32960_e55304 * (locals.var_coxeff_dn7 + locals.var_cit_a_dn7)), (assign32960_e55304 * (locals.var_coxeff_dn8 + locals.var_cit_a_dn8)), (assign32960_e55304 * (locals.var_coxeff_dn9 + locals.var_cit_a_dn9)), (assign32960_e55304 * (locals.var_coxeff_dn10 + locals.var_cit_a_dn10)), (assign32960_e55304 * (locals.var_coxeff_dn11 + locals.var_cit_a_dn11)), (assign32960_e55304 * (locals.var_coxeff_dn13 + locals.var_cit_a_dn13)), (assign32960_e55304 * (locals.var_coxeff_dn14 + locals.var_cit_a_dn14)),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32960_e55310;
        locals.var_nstar_dn0 = assign32960_e55310_d_n0;
        locals.var_nstar_dn2 = assign32960_e55310_d_n2;
        locals.var_nstar_dn3 = assign32960_e55310_d_n3;
        locals.var_nstar_dn4 = assign32960_e55310_d_n4;
        locals.var_nstar_dn5 = assign32960_e55310_d_n5;
        locals.var_nstar_dn6 = assign32960_e55310_d_n6;
        locals.var_nstar_dn7 = assign32960_e55310_d_n7;
        locals.var_nstar_dn8 = assign32960_e55310_d_n8;
        locals.var_nstar_dn9 = assign32960_e55310_d_n9;
        locals.var_nstar_dn10 = assign32960_e55310_d_n10;
        locals.var_nstar_dn11 = assign32960_e55310_d_n11;
        locals.var_nstar_dn13 = assign32960_e55310_d_n13;
        locals.var_nstar_dn14 = assign32960_e55310_d_n14;

        let (assign32970_e55349, assign32970_e55349_d_n0, assign32970_e55349_d_n2, assign32970_e55349_d_n3, assign32970_e55349_d_n4, assign32970_e55349_d_n5, assign32970_e55349_d_n6, assign32970_e55349_d_n7, assign32970_e55349_d_n8, assign32970_e55349_d_n9, assign32970_e55349_d_n10, assign32970_e55349_d_n11, assign32970_e55349_d_n13, assign32970_e55349_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32970_e55317: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32970_e55320: f64 = (locals.var_nl + locals.var_nstar);
        let assign32970_e55321: f64 = (assign32970_e55317 / assign32970_e55320);
        let (assign32970_e55346, assign32970_e55346_d_n0, assign32970_e55346_d_n2, assign32970_e55346_d_n3, assign32970_e55346_d_n4, assign32970_e55346_d_n5, assign32970_e55346_d_n6, assign32970_e55346_d_n7, assign32970_e55346_d_n8, assign32970_e55346_d_n9, assign32970_e55346_d_n10, assign32970_e55346_d_n11, assign32970_e55346_d_n13, assign32970_e55346_d_n14,) = {
            if (!(assign32970_e55321 > 1e-38)) {
                let assign32970_e55326: f64 = (-87.498233534);
                (assign32970_e55326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign32970_e55329: f64 = (locals.var_n0 + locals.var_nstar);
                let assign32970_e55332: f64 = (locals.var_nl + locals.var_nstar);
                let assign32970_e55333: f64 = (assign32970_e55329 / assign32970_e55332);
                let (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,) = {
                    if (assign32970_e55333 > 1e-38) {
                        let assign32970_e55338: f64 = (locals.var_n0 + locals.var_nstar);
                        let assign32970_e55341: f64 = (locals.var_nl + locals.var_nstar);
                        let assign32970_e55342: f64 = (assign32970_e55338 / assign32970_e55341);
                        let assign32970_e55343: f64 = (assign32970_e55342).ln();
                        (assign32970_e55343, (((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,)
            }
        };
        let assign32970_e55347: f64 = (locals.var_noiaeff * assign32970_e55346);
        (assign32970_e55347, ((locals.var_noiaeff_dn0 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n0)), ((locals.var_noiaeff_dn2 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n2)), ((locals.var_noiaeff_dn3 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n3)), ((locals.var_noiaeff_dn4 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n4)), ((locals.var_noiaeff_dn5 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n5)), ((locals.var_noiaeff_dn6 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n6)), ((locals.var_noiaeff_dn7 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n7)), ((locals.var_noiaeff_dn8 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n8)), ((locals.var_noiaeff_dn9 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n9)), ((locals.var_noiaeff_dn10 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n10)), ((locals.var_noiaeff_dn11 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n11)), ((locals.var_noiaeff_dn13 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n13)), ((locals.var_noiaeff_dn14 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32970_e55349;
        locals.var_t3_dn0 = assign32970_e55349_d_n0;
        locals.var_t3_dn2 = assign32970_e55349_d_n2;
        locals.var_t3_dn3 = assign32970_e55349_d_n3;
        locals.var_t3_dn4 = assign32970_e55349_d_n4;
        locals.var_t3_dn5 = assign32970_e55349_d_n5;
        locals.var_t3_dn6 = assign32970_e55349_d_n6;
        locals.var_t3_dn7 = assign32970_e55349_d_n7;
        locals.var_t3_dn8 = assign32970_e55349_d_n8;
        locals.var_t3_dn9 = assign32970_e55349_d_n9;
        locals.var_t3_dn10 = assign32970_e55349_d_n10;
        locals.var_t3_dn11 = assign32970_e55349_d_n11;
        locals.var_t3_dn13 = assign32970_e55349_d_n13;
        locals.var_t3_dn14 = assign32970_e55349_d_n14;

        let (assign32980_e55359, assign32980_e55359_d_n0, assign32980_e55359_d_n2, assign32980_e55359_d_n3, assign32980_e55359_d_n4, assign32980_e55359_d_n5, assign32980_e55359_d_n6, assign32980_e55359_d_n7, assign32980_e55359_d_n8, assign32980_e55359_d_n9, assign32980_e55359_d_n10, assign32980_e55359_d_n11, assign32980_e55359_d_n13, assign32980_e55359_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32980_e55356: f64 = (locals.var_n0 - locals.var_nl);
        let assign32980_e55357: f64 = (p.p1683 * assign32980_e55356);
        (assign32980_e55357, (p.p1683 * (locals.var_n0_dn0 - locals.var_nl_dn0)), (p.p1683 * (locals.var_n0_dn2 - locals.var_nl_dn2)), (p.p1683 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p1683 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p1683 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p1683 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p1683 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p1683 * (locals.var_n0_dn8 - locals.var_nl_dn8)), (p.p1683 * (locals.var_n0_dn9 - locals.var_nl_dn9)), (p.p1683 * (locals.var_n0_dn10 - locals.var_nl_dn10)), (p.p1683 * (locals.var_n0_dn11 - locals.var_nl_dn11)), (p.p1683 * (locals.var_n0_dn13 - locals.var_nl_dn13)), (p.p1683 * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32980_e55359;
        locals.var_t4_dn0 = assign32980_e55359_d_n0;
        locals.var_t4_dn2 = assign32980_e55359_d_n2;
        locals.var_t4_dn3 = assign32980_e55359_d_n3;
        locals.var_t4_dn4 = assign32980_e55359_d_n4;
        locals.var_t4_dn5 = assign32980_e55359_d_n5;
        locals.var_t4_dn6 = assign32980_e55359_d_n6;
        locals.var_t4_dn7 = assign32980_e55359_d_n7;
        locals.var_t4_dn8 = assign32980_e55359_d_n8;
        locals.var_t4_dn9 = assign32980_e55359_d_n9;
        locals.var_t4_dn10 = assign32980_e55359_d_n10;
        locals.var_t4_dn11 = assign32980_e55359_d_n11;
        locals.var_t4_dn13 = assign32980_e55359_d_n13;
        locals.var_t4_dn14 = assign32980_e55359_d_n14;

        let (assign32990_e55375, assign32990_e55375_d_n0, assign32990_e55375_d_n2, assign32990_e55375_d_n3, assign32990_e55375_d_n4, assign32990_e55375_d_n5, assign32990_e55375_d_n6, assign32990_e55375_d_n7, assign32990_e55375_d_n8, assign32990_e55375_d_n9, assign32990_e55375_d_n10, assign32990_e55375_d_n11, assign32990_e55375_d_n13, assign32990_e55375_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32990_e55365: f64 = (0.5 * p.p1684);
        let assign32990_e55368: f64 = (locals.var_n0 * locals.var_n0);
        let assign32990_e55371: f64 = (locals.var_nl * locals.var_nl);
        let assign32990_e55372: f64 = (assign32990_e55368 - assign32990_e55371);
        let assign32990_e55373: f64 = (assign32990_e55365 * assign32990_e55372);
        (assign32990_e55373, (assign32990_e55365 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32990_e55365 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32990_e55365 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32990_e55365 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32990_e55365 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32990_e55365 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32990_e55365 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32990_e55365 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32990_e55365 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32990_e55365 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32990_e55365 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32990_e55365 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32990_e55365 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32990_e55375;
        locals.var_t5_dn0 = assign32990_e55375_d_n0;
        locals.var_t5_dn2 = assign32990_e55375_d_n2;
        locals.var_t5_dn3 = assign32990_e55375_d_n3;
        locals.var_t5_dn4 = assign32990_e55375_d_n4;
        locals.var_t5_dn5 = assign32990_e55375_d_n5;
        locals.var_t5_dn6 = assign32990_e55375_d_n6;
        locals.var_t5_dn7 = assign32990_e55375_d_n7;
        locals.var_t5_dn8 = assign32990_e55375_d_n8;
        locals.var_t5_dn9 = assign32990_e55375_d_n9;
        locals.var_t5_dn10 = assign32990_e55375_d_n10;
        locals.var_t5_dn11 = assign32990_e55375_d_n11;
        locals.var_t5_dn13 = assign32990_e55375_d_n13;
        locals.var_t5_dn14 = assign32990_e55375_d_n14;

        let (assign33000_e55387, assign33000_e55387_d_n0, assign33000_e55387_d_n2, assign33000_e55387_d_n3, assign33000_e55387_d_n4, assign33000_e55387_d_n5, assign33000_e55387_d_n6, assign33000_e55387_d_n7, assign33000_e55387_d_n8, assign33000_e55387_d_n9, assign33000_e55387_d_n10, assign33000_e55387_d_n11, assign33000_e55387_d_n13, assign33000_e55387_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33000_e55381: f64 = (1.60219e-19 * locals.var_vtm);
        let assign33000_e55383: f64 = (assign33000_e55381 * locals.var_ids_v);
        let assign33000_e55385: f64 = (assign33000_e55383 * locals.var_ids_v);
        (assign33000_e55385, (((assign33000_e55381 * locals.var_ids_v_dn0) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn0)), (((assign33000_e55381 * locals.var_ids_v_dn2) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn2)), (((assign33000_e55381 * locals.var_ids_v_dn3) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn3)), (((((1.60219e-19 * locals.var_vtm_dn4) * locals.var_ids_v) + (assign33000_e55381 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn4)), (((assign33000_e55381 * locals.var_ids_v_dn5) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn5)), (((assign33000_e55381 * locals.var_ids_v_dn6) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn6)), (((assign33000_e55381 * locals.var_ids_v_dn7) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn7)), (((assign33000_e55381 * locals.var_ids_v_dn8) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn8)), (((assign33000_e55381 * locals.var_ids_v_dn9) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn9)), (((assign33000_e55381 * locals.var_ids_v_dn10) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn10)), (((assign33000_e55381 * locals.var_ids_v_dn11) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn11)), (((assign33000_e55381 * locals.var_ids_v_dn13) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn13)), (((assign33000_e55381 * locals.var_ids_v_dn14) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33000_e55387;
        locals.var_t6_dn0 = assign33000_e55387_d_n0;
        locals.var_t6_dn2 = assign33000_e55387_d_n2;
        locals.var_t6_dn3 = assign33000_e55387_d_n3;
        locals.var_t6_dn4 = assign33000_e55387_d_n4;
        locals.var_t6_dn5 = assign33000_e55387_d_n5;
        locals.var_t6_dn6 = assign33000_e55387_d_n6;
        locals.var_t6_dn7 = assign33000_e55387_d_n7;
        locals.var_t6_dn8 = assign33000_e55387_d_n8;
        locals.var_t6_dn9 = assign33000_e55387_d_n9;
        locals.var_t6_dn10 = assign33000_e55387_d_n10;
        locals.var_t6_dn11 = assign33000_e55387_d_n11;
        locals.var_t6_dn13 = assign33000_e55387_d_n13;
        locals.var_t6_dn14 = assign33000_e55387_d_n14;

        let (assign33010_e55399, assign33010_e55399_d_n0, assign33010_e55399_d_n2, assign33010_e55399_d_n3, assign33010_e55399_d_n4, assign33010_e55399_d_n5, assign33010_e55399_d_n6, assign33010_e55399_d_n7, assign33010_e55399_d_n8, assign33010_e55399_d_n9, assign33010_e55399_d_n10, assign33010_e55399_d_n11, assign33010_e55399_d_n13, assign33010_e55399_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33010_e55393: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign33010_e55395: f64 = (assign33010_e55393 * locals.var_weff0);
        let assign33010_e55397: f64 = (assign33010_e55395 * locals.var_nfintotal);
        (assign33010_e55397, (((10000000000.0 * locals.var_leffnoisq_dn0) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn2) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn3) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn4) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn5) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn6) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn7) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn8) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn9) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn10) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn11) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn13) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn14) * locals.var_weff0) * locals.var_nfintotal),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e55399;
        locals.var_t7_dn0 = assign33010_e55399_d_n0;
        locals.var_t7_dn2 = assign33010_e55399_d_n2;
        locals.var_t7_dn3 = assign33010_e55399_d_n3;
        locals.var_t7_dn4 = assign33010_e55399_d_n4;
        locals.var_t7_dn5 = assign33010_e55399_d_n5;
        locals.var_t7_dn6 = assign33010_e55399_d_n6;
        locals.var_t7_dn7 = assign33010_e55399_d_n7;
        locals.var_t7_dn8 = assign33010_e55399_d_n8;
        locals.var_t7_dn9 = assign33010_e55399_d_n9;
        locals.var_t7_dn10 = assign33010_e55399_d_n10;
        locals.var_t7_dn11 = assign33010_e55399_d_n11;
        locals.var_t7_dn13 = assign33010_e55399_d_n13;
        locals.var_t7_dn14 = assign33010_e55399_d_n14;

        let (assign33020_e55415, assign33020_e55415_d_n0, assign33020_e55415_d_n2, assign33020_e55415_d_n3, assign33020_e55415_d_n4, assign33020_e55415_d_n5, assign33020_e55415_d_n6, assign33020_e55415_d_n7, assign33020_e55415_d_n8, assign33020_e55415_d_n9, assign33020_e55415_d_n10, assign33020_e55415_d_n11, assign33020_e55415_d_n13, assign33020_e55415_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33020_e55406: f64 = (p.p1683 * locals.var_nl);
        let assign33020_e55407: f64 = (locals.var_noiaeff + assign33020_e55406);
        let assign33020_e55410: f64 = (p.p1684 * locals.var_nl);
        let assign33020_e55412: f64 = (assign33020_e55410 * locals.var_nl);
        let assign33020_e55413: f64 = (assign33020_e55407 + assign33020_e55412);
        (assign33020_e55413, ((locals.var_noiaeff_dn0 + (p.p1683 * locals.var_nl_dn0)) + (((p.p1684 * locals.var_nl_dn0) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn0))), ((locals.var_noiaeff_dn2 + (p.p1683 * locals.var_nl_dn2)) + (((p.p1684 * locals.var_nl_dn2) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn2))), ((locals.var_noiaeff_dn3 + (p.p1683 * locals.var_nl_dn3)) + (((p.p1684 * locals.var_nl_dn3) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn3))), ((locals.var_noiaeff_dn4 + (p.p1683 * locals.var_nl_dn4)) + (((p.p1684 * locals.var_nl_dn4) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn4))), ((locals.var_noiaeff_dn5 + (p.p1683 * locals.var_nl_dn5)) + (((p.p1684 * locals.var_nl_dn5) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn5))), ((locals.var_noiaeff_dn6 + (p.p1683 * locals.var_nl_dn6)) + (((p.p1684 * locals.var_nl_dn6) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn6))), ((locals.var_noiaeff_dn7 + (p.p1683 * locals.var_nl_dn7)) + (((p.p1684 * locals.var_nl_dn7) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn7))), ((locals.var_noiaeff_dn8 + (p.p1683 * locals.var_nl_dn8)) + (((p.p1684 * locals.var_nl_dn8) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn8))), ((locals.var_noiaeff_dn9 + (p.p1683 * locals.var_nl_dn9)) + (((p.p1684 * locals.var_nl_dn9) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn9))), ((locals.var_noiaeff_dn10 + (p.p1683 * locals.var_nl_dn10)) + (((p.p1684 * locals.var_nl_dn10) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn10))), ((locals.var_noiaeff_dn11 + (p.p1683 * locals.var_nl_dn11)) + (((p.p1684 * locals.var_nl_dn11) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn11))), ((locals.var_noiaeff_dn13 + (p.p1683 * locals.var_nl_dn13)) + (((p.p1684 * locals.var_nl_dn13) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn13))), ((locals.var_noiaeff_dn14 + (p.p1683 * locals.var_nl_dn14)) + (((p.p1684 * locals.var_nl_dn14) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e55415;
        locals.var_t8_dn0 = assign33020_e55415_d_n0;
        locals.var_t8_dn2 = assign33020_e55415_d_n2;
        locals.var_t8_dn3 = assign33020_e55415_d_n3;
        locals.var_t8_dn4 = assign33020_e55415_d_n4;
        locals.var_t8_dn5 = assign33020_e55415_d_n5;
        locals.var_t8_dn6 = assign33020_e55415_d_n6;
        locals.var_t8_dn7 = assign33020_e55415_d_n7;
        locals.var_t8_dn8 = assign33020_e55415_d_n8;
        locals.var_t8_dn9 = assign33020_e55415_d_n9;
        locals.var_t8_dn10 = assign33020_e55415_d_n10;
        locals.var_t8_dn11 = assign33020_e55415_d_n11;
        locals.var_t8_dn13 = assign33020_e55415_d_n13;
        locals.var_t8_dn14 = assign33020_e55415_d_n14;

        let (assign33030_e55427, assign33030_e55427_d_n0, assign33030_e55427_d_n2, assign33030_e55427_d_n3, assign33030_e55427_d_n4, assign33030_e55427_d_n5, assign33030_e55427_d_n6, assign33030_e55427_d_n7, assign33030_e55427_d_n8, assign33030_e55427_d_n9, assign33030_e55427_d_n10, assign33030_e55427_d_n11, assign33030_e55427_d_n13, assign33030_e55427_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33030_e55421: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55424: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55425: f64 = (assign33030_e55421 * assign33030_e55424);
        (assign33030_e55425, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn13, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign33030_e55427;
        locals.var_t9_dn0 = assign33030_e55427_d_n0;
        locals.var_t9_dn2 = assign33030_e55427_d_n2;
        locals.var_t9_dn3 = assign33030_e55427_d_n3;
        locals.var_t9_dn4 = assign33030_e55427_d_n4;
        locals.var_t9_dn5 = assign33030_e55427_d_n5;
        locals.var_t9_dn6 = assign33030_e55427_d_n6;
        locals.var_t9_dn7 = assign33030_e55427_d_n7;
        locals.var_t9_dn8 = assign33030_e55427_d_n8;
        locals.var_t9_dn9 = assign33030_e55427_d_n9;
        locals.var_t9_dn10 = assign33030_e55427_d_n10;
        locals.var_t9_dn11 = assign33030_e55427_d_n11;
        locals.var_t9_dn13 = assign33030_e55427_d_n13;
        locals.var_t9_dn14 = assign33030_e55427_d_n14;

        let (assign33040_e55451, assign33040_e55451_d_n0, assign33040_e55451_d_n2, assign33040_e55451_d_n3, assign33040_e55451_d_n4, assign33040_e55451_d_n5, assign33040_e55451_d_n6, assign33040_e55451_d_n7, assign33040_e55451_d_n8, assign33040_e55451_d_n9, assign33040_e55451_d_n10, assign33040_e55451_d_n11, assign33040_e55451_d_n13, assign33040_e55451_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33040_e55433: f64 = (locals.var_t1 / locals.var_t2);
        let assign33040_e55436: f64 = (locals.var_t3 + locals.var_t4);
        let assign33040_e55438: f64 = (assign33040_e55436 + locals.var_t5);
        let assign33040_e55439: f64 = (assign33040_e55433 * assign33040_e55438);
        let assign33040_e55442: f64 = (locals.var_t6 / locals.var_t7);
        let assign33040_e55444: f64 = (assign33040_e55442 * locals.var_delclm);
        let assign33040_e55446: f64 = (assign33040_e55444 * locals.var_t8);
        let assign33040_e55448: f64 = (assign33040_e55446 / locals.var_t9);
        let assign33040_e55449: f64 = (assign33040_e55439 + assign33040_e55448);
        (assign33040_e55449, ((((((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn0 + locals.var_t4_dn0) + locals.var_t5_dn0))) + ((((((((((locals.var_t6_dn0 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn0)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn0)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn0)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn2 + locals.var_t4_dn2) + locals.var_t5_dn2))) + ((((((((((locals.var_t6_dn2 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn2)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn2)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn2)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn3 + locals.var_t4_dn3) + locals.var_t5_dn3))) + ((((((((((locals.var_t6_dn3 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn3)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn3)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn3)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn4 + locals.var_t4_dn4) + locals.var_t5_dn4))) + ((((((((((locals.var_t6_dn4 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn4)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn4)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn5 + locals.var_t4_dn5) + locals.var_t5_dn5))) + ((((((((((locals.var_t6_dn5 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn5)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn5)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn6 + locals.var_t4_dn6) + locals.var_t5_dn6))) + ((((((((((locals.var_t6_dn6 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn6)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn6)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn7 + locals.var_t4_dn7) + locals.var_t5_dn7))) + ((((((((((locals.var_t6_dn7 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn7)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn7)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn7)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn8 + locals.var_t4_dn8) + locals.var_t5_dn8))) + ((((((((((locals.var_t6_dn8 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn8)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn8)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn9 + locals.var_t4_dn9) + locals.var_t5_dn9))) + ((((((((((locals.var_t6_dn9 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn9)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn9)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn9)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn10 + locals.var_t4_dn10) + locals.var_t5_dn10))) + ((((((((((locals.var_t6_dn10 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn10)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn10)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn10)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn11 + locals.var_t4_dn11) + locals.var_t5_dn11))) + ((((((((((locals.var_t6_dn11 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn11)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn11)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn11)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn13 + locals.var_t4_dn13) + locals.var_t5_dn13))) + ((((((((((locals.var_t6_dn13 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn13)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn13)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn13)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn14 + locals.var_t4_dn14) + locals.var_t5_dn14))) + ((((((((((locals.var_t6_dn14 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn14)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn14)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn14)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign33040_e55451;
        locals.var_ssi_dn0 = assign33040_e55451_d_n0;
        locals.var_ssi_dn2 = assign33040_e55451_d_n2;
        locals.var_ssi_dn3 = assign33040_e55451_d_n3;
        locals.var_ssi_dn4 = assign33040_e55451_d_n4;
        locals.var_ssi_dn5 = assign33040_e55451_d_n5;
        locals.var_ssi_dn6 = assign33040_e55451_d_n6;
        locals.var_ssi_dn7 = assign33040_e55451_d_n7;
        locals.var_ssi_dn8 = assign33040_e55451_d_n8;
        locals.var_ssi_dn9 = assign33040_e55451_d_n9;
        locals.var_ssi_dn10 = assign33040_e55451_d_n10;
        locals.var_ssi_dn11 = assign33040_e55451_d_n11;
        locals.var_ssi_dn13 = assign33040_e55451_d_n13;
        locals.var_ssi_dn14 = assign33040_e55451_d_n14;

        let (assign33050_e55461, assign33050_e55461_d_n0, assign33050_e55461_d_n2, assign33050_e55461_d_n3, assign33050_e55461_d_n4, assign33050_e55461_d_n5, assign33050_e55461_d_n6, assign33050_e55461_d_n7, assign33050_e55461_d_n8, assign33050_e55461_d_n9, assign33050_e55461_d_n10, assign33050_e55461_d_n11, assign33050_e55461_d_n13, assign33050_e55461_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33050_e55457: f64 = (locals.var_noiaeff * 1.60219e-19);
        let assign33050_e55459: f64 = (assign33050_e55457 * locals.var_vtm);
        (assign33050_e55459, ((locals.var_noiaeff_dn0 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn2 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn3 * 1.60219e-19) * locals.var_vtm), (((locals.var_noiaeff_dn4 * 1.60219e-19) * locals.var_vtm) + (assign33050_e55457 * locals.var_vtm_dn4)), ((locals.var_noiaeff_dn5 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn6 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn7 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn8 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn9 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn10 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn11 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn13 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn14 * 1.60219e-19) * locals.var_vtm),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn13, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign33050_e55461;
        locals.var_t10_dn0 = assign33050_e55461_d_n0;
        locals.var_t10_dn2 = assign33050_e55461_d_n2;
        locals.var_t10_dn3 = assign33050_e55461_d_n3;
        locals.var_t10_dn4 = assign33050_e55461_d_n4;
        locals.var_t10_dn5 = assign33050_e55461_d_n5;
        locals.var_t10_dn6 = assign33050_e55461_d_n6;
        locals.var_t10_dn7 = assign33050_e55461_d_n7;
        locals.var_t10_dn8 = assign33050_e55461_d_n8;
        locals.var_t10_dn9 = assign33050_e55461_d_n9;
        locals.var_t10_dn10 = assign33050_e55461_d_n10;
        locals.var_t10_dn11 = assign33050_e55461_d_n11;
        locals.var_t10_dn13 = assign33050_e55461_d_n13;
        locals.var_t10_dn14 = assign33050_e55461_d_n14;

        let (assign33060_e55477, assign33060_e55477_d_n0, assign33060_e55477_d_n2, assign33060_e55477_d_n3, assign33060_e55477_d_n4, assign33060_e55477_d_n5, assign33060_e55477_d_n6, assign33060_e55477_d_n7, assign33060_e55477_d_n8, assign33060_e55477_d_n9, assign33060_e55477_d_n10, assign33060_e55477_d_n11, assign33060_e55477_d_n13, assign33060_e55477_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33060_e55467: f64 = (locals.var_weff0 * locals.var_nfintotal);
        let assign33060_e55469: f64 = (assign33060_e55467 * locals.var_leffnoi);
        let assign33060_e55471: f64 = (assign33060_e55469 * 10000000000.0);
        let assign33060_e55473: f64 = (assign33060_e55471 * locals.var_nstar);
        let assign33060_e55475: f64 = (assign33060_e55473 * locals.var_nstar);
        (assign33060_e55475, ((((((assign33060_e55467 * locals.var_leffnoi_dn0) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn0)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn0)), ((((((assign33060_e55467 * locals.var_leffnoi_dn2) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn2)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn2)), ((((((assign33060_e55467 * locals.var_leffnoi_dn3) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn3)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn3)), ((((((assign33060_e55467 * locals.var_leffnoi_dn4) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn4)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn4)), ((((((assign33060_e55467 * locals.var_leffnoi_dn5) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn5)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn5)), ((((((assign33060_e55467 * locals.var_leffnoi_dn6) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn6)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn6)), ((((((assign33060_e55467 * locals.var_leffnoi_dn7) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn7)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn7)), ((((((assign33060_e55467 * locals.var_leffnoi_dn8) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn8)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn8)), ((((((assign33060_e55467 * locals.var_leffnoi_dn9) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn9)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn9)), ((((((assign33060_e55467 * locals.var_leffnoi_dn10) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn10)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn10)), ((((((assign33060_e55467 * locals.var_leffnoi_dn11) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn11)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn11)), ((((((assign33060_e55467 * locals.var_leffnoi_dn13) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn13)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn13)), ((((((assign33060_e55467 * locals.var_leffnoi_dn14) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn14)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn13, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign33060_e55477;
        locals.var_t11_dn0 = assign33060_e55477_d_n0;
        locals.var_t11_dn2 = assign33060_e55477_d_n2;
        locals.var_t11_dn3 = assign33060_e55477_d_n3;
        locals.var_t11_dn4 = assign33060_e55477_d_n4;
        locals.var_t11_dn5 = assign33060_e55477_d_n5;
        locals.var_t11_dn6 = assign33060_e55477_d_n6;
        locals.var_t11_dn7 = assign33060_e55477_d_n7;
        locals.var_t11_dn8 = assign33060_e55477_d_n8;
        locals.var_t11_dn9 = assign33060_e55477_d_n9;
        locals.var_t11_dn10 = assign33060_e55477_d_n10;
        locals.var_t11_dn11 = assign33060_e55477_d_n11;
        locals.var_t11_dn13 = assign33060_e55477_d_n13;
        locals.var_t11_dn14 = assign33060_e55477_d_n14;

        let (assign33070_e55489, assign33070_e55489_d_n0, assign33070_e55489_d_n2, assign33070_e55489_d_n3, assign33070_e55489_d_n4, assign33070_e55489_d_n5, assign33070_e55489_d_n6, assign33070_e55489_d_n7, assign33070_e55489_d_n8, assign33070_e55489_d_n9, assign33070_e55489_d_n10, assign33070_e55489_d_n11, assign33070_e55489_d_n13, assign33070_e55489_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33070_e55483: f64 = (locals.var_t10 / locals.var_t11);
        let assign33070_e55485: f64 = (assign33070_e55483 * locals.var_ids_v);
        let assign33070_e55487: f64 = (assign33070_e55485 * locals.var_ids_v);
        (assign33070_e55487, (((((((locals.var_t10_dn0 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn0)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn0)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn0)), (((((((locals.var_t10_dn2 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn2)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn2)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn2)), (((((((locals.var_t10_dn3 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn3)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn3)), (((((((locals.var_t10_dn4 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn4)), (((((((locals.var_t10_dn5 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn5)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn5)), (((((((locals.var_t10_dn6 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn6)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn6)), (((((((locals.var_t10_dn7 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn7)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn7)), (((((((locals.var_t10_dn8 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn8)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn8)), (((((((locals.var_t10_dn9 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn9)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn9)), (((((((locals.var_t10_dn10 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn10)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn10)), (((((((locals.var_t10_dn11 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn11)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn11)), (((((((locals.var_t10_dn13 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn13)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn13)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn13)), (((((((locals.var_t10_dn14 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn14)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn14)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign33070_e55489;
        locals.var_swi_dn0 = assign33070_e55489_d_n0;
        locals.var_swi_dn2 = assign33070_e55489_d_n2;
        locals.var_swi_dn3 = assign33070_e55489_d_n3;
        locals.var_swi_dn4 = assign33070_e55489_d_n4;
        locals.var_swi_dn5 = assign33070_e55489_d_n5;
        locals.var_swi_dn6 = assign33070_e55489_d_n6;
        locals.var_swi_dn7 = assign33070_e55489_d_n7;
        locals.var_swi_dn8 = assign33070_e55489_d_n8;
        locals.var_swi_dn9 = assign33070_e55489_d_n9;
        locals.var_swi_dn10 = assign33070_e55489_d_n10;
        locals.var_swi_dn11 = assign33070_e55489_d_n11;
        locals.var_swi_dn13 = assign33070_e55489_d_n13;
        locals.var_swi_dn14 = assign33070_e55489_d_n14;

    }

    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33080_e55497, assign33080_e55497_d_n0, assign33080_e55497_d_n2, assign33080_e55497_d_n3, assign33080_e55497_d_n4, assign33080_e55497_d_n5, assign33080_e55497_d_n6, assign33080_e55497_d_n7, assign33080_e55497_d_n8, assign33080_e55497_d_n9, assign33080_e55497_d_n10, assign33080_e55497_d_n11, assign33080_e55497_d_n13, assign33080_e55497_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33080_e55495: f64 = (locals.var_swi + locals.var_ssi);
        (assign33080_e55495, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33080_e55497;
        locals.var_t1_dn0 = assign33080_e55497_d_n0;
        locals.var_t1_dn2 = assign33080_e55497_d_n2;
        locals.var_t1_dn3 = assign33080_e55497_d_n3;
        locals.var_t1_dn4 = assign33080_e55497_d_n4;
        locals.var_t1_dn5 = assign33080_e55497_d_n5;
        locals.var_t1_dn6 = assign33080_e55497_d_n6;
        locals.var_t1_dn7 = assign33080_e55497_d_n7;
        locals.var_t1_dn8 = assign33080_e55497_d_n8;
        locals.var_t1_dn9 = assign33080_e55497_d_n9;
        locals.var_t1_dn10 = assign33080_e55497_d_n10;
        locals.var_t1_dn11 = assign33080_e55497_d_n11;
        locals.var_t1_dn13 = assign33080_e55497_d_n13;
        locals.var_t1_dn14 = assign33080_e55497_d_n14;

        let assign33120_e55524: f64 = if p.p79 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign33120_e55524;

        let (assign33130_e55535, assign33130_e55535_d_n0, assign33130_e55535_d_n2, assign33130_e55535_d_n3, assign33130_e55535_d_n4, assign33130_e55535_d_n5, assign33130_e55535_d_n6, assign33130_e55535_d_n7, assign33130_e55535_d_n8, assign33130_e55535_d_n9, assign33130_e55535_d_n10, assign33130_e55535_d_n11, assign33130_e55535_d_n13, assign33130_e55535_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33130_e55533: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign33130_e55533, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33130_e55535;
        locals.var_t1_dn0 = assign33130_e55535_d_n0;
        locals.var_t1_dn2 = assign33130_e55535_d_n2;
        locals.var_t1_dn3 = assign33130_e55535_d_n3;
        locals.var_t1_dn4 = assign33130_e55535_d_n4;
        locals.var_t1_dn5 = assign33130_e55535_d_n5;
        locals.var_t1_dn6 = assign33130_e55535_d_n6;
        locals.var_t1_dn7 = assign33130_e55535_d_n7;
        locals.var_t1_dn8 = assign33130_e55535_d_n8;
        locals.var_t1_dn9 = assign33130_e55535_d_n9;
        locals.var_t1_dn10 = assign33130_e55535_d_n10;
        locals.var_t1_dn11 = assign33130_e55535_d_n11;
        locals.var_t1_dn13 = assign33130_e55535_d_n13;
        locals.var_t1_dn14 = assign33130_e55535_d_n14;

        let (assign33140_e55548, assign33140_e55548_d_n0, assign33140_e55548_d_n2, assign33140_e55548_d_n3, assign33140_e55548_d_n4, assign33140_e55548_d_n5, assign33140_e55548_d_n6, assign33140_e55548_d_n7, assign33140_e55548_d_n8, assign33140_e55548_d_n9, assign33140_e55548_d_n10, assign33140_e55548_d_n11, assign33140_e55548_d_n13, assign33140_e55548_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33140_e55545: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign33140_e55546: f64 = (1.0 + assign33140_e55545);
        (assign33140_e55546, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33140_e55548;
        locals.var_t2_dn0 = assign33140_e55548_d_n0;
        locals.var_t2_dn2 = assign33140_e55548_d_n2;
        locals.var_t2_dn3 = assign33140_e55548_d_n3;
        locals.var_t2_dn4 = assign33140_e55548_d_n4;
        locals.var_t2_dn5 = assign33140_e55548_d_n5;
        locals.var_t2_dn6 = assign33140_e55548_d_n6;
        locals.var_t2_dn7 = assign33140_e55548_d_n7;
        locals.var_t2_dn8 = assign33140_e55548_d_n8;
        locals.var_t2_dn9 = assign33140_e55548_d_n9;
        locals.var_t2_dn10 = assign33140_e55548_d_n10;
        locals.var_t2_dn11 = assign33140_e55548_d_n11;
        locals.var_t2_dn13 = assign33140_e55548_d_n13;
        locals.var_t2_dn14 = assign33140_e55548_d_n14;

        let (assign33150_e55559, assign33150_e55559_d_n0, assign33150_e55559_d_n2, assign33150_e55559_d_n3, assign33150_e55559_d_n4, assign33150_e55559_d_n5, assign33150_e55559_d_n6, assign33150_e55559_d_n7, assign33150_e55559_d_n8, assign33150_e55559_d_n9, assign33150_e55559_d_n10, assign33150_e55559_d_n11, assign33150_e55559_d_n13, assign33150_e55559_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33150_e55557: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign33150_e55557, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33150_e55559;
        locals.var_t3_dn0 = assign33150_e55559_d_n0;
        locals.var_t3_dn2 = assign33150_e55559_d_n2;
        locals.var_t3_dn3 = assign33150_e55559_d_n3;
        locals.var_t3_dn4 = assign33150_e55559_d_n4;
        locals.var_t3_dn5 = assign33150_e55559_d_n5;
        locals.var_t3_dn6 = assign33150_e55559_d_n6;
        locals.var_t3_dn7 = assign33150_e55559_d_n7;
        locals.var_t3_dn8 = assign33150_e55559_d_n8;
        locals.var_t3_dn9 = assign33150_e55559_d_n9;
        locals.var_t3_dn10 = assign33150_e55559_d_n10;
        locals.var_t3_dn11 = assign33150_e55559_d_n11;
        locals.var_t3_dn13 = assign33150_e55559_d_n13;
        locals.var_t3_dn14 = assign33150_e55559_d_n14;

        let (assign33160_e55570, assign33160_e55570_d_n0, assign33160_e55570_d_n2, assign33160_e55570_d_n3, assign33160_e55570_d_n4, assign33160_e55570_d_n5, assign33160_e55570_d_n6, assign33160_e55570_d_n7, assign33160_e55570_d_n8, assign33160_e55570_d_n9, assign33160_e55570_d_n10, assign33160_e55570_d_n11, assign33160_e55570_d_n13, assign33160_e55570_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33160_e55568: f64 = (locals.var_t3 / p.p1682);
        (assign33160_e55568, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33160_e55570;
        locals.var_t4_dn0 = assign33160_e55570_d_n0;
        locals.var_t4_dn2 = assign33160_e55570_d_n2;
        locals.var_t4_dn3 = assign33160_e55570_d_n3;
        locals.var_t4_dn4 = assign33160_e55570_d_n4;
        locals.var_t4_dn5 = assign33160_e55570_d_n5;
        locals.var_t4_dn6 = assign33160_e55570_d_n6;
        locals.var_t4_dn7 = assign33160_e55570_d_n7;
        locals.var_t4_dn8 = assign33160_e55570_d_n8;
        locals.var_t4_dn9 = assign33160_e55570_d_n9;
        locals.var_t4_dn10 = assign33160_e55570_d_n10;
        locals.var_t4_dn11 = assign33160_e55570_d_n11;
        locals.var_t4_dn13 = assign33160_e55570_d_n13;
        locals.var_t4_dn14 = assign33160_e55570_d_n14;

        let (assign33170_e55598, assign33170_e55598_d_n0, assign33170_e55598_d_n2, assign33170_e55598_d_n3, assign33170_e55598_d_n4, assign33170_e55598_d_n5, assign33170_e55598_d_n6, assign33170_e55598_d_n7, assign33170_e55598_d_n8, assign33170_e55598_d_n9, assign33170_e55598_d_n10, assign33170_e55598_d_n11, assign33170_e55598_d_n13, assign33170_e55598_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33170_e55580: f64 = (locals.var_t4 + 1.0);
        let assign33170_e55583: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55586: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55587: f64 = (assign33170_e55583 * assign33170_e55586);
        let assign33170_e55590: f64 = (0.25 * p.p1688);
        let assign33170_e55592: f64 = (assign33170_e55590 * p.p1688);
        let assign33170_e55593: f64 = (assign33170_e55587 + assign33170_e55592);
        let assign33170_e55594: f64 = (assign33170_e55593).sqrt();
        let assign33170_e55595: f64 = (assign33170_e55580 + assign33170_e55594);
        let assign33170_e55596: f64 = (0.5 * assign33170_e55595);
        (assign33170_e55596, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn0)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn2)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn3)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn4)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn5)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn6)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn7)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn8)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn9)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn10)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn11)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn13)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn14)) / (2.0 * assign33170_e55594)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33170_e55598;
        locals.var_t5_dn0 = assign33170_e55598_d_n0;
        locals.var_t5_dn2 = assign33170_e55598_d_n2;
        locals.var_t5_dn3 = assign33170_e55598_d_n3;
        locals.var_t5_dn4 = assign33170_e55598_d_n4;
        locals.var_t5_dn5 = assign33170_e55598_d_n5;
        locals.var_t5_dn6 = assign33170_e55598_d_n6;
        locals.var_t5_dn7 = assign33170_e55598_d_n7;
        locals.var_t5_dn8 = assign33170_e55598_d_n8;
        locals.var_t5_dn9 = assign33170_e55598_d_n9;
        locals.var_t5_dn10 = assign33170_e55598_d_n10;
        locals.var_t5_dn11 = assign33170_e55598_d_n11;
        locals.var_t5_dn13 = assign33170_e55598_d_n13;
        locals.var_t5_dn14 = assign33170_e55598_d_n14;

        let (assign33180_e55609, assign33180_e55609_d_n0, assign33180_e55609_d_n2, assign33180_e55609_d_n3, assign33180_e55609_d_n4, assign33180_e55609_d_n5, assign33180_e55609_d_n6, assign33180_e55609_d_n7, assign33180_e55609_d_n8, assign33180_e55609_d_n9, assign33180_e55609_d_n10, assign33180_e55609_d_n11, assign33180_e55609_d_n13, assign33180_e55609_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33180_e55607: f64 = (p.p1682 * locals.var_t5);
        (assign33180_e55607, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign33180_e55609;
        locals.var_noiaeff_dn0 = assign33180_e55609_d_n0;
        locals.var_noiaeff_dn2 = assign33180_e55609_d_n2;
        locals.var_noiaeff_dn3 = assign33180_e55609_d_n3;
        locals.var_noiaeff_dn4 = assign33180_e55609_d_n4;
        locals.var_noiaeff_dn5 = assign33180_e55609_d_n5;
        locals.var_noiaeff_dn6 = assign33180_e55609_d_n6;
        locals.var_noiaeff_dn7 = assign33180_e55609_d_n7;
        locals.var_noiaeff_dn8 = assign33180_e55609_d_n8;
        locals.var_noiaeff_dn9 = assign33180_e55609_d_n9;
        locals.var_noiaeff_dn10 = assign33180_e55609_d_n10;
        locals.var_noiaeff_dn11 = assign33180_e55609_d_n11;
        locals.var_noiaeff_dn13 = assign33180_e55609_d_n13;
        locals.var_noiaeff_dn14 = assign33180_e55609_d_n14;

        let (assign33190_e55622, assign33190_e55622_d_n0, assign33190_e55622_d_n2, assign33190_e55622_d_n3, assign33190_e55622_d_n4, assign33190_e55622_d_n5, assign33190_e55622_d_n6, assign33190_e55622_d_n7, assign33190_e55622_d_n8, assign33190_e55622_d_n9, assign33190_e55622_d_n10, assign33190_e55622_d_n11, assign33190_e55622_d_n13, assign33190_e55622_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33190_e55618: f64 = (2.0 * locals.var_vtm);
        let assign33190_e55620: f64 = (assign33190_e55618 / locals.var_esatl);
        (assign33190_e55620, (-((assign33190_e55618 * locals.var_esatl_dn0) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn2) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn3) / (locals.var_esatl * locals.var_esatl))), ((((2.0 * locals.var_vtm_dn4) * locals.var_esatl) - (assign33190_e55618 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (-((assign33190_e55618 * locals.var_esatl_dn5) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn6) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn7) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn8) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn9) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn10) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn11) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn13) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn14) / (locals.var_esatl * locals.var_esatl))),)
    } else {
        (locals.var_lambdac_fn2, locals.var_lambdac_fn2_dn0, locals.var_lambdac_fn2_dn2, locals.var_lambdac_fn2_dn3, locals.var_lambdac_fn2_dn4, locals.var_lambdac_fn2_dn5, locals.var_lambdac_fn2_dn6, locals.var_lambdac_fn2_dn7, locals.var_lambdac_fn2_dn8, locals.var_lambdac_fn2_dn9, locals.var_lambdac_fn2_dn10, locals.var_lambdac_fn2_dn11, locals.var_lambdac_fn2_dn13, locals.var_lambdac_fn2_dn14,)
    }
};
        locals.var_lambdac_fn2 = assign33190_e55622;
        locals.var_lambdac_fn2_dn0 = assign33190_e55622_d_n0;
        locals.var_lambdac_fn2_dn2 = assign33190_e55622_d_n2;
        locals.var_lambdac_fn2_dn3 = assign33190_e55622_d_n3;
        locals.var_lambdac_fn2_dn4 = assign33190_e55622_d_n4;
        locals.var_lambdac_fn2_dn5 = assign33190_e55622_d_n5;
        locals.var_lambdac_fn2_dn6 = assign33190_e55622_d_n6;
        locals.var_lambdac_fn2_dn7 = assign33190_e55622_d_n7;
        locals.var_lambdac_fn2_dn8 = assign33190_e55622_d_n8;
        locals.var_lambdac_fn2_dn9 = assign33190_e55622_d_n9;
        locals.var_lambdac_fn2_dn10 = assign33190_e55622_d_n10;
        locals.var_lambdac_fn2_dn11 = assign33190_e55622_d_n11;
        locals.var_lambdac_fn2_dn13 = assign33190_e55622_d_n13;
        locals.var_lambdac_fn2_dn14 = assign33190_e55622_d_n14;

        let (assign33200_e55635, assign33200_e55635_d_n0, assign33200_e55635_d_n2, assign33200_e55635_d_n3, assign33200_e55635_d_n4, assign33200_e55635_d_n5, assign33200_e55635_d_n6, assign33200_e55635_d_n7, assign33200_e55635_d_n8, assign33200_e55635_d_n9, assign33200_e55635_d_n10, assign33200_e55635_d_n11, assign33200_e55635_d_n13, assign33200_e55635_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33200_e55632: f64 = (locals.var_lambdac_fn2 * locals.var_dqi);
        let assign33200_e55633: f64 = (1.0 + assign33200_e55632);
        (assign33200_e55633, ((locals.var_lambdac_fn2_dn0 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn0)), ((locals.var_lambdac_fn2_dn2 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn2)), ((locals.var_lambdac_fn2_dn3 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn3)), ((locals.var_lambdac_fn2_dn4 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn4)), ((locals.var_lambdac_fn2_dn5 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn5)), ((locals.var_lambdac_fn2_dn6 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn6)), ((locals.var_lambdac_fn2_dn7 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn7)), ((locals.var_lambdac_fn2_dn8 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn8)), ((locals.var_lambdac_fn2_dn9 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn9)), ((locals.var_lambdac_fn2_dn10 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn10)), ((locals.var_lambdac_fn2_dn11 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn11)), ((locals.var_lambdac_fn2_dn13 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn13)), ((locals.var_lambdac_fn2_dn14 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33200_e55635;
        locals.var_t1_dn0 = assign33200_e55635_d_n0;
        locals.var_t1_dn2 = assign33200_e55635_d_n2;
        locals.var_t1_dn3 = assign33200_e55635_d_n3;
        locals.var_t1_dn4 = assign33200_e55635_d_n4;
        locals.var_t1_dn5 = assign33200_e55635_d_n5;
        locals.var_t1_dn6 = assign33200_e55635_d_n6;
        locals.var_t1_dn7 = assign33200_e55635_d_n7;
        locals.var_t1_dn8 = assign33200_e55635_d_n8;
        locals.var_t1_dn9 = assign33200_e55635_d_n9;
        locals.var_t1_dn10 = assign33200_e55635_d_n10;
        locals.var_t1_dn11 = assign33200_e55635_d_n11;
        locals.var_t1_dn13 = assign33200_e55635_d_n13;
        locals.var_t1_dn14 = assign33200_e55635_d_n14;

        let (assign33210_e55648, assign33210_e55648_d_n0, assign33210_e55648_d_n2, assign33210_e55648_d_n3, assign33210_e55648_d_n4, assign33210_e55648_d_n5, assign33210_e55648_d_n6, assign33210_e55648_d_n7, assign33210_e55648_d_n8, assign33210_e55648_d_n9, assign33210_e55648_d_n10, assign33210_e55648_d_n11, assign33210_e55648_d_n13, assign33210_e55648_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33210_e55645: f64 = (p.p1685 * locals.var_dqi);
        let assign33210_e55646: f64 = (1.0 + assign33210_e55645);
        (assign33210_e55646, (p.p1685 * locals.var_dqi_dn0), (p.p1685 * locals.var_dqi_dn2), (p.p1685 * locals.var_dqi_dn3), (p.p1685 * locals.var_dqi_dn4), (p.p1685 * locals.var_dqi_dn5), (p.p1685 * locals.var_dqi_dn6), (p.p1685 * locals.var_dqi_dn7), (p.p1685 * locals.var_dqi_dn8), (p.p1685 * locals.var_dqi_dn9), (p.p1685 * locals.var_dqi_dn10), (p.p1685 * locals.var_dqi_dn11), (p.p1685 * locals.var_dqi_dn13), (p.p1685 * locals.var_dqi_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33210_e55648;
        locals.var_t2_dn0 = assign33210_e55648_d_n0;
        locals.var_t2_dn2 = assign33210_e55648_d_n2;
        locals.var_t2_dn3 = assign33210_e55648_d_n3;
        locals.var_t2_dn4 = assign33210_e55648_d_n4;
        locals.var_t2_dn5 = assign33210_e55648_d_n5;
        locals.var_t2_dn6 = assign33210_e55648_d_n6;
        locals.var_t2_dn7 = assign33210_e55648_d_n7;
        locals.var_t2_dn8 = assign33210_e55648_d_n8;
        locals.var_t2_dn9 = assign33210_e55648_d_n9;
        locals.var_t2_dn10 = assign33210_e55648_d_n10;
        locals.var_t2_dn11 = assign33210_e55648_d_n11;
        locals.var_t2_dn13 = assign33210_e55648_d_n13;
        locals.var_t2_dn14 = assign33210_e55648_d_n14;

        let assign33230_e55674: f64 = if ((locals.var_t1 > 0.0) && (locals.var_t2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign33230_e55674;

        let (assign33250_e55736, assign33250_e55736_d_n0, assign33250_e55736_d_n2, assign33250_e55736_d_n3, assign33250_e55736_d_n4, assign33250_e55736_d_n5, assign33250_e55736_d_n6, assign33250_e55736_d_n7, assign33250_e55736_d_n8, assign33250_e55736_d_n9, assign33250_e55736_d_n10, assign33250_e55736_d_n11, assign33250_e55736_d_n13, assign33250_e55736_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33250_e55699: f64 = (locals.var_qis + 0.5);
        let assign33250_e55702: f64 = (locals.var_qid + 0.5);
        let assign33250_e55703: f64 = (assign33250_e55699 / assign33250_e55702);
        let (assign33250_e55728, assign33250_e55728_d_n0, assign33250_e55728_d_n2, assign33250_e55728_d_n3, assign33250_e55728_d_n4, assign33250_e55728_d_n5, assign33250_e55728_d_n6, assign33250_e55728_d_n7, assign33250_e55728_d_n8, assign33250_e55728_d_n9, assign33250_e55728_d_n10, assign33250_e55728_d_n11, assign33250_e55728_d_n13, assign33250_e55728_d_n14,) = {
            if (!(assign33250_e55703 > 1e-38)) {
                let assign33250_e55708: f64 = (-87.498233534);
                (assign33250_e55708, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33250_e55711: f64 = (locals.var_qis + 0.5);
                let assign33250_e55714: f64 = (locals.var_qid + 0.5);
                let assign33250_e55715: f64 = (assign33250_e55711 / assign33250_e55714);
                let (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,) = {
                    if (assign33250_e55715 > 1e-38) {
                        let assign33250_e55720: f64 = (locals.var_qis + 0.5);
                        let assign33250_e55723: f64 = (locals.var_qid + 0.5);
                        let assign33250_e55724: f64 = (assign33250_e55720 / assign33250_e55723);
                        let assign33250_e55725: f64 = (assign33250_e55724).ln();
                        (assign33250_e55725, ((((locals.var_qis_dn0 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn0)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn2 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn2)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn3 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn3)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn4 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn4)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn5 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn5)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn6 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn6)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn7 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn7)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn8 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn8)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn9 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn9)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn10 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn10)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn11 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn11)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn13 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn13)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn14 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn14)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,)
            }
        };
        let assign33250_e55731: f64 = (locals.var_qis + locals.var_qid);
        let assign33250_e55733: f64 = (assign33250_e55731 + 1.0);
        let assign33250_e55734: f64 = (assign33250_e55728 * assign33250_e55733);
        (assign33250_e55734, ((assign33250_e55728_d_n0 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn0 + locals.var_qid_dn0))), ((assign33250_e55728_d_n2 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn2 + locals.var_qid_dn2))), ((assign33250_e55728_d_n3 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn3 + locals.var_qid_dn3))), ((assign33250_e55728_d_n4 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn4 + locals.var_qid_dn4))), ((assign33250_e55728_d_n5 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn5 + locals.var_qid_dn5))), ((assign33250_e55728_d_n6 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn6 + locals.var_qid_dn6))), ((assign33250_e55728_d_n7 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn7 + locals.var_qid_dn7))), ((assign33250_e55728_d_n8 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn8 + locals.var_qid_dn8))), ((assign33250_e55728_d_n9 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn9 + locals.var_qid_dn9))), ((assign33250_e55728_d_n10 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn10 + locals.var_qid_dn10))), ((assign33250_e55728_d_n11 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn11 + locals.var_qid_dn11))), ((assign33250_e55728_d_n13 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn13 + locals.var_qid_dn13))), ((assign33250_e55728_d_n14 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn14 + locals.var_qid_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33250_e55736;
        locals.var_t3_dn0 = assign33250_e55736_d_n0;
        locals.var_t3_dn2 = assign33250_e55736_d_n2;
        locals.var_t3_dn3 = assign33250_e55736_d_n3;
        locals.var_t3_dn4 = assign33250_e55736_d_n4;
        locals.var_t3_dn5 = assign33250_e55736_d_n5;
        locals.var_t3_dn6 = assign33250_e55736_d_n6;
        locals.var_t3_dn7 = assign33250_e55736_d_n7;
        locals.var_t3_dn8 = assign33250_e55736_d_n8;
        locals.var_t3_dn9 = assign33250_e55736_d_n9;
        locals.var_t3_dn10 = assign33250_e55736_d_n10;
        locals.var_t3_dn11 = assign33250_e55736_d_n11;
        locals.var_t3_dn13 = assign33250_e55736_d_n13;
        locals.var_t3_dn14 = assign33250_e55736_d_n14;

        let (assign33260_e55751, assign33260_e55751_d_n0, assign33260_e55751_d_n2, assign33260_e55751_d_n3, assign33260_e55751_d_n4, assign33260_e55751_d_n5, assign33260_e55751_d_n6, assign33260_e55751_d_n7, assign33260_e55751_d_n8, assign33260_e55751_d_n9, assign33260_e55751_d_n10, assign33260_e55751_d_n11, assign33260_e55751_d_n13, assign33260_e55751_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33260_e55748: f64 = (locals.var_qis - locals.var_qid);
        let assign33260_e55749: f64 = (2.0 * assign33260_e55748);
        (assign33260_e55749, (2.0 * (locals.var_qis_dn0 - locals.var_qid_dn0)), (2.0 * (locals.var_qis_dn2 - locals.var_qid_dn2)), (2.0 * (locals.var_qis_dn3 - locals.var_qid_dn3)), (2.0 * (locals.var_qis_dn4 - locals.var_qid_dn4)), (2.0 * (locals.var_qis_dn5 - locals.var_qid_dn5)), (2.0 * (locals.var_qis_dn6 - locals.var_qid_dn6)), (2.0 * (locals.var_qis_dn7 - locals.var_qid_dn7)), (2.0 * (locals.var_qis_dn8 - locals.var_qid_dn8)), (2.0 * (locals.var_qis_dn9 - locals.var_qid_dn9)), (2.0 * (locals.var_qis_dn10 - locals.var_qid_dn10)), (2.0 * (locals.var_qis_dn11 - locals.var_qid_dn11)), (2.0 * (locals.var_qis_dn13 - locals.var_qid_dn13)), (2.0 * (locals.var_qis_dn14 - locals.var_qid_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33260_e55751;
        locals.var_t4_dn0 = assign33260_e55751_d_n0;
        locals.var_t4_dn2 = assign33260_e55751_d_n2;
        locals.var_t4_dn3 = assign33260_e55751_d_n3;
        locals.var_t4_dn4 = assign33260_e55751_d_n4;
        locals.var_t4_dn5 = assign33260_e55751_d_n5;
        locals.var_t4_dn6 = assign33260_e55751_d_n6;
        locals.var_t4_dn7 = assign33260_e55751_d_n7;
        locals.var_t4_dn8 = assign33260_e55751_d_n8;
        locals.var_t4_dn9 = assign33260_e55751_d_n9;
        locals.var_t4_dn10 = assign33260_e55751_d_n10;
        locals.var_t4_dn11 = assign33260_e55751_d_n11;
        locals.var_t4_dn13 = assign33260_e55751_d_n13;
        locals.var_t4_dn14 = assign33260_e55751_d_n14;

        let assign33300_e55814: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign33300_e55814;

        let assign33310_e55817: f64 = if p.p72 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign33310_e55817;

        let (assign33320_e55823, assign33320_e55823_d_n0, assign33320_e55823_d_n2, assign33320_e55823_d_n3, assign33320_e55823_d_n4, assign33320_e55823_d_n5, assign33320_e55823_d_n6, assign33320_e55823_d_n7, assign33320_e55823_d_n8, assign33320_e55823_d_n9, assign33320_e55823_d_n10, assign33320_e55823_d_n11, assign33320_e55823_d_n13, assign33320_e55823_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33320_e55821: f64 = (locals.var_ueff * locals.var_qinv);
        (assign33320_e55821, ((locals.var_ueff_dn0 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn0)), ((locals.var_ueff_dn2 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn2)), ((locals.var_ueff_dn3 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn3)), ((locals.var_ueff_dn4 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn4)), ((locals.var_ueff_dn5 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn5)), ((locals.var_ueff_dn6 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn6)), ((locals.var_ueff_dn7 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn7)), ((locals.var_ueff_dn8 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn8)), ((locals.var_ueff_dn9 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn9)), ((locals.var_ueff_dn10 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn10)), ((locals.var_ueff_dn11 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn11)), ((locals.var_ueff_dn13 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn13)), ((locals.var_ueff_dn14 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33320_e55823;
        locals.var_t0_dn0 = assign33320_e55823_d_n0;
        locals.var_t0_dn2 = assign33320_e55823_d_n2;
        locals.var_t0_dn3 = assign33320_e55823_d_n3;
        locals.var_t0_dn4 = assign33320_e55823_d_n4;
        locals.var_t0_dn5 = assign33320_e55823_d_n5;
        locals.var_t0_dn6 = assign33320_e55823_d_n6;
        locals.var_t0_dn7 = assign33320_e55823_d_n7;
        locals.var_t0_dn8 = assign33320_e55823_d_n8;
        locals.var_t0_dn9 = assign33320_e55823_d_n9;
        locals.var_t0_dn10 = assign33320_e55823_d_n10;
        locals.var_t0_dn11 = assign33320_e55823_d_n11;
        locals.var_t0_dn13 = assign33320_e55823_d_n13;
        locals.var_t0_dn14 = assign33320_e55823_d_n14;

        let (assign33330_e55833, assign33330_e55833_d_n0, assign33330_e55833_d_n2, assign33330_e55833_d_n3, assign33330_e55833_d_n4, assign33330_e55833_d_n5, assign33330_e55833_d_n6, assign33330_e55833_d_n7, assign33330_e55833_d_n8, assign33330_e55833_d_n9, assign33330_e55833_d_n10, assign33330_e55833_d_n11, assign33330_e55833_d_n13, assign33330_e55833_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33330_e55827: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign33330_e55830: f64 = (locals.var_leff_1 * locals.var_leff_1);
        let assign33330_e55831: f64 = (assign33330_e55827 + assign33330_e55830);
        (assign33330_e55831, (((locals.var_t0_dn0 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn0)) + ((locals.var_leff_1_dn0 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn0))), (((locals.var_t0_dn2 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn2)) + ((locals.var_leff_1_dn2 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn2))), (((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3)) + ((locals.var_leff_1_dn3 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn3))), (((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4)) + ((locals.var_leff_1_dn4 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn4))), (((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5)) + ((locals.var_leff_1_dn5 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn5))), (((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6)) + ((locals.var_leff_1_dn6 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn6))), (((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7)) + ((locals.var_leff_1_dn7 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn7))), (((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8)) + ((locals.var_leff_1_dn8 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn8))), (((locals.var_t0_dn9 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn9)) + ((locals.var_leff_1_dn9 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn9))), (((locals.var_t0_dn10 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn10)) + ((locals.var_leff_1_dn10 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn10))), (((locals.var_t0_dn11 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn11)) + ((locals.var_leff_1_dn11 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn11))), (((locals.var_t0_dn13 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn13)) + ((locals.var_leff_1_dn13 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn13))), (((locals.var_t0_dn14 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn14)) + ((locals.var_leff_1_dn14 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33330_e55833;
        locals.var_t1_dn0 = assign33330_e55833_d_n0;
        locals.var_t1_dn2 = assign33330_e55833_d_n2;
        locals.var_t1_dn3 = assign33330_e55833_d_n3;
        locals.var_t1_dn4 = assign33330_e55833_d_n4;
        locals.var_t1_dn5 = assign33330_e55833_d_n5;
        locals.var_t1_dn6 = assign33330_e55833_d_n6;
        locals.var_t1_dn7 = assign33330_e55833_d_n7;
        locals.var_t1_dn8 = assign33330_e55833_d_n8;
        locals.var_t1_dn9 = assign33330_e55833_d_n9;
        locals.var_t1_dn10 = assign33330_e55833_d_n10;
        locals.var_t1_dn11 = assign33330_e55833_d_n11;
        locals.var_t1_dn13 = assign33330_e55833_d_n13;
        locals.var_t1_dn14 = assign33330_e55833_d_n14;

        let (assign33360_e55860, assign33360_e55860_d_n0, assign33360_e55860_d_n2, assign33360_e55860_d_n3, assign33360_e55860_d_n4, assign33360_e55860_d_n5, assign33360_e55860_d_n6, assign33360_e55860_d_n7, assign33360_e55860_d_n8, assign33360_e55860_d_n9, assign33360_e55860_d_n10, assign33360_e55860_d_n11, assign33360_e55860_d_n13, assign33360_e55860_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33360_e55858: f64 = (locals.var_qia / locals.var_esatl);
        (assign33360_e55858, (((locals.var_qia_dn0 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn0)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn2 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn2)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn3 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn4 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn5 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn6 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn7 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn8 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn9 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn10 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn11 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn13 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn13)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn14 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn14)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33360_e55860;
        locals.var_t0_dn0 = assign33360_e55860_d_n0;
        locals.var_t0_dn2 = assign33360_e55860_d_n2;
        locals.var_t0_dn3 = assign33360_e55860_d_n3;
        locals.var_t0_dn4 = assign33360_e55860_d_n4;
        locals.var_t0_dn5 = assign33360_e55860_d_n5;
        locals.var_t0_dn6 = assign33360_e55860_d_n6;
        locals.var_t0_dn7 = assign33360_e55860_d_n7;
        locals.var_t0_dn8 = assign33360_e55860_d_n8;
        locals.var_t0_dn9 = assign33360_e55860_d_n9;
        locals.var_t0_dn10 = assign33360_e55860_d_n10;
        locals.var_t0_dn11 = assign33360_e55860_d_n11;
        locals.var_t0_dn13 = assign33360_e55860_d_n13;
        locals.var_t0_dn14 = assign33360_e55860_d_n14;

        let (assign33370_e55869, assign33370_e55869_d_n0, assign33370_e55869_d_n2, assign33370_e55869_d_n3, assign33370_e55869_d_n4, assign33370_e55869_d_n5, assign33370_e55869_d_n6, assign33370_e55869_d_n7, assign33370_e55869_d_n8, assign33370_e55869_d_n9, assign33370_e55869_d_n10, assign33370_e55869_d_n11, assign33370_e55869_d_n13, assign33370_e55869_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33370_e55867: f64 = (locals.var_t0 * locals.var_t0);
        (assign33370_e55867, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33370_e55869;
        locals.var_t0_dn0 = assign33370_e55869_d_n0;
        locals.var_t0_dn2 = assign33370_e55869_d_n2;
        locals.var_t0_dn3 = assign33370_e55869_d_n3;
        locals.var_t0_dn4 = assign33370_e55869_d_n4;
        locals.var_t0_dn5 = assign33370_e55869_d_n5;
        locals.var_t0_dn6 = assign33370_e55869_d_n6;
        locals.var_t0_dn7 = assign33370_e55869_d_n7;
        locals.var_t0_dn8 = assign33370_e55869_d_n8;
        locals.var_t0_dn9 = assign33370_e55869_d_n9;
        locals.var_t0_dn10 = assign33370_e55869_d_n10;
        locals.var_t0_dn11 = assign33370_e55869_d_n11;
        locals.var_t0_dn13 = assign33370_e55869_d_n13;
        locals.var_t0_dn14 = assign33370_e55869_d_n14;

        let (assign33380_e55884, assign33380_e55884_d_n0, assign33380_e55884_d_n2, assign33380_e55884_d_n3, assign33380_e55884_d_n4, assign33380_e55884_d_n5, assign33380_e55884_d_n6, assign33380_e55884_d_n7, assign33380_e55884_d_n8, assign33380_e55884_d_n9, assign33380_e55884_d_n10, assign33380_e55884_d_n11, assign33380_e55884_d_n13, assign33380_e55884_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33380_e55878: f64 = (locals.var_t0 * p.p1709);
        let assign33380_e55880: f64 = (assign33380_e55878 * locals.var_leff_1);
        let assign33380_e55881: f64 = (1.0 + assign33380_e55880);
        let assign33380_e55882: f64 = (p.p1708 * assign33380_e55881);
        (assign33380_e55882, (p.p1708 * (((locals.var_t0_dn0 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn0))), (p.p1708 * (((locals.var_t0_dn2 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn2))), (p.p1708 * (((locals.var_t0_dn3 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn3))), (p.p1708 * (((locals.var_t0_dn4 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn4))), (p.p1708 * (((locals.var_t0_dn5 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn5))), (p.p1708 * (((locals.var_t0_dn6 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn6))), (p.p1708 * (((locals.var_t0_dn7 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn7))), (p.p1708 * (((locals.var_t0_dn8 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn8))), (p.p1708 * (((locals.var_t0_dn9 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn9))), (p.p1708 * (((locals.var_t0_dn10 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn10))), (p.p1708 * (((locals.var_t0_dn11 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn11))), (p.p1708 * (((locals.var_t0_dn13 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn13))), (p.p1708 * (((locals.var_t0_dn14 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noibeta, locals.var_noibeta_dn0, locals.var_noibeta_dn2, locals.var_noibeta_dn3, locals.var_noibeta_dn4, locals.var_noibeta_dn5, locals.var_noibeta_dn6, locals.var_noibeta_dn7, locals.var_noibeta_dn8, locals.var_noibeta_dn9, locals.var_noibeta_dn10, locals.var_noibeta_dn11, locals.var_noibeta_dn13, locals.var_noibeta_dn14,)
    }
};
        locals.var_noibeta = assign33380_e55884;
        locals.var_noibeta_dn0 = assign33380_e55884_d_n0;
        locals.var_noibeta_dn2 = assign33380_e55884_d_n2;
        locals.var_noibeta_dn3 = assign33380_e55884_d_n3;
        locals.var_noibeta_dn4 = assign33380_e55884_d_n4;
        locals.var_noibeta_dn5 = assign33380_e55884_d_n5;
        locals.var_noibeta_dn6 = assign33380_e55884_d_n6;
        locals.var_noibeta_dn7 = assign33380_e55884_d_n7;
        locals.var_noibeta_dn8 = assign33380_e55884_d_n8;
        locals.var_noibeta_dn9 = assign33380_e55884_d_n9;
        locals.var_noibeta_dn10 = assign33380_e55884_d_n10;
        locals.var_noibeta_dn11 = assign33380_e55884_d_n11;
        locals.var_noibeta_dn13 = assign33380_e55884_d_n13;
        locals.var_noibeta_dn14 = assign33380_e55884_d_n14;

        let (assign33390_e55899, assign33390_e55899_d_n0, assign33390_e55899_d_n2, assign33390_e55899_d_n3, assign33390_e55899_d_n4, assign33390_e55899_d_n5, assign33390_e55899_d_n6, assign33390_e55899_d_n7, assign33390_e55899_d_n8, assign33390_e55899_d_n9, assign33390_e55899_d_n10, assign33390_e55899_d_n11, assign33390_e55899_d_n13, assign33390_e55899_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33390_e55893: f64 = (locals.var_t0 * p.p1711);
        let assign33390_e55895: f64 = (assign33390_e55893 * locals.var_leff_1);
        let assign33390_e55896: f64 = (1.0 + assign33390_e55895);
        let assign33390_e55897: f64 = (p.p1710 * assign33390_e55896);
        (assign33390_e55897, (p.p1710 * (((locals.var_t0_dn0 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn0))), (p.p1710 * (((locals.var_t0_dn2 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn2))), (p.p1710 * (((locals.var_t0_dn3 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn3))), (p.p1710 * (((locals.var_t0_dn4 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn4))), (p.p1710 * (((locals.var_t0_dn5 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn5))), (p.p1710 * (((locals.var_t0_dn6 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn6))), (p.p1710 * (((locals.var_t0_dn7 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn7))), (p.p1710 * (((locals.var_t0_dn8 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn8))), (p.p1710 * (((locals.var_t0_dn9 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn9))), (p.p1710 * (((locals.var_t0_dn10 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn10))), (p.p1710 * (((locals.var_t0_dn11 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn11))), (p.p1710 * (((locals.var_t0_dn13 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn13))), (p.p1710 * (((locals.var_t0_dn14 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noitheta, locals.var_noitheta_dn0, locals.var_noitheta_dn2, locals.var_noitheta_dn3, locals.var_noitheta_dn4, locals.var_noitheta_dn5, locals.var_noitheta_dn6, locals.var_noitheta_dn7, locals.var_noitheta_dn8, locals.var_noitheta_dn9, locals.var_noitheta_dn10, locals.var_noitheta_dn11, locals.var_noitheta_dn13, locals.var_noitheta_dn14,)
    }
};
        locals.var_noitheta = assign33390_e55899;
        locals.var_noitheta_dn0 = assign33390_e55899_d_n0;
        locals.var_noitheta_dn2 = assign33390_e55899_d_n2;
        locals.var_noitheta_dn3 = assign33390_e55899_d_n3;
        locals.var_noitheta_dn4 = assign33390_e55899_d_n4;
        locals.var_noitheta_dn5 = assign33390_e55899_d_n5;
        locals.var_noitheta_dn6 = assign33390_e55899_d_n6;
        locals.var_noitheta_dn7 = assign33390_e55899_d_n7;
        locals.var_noitheta_dn8 = assign33390_e55899_d_n8;
        locals.var_noitheta_dn9 = assign33390_e55899_d_n9;
        locals.var_noitheta_dn10 = assign33390_e55899_d_n10;
        locals.var_noitheta_dn11 = assign33390_e55899_d_n11;
        locals.var_noitheta_dn13 = assign33390_e55899_d_n13;
        locals.var_noitheta_dn14 = assign33390_e55899_d_n14;

        let (assign33400_e55914, assign33400_e55914_d_n0, assign33400_e55914_d_n2, assign33400_e55914_d_n3, assign33400_e55914_d_n4, assign33400_e55914_d_n5, assign33400_e55914_d_n6, assign33400_e55914_d_n7, assign33400_e55914_d_n8, assign33400_e55914_d_n9, assign33400_e55914_d_n10, assign33400_e55914_d_n11, assign33400_e55914_d_n13, assign33400_e55914_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33400_e55908: f64 = (locals.var_t0 * p.p1713);
        let assign33400_e55910: f64 = (assign33400_e55908 * locals.var_leff_1);
        let assign33400_e55911: f64 = (1.0 + assign33400_e55910);
        let assign33400_e55912: f64 = (p.p1712 * assign33400_e55911);
        (assign33400_e55912, (p.p1712 * (((locals.var_t0_dn0 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn0))), (p.p1712 * (((locals.var_t0_dn2 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn2))), (p.p1712 * (((locals.var_t0_dn3 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn3))), (p.p1712 * (((locals.var_t0_dn4 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn4))), (p.p1712 * (((locals.var_t0_dn5 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn5))), (p.p1712 * (((locals.var_t0_dn6 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn6))), (p.p1712 * (((locals.var_t0_dn7 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn7))), (p.p1712 * (((locals.var_t0_dn8 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn8))), (p.p1712 * (((locals.var_t0_dn9 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn9))), (p.p1712 * (((locals.var_t0_dn10 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn10))), (p.p1712 * (((locals.var_t0_dn11 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn11))), (p.p1712 * (((locals.var_t0_dn13 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn13))), (p.p1712 * (((locals.var_t0_dn14 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noicorr, locals.var_noicorr_dn0, locals.var_noicorr_dn2, locals.var_noicorr_dn3, locals.var_noicorr_dn4, locals.var_noicorr_dn5, locals.var_noicorr_dn6, locals.var_noicorr_dn7, locals.var_noicorr_dn8, locals.var_noicorr_dn9, locals.var_noicorr_dn10, locals.var_noicorr_dn11, locals.var_noicorr_dn13, locals.var_noicorr_dn14,)
    }
};
        locals.var_noicorr = assign33400_e55914;
        locals.var_noicorr_dn0 = assign33400_e55914_d_n0;
        locals.var_noicorr_dn2 = assign33400_e55914_d_n2;
        locals.var_noicorr_dn3 = assign33400_e55914_d_n3;
        locals.var_noicorr_dn4 = assign33400_e55914_d_n4;
        locals.var_noicorr_dn5 = assign33400_e55914_d_n5;
        locals.var_noicorr_dn6 = assign33400_e55914_d_n6;
        locals.var_noicorr_dn7 = assign33400_e55914_d_n7;
        locals.var_noicorr_dn8 = assign33400_e55914_d_n8;
        locals.var_noicorr_dn9 = assign33400_e55914_d_n9;
        locals.var_noicorr_dn10 = assign33400_e55914_d_n10;
        locals.var_noicorr_dn11 = assign33400_e55914_d_n11;
        locals.var_noicorr_dn13 = assign33400_e55914_d_n13;
        locals.var_noicorr_dn14 = assign33400_e55914_d_n14;

    }

    pub(super) fn stamp_transient_block_124(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33410_e55929, assign33410_e55929_d_n0, assign33410_e55929_d_n2, assign33410_e55929_d_n3, assign33410_e55929_d_n4, assign33410_e55929_d_n5, assign33410_e55929_d_n6, assign33410_e55929_d_n7, assign33410_e55929_d_n8, assign33410_e55929_d_n9, assign33410_e55929_d_n10, assign33410_e55929_d_n11, assign33410_e55929_d_n13, assign33410_e55929_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33410_e55923: f64 = (locals.var_t0 * p.p1715);
        let assign33410_e55925: f64 = (assign33410_e55923 * locals.var_leff_1);
        let assign33410_e55926: f64 = (1.0 + assign33410_e55925);
        let assign33410_e55927: f64 = (p.p1714 * assign33410_e55926);
        (assign33410_e55927, (p.p1714 * (((locals.var_t0_dn0 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn0))), (p.p1714 * (((locals.var_t0_dn2 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn2))), (p.p1714 * (((locals.var_t0_dn3 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn3))), (p.p1714 * (((locals.var_t0_dn4 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn4))), (p.p1714 * (((locals.var_t0_dn5 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn5))), (p.p1714 * (((locals.var_t0_dn6 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn6))), (p.p1714 * (((locals.var_t0_dn7 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn7))), (p.p1714 * (((locals.var_t0_dn8 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn8))), (p.p1714 * (((locals.var_t0_dn9 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn9))), (p.p1714 * (((locals.var_t0_dn10 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn10))), (p.p1714 * (((locals.var_t0_dn11 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn11))), (p.p1714 * (((locals.var_t0_dn13 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn13))), (p.p1714 * (((locals.var_t0_dn14 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noilowid, locals.var_noilowid_dn0, locals.var_noilowid_dn2, locals.var_noilowid_dn3, locals.var_noilowid_dn4, locals.var_noilowid_dn5, locals.var_noilowid_dn6, locals.var_noilowid_dn7, locals.var_noilowid_dn8, locals.var_noilowid_dn9, locals.var_noilowid_dn10, locals.var_noilowid_dn11, locals.var_noilowid_dn13, locals.var_noilowid_dn14,)
    }
};
        locals.var_noilowid = assign33410_e55929;
        locals.var_noilowid_dn0 = assign33410_e55929_d_n0;
        locals.var_noilowid_dn2 = assign33410_e55929_d_n2;
        locals.var_noilowid_dn3 = assign33410_e55929_d_n3;
        locals.var_noilowid_dn4 = assign33410_e55929_d_n4;
        locals.var_noilowid_dn5 = assign33410_e55929_d_n5;
        locals.var_noilowid_dn6 = assign33410_e55929_d_n6;
        locals.var_noilowid_dn7 = assign33410_e55929_d_n7;
        locals.var_noilowid_dn8 = assign33410_e55929_d_n8;
        locals.var_noilowid_dn9 = assign33410_e55929_d_n9;
        locals.var_noilowid_dn10 = assign33410_e55929_d_n10;
        locals.var_noilowid_dn11 = assign33410_e55929_d_n11;
        locals.var_noilowid_dn13 = assign33410_e55929_d_n13;
        locals.var_noilowid_dn14 = assign33410_e55929_d_n14;

        let (assign33420_e55940, assign33420_e55940_d_n0, assign33420_e55940_d_n2, assign33420_e55940_d_n3, assign33420_e55940_d_n4, assign33420_e55940_d_n5, assign33420_e55940_d_n6, assign33420_e55940_d_n7, assign33420_e55940_d_n8, assign33420_e55940_d_n9, assign33420_e55940_d_n10, assign33420_e55940_d_n11, assign33420_e55940_d_n13, assign33420_e55940_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33420_e55936: f64 = (3.0 * locals.var_noibeta);
        let assign33420_e55938: f64 = (assign33420_e55936 * locals.var_noibeta);
        (assign33420_e55938, (((3.0 * locals.var_noibeta_dn0) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn0)), (((3.0 * locals.var_noibeta_dn2) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn2)), (((3.0 * locals.var_noibeta_dn3) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn3)), (((3.0 * locals.var_noibeta_dn4) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn4)), (((3.0 * locals.var_noibeta_dn5) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn5)), (((3.0 * locals.var_noibeta_dn6) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn6)), (((3.0 * locals.var_noibeta_dn7) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn7)), (((3.0 * locals.var_noibeta_dn8) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn8)), (((3.0 * locals.var_noibeta_dn9) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn9)), (((3.0 * locals.var_noibeta_dn10) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn10)), (((3.0 * locals.var_noibeta_dn11) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn11)), (((3.0 * locals.var_noibeta_dn13) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn13)), (((3.0 * locals.var_noibeta_dn14) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33420_e55940;
        locals.var_t1_dn0 = assign33420_e55940_d_n0;
        locals.var_t1_dn2 = assign33420_e55940_d_n2;
        locals.var_t1_dn3 = assign33420_e55940_d_n3;
        locals.var_t1_dn4 = assign33420_e55940_d_n4;
        locals.var_t1_dn5 = assign33420_e55940_d_n5;
        locals.var_t1_dn6 = assign33420_e55940_d_n6;
        locals.var_t1_dn7 = assign33420_e55940_d_n7;
        locals.var_t1_dn8 = assign33420_e55940_d_n8;
        locals.var_t1_dn9 = assign33420_e55940_d_n9;
        locals.var_t1_dn10 = assign33420_e55940_d_n10;
        locals.var_t1_dn11 = assign33420_e55940_d_n11;
        locals.var_t1_dn13 = assign33420_e55940_d_n13;
        locals.var_t1_dn14 = assign33420_e55940_d_n14;

        let (assign33430_e55951, assign33430_e55951_d_n0, assign33430_e55951_d_n2, assign33430_e55951_d_n3, assign33430_e55951_d_n4, assign33430_e55951_d_n5, assign33430_e55951_d_n6, assign33430_e55951_d_n7, assign33430_e55951_d_n8, assign33430_e55951_d_n9, assign33430_e55951_d_n10, assign33430_e55951_d_n11, assign33430_e55951_d_n13, assign33430_e55951_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33430_e55947: f64 = (7.5 * locals.var_noitheta);
        let assign33430_e55949: f64 = (assign33430_e55947 * locals.var_noitheta);
        (assign33430_e55949, (((7.5 * locals.var_noitheta_dn0) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn0)), (((7.5 * locals.var_noitheta_dn2) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn2)), (((7.5 * locals.var_noitheta_dn3) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn3)), (((7.5 * locals.var_noitheta_dn4) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn4)), (((7.5 * locals.var_noitheta_dn5) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn5)), (((7.5 * locals.var_noitheta_dn6) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn6)), (((7.5 * locals.var_noitheta_dn7) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn7)), (((7.5 * locals.var_noitheta_dn8) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn8)), (((7.5 * locals.var_noitheta_dn9) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn9)), (((7.5 * locals.var_noitheta_dn10) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn10)), (((7.5 * locals.var_noitheta_dn11) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn11)), (((7.5 * locals.var_noitheta_dn13) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn13)), (((7.5 * locals.var_noitheta_dn14) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33430_e55951;
        locals.var_t2_dn0 = assign33430_e55951_d_n0;
        locals.var_t2_dn2 = assign33430_e55951_d_n2;
        locals.var_t2_dn3 = assign33430_e55951_d_n3;
        locals.var_t2_dn4 = assign33430_e55951_d_n4;
        locals.var_t2_dn5 = assign33430_e55951_d_n5;
        locals.var_t2_dn6 = assign33430_e55951_d_n6;
        locals.var_t2_dn7 = assign33430_e55951_d_n7;
        locals.var_t2_dn8 = assign33430_e55951_d_n8;
        locals.var_t2_dn9 = assign33430_e55951_d_n9;
        locals.var_t2_dn10 = assign33430_e55951_d_n10;
        locals.var_t2_dn11 = assign33430_e55951_d_n11;
        locals.var_t2_dn13 = assign33430_e55951_d_n13;
        locals.var_t2_dn14 = assign33430_e55951_d_n14;

        let (assign33440_e55960, assign33440_e55960_d_n0, assign33440_e55960_d_n2, assign33440_e55960_d_n3, assign33440_e55960_d_n4, assign33440_e55960_d_n5, assign33440_e55960_d_n6, assign33440_e55960_d_n7, assign33440_e55960_d_n8, assign33440_e55960_d_n9, assign33440_e55960_d_n10, assign33440_e55960_d_n11, assign33440_e55960_d_n13, assign33440_e55960_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33440_e55958: f64 = (2.5298 * locals.var_noicorr);
        (assign33440_e55958, (2.5298 * locals.var_noicorr_dn0), (2.5298 * locals.var_noicorr_dn2), (2.5298 * locals.var_noicorr_dn3), (2.5298 * locals.var_noicorr_dn4), (2.5298 * locals.var_noicorr_dn5), (2.5298 * locals.var_noicorr_dn6), (2.5298 * locals.var_noicorr_dn7), (2.5298 * locals.var_noicorr_dn8), (2.5298 * locals.var_noicorr_dn9), (2.5298 * locals.var_noicorr_dn10), (2.5298 * locals.var_noicorr_dn11), (2.5298 * locals.var_noicorr_dn13), (2.5298 * locals.var_noicorr_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33440_e55960;
        locals.var_t3_dn0 = assign33440_e55960_d_n0;
        locals.var_t3_dn2 = assign33440_e55960_d_n2;
        locals.var_t3_dn3 = assign33440_e55960_d_n3;
        locals.var_t3_dn4 = assign33440_e55960_d_n4;
        locals.var_t3_dn5 = assign33440_e55960_d_n5;
        locals.var_t3_dn6 = assign33440_e55960_d_n6;
        locals.var_t3_dn7 = assign33440_e55960_d_n7;
        locals.var_t3_dn8 = assign33440_e55960_d_n8;
        locals.var_t3_dn9 = assign33440_e55960_d_n9;
        locals.var_t3_dn10 = assign33440_e55960_d_n10;
        locals.var_t3_dn11 = assign33440_e55960_d_n11;
        locals.var_t3_dn13 = assign33440_e55960_d_n13;
        locals.var_t3_dn14 = assign33440_e55960_d_n14;

        let (assign33450_e55975, assign33450_e55975_d_n0, assign33450_e55975_d_n2, assign33450_e55975_d_n3, assign33450_e55975_d_n4, assign33450_e55975_d_n5, assign33450_e55975_d_n6, assign33450_e55975_d_n7, assign33450_e55975_d_n8, assign33450_e55975_d_n9, assign33450_e55975_d_n10, assign33450_e55975_d_n11, assign33450_e55975_d_n13, assign33450_e55975_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33450_e55967: f64 = (locals.var_qid / locals.var_qis);
        let assign33450_e55971: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign33450_e55972: f64 = (1.0 - assign33450_e55971);
        let assign33450_e55973: f64 = (assign33450_e55967 * assign33450_e55972);
        (assign33450_e55973, (((((locals.var_qid_dn0 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn2 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn3 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn4 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn5 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn6 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn7 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn8 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn9 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn10 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn11 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn13 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn14 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat))))),)
    } else {
        (locals.var_noieta, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    }
};
        locals.var_noieta = assign33450_e55975;
        locals.var_noieta_dn0 = assign33450_e55975_d_n0;
        locals.var_noieta_dn2 = assign33450_e55975_d_n2;
        locals.var_noieta_dn3 = assign33450_e55975_d_n3;
        locals.var_noieta_dn4 = assign33450_e55975_d_n4;
        locals.var_noieta_dn5 = assign33450_e55975_d_n5;
        locals.var_noieta_dn6 = assign33450_e55975_d_n6;
        locals.var_noieta_dn7 = assign33450_e55975_d_n7;
        locals.var_noieta_dn8 = assign33450_e55975_d_n8;
        locals.var_noieta_dn9 = assign33450_e55975_d_n9;
        locals.var_noieta_dn10 = assign33450_e55975_d_n10;
        locals.var_noieta_dn11 = assign33450_e55975_d_n11;
        locals.var_noieta_dn13 = assign33450_e55975_d_n13;
        locals.var_noieta_dn14 = assign33450_e55975_d_n14;

        let (assign33460_e55986, assign33460_e55986_d_n0, assign33460_e55986_d_n2, assign33460_e55986_d_n3, assign33460_e55986_d_n4, assign33460_e55986_d_n5, assign33460_e55986_d_n6, assign33460_e55986_d_n7, assign33460_e55986_d_n8, assign33460_e55986_d_n9, assign33460_e55986_d_n10, assign33460_e55986_d_n11, assign33460_e55986_d_n13, assign33460_e55986_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33460_e55982: f64 = (locals.var_dvsat * locals.var_dvsat);
        let assign33460_e55984: f64 = (assign33460_e55982 * locals.var_dvsat);
        (assign33460_e55984, ((((locals.var_dvsat_dn0 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn0)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn0)), ((((locals.var_dvsat_dn2 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn2)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn2)), ((((locals.var_dvsat_dn3 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn3)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn3)), ((((locals.var_dvsat_dn4 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn4)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn4)), ((((locals.var_dvsat_dn5 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn5)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn5)), ((((locals.var_dvsat_dn6 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn6)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn6)), ((((locals.var_dvsat_dn7 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn7)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn7)), ((((locals.var_dvsat_dn8 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn8)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn8)), ((((locals.var_dvsat_dn9 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn9)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn9)), ((((locals.var_dvsat_dn10 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn10)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn10)), ((((locals.var_dvsat_dn11 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn11)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn11)), ((((locals.var_dvsat_dn13 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn13)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn13)), ((((locals.var_dvsat_dn14 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn14)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn14)),)
    } else {
        (locals.var_dvsat3, locals.var_dvsat3_dn0, locals.var_dvsat3_dn2, locals.var_dvsat3_dn3, locals.var_dvsat3_dn4, locals.var_dvsat3_dn5, locals.var_dvsat3_dn6, locals.var_dvsat3_dn7, locals.var_dvsat3_dn8, locals.var_dvsat3_dn9, locals.var_dvsat3_dn10, locals.var_dvsat3_dn11, locals.var_dvsat3_dn13, locals.var_dvsat3_dn14,)
    }
};
        locals.var_dvsat3 = assign33460_e55986;
        locals.var_dvsat3_dn0 = assign33460_e55986_d_n0;
        locals.var_dvsat3_dn2 = assign33460_e55986_d_n2;
        locals.var_dvsat3_dn3 = assign33460_e55986_d_n3;
        locals.var_dvsat3_dn4 = assign33460_e55986_d_n4;
        locals.var_dvsat3_dn5 = assign33460_e55986_d_n5;
        locals.var_dvsat3_dn6 = assign33460_e55986_d_n6;
        locals.var_dvsat3_dn7 = assign33460_e55986_d_n7;
        locals.var_dvsat3_dn8 = assign33460_e55986_d_n8;
        locals.var_dvsat3_dn9 = assign33460_e55986_d_n9;
        locals.var_dvsat3_dn10 = assign33460_e55986_d_n10;
        locals.var_dvsat3_dn11 = assign33460_e55986_d_n11;
        locals.var_dvsat3_dn13 = assign33460_e55986_d_n13;
        locals.var_dvsat3_dn14 = assign33460_e55986_d_n14;

        let (assign33470_e55997, assign33470_e55997_d_n0, assign33470_e55997_d_n2, assign33470_e55997_d_n3, assign33470_e55997_d_n4, assign33470_e55997_d_n5, assign33470_e55997_d_n6, assign33470_e55997_d_n7, assign33470_e55997_d_n8, assign33470_e55997_d_n9, assign33470_e55997_d_n10, assign33470_e55997_d_n11, assign33470_e55997_d_n13, assign33470_e55997_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33470_e55994: f64 = (locals.var_q0 + locals.var_qia);
        let assign33470_e55995: f64 = (locals.var_q0 / assign33470_e55994);
        (assign33470_e55995, (((locals.var_q0_dn0 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qia_dn0))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn2 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qia_dn2))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn3 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qia_dn3))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn4 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qia_dn4))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn5 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qia_dn5))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn6 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qia_dn6))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn7 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qia_dn7))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn8 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qia_dn8))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn9 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qia_dn9))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn10 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qia_dn10))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn11 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qia_dn11))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn13 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qia_dn13))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn14 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qia_dn14))) / (assign33470_e55994 * assign33470_e55994)),)
    } else {
        (locals.var_noiwi, locals.var_noiwi_dn0, locals.var_noiwi_dn2, locals.var_noiwi_dn3, locals.var_noiwi_dn4, locals.var_noiwi_dn5, locals.var_noiwi_dn6, locals.var_noiwi_dn7, locals.var_noiwi_dn8, locals.var_noiwi_dn9, locals.var_noiwi_dn10, locals.var_noiwi_dn11, locals.var_noiwi_dn13, locals.var_noiwi_dn14,)
    }
};
        locals.var_noiwi = assign33470_e55997;
        locals.var_noiwi_dn0 = assign33470_e55997_d_n0;
        locals.var_noiwi_dn2 = assign33470_e55997_d_n2;
        locals.var_noiwi_dn3 = assign33470_e55997_d_n3;
        locals.var_noiwi_dn4 = assign33470_e55997_d_n4;
        locals.var_noiwi_dn5 = assign33470_e55997_d_n5;
        locals.var_noiwi_dn6 = assign33470_e55997_d_n6;
        locals.var_noiwi_dn7 = assign33470_e55997_d_n7;
        locals.var_noiwi_dn8 = assign33470_e55997_d_n8;
        locals.var_noiwi_dn9 = assign33470_e55997_d_n9;
        locals.var_noiwi_dn10 = assign33470_e55997_d_n10;
        locals.var_noiwi_dn11 = assign33470_e55997_d_n11;
        locals.var_noiwi_dn13 = assign33470_e55997_d_n13;
        locals.var_noiwi_dn14 = assign33470_e55997_d_n14;

        let (assign33480_e56014, assign33480_e56014_d_n0, assign33480_e56014_d_n2, assign33480_e56014_d_n3, assign33480_e56014_d_n4, assign33480_e56014_d_n5, assign33480_e56014_d_n6, assign33480_e56014_d_n7, assign33480_e56014_d_n8, assign33480_e56014_d_n9, assign33480_e56014_d_n10, assign33480_e56014_d_n11, assign33480_e56014_d_n13, assign33480_e56014_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33480_e56005: f64 = (0.0_f64).max(locals.var_k0si_t);
        let assign33480_e56007: f64 = (assign33480_e56005 * locals.var_qis);
        let assign33480_e56010: f64 = (2.0 * locals.var_nvtm);
        let assign33480_e56011: f64 = (assign33480_e56007 + assign33480_e56010);
        let assign33480_e56012: f64 = (locals.var_k0_t / assign33480_e56011);
        (assign33480_e56012, (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3))) / (assign33480_e56011 * assign33480_e56011))), (((locals.var_k0_t_dn4 * assign33480_e56011) - (locals.var_k0_t * (((if 0.0 >= locals.var_k0si_t { 0.0 } else { locals.var_k0si_t_dn4 } * locals.var_qis) + (assign33480_e56005 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33480_e56011 * assign33480_e56011)), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14))) / (assign33480_e56011 * assign33480_e56011))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33480_e56014;
        locals.var_t4_dn0 = assign33480_e56014_d_n0;
        locals.var_t4_dn2 = assign33480_e56014_d_n2;
        locals.var_t4_dn3 = assign33480_e56014_d_n3;
        locals.var_t4_dn4 = assign33480_e56014_d_n4;
        locals.var_t4_dn5 = assign33480_e56014_d_n5;
        locals.var_t4_dn6 = assign33480_e56014_d_n6;
        locals.var_t4_dn7 = assign33480_e56014_d_n7;
        locals.var_t4_dn8 = assign33480_e56014_d_n8;
        locals.var_t4_dn9 = assign33480_e56014_d_n9;
        locals.var_t4_dn10 = assign33480_e56014_d_n10;
        locals.var_t4_dn11 = assign33480_e56014_d_n11;
        locals.var_t4_dn13 = assign33480_e56014_d_n13;
        locals.var_t4_dn14 = assign33480_e56014_d_n14;

        let (assign33490_e56023, assign33490_e56023_d_n0, assign33490_e56023_d_n2, assign33490_e56023_d_n3, assign33490_e56023_d_n4, assign33490_e56023_d_n5, assign33490_e56023_d_n6, assign33490_e56023_d_n7, assign33490_e56023_d_n8, assign33490_e56023_d_n9, assign33490_e56023_d_n10, assign33490_e56023_d_n11, assign33490_e56023_d_n13, assign33490_e56023_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33490_e56020: f64 = (-locals.var_t4);
        let assign33490_e56021: f64 = { let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33490_e56021, ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn0)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn2)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn3)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn4)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn5)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn6)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn7)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn8)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn9)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn10)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn11)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn13)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn14)),)
    } else {
        (locals.var_mnud0, locals.var_mnud0_dn0, locals.var_mnud0_dn2, locals.var_mnud0_dn3, locals.var_mnud0_dn4, locals.var_mnud0_dn5, locals.var_mnud0_dn6, locals.var_mnud0_dn7, locals.var_mnud0_dn8, locals.var_mnud0_dn9, locals.var_mnud0_dn10, locals.var_mnud0_dn11, locals.var_mnud0_dn13, locals.var_mnud0_dn14,)
    }
};
        locals.var_mnud0 = assign33490_e56023;
        locals.var_mnud0_dn0 = assign33490_e56023_d_n0;
        locals.var_mnud0_dn2 = assign33490_e56023_d_n2;
        locals.var_mnud0_dn3 = assign33490_e56023_d_n3;
        locals.var_mnud0_dn4 = assign33490_e56023_d_n4;
        locals.var_mnud0_dn5 = assign33490_e56023_d_n5;
        locals.var_mnud0_dn6 = assign33490_e56023_d_n6;
        locals.var_mnud0_dn7 = assign33490_e56023_d_n7;
        locals.var_mnud0_dn8 = assign33490_e56023_d_n8;
        locals.var_mnud0_dn9 = assign33490_e56023_d_n9;
        locals.var_mnud0_dn10 = assign33490_e56023_d_n10;
        locals.var_mnud0_dn11 = assign33490_e56023_d_n11;
        locals.var_mnud0_dn13 = assign33490_e56023_d_n13;
        locals.var_mnud0_dn14 = assign33490_e56023_d_n14;

        let assign33500_e56026: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign33500_e56026;

        let (assign33510_e56070, assign33510_e56070_d_n0, assign33510_e56070_d_n2, assign33510_e56070_d_n3, assign33510_e56070_d_n4, assign33510_e56070_d_n5, assign33510_e56070_d_n6, assign33510_e56070_d_n7, assign33510_e56070_d_n8, assign33510_e56070_d_n9, assign33510_e56070_d_n10, assign33510_e56070_d_n11, assign33510_e56070_d_n13, assign33510_e56070_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33510_e56035: f64 = (-10000.0);
        let assign33510_e56037: f64 = (assign33510_e56035 * 1e-6);
        let (assign33510_e56068, assign33510_e56068_d_n4,) = {
            if (!(locals.var_k2_t < assign33510_e56037)) {
                let assign33510_e56044: f64 = (locals.var_k2_t * locals.var_k2_t);
                let assign33510_e56047: f64 = (4.0 * 1e-6);
                let assign33510_e56049: f64 = (assign33510_e56047 * 1e-6);
                let assign33510_e56050: f64 = (assign33510_e56044 + assign33510_e56049);
                let assign33510_e56051: f64 = (assign33510_e56050).sqrt();
                let assign33510_e56052: f64 = (locals.var_k2_t + assign33510_e56051);
                let assign33510_e56053: f64 = (0.5 * assign33510_e56052);
                (assign33510_e56053, (0.5 * (locals.var_k2_t_dn4 + (((locals.var_k2_t_dn4 * locals.var_k2_t) + (locals.var_k2_t * locals.var_k2_t_dn4)) / (2.0 * assign33510_e56051)))),)
            } else {
                let assign33510_e56056: f64 = (-10000.0);
                let assign33510_e56058: f64 = (assign33510_e56056 * 1e-6);
                let (assign33510_e56067, assign33510_e56067_d_n4,) = {
                    if (locals.var_k2_t < assign33510_e56058) {
                        let assign33510_e56061: f64 = (-1e-6);
                        let assign33510_e56063: f64 = (assign33510_e56061 * 1e-6);
                        let assign33510_e56065: f64 = (assign33510_e56063 / locals.var_k2_t);
                        (assign33510_e56065, (-((assign33510_e56063 * locals.var_k2_t_dn4) / (locals.var_k2_t * locals.var_k2_t))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign33510_e56067, assign33510_e56067_d_n4,)
            }
        };
        (assign33510_e56068, 0.0, 0.0, 0.0, assign33510_e56068_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33510_e56070;
        locals.var_t4_dn0 = assign33510_e56070_d_n0;
        locals.var_t4_dn2 = assign33510_e56070_d_n2;
        locals.var_t4_dn3 = assign33510_e56070_d_n3;
        locals.var_t4_dn4 = assign33510_e56070_d_n4;
        locals.var_t4_dn5 = assign33510_e56070_d_n5;
        locals.var_t4_dn6 = assign33510_e56070_d_n6;
        locals.var_t4_dn7 = assign33510_e56070_d_n7;
        locals.var_t4_dn8 = assign33510_e56070_d_n8;
        locals.var_t4_dn9 = assign33510_e56070_d_n9;
        locals.var_t4_dn10 = assign33510_e56070_d_n10;
        locals.var_t4_dn11 = assign33510_e56070_d_n11;
        locals.var_t4_dn13 = assign33510_e56070_d_n13;
        locals.var_t4_dn14 = assign33510_e56070_d_n14;

        let (assign33520_e56089, assign33520_e56089_d_n0, assign33520_e56089_d_n2, assign33520_e56089_d_n3, assign33520_e56089_d_n4, assign33520_e56089_d_n5, assign33520_e56089_d_n6, assign33520_e56089_d_n7, assign33520_e56089_d_n8, assign33520_e56089_d_n9, assign33520_e56089_d_n10, assign33520_e56089_d_n11, assign33520_e56089_d_n13, assign33520_e56089_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33520_e56080: f64 = (0.0_f64).max(locals.var_k2si_t);
        let assign33520_e56082: f64 = (assign33520_e56080 * locals.var_qis);
        let assign33520_e56085: f64 = (2.0 * locals.var_nvtm);
        let assign33520_e56086: f64 = (assign33520_e56082 + assign33520_e56085);
        let assign33520_e56087: f64 = (locals.var_t4 / assign33520_e56086);
        (assign33520_e56087, (((locals.var_t4_dn0 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn2 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn3 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn4 * assign33520_e56086) - (locals.var_t4 * (((if 0.0 >= locals.var_k2si_t { 0.0 } else { locals.var_k2si_t_dn4 } * locals.var_qis) + (assign33520_e56080 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn5 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn6 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn7 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn8 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn9 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn10 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn11 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn13 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn14 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14)))) / (assign33520_e56086 * assign33520_e56086)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33520_e56089;
        locals.var_t5_dn0 = assign33520_e56089_d_n0;
        locals.var_t5_dn2 = assign33520_e56089_d_n2;
        locals.var_t5_dn3 = assign33520_e56089_d_n3;
        locals.var_t5_dn4 = assign33520_e56089_d_n4;
        locals.var_t5_dn5 = assign33520_e56089_d_n5;
        locals.var_t5_dn6 = assign33520_e56089_d_n6;
        locals.var_t5_dn7 = assign33520_e56089_d_n7;
        locals.var_t5_dn8 = assign33520_e56089_d_n8;
        locals.var_t5_dn9 = assign33520_e56089_d_n9;
        locals.var_t5_dn10 = assign33520_e56089_d_n10;
        locals.var_t5_dn11 = assign33520_e56089_d_n11;
        locals.var_t5_dn13 = assign33520_e56089_d_n13;
        locals.var_t5_dn14 = assign33520_e56089_d_n14;

        let (assign33530_e56104, assign33530_e56104_d_n0, assign33530_e56104_d_n2, assign33530_e56104_d_n3, assign33530_e56104_d_n4, assign33530_e56104_d_n5, assign33530_e56104_d_n6, assign33530_e56104_d_n7, assign33530_e56104_d_n8, assign33530_e56104_d_n9, assign33530_e56104_d_n10, assign33530_e56104_d_n11, assign33530_e56104_d_n13, assign33530_e56104_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33530_e56098: f64 = (locals.var_phibe_i - locals.var_veseff);
        let assign33530_e56099: f64 = (assign33530_e56098).sqrt();
        let assign33530_e56101: f64 = (locals.var_phibe_i).sqrt();
        let assign33530_e56102: f64 = (assign33530_e56099 - assign33530_e56101);
        (assign33530_e56102, ((-locals.var_veseff_dn0) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn2) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn3) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn4) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn5) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn6) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn7) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn8) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn9) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn10) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn11) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn13) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn14) / (2.0 * assign33530_e56099)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33530_e56104;
        locals.var_t6_dn0 = assign33530_e56104_d_n0;
        locals.var_t6_dn2 = assign33530_e56104_d_n2;
        locals.var_t6_dn3 = assign33530_e56104_d_n3;
        locals.var_t6_dn4 = assign33530_e56104_d_n4;
        locals.var_t6_dn5 = assign33530_e56104_d_n5;
        locals.var_t6_dn6 = assign33530_e56104_d_n6;
        locals.var_t6_dn7 = assign33530_e56104_d_n7;
        locals.var_t6_dn8 = assign33530_e56104_d_n8;
        locals.var_t6_dn9 = assign33530_e56104_d_n9;
        locals.var_t6_dn10 = assign33530_e56104_d_n10;
        locals.var_t6_dn11 = assign33530_e56104_d_n11;
        locals.var_t6_dn13 = assign33530_e56104_d_n13;
        locals.var_t6_dn14 = assign33530_e56104_d_n14;

        let (assign33540_e56117, assign33540_e56117_d_n0, assign33540_e56117_d_n2, assign33540_e56117_d_n3, assign33540_e56117_d_n4, assign33540_e56117_d_n5, assign33540_e56117_d_n6, assign33540_e56117_d_n7, assign33540_e56117_d_n8, assign33540_e56117_d_n9, assign33540_e56117_d_n10, assign33540_e56117_d_n11, assign33540_e56117_d_n13, assign33540_e56117_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33540_e56112: f64 = (-locals.var_t5);
        let assign33540_e56114: f64 = (assign33540_e56112 * locals.var_t6);
        let assign33540_e56115: f64 = { let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33540_e56115, ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn0) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn0))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn2) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn2))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn3) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn3))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn4) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn4))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn5) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn5))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn6) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn6))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn7) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn7))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn8) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn8))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn9) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn9))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn10) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn10))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn11) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn11))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn13) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn13))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn14) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn14))),)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33540_e56117;
        locals.var_mob0_dn0 = assign33540_e56117_d_n0;
        locals.var_mob0_dn2 = assign33540_e56117_d_n2;
        locals.var_mob0_dn3 = assign33540_e56117_d_n3;
        locals.var_mob0_dn4 = assign33540_e56117_d_n4;
        locals.var_mob0_dn5 = assign33540_e56117_d_n5;
        locals.var_mob0_dn6 = assign33540_e56117_d_n6;
        locals.var_mob0_dn7 = assign33540_e56117_d_n7;
        locals.var_mob0_dn8 = assign33540_e56117_d_n8;
        locals.var_mob0_dn9 = assign33540_e56117_d_n9;
        locals.var_mob0_dn10 = assign33540_e56117_d_n10;
        locals.var_mob0_dn11 = assign33540_e56117_d_n11;
        locals.var_mob0_dn13 = assign33540_e56117_d_n13;
        locals.var_mob0_dn14 = assign33540_e56117_d_n14;

        let (assign33550_e56127, assign33550_e56127_d_n0, assign33550_e56127_d_n2, assign33550_e56127_d_n3, assign33550_e56127_d_n4, assign33550_e56127_d_n5, assign33550_e56127_d_n6, assign33550_e56127_d_n7, assign33550_e56127_d_n8, assign33550_e56127_d_n9, assign33550_e56127_d_n10, assign33550_e56127_d_n11, assign33550_e56127_d_n13, assign33550_e56127_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33550_e56127;
        locals.var_mob0_dn0 = assign33550_e56127_d_n0;
        locals.var_mob0_dn2 = assign33550_e56127_d_n2;
        locals.var_mob0_dn3 = assign33550_e56127_d_n3;
        locals.var_mob0_dn4 = assign33550_e56127_d_n4;
        locals.var_mob0_dn5 = assign33550_e56127_d_n5;
        locals.var_mob0_dn6 = assign33550_e56127_d_n6;
        locals.var_mob0_dn7 = assign33550_e56127_d_n7;
        locals.var_mob0_dn8 = assign33550_e56127_d_n8;
        locals.var_mob0_dn9 = assign33550_e56127_d_n9;
        locals.var_mob0_dn10 = assign33550_e56127_d_n10;
        locals.var_mob0_dn11 = assign33550_e56127_d_n11;
        locals.var_mob0_dn13 = assign33550_e56127_d_n13;
        locals.var_mob0_dn14 = assign33550_e56127_d_n14;

        let (assign33560_e56140, assign33560_e56140_d_n0, assign33560_e56140_d_n2, assign33560_e56140_d_n3, assign33560_e56140_d_n4, assign33560_e56140_d_n5, assign33560_e56140_d_n6, assign33560_e56140_d_n7, assign33560_e56140_d_n8, assign33560_e56140_d_n9, assign33560_e56140_d_n10, assign33560_e56140_d_n11, assign33560_e56140_d_n13, assign33560_e56140_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33560_e56136: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign33560_e56137: f64 = (locals.var_qba + assign33560_e56136);
        let assign33560_e56138: f64 = (locals.var_eefffactor * assign33560_e56137);
        (assign33560_e56138, (locals.var_eefffactor * (locals.var_qba_dn0 + (locals.var_eta_mu * locals.var_qis_dn0))), (locals.var_eefffactor * (locals.var_qba_dn2 + (locals.var_eta_mu * locals.var_qis_dn2))), (locals.var_eefffactor * (locals.var_qba_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qba_dn4 + ((locals.var_eta_mu_dn4 * locals.var_qis) + (locals.var_eta_mu * locals.var_qis_dn4)))), (locals.var_eefffactor * (locals.var_qba_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qba_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qba_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qba_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qba_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qba_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qba_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))), (locals.var_eefffactor * (locals.var_qba_dn13 + (locals.var_eta_mu * locals.var_qis_dn13))), (locals.var_eefffactor * (locals.var_qba_dn14 + (locals.var_eta_mu * locals.var_qis_dn14))),)
    } else {
        (locals.var_eeffm0, locals.var_eeffm0_dn0, locals.var_eeffm0_dn2, locals.var_eeffm0_dn3, locals.var_eeffm0_dn4, locals.var_eeffm0_dn5, locals.var_eeffm0_dn6, locals.var_eeffm0_dn7, locals.var_eeffm0_dn8, locals.var_eeffm0_dn9, locals.var_eeffm0_dn10, locals.var_eeffm0_dn11, locals.var_eeffm0_dn13, locals.var_eeffm0_dn14,)
    }
};
        locals.var_eeffm0 = assign33560_e56140;
        locals.var_eeffm0_dn0 = assign33560_e56140_d_n0;
        locals.var_eeffm0_dn2 = assign33560_e56140_d_n2;
        locals.var_eeffm0_dn3 = assign33560_e56140_d_n3;
        locals.var_eeffm0_dn4 = assign33560_e56140_d_n4;
        locals.var_eeffm0_dn5 = assign33560_e56140_d_n5;
        locals.var_eeffm0_dn6 = assign33560_e56140_d_n6;
        locals.var_eeffm0_dn7 = assign33560_e56140_d_n7;
        locals.var_eeffm0_dn8 = assign33560_e56140_d_n8;
        locals.var_eeffm0_dn9 = assign33560_e56140_d_n9;
        locals.var_eeffm0_dn10 = assign33560_e56140_d_n10;
        locals.var_eeffm0_dn11 = assign33560_e56140_d_n11;
        locals.var_eeffm0_dn13 = assign33560_e56140_d_n13;
        locals.var_eeffm0_dn14 = assign33560_e56140_d_n14;

        let (assign33570_e56156, assign33570_e56156_d_n0, assign33570_e56156_d_n2, assign33570_e56156_d_n3, assign33570_e56156_d_n4, assign33570_e56156_d_n5, assign33570_e56156_d_n6, assign33570_e56156_d_n7, assign33570_e56156_d_n8, assign33570_e56156_d_n9, assign33570_e56156_d_n10, assign33570_e56156_d_n11, assign33570_e56156_d_n13, assign33570_e56156_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33570_e56149: f64 = (locals.var_qis / locals.var_qb0);
        let assign33570_e56150: f64 = (assign33570_e56149).abs();
        let assign33570_e56151: f64 = (1.0 + assign33570_e56150);
        let assign33570_e56152: f64 = (0.5 * assign33570_e56151);
        let assign33570_e56154: f64 = (assign33570_e56152).powf(locals.var_ucs_t);
        (assign33570_e56154, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }) / assign33570_e56152))) }, if locals.var_ucs_t_dn4 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * ((locals.var_ucs_t_dn4 * (assign33570_e56152).ln()) + (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }) / assign33570_e56152)))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }) / assign33570_e56152))) },)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33570_e56156;
        locals.var_t4_dn0 = assign33570_e56156_d_n0;
        locals.var_t4_dn2 = assign33570_e56156_d_n2;
        locals.var_t4_dn3 = assign33570_e56156_d_n3;
        locals.var_t4_dn4 = assign33570_e56156_d_n4;
        locals.var_t4_dn5 = assign33570_e56156_d_n5;
        locals.var_t4_dn6 = assign33570_e56156_d_n6;
        locals.var_t4_dn7 = assign33570_e56156_d_n7;
        locals.var_t4_dn8 = assign33570_e56156_d_n8;
        locals.var_t4_dn9 = assign33570_e56156_d_n9;
        locals.var_t4_dn10 = assign33570_e56156_d_n10;
        locals.var_t4_dn11 = assign33570_e56156_d_n11;
        locals.var_t4_dn13 = assign33570_e56156_d_n13;
        locals.var_t4_dn14 = assign33570_e56156_d_n14;

        let assign33580_e56159: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign33580_e56159;

        let (assign33590_e56181, assign33590_e56181_d_n0, assign33590_e56181_d_n2, assign33590_e56181_d_n3, assign33590_e56181_d_n4, assign33590_e56181_d_n5, assign33590_e56181_d_n6, assign33590_e56181_d_n7, assign33590_e56181_d_n8, assign33590_e56181_d_n9, assign33590_e56181_d_n10, assign33590_e56181_d_n11, assign33590_e56181_d_n13, assign33590_e56181_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign33590_e56169: f64 = (locals.var_uc_a * locals.var_veseff);
        let assign33590_e56170: f64 = (locals.var_ua_a + assign33590_e56169);
        let assign33590_e56172: f64 = (locals.var_eeffm0).abs();
        let assign33590_e56174: f64 = (assign33590_e56172).powf(locals.var_eu_a);
        let assign33590_e56175: f64 = (assign33590_e56170 * assign33590_e56174);
        let assign33590_e56178: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33590_e56179: f64 = (assign33590_e56175 + assign33590_e56178);
        (assign33590_e56179, ((((locals.var_ua_a_dn0 + ((locals.var_uc_a_dn0 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn0))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn0 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn2 + ((locals.var_uc_a_dn2 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn2))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn2 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn3))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn3 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn4))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn4 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn5))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn5 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn6))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn6 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn7))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn7 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn8))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn8 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn9))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn9 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn10))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn10 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn11))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn11 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn13 + ((locals.var_uc_a_dn13 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn13))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn13 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn14 + ((locals.var_uc_a_dn14 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn14))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn14 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33590_e56181;
        locals.var_t5_dn0 = assign33590_e56181_d_n0;
        locals.var_t5_dn2 = assign33590_e56181_d_n2;
        locals.var_t5_dn3 = assign33590_e56181_d_n3;
        locals.var_t5_dn4 = assign33590_e56181_d_n4;
        locals.var_t5_dn5 = assign33590_e56181_d_n5;
        locals.var_t5_dn6 = assign33590_e56181_d_n6;
        locals.var_t5_dn7 = assign33590_e56181_d_n7;
        locals.var_t5_dn8 = assign33590_e56181_d_n8;
        locals.var_t5_dn9 = assign33590_e56181_d_n9;
        locals.var_t5_dn10 = assign33590_e56181_d_n10;
        locals.var_t5_dn11 = assign33590_e56181_d_n11;
        locals.var_t5_dn13 = assign33590_e56181_d_n13;
        locals.var_t5_dn14 = assign33590_e56181_d_n14;

        let (assign33600_e56200, assign33600_e56200_d_n0, assign33600_e56200_d_n2, assign33600_e56200_d_n3, assign33600_e56200_d_n4, assign33600_e56200_d_n5, assign33600_e56200_d_n6, assign33600_e56200_d_n7, assign33600_e56200_d_n8, assign33600_e56200_d_n9, assign33600_e56200_d_n10, assign33600_e56200_d_n11, assign33600_e56200_d_n13, assign33600_e56200_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign33600_e56191: f64 = (locals.var_eeffm0).abs();
        let assign33600_e56193: f64 = (assign33600_e56191).powf(locals.var_eu_a);
        let assign33600_e56194: f64 = (locals.var_ua_a * assign33600_e56193);
        let assign33600_e56197: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33600_e56198: f64 = (assign33600_e56194 + assign33600_e56197);
        (assign33600_e56198, (((locals.var_ua_a_dn0 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn0 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn2 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn2 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn3 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn3 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn4 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn4 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn5 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn5 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn6 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn6 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn7 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn7 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn8 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn8 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn9 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn9 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn10 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn10 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn11 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn11 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn13 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn13 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn14 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn14 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33600_e56200;
        locals.var_t5_dn0 = assign33600_e56200_d_n0;
        locals.var_t5_dn2 = assign33600_e56200_d_n2;
        locals.var_t5_dn3 = assign33600_e56200_d_n3;
        locals.var_t5_dn4 = assign33600_e56200_d_n4;
        locals.var_t5_dn5 = assign33600_e56200_d_n5;
        locals.var_t5_dn6 = assign33600_e56200_d_n6;
        locals.var_t5_dn7 = assign33600_e56200_d_n7;
        locals.var_t5_dn8 = assign33600_e56200_d_n8;
        locals.var_t5_dn9 = assign33600_e56200_d_n9;
        locals.var_t5_dn10 = assign33600_e56200_d_n10;
        locals.var_t5_dn11 = assign33600_e56200_d_n11;
        locals.var_t5_dn13 = assign33600_e56200_d_n13;
        locals.var_t5_dn14 = assign33600_e56200_d_n14;

        let (assign33610_e56209, assign33610_e56209_d_n0, assign33610_e56209_d_n2, assign33610_e56209_d_n3, assign33610_e56209_d_n4, assign33610_e56209_d_n5, assign33610_e56209_d_n6, assign33610_e56209_d_n7, assign33610_e56209_d_n8, assign33610_e56209_d_n9, assign33610_e56209_d_n10, assign33610_e56209_d_n11, assign33610_e56209_d_n13, assign33610_e56209_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33610_e56207: f64 = (1.0 + locals.var_t5);
        (assign33610_e56207, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33610_e56209;
        locals.var_dmob0_dn0 = assign33610_e56209_d_n0;
        locals.var_dmob0_dn2 = assign33610_e56209_d_n2;
        locals.var_dmob0_dn3 = assign33610_e56209_d_n3;
        locals.var_dmob0_dn4 = assign33610_e56209_d_n4;
        locals.var_dmob0_dn5 = assign33610_e56209_d_n5;
        locals.var_dmob0_dn6 = assign33610_e56209_d_n6;
        locals.var_dmob0_dn7 = assign33610_e56209_d_n7;
        locals.var_dmob0_dn8 = assign33610_e56209_d_n8;
        locals.var_dmob0_dn9 = assign33610_e56209_d_n9;
        locals.var_dmob0_dn10 = assign33610_e56209_d_n10;
        locals.var_dmob0_dn11 = assign33610_e56209_d_n11;
        locals.var_dmob0_dn13 = assign33610_e56209_d_n13;
        locals.var_dmob0_dn14 = assign33610_e56209_d_n14;

    }

    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33620_e56235, assign33620_e56235_d_n0, assign33620_e56235_d_n2, assign33620_e56235_d_n3, assign33620_e56235_d_n4, assign33620_e56235_d_n5, assign33620_e56235_d_n6, assign33620_e56235_d_n7, assign33620_e56235_d_n8, assign33620_e56235_d_n9, assign33620_e56235_d_n10, assign33620_e56235_d_n11, assign33620_e56235_d_n13, assign33620_e56235_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33620_e56217: f64 = (locals.var_dmob0 + 1.0);
        let assign33620_e56220: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56223: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56224: f64 = (assign33620_e56220 * assign33620_e56223);
        let assign33620_e56227: f64 = (0.25 * p.p604);
        let assign33620_e56229: f64 = (assign33620_e56227 * p.p604);
        let assign33620_e56230: f64 = (assign33620_e56224 + assign33620_e56229);
        let assign33620_e56231: f64 = (assign33620_e56230).sqrt();
        let assign33620_e56232: f64 = (assign33620_e56217 + assign33620_e56231);
        let assign33620_e56233: f64 = (0.5 * assign33620_e56232);
        (assign33620_e56233, (0.5 * (locals.var_dmob0_dn0 + (((locals.var_dmob0_dn0 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn0)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn2 + (((locals.var_dmob0_dn2 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn2)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn3 + (((locals.var_dmob0_dn3 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn3)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn4 + (((locals.var_dmob0_dn4 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn4)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn5 + (((locals.var_dmob0_dn5 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn5)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn6 + (((locals.var_dmob0_dn6 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn6)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn7 + (((locals.var_dmob0_dn7 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn7)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn8 + (((locals.var_dmob0_dn8 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn8)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn9 + (((locals.var_dmob0_dn9 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn9)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn10 + (((locals.var_dmob0_dn10 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn10)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn11 + (((locals.var_dmob0_dn11 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn11)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn13 + (((locals.var_dmob0_dn13 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn13)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn14 + (((locals.var_dmob0_dn14 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn14)) / (2.0 * assign33620_e56231)))),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33620_e56235;
        locals.var_dmob0_dn0 = assign33620_e56235_d_n0;
        locals.var_dmob0_dn2 = assign33620_e56235_d_n2;
        locals.var_dmob0_dn3 = assign33620_e56235_d_n3;
        locals.var_dmob0_dn4 = assign33620_e56235_d_n4;
        locals.var_dmob0_dn5 = assign33620_e56235_d_n5;
        locals.var_dmob0_dn6 = assign33620_e56235_d_n6;
        locals.var_dmob0_dn7 = assign33620_e56235_d_n7;
        locals.var_dmob0_dn8 = assign33620_e56235_d_n8;
        locals.var_dmob0_dn9 = assign33620_e56235_d_n9;
        locals.var_dmob0_dn10 = assign33620_e56235_d_n10;
        locals.var_dmob0_dn11 = assign33620_e56235_d_n11;
        locals.var_dmob0_dn13 = assign33620_e56235_d_n13;
        locals.var_dmob0_dn14 = assign33620_e56235_d_n14;

        let (assign33630_e56244, assign33630_e56244_d_n0, assign33630_e56244_d_n2, assign33630_e56244_d_n3, assign33630_e56244_d_n4, assign33630_e56244_d_n5, assign33630_e56244_d_n6, assign33630_e56244_d_n7, assign33630_e56244_d_n8, assign33630_e56244_d_n9, assign33630_e56244_d_n10, assign33630_e56244_d_n11, assign33630_e56244_d_n13, assign33630_e56244_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33630_e56242: f64 = (locals.var_dmob0 / p.p24);
        (assign33630_e56242, (locals.var_dmob0_dn0 / p.p24), (locals.var_dmob0_dn2 / p.p24), (locals.var_dmob0_dn3 / p.p24), (locals.var_dmob0_dn4 / p.p24), (locals.var_dmob0_dn5 / p.p24), (locals.var_dmob0_dn6 / p.p24), (locals.var_dmob0_dn7 / p.p24), (locals.var_dmob0_dn8 / p.p24), (locals.var_dmob0_dn9 / p.p24), (locals.var_dmob0_dn10 / p.p24), (locals.var_dmob0_dn11 / p.p24), (locals.var_dmob0_dn13 / p.p24), (locals.var_dmob0_dn14 / p.p24),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33630_e56244;
        locals.var_dmob0_dn0 = assign33630_e56244_d_n0;
        locals.var_dmob0_dn2 = assign33630_e56244_d_n2;
        locals.var_dmob0_dn3 = assign33630_e56244_d_n3;
        locals.var_dmob0_dn4 = assign33630_e56244_d_n4;
        locals.var_dmob0_dn5 = assign33630_e56244_d_n5;
        locals.var_dmob0_dn6 = assign33630_e56244_d_n6;
        locals.var_dmob0_dn7 = assign33630_e56244_d_n7;
        locals.var_dmob0_dn8 = assign33630_e56244_d_n8;
        locals.var_dmob0_dn9 = assign33630_e56244_d_n9;
        locals.var_dmob0_dn10 = assign33630_e56244_d_n10;
        locals.var_dmob0_dn11 = assign33630_e56244_d_n11;
        locals.var_dmob0_dn13 = assign33630_e56244_d_n13;
        locals.var_dmob0_dn14 = assign33630_e56244_d_n14;

        let (assign33640_e56255,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33640_e56252: f64 = (0.25 * p.p453);
        let assign33640_e56253: f64 = (1.0 + assign33640_e56252);
        (assign33640_e56253,)
    } else {
        (locals.var_dvsat0,)
    }
};
        locals.var_dvsat0 = assign33640_e56255;

        let (assign33650_e56266, assign33650_e56266_d_n0, assign33650_e56266_d_n2, assign33650_e56266_d_n3, assign33650_e56266_d_n4, assign33650_e56266_d_n5, assign33650_e56266_d_n6, assign33650_e56266_d_n7, assign33650_e56266_d_n8, assign33650_e56266_d_n9, assign33650_e56266_d_n10, assign33650_e56266_d_n11, assign33650_e56266_d_n13, assign33650_e56266_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33650_e56263: f64 = (locals.var_q0 + locals.var_qis);
        let assign33650_e56264: f64 = (locals.var_q0 / assign33650_e56263);
        (assign33650_e56264, (((locals.var_q0_dn0 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qis_dn0))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn2 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qis_dn2))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn3 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qis_dn3))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn4 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qis_dn4))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn5 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qis_dn5))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn6 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qis_dn6))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn7 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qis_dn7))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn8 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qis_dn8))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn9 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qis_dn9))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn10 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qis_dn10))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn11 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qis_dn11))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn13 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qis_dn13))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn14 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qis_dn14))) / (assign33650_e56263 * assign33650_e56263)),)
    } else {
        (locals.var_etaiv0, locals.var_etaiv0_dn0, locals.var_etaiv0_dn2, locals.var_etaiv0_dn3, locals.var_etaiv0_dn4, locals.var_etaiv0_dn5, locals.var_etaiv0_dn6, locals.var_etaiv0_dn7, locals.var_etaiv0_dn8, locals.var_etaiv0_dn9, locals.var_etaiv0_dn10, locals.var_etaiv0_dn11, locals.var_etaiv0_dn13, locals.var_etaiv0_dn14,)
    }
};
        locals.var_etaiv0 = assign33650_e56266;
        locals.var_etaiv0_dn0 = assign33650_e56266_d_n0;
        locals.var_etaiv0_dn2 = assign33650_e56266_d_n2;
        locals.var_etaiv0_dn3 = assign33650_e56266_d_n3;
        locals.var_etaiv0_dn4 = assign33650_e56266_d_n4;
        locals.var_etaiv0_dn5 = assign33650_e56266_d_n5;
        locals.var_etaiv0_dn6 = assign33650_e56266_d_n6;
        locals.var_etaiv0_dn7 = assign33650_e56266_d_n7;
        locals.var_etaiv0_dn8 = assign33650_e56266_d_n8;
        locals.var_etaiv0_dn9 = assign33650_e56266_d_n9;
        locals.var_etaiv0_dn10 = assign33650_e56266_d_n10;
        locals.var_etaiv0_dn11 = assign33650_e56266_d_n11;
        locals.var_etaiv0_dn13 = assign33650_e56266_d_n13;
        locals.var_etaiv0_dn14 = assign33650_e56266_d_n14;

        let (assign33660_e56277, assign33660_e56277_d_n0, assign33660_e56277_d_n2, assign33660_e56277_d_n3, assign33660_e56277_d_n4, assign33660_e56277_d_n5, assign33660_e56277_d_n6, assign33660_e56277_d_n7, assign33660_e56277_d_n8, assign33660_e56277_d_n9, assign33660_e56277_d_n10, assign33660_e56277_d_n11, assign33660_e56277_d_n13, assign33660_e56277_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33660_e56273: f64 = (2.0 - locals.var_etaiv0);
        let assign33660_e56275: f64 = (assign33660_e56273 * locals.var_nvtm);
        (assign33660_e56275, (((-locals.var_etaiv0_dn0) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn0)), (((-locals.var_etaiv0_dn2) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn2)), (((-locals.var_etaiv0_dn3) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn3)), (((-locals.var_etaiv0_dn4) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn4)), (((-locals.var_etaiv0_dn5) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn5)), (((-locals.var_etaiv0_dn6) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn6)), (((-locals.var_etaiv0_dn7) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn7)), (((-locals.var_etaiv0_dn8) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn8)), (((-locals.var_etaiv0_dn9) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn9)), (((-locals.var_etaiv0_dn10) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn10)), (((-locals.var_etaiv0_dn11) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn11)), (((-locals.var_etaiv0_dn13) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn13)), (((-locals.var_etaiv0_dn14) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33660_e56277;
        locals.var_t4_dn0 = assign33660_e56277_d_n0;
        locals.var_t4_dn2 = assign33660_e56277_d_n2;
        locals.var_t4_dn3 = assign33660_e56277_d_n3;
        locals.var_t4_dn4 = assign33660_e56277_d_n4;
        locals.var_t4_dn5 = assign33660_e56277_d_n5;
        locals.var_t4_dn6 = assign33660_e56277_d_n6;
        locals.var_t4_dn7 = assign33660_e56277_d_n7;
        locals.var_t4_dn8 = assign33660_e56277_d_n8;
        locals.var_t4_dn9 = assign33660_e56277_d_n9;
        locals.var_t4_dn10 = assign33660_e56277_d_n10;
        locals.var_t4_dn11 = assign33660_e56277_d_n11;
        locals.var_t4_dn13 = assign33660_e56277_d_n13;
        locals.var_t4_dn14 = assign33660_e56277_d_n14;

        let (assign33670_e56286, assign33670_e56286_d_n0, assign33670_e56286_d_n2, assign33670_e56286_d_n3, assign33670_e56286_d_n4, assign33670_e56286_d_n5, assign33670_e56286_d_n6, assign33670_e56286_d_n7, assign33670_e56286_d_n8, assign33670_e56286_d_n9, assign33670_e56286_d_n10, assign33670_e56286_d_n11, assign33670_e56286_d_n13, assign33670_e56286_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33670_e56284: f64 = (locals.var_qis + locals.var_t4);
        (assign33670_e56284, (locals.var_qis_dn0 + locals.var_t4_dn0), (locals.var_qis_dn2 + locals.var_t4_dn2), (locals.var_qis_dn3 + locals.var_t4_dn3), (locals.var_qis_dn4 + locals.var_t4_dn4), (locals.var_qis_dn5 + locals.var_t4_dn5), (locals.var_qis_dn6 + locals.var_t4_dn6), (locals.var_qis_dn7 + locals.var_t4_dn7), (locals.var_qis_dn8 + locals.var_t4_dn8), (locals.var_qis_dn9 + locals.var_t4_dn9), (locals.var_qis_dn10 + locals.var_t4_dn10), (locals.var_qis_dn11 + locals.var_t4_dn11), (locals.var_qis_dn13 + locals.var_t4_dn13), (locals.var_qis_dn14 + locals.var_t4_dn14),)
    } else {
        (locals.var_ids0_ov_dqi0, locals.var_ids0_ov_dqi0_dn0, locals.var_ids0_ov_dqi0_dn2, locals.var_ids0_ov_dqi0_dn3, locals.var_ids0_ov_dqi0_dn4, locals.var_ids0_ov_dqi0_dn5, locals.var_ids0_ov_dqi0_dn6, locals.var_ids0_ov_dqi0_dn7, locals.var_ids0_ov_dqi0_dn8, locals.var_ids0_ov_dqi0_dn9, locals.var_ids0_ov_dqi0_dn10, locals.var_ids0_ov_dqi0_dn11, locals.var_ids0_ov_dqi0_dn13, locals.var_ids0_ov_dqi0_dn14,)
    }
};
        locals.var_ids0_ov_dqi0 = assign33670_e56286;
        locals.var_ids0_ov_dqi0_dn0 = assign33670_e56286_d_n0;
        locals.var_ids0_ov_dqi0_dn2 = assign33670_e56286_d_n2;
        locals.var_ids0_ov_dqi0_dn3 = assign33670_e56286_d_n3;
        locals.var_ids0_ov_dqi0_dn4 = assign33670_e56286_d_n4;
        locals.var_ids0_ov_dqi0_dn5 = assign33670_e56286_d_n5;
        locals.var_ids0_ov_dqi0_dn6 = assign33670_e56286_d_n6;
        locals.var_ids0_ov_dqi0_dn7 = assign33670_e56286_d_n7;
        locals.var_ids0_ov_dqi0_dn8 = assign33670_e56286_d_n8;
        locals.var_ids0_ov_dqi0_dn9 = assign33670_e56286_d_n9;
        locals.var_ids0_ov_dqi0_dn10 = assign33670_e56286_d_n10;
        locals.var_ids0_ov_dqi0_dn11 = assign33670_e56286_d_n11;
        locals.var_ids0_ov_dqi0_dn13 = assign33670_e56286_d_n13;
        locals.var_ids0_ov_dqi0_dn14 = assign33670_e56286_d_n14;

        let assign33680_e56289: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign33680_e56289;

        let assign33690_e56292: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign33690_e56292;

        let assign33700_e56295: f64 = if p.p64 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign33700_e56295;

        let (assign33710_e56308, assign33710_e56308_d_n0, assign33710_e56308_d_n2, assign33710_e56308_d_n3, assign33710_e56308_d_n4, assign33710_e56308_d_n5, assign33710_e56308_d_n6, assign33710_e56308_d_n7, assign33710_e56308_d_n8, assign33710_e56308_d_n9, assign33710_e56308_d_n10, assign33710_e56308_d_n11, assign33710_e56308_d_n13, assign33710_e56308_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33710_e56305: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33710_e56306: f64 = (1.0 + assign33710_e56305);
        (assign33710_e56306, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33710_e56308;
        locals.var_t4_dn0 = assign33710_e56308_d_n0;
        locals.var_t4_dn2 = assign33710_e56308_d_n2;
        locals.var_t4_dn3 = assign33710_e56308_d_n3;
        locals.var_t4_dn4 = assign33710_e56308_d_n4;
        locals.var_t4_dn5 = assign33710_e56308_d_n5;
        locals.var_t4_dn6 = assign33710_e56308_d_n6;
        locals.var_t4_dn7 = assign33710_e56308_d_n7;
        locals.var_t4_dn8 = assign33710_e56308_d_n8;
        locals.var_t4_dn9 = assign33710_e56308_d_n9;
        locals.var_t4_dn10 = assign33710_e56308_d_n10;
        locals.var_t4_dn11 = assign33710_e56308_d_n11;
        locals.var_t4_dn13 = assign33710_e56308_d_n13;
        locals.var_t4_dn14 = assign33710_e56308_d_n14;

        let (assign33720_e56319, assign33720_e56319_d_n0, assign33720_e56319_d_n2, assign33720_e56319_d_n3, assign33720_e56319_d_n4, assign33720_e56319_d_n5, assign33720_e56319_d_n6, assign33720_e56319_d_n7, assign33720_e56319_d_n8, assign33720_e56319_d_n9, assign33720_e56319_d_n10, assign33720_e56319_d_n11, assign33720_e56319_d_n13, assign33720_e56319_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33720_e56317: f64 = (1.0 / locals.var_t4);
        (assign33720_e56317, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33720_e56319;
        locals.var_t5_dn0 = assign33720_e56319_d_n0;
        locals.var_t5_dn2 = assign33720_e56319_d_n2;
        locals.var_t5_dn3 = assign33720_e56319_d_n3;
        locals.var_t5_dn4 = assign33720_e56319_d_n4;
        locals.var_t5_dn5 = assign33720_e56319_d_n5;
        locals.var_t5_dn6 = assign33720_e56319_d_n6;
        locals.var_t5_dn7 = assign33720_e56319_d_n7;
        locals.var_t5_dn8 = assign33720_e56319_d_n8;
        locals.var_t5_dn9 = assign33720_e56319_d_n9;
        locals.var_t5_dn10 = assign33720_e56319_d_n10;
        locals.var_t5_dn11 = assign33720_e56319_d_n11;
        locals.var_t5_dn13 = assign33720_e56319_d_n13;
        locals.var_t5_dn14 = assign33720_e56319_d_n14;

        let (assign33730_e56337, assign33730_e56337_d_n0, assign33730_e56337_d_n2, assign33730_e56337_d_n3, assign33730_e56337_d_n4, assign33730_e56337_d_n5, assign33730_e56337_d_n6, assign33730_e56337_d_n7, assign33730_e56337_d_n8, assign33730_e56337_d_n9, assign33730_e56337_d_n10, assign33730_e56337_d_n11, assign33730_e56337_d_n13, assign33730_e56337_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33730_e56330: f64 = (locals.var_t5 * locals.var_t5);
        let assign33730_e56332: f64 = (assign33730_e56330 + 0.01);
        let assign33730_e56333: f64 = (assign33730_e56332).sqrt();
        let assign33730_e56334: f64 = (locals.var_t5 + assign33730_e56333);
        let assign33730_e56335: f64 = (0.5 * assign33730_e56334);
        (assign33730_e56335, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33730_e56333)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33730_e56337;
        locals.var_t6_dn0 = assign33730_e56337_d_n0;
        locals.var_t6_dn2 = assign33730_e56337_d_n2;
        locals.var_t6_dn3 = assign33730_e56337_d_n3;
        locals.var_t6_dn4 = assign33730_e56337_d_n4;
        locals.var_t6_dn5 = assign33730_e56337_d_n5;
        locals.var_t6_dn6 = assign33730_e56337_d_n6;
        locals.var_t6_dn7 = assign33730_e56337_d_n7;
        locals.var_t6_dn8 = assign33730_e56337_d_n8;
        locals.var_t6_dn9 = assign33730_e56337_d_n9;
        locals.var_t6_dn10 = assign33730_e56337_d_n10;
        locals.var_t6_dn11 = assign33730_e56337_d_n11;
        locals.var_t6_dn13 = assign33730_e56337_d_n13;
        locals.var_t6_dn14 = assign33730_e56337_d_n14;

        let (assign33740_e56354, assign33740_e56354_d_n0, assign33740_e56354_d_n2, assign33740_e56354_d_n3, assign33740_e56354_d_n4, assign33740_e56354_d_n5, assign33740_e56354_d_n6, assign33740_e56354_d_n7, assign33740_e56354_d_n8, assign33740_e56354_d_n9, assign33740_e56354_d_n10, assign33740_e56354_d_n11, assign33740_e56354_d_n13, assign33740_e56354_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33740_e56348: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33740_e56349: f64 = (p.p908 + assign33740_e56348);
        let assign33740_e56350: f64 = (locals.var_rdstemp * assign33740_e56349);
        let assign33740_e56352: f64 = (assign33740_e56350 * locals.var_weffwrfactor);
        (assign33740_e56352, (((locals.var_rdstemp_dn0 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn2 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn3 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn4 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn5 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn6 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn7 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn8 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn9 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn10 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn11 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn13 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn14 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)))) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33740_e56354;
        locals.var_rdsi0_dn0 = assign33740_e56354_d_n0;
        locals.var_rdsi0_dn2 = assign33740_e56354_d_n2;
        locals.var_rdsi0_dn3 = assign33740_e56354_d_n3;
        locals.var_rdsi0_dn4 = assign33740_e56354_d_n4;
        locals.var_rdsi0_dn5 = assign33740_e56354_d_n5;
        locals.var_rdsi0_dn6 = assign33740_e56354_d_n6;
        locals.var_rdsi0_dn7 = assign33740_e56354_d_n7;
        locals.var_rdsi0_dn8 = assign33740_e56354_d_n8;
        locals.var_rdsi0_dn9 = assign33740_e56354_d_n9;
        locals.var_rdsi0_dn10 = assign33740_e56354_d_n10;
        locals.var_rdsi0_dn11 = assign33740_e56354_d_n11;
        locals.var_rdsi0_dn13 = assign33740_e56354_d_n13;
        locals.var_rdsi0_dn14 = assign33740_e56354_d_n14;

        let (assign33750_e56375, assign33750_e56375_d_n0, assign33750_e56375_d_n2, assign33750_e56375_d_n3, assign33750_e56375_d_n4, assign33750_e56375_d_n5, assign33750_e56375_d_n6, assign33750_e56375_d_n7, assign33750_e56375_d_n8, assign33750_e56375_d_n9, assign33750_e56375_d_n10, assign33750_e56375_d_n11, assign33750_e56375_d_n13, assign33750_e56375_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33750_e56364: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33750_e56366: f64 = (assign33750_e56364 * locals.var_ids0_ov_dqi0);
        let assign33750_e56369: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33750_e56370: f64 = (assign33750_e56366 / assign33750_e56369);
        let assign33750_e56372: f64 = (assign33750_e56370 * locals.var_rdsi0);
        let assign33750_e56373: f64 = (1.0 + assign33750_e56372);
        (assign33750_e56373, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn0)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn2)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn3)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn4)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn5)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn6)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn7)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn8)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn9)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn10)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn11)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn13)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn14)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33750_e56375;
        locals.var_dr0_dn0 = assign33750_e56375_d_n0;
        locals.var_dr0_dn2 = assign33750_e56375_d_n2;
        locals.var_dr0_dn3 = assign33750_e56375_d_n3;
        locals.var_dr0_dn4 = assign33750_e56375_d_n4;
        locals.var_dr0_dn5 = assign33750_e56375_d_n5;
        locals.var_dr0_dn6 = assign33750_e56375_d_n6;
        locals.var_dr0_dn7 = assign33750_e56375_d_n7;
        locals.var_dr0_dn8 = assign33750_e56375_d_n8;
        locals.var_dr0_dn9 = assign33750_e56375_d_n9;
        locals.var_dr0_dn10 = assign33750_e56375_d_n10;
        locals.var_dr0_dn11 = assign33750_e56375_d_n11;
        locals.var_dr0_dn13 = assign33750_e56375_d_n13;
        locals.var_dr0_dn14 = assign33750_e56375_d_n14;

        let (assign33760_e56387, assign33760_e56387_d_n0, assign33760_e56387_d_n2, assign33760_e56387_d_n3, assign33760_e56387_d_n4, assign33760_e56387_d_n5, assign33760_e56387_d_n6, assign33760_e56387_d_n7, assign33760_e56387_d_n8, assign33760_e56387_d_n9, assign33760_e56387_d_n10, assign33760_e56387_d_n11, assign33760_e56387_d_n13, assign33760_e56387_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard637 != 0.0) && (locals.var_guard636 == 0.0))) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33760_e56387;
        locals.var_dr0_dn0 = assign33760_e56387_d_n0;
        locals.var_dr0_dn2 = assign33760_e56387_d_n2;
        locals.var_dr0_dn3 = assign33760_e56387_d_n3;
        locals.var_dr0_dn4 = assign33760_e56387_d_n4;
        locals.var_dr0_dn5 = assign33760_e56387_d_n5;
        locals.var_dr0_dn6 = assign33760_e56387_d_n6;
        locals.var_dr0_dn7 = assign33760_e56387_d_n7;
        locals.var_dr0_dn8 = assign33760_e56387_d_n8;
        locals.var_dr0_dn9 = assign33760_e56387_d_n9;
        locals.var_dr0_dn10 = assign33760_e56387_d_n10;
        locals.var_dr0_dn11 = assign33760_e56387_d_n11;
        locals.var_dr0_dn13 = assign33760_e56387_d_n13;
        locals.var_dr0_dn14 = assign33760_e56387_d_n14;

        let (assign33770_e56405, assign33770_e56405_d_n0, assign33770_e56405_d_n2, assign33770_e56405_d_n3, assign33770_e56405_d_n4, assign33770_e56405_d_n5, assign33770_e56405_d_n6, assign33770_e56405_d_n7, assign33770_e56405_d_n8, assign33770_e56405_d_n9, assign33770_e56405_d_n10, assign33770_e56405_d_n11, assign33770_e56405_d_n13, assign33770_e56405_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33770_e56402: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33770_e56403: f64 = (1.0 + assign33770_e56402);
        (assign33770_e56403, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33770_e56405;
        locals.var_t4_dn0 = assign33770_e56405_d_n0;
        locals.var_t4_dn2 = assign33770_e56405_d_n2;
        locals.var_t4_dn3 = assign33770_e56405_d_n3;
        locals.var_t4_dn4 = assign33770_e56405_d_n4;
        locals.var_t4_dn5 = assign33770_e56405_d_n5;
        locals.var_t4_dn6 = assign33770_e56405_d_n6;
        locals.var_t4_dn7 = assign33770_e56405_d_n7;
        locals.var_t4_dn8 = assign33770_e56405_d_n8;
        locals.var_t4_dn9 = assign33770_e56405_d_n9;
        locals.var_t4_dn10 = assign33770_e56405_d_n10;
        locals.var_t4_dn11 = assign33770_e56405_d_n11;
        locals.var_t4_dn13 = assign33770_e56405_d_n13;
        locals.var_t4_dn14 = assign33770_e56405_d_n14;

        let (assign33780_e56421, assign33780_e56421_d_n0, assign33780_e56421_d_n2, assign33780_e56421_d_n3, assign33780_e56421_d_n4, assign33780_e56421_d_n5, assign33780_e56421_d_n6, assign33780_e56421_d_n7, assign33780_e56421_d_n8, assign33780_e56421_d_n9, assign33780_e56421_d_n10, assign33780_e56421_d_n11, assign33780_e56421_d_n13, assign33780_e56421_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33780_e56419: f64 = (1.0 / locals.var_t4);
        (assign33780_e56419, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33780_e56421;
        locals.var_t5_dn0 = assign33780_e56421_d_n0;
        locals.var_t5_dn2 = assign33780_e56421_d_n2;
        locals.var_t5_dn3 = assign33780_e56421_d_n3;
        locals.var_t5_dn4 = assign33780_e56421_d_n4;
        locals.var_t5_dn5 = assign33780_e56421_d_n5;
        locals.var_t5_dn6 = assign33780_e56421_d_n6;
        locals.var_t5_dn7 = assign33780_e56421_d_n7;
        locals.var_t5_dn8 = assign33780_e56421_d_n8;
        locals.var_t5_dn9 = assign33780_e56421_d_n9;
        locals.var_t5_dn10 = assign33780_e56421_d_n10;
        locals.var_t5_dn11 = assign33780_e56421_d_n11;
        locals.var_t5_dn13 = assign33780_e56421_d_n13;
        locals.var_t5_dn14 = assign33780_e56421_d_n14;

        let (assign33790_e56444, assign33790_e56444_d_n0, assign33790_e56444_d_n2, assign33790_e56444_d_n3, assign33790_e56444_d_n4, assign33790_e56444_d_n5, assign33790_e56444_d_n6, assign33790_e56444_d_n7, assign33790_e56444_d_n8, assign33790_e56444_d_n9, assign33790_e56444_d_n10, assign33790_e56444_d_n11, assign33790_e56444_d_n13, assign33790_e56444_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33790_e56437: f64 = (locals.var_t5 * locals.var_t5);
        let assign33790_e56439: f64 = (assign33790_e56437 + 0.01);
        let assign33790_e56440: f64 = (assign33790_e56439).sqrt();
        let assign33790_e56441: f64 = (locals.var_t5 + assign33790_e56440);
        let assign33790_e56442: f64 = (0.5 * assign33790_e56441);
        (assign33790_e56442, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33790_e56440)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33790_e56444;
        locals.var_t6_dn0 = assign33790_e56444_d_n0;
        locals.var_t6_dn2 = assign33790_e56444_d_n2;
        locals.var_t6_dn3 = assign33790_e56444_d_n3;
        locals.var_t6_dn4 = assign33790_e56444_d_n4;
        locals.var_t6_dn5 = assign33790_e56444_d_n5;
        locals.var_t6_dn6 = assign33790_e56444_d_n6;
        locals.var_t6_dn7 = assign33790_e56444_d_n7;
        locals.var_t6_dn8 = assign33790_e56444_d_n8;
        locals.var_t6_dn9 = assign33790_e56444_d_n9;
        locals.var_t6_dn10 = assign33790_e56444_d_n10;
        locals.var_t6_dn11 = assign33790_e56444_d_n11;
        locals.var_t6_dn13 = assign33790_e56444_d_n13;
        locals.var_t6_dn14 = assign33790_e56444_d_n14;

        let (assign33800_e56464, assign33800_e56464_d_n0, assign33800_e56464_d_n2, assign33800_e56464_d_n3, assign33800_e56464_d_n4, assign33800_e56464_d_n5, assign33800_e56464_d_n6, assign33800_e56464_d_n7, assign33800_e56464_d_n8, assign33800_e56464_d_n9, assign33800_e56464_d_n10, assign33800_e56464_d_n11, assign33800_e56464_d_n13, assign33800_e56464_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33800_e56459: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33800_e56460: f64 = (p.p908 + assign33800_e56459);
        let assign33800_e56462: f64 = (assign33800_e56460 * locals.var_weffwrfactor);
        (assign33800_e56462, (((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33800_e56464;
        locals.var_rdsi0_dn0 = assign33800_e56464_d_n0;
        locals.var_rdsi0_dn2 = assign33800_e56464_d_n2;
        locals.var_rdsi0_dn3 = assign33800_e56464_d_n3;
        locals.var_rdsi0_dn4 = assign33800_e56464_d_n4;
        locals.var_rdsi0_dn5 = assign33800_e56464_d_n5;
        locals.var_rdsi0_dn6 = assign33800_e56464_d_n6;
        locals.var_rdsi0_dn7 = assign33800_e56464_d_n7;
        locals.var_rdsi0_dn8 = assign33800_e56464_d_n8;
        locals.var_rdsi0_dn9 = assign33800_e56464_d_n9;
        locals.var_rdsi0_dn10 = assign33800_e56464_d_n10;
        locals.var_rdsi0_dn11 = assign33800_e56464_d_n11;
        locals.var_rdsi0_dn13 = assign33800_e56464_d_n13;
        locals.var_rdsi0_dn14 = assign33800_e56464_d_n14;

        let (assign33810_e56484, assign33810_e56484_d_n0, assign33810_e56484_d_n2, assign33810_e56484_d_n3, assign33810_e56484_d_n4, assign33810_e56484_d_n5, assign33810_e56484_d_n6, assign33810_e56484_d_n7, assign33810_e56484_d_n8, assign33810_e56484_d_n9, assign33810_e56484_d_n10, assign33810_e56484_d_n11, assign33810_e56484_d_n13, assign33810_e56484_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33810_e56479: f64 = (locals.var_rsourcegeo + locals.var_rdraingeo);
        let assign33810_e56481: f64 = (assign33810_e56479 + locals.var_rdsi0);
        let assign33810_e56482: f64 = (locals.var_rdstemp * assign33810_e56481);
        (assign33810_e56482, ((locals.var_rdstemp_dn0 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn0 + locals.var_rdraingeo_dn0) + locals.var_rdsi0_dn0))), ((locals.var_rdstemp_dn2 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn2 + locals.var_rdraingeo_dn2) + locals.var_rdsi0_dn2))), ((locals.var_rdstemp_dn3 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn3 + locals.var_rdraingeo_dn3) + locals.var_rdsi0_dn3))), ((locals.var_rdstemp_dn4 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn4 + locals.var_rdraingeo_dn4) + locals.var_rdsi0_dn4))), ((locals.var_rdstemp_dn5 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn5 + locals.var_rdraingeo_dn5) + locals.var_rdsi0_dn5))), ((locals.var_rdstemp_dn6 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn6 + locals.var_rdraingeo_dn6) + locals.var_rdsi0_dn6))), ((locals.var_rdstemp_dn7 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn7 + locals.var_rdraingeo_dn7) + locals.var_rdsi0_dn7))), ((locals.var_rdstemp_dn8 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn8 + locals.var_rdraingeo_dn8) + locals.var_rdsi0_dn8))), ((locals.var_rdstemp_dn9 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn9 + locals.var_rdraingeo_dn9) + locals.var_rdsi0_dn9))), ((locals.var_rdstemp_dn10 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn10 + locals.var_rdraingeo_dn10) + locals.var_rdsi0_dn10))), ((locals.var_rdstemp_dn11 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn11 + locals.var_rdraingeo_dn11) + locals.var_rdsi0_dn11))), ((locals.var_rdstemp_dn13 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn13 + locals.var_rdraingeo_dn13) + locals.var_rdsi0_dn13))), ((locals.var_rdstemp_dn14 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn14 + locals.var_rdraingeo_dn14) + locals.var_rdsi0_dn14))),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33810_e56484;
        locals.var_rdsi0_dn0 = assign33810_e56484_d_n0;
        locals.var_rdsi0_dn2 = assign33810_e56484_d_n2;
        locals.var_rdsi0_dn3 = assign33810_e56484_d_n3;
        locals.var_rdsi0_dn4 = assign33810_e56484_d_n4;
        locals.var_rdsi0_dn5 = assign33810_e56484_d_n5;
        locals.var_rdsi0_dn6 = assign33810_e56484_d_n6;
        locals.var_rdsi0_dn7 = assign33810_e56484_d_n7;
        locals.var_rdsi0_dn8 = assign33810_e56484_d_n8;
        locals.var_rdsi0_dn9 = assign33810_e56484_d_n9;
        locals.var_rdsi0_dn10 = assign33810_e56484_d_n10;
        locals.var_rdsi0_dn11 = assign33810_e56484_d_n11;
        locals.var_rdsi0_dn13 = assign33810_e56484_d_n13;
        locals.var_rdsi0_dn14 = assign33810_e56484_d_n14;

        let (assign33820_e56510, assign33820_e56510_d_n0, assign33820_e56510_d_n2, assign33820_e56510_d_n3, assign33820_e56510_d_n4, assign33820_e56510_d_n5, assign33820_e56510_d_n6, assign33820_e56510_d_n7, assign33820_e56510_d_n8, assign33820_e56510_d_n9, assign33820_e56510_d_n10, assign33820_e56510_d_n11, assign33820_e56510_d_n13, assign33820_e56510_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33820_e56499: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33820_e56501: f64 = (assign33820_e56499 * locals.var_ids0_ov_dqi0);
        let assign33820_e56504: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33820_e56505: f64 = (assign33820_e56501 / assign33820_e56504);
        let assign33820_e56507: f64 = (assign33820_e56505 * locals.var_rdsi0);
        let assign33820_e56508: f64 = (1.0 + assign33820_e56507);
        (assign33820_e56508, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn0)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn2)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn3)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn4)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn5)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn6)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn7)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn8)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn9)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn10)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn11)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn13)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn14)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33820_e56510;
        locals.var_dr0_dn0 = assign33820_e56510_d_n0;
        locals.var_dr0_dn2 = assign33820_e56510_d_n2;
        locals.var_dr0_dn3 = assign33820_e56510_d_n3;
        locals.var_dr0_dn4 = assign33820_e56510_d_n4;
        locals.var_dr0_dn5 = assign33820_e56510_d_n5;
        locals.var_dr0_dn6 = assign33820_e56510_d_n6;
        locals.var_dr0_dn7 = assign33820_e56510_d_n7;
        locals.var_dr0_dn8 = assign33820_e56510_d_n8;
        locals.var_dr0_dn9 = assign33820_e56510_d_n9;
        locals.var_dr0_dn10 = assign33820_e56510_d_n10;
        locals.var_dr0_dn11 = assign33820_e56510_d_n11;
        locals.var_dr0_dn13 = assign33820_e56510_d_n13;
        locals.var_dr0_dn14 = assign33820_e56510_d_n14;

        let (assign33830_e56531, assign33830_e56531_d_n0, assign33830_e56531_d_n2, assign33830_e56531_d_n3, assign33830_e56531_d_n4, assign33830_e56531_d_n5, assign33830_e56531_d_n6, assign33830_e56531_d_n7, assign33830_e56531_d_n8, assign33830_e56531_d_n9, assign33830_e56531_d_n10, assign33830_e56531_d_n11, assign33830_e56531_d_n13, assign33830_e56531_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33830_e56517: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33830_e56519: f64 = (assign33830_e56517 * locals.var_qis);
        let assign33830_e56521: f64 = (assign33830_e56519 * locals.var_mnud0);
        let assign33830_e56523: f64 = (assign33830_e56521 * locals.var_mob0);
        let assign33830_e56526: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33830_e56528: f64 = (assign33830_e56526 * locals.var_dr0);
        let assign33830_e56529: f64 = (assign33830_e56523 / assign33830_e56528);
        (assign33830_e56529, ((((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn0)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn0)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn0)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn0 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn0)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn2)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn2)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn2)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn2 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn2)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn3)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn3)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn3)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn3 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn3)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn4)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn4)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn4)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn4 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn4)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn5)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn5)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn5)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn5 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn5)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn6)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn6)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn6)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn6 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn6)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn7)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn7)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn7)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn7 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn7)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn8)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn8)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn8)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn8 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn8)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn9)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn9)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn9)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn9 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn9)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn10)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn10)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn10)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn10 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn10)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn11)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn11)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn11)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn11 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn11)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn13)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn13)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn13)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn13 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn13)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn14)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn14)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn14)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn14 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn14)))) / (assign33830_e56528 * assign33830_e56528)),)
    } else {
        (locals.var_noigd0, locals.var_noigd0_dn0, locals.var_noigd0_dn2, locals.var_noigd0_dn3, locals.var_noigd0_dn4, locals.var_noigd0_dn5, locals.var_noigd0_dn6, locals.var_noigd0_dn7, locals.var_noigd0_dn8, locals.var_noigd0_dn9, locals.var_noigd0_dn10, locals.var_noigd0_dn11, locals.var_noigd0_dn13, locals.var_noigd0_dn14,)
    }
};
        locals.var_noigd0 = assign33830_e56531;
        locals.var_noigd0_dn0 = assign33830_e56531_d_n0;
        locals.var_noigd0_dn2 = assign33830_e56531_d_n2;
        locals.var_noigd0_dn3 = assign33830_e56531_d_n3;
        locals.var_noigd0_dn4 = assign33830_e56531_d_n4;
        locals.var_noigd0_dn5 = assign33830_e56531_d_n5;
        locals.var_noigd0_dn6 = assign33830_e56531_d_n6;
        locals.var_noigd0_dn7 = assign33830_e56531_d_n7;
        locals.var_noigd0_dn8 = assign33830_e56531_d_n8;
        locals.var_noigd0_dn9 = assign33830_e56531_d_n9;
        locals.var_noigd0_dn10 = assign33830_e56531_d_n10;
        locals.var_noigd0_dn11 = assign33830_e56531_d_n11;
        locals.var_noigd0_dn13 = assign33830_e56531_d_n13;
        locals.var_noigd0_dn14 = assign33830_e56531_d_n14;

        let (assign33840_e56540, assign33840_e56540_d_n0, assign33840_e56540_d_n2, assign33840_e56540_d_n3, assign33840_e56540_d_n4, assign33840_e56540_d_n5, assign33840_e56540_d_n6, assign33840_e56540_d_n7, assign33840_e56540_d_n8, assign33840_e56540_d_n9, assign33840_e56540_d_n10, assign33840_e56540_d_n11, assign33840_e56540_d_n13, assign33840_e56540_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33840_e56538: f64 = (1.0 + locals.var_noieta);
        (assign33840_e56538, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33840_e56540;
        locals.var_t4_dn0 = assign33840_e56540_d_n0;
        locals.var_t4_dn2 = assign33840_e56540_d_n2;
        locals.var_t4_dn3 = assign33840_e56540_d_n3;
        locals.var_t4_dn4 = assign33840_e56540_d_n4;
        locals.var_t4_dn5 = assign33840_e56540_d_n5;
        locals.var_t4_dn6 = assign33840_e56540_d_n6;
        locals.var_t4_dn7 = assign33840_e56540_d_n7;
        locals.var_t4_dn8 = assign33840_e56540_d_n8;
        locals.var_t4_dn9 = assign33840_e56540_d_n9;
        locals.var_t4_dn10 = assign33840_e56540_d_n10;
        locals.var_t4_dn11 = assign33840_e56540_d_n11;
        locals.var_t4_dn13 = assign33840_e56540_d_n13;
        locals.var_t4_dn14 = assign33840_e56540_d_n14;

    }

    pub(super) fn stamp_transient_block_126(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33850_e56549, assign33850_e56549_d_n0, assign33850_e56549_d_n2, assign33850_e56549_d_n3, assign33850_e56549_d_n4, assign33850_e56549_d_n5, assign33850_e56549_d_n6, assign33850_e56549_d_n7, assign33850_e56549_d_n8, assign33850_e56549_d_n9, assign33850_e56549_d_n10, assign33850_e56549_d_n11, assign33850_e56549_d_n13, assign33850_e56549_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33850_e56547: f64 = (1.0 - locals.var_noieta);
        (assign33850_e56547, (-locals.var_noieta_dn0), (-locals.var_noieta_dn2), (-locals.var_noieta_dn3), (-locals.var_noieta_dn4), (-locals.var_noieta_dn5), (-locals.var_noieta_dn6), (-locals.var_noieta_dn7), (-locals.var_noieta_dn8), (-locals.var_noieta_dn9), (-locals.var_noieta_dn10), (-locals.var_noieta_dn11), (-locals.var_noieta_dn13), (-locals.var_noieta_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33850_e56549;
        locals.var_t5_dn0 = assign33850_e56549_d_n0;
        locals.var_t5_dn2 = assign33850_e56549_d_n2;
        locals.var_t5_dn3 = assign33850_e56549_d_n3;
        locals.var_t5_dn4 = assign33850_e56549_d_n4;
        locals.var_t5_dn5 = assign33850_e56549_d_n5;
        locals.var_t5_dn6 = assign33850_e56549_d_n6;
        locals.var_t5_dn7 = assign33850_e56549_d_n7;
        locals.var_t5_dn8 = assign33850_e56549_d_n8;
        locals.var_t5_dn9 = assign33850_e56549_d_n9;
        locals.var_t5_dn10 = assign33850_e56549_d_n10;
        locals.var_t5_dn11 = assign33850_e56549_d_n11;
        locals.var_t5_dn13 = assign33850_e56549_d_n13;
        locals.var_t5_dn14 = assign33850_e56549_d_n14;

        let (assign33860_e56562, assign33860_e56562_d_n0, assign33860_e56562_d_n2, assign33860_e56562_d_n3, assign33860_e56562_d_n4, assign33860_e56562_d_n5, assign33860_e56562_d_n6, assign33860_e56562_d_n7, assign33860_e56562_d_n8, assign33860_e56562_d_n9, assign33860_e56562_d_n10, assign33860_e56562_d_n11, assign33860_e56562_d_n13, assign33860_e56562_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33860_e56556: f64 = (2.0 * locals.var_noiwi);
        let assign33860_e56558: f64 = (assign33860_e56556 / locals.var_qis);
        let assign33860_e56560: f64 = (assign33860_e56558 * locals.var_nvtm);
        (assign33860_e56560, ((((((2.0 * locals.var_noiwi_dn0) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn0)), ((((((2.0 * locals.var_noiwi_dn2) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn2)), ((((((2.0 * locals.var_noiwi_dn3) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn3)), ((((((2.0 * locals.var_noiwi_dn4) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn4)), ((((((2.0 * locals.var_noiwi_dn5) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn5)), ((((((2.0 * locals.var_noiwi_dn6) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn6)), ((((((2.0 * locals.var_noiwi_dn7) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn7)), ((((((2.0 * locals.var_noiwi_dn8) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn8)), ((((((2.0 * locals.var_noiwi_dn9) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn9)), ((((((2.0 * locals.var_noiwi_dn10) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn10)), ((((((2.0 * locals.var_noiwi_dn11) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn11)), ((((((2.0 * locals.var_noiwi_dn13) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn13)), ((((((2.0 * locals.var_noiwi_dn14) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33860_e56562;
        locals.var_t6_dn0 = assign33860_e56562_d_n0;
        locals.var_t6_dn2 = assign33860_e56562_d_n2;
        locals.var_t6_dn3 = assign33860_e56562_d_n3;
        locals.var_t6_dn4 = assign33860_e56562_d_n4;
        locals.var_t6_dn5 = assign33860_e56562_d_n5;
        locals.var_t6_dn6 = assign33860_e56562_d_n6;
        locals.var_t6_dn7 = assign33860_e56562_d_n7;
        locals.var_t6_dn8 = assign33860_e56562_d_n8;
        locals.var_t6_dn9 = assign33860_e56562_d_n9;
        locals.var_t6_dn10 = assign33860_e56562_d_n10;
        locals.var_t6_dn11 = assign33860_e56562_d_n11;
        locals.var_t6_dn13 = assign33860_e56562_d_n13;
        locals.var_t6_dn14 = assign33860_e56562_d_n14;

        let (assign33870_e56571, assign33870_e56571_d_n0, assign33870_e56571_d_n2, assign33870_e56571_d_n3, assign33870_e56571_d_n4, assign33870_e56571_d_n5, assign33870_e56571_d_n6, assign33870_e56571_d_n7, assign33870_e56571_d_n8, assign33870_e56571_d_n9, assign33870_e56571_d_n10, assign33870_e56571_d_n11, assign33870_e56571_d_n13, assign33870_e56571_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33870_e56569: f64 = (locals.var_t4 + locals.var_t6);
        (assign33870_e56569, (locals.var_t4_dn0 + locals.var_t6_dn0), (locals.var_t4_dn2 + locals.var_t6_dn2), (locals.var_t4_dn3 + locals.var_t6_dn3), (locals.var_t4_dn4 + locals.var_t6_dn4), (locals.var_t4_dn5 + locals.var_t6_dn5), (locals.var_t4_dn6 + locals.var_t6_dn6), (locals.var_t4_dn7 + locals.var_t6_dn7), (locals.var_t4_dn8 + locals.var_t6_dn8), (locals.var_t4_dn9 + locals.var_t6_dn9), (locals.var_t4_dn10 + locals.var_t6_dn10), (locals.var_t4_dn11 + locals.var_t6_dn11), (locals.var_t4_dn13 + locals.var_t6_dn13), (locals.var_t4_dn14 + locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33870_e56571;
        locals.var_t7_dn0 = assign33870_e56571_d_n0;
        locals.var_t7_dn2 = assign33870_e56571_d_n2;
        locals.var_t7_dn3 = assign33870_e56571_d_n3;
        locals.var_t7_dn4 = assign33870_e56571_d_n4;
        locals.var_t7_dn5 = assign33870_e56571_d_n5;
        locals.var_t7_dn6 = assign33870_e56571_d_n6;
        locals.var_t7_dn7 = assign33870_e56571_d_n7;
        locals.var_t7_dn8 = assign33870_e56571_d_n8;
        locals.var_t7_dn9 = assign33870_e56571_d_n9;
        locals.var_t7_dn10 = assign33870_e56571_d_n10;
        locals.var_t7_dn11 = assign33870_e56571_d_n11;
        locals.var_t7_dn13 = assign33870_e56571_d_n13;
        locals.var_t7_dn14 = assign33870_e56571_d_n14;

        let (assign33880_e56580, assign33880_e56580_d_n0, assign33880_e56580_d_n2, assign33880_e56580_d_n3, assign33880_e56580_d_n4, assign33880_e56580_d_n5, assign33880_e56580_d_n6, assign33880_e56580_d_n7, assign33880_e56580_d_n8, assign33880_e56580_d_n9, assign33880_e56580_d_n10, assign33880_e56580_d_n11, assign33880_e56580_d_n13, assign33880_e56580_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33880_e56578: f64 = (locals.var_t5 * locals.var_t5);
        (assign33880_e56578, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_2, locals.var_t5_2_dn0, locals.var_t5_2_dn2, locals.var_t5_2_dn3, locals.var_t5_2_dn4, locals.var_t5_2_dn5, locals.var_t5_2_dn6, locals.var_t5_2_dn7, locals.var_t5_2_dn8, locals.var_t5_2_dn9, locals.var_t5_2_dn10, locals.var_t5_2_dn11, locals.var_t5_2_dn13, locals.var_t5_2_dn14,)
    }
};
        locals.var_t5_2 = assign33880_e56580;
        locals.var_t5_2_dn0 = assign33880_e56580_d_n0;
        locals.var_t5_2_dn2 = assign33880_e56580_d_n2;
        locals.var_t5_2_dn3 = assign33880_e56580_d_n3;
        locals.var_t5_2_dn4 = assign33880_e56580_d_n4;
        locals.var_t5_2_dn5 = assign33880_e56580_d_n5;
        locals.var_t5_2_dn6 = assign33880_e56580_d_n6;
        locals.var_t5_2_dn7 = assign33880_e56580_d_n7;
        locals.var_t5_2_dn8 = assign33880_e56580_d_n8;
        locals.var_t5_2_dn9 = assign33880_e56580_d_n9;
        locals.var_t5_2_dn10 = assign33880_e56580_d_n10;
        locals.var_t5_2_dn11 = assign33880_e56580_d_n11;
        locals.var_t5_2_dn13 = assign33880_e56580_d_n13;
        locals.var_t5_2_dn14 = assign33880_e56580_d_n14;

        let (assign33890_e56589, assign33890_e56589_d_n0, assign33890_e56589_d_n2, assign33890_e56589_d_n3, assign33890_e56589_d_n4, assign33890_e56589_d_n5, assign33890_e56589_d_n6, assign33890_e56589_d_n7, assign33890_e56589_d_n8, assign33890_e56589_d_n9, assign33890_e56589_d_n10, assign33890_e56589_d_n11, assign33890_e56589_d_n13, assign33890_e56589_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33890_e56587: f64 = (locals.var_t5_2 * locals.var_t5);
        (assign33890_e56587, ((locals.var_t5_2_dn0 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn0)), ((locals.var_t5_2_dn2 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn2)), ((locals.var_t5_2_dn3 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn3)), ((locals.var_t5_2_dn4 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn4)), ((locals.var_t5_2_dn5 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn5)), ((locals.var_t5_2_dn6 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn6)), ((locals.var_t5_2_dn7 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn7)), ((locals.var_t5_2_dn8 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn8)), ((locals.var_t5_2_dn9 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn9)), ((locals.var_t5_2_dn10 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn10)), ((locals.var_t5_2_dn11 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn11)), ((locals.var_t5_2_dn13 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn13)), ((locals.var_t5_2_dn14 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_3, locals.var_t5_3_dn0, locals.var_t5_3_dn2, locals.var_t5_3_dn3, locals.var_t5_3_dn4, locals.var_t5_3_dn5, locals.var_t5_3_dn6, locals.var_t5_3_dn7, locals.var_t5_3_dn8, locals.var_t5_3_dn9, locals.var_t5_3_dn10, locals.var_t5_3_dn11, locals.var_t5_3_dn13, locals.var_t5_3_dn14,)
    }
};
        locals.var_t5_3 = assign33890_e56589;
        locals.var_t5_3_dn0 = assign33890_e56589_d_n0;
        locals.var_t5_3_dn2 = assign33890_e56589_d_n2;
        locals.var_t5_3_dn3 = assign33890_e56589_d_n3;
        locals.var_t5_3_dn4 = assign33890_e56589_d_n4;
        locals.var_t5_3_dn5 = assign33890_e56589_d_n5;
        locals.var_t5_3_dn6 = assign33890_e56589_d_n6;
        locals.var_t5_3_dn7 = assign33890_e56589_d_n7;
        locals.var_t5_3_dn8 = assign33890_e56589_d_n8;
        locals.var_t5_3_dn9 = assign33890_e56589_d_n9;
        locals.var_t5_3_dn10 = assign33890_e56589_d_n10;
        locals.var_t5_3_dn11 = assign33890_e56589_d_n11;
        locals.var_t5_3_dn13 = assign33890_e56589_d_n13;
        locals.var_t5_3_dn14 = assign33890_e56589_d_n14;

        let (assign33900_e56598, assign33900_e56598_d_n0, assign33900_e56598_d_n2, assign33900_e56598_d_n3, assign33900_e56598_d_n4, assign33900_e56598_d_n5, assign33900_e56598_d_n6, assign33900_e56598_d_n7, assign33900_e56598_d_n8, assign33900_e56598_d_n9, assign33900_e56598_d_n10, assign33900_e56598_d_n11, assign33900_e56598_d_n13, assign33900_e56598_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33900_e56596: f64 = (locals.var_t5_3 * locals.var_t5);
        (assign33900_e56596, ((locals.var_t5_3_dn0 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn0)), ((locals.var_t5_3_dn2 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn2)), ((locals.var_t5_3_dn3 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn3)), ((locals.var_t5_3_dn4 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn4)), ((locals.var_t5_3_dn5 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn5)), ((locals.var_t5_3_dn6 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn6)), ((locals.var_t5_3_dn7 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn7)), ((locals.var_t5_3_dn8 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn8)), ((locals.var_t5_3_dn9 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn9)), ((locals.var_t5_3_dn10 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn10)), ((locals.var_t5_3_dn11 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn11)), ((locals.var_t5_3_dn13 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn13)), ((locals.var_t5_3_dn14 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_4, locals.var_t5_4_dn0, locals.var_t5_4_dn2, locals.var_t5_4_dn3, locals.var_t5_4_dn4, locals.var_t5_4_dn5, locals.var_t5_4_dn6, locals.var_t5_4_dn7, locals.var_t5_4_dn8, locals.var_t5_4_dn9, locals.var_t5_4_dn10, locals.var_t5_4_dn11, locals.var_t5_4_dn13, locals.var_t5_4_dn14,)
    }
};
        locals.var_t5_4 = assign33900_e56598;
        locals.var_t5_4_dn0 = assign33900_e56598_d_n0;
        locals.var_t5_4_dn2 = assign33900_e56598_d_n2;
        locals.var_t5_4_dn3 = assign33900_e56598_d_n3;
        locals.var_t5_4_dn4 = assign33900_e56598_d_n4;
        locals.var_t5_4_dn5 = assign33900_e56598_d_n5;
        locals.var_t5_4_dn6 = assign33900_e56598_d_n6;
        locals.var_t5_4_dn7 = assign33900_e56598_d_n7;
        locals.var_t5_4_dn8 = assign33900_e56598_d_n8;
        locals.var_t5_4_dn9 = assign33900_e56598_d_n9;
        locals.var_t5_4_dn10 = assign33900_e56598_d_n10;
        locals.var_t5_4_dn11 = assign33900_e56598_d_n11;
        locals.var_t5_4_dn13 = assign33900_e56598_d_n13;
        locals.var_t5_4_dn14 = assign33900_e56598_d_n14;

        let (assign33910_e56607, assign33910_e56607_d_n0, assign33910_e56607_d_n2, assign33910_e56607_d_n3, assign33910_e56607_d_n4, assign33910_e56607_d_n5, assign33910_e56607_d_n6, assign33910_e56607_d_n7, assign33910_e56607_d_n8, assign33910_e56607_d_n9, assign33910_e56607_d_n10, assign33910_e56607_d_n11, assign33910_e56607_d_n13, assign33910_e56607_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33910_e56605: f64 = (locals.var_t7 * locals.var_t7);
        (assign33910_e56605, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_2, locals.var_t7_2_dn0, locals.var_t7_2_dn2, locals.var_t7_2_dn3, locals.var_t7_2_dn4, locals.var_t7_2_dn5, locals.var_t7_2_dn6, locals.var_t7_2_dn7, locals.var_t7_2_dn8, locals.var_t7_2_dn9, locals.var_t7_2_dn10, locals.var_t7_2_dn11, locals.var_t7_2_dn13, locals.var_t7_2_dn14,)
    }
};
        locals.var_t7_2 = assign33910_e56607;
        locals.var_t7_2_dn0 = assign33910_e56607_d_n0;
        locals.var_t7_2_dn2 = assign33910_e56607_d_n2;
        locals.var_t7_2_dn3 = assign33910_e56607_d_n3;
        locals.var_t7_2_dn4 = assign33910_e56607_d_n4;
        locals.var_t7_2_dn5 = assign33910_e56607_d_n5;
        locals.var_t7_2_dn6 = assign33910_e56607_d_n6;
        locals.var_t7_2_dn7 = assign33910_e56607_d_n7;
        locals.var_t7_2_dn8 = assign33910_e56607_d_n8;
        locals.var_t7_2_dn9 = assign33910_e56607_d_n9;
        locals.var_t7_2_dn10 = assign33910_e56607_d_n10;
        locals.var_t7_2_dn11 = assign33910_e56607_d_n11;
        locals.var_t7_2_dn13 = assign33910_e56607_d_n13;
        locals.var_t7_2_dn14 = assign33910_e56607_d_n14;

        let (assign33920_e56616, assign33920_e56616_d_n0, assign33920_e56616_d_n2, assign33920_e56616_d_n3, assign33920_e56616_d_n4, assign33920_e56616_d_n5, assign33920_e56616_d_n6, assign33920_e56616_d_n7, assign33920_e56616_d_n8, assign33920_e56616_d_n9, assign33920_e56616_d_n10, assign33920_e56616_d_n11, assign33920_e56616_d_n13, assign33920_e56616_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33920_e56614: f64 = (locals.var_t7_2 * locals.var_t7);
        (assign33920_e56614, ((locals.var_t7_2_dn0 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn0)), ((locals.var_t7_2_dn2 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn2)), ((locals.var_t7_2_dn3 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn3)), ((locals.var_t7_2_dn4 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn4)), ((locals.var_t7_2_dn5 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn5)), ((locals.var_t7_2_dn6 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn6)), ((locals.var_t7_2_dn7 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn7)), ((locals.var_t7_2_dn8 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn8)), ((locals.var_t7_2_dn9 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn9)), ((locals.var_t7_2_dn10 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn10)), ((locals.var_t7_2_dn11 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn11)), ((locals.var_t7_2_dn13 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn13)), ((locals.var_t7_2_dn14 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_3, locals.var_t7_3_dn0, locals.var_t7_3_dn2, locals.var_t7_3_dn3, locals.var_t7_3_dn4, locals.var_t7_3_dn5, locals.var_t7_3_dn6, locals.var_t7_3_dn7, locals.var_t7_3_dn8, locals.var_t7_3_dn9, locals.var_t7_3_dn10, locals.var_t7_3_dn11, locals.var_t7_3_dn13, locals.var_t7_3_dn14,)
    }
};
        locals.var_t7_3 = assign33920_e56616;
        locals.var_t7_3_dn0 = assign33920_e56616_d_n0;
        locals.var_t7_3_dn2 = assign33920_e56616_d_n2;
        locals.var_t7_3_dn3 = assign33920_e56616_d_n3;
        locals.var_t7_3_dn4 = assign33920_e56616_d_n4;
        locals.var_t7_3_dn5 = assign33920_e56616_d_n5;
        locals.var_t7_3_dn6 = assign33920_e56616_d_n6;
        locals.var_t7_3_dn7 = assign33920_e56616_d_n7;
        locals.var_t7_3_dn8 = assign33920_e56616_d_n8;
        locals.var_t7_3_dn9 = assign33920_e56616_d_n9;
        locals.var_t7_3_dn10 = assign33920_e56616_d_n10;
        locals.var_t7_3_dn11 = assign33920_e56616_d_n11;
        locals.var_t7_3_dn13 = assign33920_e56616_d_n13;
        locals.var_t7_3_dn14 = assign33920_e56616_d_n14;

        let (assign33930_e56625, assign33930_e56625_d_n0, assign33930_e56625_d_n2, assign33930_e56625_d_n3, assign33930_e56625_d_n4, assign33930_e56625_d_n5, assign33930_e56625_d_n6, assign33930_e56625_d_n7, assign33930_e56625_d_n8, assign33930_e56625_d_n9, assign33930_e56625_d_n10, assign33930_e56625_d_n11, assign33930_e56625_d_n13, assign33930_e56625_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33930_e56623: f64 = (locals.var_t7_3 * locals.var_t7);
        (assign33930_e56623, ((locals.var_t7_3_dn0 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn0)), ((locals.var_t7_3_dn2 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn2)), ((locals.var_t7_3_dn3 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn3)), ((locals.var_t7_3_dn4 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn4)), ((locals.var_t7_3_dn5 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn5)), ((locals.var_t7_3_dn6 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn6)), ((locals.var_t7_3_dn7 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn7)), ((locals.var_t7_3_dn8 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn8)), ((locals.var_t7_3_dn9 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn9)), ((locals.var_t7_3_dn10 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn10)), ((locals.var_t7_3_dn11 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn11)), ((locals.var_t7_3_dn13 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn13)), ((locals.var_t7_3_dn14 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_4, locals.var_t7_4_dn0, locals.var_t7_4_dn2, locals.var_t7_4_dn3, locals.var_t7_4_dn4, locals.var_t7_4_dn5, locals.var_t7_4_dn6, locals.var_t7_4_dn7, locals.var_t7_4_dn8, locals.var_t7_4_dn9, locals.var_t7_4_dn10, locals.var_t7_4_dn11, locals.var_t7_4_dn13, locals.var_t7_4_dn14,)
    }
};
        locals.var_t7_4 = assign33930_e56625;
        locals.var_t7_4_dn0 = assign33930_e56625_d_n0;
        locals.var_t7_4_dn2 = assign33930_e56625_d_n2;
        locals.var_t7_4_dn3 = assign33930_e56625_d_n3;
        locals.var_t7_4_dn4 = assign33930_e56625_d_n4;
        locals.var_t7_4_dn5 = assign33930_e56625_d_n5;
        locals.var_t7_4_dn6 = assign33930_e56625_d_n6;
        locals.var_t7_4_dn7 = assign33930_e56625_d_n7;
        locals.var_t7_4_dn8 = assign33930_e56625_d_n8;
        locals.var_t7_4_dn9 = assign33930_e56625_d_n9;
        locals.var_t7_4_dn10 = assign33930_e56625_d_n10;
        locals.var_t7_4_dn11 = assign33930_e56625_d_n11;
        locals.var_t7_4_dn13 = assign33930_e56625_d_n13;
        locals.var_t7_4_dn14 = assign33930_e56625_d_n14;

        let (assign33940_e56634, assign33940_e56634_d_n0, assign33940_e56634_d_n2, assign33940_e56634_d_n3, assign33940_e56634_d_n4, assign33940_e56634_d_n5, assign33940_e56634_d_n6, assign33940_e56634_d_n7, assign33940_e56634_d_n8, assign33940_e56634_d_n9, assign33940_e56634_d_n10, assign33940_e56634_d_n11, assign33940_e56634_d_n13, assign33940_e56634_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33940_e56632: f64 = (locals.var_t7_4 * locals.var_t7);
        (assign33940_e56632, ((locals.var_t7_4_dn0 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn0)), ((locals.var_t7_4_dn2 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn2)), ((locals.var_t7_4_dn3 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn3)), ((locals.var_t7_4_dn4 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn4)), ((locals.var_t7_4_dn5 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn5)), ((locals.var_t7_4_dn6 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn6)), ((locals.var_t7_4_dn7 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn7)), ((locals.var_t7_4_dn8 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn8)), ((locals.var_t7_4_dn9 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn9)), ((locals.var_t7_4_dn10 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn10)), ((locals.var_t7_4_dn11 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn11)), ((locals.var_t7_4_dn13 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn13)), ((locals.var_t7_4_dn14 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_5, locals.var_t7_5_dn0, locals.var_t7_5_dn2, locals.var_t7_5_dn3, locals.var_t7_5_dn4, locals.var_t7_5_dn5, locals.var_t7_5_dn6, locals.var_t7_5_dn7, locals.var_t7_5_dn8, locals.var_t7_5_dn9, locals.var_t7_5_dn10, locals.var_t7_5_dn11, locals.var_t7_5_dn13, locals.var_t7_5_dn14,)
    }
};
        locals.var_t7_5 = assign33940_e56634;
        locals.var_t7_5_dn0 = assign33940_e56634_d_n0;
        locals.var_t7_5_dn2 = assign33940_e56634_d_n2;
        locals.var_t7_5_dn3 = assign33940_e56634_d_n3;
        locals.var_t7_5_dn4 = assign33940_e56634_d_n4;
        locals.var_t7_5_dn5 = assign33940_e56634_d_n5;
        locals.var_t7_5_dn6 = assign33940_e56634_d_n6;
        locals.var_t7_5_dn7 = assign33940_e56634_d_n7;
        locals.var_t7_5_dn8 = assign33940_e56634_d_n8;
        locals.var_t7_5_dn9 = assign33940_e56634_d_n9;
        locals.var_t7_5_dn10 = assign33940_e56634_d_n10;
        locals.var_t7_5_dn11 = assign33940_e56634_d_n11;
        locals.var_t7_5_dn13 = assign33940_e56634_d_n13;
        locals.var_t7_5_dn14 = assign33940_e56634_d_n14;

        let (assign33950_e56643, assign33950_e56643_d_n0, assign33950_e56643_d_n2, assign33950_e56643_d_n3, assign33950_e56643_d_n4, assign33950_e56643_d_n5, assign33950_e56643_d_n6, assign33950_e56643_d_n7, assign33950_e56643_d_n8, assign33950_e56643_d_n9, assign33950_e56643_d_n10, assign33950_e56643_d_n11, assign33950_e56643_d_n13, assign33950_e56643_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33950_e56641: f64 = (0.5 * locals.var_t4);
        (assign33950_e56641, (0.5 * locals.var_t4_dn0), (0.5 * locals.var_t4_dn2), (0.5 * locals.var_t4_dn3), (0.5 * locals.var_t4_dn4), (0.5 * locals.var_t4_dn5), (0.5 * locals.var_t4_dn6), (0.5 * locals.var_t4_dn7), (0.5 * locals.var_t4_dn8), (0.5 * locals.var_t4_dn9), (0.5 * locals.var_t4_dn10), (0.5 * locals.var_t4_dn11), (0.5 * locals.var_t4_dn13), (0.5 * locals.var_t4_dn14),)
    } else {
        (locals.var_gamma1, locals.var_gamma1_dn0, locals.var_gamma1_dn2, locals.var_gamma1_dn3, locals.var_gamma1_dn4, locals.var_gamma1_dn5, locals.var_gamma1_dn6, locals.var_gamma1_dn7, locals.var_gamma1_dn8, locals.var_gamma1_dn9, locals.var_gamma1_dn10, locals.var_gamma1_dn11, locals.var_gamma1_dn13, locals.var_gamma1_dn14,)
    }
};
        locals.var_gamma1 = assign33950_e56643;
        locals.var_gamma1_dn0 = assign33950_e56643_d_n0;
        locals.var_gamma1_dn2 = assign33950_e56643_d_n2;
        locals.var_gamma1_dn3 = assign33950_e56643_d_n3;
        locals.var_gamma1_dn4 = assign33950_e56643_d_n4;
        locals.var_gamma1_dn5 = assign33950_e56643_d_n5;
        locals.var_gamma1_dn6 = assign33950_e56643_d_n6;
        locals.var_gamma1_dn7 = assign33950_e56643_d_n7;
        locals.var_gamma1_dn8 = assign33950_e56643_d_n8;
        locals.var_gamma1_dn9 = assign33950_e56643_d_n9;
        locals.var_gamma1_dn10 = assign33950_e56643_d_n10;
        locals.var_gamma1_dn11 = assign33950_e56643_d_n11;
        locals.var_gamma1_dn13 = assign33950_e56643_d_n13;
        locals.var_gamma1_dn14 = assign33950_e56643_d_n14;

        let (assign33960_e56654, assign33960_e56654_d_n0, assign33960_e56654_d_n2, assign33960_e56654_d_n3, assign33960_e56654_d_n4, assign33960_e56654_d_n5, assign33960_e56654_d_n6, assign33960_e56654_d_n7, assign33960_e56654_d_n8, assign33960_e56654_d_n9, assign33960_e56654_d_n10, assign33960_e56654_d_n11, assign33960_e56654_d_n13, assign33960_e56654_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33960_e56651: f64 = (6.0 * locals.var_t7);
        let assign33960_e56652: f64 = (locals.var_t5_2 / assign33960_e56651);
        (assign33960_e56652, (((locals.var_t5_2_dn0 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn0))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn2 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn2))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn3 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn3))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn4 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn4))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn5 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn5))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn6 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn6))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn7 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn7))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn8 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn8))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn9 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn9))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn10 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn10))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn11 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn11))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn13 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn13))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn14 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn14))) / (assign33960_e56651 * assign33960_e56651)),)
    } else {
        (locals.var_gamma2, locals.var_gamma2_dn0, locals.var_gamma2_dn2, locals.var_gamma2_dn3, locals.var_gamma2_dn4, locals.var_gamma2_dn5, locals.var_gamma2_dn6, locals.var_gamma2_dn7, locals.var_gamma2_dn8, locals.var_gamma2_dn9, locals.var_gamma2_dn10, locals.var_gamma2_dn11, locals.var_gamma2_dn13, locals.var_gamma2_dn14,)
    }
};
        locals.var_gamma2 = assign33960_e56654;
        locals.var_gamma2_dn0 = assign33960_e56654_d_n0;
        locals.var_gamma2_dn2 = assign33960_e56654_d_n2;
        locals.var_gamma2_dn3 = assign33960_e56654_d_n3;
        locals.var_gamma2_dn4 = assign33960_e56654_d_n4;
        locals.var_gamma2_dn5 = assign33960_e56654_d_n5;
        locals.var_gamma2_dn6 = assign33960_e56654_d_n6;
        locals.var_gamma2_dn7 = assign33960_e56654_d_n7;
        locals.var_gamma2_dn8 = assign33960_e56654_d_n8;
        locals.var_gamma2_dn9 = assign33960_e56654_d_n9;
        locals.var_gamma2_dn10 = assign33960_e56654_d_n10;
        locals.var_gamma2_dn11 = assign33960_e56654_d_n11;
        locals.var_gamma2_dn13 = assign33960_e56654_d_n13;
        locals.var_gamma2_dn14 = assign33960_e56654_d_n14;

        let (assign33970_e56667, assign33970_e56667_d_n0, assign33970_e56667_d_n2, assign33970_e56667_d_n3, assign33970_e56667_d_n4, assign33970_e56667_d_n5, assign33970_e56667_d_n6, assign33970_e56667_d_n7, assign33970_e56667_d_n8, assign33970_e56667_d_n9, assign33970_e56667_d_n10, assign33970_e56667_d_n11, assign33970_e56667_d_n13, assign33970_e56667_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33970_e56661: f64 = (locals.var_moc / locals.var_dvsat);
        let assign33970_e56664: f64 = (locals.var_gamma1 + locals.var_gamma2);
        let assign33970_e56665: f64 = (assign33970_e56661 * assign33970_e56664);
        (assign33970_e56665, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn0 + locals.var_gamma2_dn0))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn2 + locals.var_gamma2_dn2))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn3 + locals.var_gamma2_dn3))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn4 + locals.var_gamma2_dn4))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn5 + locals.var_gamma2_dn5))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn6 + locals.var_gamma2_dn6))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn7 + locals.var_gamma2_dn7))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn8 + locals.var_gamma2_dn8))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn9 + locals.var_gamma2_dn9))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn10 + locals.var_gamma2_dn10))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn11 + locals.var_gamma2_dn11))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn13 + locals.var_gamma2_dn13))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn14 + locals.var_gamma2_dn14))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign33970_e56667;
        locals.var_gamma_dn0 = assign33970_e56667_d_n0;
        locals.var_gamma_dn2 = assign33970_e56667_d_n2;
        locals.var_gamma_dn3 = assign33970_e56667_d_n3;
        locals.var_gamma_dn4 = assign33970_e56667_d_n4;
        locals.var_gamma_dn5 = assign33970_e56667_d_n5;
        locals.var_gamma_dn6 = assign33970_e56667_d_n6;
        locals.var_gamma_dn7 = assign33970_e56667_d_n7;
        locals.var_gamma_dn8 = assign33970_e56667_d_n8;
        locals.var_gamma_dn9 = assign33970_e56667_d_n9;
        locals.var_gamma_dn10 = assign33970_e56667_d_n10;
        locals.var_gamma_dn11 = assign33970_e56667_d_n11;
        locals.var_gamma_dn13 = assign33970_e56667_d_n13;
        locals.var_gamma_dn14 = assign33970_e56667_d_n14;

        let (assign33980_e56676, assign33980_e56676_d_n0, assign33980_e56676_d_n2, assign33980_e56676_d_n3, assign33980_e56676_d_n4, assign33980_e56676_d_n5, assign33980_e56676_d_n6, assign33980_e56676_d_n7, assign33980_e56676_d_n8, assign33980_e56676_d_n9, assign33980_e56676_d_n10, assign33980_e56676_d_n11, assign33980_e56676_d_n13, assign33980_e56676_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33980_e56674: f64 = (locals.var_t4 / locals.var_t7_2);
        (assign33980_e56674, (((locals.var_t4_dn0 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn0)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn2 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn2)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn3 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn3)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn4 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn4)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn5 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn5)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn6 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn6)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn7 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn7)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn8 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn8)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn9 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn9)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn10 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn10)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn11 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn11)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn13 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn13)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn14 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn14)) / (locals.var_t7_2 * locals.var_t7_2)),)
    } else {
        (locals.var_delta1, locals.var_delta1_dn0, locals.var_delta1_dn2, locals.var_delta1_dn3, locals.var_delta1_dn4, locals.var_delta1_dn5, locals.var_delta1_dn6, locals.var_delta1_dn7, locals.var_delta1_dn8, locals.var_delta1_dn9, locals.var_delta1_dn10, locals.var_delta1_dn11, locals.var_delta1_dn13, locals.var_delta1_dn14,)
    }
};
        locals.var_delta1 = assign33980_e56676;
        locals.var_delta1_dn0 = assign33980_e56676_d_n0;
        locals.var_delta1_dn2 = assign33980_e56676_d_n2;
        locals.var_delta1_dn3 = assign33980_e56676_d_n3;
        locals.var_delta1_dn4 = assign33980_e56676_d_n4;
        locals.var_delta1_dn5 = assign33980_e56676_d_n5;
        locals.var_delta1_dn6 = assign33980_e56676_d_n6;
        locals.var_delta1_dn7 = assign33980_e56676_d_n7;
        locals.var_delta1_dn8 = assign33980_e56676_d_n8;
        locals.var_delta1_dn9 = assign33980_e56676_d_n9;
        locals.var_delta1_dn10 = assign33980_e56676_d_n10;
        locals.var_delta1_dn11 = assign33980_e56676_d_n11;
        locals.var_delta1_dn13 = assign33980_e56676_d_n13;
        locals.var_delta1_dn14 = assign33980_e56676_d_n14;

        let (assign33990_e56693, assign33990_e56693_d_n0, assign33990_e56693_d_n2, assign33990_e56693_d_n3, assign33990_e56693_d_n4, assign33990_e56693_d_n5, assign33990_e56693_d_n6, assign33990_e56693_d_n7, assign33990_e56693_d_n8, assign33990_e56693_d_n9, assign33990_e56693_d_n10, assign33990_e56693_d_n11, assign33990_e56693_d_n13, assign33990_e56693_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33990_e56683: f64 = (6.0 * locals.var_t4);
        let assign33990_e56685: f64 = (assign33990_e56683 + locals.var_t6);
        let assign33990_e56687: f64 = (assign33990_e56685 * locals.var_t5_2);
        let assign33990_e56690: f64 = (15.0 * locals.var_t7_4);
        let assign33990_e56691: f64 = (assign33990_e56687 / assign33990_e56690);
        (assign33990_e56691, (((((((6.0 * locals.var_t4_dn0) + locals.var_t6_dn0) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn0)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn0))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn2) + locals.var_t6_dn2) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn2)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn2))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn3) + locals.var_t6_dn3) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn3)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn3))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn4) + locals.var_t6_dn4) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn4)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn4))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn5) + locals.var_t6_dn5) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn5)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn5))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn6) + locals.var_t6_dn6) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn6)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn6))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn7) + locals.var_t6_dn7) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn7)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn7))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn8) + locals.var_t6_dn8) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn8)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn8))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn9) + locals.var_t6_dn9) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn9)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn9))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn10) + locals.var_t6_dn10) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn10)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn10))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn11) + locals.var_t6_dn11) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn11)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn11))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn13) + locals.var_t6_dn13) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn13)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn13))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn14) + locals.var_t6_dn14) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn14)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn14))) / (assign33990_e56690 * assign33990_e56690)),)
    } else {
        (locals.var_delta2, locals.var_delta2_dn0, locals.var_delta2_dn2, locals.var_delta2_dn3, locals.var_delta2_dn4, locals.var_delta2_dn5, locals.var_delta2_dn6, locals.var_delta2_dn7, locals.var_delta2_dn8, locals.var_delta2_dn9, locals.var_delta2_dn10, locals.var_delta2_dn11, locals.var_delta2_dn13, locals.var_delta2_dn14,)
    }
};
        locals.var_delta2 = assign33990_e56693;
        locals.var_delta2_dn0 = assign33990_e56693_d_n0;
        locals.var_delta2_dn2 = assign33990_e56693_d_n2;
        locals.var_delta2_dn3 = assign33990_e56693_d_n3;
        locals.var_delta2_dn4 = assign33990_e56693_d_n4;
        locals.var_delta2_dn5 = assign33990_e56693_d_n5;
        locals.var_delta2_dn6 = assign33990_e56693_d_n6;
        locals.var_delta2_dn7 = assign33990_e56693_d_n7;
        locals.var_delta2_dn8 = assign33990_e56693_d_n8;
        locals.var_delta2_dn9 = assign33990_e56693_d_n9;
        locals.var_delta2_dn10 = assign33990_e56693_d_n10;
        locals.var_delta2_dn11 = assign33990_e56693_d_n11;
        locals.var_delta2_dn13 = assign33990_e56693_d_n13;
        locals.var_delta2_dn14 = assign33990_e56693_d_n14;

        let (assign34000_e56704, assign34000_e56704_d_n0, assign34000_e56704_d_n2, assign34000_e56704_d_n3, assign34000_e56704_d_n4, assign34000_e56704_d_n5, assign34000_e56704_d_n6, assign34000_e56704_d_n7, assign34000_e56704_d_n8, assign34000_e56704_d_n9, assign34000_e56704_d_n10, assign34000_e56704_d_n11, assign34000_e56704_d_n13, assign34000_e56704_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34000_e56701: f64 = (9.0 * locals.var_t7_5);
        let assign34000_e56702: f64 = (locals.var_t5_4 / assign34000_e56701);
        (assign34000_e56702, (((locals.var_t5_4_dn0 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn0))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn2 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn2))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn3 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn3))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn4 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn4))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn5 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn5))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn6 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn6))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn7 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn7))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn8 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn8))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn9 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn9))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn10 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn10))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn11 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn11))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn13 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn13))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn14 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn14))) / (assign34000_e56701 * assign34000_e56701)),)
    } else {
        (locals.var_delta3, locals.var_delta3_dn0, locals.var_delta3_dn2, locals.var_delta3_dn3, locals.var_delta3_dn4, locals.var_delta3_dn5, locals.var_delta3_dn6, locals.var_delta3_dn7, locals.var_delta3_dn8, locals.var_delta3_dn9, locals.var_delta3_dn10, locals.var_delta3_dn11, locals.var_delta3_dn13, locals.var_delta3_dn14,)
    }
};
        locals.var_delta3 = assign34000_e56704;
        locals.var_delta3_dn0 = assign34000_e56704_d_n0;
        locals.var_delta3_dn2 = assign34000_e56704_d_n2;
        locals.var_delta3_dn3 = assign34000_e56704_d_n3;
        locals.var_delta3_dn4 = assign34000_e56704_d_n4;
        locals.var_delta3_dn5 = assign34000_e56704_d_n5;
        locals.var_delta3_dn6 = assign34000_e56704_d_n6;
        locals.var_delta3_dn7 = assign34000_e56704_d_n7;
        locals.var_delta3_dn8 = assign34000_e56704_d_n8;
        locals.var_delta3_dn9 = assign34000_e56704_d_n9;
        locals.var_delta3_dn10 = assign34000_e56704_d_n10;
        locals.var_delta3_dn11 = assign34000_e56704_d_n11;
        locals.var_delta3_dn13 = assign34000_e56704_d_n13;
        locals.var_delta3_dn14 = assign34000_e56704_d_n14;

        let (assign34010_e56721, assign34010_e56721_d_n0, assign34010_e56721_d_n2, assign34010_e56721_d_n3, assign34010_e56721_d_n4, assign34010_e56721_d_n5, assign34010_e56721_d_n6, assign34010_e56721_d_n7, assign34010_e56721_d_n8, assign34010_e56721_d_n9, assign34010_e56721_d_n10, assign34010_e56721_d_n11, assign34010_e56721_d_n13, assign34010_e56721_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34010_e56711: f64 = (locals.var_moc / 6.0);
        let assign34010_e56713: f64 = (assign34010_e56711 * locals.var_dvsat3);
        let assign34010_e56716: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34010_e56718: f64 = (assign34010_e56716 + locals.var_delta3);
        let assign34010_e56719: f64 = (assign34010_e56713 * assign34010_e56718);
        (assign34010_e56719, (((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn0)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn2)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn3)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn4)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn5)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn6)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn7)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn8)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn9)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn10)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn11)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn13)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn14)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34010_e56721;
        locals.var_delta_dn0 = assign34010_e56721_d_n0;
        locals.var_delta_dn2 = assign34010_e56721_d_n2;
        locals.var_delta_dn3 = assign34010_e56721_d_n3;
        locals.var_delta_dn4 = assign34010_e56721_d_n4;
        locals.var_delta_dn5 = assign34010_e56721_d_n5;
        locals.var_delta_dn6 = assign34010_e56721_d_n6;
        locals.var_delta_dn7 = assign34010_e56721_d_n7;
        locals.var_delta_dn8 = assign34010_e56721_d_n8;
        locals.var_delta_dn9 = assign34010_e56721_d_n9;
        locals.var_delta_dn10 = assign34010_e56721_d_n10;
        locals.var_delta_dn11 = assign34010_e56721_d_n11;
        locals.var_delta_dn13 = assign34010_e56721_d_n13;
        locals.var_delta_dn14 = assign34010_e56721_d_n14;

        let (assign34100_e56816, assign34100_e56816_d_n0, assign34100_e56816_d_n2, assign34100_e56816_d_n3, assign34100_e56816_d_n4, assign34100_e56816_d_n5, assign34100_e56816_d_n6, assign34100_e56816_d_n7, assign34100_e56816_d_n8, assign34100_e56816_d_n9, assign34100_e56816_d_n10, assign34100_e56816_d_n11, assign34100_e56816_d_n13, assign34100_e56816_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34100_e56805: f64 = (locals.var_noilowid * locals.var_noilowid);
        let assign34100_e56808: f64 = (p.p1716 + locals.var_qia);
        let assign34100_e56809: f64 = (assign34100_e56805 / assign34100_e56808);
        let assign34100_e56812: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign34100_e56813: f64 = (assign34100_e56809 * assign34100_e56812);
        let assign34100_e56814: f64 = (1.0 + assign34100_e56813);
        (assign34100_e56814, (((((((locals.var_noilowid_dn0 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn0)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn0)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn2 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn2)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn2)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn3 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn3)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn3)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn4 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn4)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn4)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn5 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn5)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn5)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn6 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn6)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn6)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn7 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn7)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn7)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn8 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn8)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn8)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn9 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn9)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn9)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn10 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn10)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn10)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn11 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn11)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn11)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn13 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn13)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn13)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn14 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn14)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn14)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign34100_e56816;
        locals.var_t8_dn0 = assign34100_e56816_d_n0;
        locals.var_t8_dn2 = assign34100_e56816_d_n2;
        locals.var_t8_dn3 = assign34100_e56816_d_n3;
        locals.var_t8_dn4 = assign34100_e56816_d_n4;
        locals.var_t8_dn5 = assign34100_e56816_d_n5;
        locals.var_t8_dn6 = assign34100_e56816_d_n6;
        locals.var_t8_dn7 = assign34100_e56816_d_n7;
        locals.var_t8_dn8 = assign34100_e56816_d_n8;
        locals.var_t8_dn9 = assign34100_e56816_d_n9;
        locals.var_t8_dn10 = assign34100_e56816_d_n10;
        locals.var_t8_dn11 = assign34100_e56816_d_n11;
        locals.var_t8_dn13 = assign34100_e56816_d_n13;
        locals.var_t8_dn14 = assign34100_e56816_d_n14;

        let (assign34110_e56833, assign34110_e56833_d_n0, assign34110_e56833_d_n2, assign34110_e56833_d_n3, assign34110_e56833_d_n4, assign34110_e56833_d_n5, assign34110_e56833_d_n6, assign34110_e56833_d_n7, assign34110_e56833_d_n8, assign34110_e56833_d_n9, assign34110_e56833_d_n10, assign34110_e56833_d_n11, assign34110_e56833_d_n13, assign34110_e56833_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34110_e56823: f64 = (locals.var_moc / locals.var_dvsat);
        let assign34110_e56826: f64 = (locals.var_t8 * locals.var_gamma1);
        let assign34110_e56829: f64 = (locals.var_t1 * locals.var_gamma2);
        let assign34110_e56830: f64 = (assign34110_e56826 + assign34110_e56829);
        let assign34110_e56831: f64 = (assign34110_e56823 * assign34110_e56830);
        (assign34110_e56831, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn0 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn0)) + ((locals.var_t1_dn0 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn0))))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn2 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn2)) + ((locals.var_t1_dn2 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn2))))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn3 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn3)) + ((locals.var_t1_dn3 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn3))))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn4 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn4)) + ((locals.var_t1_dn4 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn4))))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn5 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn5)) + ((locals.var_t1_dn5 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn5))))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn6 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn6)) + ((locals.var_t1_dn6 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn6))))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn7 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn7)) + ((locals.var_t1_dn7 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn7))))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn8 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn8)) + ((locals.var_t1_dn8 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn8))))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn9 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn9)) + ((locals.var_t1_dn9 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn9))))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn10 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn10)) + ((locals.var_t1_dn10 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn10))))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn11 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn11)) + ((locals.var_t1_dn11 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn11))))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn13 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn13)) + ((locals.var_t1_dn13 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn13))))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn14 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn14)) + ((locals.var_t1_dn14 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn14))))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign34110_e56833;
        locals.var_gamma_dn0 = assign34110_e56833_d_n0;
        locals.var_gamma_dn2 = assign34110_e56833_d_n2;
        locals.var_gamma_dn3 = assign34110_e56833_d_n3;
        locals.var_gamma_dn4 = assign34110_e56833_d_n4;
        locals.var_gamma_dn5 = assign34110_e56833_d_n5;
        locals.var_gamma_dn6 = assign34110_e56833_d_n6;
        locals.var_gamma_dn7 = assign34110_e56833_d_n7;
        locals.var_gamma_dn8 = assign34110_e56833_d_n8;
        locals.var_gamma_dn9 = assign34110_e56833_d_n9;
        locals.var_gamma_dn10 = assign34110_e56833_d_n10;
        locals.var_gamma_dn11 = assign34110_e56833_d_n11;
        locals.var_gamma_dn13 = assign34110_e56833_d_n13;
        locals.var_gamma_dn14 = assign34110_e56833_d_n14;

        let (assign34130_e56867, assign34130_e56867_d_n0, assign34130_e56867_d_n2, assign34130_e56867_d_n3, assign34130_e56867_d_n4, assign34130_e56867_d_n5, assign34130_e56867_d_n6, assign34130_e56867_d_n7, assign34130_e56867_d_n8, assign34130_e56867_d_n9, assign34130_e56867_d_n10, assign34130_e56867_d_n11, assign34130_e56867_d_n13, assign34130_e56867_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34130_e56855: f64 = (locals.var_moc / 6.0);
        let assign34130_e56857: f64 = (assign34130_e56855 * locals.var_dvsat3);
        let assign34130_e56859: f64 = (assign34130_e56857 * locals.var_t2);
        let assign34130_e56862: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34130_e56864: f64 = (assign34130_e56862 + locals.var_delta3);
        let assign34130_e56865: f64 = (assign34130_e56859 * assign34130_e56864);
        (assign34130_e56865, (((((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn0)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn0)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn2)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn2)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn3)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn3)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn4)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn4)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn5)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn5)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn6)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn6)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn7)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn7)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn8)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn8)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn9)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn9)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn10)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn10)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn11)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn11)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn13)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn13)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn14)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn14)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34130_e56867;
        locals.var_delta_dn0 = assign34130_e56867_d_n0;
        locals.var_delta_dn2 = assign34130_e56867_d_n2;
        locals.var_delta_dn3 = assign34130_e56867_d_n3;
        locals.var_delta_dn4 = assign34130_e56867_d_n4;
        locals.var_delta_dn5 = assign34130_e56867_d_n5;
        locals.var_delta_dn6 = assign34130_e56867_d_n6;
        locals.var_delta_dn7 = assign34130_e56867_d_n7;
        locals.var_delta_dn8 = assign34130_e56867_d_n8;
        locals.var_delta_dn9 = assign34130_e56867_d_n9;
        locals.var_delta_dn10 = assign34130_e56867_d_n10;
        locals.var_delta_dn11 = assign34130_e56867_d_n11;
        locals.var_delta_dn13 = assign34130_e56867_d_n13;
        locals.var_delta_dn14 = assign34130_e56867_d_n14;

    }

    pub(super) fn stamp_transient_block_127(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign34140_e56887, assign34140_e56887_d_n0, assign34140_e56887_d_n2, assign34140_e56887_d_n3, assign34140_e56887_d_n4, assign34140_e56887_d_n5, assign34140_e56887_d_n6, assign34140_e56887_d_n7, assign34140_e56887_d_n8, assign34140_e56887_d_n9, assign34140_e56887_d_n10, assign34140_e56887_d_n11, assign34140_e56887_d_n13, assign34140_e56887_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34140_e56874: f64 = (locals.var_delta / locals.var_gamma);
        let assign34140_e56875: f64 = (assign34140_e56874).sqrt();
        let assign34140_e56877: f64 = (assign34140_e56875 * locals.var_nfintotal);
        let assign34140_e56879: f64 = (assign34140_e56877 * locals.var_coxeff);
        let assign34140_e56881: f64 = (assign34140_e56879 * locals.var_weffcv0);
        let assign34140_e56883: f64 = (assign34140_e56881 * locals.var_leffcv_1);
        let assign34140_e56885: f64 = (assign34140_e56883 / locals.var_noigd0);
        (assign34140_e56885, (((((((((((((locals.var_delta_dn0 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn0)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn0)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn0)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn0)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn2 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn2)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn2)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn2)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn2)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn3 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn3)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn3)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn3)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn3)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn4 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn4)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn4)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn4)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn4)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn5 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn5)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn5)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn5)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn5)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn6 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn6)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn6)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn6)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn6)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn7 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn7)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn7)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn7)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn7)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn8 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn8)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn8)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn8)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn8)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn9 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn9)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn9)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn9)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn9)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn10 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn10)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn10)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn10)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn10)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn11 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn11)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn11)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn11)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn11)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn13 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn13)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn13)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn13)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn13)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn14 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn14)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn14)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn14)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn14)) / (locals.var_noigd0 * locals.var_noigd0)),)
    } else {
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn3, locals.var_sigrat_dn4, locals.var_sigrat_dn5, locals.var_sigrat_dn6, locals.var_sigrat_dn7, locals.var_sigrat_dn8, locals.var_sigrat_dn9, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn13, locals.var_sigrat_dn14,)
    }
};
        locals.var_sigrat = assign34140_e56887;
        locals.var_sigrat_dn0 = assign34140_e56887_d_n0;
        locals.var_sigrat_dn2 = assign34140_e56887_d_n2;
        locals.var_sigrat_dn3 = assign34140_e56887_d_n3;
        locals.var_sigrat_dn4 = assign34140_e56887_d_n4;
        locals.var_sigrat_dn5 = assign34140_e56887_d_n5;
        locals.var_sigrat_dn6 = assign34140_e56887_d_n6;
        locals.var_sigrat_dn7 = assign34140_e56887_d_n7;
        locals.var_sigrat_dn8 = assign34140_e56887_d_n8;
        locals.var_sigrat_dn9 = assign34140_e56887_d_n9;
        locals.var_sigrat_dn10 = assign34140_e56887_d_n10;
        locals.var_sigrat_dn11 = assign34140_e56887_d_n11;
        locals.var_sigrat_dn13 = assign34140_e56887_d_n13;
        locals.var_sigrat_dn14 = assign34140_e56887_d_n14;

        let assign34160_e56893: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign34160_e56893;

        let assign34250_e56944: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign34250_e56944;

        let assign34260_e56947: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign34260_e56947;

        let assign34270_e56950: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign34270_e56950;

        let assign34280_e56953: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign34280_e56953;

        let assign34290_e56956: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign34290_e56956;

        let assign34300_e56959: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign34300_e56959;

        let assign34310_e56962: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign34310_e56962;

        let assign34320_e56965: f64 = if p.p1910 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign34320_e56965;

        let (assign34330_e57042, assign34330_e57042_d_n4,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34330_e56972: f64 = (p.p1912 * locals.var_deltemp);
        let assign34330_e56973: f64 = (1.0 + assign34330_e56972);
        let assign34330_e56975: f64 = (assign34330_e56973 - 1e-6);
        let assign34330_e56977: f64 = (-10000.0);
        let assign34330_e56979: f64 = (assign34330_e56977 * 0.001);
        let (assign34330_e57040, assign34330_e57040_d_n4,) = {
            if (!(assign34330_e56975 < assign34330_e56979)) {
                let assign34330_e56986: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56987: f64 = (1.0 + assign34330_e56986);
                let assign34330_e56989: f64 = (assign34330_e56987 - 1e-6);
                let assign34330_e56993: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56994: f64 = (1.0 + assign34330_e56993);
                let assign34330_e56996: f64 = (assign34330_e56994 - 1e-6);
                let assign34330_e57000: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57001: f64 = (1.0 + assign34330_e57000);
                let assign34330_e57003: f64 = (assign34330_e57001 - 1e-6);
                let assign34330_e57004: f64 = (assign34330_e56996 * assign34330_e57003);
                let assign34330_e57007: f64 = (4.0 * 0.001);
                let assign34330_e57009: f64 = (assign34330_e57007 * 0.001);
                let assign34330_e57010: f64 = (assign34330_e57004 + assign34330_e57009);
                let assign34330_e57011: f64 = (assign34330_e57010).sqrt();
                let assign34330_e57012: f64 = (assign34330_e56989 + assign34330_e57011);
                let assign34330_e57013: f64 = (0.5 * assign34330_e57012);
                (assign34330_e57013, (0.5 * ((p.p1912 * locals.var_deltemp_dn4) + ((((p.p1912 * locals.var_deltemp_dn4) * assign34330_e57003) + (assign34330_e56996 * (p.p1912 * locals.var_deltemp_dn4))) / (2.0 * assign34330_e57011)))),)
            } else {
                let assign34330_e57017: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57018: f64 = (1.0 + assign34330_e57017);
                let assign34330_e57020: f64 = (assign34330_e57018 - 1e-6);
                let assign34330_e57022: f64 = (-10000.0);
                let assign34330_e57024: f64 = (assign34330_e57022 * 0.001);
                let (assign34330_e57039, assign34330_e57039_d_n4,) = {
                    if (assign34330_e57020 < assign34330_e57024) {
                        let assign34330_e57027: f64 = (-0.001);
                        let assign34330_e57029: f64 = (assign34330_e57027 * 0.001);
                        let assign34330_e57033: f64 = (p.p1912 * locals.var_deltemp);
                        let assign34330_e57034: f64 = (1.0 + assign34330_e57033);
                        let assign34330_e57036: f64 = (assign34330_e57034 - 1e-6);
                        let assign34330_e57037: f64 = (assign34330_e57029 / assign34330_e57036);
                        (assign34330_e57037, (-((assign34330_e57029 * (p.p1912 * locals.var_deltemp_dn4)) / (assign34330_e57036 * assign34330_e57036))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34330_e57039, assign34330_e57039_d_n4,)
            }
        };
        (assign34330_e57040, assign34330_e57040_d_n4,)
    } else {
        (locals.var_rdstempvs, locals.var_rdstempvs_dn4,)
    }
};
        locals.var_rdstempvs = assign34330_e57042;
        locals.var_rdstempvs_dn4 = assign34330_e57042_d_n4;

        let assign34340_e57045: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign34340_e57045;

        let (assign34350_e57096, assign34350_e57096_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign34350_e57053: f64 = (-p.p1904);
        let assign34350_e57056: f64 = (-p.p1913);
        let assign34350_e57058: f64 = (assign34350_e57056 * locals.var_deltemp);
        let assign34350_e57060: f64 = (-p.p1904);
        let assign34350_e57061: f64 = (assign34350_e57058 - assign34350_e57060);
        let assign34350_e57063: f64 = (assign34350_e57061 - 1e-6);
        let assign34350_e57065: f64 = (-p.p1913);
        let assign34350_e57067: f64 = (assign34350_e57065 * locals.var_deltemp);
        let assign34350_e57069: f64 = (-p.p1904);
        let assign34350_e57070: f64 = (assign34350_e57067 - assign34350_e57069);
        let assign34350_e57072: f64 = (assign34350_e57070 - 1e-6);
        let assign34350_e57074: f64 = (-p.p1913);
        let assign34350_e57076: f64 = (assign34350_e57074 * locals.var_deltemp);
        let assign34350_e57078: f64 = (-p.p1904);
        let assign34350_e57079: f64 = (assign34350_e57076 - assign34350_e57078);
        let assign34350_e57081: f64 = (assign34350_e57079 - 1e-6);
        let assign34350_e57082: f64 = (assign34350_e57072 * assign34350_e57081);
        let assign34350_e57085: f64 = (-p.p1904);
        let assign34350_e57086: f64 = (4.0 * assign34350_e57085);
        let assign34350_e57088: f64 = (assign34350_e57086 * 1e-6);
        let assign34350_e57089: f64 = (assign34350_e57082 - assign34350_e57088);
        let assign34350_e57090: f64 = (assign34350_e57089).sqrt();
        let assign34350_e57091: f64 = (assign34350_e57063 + assign34350_e57090);
        let assign34350_e57092: f64 = (0.5 * assign34350_e57091);
        let assign34350_e57093: f64 = (assign34350_e57053 + assign34350_e57092);
        let assign34350_e57094: f64 = (p.p1904 + assign34350_e57093);
        (assign34350_e57094, (0.5 * ((assign34350_e57056 * locals.var_deltemp_dn4) + ((((assign34350_e57065 * locals.var_deltemp_dn4) * assign34350_e57081) + (assign34350_e57072 * (assign34350_e57074 * locals.var_deltemp_dn4))) / (2.0 * assign34350_e57090)))),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34350_e57096;
        locals.var_vsatrsd_t_dn4 = assign34350_e57096_d_n4;

        let (assign34360_e57184, assign34360_e57184_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) {
        let assign34360_e57106: f64 = (-p.p1913);
        let assign34360_e57108: f64 = (assign34360_e57106 * locals.var_deltemp);
        let assign34360_e57109: f64 = (1.0 + assign34360_e57108);
        let assign34360_e57111: f64 = (assign34360_e57109 - 1e-6);
        let assign34360_e57113: f64 = (-10000.0);
        let assign34360_e57115: f64 = (assign34360_e57113 * 0.001);
        let (assign34360_e57181, assign34360_e57181_d_n4,) = {
            if (!(assign34360_e57111 < assign34360_e57115)) {
                let assign34360_e57121: f64 = (-p.p1913);
                let assign34360_e57123: f64 = (assign34360_e57121 * locals.var_deltemp);
                let assign34360_e57124: f64 = (1.0 + assign34360_e57123);
                let assign34360_e57126: f64 = (assign34360_e57124 - 1e-6);
                let assign34360_e57129: f64 = (-p.p1913);
                let assign34360_e57131: f64 = (assign34360_e57129 * locals.var_deltemp);
                let assign34360_e57132: f64 = (1.0 + assign34360_e57131);
                let assign34360_e57134: f64 = (assign34360_e57132 - 1e-6);
                let assign34360_e57137: f64 = (-p.p1913);
                let assign34360_e57139: f64 = (assign34360_e57137 * locals.var_deltemp);
                let assign34360_e57140: f64 = (1.0 + assign34360_e57139);
                let assign34360_e57142: f64 = (assign34360_e57140 - 1e-6);
                let assign34360_e57143: f64 = (assign34360_e57134 * assign34360_e57142);
                let assign34360_e57146: f64 = (4.0 * 0.001);
                let assign34360_e57148: f64 = (assign34360_e57146 * 0.001);
                let assign34360_e57149: f64 = (assign34360_e57143 + assign34360_e57148);
                let assign34360_e57150: f64 = (assign34360_e57149).sqrt();
                let assign34360_e57151: f64 = (assign34360_e57126 + assign34360_e57150);
                let assign34360_e57152: f64 = (0.5 * assign34360_e57151);
                (assign34360_e57152, (0.5 * ((assign34360_e57121 * locals.var_deltemp_dn4) + ((((assign34360_e57129 * locals.var_deltemp_dn4) * assign34360_e57142) + (assign34360_e57134 * (assign34360_e57137 * locals.var_deltemp_dn4))) / (2.0 * assign34360_e57150)))),)
            } else {
                let assign34360_e57155: f64 = (-p.p1913);
                let assign34360_e57157: f64 = (assign34360_e57155 * locals.var_deltemp);
                let assign34360_e57158: f64 = (1.0 + assign34360_e57157);
                let assign34360_e57160: f64 = (assign34360_e57158 - 1e-6);
                let assign34360_e57162: f64 = (-10000.0);
                let assign34360_e57164: f64 = (assign34360_e57162 * 0.001);
                let (assign34360_e57180, assign34360_e57180_d_n4,) = {
                    if (assign34360_e57160 < assign34360_e57164) {
                        let assign34360_e57167: f64 = (-0.001);
                        let assign34360_e57169: f64 = (assign34360_e57167 * 0.001);
                        let assign34360_e57172: f64 = (-p.p1913);
                        let assign34360_e57174: f64 = (assign34360_e57172 * locals.var_deltemp);
                        let assign34360_e57175: f64 = (1.0 + assign34360_e57174);
                        let assign34360_e57177: f64 = (assign34360_e57175 - 1e-6);
                        let assign34360_e57178: f64 = (assign34360_e57169 / assign34360_e57177);
                        (assign34360_e57178, (-((assign34360_e57169 * (assign34360_e57172 * locals.var_deltemp_dn4)) / (assign34360_e57177 * assign34360_e57177))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34360_e57180, assign34360_e57180_d_n4,)
            }
        };
        let assign34360_e57182: f64 = (p.p1904 * assign34360_e57181);
        (assign34360_e57182, (p.p1904 * assign34360_e57181_d_n4),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34360_e57184;
        locals.var_vsatrsd_t_dn4 = assign34360_e57184_d_n4;

        let (assign34370_e57192, assign34370_e57192_d_n0, assign34370_e57192_d_n2, assign34370_e57192_d_n3, assign34370_e57192_d_n4, assign34370_e57192_d_n5, assign34370_e57192_d_n6, assign34370_e57192_d_n7, assign34370_e57192_d_n8, assign34370_e57192_d_n9, assign34370_e57192_d_n10, assign34370_e57192_d_n11, assign34370_e57192_d_n13, assign34370_e57192_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34370_e57190: f64 = (locals.var_qis - p.p1906);
        (assign34370_e57190, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34370_e57192;
        locals.var_t0_dn0 = assign34370_e57192_d_n0;
        locals.var_t0_dn2 = assign34370_e57192_d_n2;
        locals.var_t0_dn3 = assign34370_e57192_d_n3;
        locals.var_t0_dn4 = assign34370_e57192_d_n4;
        locals.var_t0_dn5 = assign34370_e57192_d_n5;
        locals.var_t0_dn6 = assign34370_e57192_d_n6;
        locals.var_t0_dn7 = assign34370_e57192_d_n7;
        locals.var_t0_dn8 = assign34370_e57192_d_n8;
        locals.var_t0_dn9 = assign34370_e57192_d_n9;
        locals.var_t0_dn10 = assign34370_e57192_d_n10;
        locals.var_t0_dn11 = assign34370_e57192_d_n11;
        locals.var_t0_dn13 = assign34370_e57192_d_n13;
        locals.var_t0_dn14 = assign34370_e57192_d_n14;

        let (assign34380_e57217, assign34380_e57217_d_n0, assign34380_e57217_d_n2, assign34380_e57217_d_n3, assign34380_e57217_d_n4, assign34380_e57217_d_n5, assign34380_e57217_d_n6, assign34380_e57217_d_n7, assign34380_e57217_d_n8, assign34380_e57217_d_n9, assign34380_e57217_d_n10, assign34380_e57217_d_n11, assign34380_e57217_d_n13, assign34380_e57217_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34380_e57199: f64 = (locals.var_t0 + 0.1);
        let assign34380_e57202: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57205: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57206: f64 = (assign34380_e57202 * assign34380_e57205);
        let assign34380_e57209: f64 = (0.25 * 2.0);
        let assign34380_e57211: f64 = (assign34380_e57209 * 2.0);
        let assign34380_e57212: f64 = (assign34380_e57206 + assign34380_e57211);
        let assign34380_e57213: f64 = (assign34380_e57212).sqrt();
        let assign34380_e57214: f64 = (assign34380_e57199 + assign34380_e57213);
        let assign34380_e57215: f64 = (0.5 * assign34380_e57214);
        (assign34380_e57215, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn0)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn2)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn3)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn4)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn5)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn6)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn7)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn8)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn9)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn10)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn11)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn13)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn14)) / (2.0 * assign34380_e57213)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34380_e57217;
        locals.var_t0_dn0 = assign34380_e57217_d_n0;
        locals.var_t0_dn2 = assign34380_e57217_d_n2;
        locals.var_t0_dn3 = assign34380_e57217_d_n3;
        locals.var_t0_dn4 = assign34380_e57217_d_n4;
        locals.var_t0_dn5 = assign34380_e57217_d_n5;
        locals.var_t0_dn6 = assign34380_e57217_d_n6;
        locals.var_t0_dn7 = assign34380_e57217_d_n7;
        locals.var_t0_dn8 = assign34380_e57217_d_n8;
        locals.var_t0_dn9 = assign34380_e57217_d_n9;
        locals.var_t0_dn10 = assign34380_e57217_d_n10;
        locals.var_t0_dn11 = assign34380_e57217_d_n11;
        locals.var_t0_dn13 = assign34380_e57217_d_n13;
        locals.var_t0_dn14 = assign34380_e57217_d_n14;

        let (assign34390_e57233, assign34390_e57233_d_n0, assign34390_e57233_d_n2, assign34390_e57233_d_n3, assign34390_e57233_d_n4, assign34390_e57233_d_n5, assign34390_e57233_d_n6, assign34390_e57233_d_n7, assign34390_e57233_d_n8, assign34390_e57233_d_n9, assign34390_e57233_d_n10, assign34390_e57233_d_n11, assign34390_e57233_d_n13, assign34390_e57233_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34390_e57223: f64 = (10.0 * p.p1907);
        let assign34390_e57225: f64 = (assign34390_e57223 * locals.var_t0);
        let assign34390_e57228: f64 = (10.0 * p.p1907);
        let assign34390_e57230: f64 = (assign34390_e57228 + locals.var_t0);
        let assign34390_e57231: f64 = (assign34390_e57225 / assign34390_e57230);
        (assign34390_e57231, ((((assign34390_e57223 * locals.var_t0_dn0) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn0)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn2) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn2)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn3) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn3)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn4) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn4)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn5) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn5)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn6) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn6)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn7) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn7)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn8) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn8)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn9) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn9)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn10) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn10)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn11) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn11)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn13) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn13)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn14) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn14)) / (assign34390_e57230 * assign34390_e57230)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34390_e57233;
        locals.var_t1_dn0 = assign34390_e57233_d_n0;
        locals.var_t1_dn2 = assign34390_e57233_d_n2;
        locals.var_t1_dn3 = assign34390_e57233_d_n3;
        locals.var_t1_dn4 = assign34390_e57233_d_n4;
        locals.var_t1_dn5 = assign34390_e57233_d_n5;
        locals.var_t1_dn6 = assign34390_e57233_d_n6;
        locals.var_t1_dn7 = assign34390_e57233_d_n7;
        locals.var_t1_dn8 = assign34390_e57233_d_n8;
        locals.var_t1_dn9 = assign34390_e57233_d_n9;
        locals.var_t1_dn10 = assign34390_e57233_d_n10;
        locals.var_t1_dn11 = assign34390_e57233_d_n11;
        locals.var_t1_dn13 = assign34390_e57233_d_n13;
        locals.var_t1_dn14 = assign34390_e57233_d_n14;

        let (assign34400_e57245, assign34400_e57245_d_n0, assign34400_e57245_d_n2, assign34400_e57245_d_n3, assign34400_e57245_d_n4, assign34400_e57245_d_n5, assign34400_e57245_d_n6, assign34400_e57245_d_n7, assign34400_e57245_d_n8, assign34400_e57245_d_n9, assign34400_e57245_d_n10, assign34400_e57245_d_n11, assign34400_e57245_d_n13, assign34400_e57245_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34400_e57241: f64 = (p.p1905 * locals.var_t1);
        let assign34400_e57242: f64 = (1.0 + assign34400_e57241);
        let assign34400_e57243: f64 = (locals.var_vsatrsd_t * assign34400_e57242);
        (assign34400_e57243, (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn0)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn2)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn3)), ((locals.var_vsatrsd_t_dn4 * assign34400_e57242) + (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn4))), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn5)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn6)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn7)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn8)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn9)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn10)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn11)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn13)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn14)),)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34400_e57245;
        locals.var_vsatrsd_eff_dn0 = assign34400_e57245_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34400_e57245_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34400_e57245_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34400_e57245_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34400_e57245_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34400_e57245_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34400_e57245_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34400_e57245_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34400_e57245_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34400_e57245_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34400_e57245_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34400_e57245_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34400_e57245_d_n14;

        let (assign34410_e57286, assign34410_e57286_d_n0, assign34410_e57286_d_n2, assign34410_e57286_d_n3, assign34410_e57286_d_n4, assign34410_e57286_d_n5, assign34410_e57286_d_n6, assign34410_e57286_d_n7, assign34410_e57286_d_n8, assign34410_e57286_d_n9, assign34410_e57286_d_n10, assign34410_e57286_d_n11, assign34410_e57286_d_n13, assign34410_e57286_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34410_e57251: f64 = (-10000.0);
        let assign34410_e57253: f64 = (assign34410_e57251 * 10.0);
        let (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,) = {
            if (!(locals.var_vsatrsd_eff < assign34410_e57253)) {
                let assign34410_e57260: f64 = (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff);
                let assign34410_e57263: f64 = (4.0 * 10.0);
                let assign34410_e57265: f64 = (assign34410_e57263 * 10.0);
                let assign34410_e57266: f64 = (assign34410_e57260 + assign34410_e57265);
                let assign34410_e57267: f64 = (assign34410_e57266).sqrt();
                let assign34410_e57268: f64 = (locals.var_vsatrsd_eff + assign34410_e57267);
                let assign34410_e57269: f64 = (0.5 * assign34410_e57268);
                (assign34410_e57269, (0.5 * (locals.var_vsatrsd_eff_dn0 + (((locals.var_vsatrsd_eff_dn0 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn0)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn2 + (((locals.var_vsatrsd_eff_dn2 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn2)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn3 + (((locals.var_vsatrsd_eff_dn3 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn3)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn4 + (((locals.var_vsatrsd_eff_dn4 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn4)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn5 + (((locals.var_vsatrsd_eff_dn5 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn5)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn6 + (((locals.var_vsatrsd_eff_dn6 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn6)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn7 + (((locals.var_vsatrsd_eff_dn7 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn7)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn8 + (((locals.var_vsatrsd_eff_dn8 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn8)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn9 + (((locals.var_vsatrsd_eff_dn9 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn9)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn10 + (((locals.var_vsatrsd_eff_dn10 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn10)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn11 + (((locals.var_vsatrsd_eff_dn11 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn11)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn13 + (((locals.var_vsatrsd_eff_dn13 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn13)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn14 + (((locals.var_vsatrsd_eff_dn14 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn14)) / (2.0 * assign34410_e57267)))),)
            } else {
                let assign34410_e57272: f64 = (-10000.0);
                let assign34410_e57274: f64 = (assign34410_e57272 * 10.0);
                let (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,) = {
                    if (locals.var_vsatrsd_eff < assign34410_e57274) {
                        let assign34410_e57277: f64 = (-10.0);
                        let assign34410_e57279: f64 = (assign34410_e57277 * 10.0);
                        let assign34410_e57281: f64 = (assign34410_e57279 / locals.var_vsatrsd_eff);
                        (assign34410_e57281, (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn0) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn2) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn3) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn4) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn5) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn6) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn7) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn8) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn9) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn10) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn11) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn13) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn14) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,)
            }
        };
        (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34410_e57286;
        locals.var_vsatrsd_eff_dn0 = assign34410_e57286_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34410_e57286_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34410_e57286_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34410_e57286_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34410_e57286_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34410_e57286_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34410_e57286_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34410_e57286_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34410_e57286_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34410_e57286_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34410_e57286_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34410_e57286_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34410_e57286_d_n14;

        let (assign34420_e57298, assign34420_e57298_d_n0, assign34420_e57298_d_n2, assign34420_e57298_d_n3, assign34420_e57298_d_n4, assign34420_e57298_d_n5, assign34420_e57298_d_n6, assign34420_e57298_d_n7, assign34420_e57298_d_n8, assign34420_e57298_d_n9, assign34420_e57298_d_n10, assign34420_e57298_d_n11, assign34420_e57298_d_n13, assign34420_e57298_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34420_e57292: f64 = (locals.var_nfintotal * locals.var_weff0);
        let assign34420_e57294: f64 = (assign34420_e57292 * 1.60219e-19);
        let assign34420_e57296: f64 = (assign34420_e57294 * locals.var_vsatrsd_eff);
        (assign34420_e57296, (assign34420_e57294 * locals.var_vsatrsd_eff_dn0), (assign34420_e57294 * locals.var_vsatrsd_eff_dn2), (assign34420_e57294 * locals.var_vsatrsd_eff_dn3), (assign34420_e57294 * locals.var_vsatrsd_eff_dn4), (assign34420_e57294 * locals.var_vsatrsd_eff_dn5), (assign34420_e57294 * locals.var_vsatrsd_eff_dn6), (assign34420_e57294 * locals.var_vsatrsd_eff_dn7), (assign34420_e57294 * locals.var_vsatrsd_eff_dn8), (assign34420_e57294 * locals.var_vsatrsd_eff_dn9), (assign34420_e57294 * locals.var_vsatrsd_eff_dn10), (assign34420_e57294 * locals.var_vsatrsd_eff_dn11), (assign34420_e57294 * locals.var_vsatrsd_eff_dn13), (assign34420_e57294 * locals.var_vsatrsd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34420_e57298;
        locals.var_t2_dn0 = assign34420_e57298_d_n0;
        locals.var_t2_dn2 = assign34420_e57298_d_n2;
        locals.var_t2_dn3 = assign34420_e57298_d_n3;
        locals.var_t2_dn4 = assign34420_e57298_d_n4;
        locals.var_t2_dn5 = assign34420_e57298_d_n5;
        locals.var_t2_dn6 = assign34420_e57298_d_n6;
        locals.var_t2_dn7 = assign34420_e57298_d_n7;
        locals.var_t2_dn8 = assign34420_e57298_d_n8;
        locals.var_t2_dn9 = assign34420_e57298_d_n9;
        locals.var_t2_dn10 = assign34420_e57298_d_n10;
        locals.var_t2_dn11 = assign34420_e57298_d_n11;
        locals.var_t2_dn13 = assign34420_e57298_d_n13;
        locals.var_t2_dn14 = assign34420_e57298_d_n14;

        let (assign34430_e57305, assign34430_e57305_d_n0, assign34430_e57305_d_n2, assign34430_e57305_d_n3, assign34430_e57305_d_n4, assign34430_e57305_d_n5, assign34430_e57305_d_n6, assign34430_e57305_d_n7, assign34430_e57305_d_n8, assign34430_e57305_d_n9, assign34430_e57305_d_n10, assign34430_e57305_d_n11, assign34430_e57305_d_n13, assign34430_e57305_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34430_e57303: f64 = ((nv9 - nv7)).abs();
        (assign34430_e57303, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv9 - nv7) >= 0.0 { -1.0 } else { 1.0 }, 0.0, if (nv9 - nv7) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34430_e57305;
        locals.var_t5_dn0 = assign34430_e57305_d_n0;
        locals.var_t5_dn2 = assign34430_e57305_d_n2;
        locals.var_t5_dn3 = assign34430_e57305_d_n3;
        locals.var_t5_dn4 = assign34430_e57305_d_n4;
        locals.var_t5_dn5 = assign34430_e57305_d_n5;
        locals.var_t5_dn6 = assign34430_e57305_d_n6;
        locals.var_t5_dn7 = assign34430_e57305_d_n7;
        locals.var_t5_dn8 = assign34430_e57305_d_n8;
        locals.var_t5_dn9 = assign34430_e57305_d_n9;
        locals.var_t5_dn10 = assign34430_e57305_d_n10;
        locals.var_t5_dn11 = assign34430_e57305_d_n11;
        locals.var_t5_dn13 = assign34430_e57305_d_n13;
        locals.var_t5_dn14 = assign34430_e57305_d_n14;

        let assign34440_e57308: f64 = if p.p1917 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign34440_e57308;

        let (assign34450_e57316, assign34450_e57316_d_n0, assign34450_e57316_d_n2, assign34450_e57316_d_n3, assign34450_e57316_d_n4, assign34450_e57316_d_n5, assign34450_e57316_d_n6, assign34450_e57316_d_n7, assign34450_e57316_d_n8, assign34450_e57316_d_n9, assign34450_e57316_d_n10, assign34450_e57316_d_n11, assign34450_e57316_d_n13, assign34450_e57316_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34450_e57316;
        locals.var_t3_dn0 = assign34450_e57316_d_n0;
        locals.var_t3_dn2 = assign34450_e57316_d_n2;
        locals.var_t3_dn3 = assign34450_e57316_d_n3;
        locals.var_t3_dn4 = assign34450_e57316_d_n4;
        locals.var_t3_dn5 = assign34450_e57316_d_n5;
        locals.var_t3_dn6 = assign34450_e57316_d_n6;
        locals.var_t3_dn7 = assign34450_e57316_d_n7;
        locals.var_t3_dn8 = assign34450_e57316_d_n8;
        locals.var_t3_dn9 = assign34450_e57316_d_n9;
        locals.var_t3_dn10 = assign34450_e57316_d_n10;
        locals.var_t3_dn11 = assign34450_e57316_d_n11;
        locals.var_t3_dn13 = assign34450_e57316_d_n13;
        locals.var_t3_dn14 = assign34450_e57316_d_n14;

        let (assign34460_e57350, assign34460_e57350_d_n0, assign34460_e57350_d_n2, assign34460_e57350_d_n3, assign34460_e57350_d_n4, assign34460_e57350_d_n5, assign34460_e57350_d_n6, assign34460_e57350_d_n7, assign34460_e57350_d_n8, assign34460_e57350_d_n9, assign34460_e57350_d_n10, assign34460_e57350_d_n11, assign34460_e57350_d_n13, assign34460_e57350_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34460_e57326: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57328: f64 = assign34460_e57326;
        let assign34460_e57331: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57333: f64 = assign34460_e57331;
        let assign34460_e57336: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57338: f64 = assign34460_e57336;
        let assign34460_e57339: f64 = (assign34460_e57333 * assign34460_e57338);
        let assign34460_e57342: f64 = (0.25 * 0.5);
        let assign34460_e57344: f64 = (assign34460_e57342 * 0.5);
        let assign34460_e57345: f64 = (assign34460_e57339 + assign34460_e57344);
        let assign34460_e57346: f64 = (assign34460_e57345).sqrt();
        let assign34460_e57347: f64 = (assign34460_e57328 + assign34460_e57346);
        let assign34460_e57348: f64 = (0.5 * assign34460_e57347);
        (assign34460_e57348, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn0)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn2)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn3)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn4)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn5)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn6)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn7)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn8)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn9)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn10)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn11)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn13)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn14)) / (2.0 * assign34460_e57346)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34460_e57350;
        locals.var_t3_dn0 = assign34460_e57350_d_n0;
        locals.var_t3_dn2 = assign34460_e57350_d_n2;
        locals.var_t3_dn3 = assign34460_e57350_d_n3;
        locals.var_t3_dn4 = assign34460_e57350_d_n4;
        locals.var_t3_dn5 = assign34460_e57350_d_n5;
        locals.var_t3_dn6 = assign34460_e57350_d_n6;
        locals.var_t3_dn7 = assign34460_e57350_d_n7;
        locals.var_t3_dn8 = assign34460_e57350_d_n8;
        locals.var_t3_dn9 = assign34460_e57350_d_n9;
        locals.var_t3_dn10 = assign34460_e57350_d_n10;
        locals.var_t3_dn11 = assign34460_e57350_d_n11;
        locals.var_t3_dn13 = assign34460_e57350_d_n13;
        locals.var_t3_dn14 = assign34460_e57350_d_n14;

        let (assign34470_e57363, assign34470_e57363_d_n0, assign34470_e57363_d_n2, assign34470_e57363_d_n3, assign34470_e57363_d_n4, assign34470_e57363_d_n5, assign34470_e57363_d_n6, assign34470_e57363_d_n7, assign34470_e57363_d_n8, assign34470_e57363_d_n9, assign34470_e57363_d_n10, assign34470_e57363_d_n11, assign34470_e57363_d_n13, assign34470_e57363_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34470_e57360: f64 = (locals.var_t3 * p.p1917);
        let assign34470_e57361: f64 = (1.0 + assign34470_e57360);
        (assign34470_e57361, (locals.var_t3_dn0 * p.p1917), (locals.var_t3_dn2 * p.p1917), (locals.var_t3_dn3 * p.p1917), (locals.var_t3_dn4 * p.p1917), (locals.var_t3_dn5 * p.p1917), (locals.var_t3_dn6 * p.p1917), (locals.var_t3_dn7 * p.p1917), (locals.var_t3_dn8 * p.p1917), (locals.var_t3_dn9 * p.p1917), (locals.var_t3_dn10 * p.p1917), (locals.var_t3_dn11 * p.p1917), (locals.var_t3_dn13 * p.p1917), (locals.var_t3_dn14 * p.p1917),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34470_e57363;
        locals.var_t3_dn0 = assign34470_e57363_d_n0;
        locals.var_t3_dn2 = assign34470_e57363_d_n2;
        locals.var_t3_dn3 = assign34470_e57363_d_n3;
        locals.var_t3_dn4 = assign34470_e57363_d_n4;
        locals.var_t3_dn5 = assign34470_e57363_d_n5;
        locals.var_t3_dn6 = assign34470_e57363_d_n6;
        locals.var_t3_dn7 = assign34470_e57363_d_n7;
        locals.var_t3_dn8 = assign34470_e57363_d_n8;
        locals.var_t3_dn9 = assign34470_e57363_d_n9;
        locals.var_t3_dn10 = assign34470_e57363_d_n10;
        locals.var_t3_dn11 = assign34470_e57363_d_n11;
        locals.var_t3_dn13 = assign34470_e57363_d_n13;
        locals.var_t3_dn14 = assign34470_e57363_d_n14;

    }
}
