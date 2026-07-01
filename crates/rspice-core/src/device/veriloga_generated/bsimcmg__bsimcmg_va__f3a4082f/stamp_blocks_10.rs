#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12000_e15782, assign12000_e15782_d_n0, assign12000_e15782_d_n2, assign12000_e15782_d_n3, assign12000_e15782_d_n4, assign12000_e15782_d_n5, assign12000_e15782_d_n6, assign12000_e15782_d_n7, assign12000_e15782_d_n8, assign12000_e15782_d_n9, assign12000_e15782_d_n10, assign12000_e15782_d_n11, assign12000_e15782_d_n13, assign12000_e15782_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign12000_e15748: f64 = (2.0 * locals.var_epssp);
        let assign12000_e15750: f64 = (2.0_f64).sqrt();
        let assign12000_e15751: f64 = (assign12000_e15748 * assign12000_e15750);
        let assign12000_e15753: f64 = (assign12000_e15751 / 3.141592653589793);
        let assign12000_e15755: f64 = (assign12000_e15753 * 0.7);
        let assign12000_e15757: f64 = (assign12000_e15755 * locals.var_dcf);
        let assign12000_e15759: f64 = (assign12000_e15757 / locals.var_tt0);
        let assign12000_e15762: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign12000_e15779, assign12000_e15779_d_n0, assign12000_e15779_d_n2, assign12000_e15779_d_n3, assign12000_e15779_d_n4, assign12000_e15779_d_n5, assign12000_e15779_d_n6, assign12000_e15779_d_n7, assign12000_e15779_d_n8, assign12000_e15779_d_n9, assign12000_e15779_d_n10, assign12000_e15779_d_n11, assign12000_e15779_d_n13, assign12000_e15779_d_n14,) = {
            if (!(assign12000_e15762 > 1e-38)) {
                let assign12000_e15767: f64 = (-87.498233534);
                (assign12000_e15767, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign12000_e15770: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign12000_e15778, assign12000_e15778_d_n0, assign12000_e15778_d_n2, assign12000_e15778_d_n3, assign12000_e15778_d_n4, assign12000_e15778_d_n5, assign12000_e15778_d_n6, assign12000_e15778_d_n7, assign12000_e15778_d_n8, assign12000_e15778_d_n9, assign12000_e15778_d_n10, assign12000_e15778_d_n11, assign12000_e15778_d_n13, assign12000_e15778_d_n14,) = {
                    if (assign12000_e15770 > 1e-38) {
                        let assign12000_e15775: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign12000_e15776: f64 = (assign12000_e15775).ln();
                        (assign12000_e15776, ((locals.var_tt1_dn0 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn2 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn3 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn4 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn5 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn6 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn7 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn8 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn9 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn10 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn11 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn13 / locals.var_tt2) / assign12000_e15775), ((locals.var_tt1_dn14 / locals.var_tt2) / assign12000_e15775),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12000_e15778, assign12000_e15778_d_n0, assign12000_e15778_d_n2, assign12000_e15778_d_n3, assign12000_e15778_d_n4, assign12000_e15778_d_n5, assign12000_e15778_d_n6, assign12000_e15778_d_n7, assign12000_e15778_d_n8, assign12000_e15778_d_n9, assign12000_e15778_d_n10, assign12000_e15778_d_n11, assign12000_e15778_d_n13, assign12000_e15778_d_n14,)
            }
        };
        let assign12000_e15780: f64 = (assign12000_e15759 * assign12000_e15779);
        (assign12000_e15780, (assign12000_e15759 * assign12000_e15779_d_n0), (assign12000_e15759 * assign12000_e15779_d_n2), (assign12000_e15759 * assign12000_e15779_d_n3), (assign12000_e15759 * assign12000_e15779_d_n4), (assign12000_e15759 * assign12000_e15779_d_n5), (assign12000_e15759 * assign12000_e15779_d_n6), (assign12000_e15759 * assign12000_e15779_d_n7), (assign12000_e15759 * assign12000_e15779_d_n8), (assign12000_e15759 * assign12000_e15779_d_n9), (assign12000_e15759 * assign12000_e15779_d_n10), (assign12000_e15759 * assign12000_e15779_d_n11), (assign12000_e15759 * assign12000_e15779_d_n13), (assign12000_e15759 * assign12000_e15779_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign12000_e15782;
        locals.var_cfgsat_dn0 = assign12000_e15782_d_n0;
        locals.var_cfgsat_dn2 = assign12000_e15782_d_n2;
        locals.var_cfgsat_dn3 = assign12000_e15782_d_n3;
        locals.var_cfgsat_dn4 = assign12000_e15782_d_n4;
        locals.var_cfgsat_dn5 = assign12000_e15782_d_n5;
        locals.var_cfgsat_dn6 = assign12000_e15782_d_n6;
        locals.var_cfgsat_dn7 = assign12000_e15782_d_n7;
        locals.var_cfgsat_dn8 = assign12000_e15782_d_n8;
        locals.var_cfgsat_dn9 = assign12000_e15782_d_n9;
        locals.var_cfgsat_dn10 = assign12000_e15782_d_n10;
        locals.var_cfgsat_dn11 = assign12000_e15782_d_n11;
        locals.var_cfgsat_dn13 = assign12000_e15782_d_n13;
        locals.var_cfgsat_dn14 = assign12000_e15782_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign12010_e15788, assign12010_e15788_d_n0, assign12010_e15788_d_n2, assign12010_e15788_d_n3, assign12010_e15788_d_n4, assign12010_e15788_d_n5, assign12010_e15788_d_n6, assign12010_e15788_d_n7, assign12010_e15788_d_n8, assign12010_e15788_d_n9, assign12010_e15788_d_n10, assign12010_e15788_d_n11, assign12010_e15788_d_n13, assign12010_e15788_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign12010_e15788;
        locals.var_delta_dn0 = assign12010_e15788_d_n0;
        locals.var_delta_dn2 = assign12010_e15788_d_n2;
        locals.var_delta_dn3 = assign12010_e15788_d_n3;
        locals.var_delta_dn4 = assign12010_e15788_d_n4;
        locals.var_delta_dn5 = assign12010_e15788_d_n5;
        locals.var_delta_dn6 = assign12010_e15788_d_n6;
        locals.var_delta_dn7 = assign12010_e15788_d_n7;
        locals.var_delta_dn8 = assign12010_e15788_d_n8;
        locals.var_delta_dn9 = assign12010_e15788_d_n9;
        locals.var_delta_dn10 = assign12010_e15788_d_n10;
        locals.var_delta_dn11 = assign12010_e15788_d_n11;
        locals.var_delta_dn13 = assign12010_e15788_d_n13;
        locals.var_delta_dn14 = assign12010_e15788_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign12020_e15798, assign12020_e15798_d_n0, assign12020_e15798_d_n2, assign12020_e15798_d_n3, assign12020_e15798_d_n4, assign12020_e15798_d_n5, assign12020_e15798_d_n6, assign12020_e15798_d_n7, assign12020_e15798_d_n8, assign12020_e15798_d_n9, assign12020_e15798_d_n10, assign12020_e15798_d_n11, assign12020_e15798_d_n13, assign12020_e15798_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign12020_e15794: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign12020_e15796: f64 = (assign12020_e15794 - locals.var_delta);
        (assign12020_e15796, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12020_e15798;
        locals.var_tt1_dn0 = assign12020_e15798_d_n0;
        locals.var_tt1_dn2 = assign12020_e15798_d_n2;
        locals.var_tt1_dn3 = assign12020_e15798_d_n3;
        locals.var_tt1_dn4 = assign12020_e15798_d_n4;
        locals.var_tt1_dn5 = assign12020_e15798_d_n5;
        locals.var_tt1_dn6 = assign12020_e15798_d_n6;
        locals.var_tt1_dn7 = assign12020_e15798_d_n7;
        locals.var_tt1_dn8 = assign12020_e15798_d_n8;
        locals.var_tt1_dn9 = assign12020_e15798_d_n9;
        locals.var_tt1_dn10 = assign12020_e15798_d_n10;
        locals.var_tt1_dn11 = assign12020_e15798_d_n11;
        locals.var_tt1_dn13 = assign12020_e15798_d_n13;
        locals.var_tt1_dn14 = assign12020_e15798_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12030_e15821, assign12030_e15821_d_n0, assign12030_e15821_d_n2, assign12030_e15821_d_n3, assign12030_e15821_d_n4, assign12030_e15821_d_n5, assign12030_e15821_d_n6, assign12030_e15821_d_n7, assign12030_e15821_d_n8, assign12030_e15821_d_n9, assign12030_e15821_d_n10, assign12030_e15821_d_n11, assign12030_e15821_d_n13, assign12030_e15821_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign12030_e15808: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign12030_e15811: f64 = (4.0 * locals.var_delta);
        let assign12030_e15813: f64 = (assign12030_e15811 * locals.var_cfgsat);
        let assign12030_e15814: f64 = (assign12030_e15808 + assign12030_e15813);
        let assign12030_e15815: f64 = (assign12030_e15814).sqrt();
        let assign12030_e15816: f64 = (locals.var_tt1 + assign12030_e15815);
        let assign12030_e15817: f64 = (0.5 * assign12030_e15816);
        let assign12030_e15818: f64 = (locals.var_cfgsat - assign12030_e15817);
        let assign12030_e15819: f64 = (p.p40 * assign12030_e15818);
        (assign12030_e15819, (p.p40 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn0))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn2))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn3))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn4))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn5))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn6))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn7))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn8))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn9))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn10))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn11))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn13))) / (2.0 * assign12030_e15815)))))), (p.p40 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign12030_e15811 * locals.var_cfgsat_dn14))) / (2.0 * assign12030_e15815)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign12030_e15821;
        locals.var_cfg_dn0 = assign12030_e15821_d_n0;
        locals.var_cfg_dn2 = assign12030_e15821_d_n2;
        locals.var_cfg_dn3 = assign12030_e15821_d_n3;
        locals.var_cfg_dn4 = assign12030_e15821_d_n4;
        locals.var_cfg_dn5 = assign12030_e15821_d_n5;
        locals.var_cfg_dn6 = assign12030_e15821_d_n6;
        locals.var_cfg_dn7 = assign12030_e15821_d_n7;
        locals.var_cfg_dn8 = assign12030_e15821_d_n8;
        locals.var_cfg_dn9 = assign12030_e15821_d_n9;
        locals.var_cfg_dn10 = assign12030_e15821_d_n10;
        locals.var_cfg_dn11 = assign12030_e15821_d_n11;
        locals.var_cfg_dn13 = assign12030_e15821_d_n13;
        locals.var_cfg_dn14 = assign12030_e15821_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign12040_e15829, assign12040_e15829_d_n0, assign12040_e15829_d_n2, assign12040_e15829_d_n3, assign12040_e15829_d_n4, assign12040_e15829_d_n5, assign12040_e15829_d_n6, assign12040_e15829_d_n7, assign12040_e15829_d_n8, assign12040_e15829_d_n9, assign12040_e15829_d_n10, assign12040_e15829_d_n11, assign12040_e15829_d_n13, assign12040_e15829_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 != 0.0)) {
        let assign12040_e15827: f64 = (locals.var_ccg + locals.var_cfg);
        (assign12040_e15827, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_sidetopm, locals.var_cgg_sidetopm_dn0, locals.var_cgg_sidetopm_dn2, locals.var_cgg_sidetopm_dn3, locals.var_cgg_sidetopm_dn4, locals.var_cgg_sidetopm_dn5, locals.var_cgg_sidetopm_dn6, locals.var_cgg_sidetopm_dn7, locals.var_cgg_sidetopm_dn8, locals.var_cgg_sidetopm_dn9, locals.var_cgg_sidetopm_dn10, locals.var_cgg_sidetopm_dn11, locals.var_cgg_sidetopm_dn13, locals.var_cgg_sidetopm_dn14,)
    }
};
        locals.var_cgg_sidetopm = assign12040_e15829;
        locals.var_cgg_sidetopm_dn0 = assign12040_e15829_d_n0;
        locals.var_cgg_sidetopm_dn2 = assign12040_e15829_d_n2;
        locals.var_cgg_sidetopm_dn3 = assign12040_e15829_d_n3;
        locals.var_cgg_sidetopm_dn4 = assign12040_e15829_d_n4;
        locals.var_cgg_sidetopm_dn5 = assign12040_e15829_d_n5;
        locals.var_cgg_sidetopm_dn6 = assign12040_e15829_d_n6;
        locals.var_cgg_sidetopm_dn7 = assign12040_e15829_d_n7;
        locals.var_cgg_sidetopm_dn8 = assign12040_e15829_d_n8;
        locals.var_cgg_sidetopm_dn9 = assign12040_e15829_d_n9;
        locals.var_cgg_sidetopm_dn10 = assign12040_e15829_d_n10;
        locals.var_cgg_sidetopm_dn11 = assign12040_e15829_d_n11;
        locals.var_cgg_sidetopm_dn13 = assign12040_e15829_d_n13;
        locals.var_cgg_sidetopm_dn14 = assign12040_e15829_d_n14;
        locals.var_cgg_sidetopm_rv = 0.0;

        let (assign12050_e15844,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12050_e15838: f64 = (locals.var_wg + p.p90);
        let assign12050_e15839: f64 = (0.2 * assign12050_e15838);
        let assign12050_e15841: f64 = (assign12050_e15839 / locals.var_trsd);
        let assign12050_e15842: f64 = (2.3 + assign12050_e15841);
        (assign12050_e15842,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign12050_e15844;
        locals.var_hr_rv = 0.0;

        let (assign12060_e15851,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign12060_e15851;
        locals.var_lr_rv = 0.0;

        let (assign12070_e15863,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12070_e15858: f64 = (locals.var_wg + p.p90);
        let assign12070_e15860: f64 = (assign12070_e15858 - locals.var_trsd);
        let assign12070_e15861: f64 = (assign12070_e15860).abs();
        (assign12070_e15861,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign12070_e15863;
        locals.var_hgdelta_rv = 0.0;

        let (assign12080_e15872,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12080_e15870: f64 = (p.p1087 * locals.var_lr);
        (assign12080_e15870,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign12080_e15872;
        locals.var_lmax_rv = 0.0;

        let (assign12090_e15883,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12090_e15880: f64 = (locals.var_wg + p.p90);
        let assign12090_e15881: f64 = (locals.var_trsd).min(assign12090_e15880);
        (assign12090_e15881,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign12090_e15883;
        locals.var_y_rv = 0.0;

        let (assign12100_e15894,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12100_e15891: f64 = (locals.var_hr + 1.0);
        let assign12100_e15892: f64 = (p.p1087 / assign12100_e15891);
        (assign12100_e15892,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12100_e15894;
        locals.var_x_rv = 0.0;

        let (assign12110_e15901,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign12110_e15901;
        locals.var_cnon_rv = 0.0;

        let (assign12120_e15914,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12120_e15909: f64 = (locals.var_y - locals.var_x);
        let assign12120_e15910: f64 = (locals.var_epssp * assign12120_e15909);
        let assign12120_e15912: f64 = (assign12120_e15910 / p.p1087);
        (assign12120_e15912,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign12120_e15914;
        locals.var_ccgsat_rv = 0.0;

        let (assign12130_e15923, assign12130_e15923_d_n0, assign12130_e15923_d_n2, assign12130_e15923_d_n3, assign12130_e15923_d_n4, assign12130_e15923_d_n5, assign12130_e15923_d_n6, assign12130_e15923_d_n7, assign12130_e15923_d_n8, assign12130_e15923_d_n9, assign12130_e15923_d_n10, assign12130_e15923_d_n11, assign12130_e15923_d_n13, assign12130_e15923_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12130_e15921: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign12130_e15921, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12130_e15923;
        locals.var_tt1_dn0 = assign12130_e15923_d_n0;
        locals.var_tt1_dn2 = assign12130_e15923_d_n2;
        locals.var_tt1_dn3 = assign12130_e15923_d_n3;
        locals.var_tt1_dn4 = assign12130_e15923_d_n4;
        locals.var_tt1_dn5 = assign12130_e15923_d_n5;
        locals.var_tt1_dn6 = assign12130_e15923_d_n6;
        locals.var_tt1_dn7 = assign12130_e15923_d_n7;
        locals.var_tt1_dn8 = assign12130_e15923_d_n8;
        locals.var_tt1_dn9 = assign12130_e15923_d_n9;
        locals.var_tt1_dn10 = assign12130_e15923_d_n10;
        locals.var_tt1_dn11 = assign12130_e15923_d_n11;
        locals.var_tt1_dn13 = assign12130_e15923_d_n13;
        locals.var_tt1_dn14 = assign12130_e15923_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign12140_e15926: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard219 = assign12140_e15926;
        locals.var_guard219_rv = 0.0;

        let (assign12150_e15935, assign12150_e15935_d_n0, assign12150_e15935_d_n2, assign12150_e15935_d_n3, assign12150_e15935_d_n4, assign12150_e15935_d_n5, assign12150_e15935_d_n6, assign12150_e15935_d_n7, assign12150_e15935_d_n8, assign12150_e15935_d_n9, assign12150_e15935_d_n10, assign12150_e15935_d_n11, assign12150_e15935_d_n13, assign12150_e15935_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard219 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12150_e15935;
        locals.var_ccg1_dn0 = assign12150_e15935_d_n0;
        locals.var_ccg1_dn2 = assign12150_e15935_d_n2;
        locals.var_ccg1_dn3 = assign12150_e15935_d_n3;
        locals.var_ccg1_dn4 = assign12150_e15935_d_n4;
        locals.var_ccg1_dn5 = assign12150_e15935_d_n5;
        locals.var_ccg1_dn6 = assign12150_e15935_d_n6;
        locals.var_ccg1_dn7 = assign12150_e15935_d_n7;
        locals.var_ccg1_dn8 = assign12150_e15935_d_n8;
        locals.var_ccg1_dn9 = assign12150_e15935_d_n9;
        locals.var_ccg1_dn10 = assign12150_e15935_d_n10;
        locals.var_ccg1_dn11 = assign12150_e15935_d_n11;
        locals.var_ccg1_dn13 = assign12150_e15935_d_n13;
        locals.var_ccg1_dn14 = assign12150_e15935_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12160_e15982, assign12160_e15982_d_n0, assign12160_e15982_d_n2, assign12160_e15982_d_n3, assign12160_e15982_d_n4, assign12160_e15982_d_n5, assign12160_e15982_d_n6, assign12160_e15982_d_n7, assign12160_e15982_d_n8, assign12160_e15982_d_n9, assign12160_e15982_d_n10, assign12160_e15982_d_n11, assign12160_e15982_d_n13, assign12160_e15982_d_n14,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) && (locals.var_guard219 == 0.0)) {
        let assign12160_e15945: f64 = (1.0 / locals.var_cnon);
        let assign12160_e15952: f64 = (-37.0);
        let (assign12160_e15979, assign12160_e15979_d_n0, assign12160_e15979_d_n2, assign12160_e15979_d_n3, assign12160_e15979_d_n4, assign12160_e15979_d_n5, assign12160_e15979_d_n6, assign12160_e15979_d_n7, assign12160_e15979_d_n8, assign12160_e15979_d_n9, assign12160_e15979_d_n10, assign12160_e15979_d_n11, assign12160_e15979_d_n13, assign12160_e15979_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign12160_e15952))) {
                let assign12160_e15958: f64 = (locals.var_tt1).exp();
                let assign12160_e15959: f64 = (1.0 + assign12160_e15958);
                let assign12160_e15960: f64 = (assign12160_e15959).ln();
                (assign12160_e15960, ((assign12160_e15958 * locals.var_tt1_dn0) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn2) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn3) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn4) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn5) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn6) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn7) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn8) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn9) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn10) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn11) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn13) / assign12160_e15959), ((assign12160_e15958 * locals.var_tt1_dn14) / assign12160_e15959),)
            } else {
                let assign12160_e15967: f64 = (-37.0);
                let (assign12160_e15978, assign12160_e15978_d_n0, assign12160_e15978_d_n2, assign12160_e15978_d_n3, assign12160_e15978_d_n4, assign12160_e15978_d_n5, assign12160_e15978_d_n6, assign12160_e15978_d_n7, assign12160_e15978_d_n8, assign12160_e15978_d_n9, assign12160_e15978_d_n10, assign12160_e15978_d_n11, assign12160_e15978_d_n13, assign12160_e15978_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign12160_e15967)) {
                        let assign12160_e15971: f64 = (locals.var_tt1).exp();
                        (assign12160_e15971, (assign12160_e15971 * locals.var_tt1_dn0), (assign12160_e15971 * locals.var_tt1_dn2), (assign12160_e15971 * locals.var_tt1_dn3), (assign12160_e15971 * locals.var_tt1_dn4), (assign12160_e15971 * locals.var_tt1_dn5), (assign12160_e15971 * locals.var_tt1_dn6), (assign12160_e15971 * locals.var_tt1_dn7), (assign12160_e15971 * locals.var_tt1_dn8), (assign12160_e15971 * locals.var_tt1_dn9), (assign12160_e15971 * locals.var_tt1_dn10), (assign12160_e15971 * locals.var_tt1_dn11), (assign12160_e15971 * locals.var_tt1_dn13), (assign12160_e15971 * locals.var_tt1_dn14),)
                    } else {
                        let (assign12160_e15977, assign12160_e15977_d_n0, assign12160_e15977_d_n2, assign12160_e15977_d_n3, assign12160_e15977_d_n4, assign12160_e15977_d_n5, assign12160_e15977_d_n6, assign12160_e15977_d_n7, assign12160_e15977_d_n8, assign12160_e15977_d_n9, assign12160_e15977_d_n10, assign12160_e15977_d_n11, assign12160_e15977_d_n13, assign12160_e15977_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign12160_e15977, assign12160_e15977_d_n0, assign12160_e15977_d_n2, assign12160_e15977_d_n3, assign12160_e15977_d_n4, assign12160_e15977_d_n5, assign12160_e15977_d_n6, assign12160_e15977_d_n7, assign12160_e15977_d_n8, assign12160_e15977_d_n9, assign12160_e15977_d_n10, assign12160_e15977_d_n11, assign12160_e15977_d_n13, assign12160_e15977_d_n14,)
                    }
                };
                (assign12160_e15978, assign12160_e15978_d_n0, assign12160_e15978_d_n2, assign12160_e15978_d_n3, assign12160_e15978_d_n4, assign12160_e15978_d_n5, assign12160_e15978_d_n6, assign12160_e15978_d_n7, assign12160_e15978_d_n8, assign12160_e15978_d_n9, assign12160_e15978_d_n10, assign12160_e15978_d_n11, assign12160_e15978_d_n13, assign12160_e15978_d_n14,)
            }
        };
        let assign12160_e15980: f64 = (assign12160_e15945 * assign12160_e15979);
        (assign12160_e15980, (assign12160_e15945 * assign12160_e15979_d_n0), (assign12160_e15945 * assign12160_e15979_d_n2), (assign12160_e15945 * assign12160_e15979_d_n3), (assign12160_e15945 * assign12160_e15979_d_n4), (assign12160_e15945 * assign12160_e15979_d_n5), (assign12160_e15945 * assign12160_e15979_d_n6), (assign12160_e15945 * assign12160_e15979_d_n7), (assign12160_e15945 * assign12160_e15979_d_n8), (assign12160_e15945 * assign12160_e15979_d_n9), (assign12160_e15945 * assign12160_e15979_d_n10), (assign12160_e15945 * assign12160_e15979_d_n11), (assign12160_e15945 * assign12160_e15979_d_n13), (assign12160_e15945 * assign12160_e15979_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12160_e15982;
        locals.var_ccg1_dn0 = assign12160_e15982_d_n0;
        locals.var_ccg1_dn2 = assign12160_e15982_d_n2;
        locals.var_ccg1_dn3 = assign12160_e15982_d_n3;
        locals.var_ccg1_dn4 = assign12160_e15982_d_n4;
        locals.var_ccg1_dn5 = assign12160_e15982_d_n5;
        locals.var_ccg1_dn6 = assign12160_e15982_d_n6;
        locals.var_ccg1_dn7 = assign12160_e15982_d_n7;
        locals.var_ccg1_dn8 = assign12160_e15982_d_n8;
        locals.var_ccg1_dn9 = assign12160_e15982_d_n9;
        locals.var_ccg1_dn10 = assign12160_e15982_d_n10;
        locals.var_ccg1_dn11 = assign12160_e15982_d_n11;
        locals.var_ccg1_dn13 = assign12160_e15982_d_n13;
        locals.var_ccg1_dn14 = assign12160_e15982_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12170_e16001,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12170_e15991: f64 = (locals.var_wg + p.p90);
        let assign12170_e15992: f64 = (locals.var_trsd / assign12170_e15991);
        let assign12170_e15995: f64 = (locals.var_wg + p.p90);
        let assign12170_e15997: f64 = (assign12170_e15995 / locals.var_trsd);
        let assign12170_e15998: f64 = (assign12170_e15992).min(assign12170_e15997);
        let assign12170_e15999: f64 = (0.5 * assign12170_e15998);
        (assign12170_e15999,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign12170_e16001;
        locals.var_r1cf_rv = 0.0;

        let (assign12180_e16010,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12180_e16008: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign12180_e16008,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign12180_e16010;
        locals.var_rcf_rv = 0.0;

        let (assign12190_e16060,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12190_e16017: f64 = (locals.var_epssp * 2.0);
        let assign12190_e16019: f64 = (assign12190_e16017 / 3.141592653589793);
        let assign12190_e16023: f64 = (0.5 * 3.141592653589793);
        let assign12190_e16025: f64 = (assign12190_e16023 * locals.var_rcf);
        let assign12190_e16026: f64 = (p.p1087 + assign12190_e16025);
        let assign12190_e16028: f64 = (assign12190_e16026 / p.p1087);
        let (assign12190_e16057,) = {
            if (!(assign12190_e16028 > 1e-38)) {
                let assign12190_e16033: f64 = (-87.498233534);
                (assign12190_e16033,)
            } else {
                let assign12190_e16037: f64 = (0.5 * 3.141592653589793);
                let assign12190_e16039: f64 = (assign12190_e16037 * locals.var_rcf);
                let assign12190_e16040: f64 = (p.p1087 + assign12190_e16039);
                let assign12190_e16042: f64 = (assign12190_e16040 / p.p1087);
                let (assign12190_e16056,) = {
                    if (assign12190_e16042 > 1e-38) {
                        let assign12190_e16048: f64 = (0.5 * 3.141592653589793);
                        let assign12190_e16050: f64 = (assign12190_e16048 * locals.var_rcf);
                        let assign12190_e16051: f64 = (p.p1087 + assign12190_e16050);
                        let assign12190_e16053: f64 = (assign12190_e16051 / p.p1087);
                        let assign12190_e16054: f64 = (assign12190_e16053).ln();
                        (assign12190_e16054,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12190_e16056,)
            }
        };
        let assign12190_e16058: f64 = (assign12190_e16019 * assign12190_e16057);
        (assign12190_e16058,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign12190_e16060;
        locals.var_ccg2_rv = 0.0;

        let (assign12200_e16071, assign12200_e16071_d_n0, assign12200_e16071_d_n2, assign12200_e16071_d_n3, assign12200_e16071_d_n4, assign12200_e16071_d_n5, assign12200_e16071_d_n6, assign12200_e16071_d_n7, assign12200_e16071_d_n8, assign12200_e16071_d_n9, assign12200_e16071_d_n10, assign12200_e16071_d_n11, assign12200_e16071_d_n13, assign12200_e16071_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12200_e16068: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign12200_e16069: f64 = (p.p40 * assign12200_e16068);
        (assign12200_e16069, (p.p40 * locals.var_ccg1_dn0), (p.p40 * locals.var_ccg1_dn2), (p.p40 * locals.var_ccg1_dn3), (p.p40 * locals.var_ccg1_dn4), (p.p40 * locals.var_ccg1_dn5), (p.p40 * locals.var_ccg1_dn6), (p.p40 * locals.var_ccg1_dn7), (p.p40 * locals.var_ccg1_dn8), (p.p40 * locals.var_ccg1_dn9), (p.p40 * locals.var_ccg1_dn10), (p.p40 * locals.var_ccg1_dn11), (p.p40 * locals.var_ccg1_dn13), (p.p40 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign12200_e16071;
        locals.var_ccg_dn0 = assign12200_e16071_d_n0;
        locals.var_ccg_dn2 = assign12200_e16071_d_n2;
        locals.var_ccg_dn3 = assign12200_e16071_d_n3;
        locals.var_ccg_dn4 = assign12200_e16071_d_n4;
        locals.var_ccg_dn5 = assign12200_e16071_d_n5;
        locals.var_ccg_dn6 = assign12200_e16071_d_n6;
        locals.var_ccg_dn7 = assign12200_e16071_d_n7;
        locals.var_ccg_dn8 = assign12200_e16071_d_n8;
        locals.var_ccg_dn9 = assign12200_e16071_d_n9;
        locals.var_ccg_dn10 = assign12200_e16071_d_n10;
        locals.var_ccg_dn11 = assign12200_e16071_d_n11;
        locals.var_ccg_dn13 = assign12200_e16071_d_n13;
        locals.var_ccg_dn14 = assign12200_e16071_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign12210_e16080,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12210_e16078: f64 = (locals.var_lmax / locals.var_wg);
        (assign12210_e16078,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12210_e16080;
        locals.var_x_rv = 0.0;

        let (assign12220_e16096,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12220_e16089: f64 = (locals.var_x + 1.0);
        let assign12220_e16090: f64 = (2.0 * assign12220_e16089);
        let assign12220_e16091: f64 = (assign12220_e16090).sqrt();
        let assign12220_e16093: f64 = (assign12220_e16091 * 3.141592653589793);
        let assign12220_e16094: f64 = (4.0 / assign12220_e16093);
        (assign12220_e16094,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign12220_e16096;
        locals.var_c1_rv = 0.0;

        let (assign12230_e16133,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12230_e16103: f64 = (p.p90 * p.p90);
        let assign12230_e16106: f64 = (2.0 * locals.var_wg);
        let assign12230_e16108: f64 = (assign12230_e16106 * p.p90);
        let assign12230_e16109: f64 = (assign12230_e16103 + assign12230_e16108);
        let assign12230_e16112: f64 = (locals.var_wg * locals.var_wg);
        let assign12230_e16115: f64 = (locals.var_x + 1.0);
        let assign12230_e16116: f64 = (assign12230_e16112 * assign12230_e16115);
        let assign12230_e16117: f64 = (assign12230_e16109 + assign12230_e16116);
        let assign12230_e16118: f64 = (assign12230_e16117).sqrt();
        let assign12230_e16121: f64 = (locals.var_x + 1.0);
        let assign12230_e16122: f64 = (assign12230_e16121).sqrt();
        let assign12230_e16123: f64 = (assign12230_e16118 * assign12230_e16122);
        let assign12230_e16125: f64 = (assign12230_e16123 + p.p90);
        let assign12230_e16128: f64 = (locals.var_wg * locals.var_x);
        let assign12230_e16129: f64 = (assign12230_e16125 + assign12230_e16128);
        let assign12230_e16131: f64 = (assign12230_e16129 + locals.var_wg);
        (assign12230_e16131,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign12230_e16133;
        locals.var_c2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12240_e16155,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12240_e16141: f64 = (locals.var_x + 1.0);
        let assign12240_e16144: f64 = (locals.var_x + 4.0);
        let assign12240_e16145: f64 = (assign12240_e16141 * assign12240_e16144);
        let assign12240_e16146: f64 = (assign12240_e16145).sqrt();
        let assign12240_e16147: f64 = (p.p90 * assign12240_e16146);
        let assign12240_e16151: f64 = (locals.var_x + 2.0);
        let assign12240_e16152: f64 = (p.p90 * assign12240_e16151);
        let assign12240_e16153: f64 = (assign12240_e16147 + assign12240_e16152);
        (assign12240_e16153,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign12240_e16155;
        locals.var_c3_rv = 0.0;

        let (assign12250_e16187,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12250_e16164: f64 = (locals.var_c2 / locals.var_c3);
        let (assign12250_e16181,) = {
            if (!(assign12250_e16164 > 1e-38)) {
                let assign12250_e16169: f64 = (-87.498233534);
                (assign12250_e16169,)
            } else {
                let assign12250_e16172: f64 = (locals.var_c2 / locals.var_c3);
                let (assign12250_e16180,) = {
                    if (assign12250_e16172 > 1e-38) {
                        let assign12250_e16177: f64 = (locals.var_c2 / locals.var_c3);
                        let assign12250_e16178: f64 = (assign12250_e16177).ln();
                        (assign12250_e16178,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12250_e16180,)
            }
        };
        let assign12250_e16182: f64 = (locals.var_c1 * assign12250_e16181);
        let assign12250_e16184: f64 = (assign12250_e16182 + 12.27);
        let assign12250_e16185: f64 = (locals.var_epssp * assign12250_e16184);
        (assign12250_e16185,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign12250_e16187;
        locals.var_cfglog_rv = 0.0;

        let (assign12260_e16196,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12260_e16194: f64 = (locals.var_hr * locals.var_lr);
        (assign12260_e16194,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign12260_e16196;
        locals.var_dcf_rv = 0.0;

        let (assign12270_e16208,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12270_e16203: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12270_e16205: f64 = (assign12270_e16203 + 1.0);
        let assign12270_e16206: f64 = (assign12270_e16205).sqrt();
        (assign12270_e16206,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign12270_e16208;
        locals.var_tt0_rv = 0.0;

        let (assign12280_e16258, assign12280_e16258_d_n0, assign12280_e16258_d_n2, assign12280_e16258_d_n3, assign12280_e16258_d_n4, assign12280_e16258_d_n5, assign12280_e16258_d_n6, assign12280_e16258_d_n7, assign12280_e16258_d_n8, assign12280_e16258_d_n9, assign12280_e16258_d_n10, assign12280_e16258_d_n11, assign12280_e16258_d_n13, assign12280_e16258_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12280_e16215: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12280_e16217: f64 = (assign12280_e16215 + 1.0);
        let assign12280_e16220: f64 = (locals.var_dcf * p.p90);
        let assign12280_e16223: f64 = (locals.var_dcf * p.p90);
        let assign12280_e16224: f64 = (assign12280_e16220 * assign12280_e16223);
        let assign12280_e16227: f64 = (2.0 * locals.var_dcf);
        let assign12280_e16229: f64 = (assign12280_e16227 * locals.var_lmax);
        let assign12280_e16231: f64 = (assign12280_e16229 * p.p90);
        let assign12280_e16232: f64 = (assign12280_e16224 + assign12280_e16231);
        let assign12280_e16235: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12280_e16237: f64 = (assign12280_e16235 + 1.0);
        let assign12280_e16239: f64 = (assign12280_e16237 * locals.var_lmax);
        let assign12280_e16241: f64 = (assign12280_e16239 * locals.var_lmax);
        let assign12280_e16242: f64 = (assign12280_e16232 + assign12280_e16241);
        let assign12280_e16243: f64 = (assign12280_e16217 * assign12280_e16242);
        let assign12280_e16244: f64 = (assign12280_e16243).sqrt();
        let assign12280_e16247: f64 = (locals.var_dcf * p.p90);
        let assign12280_e16248: f64 = (assign12280_e16244 + assign12280_e16247);
        let assign12280_e16251: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12280_e16253: f64 = (assign12280_e16251 * locals.var_lmax);
        let assign12280_e16254: f64 = (assign12280_e16248 + assign12280_e16253);
        let assign12280_e16256: f64 = (assign12280_e16254 + locals.var_lmax);
        (assign12280_e16256, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12280_e16258;
        locals.var_tt1_dn0 = assign12280_e16258_d_n0;
        locals.var_tt1_dn2 = assign12280_e16258_d_n2;
        locals.var_tt1_dn3 = assign12280_e16258_d_n3;
        locals.var_tt1_dn4 = assign12280_e16258_d_n4;
        locals.var_tt1_dn5 = assign12280_e16258_d_n5;
        locals.var_tt1_dn6 = assign12280_e16258_d_n6;
        locals.var_tt1_dn7 = assign12280_e16258_d_n7;
        locals.var_tt1_dn8 = assign12280_e16258_d_n8;
        locals.var_tt1_dn9 = assign12280_e16258_d_n9;
        locals.var_tt1_dn10 = assign12280_e16258_d_n10;
        locals.var_tt1_dn11 = assign12280_e16258_d_n11;
        locals.var_tt1_dn13 = assign12280_e16258_d_n13;
        locals.var_tt1_dn14 = assign12280_e16258_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12290_e16271,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12290_e16265: f64 = (locals.var_tt0 + 1.0);
        let assign12290_e16268: f64 = (locals.var_dcf * p.p90);
        let assign12290_e16269: f64 = (assign12290_e16265 * assign12290_e16268);
        (assign12290_e16269,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign12290_e16271;
        locals.var_tt2_rv = 0.0;

        let (assign12300_e16312, assign12300_e16312_d_n0, assign12300_e16312_d_n2, assign12300_e16312_d_n3, assign12300_e16312_d_n4, assign12300_e16312_d_n5, assign12300_e16312_d_n6, assign12300_e16312_d_n7, assign12300_e16312_d_n8, assign12300_e16312_d_n9, assign12300_e16312_d_n10, assign12300_e16312_d_n11, assign12300_e16312_d_n13, assign12300_e16312_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12300_e16278: f64 = (2.0 * locals.var_epssp);
        let assign12300_e16280: f64 = (2.0_f64).sqrt();
        let assign12300_e16281: f64 = (assign12300_e16278 * assign12300_e16280);
        let assign12300_e16283: f64 = (assign12300_e16281 / 3.141592653589793);
        let assign12300_e16285: f64 = (assign12300_e16283 * 0.85);
        let assign12300_e16287: f64 = (assign12300_e16285 * locals.var_dcf);
        let assign12300_e16289: f64 = (assign12300_e16287 / locals.var_tt0);
        let assign12300_e16292: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign12300_e16309, assign12300_e16309_d_n0, assign12300_e16309_d_n2, assign12300_e16309_d_n3, assign12300_e16309_d_n4, assign12300_e16309_d_n5, assign12300_e16309_d_n6, assign12300_e16309_d_n7, assign12300_e16309_d_n8, assign12300_e16309_d_n9, assign12300_e16309_d_n10, assign12300_e16309_d_n11, assign12300_e16309_d_n13, assign12300_e16309_d_n14,) = {
            if (!(assign12300_e16292 > 1e-38)) {
                let assign12300_e16297: f64 = (-87.498233534);
                (assign12300_e16297, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign12300_e16300: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign12300_e16308, assign12300_e16308_d_n0, assign12300_e16308_d_n2, assign12300_e16308_d_n3, assign12300_e16308_d_n4, assign12300_e16308_d_n5, assign12300_e16308_d_n6, assign12300_e16308_d_n7, assign12300_e16308_d_n8, assign12300_e16308_d_n9, assign12300_e16308_d_n10, assign12300_e16308_d_n11, assign12300_e16308_d_n13, assign12300_e16308_d_n14,) = {
                    if (assign12300_e16300 > 1e-38) {
                        let assign12300_e16305: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign12300_e16306: f64 = (assign12300_e16305).ln();
                        (assign12300_e16306, ((locals.var_tt1_dn0 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn2 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn3 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn4 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn5 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn6 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn7 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn8 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn9 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn10 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn11 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn13 / locals.var_tt2) / assign12300_e16305), ((locals.var_tt1_dn14 / locals.var_tt2) / assign12300_e16305),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12300_e16308, assign12300_e16308_d_n0, assign12300_e16308_d_n2, assign12300_e16308_d_n3, assign12300_e16308_d_n4, assign12300_e16308_d_n5, assign12300_e16308_d_n6, assign12300_e16308_d_n7, assign12300_e16308_d_n8, assign12300_e16308_d_n9, assign12300_e16308_d_n10, assign12300_e16308_d_n11, assign12300_e16308_d_n13, assign12300_e16308_d_n14,)
            }
        };
        let assign12300_e16310: f64 = (assign12300_e16289 * assign12300_e16309);
        (assign12300_e16310, (assign12300_e16289 * assign12300_e16309_d_n0), (assign12300_e16289 * assign12300_e16309_d_n2), (assign12300_e16289 * assign12300_e16309_d_n3), (assign12300_e16289 * assign12300_e16309_d_n4), (assign12300_e16289 * assign12300_e16309_d_n5), (assign12300_e16289 * assign12300_e16309_d_n6), (assign12300_e16289 * assign12300_e16309_d_n7), (assign12300_e16289 * assign12300_e16309_d_n8), (assign12300_e16289 * assign12300_e16309_d_n9), (assign12300_e16289 * assign12300_e16309_d_n10), (assign12300_e16289 * assign12300_e16309_d_n11), (assign12300_e16289 * assign12300_e16309_d_n13), (assign12300_e16289 * assign12300_e16309_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign12300_e16312;
        locals.var_cfgsat_dn0 = assign12300_e16312_d_n0;
        locals.var_cfgsat_dn2 = assign12300_e16312_d_n2;
        locals.var_cfgsat_dn3 = assign12300_e16312_d_n3;
        locals.var_cfgsat_dn4 = assign12300_e16312_d_n4;
        locals.var_cfgsat_dn5 = assign12300_e16312_d_n5;
        locals.var_cfgsat_dn6 = assign12300_e16312_d_n6;
        locals.var_cfgsat_dn7 = assign12300_e16312_d_n7;
        locals.var_cfgsat_dn8 = assign12300_e16312_d_n8;
        locals.var_cfgsat_dn9 = assign12300_e16312_d_n9;
        locals.var_cfgsat_dn10 = assign12300_e16312_d_n10;
        locals.var_cfgsat_dn11 = assign12300_e16312_d_n11;
        locals.var_cfgsat_dn13 = assign12300_e16312_d_n13;
        locals.var_cfgsat_dn14 = assign12300_e16312_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign12310_e16319, assign12310_e16319_d_n0, assign12310_e16319_d_n2, assign12310_e16319_d_n3, assign12310_e16319_d_n4, assign12310_e16319_d_n5, assign12310_e16319_d_n6, assign12310_e16319_d_n7, assign12310_e16319_d_n8, assign12310_e16319_d_n9, assign12310_e16319_d_n10, assign12310_e16319_d_n11, assign12310_e16319_d_n13, assign12310_e16319_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign12310_e16319;
        locals.var_delta_dn0 = assign12310_e16319_d_n0;
        locals.var_delta_dn2 = assign12310_e16319_d_n2;
        locals.var_delta_dn3 = assign12310_e16319_d_n3;
        locals.var_delta_dn4 = assign12310_e16319_d_n4;
        locals.var_delta_dn5 = assign12310_e16319_d_n5;
        locals.var_delta_dn6 = assign12310_e16319_d_n6;
        locals.var_delta_dn7 = assign12310_e16319_d_n7;
        locals.var_delta_dn8 = assign12310_e16319_d_n8;
        locals.var_delta_dn9 = assign12310_e16319_d_n9;
        locals.var_delta_dn10 = assign12310_e16319_d_n10;
        locals.var_delta_dn11 = assign12310_e16319_d_n11;
        locals.var_delta_dn13 = assign12310_e16319_d_n13;
        locals.var_delta_dn14 = assign12310_e16319_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign12320_e16330, assign12320_e16330_d_n0, assign12320_e16330_d_n2, assign12320_e16330_d_n3, assign12320_e16330_d_n4, assign12320_e16330_d_n5, assign12320_e16330_d_n6, assign12320_e16330_d_n7, assign12320_e16330_d_n8, assign12320_e16330_d_n9, assign12320_e16330_d_n10, assign12320_e16330_d_n11, assign12320_e16330_d_n13, assign12320_e16330_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12320_e16326: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign12320_e16328: f64 = (assign12320_e16326 - locals.var_delta);
        (assign12320_e16328, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12320_e16330;
        locals.var_tt1_dn0 = assign12320_e16330_d_n0;
        locals.var_tt1_dn2 = assign12320_e16330_d_n2;
        locals.var_tt1_dn3 = assign12320_e16330_d_n3;
        locals.var_tt1_dn4 = assign12320_e16330_d_n4;
        locals.var_tt1_dn5 = assign12320_e16330_d_n5;
        locals.var_tt1_dn6 = assign12320_e16330_d_n6;
        locals.var_tt1_dn7 = assign12320_e16330_d_n7;
        locals.var_tt1_dn8 = assign12320_e16330_d_n8;
        locals.var_tt1_dn9 = assign12320_e16330_d_n9;
        locals.var_tt1_dn10 = assign12320_e16330_d_n10;
        locals.var_tt1_dn11 = assign12320_e16330_d_n11;
        locals.var_tt1_dn13 = assign12320_e16330_d_n13;
        locals.var_tt1_dn14 = assign12320_e16330_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12330_e16354, assign12330_e16354_d_n0, assign12330_e16354_d_n2, assign12330_e16354_d_n3, assign12330_e16354_d_n4, assign12330_e16354_d_n5, assign12330_e16354_d_n6, assign12330_e16354_d_n7, assign12330_e16354_d_n8, assign12330_e16354_d_n9, assign12330_e16354_d_n10, assign12330_e16354_d_n11, assign12330_e16354_d_n13, assign12330_e16354_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12330_e16341: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign12330_e16344: f64 = (4.0 * locals.var_delta);
        let assign12330_e16346: f64 = (assign12330_e16344 * locals.var_cfgsat);
        let assign12330_e16347: f64 = (assign12330_e16341 + assign12330_e16346);
        let assign12330_e16348: f64 = (assign12330_e16347).sqrt();
        let assign12330_e16349: f64 = (locals.var_tt1 + assign12330_e16348);
        let assign12330_e16350: f64 = (0.5 * assign12330_e16349);
        let assign12330_e16351: f64 = (locals.var_cfgsat - assign12330_e16350);
        let assign12330_e16352: f64 = (p.p40 * assign12330_e16351);
        (assign12330_e16352, (p.p40 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn0))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn2))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn3))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn4))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn5))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn6))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn7))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn8))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn9))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn10))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn11))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn13))) / (2.0 * assign12330_e16348)))))), (p.p40 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign12330_e16344 * locals.var_cfgsat_dn14))) / (2.0 * assign12330_e16348)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign12330_e16354;
        locals.var_cfg_dn0 = assign12330_e16354_d_n0;
        locals.var_cfg_dn2 = assign12330_e16354_d_n2;
        locals.var_cfg_dn3 = assign12330_e16354_d_n3;
        locals.var_cfg_dn4 = assign12330_e16354_d_n4;
        locals.var_cfg_dn5 = assign12330_e16354_d_n5;
        locals.var_cfg_dn6 = assign12330_e16354_d_n6;
        locals.var_cfg_dn7 = assign12330_e16354_d_n7;
        locals.var_cfg_dn8 = assign12330_e16354_d_n8;
        locals.var_cfg_dn9 = assign12330_e16354_d_n9;
        locals.var_cfg_dn10 = assign12330_e16354_d_n10;
        locals.var_cfg_dn11 = assign12330_e16354_d_n11;
        locals.var_cfg_dn13 = assign12330_e16354_d_n13;
        locals.var_cfg_dn14 = assign12330_e16354_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign12340_e16363, assign12340_e16363_d_n0, assign12340_e16363_d_n2, assign12340_e16363_d_n3, assign12340_e16363_d_n4, assign12340_e16363_d_n5, assign12340_e16363_d_n6, assign12340_e16363_d_n7, assign12340_e16363_d_n8, assign12340_e16363_d_n9, assign12340_e16363_d_n10, assign12340_e16363_d_n11, assign12340_e16363_d_n13, assign12340_e16363_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard217 == 0.0)) {
        let assign12340_e16361: f64 = (locals.var_ccg + locals.var_cfg);
        (assign12340_e16361, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_sidetopm, locals.var_cgg_sidetopm_dn0, locals.var_cgg_sidetopm_dn2, locals.var_cgg_sidetopm_dn3, locals.var_cgg_sidetopm_dn4, locals.var_cgg_sidetopm_dn5, locals.var_cgg_sidetopm_dn6, locals.var_cgg_sidetopm_dn7, locals.var_cgg_sidetopm_dn8, locals.var_cgg_sidetopm_dn9, locals.var_cgg_sidetopm_dn10, locals.var_cgg_sidetopm_dn11, locals.var_cgg_sidetopm_dn13, locals.var_cgg_sidetopm_dn14,)
    }
};
        locals.var_cgg_sidetopm = assign12340_e16363;
        locals.var_cgg_sidetopm_dn0 = assign12340_e16363_d_n0;
        locals.var_cgg_sidetopm_dn2 = assign12340_e16363_d_n2;
        locals.var_cgg_sidetopm_dn3 = assign12340_e16363_d_n3;
        locals.var_cgg_sidetopm_dn4 = assign12340_e16363_d_n4;
        locals.var_cgg_sidetopm_dn5 = assign12340_e16363_d_n5;
        locals.var_cgg_sidetopm_dn6 = assign12340_e16363_d_n6;
        locals.var_cgg_sidetopm_dn7 = assign12340_e16363_d_n7;
        locals.var_cgg_sidetopm_dn8 = assign12340_e16363_d_n8;
        locals.var_cgg_sidetopm_dn9 = assign12340_e16363_d_n9;
        locals.var_cgg_sidetopm_dn10 = assign12340_e16363_d_n10;
        locals.var_cgg_sidetopm_dn11 = assign12340_e16363_d_n11;
        locals.var_cgg_sidetopm_dn13 = assign12340_e16363_d_n13;
        locals.var_cgg_sidetopm_dn14 = assign12340_e16363_d_n14;
        locals.var_cgg_sidetopm_rv = 0.0;

        let (assign12350_e16375,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12350_e16369: f64 = (locals.var_wg + p.p90);
        let assign12350_e16370: f64 = (0.2 * assign12350_e16369);
        let assign12350_e16372: f64 = (assign12350_e16370 / locals.var_trsd);
        let assign12350_e16373: f64 = (2.3 + assign12350_e16372);
        (assign12350_e16373,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign12350_e16375;
        locals.var_hr_rv = 0.0;

        let (assign12360_e16379,) = {
    if (locals.var_guard213 != 0.0) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign12360_e16379;
        locals.var_lr_rv = 0.0;

        let (assign12370_e16388,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12370_e16383: f64 = (locals.var_wg + p.p90);
        let assign12370_e16385: f64 = (assign12370_e16383 - locals.var_trsd);
        let assign12370_e16386: f64 = (assign12370_e16385).abs();
        (assign12370_e16386,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign12370_e16388;
        locals.var_hgdelta_rv = 0.0;

        let (assign12380_e16394,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12380_e16392: f64 = (p.p1087 * locals.var_lr);
        (assign12380_e16392,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign12380_e16394;
        locals.var_lmax_rv = 0.0;

        let (assign12390_e16402,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12390_e16399: f64 = (locals.var_wg + p.p90);
        let assign12390_e16400: f64 = (locals.var_trsd).min(assign12390_e16399);
        (assign12390_e16400,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign12390_e16402;
        locals.var_y_rv = 0.0;

        let (assign12400_e16410,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12400_e16407: f64 = (locals.var_hr + 1.0);
        let assign12400_e16408: f64 = (p.p1087 / assign12400_e16407);
        (assign12400_e16408,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12400_e16410;
        locals.var_x_rv = 0.0;

        let (assign12410_e16414,) = {
    if (locals.var_guard213 != 0.0) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign12410_e16414;
        locals.var_cnon_rv = 0.0;

        let (assign12420_e16424,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12420_e16419: f64 = (locals.var_y - locals.var_x);
        let assign12420_e16420: f64 = (locals.var_epssp * assign12420_e16419);
        let assign12420_e16422: f64 = (assign12420_e16420 / p.p1087);
        (assign12420_e16422,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign12420_e16424;
        locals.var_ccgsat_rv = 0.0;

        let (assign12430_e16430, assign12430_e16430_d_n0, assign12430_e16430_d_n2, assign12430_e16430_d_n3, assign12430_e16430_d_n4, assign12430_e16430_d_n5, assign12430_e16430_d_n6, assign12430_e16430_d_n7, assign12430_e16430_d_n8, assign12430_e16430_d_n9, assign12430_e16430_d_n10, assign12430_e16430_d_n11, assign12430_e16430_d_n13, assign12430_e16430_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12430_e16428: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign12430_e16428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12430_e16430;
        locals.var_tt1_dn0 = assign12430_e16430_d_n0;
        locals.var_tt1_dn2 = assign12430_e16430_d_n2;
        locals.var_tt1_dn3 = assign12430_e16430_d_n3;
        locals.var_tt1_dn4 = assign12430_e16430_d_n4;
        locals.var_tt1_dn5 = assign12430_e16430_d_n5;
        locals.var_tt1_dn6 = assign12430_e16430_d_n6;
        locals.var_tt1_dn7 = assign12430_e16430_d_n7;
        locals.var_tt1_dn8 = assign12430_e16430_d_n8;
        locals.var_tt1_dn9 = assign12430_e16430_d_n9;
        locals.var_tt1_dn10 = assign12430_e16430_d_n10;
        locals.var_tt1_dn11 = assign12430_e16430_d_n11;
        locals.var_tt1_dn13 = assign12430_e16430_d_n13;
        locals.var_tt1_dn14 = assign12430_e16430_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign12440_e16433: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard220 = assign12440_e16433;
        locals.var_guard220_rv = 0.0;

        let (assign12450_e16439, assign12450_e16439_d_n0, assign12450_e16439_d_n2, assign12450_e16439_d_n3, assign12450_e16439_d_n4, assign12450_e16439_d_n5, assign12450_e16439_d_n6, assign12450_e16439_d_n7, assign12450_e16439_d_n8, assign12450_e16439_d_n9, assign12450_e16439_d_n10, assign12450_e16439_d_n11, assign12450_e16439_d_n13, assign12450_e16439_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard220 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12450_e16439;
        locals.var_ccg1_dn0 = assign12450_e16439_d_n0;
        locals.var_ccg1_dn2 = assign12450_e16439_d_n2;
        locals.var_ccg1_dn3 = assign12450_e16439_d_n3;
        locals.var_ccg1_dn4 = assign12450_e16439_d_n4;
        locals.var_ccg1_dn5 = assign12450_e16439_d_n5;
        locals.var_ccg1_dn6 = assign12450_e16439_d_n6;
        locals.var_ccg1_dn7 = assign12450_e16439_d_n7;
        locals.var_ccg1_dn8 = assign12450_e16439_d_n8;
        locals.var_ccg1_dn9 = assign12450_e16439_d_n9;
        locals.var_ccg1_dn10 = assign12450_e16439_d_n10;
        locals.var_ccg1_dn11 = assign12450_e16439_d_n11;
        locals.var_ccg1_dn13 = assign12450_e16439_d_n13;
        locals.var_ccg1_dn14 = assign12450_e16439_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12460_e16483, assign12460_e16483_d_n0, assign12460_e16483_d_n2, assign12460_e16483_d_n3, assign12460_e16483_d_n4, assign12460_e16483_d_n5, assign12460_e16483_d_n6, assign12460_e16483_d_n7, assign12460_e16483_d_n8, assign12460_e16483_d_n9, assign12460_e16483_d_n10, assign12460_e16483_d_n11, assign12460_e16483_d_n13, assign12460_e16483_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard220 == 0.0)) {
        let assign12460_e16446: f64 = (1.0 / locals.var_cnon);
        let assign12460_e16453: f64 = (-37.0);
        let (assign12460_e16480, assign12460_e16480_d_n0, assign12460_e16480_d_n2, assign12460_e16480_d_n3, assign12460_e16480_d_n4, assign12460_e16480_d_n5, assign12460_e16480_d_n6, assign12460_e16480_d_n7, assign12460_e16480_d_n8, assign12460_e16480_d_n9, assign12460_e16480_d_n10, assign12460_e16480_d_n11, assign12460_e16480_d_n13, assign12460_e16480_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign12460_e16453))) {
                let assign12460_e16459: f64 = (locals.var_tt1).exp();
                let assign12460_e16460: f64 = (1.0 + assign12460_e16459);
                let assign12460_e16461: f64 = (assign12460_e16460).ln();
                (assign12460_e16461, ((assign12460_e16459 * locals.var_tt1_dn0) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn2) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn3) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn4) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn5) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn6) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn7) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn8) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn9) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn10) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn11) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn13) / assign12460_e16460), ((assign12460_e16459 * locals.var_tt1_dn14) / assign12460_e16460),)
            } else {
                let assign12460_e16468: f64 = (-37.0);
                let (assign12460_e16479, assign12460_e16479_d_n0, assign12460_e16479_d_n2, assign12460_e16479_d_n3, assign12460_e16479_d_n4, assign12460_e16479_d_n5, assign12460_e16479_d_n6, assign12460_e16479_d_n7, assign12460_e16479_d_n8, assign12460_e16479_d_n9, assign12460_e16479_d_n10, assign12460_e16479_d_n11, assign12460_e16479_d_n13, assign12460_e16479_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign12460_e16468)) {
                        let assign12460_e16472: f64 = (locals.var_tt1).exp();
                        (assign12460_e16472, (assign12460_e16472 * locals.var_tt1_dn0), (assign12460_e16472 * locals.var_tt1_dn2), (assign12460_e16472 * locals.var_tt1_dn3), (assign12460_e16472 * locals.var_tt1_dn4), (assign12460_e16472 * locals.var_tt1_dn5), (assign12460_e16472 * locals.var_tt1_dn6), (assign12460_e16472 * locals.var_tt1_dn7), (assign12460_e16472 * locals.var_tt1_dn8), (assign12460_e16472 * locals.var_tt1_dn9), (assign12460_e16472 * locals.var_tt1_dn10), (assign12460_e16472 * locals.var_tt1_dn11), (assign12460_e16472 * locals.var_tt1_dn13), (assign12460_e16472 * locals.var_tt1_dn14),)
                    } else {
                        let (assign12460_e16478, assign12460_e16478_d_n0, assign12460_e16478_d_n2, assign12460_e16478_d_n3, assign12460_e16478_d_n4, assign12460_e16478_d_n5, assign12460_e16478_d_n6, assign12460_e16478_d_n7, assign12460_e16478_d_n8, assign12460_e16478_d_n9, assign12460_e16478_d_n10, assign12460_e16478_d_n11, assign12460_e16478_d_n13, assign12460_e16478_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign12460_e16478, assign12460_e16478_d_n0, assign12460_e16478_d_n2, assign12460_e16478_d_n3, assign12460_e16478_d_n4, assign12460_e16478_d_n5, assign12460_e16478_d_n6, assign12460_e16478_d_n7, assign12460_e16478_d_n8, assign12460_e16478_d_n9, assign12460_e16478_d_n10, assign12460_e16478_d_n11, assign12460_e16478_d_n13, assign12460_e16478_d_n14,)
                    }
                };
                (assign12460_e16479, assign12460_e16479_d_n0, assign12460_e16479_d_n2, assign12460_e16479_d_n3, assign12460_e16479_d_n4, assign12460_e16479_d_n5, assign12460_e16479_d_n6, assign12460_e16479_d_n7, assign12460_e16479_d_n8, assign12460_e16479_d_n9, assign12460_e16479_d_n10, assign12460_e16479_d_n11, assign12460_e16479_d_n13, assign12460_e16479_d_n14,)
            }
        };
        let assign12460_e16481: f64 = (assign12460_e16446 * assign12460_e16480);
        (assign12460_e16481, (assign12460_e16446 * assign12460_e16480_d_n0), (assign12460_e16446 * assign12460_e16480_d_n2), (assign12460_e16446 * assign12460_e16480_d_n3), (assign12460_e16446 * assign12460_e16480_d_n4), (assign12460_e16446 * assign12460_e16480_d_n5), (assign12460_e16446 * assign12460_e16480_d_n6), (assign12460_e16446 * assign12460_e16480_d_n7), (assign12460_e16446 * assign12460_e16480_d_n8), (assign12460_e16446 * assign12460_e16480_d_n9), (assign12460_e16446 * assign12460_e16480_d_n10), (assign12460_e16446 * assign12460_e16480_d_n11), (assign12460_e16446 * assign12460_e16480_d_n13), (assign12460_e16446 * assign12460_e16480_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12460_e16483;
        locals.var_ccg1_dn0 = assign12460_e16483_d_n0;
        locals.var_ccg1_dn2 = assign12460_e16483_d_n2;
        locals.var_ccg1_dn3 = assign12460_e16483_d_n3;
        locals.var_ccg1_dn4 = assign12460_e16483_d_n4;
        locals.var_ccg1_dn5 = assign12460_e16483_d_n5;
        locals.var_ccg1_dn6 = assign12460_e16483_d_n6;
        locals.var_ccg1_dn7 = assign12460_e16483_d_n7;
        locals.var_ccg1_dn8 = assign12460_e16483_d_n8;
        locals.var_ccg1_dn9 = assign12460_e16483_d_n9;
        locals.var_ccg1_dn10 = assign12460_e16483_d_n10;
        locals.var_ccg1_dn11 = assign12460_e16483_d_n11;
        locals.var_ccg1_dn13 = assign12460_e16483_d_n13;
        locals.var_ccg1_dn14 = assign12460_e16483_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12470_e16499,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12470_e16489: f64 = (locals.var_wg + p.p90);
        let assign12470_e16490: f64 = (locals.var_trsd / assign12470_e16489);
        let assign12470_e16493: f64 = (locals.var_wg + p.p90);
        let assign12470_e16495: f64 = (assign12470_e16493 / locals.var_trsd);
        let assign12470_e16496: f64 = (assign12470_e16490).min(assign12470_e16495);
        let assign12470_e16497: f64 = (0.5 * assign12470_e16496);
        (assign12470_e16497,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign12470_e16499;
        locals.var_r1cf_rv = 0.0;

        let (assign12480_e16505,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12480_e16503: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign12480_e16503,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign12480_e16505;
        locals.var_rcf_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12490_e16552,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12490_e16509: f64 = (locals.var_epssp * 2.0);
        let assign12490_e16511: f64 = (assign12490_e16509 / 3.141592653589793);
        let assign12490_e16515: f64 = (0.5 * 3.141592653589793);
        let assign12490_e16517: f64 = (assign12490_e16515 * locals.var_rcf);
        let assign12490_e16518: f64 = (p.p1087 + assign12490_e16517);
        let assign12490_e16520: f64 = (assign12490_e16518 / p.p1087);
        let (assign12490_e16549,) = {
            if (!(assign12490_e16520 > 1e-38)) {
                let assign12490_e16525: f64 = (-87.498233534);
                (assign12490_e16525,)
            } else {
                let assign12490_e16529: f64 = (0.5 * 3.141592653589793);
                let assign12490_e16531: f64 = (assign12490_e16529 * locals.var_rcf);
                let assign12490_e16532: f64 = (p.p1087 + assign12490_e16531);
                let assign12490_e16534: f64 = (assign12490_e16532 / p.p1087);
                let (assign12490_e16548,) = {
                    if (assign12490_e16534 > 1e-38) {
                        let assign12490_e16540: f64 = (0.5 * 3.141592653589793);
                        let assign12490_e16542: f64 = (assign12490_e16540 * locals.var_rcf);
                        let assign12490_e16543: f64 = (p.p1087 + assign12490_e16542);
                        let assign12490_e16545: f64 = (assign12490_e16543 / p.p1087);
                        let assign12490_e16546: f64 = (assign12490_e16545).ln();
                        (assign12490_e16546,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12490_e16548,)
            }
        };
        let assign12490_e16550: f64 = (assign12490_e16511 * assign12490_e16549);
        (assign12490_e16550,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign12490_e16552;
        locals.var_ccg2_rv = 0.0;

        let (assign12500_e16560, assign12500_e16560_d_n0, assign12500_e16560_d_n2, assign12500_e16560_d_n3, assign12500_e16560_d_n4, assign12500_e16560_d_n5, assign12500_e16560_d_n6, assign12500_e16560_d_n7, assign12500_e16560_d_n8, assign12500_e16560_d_n9, assign12500_e16560_d_n10, assign12500_e16560_d_n11, assign12500_e16560_d_n13, assign12500_e16560_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12500_e16557: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign12500_e16558: f64 = (p.p40 * assign12500_e16557);
        (assign12500_e16558, (p.p40 * locals.var_ccg1_dn0), (p.p40 * locals.var_ccg1_dn2), (p.p40 * locals.var_ccg1_dn3), (p.p40 * locals.var_ccg1_dn4), (p.p40 * locals.var_ccg1_dn5), (p.p40 * locals.var_ccg1_dn6), (p.p40 * locals.var_ccg1_dn7), (p.p40 * locals.var_ccg1_dn8), (p.p40 * locals.var_ccg1_dn9), (p.p40 * locals.var_ccg1_dn10), (p.p40 * locals.var_ccg1_dn11), (p.p40 * locals.var_ccg1_dn13), (p.p40 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign12500_e16560;
        locals.var_ccg_dn0 = assign12500_e16560_d_n0;
        locals.var_ccg_dn2 = assign12500_e16560_d_n2;
        locals.var_ccg_dn3 = assign12500_e16560_d_n3;
        locals.var_ccg_dn4 = assign12500_e16560_d_n4;
        locals.var_ccg_dn5 = assign12500_e16560_d_n5;
        locals.var_ccg_dn6 = assign12500_e16560_d_n6;
        locals.var_ccg_dn7 = assign12500_e16560_d_n7;
        locals.var_ccg_dn8 = assign12500_e16560_d_n8;
        locals.var_ccg_dn9 = assign12500_e16560_d_n9;
        locals.var_ccg_dn10 = assign12500_e16560_d_n10;
        locals.var_ccg_dn11 = assign12500_e16560_d_n11;
        locals.var_ccg_dn13 = assign12500_e16560_d_n13;
        locals.var_ccg_dn14 = assign12500_e16560_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign12510_e16566,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12510_e16564: f64 = (locals.var_lmax / locals.var_wg);
        (assign12510_e16564,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12510_e16566;
        locals.var_x_rv = 0.0;

        let (assign12520_e16579,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12520_e16572: f64 = (locals.var_x + 1.0);
        let assign12520_e16573: f64 = (2.0 * assign12520_e16572);
        let assign12520_e16574: f64 = (assign12520_e16573).sqrt();
        let assign12520_e16576: f64 = (assign12520_e16574 * 3.141592653589793);
        let assign12520_e16577: f64 = (4.0 / assign12520_e16576);
        (assign12520_e16577,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign12520_e16579;
        locals.var_c1_rv = 0.0;

        let (assign12530_e16613,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12530_e16583: f64 = (p.p90 * p.p90);
        let assign12530_e16586: f64 = (2.0 * locals.var_wg);
        let assign12530_e16588: f64 = (assign12530_e16586 * p.p90);
        let assign12530_e16589: f64 = (assign12530_e16583 + assign12530_e16588);
        let assign12530_e16592: f64 = (locals.var_wg * locals.var_wg);
        let assign12530_e16595: f64 = (locals.var_x + 1.0);
        let assign12530_e16596: f64 = (assign12530_e16592 * assign12530_e16595);
        let assign12530_e16597: f64 = (assign12530_e16589 + assign12530_e16596);
        let assign12530_e16598: f64 = (assign12530_e16597).sqrt();
        let assign12530_e16601: f64 = (locals.var_x + 1.0);
        let assign12530_e16602: f64 = (assign12530_e16601).sqrt();
        let assign12530_e16603: f64 = (assign12530_e16598 * assign12530_e16602);
        let assign12530_e16605: f64 = (assign12530_e16603 + p.p90);
        let assign12530_e16608: f64 = (locals.var_wg * locals.var_x);
        let assign12530_e16609: f64 = (assign12530_e16605 + assign12530_e16608);
        let assign12530_e16611: f64 = (assign12530_e16609 + locals.var_wg);
        (assign12530_e16611,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign12530_e16613;
        locals.var_c2_rv = 0.0;

        let (assign12540_e16632,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12540_e16618: f64 = (locals.var_x + 1.0);
        let assign12540_e16621: f64 = (locals.var_x + 4.0);
        let assign12540_e16622: f64 = (assign12540_e16618 * assign12540_e16621);
        let assign12540_e16623: f64 = (assign12540_e16622).sqrt();
        let assign12540_e16624: f64 = (p.p90 * assign12540_e16623);
        let assign12540_e16628: f64 = (locals.var_x + 2.0);
        let assign12540_e16629: f64 = (p.p90 * assign12540_e16628);
        let assign12540_e16630: f64 = (assign12540_e16624 + assign12540_e16629);
        (assign12540_e16630,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign12540_e16632;
        locals.var_c3_rv = 0.0;

        let (assign12550_e16661,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12550_e16638: f64 = (locals.var_c2 / locals.var_c3);
        let (assign12550_e16655,) = {
            if (!(assign12550_e16638 > 1e-38)) {
                let assign12550_e16643: f64 = (-87.498233534);
                (assign12550_e16643,)
            } else {
                let assign12550_e16646: f64 = (locals.var_c2 / locals.var_c3);
                let (assign12550_e16654,) = {
                    if (assign12550_e16646 > 1e-38) {
                        let assign12550_e16651: f64 = (locals.var_c2 / locals.var_c3);
                        let assign12550_e16652: f64 = (assign12550_e16651).ln();
                        (assign12550_e16652,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12550_e16654,)
            }
        };
        let assign12550_e16656: f64 = (locals.var_c1 * assign12550_e16655);
        let assign12550_e16658: f64 = (assign12550_e16656 + 12.27);
        let assign12550_e16659: f64 = (locals.var_epssp * assign12550_e16658);
        (assign12550_e16659,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign12550_e16661;
        locals.var_cfglog_rv = 0.0;

        let (assign12560_e16667,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12560_e16665: f64 = (locals.var_hr * locals.var_lr);
        (assign12560_e16665,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign12560_e16667;
        locals.var_dcf_rv = 0.0;

        let (assign12570_e16676,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12570_e16671: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12570_e16673: f64 = (assign12570_e16671 + 1.0);
        let assign12570_e16674: f64 = (assign12570_e16673).sqrt();
        (assign12570_e16674,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign12570_e16676;
        locals.var_tt0_rv = 0.0;

        let (assign12580_e16723, assign12580_e16723_d_n0, assign12580_e16723_d_n2, assign12580_e16723_d_n3, assign12580_e16723_d_n4, assign12580_e16723_d_n5, assign12580_e16723_d_n6, assign12580_e16723_d_n7, assign12580_e16723_d_n8, assign12580_e16723_d_n9, assign12580_e16723_d_n10, assign12580_e16723_d_n11, assign12580_e16723_d_n13, assign12580_e16723_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12580_e16680: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12580_e16682: f64 = (assign12580_e16680 + 1.0);
        let assign12580_e16685: f64 = (locals.var_dcf * p.p90);
        let assign12580_e16688: f64 = (locals.var_dcf * p.p90);
        let assign12580_e16689: f64 = (assign12580_e16685 * assign12580_e16688);
        let assign12580_e16692: f64 = (2.0 * locals.var_dcf);
        let assign12580_e16694: f64 = (assign12580_e16692 * locals.var_lmax);
        let assign12580_e16696: f64 = (assign12580_e16694 * p.p90);
        let assign12580_e16697: f64 = (assign12580_e16689 + assign12580_e16696);
        let assign12580_e16700: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12580_e16702: f64 = (assign12580_e16700 + 1.0);
        let assign12580_e16704: f64 = (assign12580_e16702 * locals.var_lmax);
        let assign12580_e16706: f64 = (assign12580_e16704 * locals.var_lmax);
        let assign12580_e16707: f64 = (assign12580_e16697 + assign12580_e16706);
        let assign12580_e16708: f64 = (assign12580_e16682 * assign12580_e16707);
        let assign12580_e16709: f64 = (assign12580_e16708).sqrt();
        let assign12580_e16712: f64 = (locals.var_dcf * p.p90);
        let assign12580_e16713: f64 = (assign12580_e16709 + assign12580_e16712);
        let assign12580_e16716: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12580_e16718: f64 = (assign12580_e16716 * locals.var_lmax);
        let assign12580_e16719: f64 = (assign12580_e16713 + assign12580_e16718);
        let assign12580_e16721: f64 = (assign12580_e16719 + locals.var_lmax);
        (assign12580_e16721, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12580_e16723;
        locals.var_tt1_dn0 = assign12580_e16723_d_n0;
        locals.var_tt1_dn2 = assign12580_e16723_d_n2;
        locals.var_tt1_dn3 = assign12580_e16723_d_n3;
        locals.var_tt1_dn4 = assign12580_e16723_d_n4;
        locals.var_tt1_dn5 = assign12580_e16723_d_n5;
        locals.var_tt1_dn6 = assign12580_e16723_d_n6;
        locals.var_tt1_dn7 = assign12580_e16723_d_n7;
        locals.var_tt1_dn8 = assign12580_e16723_d_n8;
        locals.var_tt1_dn9 = assign12580_e16723_d_n9;
        locals.var_tt1_dn10 = assign12580_e16723_d_n10;
        locals.var_tt1_dn11 = assign12580_e16723_d_n11;
        locals.var_tt1_dn13 = assign12580_e16723_d_n13;
        locals.var_tt1_dn14 = assign12580_e16723_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12590_e16733,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12590_e16727: f64 = (locals.var_tt0 + 1.0);
        let assign12590_e16730: f64 = (locals.var_dcf * p.p90);
        let assign12590_e16731: f64 = (assign12590_e16727 * assign12590_e16730);
        (assign12590_e16731,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign12590_e16733;
        locals.var_tt2_rv = 0.0;

        let (assign12600_e16771, assign12600_e16771_d_n0, assign12600_e16771_d_n2, assign12600_e16771_d_n3, assign12600_e16771_d_n4, assign12600_e16771_d_n5, assign12600_e16771_d_n6, assign12600_e16771_d_n7, assign12600_e16771_d_n8, assign12600_e16771_d_n9, assign12600_e16771_d_n10, assign12600_e16771_d_n11, assign12600_e16771_d_n13, assign12600_e16771_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12600_e16737: f64 = (2.0 * locals.var_epssp);
        let assign12600_e16739: f64 = (2.0_f64).sqrt();
        let assign12600_e16740: f64 = (assign12600_e16737 * assign12600_e16739);
        let assign12600_e16742: f64 = (assign12600_e16740 / 3.141592653589793);
        let assign12600_e16744: f64 = (assign12600_e16742 * 0.85);
        let assign12600_e16746: f64 = (assign12600_e16744 * locals.var_dcf);
        let assign12600_e16748: f64 = (assign12600_e16746 / locals.var_tt0);
        let assign12600_e16751: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign12600_e16768, assign12600_e16768_d_n0, assign12600_e16768_d_n2, assign12600_e16768_d_n3, assign12600_e16768_d_n4, assign12600_e16768_d_n5, assign12600_e16768_d_n6, assign12600_e16768_d_n7, assign12600_e16768_d_n8, assign12600_e16768_d_n9, assign12600_e16768_d_n10, assign12600_e16768_d_n11, assign12600_e16768_d_n13, assign12600_e16768_d_n14,) = {
            if (!(assign12600_e16751 > 1e-38)) {
                let assign12600_e16756: f64 = (-87.498233534);
                (assign12600_e16756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign12600_e16759: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign12600_e16767, assign12600_e16767_d_n0, assign12600_e16767_d_n2, assign12600_e16767_d_n3, assign12600_e16767_d_n4, assign12600_e16767_d_n5, assign12600_e16767_d_n6, assign12600_e16767_d_n7, assign12600_e16767_d_n8, assign12600_e16767_d_n9, assign12600_e16767_d_n10, assign12600_e16767_d_n11, assign12600_e16767_d_n13, assign12600_e16767_d_n14,) = {
                    if (assign12600_e16759 > 1e-38) {
                        let assign12600_e16764: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign12600_e16765: f64 = (assign12600_e16764).ln();
                        (assign12600_e16765, ((locals.var_tt1_dn0 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn2 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn3 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn4 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn5 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn6 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn7 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn8 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn9 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn10 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn11 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn13 / locals.var_tt2) / assign12600_e16764), ((locals.var_tt1_dn14 / locals.var_tt2) / assign12600_e16764),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12600_e16767, assign12600_e16767_d_n0, assign12600_e16767_d_n2, assign12600_e16767_d_n3, assign12600_e16767_d_n4, assign12600_e16767_d_n5, assign12600_e16767_d_n6, assign12600_e16767_d_n7, assign12600_e16767_d_n8, assign12600_e16767_d_n9, assign12600_e16767_d_n10, assign12600_e16767_d_n11, assign12600_e16767_d_n13, assign12600_e16767_d_n14,)
            }
        };
        let assign12600_e16769: f64 = (assign12600_e16748 * assign12600_e16768);
        (assign12600_e16769, (assign12600_e16748 * assign12600_e16768_d_n0), (assign12600_e16748 * assign12600_e16768_d_n2), (assign12600_e16748 * assign12600_e16768_d_n3), (assign12600_e16748 * assign12600_e16768_d_n4), (assign12600_e16748 * assign12600_e16768_d_n5), (assign12600_e16748 * assign12600_e16768_d_n6), (assign12600_e16748 * assign12600_e16768_d_n7), (assign12600_e16748 * assign12600_e16768_d_n8), (assign12600_e16748 * assign12600_e16768_d_n9), (assign12600_e16748 * assign12600_e16768_d_n10), (assign12600_e16748 * assign12600_e16768_d_n11), (assign12600_e16748 * assign12600_e16768_d_n13), (assign12600_e16748 * assign12600_e16768_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign12600_e16771;
        locals.var_cfgsat_dn0 = assign12600_e16771_d_n0;
        locals.var_cfgsat_dn2 = assign12600_e16771_d_n2;
        locals.var_cfgsat_dn3 = assign12600_e16771_d_n3;
        locals.var_cfgsat_dn4 = assign12600_e16771_d_n4;
        locals.var_cfgsat_dn5 = assign12600_e16771_d_n5;
        locals.var_cfgsat_dn6 = assign12600_e16771_d_n6;
        locals.var_cfgsat_dn7 = assign12600_e16771_d_n7;
        locals.var_cfgsat_dn8 = assign12600_e16771_d_n8;
        locals.var_cfgsat_dn9 = assign12600_e16771_d_n9;
        locals.var_cfgsat_dn10 = assign12600_e16771_d_n10;
        locals.var_cfgsat_dn11 = assign12600_e16771_d_n11;
        locals.var_cfgsat_dn13 = assign12600_e16771_d_n13;
        locals.var_cfgsat_dn14 = assign12600_e16771_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign12610_e16775, assign12610_e16775_d_n0, assign12610_e16775_d_n2, assign12610_e16775_d_n3, assign12610_e16775_d_n4, assign12610_e16775_d_n5, assign12610_e16775_d_n6, assign12610_e16775_d_n7, assign12610_e16775_d_n8, assign12610_e16775_d_n9, assign12610_e16775_d_n10, assign12610_e16775_d_n11, assign12610_e16775_d_n13, assign12610_e16775_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign12610_e16775;
        locals.var_delta_dn0 = assign12610_e16775_d_n0;
        locals.var_delta_dn2 = assign12610_e16775_d_n2;
        locals.var_delta_dn3 = assign12610_e16775_d_n3;
        locals.var_delta_dn4 = assign12610_e16775_d_n4;
        locals.var_delta_dn5 = assign12610_e16775_d_n5;
        locals.var_delta_dn6 = assign12610_e16775_d_n6;
        locals.var_delta_dn7 = assign12610_e16775_d_n7;
        locals.var_delta_dn8 = assign12610_e16775_d_n8;
        locals.var_delta_dn9 = assign12610_e16775_d_n9;
        locals.var_delta_dn10 = assign12610_e16775_d_n10;
        locals.var_delta_dn11 = assign12610_e16775_d_n11;
        locals.var_delta_dn13 = assign12610_e16775_d_n13;
        locals.var_delta_dn14 = assign12610_e16775_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign12620_e16783, assign12620_e16783_d_n0, assign12620_e16783_d_n2, assign12620_e16783_d_n3, assign12620_e16783_d_n4, assign12620_e16783_d_n5, assign12620_e16783_d_n6, assign12620_e16783_d_n7, assign12620_e16783_d_n8, assign12620_e16783_d_n9, assign12620_e16783_d_n10, assign12620_e16783_d_n11, assign12620_e16783_d_n13, assign12620_e16783_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12620_e16779: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign12620_e16781: f64 = (assign12620_e16779 - locals.var_delta);
        (assign12620_e16781, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12620_e16783;
        locals.var_tt1_dn0 = assign12620_e16783_d_n0;
        locals.var_tt1_dn2 = assign12620_e16783_d_n2;
        locals.var_tt1_dn3 = assign12620_e16783_d_n3;
        locals.var_tt1_dn4 = assign12620_e16783_d_n4;
        locals.var_tt1_dn5 = assign12620_e16783_d_n5;
        locals.var_tt1_dn6 = assign12620_e16783_d_n6;
        locals.var_tt1_dn7 = assign12620_e16783_d_n7;
        locals.var_tt1_dn8 = assign12620_e16783_d_n8;
        locals.var_tt1_dn9 = assign12620_e16783_d_n9;
        locals.var_tt1_dn10 = assign12620_e16783_d_n10;
        locals.var_tt1_dn11 = assign12620_e16783_d_n11;
        locals.var_tt1_dn13 = assign12620_e16783_d_n13;
        locals.var_tt1_dn14 = assign12620_e16783_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12630_e16804, assign12630_e16804_d_n0, assign12630_e16804_d_n2, assign12630_e16804_d_n3, assign12630_e16804_d_n4, assign12630_e16804_d_n5, assign12630_e16804_d_n6, assign12630_e16804_d_n7, assign12630_e16804_d_n8, assign12630_e16804_d_n9, assign12630_e16804_d_n10, assign12630_e16804_d_n11, assign12630_e16804_d_n13, assign12630_e16804_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12630_e16791: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign12630_e16794: f64 = (4.0 * locals.var_delta);
        let assign12630_e16796: f64 = (assign12630_e16794 * locals.var_cfgsat);
        let assign12630_e16797: f64 = (assign12630_e16791 + assign12630_e16796);
        let assign12630_e16798: f64 = (assign12630_e16797).sqrt();
        let assign12630_e16799: f64 = (locals.var_tt1 + assign12630_e16798);
        let assign12630_e16800: f64 = (0.5 * assign12630_e16799);
        let assign12630_e16801: f64 = (locals.var_cfgsat - assign12630_e16800);
        let assign12630_e16802: f64 = (p.p40 * assign12630_e16801);
        (assign12630_e16802, (p.p40 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn0))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn2))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn3))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn4))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn5))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn6))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn7))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn8))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn9))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn10))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn11))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn13))) / (2.0 * assign12630_e16798)))))), (p.p40 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign12630_e16794 * locals.var_cfgsat_dn14))) / (2.0 * assign12630_e16798)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign12630_e16804;
        locals.var_cfg_dn0 = assign12630_e16804_d_n0;
        locals.var_cfg_dn2 = assign12630_e16804_d_n2;
        locals.var_cfg_dn3 = assign12630_e16804_d_n3;
        locals.var_cfg_dn4 = assign12630_e16804_d_n4;
        locals.var_cfg_dn5 = assign12630_e16804_d_n5;
        locals.var_cfg_dn6 = assign12630_e16804_d_n6;
        locals.var_cfg_dn7 = assign12630_e16804_d_n7;
        locals.var_cfg_dn8 = assign12630_e16804_d_n8;
        locals.var_cfg_dn9 = assign12630_e16804_d_n9;
        locals.var_cfg_dn10 = assign12630_e16804_d_n10;
        locals.var_cfg_dn11 = assign12630_e16804_d_n11;
        locals.var_cfg_dn13 = assign12630_e16804_d_n13;
        locals.var_cfg_dn14 = assign12630_e16804_d_n14;
        locals.var_cfg_rv = 0.0;

        let (assign12640_e16810, assign12640_e16810_d_n0, assign12640_e16810_d_n2, assign12640_e16810_d_n3, assign12640_e16810_d_n4, assign12640_e16810_d_n5, assign12640_e16810_d_n6, assign12640_e16810_d_n7, assign12640_e16810_d_n8, assign12640_e16810_d_n9, assign12640_e16810_d_n10, assign12640_e16810_d_n11, assign12640_e16810_d_n13, assign12640_e16810_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12640_e16808: f64 = (locals.var_ccg + locals.var_cfg);
        (assign12640_e16808, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_sidetb, locals.var_cgg_sidetb_dn0, locals.var_cgg_sidetb_dn2, locals.var_cgg_sidetb_dn3, locals.var_cgg_sidetb_dn4, locals.var_cgg_sidetb_dn5, locals.var_cgg_sidetb_dn6, locals.var_cgg_sidetb_dn7, locals.var_cgg_sidetb_dn8, locals.var_cgg_sidetb_dn9, locals.var_cgg_sidetb_dn10, locals.var_cgg_sidetb_dn11, locals.var_cgg_sidetb_dn13, locals.var_cgg_sidetb_dn14,)
    }
};
        locals.var_cgg_sidetb = assign12640_e16810;
        locals.var_cgg_sidetb_dn0 = assign12640_e16810_d_n0;
        locals.var_cgg_sidetb_dn2 = assign12640_e16810_d_n2;
        locals.var_cgg_sidetb_dn3 = assign12640_e16810_d_n3;
        locals.var_cgg_sidetb_dn4 = assign12640_e16810_d_n4;
        locals.var_cgg_sidetb_dn5 = assign12640_e16810_d_n5;
        locals.var_cgg_sidetb_dn6 = assign12640_e16810_d_n6;
        locals.var_cgg_sidetb_dn7 = assign12640_e16810_d_n7;
        locals.var_cgg_sidetb_dn8 = assign12640_e16810_d_n8;
        locals.var_cgg_sidetb_dn9 = assign12640_e16810_d_n9;
        locals.var_cgg_sidetb_dn10 = assign12640_e16810_d_n10;
        locals.var_cgg_sidetb_dn11 = assign12640_e16810_d_n11;
        locals.var_cgg_sidetb_dn13 = assign12640_e16810_d_n13;
        locals.var_cgg_sidetb_dn14 = assign12640_e16810_d_n14;
        locals.var_cgg_sidetb_rv = 0.0;

        let (assign12650_e16822,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12650_e16816: f64 = (locals.var_wg + p.p90);
        let assign12650_e16817: f64 = (0.2 * assign12650_e16816);
        let assign12650_e16819: f64 = (assign12650_e16817 / locals.var_trsd);
        let assign12650_e16820: f64 = (2.3 + assign12650_e16819);
        (assign12650_e16820,)
    } else {
        (locals.var_hr,)
    }
};
        locals.var_hr = assign12650_e16822;
        locals.var_hr_rv = 0.0;

        let (assign12660_e16826,) = {
    if (locals.var_guard213 != 0.0) {
        (1.05,)
    } else {
        (locals.var_lr,)
    }
};
        locals.var_lr = assign12660_e16826;
        locals.var_lr_rv = 0.0;

        let (assign12670_e16835,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12670_e16830: f64 = (locals.var_wg + p.p90);
        let assign12670_e16832: f64 = (assign12670_e16830 - locals.var_trsd);
        let assign12670_e16833: f64 = (assign12670_e16832).abs();
        (assign12670_e16833,)
    } else {
        (locals.var_hgdelta,)
    }
};
        locals.var_hgdelta = assign12670_e16835;
        locals.var_hgdelta_rv = 0.0;

        let (assign12680_e16841,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12680_e16839: f64 = (p.p1087 * locals.var_lr);
        (assign12680_e16839,)
    } else {
        (locals.var_lmax,)
    }
};
        locals.var_lmax = assign12680_e16841;
        locals.var_lmax_rv = 0.0;

        let (assign12690_e16849,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12690_e16846: f64 = (locals.var_wg + p.p90);
        let assign12690_e16847: f64 = (locals.var_trsd).min(assign12690_e16846);
        (assign12690_e16847,)
    } else {
        (locals.var_y,)
    }
};
        locals.var_y = assign12690_e16849;
        locals.var_y_rv = 0.0;

        let (assign12700_e16857,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12700_e16854: f64 = (locals.var_hr + 1.0);
        let assign12700_e16855: f64 = (p.p1087 / assign12700_e16854);
        (assign12700_e16855,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12700_e16857;
        locals.var_x_rv = 0.0;

        let (assign12710_e16861,) = {
    if (locals.var_guard213 != 0.0) {
        (1700000000000.0,)
    } else {
        (locals.var_cnon,)
    }
};
        locals.var_cnon = assign12710_e16861;
        locals.var_cnon_rv = 0.0;

        let (assign12720_e16871,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12720_e16866: f64 = (locals.var_y - locals.var_x);
        let assign12720_e16867: f64 = (locals.var_epssp * assign12720_e16866);
        let assign12720_e16869: f64 = (assign12720_e16867 / p.p1087);
        (assign12720_e16869,)
    } else {
        (locals.var_ccgsat,)
    }
};
        locals.var_ccgsat = assign12720_e16871;
        locals.var_ccgsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12730_e16877, assign12730_e16877_d_n0, assign12730_e16877_d_n2, assign12730_e16877_d_n3, assign12730_e16877_d_n4, assign12730_e16877_d_n5, assign12730_e16877_d_n6, assign12730_e16877_d_n7, assign12730_e16877_d_n8, assign12730_e16877_d_n9, assign12730_e16877_d_n10, assign12730_e16877_d_n11, assign12730_e16877_d_n13, assign12730_e16877_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12730_e16875: f64 = (locals.var_cnon * locals.var_ccgsat);
        (assign12730_e16875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12730_e16877;
        locals.var_tt1_dn0 = assign12730_e16877_d_n0;
        locals.var_tt1_dn2 = assign12730_e16877_d_n2;
        locals.var_tt1_dn3 = assign12730_e16877_d_n3;
        locals.var_tt1_dn4 = assign12730_e16877_d_n4;
        locals.var_tt1_dn5 = assign12730_e16877_d_n5;
        locals.var_tt1_dn6 = assign12730_e16877_d_n6;
        locals.var_tt1_dn7 = assign12730_e16877_d_n7;
        locals.var_tt1_dn8 = assign12730_e16877_d_n8;
        locals.var_tt1_dn9 = assign12730_e16877_d_n9;
        locals.var_tt1_dn10 = assign12730_e16877_d_n10;
        locals.var_tt1_dn11 = assign12730_e16877_d_n11;
        locals.var_tt1_dn13 = assign12730_e16877_d_n13;
        locals.var_tt1_dn14 = assign12730_e16877_d_n14;
        locals.var_tt1_rv = 0.0;

        let assign12740_e16880: f64 = if locals.var_tt1 > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard221 = assign12740_e16880;
        locals.var_guard221_rv = 0.0;

        let (assign12750_e16886, assign12750_e16886_d_n0, assign12750_e16886_d_n2, assign12750_e16886_d_n3, assign12750_e16886_d_n4, assign12750_e16886_d_n5, assign12750_e16886_d_n6, assign12750_e16886_d_n7, assign12750_e16886_d_n8, assign12750_e16886_d_n9, assign12750_e16886_d_n10, assign12750_e16886_d_n11, assign12750_e16886_d_n13, assign12750_e16886_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard221 != 0.0)) {
        (locals.var_ccgsat, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12750_e16886;
        locals.var_ccg1_dn0 = assign12750_e16886_d_n0;
        locals.var_ccg1_dn2 = assign12750_e16886_d_n2;
        locals.var_ccg1_dn3 = assign12750_e16886_d_n3;
        locals.var_ccg1_dn4 = assign12750_e16886_d_n4;
        locals.var_ccg1_dn5 = assign12750_e16886_d_n5;
        locals.var_ccg1_dn6 = assign12750_e16886_d_n6;
        locals.var_ccg1_dn7 = assign12750_e16886_d_n7;
        locals.var_ccg1_dn8 = assign12750_e16886_d_n8;
        locals.var_ccg1_dn9 = assign12750_e16886_d_n9;
        locals.var_ccg1_dn10 = assign12750_e16886_d_n10;
        locals.var_ccg1_dn11 = assign12750_e16886_d_n11;
        locals.var_ccg1_dn13 = assign12750_e16886_d_n13;
        locals.var_ccg1_dn14 = assign12750_e16886_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12760_e16930, assign12760_e16930_d_n0, assign12760_e16930_d_n2, assign12760_e16930_d_n3, assign12760_e16930_d_n4, assign12760_e16930_d_n5, assign12760_e16930_d_n6, assign12760_e16930_d_n7, assign12760_e16930_d_n8, assign12760_e16930_d_n9, assign12760_e16930_d_n10, assign12760_e16930_d_n11, assign12760_e16930_d_n13, assign12760_e16930_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard221 == 0.0)) {
        let assign12760_e16893: f64 = (1.0 / locals.var_cnon);
        let assign12760_e16900: f64 = (-37.0);
        let (assign12760_e16927, assign12760_e16927_d_n0, assign12760_e16927_d_n2, assign12760_e16927_d_n3, assign12760_e16927_d_n4, assign12760_e16927_d_n5, assign12760_e16927_d_n6, assign12760_e16927_d_n7, assign12760_e16927_d_n8, assign12760_e16927_d_n9, assign12760_e16927_d_n10, assign12760_e16927_d_n11, assign12760_e16927_d_n13, assign12760_e16927_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign12760_e16900))) {
                let assign12760_e16906: f64 = (locals.var_tt1).exp();
                let assign12760_e16907: f64 = (1.0 + assign12760_e16906);
                let assign12760_e16908: f64 = (assign12760_e16907).ln();
                (assign12760_e16908, ((assign12760_e16906 * locals.var_tt1_dn0) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn2) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn3) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn4) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn5) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn6) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn7) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn8) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn9) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn10) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn11) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn13) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn14) / assign12760_e16907),)
            } else {
                let assign12760_e16915: f64 = (-37.0);
                let (assign12760_e16926, assign12760_e16926_d_n0, assign12760_e16926_d_n2, assign12760_e16926_d_n3, assign12760_e16926_d_n4, assign12760_e16926_d_n5, assign12760_e16926_d_n6, assign12760_e16926_d_n7, assign12760_e16926_d_n8, assign12760_e16926_d_n9, assign12760_e16926_d_n10, assign12760_e16926_d_n11, assign12760_e16926_d_n13, assign12760_e16926_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign12760_e16915)) {
                        let assign12760_e16919: f64 = (locals.var_tt1).exp();
                        (assign12760_e16919, (assign12760_e16919 * locals.var_tt1_dn0), (assign12760_e16919 * locals.var_tt1_dn2), (assign12760_e16919 * locals.var_tt1_dn3), (assign12760_e16919 * locals.var_tt1_dn4), (assign12760_e16919 * locals.var_tt1_dn5), (assign12760_e16919 * locals.var_tt1_dn6), (assign12760_e16919 * locals.var_tt1_dn7), (assign12760_e16919 * locals.var_tt1_dn8), (assign12760_e16919 * locals.var_tt1_dn9), (assign12760_e16919 * locals.var_tt1_dn10), (assign12760_e16919 * locals.var_tt1_dn11), (assign12760_e16919 * locals.var_tt1_dn13), (assign12760_e16919 * locals.var_tt1_dn14),)
                    } else {
                        let (assign12760_e16925, assign12760_e16925_d_n0, assign12760_e16925_d_n2, assign12760_e16925_d_n3, assign12760_e16925_d_n4, assign12760_e16925_d_n5, assign12760_e16925_d_n6, assign12760_e16925_d_n7, assign12760_e16925_d_n8, assign12760_e16925_d_n9, assign12760_e16925_d_n10, assign12760_e16925_d_n11, assign12760_e16925_d_n13, assign12760_e16925_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign12760_e16925, assign12760_e16925_d_n0, assign12760_e16925_d_n2, assign12760_e16925_d_n3, assign12760_e16925_d_n4, assign12760_e16925_d_n5, assign12760_e16925_d_n6, assign12760_e16925_d_n7, assign12760_e16925_d_n8, assign12760_e16925_d_n9, assign12760_e16925_d_n10, assign12760_e16925_d_n11, assign12760_e16925_d_n13, assign12760_e16925_d_n14,)
                    }
                };
                (assign12760_e16926, assign12760_e16926_d_n0, assign12760_e16926_d_n2, assign12760_e16926_d_n3, assign12760_e16926_d_n4, assign12760_e16926_d_n5, assign12760_e16926_d_n6, assign12760_e16926_d_n7, assign12760_e16926_d_n8, assign12760_e16926_d_n9, assign12760_e16926_d_n10, assign12760_e16926_d_n11, assign12760_e16926_d_n13, assign12760_e16926_d_n14,)
            }
        };
        let assign12760_e16928: f64 = (assign12760_e16893 * assign12760_e16927);
        (assign12760_e16928, (assign12760_e16893 * assign12760_e16927_d_n0), (assign12760_e16893 * assign12760_e16927_d_n2), (assign12760_e16893 * assign12760_e16927_d_n3), (assign12760_e16893 * assign12760_e16927_d_n4), (assign12760_e16893 * assign12760_e16927_d_n5), (assign12760_e16893 * assign12760_e16927_d_n6), (assign12760_e16893 * assign12760_e16927_d_n7), (assign12760_e16893 * assign12760_e16927_d_n8), (assign12760_e16893 * assign12760_e16927_d_n9), (assign12760_e16893 * assign12760_e16927_d_n10), (assign12760_e16893 * assign12760_e16927_d_n11), (assign12760_e16893 * assign12760_e16927_d_n13), (assign12760_e16893 * assign12760_e16927_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12760_e16930;
        locals.var_ccg1_dn0 = assign12760_e16930_d_n0;
        locals.var_ccg1_dn2 = assign12760_e16930_d_n2;
        locals.var_ccg1_dn3 = assign12760_e16930_d_n3;
        locals.var_ccg1_dn4 = assign12760_e16930_d_n4;
        locals.var_ccg1_dn5 = assign12760_e16930_d_n5;
        locals.var_ccg1_dn6 = assign12760_e16930_d_n6;
        locals.var_ccg1_dn7 = assign12760_e16930_d_n7;
        locals.var_ccg1_dn8 = assign12760_e16930_d_n8;
        locals.var_ccg1_dn9 = assign12760_e16930_d_n9;
        locals.var_ccg1_dn10 = assign12760_e16930_d_n10;
        locals.var_ccg1_dn11 = assign12760_e16930_d_n11;
        locals.var_ccg1_dn13 = assign12760_e16930_d_n13;
        locals.var_ccg1_dn14 = assign12760_e16930_d_n14;
        locals.var_ccg1_rv = 0.0;

        let (assign12770_e16946,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12770_e16936: f64 = (locals.var_wg + p.p90);
        let assign12770_e16937: f64 = (locals.var_trsd / assign12770_e16936);
        let assign12770_e16940: f64 = (locals.var_wg + p.p90);
        let assign12770_e16942: f64 = (assign12770_e16940 / locals.var_trsd);
        let assign12770_e16943: f64 = (assign12770_e16937).min(assign12770_e16942);
        let assign12770_e16944: f64 = (0.5 * assign12770_e16943);
        (assign12770_e16944,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign12770_e16946;
        locals.var_r1cf_rv = 0.0;

        let (assign12780_e16952,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12780_e16950: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign12780_e16950,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign12780_e16952;
        locals.var_rcf_rv = 0.0;

        let (assign12790_e16999,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12790_e16956: f64 = (locals.var_epssp * 2.0);
        let assign12790_e16958: f64 = (assign12790_e16956 / 3.141592653589793);
        let assign12790_e16962: f64 = (0.5 * 3.141592653589793);
        let assign12790_e16964: f64 = (assign12790_e16962 * locals.var_rcf);
        let assign12790_e16965: f64 = (p.p1087 + assign12790_e16964);
        let assign12790_e16967: f64 = (assign12790_e16965 / p.p1087);
        let (assign12790_e16996,) = {
            if (!(assign12790_e16967 > 1e-38)) {
                let assign12790_e16972: f64 = (-87.498233534);
                (assign12790_e16972,)
            } else {
                let assign12790_e16976: f64 = (0.5 * 3.141592653589793);
                let assign12790_e16978: f64 = (assign12790_e16976 * locals.var_rcf);
                let assign12790_e16979: f64 = (p.p1087 + assign12790_e16978);
                let assign12790_e16981: f64 = (assign12790_e16979 / p.p1087);
                let (assign12790_e16995,) = {
                    if (assign12790_e16981 > 1e-38) {
                        let assign12790_e16987: f64 = (0.5 * 3.141592653589793);
                        let assign12790_e16989: f64 = (assign12790_e16987 * locals.var_rcf);
                        let assign12790_e16990: f64 = (p.p1087 + assign12790_e16989);
                        let assign12790_e16992: f64 = (assign12790_e16990 / p.p1087);
                        let assign12790_e16993: f64 = (assign12790_e16992).ln();
                        (assign12790_e16993,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12790_e16995,)
            }
        };
        let assign12790_e16997: f64 = (assign12790_e16958 * assign12790_e16996);
        (assign12790_e16997,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign12790_e16999;
        locals.var_ccg2_rv = 0.0;

        let (assign12800_e17007, assign12800_e17007_d_n0, assign12800_e17007_d_n2, assign12800_e17007_d_n3, assign12800_e17007_d_n4, assign12800_e17007_d_n5, assign12800_e17007_d_n6, assign12800_e17007_d_n7, assign12800_e17007_d_n8, assign12800_e17007_d_n9, assign12800_e17007_d_n10, assign12800_e17007_d_n11, assign12800_e17007_d_n13, assign12800_e17007_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12800_e17004: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign12800_e17005: f64 = (p.p42 * assign12800_e17004);
        (assign12800_e17005, (p.p42 * locals.var_ccg1_dn0), (p.p42 * locals.var_ccg1_dn2), (p.p42 * locals.var_ccg1_dn3), (p.p42 * locals.var_ccg1_dn4), (p.p42 * locals.var_ccg1_dn5), (p.p42 * locals.var_ccg1_dn6), (p.p42 * locals.var_ccg1_dn7), (p.p42 * locals.var_ccg1_dn8), (p.p42 * locals.var_ccg1_dn9), (p.p42 * locals.var_ccg1_dn10), (p.p42 * locals.var_ccg1_dn11), (p.p42 * locals.var_ccg1_dn13), (p.p42 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign12800_e17007;
        locals.var_ccg_dn0 = assign12800_e17007_d_n0;
        locals.var_ccg_dn2 = assign12800_e17007_d_n2;
        locals.var_ccg_dn3 = assign12800_e17007_d_n3;
        locals.var_ccg_dn4 = assign12800_e17007_d_n4;
        locals.var_ccg_dn5 = assign12800_e17007_d_n5;
        locals.var_ccg_dn6 = assign12800_e17007_d_n6;
        locals.var_ccg_dn7 = assign12800_e17007_d_n7;
        locals.var_ccg_dn8 = assign12800_e17007_d_n8;
        locals.var_ccg_dn9 = assign12800_e17007_d_n9;
        locals.var_ccg_dn10 = assign12800_e17007_d_n10;
        locals.var_ccg_dn11 = assign12800_e17007_d_n11;
        locals.var_ccg_dn13 = assign12800_e17007_d_n13;
        locals.var_ccg_dn14 = assign12800_e17007_d_n14;
        locals.var_ccg_rv = 0.0;

        let (assign12810_e17013,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12810_e17011: f64 = (locals.var_lmax / locals.var_wg);
        (assign12810_e17011,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12810_e17013;
        locals.var_x_rv = 0.0;

        let (assign12820_e17026,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12820_e17019: f64 = (locals.var_x + 1.0);
        let assign12820_e17020: f64 = (2.0 * assign12820_e17019);
        let assign12820_e17021: f64 = (assign12820_e17020).sqrt();
        let assign12820_e17023: f64 = (assign12820_e17021 * 3.141592653589793);
        let assign12820_e17024: f64 = (4.0 / assign12820_e17023);
        (assign12820_e17024,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign12820_e17026;
        locals.var_c1_rv = 0.0;

        let (assign12830_e17060,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12830_e17030: f64 = (p.p90 * p.p90);
        let assign12830_e17033: f64 = (2.0 * locals.var_wg);
        let assign12830_e17035: f64 = (assign12830_e17033 * p.p90);
        let assign12830_e17036: f64 = (assign12830_e17030 + assign12830_e17035);
        let assign12830_e17039: f64 = (locals.var_wg * locals.var_wg);
        let assign12830_e17042: f64 = (locals.var_x + 1.0);
        let assign12830_e17043: f64 = (assign12830_e17039 * assign12830_e17042);
        let assign12830_e17044: f64 = (assign12830_e17036 + assign12830_e17043);
        let assign12830_e17045: f64 = (assign12830_e17044).sqrt();
        let assign12830_e17048: f64 = (locals.var_x + 1.0);
        let assign12830_e17049: f64 = (assign12830_e17048).sqrt();
        let assign12830_e17050: f64 = (assign12830_e17045 * assign12830_e17049);
        let assign12830_e17052: f64 = (assign12830_e17050 + p.p90);
        let assign12830_e17055: f64 = (locals.var_wg * locals.var_x);
        let assign12830_e17056: f64 = (assign12830_e17052 + assign12830_e17055);
        let assign12830_e17058: f64 = (assign12830_e17056 + locals.var_wg);
        (assign12830_e17058,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign12830_e17060;
        locals.var_c2_rv = 0.0;

        let (assign12840_e17079,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12840_e17065: f64 = (locals.var_x + 1.0);
        let assign12840_e17068: f64 = (locals.var_x + 4.0);
        let assign12840_e17069: f64 = (assign12840_e17065 * assign12840_e17068);
        let assign12840_e17070: f64 = (assign12840_e17069).sqrt();
        let assign12840_e17071: f64 = (p.p90 * assign12840_e17070);
        let assign12840_e17075: f64 = (locals.var_x + 2.0);
        let assign12840_e17076: f64 = (p.p90 * assign12840_e17075);
        let assign12840_e17077: f64 = (assign12840_e17071 + assign12840_e17076);
        (assign12840_e17077,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign12840_e17079;
        locals.var_c3_rv = 0.0;

        let (assign12850_e17108,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12850_e17085: f64 = (locals.var_c2 / locals.var_c3);
        let (assign12850_e17102,) = {
            if (!(assign12850_e17085 > 1e-38)) {
                let assign12850_e17090: f64 = (-87.498233534);
                (assign12850_e17090,)
            } else {
                let assign12850_e17093: f64 = (locals.var_c2 / locals.var_c3);
                let (assign12850_e17101,) = {
                    if (assign12850_e17093 > 1e-38) {
                        let assign12850_e17098: f64 = (locals.var_c2 / locals.var_c3);
                        let assign12850_e17099: f64 = (assign12850_e17098).ln();
                        (assign12850_e17099,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12850_e17101,)
            }
        };
        let assign12850_e17103: f64 = (locals.var_c1 * assign12850_e17102);
        let assign12850_e17105: f64 = (assign12850_e17103 + 12.27);
        let assign12850_e17106: f64 = (locals.var_epssp * assign12850_e17105);
        (assign12850_e17106,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign12850_e17108;
        locals.var_cfglog_rv = 0.0;

        let (assign12860_e17114,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12860_e17112: f64 = (locals.var_hr * locals.var_lr);
        (assign12860_e17112,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign12860_e17114;
        locals.var_dcf_rv = 0.0;

        let (assign12870_e17123,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12870_e17118: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12870_e17120: f64 = (assign12870_e17118 + 1.0);
        let assign12870_e17121: f64 = (assign12870_e17120).sqrt();
        (assign12870_e17121,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign12870_e17123;
        locals.var_tt0_rv = 0.0;

        let (assign12880_e17170, assign12880_e17170_d_n0, assign12880_e17170_d_n2, assign12880_e17170_d_n3, assign12880_e17170_d_n4, assign12880_e17170_d_n5, assign12880_e17170_d_n6, assign12880_e17170_d_n7, assign12880_e17170_d_n8, assign12880_e17170_d_n9, assign12880_e17170_d_n10, assign12880_e17170_d_n11, assign12880_e17170_d_n13, assign12880_e17170_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12880_e17127: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17129: f64 = (assign12880_e17127 + 1.0);
        let assign12880_e17132: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17135: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17136: f64 = (assign12880_e17132 * assign12880_e17135);
        let assign12880_e17139: f64 = (2.0 * locals.var_dcf);
        let assign12880_e17141: f64 = (assign12880_e17139 * locals.var_lmax);
        let assign12880_e17143: f64 = (assign12880_e17141 * p.p90);
        let assign12880_e17144: f64 = (assign12880_e17136 + assign12880_e17143);
        let assign12880_e17147: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17149: f64 = (assign12880_e17147 + 1.0);
        let assign12880_e17151: f64 = (assign12880_e17149 * locals.var_lmax);
        let assign12880_e17153: f64 = (assign12880_e17151 * locals.var_lmax);
        let assign12880_e17154: f64 = (assign12880_e17144 + assign12880_e17153);
        let assign12880_e17155: f64 = (assign12880_e17129 * assign12880_e17154);
        let assign12880_e17156: f64 = (assign12880_e17155).sqrt();
        let assign12880_e17159: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17160: f64 = (assign12880_e17156 + assign12880_e17159);
        let assign12880_e17163: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17165: f64 = (assign12880_e17163 * locals.var_lmax);
        let assign12880_e17166: f64 = (assign12880_e17160 + assign12880_e17165);
        let assign12880_e17168: f64 = (assign12880_e17166 + locals.var_lmax);
        (assign12880_e17168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12880_e17170;
        locals.var_tt1_dn0 = assign12880_e17170_d_n0;
        locals.var_tt1_dn2 = assign12880_e17170_d_n2;
        locals.var_tt1_dn3 = assign12880_e17170_d_n3;
        locals.var_tt1_dn4 = assign12880_e17170_d_n4;
        locals.var_tt1_dn5 = assign12880_e17170_d_n5;
        locals.var_tt1_dn6 = assign12880_e17170_d_n6;
        locals.var_tt1_dn7 = assign12880_e17170_d_n7;
        locals.var_tt1_dn8 = assign12880_e17170_d_n8;
        locals.var_tt1_dn9 = assign12880_e17170_d_n9;
        locals.var_tt1_dn10 = assign12880_e17170_d_n10;
        locals.var_tt1_dn11 = assign12880_e17170_d_n11;
        locals.var_tt1_dn13 = assign12880_e17170_d_n13;
        locals.var_tt1_dn14 = assign12880_e17170_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12890_e17180,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12890_e17174: f64 = (locals.var_tt0 + 1.0);
        let assign12890_e17177: f64 = (locals.var_dcf * p.p90);
        let assign12890_e17178: f64 = (assign12890_e17174 * assign12890_e17177);
        (assign12890_e17178,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign12890_e17180;
        locals.var_tt2_rv = 0.0;

        let (assign12900_e17218, assign12900_e17218_d_n0, assign12900_e17218_d_n2, assign12900_e17218_d_n3, assign12900_e17218_d_n4, assign12900_e17218_d_n5, assign12900_e17218_d_n6, assign12900_e17218_d_n7, assign12900_e17218_d_n8, assign12900_e17218_d_n9, assign12900_e17218_d_n10, assign12900_e17218_d_n11, assign12900_e17218_d_n13, assign12900_e17218_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12900_e17184: f64 = (2.0 * locals.var_epssp);
        let assign12900_e17186: f64 = (2.0_f64).sqrt();
        let assign12900_e17187: f64 = (assign12900_e17184 * assign12900_e17186);
        let assign12900_e17189: f64 = (assign12900_e17187 / 3.141592653589793);
        let assign12900_e17191: f64 = (assign12900_e17189 * 0.85);
        let assign12900_e17193: f64 = (assign12900_e17191 * locals.var_dcf);
        let assign12900_e17195: f64 = (assign12900_e17193 / locals.var_tt0);
        let assign12900_e17198: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign12900_e17215, assign12900_e17215_d_n0, assign12900_e17215_d_n2, assign12900_e17215_d_n3, assign12900_e17215_d_n4, assign12900_e17215_d_n5, assign12900_e17215_d_n6, assign12900_e17215_d_n7, assign12900_e17215_d_n8, assign12900_e17215_d_n9, assign12900_e17215_d_n10, assign12900_e17215_d_n11, assign12900_e17215_d_n13, assign12900_e17215_d_n14,) = {
            if (!(assign12900_e17198 > 1e-38)) {
                let assign12900_e17203: f64 = (-87.498233534);
                (assign12900_e17203, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign12900_e17206: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign12900_e17214, assign12900_e17214_d_n0, assign12900_e17214_d_n2, assign12900_e17214_d_n3, assign12900_e17214_d_n4, assign12900_e17214_d_n5, assign12900_e17214_d_n6, assign12900_e17214_d_n7, assign12900_e17214_d_n8, assign12900_e17214_d_n9, assign12900_e17214_d_n10, assign12900_e17214_d_n11, assign12900_e17214_d_n13, assign12900_e17214_d_n14,) = {
                    if (assign12900_e17206 > 1e-38) {
                        let assign12900_e17211: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign12900_e17212: f64 = (assign12900_e17211).ln();
                        (assign12900_e17212, ((locals.var_tt1_dn0 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn2 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn3 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn4 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn5 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn6 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn7 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn8 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn9 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn10 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn11 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn13 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn14 / locals.var_tt2) / assign12900_e17211),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12900_e17214, assign12900_e17214_d_n0, assign12900_e17214_d_n2, assign12900_e17214_d_n3, assign12900_e17214_d_n4, assign12900_e17214_d_n5, assign12900_e17214_d_n6, assign12900_e17214_d_n7, assign12900_e17214_d_n8, assign12900_e17214_d_n9, assign12900_e17214_d_n10, assign12900_e17214_d_n11, assign12900_e17214_d_n13, assign12900_e17214_d_n14,)
            }
        };
        let assign12900_e17216: f64 = (assign12900_e17195 * assign12900_e17215);
        (assign12900_e17216, (assign12900_e17195 * assign12900_e17215_d_n0), (assign12900_e17195 * assign12900_e17215_d_n2), (assign12900_e17195 * assign12900_e17215_d_n3), (assign12900_e17195 * assign12900_e17215_d_n4), (assign12900_e17195 * assign12900_e17215_d_n5), (assign12900_e17195 * assign12900_e17215_d_n6), (assign12900_e17195 * assign12900_e17215_d_n7), (assign12900_e17195 * assign12900_e17215_d_n8), (assign12900_e17195 * assign12900_e17215_d_n9), (assign12900_e17195 * assign12900_e17215_d_n10), (assign12900_e17195 * assign12900_e17215_d_n11), (assign12900_e17195 * assign12900_e17215_d_n13), (assign12900_e17195 * assign12900_e17215_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign12900_e17218;
        locals.var_cfgsat_dn0 = assign12900_e17218_d_n0;
        locals.var_cfgsat_dn2 = assign12900_e17218_d_n2;
        locals.var_cfgsat_dn3 = assign12900_e17218_d_n3;
        locals.var_cfgsat_dn4 = assign12900_e17218_d_n4;
        locals.var_cfgsat_dn5 = assign12900_e17218_d_n5;
        locals.var_cfgsat_dn6 = assign12900_e17218_d_n6;
        locals.var_cfgsat_dn7 = assign12900_e17218_d_n7;
        locals.var_cfgsat_dn8 = assign12900_e17218_d_n8;
        locals.var_cfgsat_dn9 = assign12900_e17218_d_n9;
        locals.var_cfgsat_dn10 = assign12900_e17218_d_n10;
        locals.var_cfgsat_dn11 = assign12900_e17218_d_n11;
        locals.var_cfgsat_dn13 = assign12900_e17218_d_n13;
        locals.var_cfgsat_dn14 = assign12900_e17218_d_n14;
        locals.var_cfgsat_rv = 0.0;

        let (assign12910_e17222, assign12910_e17222_d_n0, assign12910_e17222_d_n2, assign12910_e17222_d_n3, assign12910_e17222_d_n4, assign12910_e17222_d_n5, assign12910_e17222_d_n6, assign12910_e17222_d_n7, assign12910_e17222_d_n8, assign12910_e17222_d_n9, assign12910_e17222_d_n10, assign12910_e17222_d_n11, assign12910_e17222_d_n13, assign12910_e17222_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign12910_e17222;
        locals.var_delta_dn0 = assign12910_e17222_d_n0;
        locals.var_delta_dn2 = assign12910_e17222_d_n2;
        locals.var_delta_dn3 = assign12910_e17222_d_n3;
        locals.var_delta_dn4 = assign12910_e17222_d_n4;
        locals.var_delta_dn5 = assign12910_e17222_d_n5;
        locals.var_delta_dn6 = assign12910_e17222_d_n6;
        locals.var_delta_dn7 = assign12910_e17222_d_n7;
        locals.var_delta_dn8 = assign12910_e17222_d_n8;
        locals.var_delta_dn9 = assign12910_e17222_d_n9;
        locals.var_delta_dn10 = assign12910_e17222_d_n10;
        locals.var_delta_dn11 = assign12910_e17222_d_n11;
        locals.var_delta_dn13 = assign12910_e17222_d_n13;
        locals.var_delta_dn14 = assign12910_e17222_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign12920_e17230, assign12920_e17230_d_n0, assign12920_e17230_d_n2, assign12920_e17230_d_n3, assign12920_e17230_d_n4, assign12920_e17230_d_n5, assign12920_e17230_d_n6, assign12920_e17230_d_n7, assign12920_e17230_d_n8, assign12920_e17230_d_n9, assign12920_e17230_d_n10, assign12920_e17230_d_n11, assign12920_e17230_d_n13, assign12920_e17230_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12920_e17226: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign12920_e17228: f64 = (assign12920_e17226 - locals.var_delta);
        (assign12920_e17228, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12920_e17230;
        locals.var_tt1_dn0 = assign12920_e17230_d_n0;
        locals.var_tt1_dn2 = assign12920_e17230_d_n2;
        locals.var_tt1_dn3 = assign12920_e17230_d_n3;
        locals.var_tt1_dn4 = assign12920_e17230_d_n4;
        locals.var_tt1_dn5 = assign12920_e17230_d_n5;
        locals.var_tt1_dn6 = assign12920_e17230_d_n6;
        locals.var_tt1_dn7 = assign12920_e17230_d_n7;
        locals.var_tt1_dn8 = assign12920_e17230_d_n8;
        locals.var_tt1_dn9 = assign12920_e17230_d_n9;
        locals.var_tt1_dn10 = assign12920_e17230_d_n10;
        locals.var_tt1_dn11 = assign12920_e17230_d_n11;
        locals.var_tt1_dn13 = assign12920_e17230_d_n13;
        locals.var_tt1_dn14 = assign12920_e17230_d_n14;
        locals.var_tt1_rv = 0.0;

        let (assign12930_e17251, assign12930_e17251_d_n0, assign12930_e17251_d_n2, assign12930_e17251_d_n3, assign12930_e17251_d_n4, assign12930_e17251_d_n5, assign12930_e17251_d_n6, assign12930_e17251_d_n7, assign12930_e17251_d_n8, assign12930_e17251_d_n9, assign12930_e17251_d_n10, assign12930_e17251_d_n11, assign12930_e17251_d_n13, assign12930_e17251_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12930_e17238: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign12930_e17241: f64 = (4.0 * locals.var_delta);
        let assign12930_e17243: f64 = (assign12930_e17241 * locals.var_cfgsat);
        let assign12930_e17244: f64 = (assign12930_e17238 + assign12930_e17243);
        let assign12930_e17245: f64 = (assign12930_e17244).sqrt();
        let assign12930_e17246: f64 = (locals.var_tt1 + assign12930_e17245);
        let assign12930_e17247: f64 = (0.5 * assign12930_e17246);
        let assign12930_e17248: f64 = (locals.var_cfgsat - assign12930_e17247);
        let assign12930_e17249: f64 = (p.p42 * assign12930_e17248);
        (assign12930_e17249, (p.p42 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn0))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn2))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn3))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn4))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn5))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn6))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn7))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn8))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn9))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn10))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn11))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn13))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn14))) / (2.0 * assign12930_e17245)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign12930_e17251;
        locals.var_cfg_dn0 = assign12930_e17251_d_n0;
        locals.var_cfg_dn2 = assign12930_e17251_d_n2;
        locals.var_cfg_dn3 = assign12930_e17251_d_n3;
        locals.var_cfg_dn4 = assign12930_e17251_d_n4;
        locals.var_cfg_dn5 = assign12930_e17251_d_n5;
        locals.var_cfg_dn6 = assign12930_e17251_d_n6;
        locals.var_cfg_dn7 = assign12930_e17251_d_n7;
        locals.var_cfg_dn8 = assign12930_e17251_d_n8;
        locals.var_cfg_dn9 = assign12930_e17251_d_n9;
        locals.var_cfg_dn10 = assign12930_e17251_d_n10;
        locals.var_cfg_dn11 = assign12930_e17251_d_n11;
        locals.var_cfg_dn13 = assign12930_e17251_d_n13;
        locals.var_cfg_dn14 = assign12930_e17251_d_n14;
        locals.var_cfg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign12940_e17257, assign12940_e17257_d_n0, assign12940_e17257_d_n2, assign12940_e17257_d_n3, assign12940_e17257_d_n4, assign12940_e17257_d_n5, assign12940_e17257_d_n6, assign12940_e17257_d_n7, assign12940_e17257_d_n8, assign12940_e17257_d_n9, assign12940_e17257_d_n10, assign12940_e17257_d_n11, assign12940_e17257_d_n13, assign12940_e17257_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12940_e17255: f64 = (locals.var_ccg + locals.var_cfg);
        (assign12940_e17255, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_sidepff, locals.var_cgg_sidepff_dn0, locals.var_cgg_sidepff_dn2, locals.var_cgg_sidepff_dn3, locals.var_cgg_sidepff_dn4, locals.var_cgg_sidepff_dn5, locals.var_cgg_sidepff_dn6, locals.var_cgg_sidepff_dn7, locals.var_cgg_sidepff_dn8, locals.var_cgg_sidepff_dn9, locals.var_cgg_sidepff_dn10, locals.var_cgg_sidepff_dn11, locals.var_cgg_sidepff_dn13, locals.var_cgg_sidepff_dn14,)
    }
};
        locals.var_cgg_sidepff = assign12940_e17257;
        locals.var_cgg_sidepff_dn0 = assign12940_e17257_d_n0;
        locals.var_cgg_sidepff_dn2 = assign12940_e17257_d_n2;
        locals.var_cgg_sidepff_dn3 = assign12940_e17257_d_n3;
        locals.var_cgg_sidepff_dn4 = assign12940_e17257_d_n4;
        locals.var_cgg_sidepff_dn5 = assign12940_e17257_d_n5;
        locals.var_cgg_sidepff_dn6 = assign12940_e17257_d_n6;
        locals.var_cgg_sidepff_dn7 = assign12940_e17257_d_n7;
        locals.var_cgg_sidepff_dn8 = assign12940_e17257_d_n8;
        locals.var_cgg_sidepff_dn9 = assign12940_e17257_d_n9;
        locals.var_cgg_sidepff_dn10 = assign12940_e17257_d_n10;
        locals.var_cgg_sidepff_dn11 = assign12940_e17257_d_n11;
        locals.var_cgg_sidepff_dn13 = assign12940_e17257_d_n13;
        locals.var_cgg_sidepff_dn14 = assign12940_e17257_d_n14;
        locals.var_cgg_sidepff_rv = 0.0;

        let assign12950_e17260: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign12950_e17260;
        locals.var_guard222_rv = 0.0;

        let (assign12960_e17266,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard222 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12960_e17266;
        locals.var_acorner_topm_rv = 0.0;

        let assign12970_e17269: f64 = if p.p1080 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign12970_e17269;
        locals.var_guard223_rv = 0.0;

        let (assign12980_e17286,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard222 == 0.0)) && (locals.var_guard223 != 0.0)) {
        let assign12980_e17278: f64 = (p.p4 - p.p43);
        let assign12980_e17281: f64 = (p.p1080 * p.p1084);
        let assign12980_e17283: f64 = (assign12980_e17281 + p.p1081);
        let assign12980_e17284: f64 = (assign12980_e17278 * assign12980_e17283);
        (assign12980_e17284,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12980_e17286;
        locals.var_acorner_topm_rv = 0.0;

        let (assign12990_e17300,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard222 == 0.0)) && (locals.var_guard223 == 0.0)) {
        let assign12990_e17296: f64 = (p.p4 - p.p43);
        let assign12990_e17298: f64 = (assign12990_e17296 * locals.var_hrsd);
        (assign12990_e17298,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12990_e17300;
        locals.var_acorner_topm_rv = 0.0;

        let (assign13000_e17308,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13000_e17304: f64 = (p.p4 - p.p43);
        let assign13000_e17306: f64 = (assign13000_e17304 * locals.var_hrsd2);
        (assign13000_e17306,)
    } else {
        (locals.var_acorner_tb,)
    }
};
        locals.var_acorner_tb = assign13000_e17308;
        locals.var_acorner_tb_rv = 0.0;

        let (assign13010_e17328,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13010_e17314: f64 = (2.0 * p.p56);
        let assign13010_e17316: f64 = (assign13010_e17314 * locals.var_acorner_tb);
        let assign13010_e17317: f64 = (locals.var_acorner_topm + assign13010_e17316);
        let assign13010_e17318: f64 = (p.p5 * assign13010_e17317);
        let assign13010_e17320: f64 = (assign13010_e17318 + p.p1092);
        let assign13010_e17322: f64 = (assign13010_e17320 + p.p1091);
        let assign13010_e17324: f64 = (assign13010_e17322 * locals.var_epssp);
        let assign13010_e17326: f64 = (assign13010_e17324 / p.p1087);
        (assign13010_e17326,)
    } else {
        (locals.var_ccorner,)
    }
};
        locals.var_ccorner = assign13010_e17328;
        locals.var_ccorner_rv = 0.0;

        let (assign13020_e17360, assign13020_e17360_d_n0, assign13020_e17360_d_n2, assign13020_e17360_d_n3, assign13020_e17360_d_n4, assign13020_e17360_d_n5, assign13020_e17360_d_n6, assign13020_e17360_d_n7, assign13020_e17360_d_n8, assign13020_e17360_d_n9, assign13020_e17360_d_n10, assign13020_e17360_d_n11, assign13020_e17360_d_n13, assign13020_e17360_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13020_e17334: f64 = (2.0 * p.p56);
        let assign13020_e17336: f64 = (assign13020_e17334 * locals.var_cgg_tb);
        let assign13020_e17337: f64 = (locals.var_cgg_topm + assign13020_e17336);
        let assign13020_e17339: f64 = (assign13020_e17337 * p.p5);
        let assign13020_e17340: f64 = (locals.var_ccorner + assign13020_e17339);
        let assign13020_e17346: f64 = (p.p56 - 1.0);
        let assign13020_e17347: f64 = (locals.var_cgg_sidetb * assign13020_e17346);
        let assign13020_e17348: f64 = (locals.var_cgg_sidetopm + assign13020_e17347);
        let assign13020_e17350: f64 = (assign13020_e17348 + locals.var_cgg_sidepff);
        let assign13020_e17351: f64 = (p.p1103 * assign13020_e17350);
        let assign13020_e17353: f64 = (assign13020_e17351 * p.p5);
        let assign13020_e17355: f64 = (assign13020_e17353 * 2.0);
        let assign13020_e17356: f64 = (assign13020_e17340 + assign13020_e17355);
        let assign13020_e17358: f64 = (assign13020_e17356 * p.p59);
        (assign13020_e17358, ((((locals.var_cgg_topm_dn0 + (assign13020_e17334 * locals.var_cgg_tb_dn0)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn0 + (locals.var_cgg_sidetb_dn0 * assign13020_e17346)) + locals.var_cgg_sidepff_dn0)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn2 + (assign13020_e17334 * locals.var_cgg_tb_dn2)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn2 + (locals.var_cgg_sidetb_dn2 * assign13020_e17346)) + locals.var_cgg_sidepff_dn2)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn3 + (assign13020_e17334 * locals.var_cgg_tb_dn3)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn3 + (locals.var_cgg_sidetb_dn3 * assign13020_e17346)) + locals.var_cgg_sidepff_dn3)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn4 + (assign13020_e17334 * locals.var_cgg_tb_dn4)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn4 + (locals.var_cgg_sidetb_dn4 * assign13020_e17346)) + locals.var_cgg_sidepff_dn4)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn5 + (assign13020_e17334 * locals.var_cgg_tb_dn5)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn5 + (locals.var_cgg_sidetb_dn5 * assign13020_e17346)) + locals.var_cgg_sidepff_dn5)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn6 + (assign13020_e17334 * locals.var_cgg_tb_dn6)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn6 + (locals.var_cgg_sidetb_dn6 * assign13020_e17346)) + locals.var_cgg_sidepff_dn6)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn7 + (assign13020_e17334 * locals.var_cgg_tb_dn7)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn7 + (locals.var_cgg_sidetb_dn7 * assign13020_e17346)) + locals.var_cgg_sidepff_dn7)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn8 + (assign13020_e17334 * locals.var_cgg_tb_dn8)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn8 + (locals.var_cgg_sidetb_dn8 * assign13020_e17346)) + locals.var_cgg_sidepff_dn8)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn9 + (assign13020_e17334 * locals.var_cgg_tb_dn9)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn9 + (locals.var_cgg_sidetb_dn9 * assign13020_e17346)) + locals.var_cgg_sidepff_dn9)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn10 + (assign13020_e17334 * locals.var_cgg_tb_dn10)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn10 + (locals.var_cgg_sidetb_dn10 * assign13020_e17346)) + locals.var_cgg_sidepff_dn10)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn11 + (assign13020_e17334 * locals.var_cgg_tb_dn11)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn11 + (locals.var_cgg_sidetb_dn11 * assign13020_e17346)) + locals.var_cgg_sidepff_dn11)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn13 + (assign13020_e17334 * locals.var_cgg_tb_dn13)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn13 + (locals.var_cgg_sidetb_dn13 * assign13020_e17346)) + locals.var_cgg_sidepff_dn13)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn14 + (assign13020_e17334 * locals.var_cgg_tb_dn14)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn14 + (locals.var_cgg_sidetb_dn14 * assign13020_e17346)) + locals.var_cgg_sidepff_dn14)) * p.p5) * 2.0)) * p.p59),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign13020_e17360;
        locals.var_cfr_geo_dn0 = assign13020_e17360_d_n0;
        locals.var_cfr_geo_dn2 = assign13020_e17360_d_n2;
        locals.var_cfr_geo_dn3 = assign13020_e17360_d_n3;
        locals.var_cfr_geo_dn4 = assign13020_e17360_d_n4;
        locals.var_cfr_geo_dn5 = assign13020_e17360_d_n5;
        locals.var_cfr_geo_dn6 = assign13020_e17360_d_n6;
        locals.var_cfr_geo_dn7 = assign13020_e17360_d_n7;
        locals.var_cfr_geo_dn8 = assign13020_e17360_d_n8;
        locals.var_cfr_geo_dn9 = assign13020_e17360_d_n9;
        locals.var_cfr_geo_dn10 = assign13020_e17360_d_n10;
        locals.var_cfr_geo_dn11 = assign13020_e17360_d_n11;
        locals.var_cfr_geo_dn13 = assign13020_e17360_d_n13;
        locals.var_cfr_geo_dn14 = assign13020_e17360_d_n14;
        locals.var_cfr_geo_rv = 0.0;

        let (assign13030_e17380, assign13030_e17380_d_n0, assign13030_e17380_d_n2, assign13030_e17380_d_n3, assign13030_e17380_d_n4, assign13030_e17380_d_n5, assign13030_e17380_d_n6, assign13030_e17380_d_n7, assign13030_e17380_d_n8, assign13030_e17380_d_n9, assign13030_e17380_d_n10, assign13030_e17380_d_n11, assign13030_e17380_d_n13, assign13030_e17380_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13030_e17367: f64 = (p.p1100 * p.p43);
        let assign13030_e17368: f64 = (p.p1099 + assign13030_e17367);
        let assign13030_e17371: f64 = (p.p1101 * p.p4);
        let assign13030_e17372: f64 = (assign13030_e17368 + assign13030_e17371);
        let assign13030_e17375: f64 = (p.p1102 * p.p20);
        let assign13030_e17376: f64 = (assign13030_e17372 + assign13030_e17375);
        let assign13030_e17377: f64 = (0.0_f64).max(assign13030_e17376);
        let assign13030_e17378: f64 = (locals.var_cfr_geo * assign13030_e17377);
        (assign13030_e17378, (locals.var_cfr_geo_dn0 * assign13030_e17377), (locals.var_cfr_geo_dn2 * assign13030_e17377), (locals.var_cfr_geo_dn3 * assign13030_e17377), (locals.var_cfr_geo_dn4 * assign13030_e17377), (locals.var_cfr_geo_dn5 * assign13030_e17377), (locals.var_cfr_geo_dn6 * assign13030_e17377), (locals.var_cfr_geo_dn7 * assign13030_e17377), (locals.var_cfr_geo_dn8 * assign13030_e17377), (locals.var_cfr_geo_dn9 * assign13030_e17377), (locals.var_cfr_geo_dn10 * assign13030_e17377), (locals.var_cfr_geo_dn11 * assign13030_e17377), (locals.var_cfr_geo_dn13 * assign13030_e17377), (locals.var_cfr_geo_dn14 * assign13030_e17377),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign13030_e17380;
        locals.var_cfr_geo_dn0 = assign13030_e17380_d_n0;
        locals.var_cfr_geo_dn2 = assign13030_e17380_d_n2;
        locals.var_cfr_geo_dn3 = assign13030_e17380_d_n3;
        locals.var_cfr_geo_dn4 = assign13030_e17380_d_n4;
        locals.var_cfr_geo_dn5 = assign13030_e17380_d_n5;
        locals.var_cfr_geo_dn6 = assign13030_e17380_d_n6;
        locals.var_cfr_geo_dn7 = assign13030_e17380_d_n7;
        locals.var_cfr_geo_dn8 = assign13030_e17380_d_n8;
        locals.var_cfr_geo_dn9 = assign13030_e17380_d_n9;
        locals.var_cfr_geo_dn10 = assign13030_e17380_d_n10;
        locals.var_cfr_geo_dn11 = assign13030_e17380_d_n11;
        locals.var_cfr_geo_dn13 = assign13030_e17380_d_n13;
        locals.var_cfr_geo_dn14 = assign13030_e17380_d_n14;
        locals.var_cfr_geo_rv = 0.0;

        let assign13040_e17385: f64 = (p.p92 / p.p91);
        let assign13040_e17386: f64 = (1.0 + assign13040_e17385);
        let (assign13040_e17407,) = {
    if (!(assign13040_e17386 > 1e-38)) {
        let assign13040_e17391: f64 = (-87.498233534);
        (assign13040_e17391,)
    } else {
        let assign13040_e17395: f64 = (p.p92 / p.p91);
        let assign13040_e17396: f64 = (1.0 + assign13040_e17395);
        let (assign13040_e17406,) = {
            if (assign13040_e17396 > 1e-38) {
                let assign13040_e17402: f64 = (p.p92 / p.p91);
                let assign13040_e17403: f64 = (1.0 + assign13040_e17402);
                let assign13040_e17404: f64 = (assign13040_e17403).ln();
                (assign13040_e17404,)
            } else {
                (0.0,)
            }
        };
        (assign13040_e17406,)
    }
};
        let assign13040_e17408: f64 = (p.p1583 * assign13040_e17407);
        locals.var_t0 = assign13040_e17408;
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
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13050_e17411: f64 = (locals.var_cbox * p.p7);
        let assign13050_e17417: f64 = (p.p4 * locals.var_nfintotal);
        let assign13050_e17418: f64 = (p.p9 - assign13050_e17417);
        let assign13050_e17419: f64 = (0.0_f64).max(assign13050_e17418);
        let assign13050_e17420: f64 = (locals.var_t0 * assign13050_e17419);
        let assign13050_e17421: f64 = (assign13050_e17411 + assign13050_e17420);
        locals.var_csbox = assign13050_e17421;
        locals.var_csbox_dn0 = (locals.var_t0_dn0 * assign13050_e17419);
        locals.var_csbox_dn2 = (locals.var_t0_dn2 * assign13050_e17419);
        locals.var_csbox_dn3 = (locals.var_t0_dn3 * assign13050_e17419);
        locals.var_csbox_dn4 = (locals.var_t0_dn4 * assign13050_e17419);
        locals.var_csbox_dn5 = (locals.var_t0_dn5 * assign13050_e17419);
        locals.var_csbox_dn6 = (locals.var_t0_dn6 * assign13050_e17419);
        locals.var_csbox_dn7 = (locals.var_t0_dn7 * assign13050_e17419);
        locals.var_csbox_dn8 = (locals.var_t0_dn8 * assign13050_e17419);
        locals.var_csbox_dn9 = (locals.var_t0_dn9 * assign13050_e17419);
        locals.var_csbox_dn10 = (locals.var_t0_dn10 * assign13050_e17419);
        locals.var_csbox_dn11 = (locals.var_t0_dn11 * assign13050_e17419);
        locals.var_csbox_dn13 = (locals.var_t0_dn13 * assign13050_e17419);
        locals.var_csbox_dn14 = (locals.var_t0_dn14 * assign13050_e17419);
        locals.var_csbox_rv = 0.0;

        let assign13060_e17424: f64 = (locals.var_cbox * p.p8);
        let assign13060_e17430: f64 = (p.p4 * locals.var_nfintotal);
        let assign13060_e17431: f64 = (p.p10 - assign13060_e17430);
        let assign13060_e17432: f64 = (0.0_f64).max(assign13060_e17431);
        let assign13060_e17433: f64 = (locals.var_t0 * assign13060_e17432);
        let assign13060_e17434: f64 = (assign13060_e17424 + assign13060_e17433);
        locals.var_cdbox = assign13060_e17434;
        locals.var_cdbox_dn0 = (locals.var_t0_dn0 * assign13060_e17432);
        locals.var_cdbox_dn2 = (locals.var_t0_dn2 * assign13060_e17432);
        locals.var_cdbox_dn3 = (locals.var_t0_dn3 * assign13060_e17432);
        locals.var_cdbox_dn4 = (locals.var_t0_dn4 * assign13060_e17432);
        locals.var_cdbox_dn5 = (locals.var_t0_dn5 * assign13060_e17432);
        locals.var_cdbox_dn6 = (locals.var_t0_dn6 * assign13060_e17432);
        locals.var_cdbox_dn7 = (locals.var_t0_dn7 * assign13060_e17432);
        locals.var_cdbox_dn8 = (locals.var_t0_dn8 * assign13060_e17432);
        locals.var_cdbox_dn9 = (locals.var_t0_dn9 * assign13060_e17432);
        locals.var_cdbox_dn10 = (locals.var_t0_dn10 * assign13060_e17432);
        locals.var_cdbox_dn11 = (locals.var_t0_dn11 * assign13060_e17432);
        locals.var_cdbox_dn13 = (locals.var_t0_dn13 * assign13060_e17432);
        locals.var_cdbox_dn14 = (locals.var_t0_dn14 * assign13060_e17432);
        locals.var_cdbox_rv = 0.0;

        let assign13070_e17437: f64 = if p.p62 != 5.0 { 1.0 } else { 0.0 };
        locals.var_guard224 = assign13070_e17437;
        locals.var_guard224_rv = 0.0;

        let (assign13080_e17451,) = {
    if (locals.var_guard224 != 0.0) {
        let assign13080_e17441: f64 = (p.p1544 * p.p59);
        let assign13080_e17443: f64 = (assign13080_e17441 * p.p6);
        let assign13080_e17446: f64 = (p.p1545 * locals.var_nfintotal);
        let assign13080_e17447: f64 = (assign13080_e17443 + assign13080_e17446);
        let assign13080_e17449: f64 = (assign13080_e17447 * locals.var_lg);
        (assign13080_e17449,)
    } else {
        (locals.var_cgbox,)
    }
};
        locals.var_cgbox = assign13080_e17451;
        locals.var_cgbox_rv = 0.0;

        let (assign13090_e17470,) = {
    if (locals.var_guard224 == 0.0) {
        let assign13090_e17456: f64 = (p.p1544 * p.p59);
        let assign13090_e17458: f64 = (assign13090_e17456 * p.p6);
        let assign13090_e17462: f64 = (p.p1546 * locals.var_wgaaeff);
        let assign13090_e17463: f64 = (p.p1545 + assign13090_e17462);
        let assign13090_e17465: f64 = (assign13090_e17463 * locals.var_nfintotal);
        let assign13090_e17466: f64 = (assign13090_e17458 + assign13090_e17465);
        let assign13090_e17468: f64 = (assign13090_e17466 * locals.var_lg);
        (assign13090_e17468,)
    } else {
        (locals.var_cgbox,)
    }
};
        locals.var_cgbox = assign13090_e17470;
        locals.var_cgbox_rv = 0.0;

        let assign13100_e17474: f64 = (locals.var_epsratio * p.p89);
        let assign13100_e17475: f64 = (1e-8 / assign13100_e17474);
        locals.var_eefffactor = assign13100_e17475;
        locals.var_eefffactor_rv = 0.0;

        let assign13110_e17480: f64 = (locals.var_weff0 * 1000000.0);
        let assign13110_e17482: f64 = (assign13110_e17480).powf(locals.var_wr_i);
        let assign13110_e17483: f64 = (locals.var_nfintotal * assign13110_e17482);
        let assign13110_e17484: f64 = (1.0 / assign13110_e17483);
        locals.var_weffwrfactor = assign13110_e17484;
        locals.var_weffwrfactor_rv = 0.0;

        let assign13120_e17487: f64 = (locals.var_epsratio * p.p89);
        let assign13120_e17489: f64 = (assign13120_e17487 * 0.5);
        let assign13120_e17491: f64 = (assign13120_e17489 * p.p3);
        let assign13120_e17492: f64 = (assign13120_e17491).sqrt();
        locals.var_litl = assign13120_e17492;
        locals.var_litl_rv = 0.0;

        let assign13130_e17495: f64 = (locals.var_epssub * locals.var_ach);
        let assign13130_e17497: f64 = (assign13130_e17495 / locals.var_cins);
        let assign13130_e17501: f64 = (locals.var_ach * locals.var_cins);
        let assign13130_e17504: f64 = (2.0 * locals.var_epssub);
        let assign13130_e17506: f64 = (assign13130_e17504 * locals.var_weff_ufcm);
        let assign13130_e17508: f64 = (assign13130_e17506 * locals.var_weff_ufcm);
        let assign13130_e17509: f64 = (assign13130_e17501 / assign13130_e17508);
        let assign13130_e17510: f64 = (1.0 + assign13130_e17509);
        let assign13130_e17511: f64 = (assign13130_e17497 * assign13130_e17510);
        let assign13130_e17512: f64 = (assign13130_e17511).sqrt();
        locals.var_scl = assign13130_e17512;
        locals.var_scl_rv = 0.0;

        let assign13140_e17515: f64 = if (!param_given[172]) { 1.0 } else { 0.0 };
        locals.var_guard225 = assign13140_e17515;
        locals.var_guard225_rv = 0.0;

        let (assign13150_e17525, assign13150_e17525_d_n0, assign13150_e17525_d_n2, assign13150_e17525_d_n3, assign13150_e17525_d_n4, assign13150_e17525_d_n5, assign13150_e17525_d_n6, assign13150_e17525_d_n7, assign13150_e17525_d_n8, assign13150_e17525_d_n9, assign13150_e17525_d_n10, assign13150_e17525_d_n11, assign13150_e17525_d_n13, assign13150_e17525_d_n14,) = {
    if (locals.var_guard225 != 0.0) {
        let assign13150_e17519: f64 = (locals.var_dvt1_i * locals.var_leff_1);
        let assign13150_e17521: f64 = (assign13150_e17519 / locals.var_scl);
        let assign13150_e17523: f64 = (assign13150_e17521 + 1e-6);
        (assign13150_e17523, ((locals.var_dvt1_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13150_e17525;
        locals.var_tmp_dn0 = assign13150_e17525_d_n0;
        locals.var_tmp_dn2 = assign13150_e17525_d_n2;
        locals.var_tmp_dn3 = assign13150_e17525_d_n3;
        locals.var_tmp_dn4 = assign13150_e17525_d_n4;
        locals.var_tmp_dn5 = assign13150_e17525_d_n5;
        locals.var_tmp_dn6 = assign13150_e17525_d_n6;
        locals.var_tmp_dn7 = assign13150_e17525_d_n7;
        locals.var_tmp_dn8 = assign13150_e17525_d_n8;
        locals.var_tmp_dn9 = assign13150_e17525_d_n9;
        locals.var_tmp_dn10 = assign13150_e17525_d_n10;
        locals.var_tmp_dn11 = assign13150_e17525_d_n11;
        locals.var_tmp_dn13 = assign13150_e17525_d_n13;
        locals.var_tmp_dn14 = assign13150_e17525_d_n14;
        locals.var_tmp_rv = 0.0;

        let assign13160_e17528: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign13160_e17528;
        locals.var_guard226_rv = 0.0;

        let (assign13170_e17539, assign13170_e17539_d_n0, assign13170_e17539_d_n2, assign13170_e17539_d_n3, assign13170_e17539_d_n4, assign13170_e17539_d_n5, assign13170_e17539_d_n6, assign13170_e17539_d_n7, assign13170_e17539_d_n8, assign13170_e17539_d_n9, assign13170_e17539_d_n10, assign13170_e17539_d_n11, assign13170_e17539_d_n13, assign13170_e17539_d_n14,) = {
    if ((locals.var_guard225 != 0.0) && (locals.var_guard226 != 0.0)) {
        let assign13170_e17534: f64 = (locals.var_tmp).cosh();
        let assign13170_e17536: f64 = (assign13170_e17534 - 1.0);
        let assign13170_e17537: f64 = (0.5 / assign13170_e17536);
        (assign13170_e17537, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13170_e17536 * assign13170_e17536))),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13170_e17539;
        locals.var_theta_sce_dn0 = assign13170_e17539_d_n0;
        locals.var_theta_sce_dn2 = assign13170_e17539_d_n2;
        locals.var_theta_sce_dn3 = assign13170_e17539_d_n3;
        locals.var_theta_sce_dn4 = assign13170_e17539_d_n4;
        locals.var_theta_sce_dn5 = assign13170_e17539_d_n5;
        locals.var_theta_sce_dn6 = assign13170_e17539_d_n6;
        locals.var_theta_sce_dn7 = assign13170_e17539_d_n7;
        locals.var_theta_sce_dn8 = assign13170_e17539_d_n8;
        locals.var_theta_sce_dn9 = assign13170_e17539_d_n9;
        locals.var_theta_sce_dn10 = assign13170_e17539_d_n10;
        locals.var_theta_sce_dn11 = assign13170_e17539_d_n11;
        locals.var_theta_sce_dn13 = assign13170_e17539_d_n13;
        locals.var_theta_sce_dn14 = assign13170_e17539_d_n14;
        locals.var_theta_sce_rv = 0.0;

        let (assign13180_e17548, assign13180_e17548_d_n0, assign13180_e17548_d_n2, assign13180_e17548_d_n3, assign13180_e17548_d_n4, assign13180_e17548_d_n5, assign13180_e17548_d_n6, assign13180_e17548_d_n7, assign13180_e17548_d_n8, assign13180_e17548_d_n9, assign13180_e17548_d_n10, assign13180_e17548_d_n11, assign13180_e17548_d_n13, assign13180_e17548_d_n14,) = {
    if ((locals.var_guard225 != 0.0) && (locals.var_guard226 == 0.0)) {
        let assign13180_e17545: f64 = (-locals.var_tmp);
        let assign13180_e17546: f64 = { let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13180_e17546, ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13180_e17548;
        locals.var_theta_sce_dn0 = assign13180_e17548_d_n0;
        locals.var_theta_sce_dn2 = assign13180_e17548_d_n2;
        locals.var_theta_sce_dn3 = assign13180_e17548_d_n3;
        locals.var_theta_sce_dn4 = assign13180_e17548_d_n4;
        locals.var_theta_sce_dn5 = assign13180_e17548_d_n5;
        locals.var_theta_sce_dn6 = assign13180_e17548_d_n6;
        locals.var_theta_sce_dn7 = assign13180_e17548_d_n7;
        locals.var_theta_sce_dn8 = assign13180_e17548_d_n8;
        locals.var_theta_sce_dn9 = assign13180_e17548_d_n9;
        locals.var_theta_sce_dn10 = assign13180_e17548_d_n10;
        locals.var_theta_sce_dn11 = assign13180_e17548_d_n11;
        locals.var_theta_sce_dn13 = assign13180_e17548_d_n13;
        locals.var_theta_sce_dn14 = assign13180_e17548_d_n14;
        locals.var_theta_sce_rv = 0.0;

        let (assign13190_e17553, assign13190_e17553_d_n0, assign13190_e17553_d_n2, assign13190_e17553_d_n3, assign13190_e17553_d_n4, assign13190_e17553_d_n5, assign13190_e17553_d_n6, assign13190_e17553_d_n7, assign13190_e17553_d_n8, assign13190_e17553_d_n9, assign13190_e17553_d_n10, assign13190_e17553_d_n11, assign13190_e17553_d_n13, assign13190_e17553_d_n14,) = {
    if (locals.var_guard225 == 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13190_e17553;
        locals.var_theta_sce_dn0 = assign13190_e17553_d_n0;
        locals.var_theta_sce_dn2 = assign13190_e17553_d_n2;
        locals.var_theta_sce_dn3 = assign13190_e17553_d_n3;
        locals.var_theta_sce_dn4 = assign13190_e17553_d_n4;
        locals.var_theta_sce_dn5 = assign13190_e17553_d_n5;
        locals.var_theta_sce_dn6 = assign13190_e17553_d_n6;
        locals.var_theta_sce_dn7 = assign13190_e17553_d_n7;
        locals.var_theta_sce_dn8 = assign13190_e17553_d_n8;
        locals.var_theta_sce_dn9 = assign13190_e17553_d_n9;
        locals.var_theta_sce_dn10 = assign13190_e17553_d_n10;
        locals.var_theta_sce_dn11 = assign13190_e17553_d_n11;
        locals.var_theta_sce_dn13 = assign13190_e17553_d_n13;
        locals.var_theta_sce_dn14 = assign13190_e17553_d_n14;
        locals.var_theta_sce_rv = 0.0;

        let assign13200_e17556: f64 = if (!param_given[174]) { 1.0 } else { 0.0 };
        locals.var_guard227 = assign13200_e17556;
        locals.var_guard227_rv = 0.0;

        let (assign13210_e17566, assign13210_e17566_d_n0, assign13210_e17566_d_n2, assign13210_e17566_d_n3, assign13210_e17566_d_n4, assign13210_e17566_d_n5, assign13210_e17566_d_n6, assign13210_e17566_d_n7, assign13210_e17566_d_n8, assign13210_e17566_d_n9, assign13210_e17566_d_n10, assign13210_e17566_d_n11, assign13210_e17566_d_n13, assign13210_e17566_d_n14,) = {
    if (locals.var_guard227 != 0.0) {
        let assign13210_e17560: f64 = (locals.var_dvt1ss_i * locals.var_leff_1);
        let assign13210_e17562: f64 = (assign13210_e17560 / locals.var_scl);
        let assign13210_e17564: f64 = (assign13210_e17562 + 1e-6);
        (assign13210_e17564, ((locals.var_dvt1ss_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13210_e17566;
        locals.var_tmp_dn0 = assign13210_e17566_d_n0;
        locals.var_tmp_dn2 = assign13210_e17566_d_n2;
        locals.var_tmp_dn3 = assign13210_e17566_d_n3;
        locals.var_tmp_dn4 = assign13210_e17566_d_n4;
        locals.var_tmp_dn5 = assign13210_e17566_d_n5;
        locals.var_tmp_dn6 = assign13210_e17566_d_n6;
        locals.var_tmp_dn7 = assign13210_e17566_d_n7;
        locals.var_tmp_dn8 = assign13210_e17566_d_n8;
        locals.var_tmp_dn9 = assign13210_e17566_d_n9;
        locals.var_tmp_dn10 = assign13210_e17566_d_n10;
        locals.var_tmp_dn11 = assign13210_e17566_d_n11;
        locals.var_tmp_dn13 = assign13210_e17566_d_n13;
        locals.var_tmp_dn14 = assign13210_e17566_d_n14;
        locals.var_tmp_rv = 0.0;

        let assign13220_e17569: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign13220_e17569;
        locals.var_guard228_rv = 0.0;

        let (assign13230_e17580, assign13230_e17580_d_n0, assign13230_e17580_d_n2, assign13230_e17580_d_n3, assign13230_e17580_d_n4, assign13230_e17580_d_n5, assign13230_e17580_d_n6, assign13230_e17580_d_n7, assign13230_e17580_d_n8, assign13230_e17580_d_n9, assign13230_e17580_d_n10, assign13230_e17580_d_n11, assign13230_e17580_d_n13, assign13230_e17580_d_n14,) = {
    if ((locals.var_guard227 != 0.0) && (locals.var_guard228 != 0.0)) {
        let assign13230_e17575: f64 = (locals.var_tmp).cosh();
        let assign13230_e17577: f64 = (assign13230_e17575 - 1.0);
        let assign13230_e17578: f64 = (0.5 / assign13230_e17577);
        (assign13230_e17578, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13230_e17577 * assign13230_e17577))),)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13230_e17580;
        locals.var_theta_sw_dn0 = assign13230_e17580_d_n0;
        locals.var_theta_sw_dn2 = assign13230_e17580_d_n2;
        locals.var_theta_sw_dn3 = assign13230_e17580_d_n3;
        locals.var_theta_sw_dn4 = assign13230_e17580_d_n4;
        locals.var_theta_sw_dn5 = assign13230_e17580_d_n5;
        locals.var_theta_sw_dn6 = assign13230_e17580_d_n6;
        locals.var_theta_sw_dn7 = assign13230_e17580_d_n7;
        locals.var_theta_sw_dn8 = assign13230_e17580_d_n8;
        locals.var_theta_sw_dn9 = assign13230_e17580_d_n9;
        locals.var_theta_sw_dn10 = assign13230_e17580_d_n10;
        locals.var_theta_sw_dn11 = assign13230_e17580_d_n11;
        locals.var_theta_sw_dn13 = assign13230_e17580_d_n13;
        locals.var_theta_sw_dn14 = assign13230_e17580_d_n14;
        locals.var_theta_sw_rv = 0.0;

        let (assign13240_e17589, assign13240_e17589_d_n0, assign13240_e17589_d_n2, assign13240_e17589_d_n3, assign13240_e17589_d_n4, assign13240_e17589_d_n5, assign13240_e17589_d_n6, assign13240_e17589_d_n7, assign13240_e17589_d_n8, assign13240_e17589_d_n9, assign13240_e17589_d_n10, assign13240_e17589_d_n11, assign13240_e17589_d_n13, assign13240_e17589_d_n14,) = {
    if ((locals.var_guard227 != 0.0) && (locals.var_guard228 == 0.0)) {
        let assign13240_e17586: f64 = (-locals.var_tmp);
        let assign13240_e17587: f64 = { let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13240_e17587, ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13240_e17589;
        locals.var_theta_sw_dn0 = assign13240_e17589_d_n0;
        locals.var_theta_sw_dn2 = assign13240_e17589_d_n2;
        locals.var_theta_sw_dn3 = assign13240_e17589_d_n3;
        locals.var_theta_sw_dn4 = assign13240_e17589_d_n4;
        locals.var_theta_sw_dn5 = assign13240_e17589_d_n5;
        locals.var_theta_sw_dn6 = assign13240_e17589_d_n6;
        locals.var_theta_sw_dn7 = assign13240_e17589_d_n7;
        locals.var_theta_sw_dn8 = assign13240_e17589_d_n8;
        locals.var_theta_sw_dn9 = assign13240_e17589_d_n9;
        locals.var_theta_sw_dn10 = assign13240_e17589_d_n10;
        locals.var_theta_sw_dn11 = assign13240_e17589_d_n11;
        locals.var_theta_sw_dn13 = assign13240_e17589_d_n13;
        locals.var_theta_sw_dn14 = assign13240_e17589_d_n14;
        locals.var_theta_sw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign13250_e17594, assign13250_e17594_d_n0, assign13250_e17594_d_n2, assign13250_e17594_d_n3, assign13250_e17594_d_n4, assign13250_e17594_d_n5, assign13250_e17594_d_n6, assign13250_e17594_d_n7, assign13250_e17594_d_n8, assign13250_e17594_d_n9, assign13250_e17594_d_n10, assign13250_e17594_d_n11, assign13250_e17594_d_n13, assign13250_e17594_d_n14,) = {
    if (locals.var_guard227 == 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13250_e17594;
        locals.var_theta_sw_dn0 = assign13250_e17594_d_n0;
        locals.var_theta_sw_dn2 = assign13250_e17594_d_n2;
        locals.var_theta_sw_dn3 = assign13250_e17594_d_n3;
        locals.var_theta_sw_dn4 = assign13250_e17594_d_n4;
        locals.var_theta_sw_dn5 = assign13250_e17594_d_n5;
        locals.var_theta_sw_dn6 = assign13250_e17594_d_n6;
        locals.var_theta_sw_dn7 = assign13250_e17594_d_n7;
        locals.var_theta_sw_dn8 = assign13250_e17594_d_n8;
        locals.var_theta_sw_dn9 = assign13250_e17594_d_n9;
        locals.var_theta_sw_dn10 = assign13250_e17594_d_n10;
        locals.var_theta_sw_dn11 = assign13250_e17594_d_n11;
        locals.var_theta_sw_dn13 = assign13250_e17594_d_n13;
        locals.var_theta_sw_dn14 = assign13250_e17594_d_n14;
        locals.var_theta_sw_rv = 0.0;

        let assign13260_e17597: f64 = if (!param_given[173]) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign13260_e17597;
        locals.var_guard229_rv = 0.0;

        let (assign13270_e17607, assign13270_e17607_d_n0, assign13270_e17607_d_n2, assign13270_e17607_d_n3, assign13270_e17607_d_n4, assign13270_e17607_d_n5, assign13270_e17607_d_n6, assign13270_e17607_d_n7, assign13270_e17607_d_n8, assign13270_e17607_d_n9, assign13270_e17607_d_n10, assign13270_e17607_d_n11, assign13270_e17607_d_n13, assign13270_e17607_d_n14,) = {
    if (locals.var_guard229 != 0.0) {
        let assign13270_e17601: f64 = (locals.var_dsub_i * locals.var_leff_1);
        let assign13270_e17603: f64 = (assign13270_e17601 / locals.var_scl);
        let assign13270_e17605: f64 = (assign13270_e17603 + 1e-6);
        (assign13270_e17605, ((locals.var_dsub_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13270_e17607;
        locals.var_tmp_dn0 = assign13270_e17607_d_n0;
        locals.var_tmp_dn2 = assign13270_e17607_d_n2;
        locals.var_tmp_dn3 = assign13270_e17607_d_n3;
        locals.var_tmp_dn4 = assign13270_e17607_d_n4;
        locals.var_tmp_dn5 = assign13270_e17607_d_n5;
        locals.var_tmp_dn6 = assign13270_e17607_d_n6;
        locals.var_tmp_dn7 = assign13270_e17607_d_n7;
        locals.var_tmp_dn8 = assign13270_e17607_d_n8;
        locals.var_tmp_dn9 = assign13270_e17607_d_n9;
        locals.var_tmp_dn10 = assign13270_e17607_d_n10;
        locals.var_tmp_dn11 = assign13270_e17607_d_n11;
        locals.var_tmp_dn13 = assign13270_e17607_d_n13;
        locals.var_tmp_dn14 = assign13270_e17607_d_n14;
        locals.var_tmp_rv = 0.0;

        let assign13280_e17610: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign13280_e17610;
        locals.var_guard230_rv = 0.0;

        let (assign13290_e17621, assign13290_e17621_d_n0, assign13290_e17621_d_n2, assign13290_e17621_d_n3, assign13290_e17621_d_n4, assign13290_e17621_d_n5, assign13290_e17621_d_n6, assign13290_e17621_d_n7, assign13290_e17621_d_n8, assign13290_e17621_d_n9, assign13290_e17621_d_n10, assign13290_e17621_d_n11, assign13290_e17621_d_n13, assign13290_e17621_d_n14,) = {
    if ((locals.var_guard229 != 0.0) && (locals.var_guard230 != 0.0)) {
        let assign13290_e17616: f64 = (locals.var_tmp).cosh();
        let assign13290_e17618: f64 = (assign13290_e17616 - 1.0);
        let assign13290_e17619: f64 = (0.5 / assign13290_e17618);
        (assign13290_e17619, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13290_e17618 * assign13290_e17618))),)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13290_e17621;
        locals.var_theta_dibl_dn0 = assign13290_e17621_d_n0;
        locals.var_theta_dibl_dn2 = assign13290_e17621_d_n2;
        locals.var_theta_dibl_dn3 = assign13290_e17621_d_n3;
        locals.var_theta_dibl_dn4 = assign13290_e17621_d_n4;
        locals.var_theta_dibl_dn5 = assign13290_e17621_d_n5;
        locals.var_theta_dibl_dn6 = assign13290_e17621_d_n6;
        locals.var_theta_dibl_dn7 = assign13290_e17621_d_n7;
        locals.var_theta_dibl_dn8 = assign13290_e17621_d_n8;
        locals.var_theta_dibl_dn9 = assign13290_e17621_d_n9;
        locals.var_theta_dibl_dn10 = assign13290_e17621_d_n10;
        locals.var_theta_dibl_dn11 = assign13290_e17621_d_n11;
        locals.var_theta_dibl_dn13 = assign13290_e17621_d_n13;
        locals.var_theta_dibl_dn14 = assign13290_e17621_d_n14;
        locals.var_theta_dibl_rv = 0.0;

        let (assign13300_e17630, assign13300_e17630_d_n0, assign13300_e17630_d_n2, assign13300_e17630_d_n3, assign13300_e17630_d_n4, assign13300_e17630_d_n5, assign13300_e17630_d_n6, assign13300_e17630_d_n7, assign13300_e17630_d_n8, assign13300_e17630_d_n9, assign13300_e17630_d_n10, assign13300_e17630_d_n11, assign13300_e17630_d_n13, assign13300_e17630_d_n14,) = {
    if ((locals.var_guard229 != 0.0) && (locals.var_guard230 == 0.0)) {
        let assign13300_e17627: f64 = (-locals.var_tmp);
        let assign13300_e17628: f64 = { let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13300_e17628, ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13300_e17630;
        locals.var_theta_dibl_dn0 = assign13300_e17630_d_n0;
        locals.var_theta_dibl_dn2 = assign13300_e17630_d_n2;
        locals.var_theta_dibl_dn3 = assign13300_e17630_d_n3;
        locals.var_theta_dibl_dn4 = assign13300_e17630_d_n4;
        locals.var_theta_dibl_dn5 = assign13300_e17630_d_n5;
        locals.var_theta_dibl_dn6 = assign13300_e17630_d_n6;
        locals.var_theta_dibl_dn7 = assign13300_e17630_d_n7;
        locals.var_theta_dibl_dn8 = assign13300_e17630_d_n8;
        locals.var_theta_dibl_dn9 = assign13300_e17630_d_n9;
        locals.var_theta_dibl_dn10 = assign13300_e17630_d_n10;
        locals.var_theta_dibl_dn11 = assign13300_e17630_d_n11;
        locals.var_theta_dibl_dn13 = assign13300_e17630_d_n13;
        locals.var_theta_dibl_dn14 = assign13300_e17630_d_n14;
        locals.var_theta_dibl_rv = 0.0;

        let (assign13310_e17635, assign13310_e17635_d_n0, assign13310_e17635_d_n2, assign13310_e17635_d_n3, assign13310_e17635_d_n4, assign13310_e17635_d_n5, assign13310_e17635_d_n6, assign13310_e17635_d_n7, assign13310_e17635_d_n8, assign13310_e17635_d_n9, assign13310_e17635_d_n10, assign13310_e17635_d_n11, assign13310_e17635_d_n13, assign13310_e17635_d_n14,) = {
    if (locals.var_guard229 == 0.0) {
        (p.p173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13310_e17635;
        locals.var_theta_dibl_dn0 = assign13310_e17635_d_n0;
        locals.var_theta_dibl_dn2 = assign13310_e17635_d_n2;
        locals.var_theta_dibl_dn3 = assign13310_e17635_d_n3;
        locals.var_theta_dibl_dn4 = assign13310_e17635_d_n4;
        locals.var_theta_dibl_dn5 = assign13310_e17635_d_n5;
        locals.var_theta_dibl_dn6 = assign13310_e17635_d_n6;
        locals.var_theta_dibl_dn7 = assign13310_e17635_d_n7;
        locals.var_theta_dibl_dn8 = assign13310_e17635_d_n8;
        locals.var_theta_dibl_dn9 = assign13310_e17635_d_n9;
        locals.var_theta_dibl_dn10 = assign13310_e17635_d_n10;
        locals.var_theta_dibl_dn11 = assign13310_e17635_d_n11;
        locals.var_theta_dibl_dn13 = assign13310_e17635_d_n13;
        locals.var_theta_dibl_dn14 = assign13310_e17635_d_n14;
        locals.var_theta_dibl_rv = 0.0;

        let assign13320_e17639: f64 = (locals.var_lpe0_i / locals.var_leff_1);
        let assign13320_e17640: f64 = (1.0 + assign13320_e17639);
        let assign13320_e17641: f64 = (assign13320_e17640).sqrt();
        let assign13320_e17643: f64 = (assign13320_e17641 - 1.0);
        locals.var_theta_rsce = assign13320_e17643;
        locals.var_theta_rsce_dn0 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn2 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn3 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn4 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn5 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn6 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn7 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn8 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn9 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn10 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn11 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn13 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn14 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_rv = 0.0;

        let assign13330_e17646: f64 = (locals.var_dsub_i * locals.var_leff_1);
        let assign13330_e17648: f64 = (assign13330_e17646 / locals.var_scl);
        let assign13330_e17650: f64 = (assign13330_e17648 + 1e-6);
        locals.var_tmp = assign13330_e17650;
        locals.var_tmp_dn0 = ((locals.var_dsub_i * locals.var_leff_1_dn0) / locals.var_scl);
        locals.var_tmp_dn2 = ((locals.var_dsub_i * locals.var_leff_1_dn2) / locals.var_scl);
        locals.var_tmp_dn3 = ((locals.var_dsub_i * locals.var_leff_1_dn3) / locals.var_scl);
        locals.var_tmp_dn4 = ((locals.var_dsub_i * locals.var_leff_1_dn4) / locals.var_scl);
        locals.var_tmp_dn5 = ((locals.var_dsub_i * locals.var_leff_1_dn5) / locals.var_scl);
        locals.var_tmp_dn6 = ((locals.var_dsub_i * locals.var_leff_1_dn6) / locals.var_scl);
        locals.var_tmp_dn7 = ((locals.var_dsub_i * locals.var_leff_1_dn7) / locals.var_scl);
        locals.var_tmp_dn8 = ((locals.var_dsub_i * locals.var_leff_1_dn8) / locals.var_scl);
        locals.var_tmp_dn9 = ((locals.var_dsub_i * locals.var_leff_1_dn9) / locals.var_scl);
        locals.var_tmp_dn10 = ((locals.var_dsub_i * locals.var_leff_1_dn10) / locals.var_scl);
        locals.var_tmp_dn11 = ((locals.var_dsub_i * locals.var_leff_1_dn11) / locals.var_scl);
        locals.var_tmp_dn13 = ((locals.var_dsub_i * locals.var_leff_1_dn13) / locals.var_scl);
        locals.var_tmp_dn14 = ((locals.var_dsub_i * locals.var_leff_1_dn14) / locals.var_scl);
        locals.var_tmp_rv = 0.0;

        let assign13340_e17653: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign13340_e17653;
        locals.var_guard231_rv = 0.0;

        let (assign13350_e17668, assign13350_e17668_d_n0, assign13350_e17668_d_n2, assign13350_e17668_d_n3, assign13350_e17668_d_n4, assign13350_e17668_d_n5, assign13350_e17668_d_n6, assign13350_e17668_d_n7, assign13350_e17668_d_n8, assign13350_e17668_d_n9, assign13350_e17668_d_n10, assign13350_e17668_d_n11, assign13350_e17668_d_n13, assign13350_e17668_d_n14,) = {
    if (locals.var_guard231 != 0.0) {
        let assign13350_e17659: f64 = (locals.var_tmp).cosh();
        let assign13350_e17661: f64 = (assign13350_e17659 - 2.0);
        let assign13350_e17662: f64 = (p.p171 * assign13350_e17661);
        let assign13350_e17663: f64 = (1.0 + assign13350_e17662);
        let assign13350_e17665: f64 = (assign13350_e17663).max(1e-6);
        let assign13350_e17666: f64 = (1.0 / assign13350_e17665);
        (assign13350_e17666, (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))),)
    } else {
        (locals.var_theta_dits, locals.var_theta_dits_dn0, locals.var_theta_dits_dn2, locals.var_theta_dits_dn3, locals.var_theta_dits_dn4, locals.var_theta_dits_dn5, locals.var_theta_dits_dn6, locals.var_theta_dits_dn7, locals.var_theta_dits_dn8, locals.var_theta_dits_dn9, locals.var_theta_dits_dn10, locals.var_theta_dits_dn11, locals.var_theta_dits_dn13, locals.var_theta_dits_dn14,)
    }
};
        locals.var_theta_dits = assign13350_e17668;
        locals.var_theta_dits_dn0 = assign13350_e17668_d_n0;
        locals.var_theta_dits_dn2 = assign13350_e17668_d_n2;
        locals.var_theta_dits_dn3 = assign13350_e17668_d_n3;
        locals.var_theta_dits_dn4 = assign13350_e17668_d_n4;
        locals.var_theta_dits_dn5 = assign13350_e17668_d_n5;
        locals.var_theta_dits_dn6 = assign13350_e17668_d_n6;
        locals.var_theta_dits_dn7 = assign13350_e17668_d_n7;
        locals.var_theta_dits_dn8 = assign13350_e17668_d_n8;
        locals.var_theta_dits_dn9 = assign13350_e17668_d_n9;
        locals.var_theta_dits_dn10 = assign13350_e17668_d_n10;
        locals.var_theta_dits_dn11 = assign13350_e17668_d_n11;
        locals.var_theta_dits_dn13 = assign13350_e17668_d_n13;
        locals.var_theta_dits_dn14 = assign13350_e17668_d_n14;
        locals.var_theta_dits_rv = 0.0;

        let (assign13360_e17683, assign13360_e17683_d_n0, assign13360_e17683_d_n2, assign13360_e17683_d_n3, assign13360_e17683_d_n4, assign13360_e17683_d_n5, assign13360_e17683_d_n6, assign13360_e17683_d_n7, assign13360_e17683_d_n8, assign13360_e17683_d_n9, assign13360_e17683_d_n10, assign13360_e17683_d_n11, assign13360_e17683_d_n13, assign13360_e17683_d_n14,) = {
    if (locals.var_guard231 == 0.0) {
        let assign13360_e17672: f64 = (-locals.var_tmp);
        let assign13360_e17673: f64 = { let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13360_e17676: f64 = (-locals.var_tmp);
        let assign13360_e17677: f64 = { let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13360_e17678: f64 = (p.p171 + assign13360_e17677);
        let assign13360_e17680: f64 = (assign13360_e17678).max(1e-6);
        let assign13360_e17681: f64 = (assign13360_e17673 / assign13360_e17680);
        (assign13360_e17681, (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)),)
    } else {
        (locals.var_theta_dits, locals.var_theta_dits_dn0, locals.var_theta_dits_dn2, locals.var_theta_dits_dn3, locals.var_theta_dits_dn4, locals.var_theta_dits_dn5, locals.var_theta_dits_dn6, locals.var_theta_dits_dn7, locals.var_theta_dits_dn8, locals.var_theta_dits_dn9, locals.var_theta_dits_dn10, locals.var_theta_dits_dn11, locals.var_theta_dits_dn13, locals.var_theta_dits_dn14,)
    }
};
        locals.var_theta_dits = assign13360_e17683;
        locals.var_theta_dits_dn0 = assign13360_e17683_d_n0;
        locals.var_theta_dits_dn2 = assign13360_e17683_d_n2;
        locals.var_theta_dits_dn3 = assign13360_e17683_d_n3;
        locals.var_theta_dits_dn4 = assign13360_e17683_d_n4;
        locals.var_theta_dits_dn5 = assign13360_e17683_d_n5;
        locals.var_theta_dits_dn6 = assign13360_e17683_d_n6;
        locals.var_theta_dits_dn7 = assign13360_e17683_d_n7;
        locals.var_theta_dits_dn8 = assign13360_e17683_d_n8;
        locals.var_theta_dits_dn9 = assign13360_e17683_d_n9;
        locals.var_theta_dits_dn10 = assign13360_e17683_d_n10;
        locals.var_theta_dits_dn11 = assign13360_e17683_d_n11;
        locals.var_theta_dits_dn13 = assign13360_e17683_d_n13;
        locals.var_theta_dits_dn14 = assign13360_e17683_d_n14;
        locals.var_theta_dits_rv = 0.0;

        let assign13370_e17686: f64 = (1.60219e-19 * locals.var_nbody_i);
        let assign13370_e17688: f64 = (assign13370_e17686 * locals.var_ach);
        let assign13370_e17690: f64 = (assign13370_e17688 / locals.var_cins);
        locals.var_qbs = assign13370_e17690;
        locals.var_qbs_rv = 0.0;

        let assign13380_e17693: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign13380_e17693;
        locals.var_guard232_rv = 0.0;

        let (assign13400_e17701,) = {
    if (locals.var_guard232 != 0.0) {
        (745669000000.0,)
    } else {
        (locals.var_bechvb,)
    }
};
        locals.var_bechvb = assign13400_e17701;
        locals.var_bechvb_rv = 0.0;

        let (assign13420_e17711,) = {
    if (locals.var_guard232 == 0.0) {
        (1166450000000.0,)
    } else {
        (locals.var_bechvb,)
    }
};
        locals.var_bechvb = assign13420_e17711;
        locals.var_bechvb_rv = 0.0;

        let assign13430_e17714: f64 = (p.p1109 * p.p1109);
        locals.var_t0 = assign13430_e17714;
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
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13440_e17717: f64 = (p.p1109 * locals.var_poxedge_i);
        locals.var_t1 = assign13440_e17717;
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
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13450_e17720: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign13450_e17720;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn13 = ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let assign13490_e17742: f64 = (-273.15);
        let assign13490_e17743: f64 = if p.p1717 < assign13490_e17742 { 1.0 } else { 0.0 };
        locals.var_guard233 = assign13490_e17743;
        locals.var_guard233_rv = 0.0;

        let (assign13500_e17747,) = {
    if (locals.var_guard233 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13500_e17747;
        locals.var_tnom_rv = 0.0;

        let (assign13510_e17754,) = {
    if (locals.var_guard233 == 0.0) {
        let assign13510_e17752: f64 = (p.p1717 + 273.15);
        (assign13510_e17752,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13510_e17754;
        locals.var_tnom_rv = 0.0;

        let assign13520_e17757: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign13520_e17757;
        locals.var_guard234_rv = 0.0;

        let (assign13530_e17778,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13530_e17761: f64 = (p.p1806 - locals.var_dimension1_i);
        let assign13530_e17765: f64 = (p.p1827 * 1000000000.0);
        let assign13530_e17768: f64 = (p.p43 * 1000000000.0);
        let assign13530_e17769: f64 = (assign13530_e17765 - assign13530_e17768);
        let assign13530_e17771: f64 = (assign13530_e17769 / p.p1828);
        let assign13530_e17772: f64 = { let limited_exp_arg = assign13530_e17771; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13530_e17773: f64 = (1.0 + assign13530_e17772);
        let assign13530_e17774: f64 = (assign13530_e17761 / assign13530_e17773);
        let assign13530_e17776: f64 = (assign13530_e17774 + locals.var_dimension1_i);
        (assign13530_e17776,)
    } else {
        (locals.var_d1,)
    }
};
        locals.var_d1 = assign13530_e17778;
        locals.var_d1_rv = 0.0;

        let (assign13540_e17799,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13540_e17782: f64 = (p.p1813 - locals.var_dimension2_i);
        let assign13540_e17786: f64 = (p.p1827 * 1000000000.0);
        let assign13540_e17789: f64 = (p.p43 * 1000000000.0);
        let assign13540_e17790: f64 = (assign13540_e17786 - assign13540_e17789);
        let assign13540_e17792: f64 = (assign13540_e17790 / p.p1828);
        let assign13540_e17793: f64 = { let limited_exp_arg = assign13540_e17792; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13540_e17794: f64 = (1.0 + assign13540_e17793);
        let assign13540_e17795: f64 = (assign13540_e17782 / assign13540_e17794);
        let assign13540_e17797: f64 = (assign13540_e17795 + locals.var_dimension2_i);
        (assign13540_e17797,)
    } else {
        (locals.var_d2,)
    }
};
        locals.var_d2 = assign13540_e17799;
        locals.var_d2_rv = 0.0;

        let (assign13550_e17820,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13550_e17803: f64 = (p.p1820 - locals.var_dimension3_i);
        let assign13550_e17807: f64 = (p.p1827 * 1000000000.0);
        let assign13550_e17810: f64 = (p.p43 * 1000000000.0);
        let assign13550_e17811: f64 = (assign13550_e17807 - assign13550_e17810);
        let assign13550_e17813: f64 = (assign13550_e17811 / p.p1828);
        let assign13550_e17814: f64 = { let limited_exp_arg = assign13550_e17813; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13550_e17815: f64 = (1.0 + assign13550_e17814);
        let assign13550_e17816: f64 = (assign13550_e17803 / assign13550_e17815);
        let assign13550_e17818: f64 = (assign13550_e17816 + locals.var_dimension3_i);
        (assign13550_e17818,)
    } else {
        (locals.var_d3,)
    }
};
        locals.var_d3 = assign13550_e17820;
        locals.var_d3_rv = 0.0;

        let (assign13560_e17891,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13560_e17824: f64 = (-p.p1847);
        let assign13560_e17828: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17831: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17832: f64 = (assign13560_e17828 - assign13560_e17831);
        let assign13560_e17834: f64 = (assign13560_e17832 / p.p1851);
        let assign13560_e17835: f64 = { let limited_exp_arg = assign13560_e17834; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17836: f64 = (1.0 + assign13560_e17835);
        let assign13560_e17837: f64 = (assign13560_e17824 / assign13560_e17836);
        let assign13560_e17839: f64 = (assign13560_e17837 + locals.var_ssp1_i);
        let assign13560_e17841: f64 = assign13560_e17839;
        let assign13560_e17843: f64 = (-p.p1847);
        let assign13560_e17847: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17850: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17851: f64 = (assign13560_e17847 - assign13560_e17850);
        let assign13560_e17853: f64 = (assign13560_e17851 / p.p1851);
        let assign13560_e17854: f64 = { let limited_exp_arg = assign13560_e17853; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17855: f64 = (1.0 + assign13560_e17854);
        let assign13560_e17856: f64 = (assign13560_e17843 / assign13560_e17855);
        let assign13560_e17858: f64 = (assign13560_e17856 + locals.var_ssp1_i);
        let assign13560_e17860: f64 = assign13560_e17858;
        let assign13560_e17862: f64 = (-p.p1847);
        let assign13560_e17866: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17869: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17870: f64 = (assign13560_e17866 - assign13560_e17869);
        let assign13560_e17872: f64 = (assign13560_e17870 / p.p1851);
        let assign13560_e17873: f64 = { let limited_exp_arg = assign13560_e17872; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17874: f64 = (1.0 + assign13560_e17873);
        let assign13560_e17875: f64 = (assign13560_e17862 / assign13560_e17874);
        let assign13560_e17877: f64 = (assign13560_e17875 + locals.var_ssp1_i);
        let assign13560_e17879: f64 = assign13560_e17877;
        let assign13560_e17880: f64 = (assign13560_e17860 * assign13560_e17879);
        let assign13560_e17883: f64 = (0.25 * 0.001);
        let assign13560_e17885: f64 = (assign13560_e17883 * 0.001);
        let assign13560_e17886: f64 = (assign13560_e17880 + assign13560_e17885);
        let assign13560_e17887: f64 = (assign13560_e17886).sqrt();
        let assign13560_e17888: f64 = (assign13560_e17841 + assign13560_e17887);
        let assign13560_e17889: f64 = (0.5 * assign13560_e17888);
        (assign13560_e17889,)
    } else {
        (locals.var_p1,)
    }
};
        locals.var_p1 = assign13560_e17891;
        locals.var_p1_rv = 0.0;

        let (assign13570_e17962,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13570_e17895: f64 = (-p.p1848);
        let assign13570_e17899: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17902: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17903: f64 = (assign13570_e17899 - assign13570_e17902);
        let assign13570_e17905: f64 = (assign13570_e17903 / p.p1851);
        let assign13570_e17906: f64 = { let limited_exp_arg = assign13570_e17905; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17907: f64 = (1.0 + assign13570_e17906);
        let assign13570_e17908: f64 = (assign13570_e17895 / assign13570_e17907);
        let assign13570_e17910: f64 = (assign13570_e17908 + locals.var_ssp2_i);
        let assign13570_e17912: f64 = assign13570_e17910;
        let assign13570_e17914: f64 = (-p.p1848);
        let assign13570_e17918: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17921: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17922: f64 = (assign13570_e17918 - assign13570_e17921);
        let assign13570_e17924: f64 = (assign13570_e17922 / p.p1851);
        let assign13570_e17925: f64 = { let limited_exp_arg = assign13570_e17924; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17926: f64 = (1.0 + assign13570_e17925);
        let assign13570_e17927: f64 = (assign13570_e17914 / assign13570_e17926);
        let assign13570_e17929: f64 = (assign13570_e17927 + locals.var_ssp2_i);
        let assign13570_e17931: f64 = assign13570_e17929;
        let assign13570_e17933: f64 = (-p.p1848);
        let assign13570_e17937: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17940: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17941: f64 = (assign13570_e17937 - assign13570_e17940);
        let assign13570_e17943: f64 = (assign13570_e17941 / p.p1851);
        let assign13570_e17944: f64 = { let limited_exp_arg = assign13570_e17943; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17945: f64 = (1.0 + assign13570_e17944);
        let assign13570_e17946: f64 = (assign13570_e17933 / assign13570_e17945);
        let assign13570_e17948: f64 = (assign13570_e17946 + locals.var_ssp2_i);
        let assign13570_e17950: f64 = assign13570_e17948;
        let assign13570_e17951: f64 = (assign13570_e17931 * assign13570_e17950);
        let assign13570_e17954: f64 = (0.25 * 0.001);
        let assign13570_e17956: f64 = (assign13570_e17954 * 0.001);
        let assign13570_e17957: f64 = (assign13570_e17951 + assign13570_e17956);
        let assign13570_e17958: f64 = (assign13570_e17957).sqrt();
        let assign13570_e17959: f64 = (assign13570_e17912 + assign13570_e17958);
        let assign13570_e17960: f64 = (0.5 * assign13570_e17959);
        (assign13570_e17960,)
    } else {
        (locals.var_p2,)
    }
};
        locals.var_p2 = assign13570_e17962;
        locals.var_p2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13580_e18033,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13580_e17966: f64 = (-p.p1849);
        let assign13580_e17970: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e17973: f64 = (p.p43 * 1000000000.0);
        let assign13580_e17974: f64 = (assign13580_e17970 - assign13580_e17973);
        let assign13580_e17976: f64 = (assign13580_e17974 / p.p1851);
        let assign13580_e17977: f64 = { let limited_exp_arg = assign13580_e17976; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e17978: f64 = (1.0 + assign13580_e17977);
        let assign13580_e17979: f64 = (assign13580_e17966 / assign13580_e17978);
        let assign13580_e17981: f64 = (assign13580_e17979 + locals.var_ssp3_i);
        let assign13580_e17983: f64 = assign13580_e17981;
        let assign13580_e17985: f64 = (-p.p1849);
        let assign13580_e17989: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e17992: f64 = (p.p43 * 1000000000.0);
        let assign13580_e17993: f64 = (assign13580_e17989 - assign13580_e17992);
        let assign13580_e17995: f64 = (assign13580_e17993 / p.p1851);
        let assign13580_e17996: f64 = { let limited_exp_arg = assign13580_e17995; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e17997: f64 = (1.0 + assign13580_e17996);
        let assign13580_e17998: f64 = (assign13580_e17985 / assign13580_e17997);
        let assign13580_e18000: f64 = (assign13580_e17998 + locals.var_ssp3_i);
        let assign13580_e18002: f64 = assign13580_e18000;
        let assign13580_e18004: f64 = (-p.p1849);
        let assign13580_e18008: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e18011: f64 = (p.p43 * 1000000000.0);
        let assign13580_e18012: f64 = (assign13580_e18008 - assign13580_e18011);
        let assign13580_e18014: f64 = (assign13580_e18012 / p.p1851);
        let assign13580_e18015: f64 = { let limited_exp_arg = assign13580_e18014; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e18016: f64 = (1.0 + assign13580_e18015);
        let assign13580_e18017: f64 = (assign13580_e18004 / assign13580_e18016);
        let assign13580_e18019: f64 = (assign13580_e18017 + locals.var_ssp3_i);
        let assign13580_e18021: f64 = assign13580_e18019;
        let assign13580_e18022: f64 = (assign13580_e18002 * assign13580_e18021);
        let assign13580_e18025: f64 = (0.25 * 0.001);
        let assign13580_e18027: f64 = (assign13580_e18025 * 0.001);
        let assign13580_e18028: f64 = (assign13580_e18022 + assign13580_e18027);
        let assign13580_e18029: f64 = (assign13580_e18028).sqrt();
        let assign13580_e18030: f64 = (assign13580_e17983 + assign13580_e18029);
        let assign13580_e18031: f64 = (0.5 * assign13580_e18030);
        (assign13580_e18031,)
    } else {
        (locals.var_p3,)
    }
};
        locals.var_p3 = assign13580_e18033;
        locals.var_p3_rv = 0.0;

        let (assign13590_e18153,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13590_e18040: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18041: f64 = (1.001 * assign13590_e18040);
        let assign13590_e18043: f64 = assign13590_e18041;
        let assign13590_e18047: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18048: f64 = (1.001 * assign13590_e18047);
        let assign13590_e18050: f64 = assign13590_e18048;
        let assign13590_e18054: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18055: f64 = (1.001 * assign13590_e18054);
        let assign13590_e18057: f64 = assign13590_e18055;
        let assign13590_e18058: f64 = (assign13590_e18050 * assign13590_e18057);
        let assign13590_e18061: f64 = (0.25 * 0.001);
        let assign13590_e18063: f64 = (assign13590_e18061 * 0.001);
        let assign13590_e18064: f64 = (assign13590_e18058 + assign13590_e18063);
        let assign13590_e18065: f64 = (assign13590_e18064).sqrt();
        let assign13590_e18066: f64 = (assign13590_e18043 + assign13590_e18065);
        let assign13590_e18067: f64 = (0.5 * assign13590_e18066);
        let assign13590_e18069: f64 = (assign13590_e18067 + 1.0);
        let assign13590_e18074: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18075: f64 = (1.001 * assign13590_e18074);
        let assign13590_e18077: f64 = assign13590_e18075;
        let assign13590_e18081: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18082: f64 = (1.001 * assign13590_e18081);
        let assign13590_e18084: f64 = assign13590_e18082;
        let assign13590_e18088: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18089: f64 = (1.001 * assign13590_e18088);
        let assign13590_e18091: f64 = assign13590_e18089;
        let assign13590_e18092: f64 = (assign13590_e18084 * assign13590_e18091);
        let assign13590_e18095: f64 = (0.25 * 0.001);
        let assign13590_e18097: f64 = (assign13590_e18095 * 0.001);
        let assign13590_e18098: f64 = (assign13590_e18092 + assign13590_e18097);
        let assign13590_e18099: f64 = (assign13590_e18098).sqrt();
        let assign13590_e18100: f64 = (assign13590_e18077 + assign13590_e18099);
        let assign13590_e18101: f64 = (0.5 * assign13590_e18100);
        let assign13590_e18103: f64 = (assign13590_e18101 - 1.0);
        let assign13590_e18108: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18109: f64 = (1.001 * assign13590_e18108);
        let assign13590_e18111: f64 = assign13590_e18109;
        let assign13590_e18115: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18116: f64 = (1.001 * assign13590_e18115);
        let assign13590_e18118: f64 = assign13590_e18116;
        let assign13590_e18122: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18123: f64 = (1.001 * assign13590_e18122);
        let assign13590_e18125: f64 = assign13590_e18123;
        let assign13590_e18126: f64 = (assign13590_e18118 * assign13590_e18125);
        let assign13590_e18129: f64 = (0.25 * 0.001);
        let assign13590_e18131: f64 = (assign13590_e18129 * 0.001);
        let assign13590_e18132: f64 = (assign13590_e18126 + assign13590_e18131);
        let assign13590_e18133: f64 = (assign13590_e18132).sqrt();
        let assign13590_e18134: f64 = (assign13590_e18111 + assign13590_e18133);
        let assign13590_e18135: f64 = (0.5 * assign13590_e18134);
        let assign13590_e18137: f64 = (assign13590_e18135 - 1.0);
        let assign13590_e18138: f64 = (assign13590_e18103 * assign13590_e18137);
        let assign13590_e18141: f64 = (0.25 * 0.001);
        let assign13590_e18143: f64 = (assign13590_e18141 * 0.001);
        let assign13590_e18144: f64 = (assign13590_e18138 + assign13590_e18143);
        let assign13590_e18145: f64 = (assign13590_e18144).sqrt();
        let assign13590_e18146: f64 = (assign13590_e18069 - assign13590_e18145);
        let assign13590_e18147: f64 = (0.5 * assign13590_e18146);
        let assign13590_e18150: f64 = (0.25 * 0.001);
        let assign13590_e18151: f64 = (assign13590_e18147 + assign13590_e18150);
        (assign13590_e18151,)
    } else {
        (locals.var_wp1,)
    }
};
        locals.var_wp1 = assign13590_e18153;
        locals.var_wp1_rv = 0.0;

        let (assign13600_e18273,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13600_e18160: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18161: f64 = (1.001 * assign13600_e18160);
        let assign13600_e18163: f64 = assign13600_e18161;
        let assign13600_e18167: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18168: f64 = (1.001 * assign13600_e18167);
        let assign13600_e18170: f64 = assign13600_e18168;
        let assign13600_e18174: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18175: f64 = (1.001 * assign13600_e18174);
        let assign13600_e18177: f64 = assign13600_e18175;
        let assign13600_e18178: f64 = (assign13600_e18170 * assign13600_e18177);
        let assign13600_e18181: f64 = (0.25 * 0.001);
        let assign13600_e18183: f64 = (assign13600_e18181 * 0.001);
        let assign13600_e18184: f64 = (assign13600_e18178 + assign13600_e18183);
        let assign13600_e18185: f64 = (assign13600_e18184).sqrt();
        let assign13600_e18186: f64 = (assign13600_e18163 + assign13600_e18185);
        let assign13600_e18187: f64 = (0.5 * assign13600_e18186);
        let assign13600_e18189: f64 = (assign13600_e18187 + 1.0);
        let assign13600_e18194: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18195: f64 = (1.001 * assign13600_e18194);
        let assign13600_e18197: f64 = assign13600_e18195;
        let assign13600_e18201: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18202: f64 = (1.001 * assign13600_e18201);
        let assign13600_e18204: f64 = assign13600_e18202;
        let assign13600_e18208: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18209: f64 = (1.001 * assign13600_e18208);
        let assign13600_e18211: f64 = assign13600_e18209;
        let assign13600_e18212: f64 = (assign13600_e18204 * assign13600_e18211);
        let assign13600_e18215: f64 = (0.25 * 0.001);
        let assign13600_e18217: f64 = (assign13600_e18215 * 0.001);
        let assign13600_e18218: f64 = (assign13600_e18212 + assign13600_e18217);
        let assign13600_e18219: f64 = (assign13600_e18218).sqrt();
        let assign13600_e18220: f64 = (assign13600_e18197 + assign13600_e18219);
        let assign13600_e18221: f64 = (0.5 * assign13600_e18220);
        let assign13600_e18223: f64 = (assign13600_e18221 - 1.0);
        let assign13600_e18228: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18229: f64 = (1.001 * assign13600_e18228);
        let assign13600_e18231: f64 = assign13600_e18229;
        let assign13600_e18235: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18236: f64 = (1.001 * assign13600_e18235);
        let assign13600_e18238: f64 = assign13600_e18236;
        let assign13600_e18242: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18243: f64 = (1.001 * assign13600_e18242);
        let assign13600_e18245: f64 = assign13600_e18243;
        let assign13600_e18246: f64 = (assign13600_e18238 * assign13600_e18245);
        let assign13600_e18249: f64 = (0.25 * 0.001);
        let assign13600_e18251: f64 = (assign13600_e18249 * 0.001);
        let assign13600_e18252: f64 = (assign13600_e18246 + assign13600_e18251);
        let assign13600_e18253: f64 = (assign13600_e18252).sqrt();
        let assign13600_e18254: f64 = (assign13600_e18231 + assign13600_e18253);
        let assign13600_e18255: f64 = (0.5 * assign13600_e18254);
        let assign13600_e18257: f64 = (assign13600_e18255 - 1.0);
        let assign13600_e18258: f64 = (assign13600_e18223 * assign13600_e18257);
        let assign13600_e18261: f64 = (0.25 * 0.001);
        let assign13600_e18263: f64 = (assign13600_e18261 * 0.001);
        let assign13600_e18264: f64 = (assign13600_e18258 + assign13600_e18263);
        let assign13600_e18265: f64 = (assign13600_e18264).sqrt();
        let assign13600_e18266: f64 = (assign13600_e18189 - assign13600_e18265);
        let assign13600_e18267: f64 = (0.5 * assign13600_e18266);
        let assign13600_e18270: f64 = (0.25 * 0.001);
        let assign13600_e18271: f64 = (assign13600_e18267 + assign13600_e18270);
        (assign13600_e18271,)
    } else {
        (locals.var_tp1,)
    }
};
        locals.var_tp1 = assign13600_e18273;
        locals.var_tp1_rv = 0.0;

        let (assign13610_e18393,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13610_e18280: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18281: f64 = (1.001 * assign13610_e18280);
        let assign13610_e18283: f64 = assign13610_e18281;
        let assign13610_e18287: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18288: f64 = (1.001 * assign13610_e18287);
        let assign13610_e18290: f64 = assign13610_e18288;
        let assign13610_e18294: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18295: f64 = (1.001 * assign13610_e18294);
        let assign13610_e18297: f64 = assign13610_e18295;
        let assign13610_e18298: f64 = (assign13610_e18290 * assign13610_e18297);
        let assign13610_e18301: f64 = (0.25 * 0.001);
        let assign13610_e18303: f64 = (assign13610_e18301 * 0.001);
        let assign13610_e18304: f64 = (assign13610_e18298 + assign13610_e18303);
        let assign13610_e18305: f64 = (assign13610_e18304).sqrt();
        let assign13610_e18306: f64 = (assign13610_e18283 + assign13610_e18305);
        let assign13610_e18307: f64 = (0.5 * assign13610_e18306);
        let assign13610_e18309: f64 = (assign13610_e18307 + 1.0);
        let assign13610_e18314: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18315: f64 = (1.001 * assign13610_e18314);
        let assign13610_e18317: f64 = assign13610_e18315;
        let assign13610_e18321: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18322: f64 = (1.001 * assign13610_e18321);
        let assign13610_e18324: f64 = assign13610_e18322;
        let assign13610_e18328: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18329: f64 = (1.001 * assign13610_e18328);
        let assign13610_e18331: f64 = assign13610_e18329;
        let assign13610_e18332: f64 = (assign13610_e18324 * assign13610_e18331);
        let assign13610_e18335: f64 = (0.25 * 0.001);
        let assign13610_e18337: f64 = (assign13610_e18335 * 0.001);
        let assign13610_e18338: f64 = (assign13610_e18332 + assign13610_e18337);
        let assign13610_e18339: f64 = (assign13610_e18338).sqrt();
        let assign13610_e18340: f64 = (assign13610_e18317 + assign13610_e18339);
        let assign13610_e18341: f64 = (0.5 * assign13610_e18340);
        let assign13610_e18343: f64 = (assign13610_e18341 - 1.0);
        let assign13610_e18348: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18349: f64 = (1.001 * assign13610_e18348);
        let assign13610_e18351: f64 = assign13610_e18349;
        let assign13610_e18355: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18356: f64 = (1.001 * assign13610_e18355);
        let assign13610_e18358: f64 = assign13610_e18356;
        let assign13610_e18362: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18363: f64 = (1.001 * assign13610_e18362);
        let assign13610_e18365: f64 = assign13610_e18363;
        let assign13610_e18366: f64 = (assign13610_e18358 * assign13610_e18365);
        let assign13610_e18369: f64 = (0.25 * 0.001);
        let assign13610_e18371: f64 = (assign13610_e18369 * 0.001);
        let assign13610_e18372: f64 = (assign13610_e18366 + assign13610_e18371);
        let assign13610_e18373: f64 = (assign13610_e18372).sqrt();
        let assign13610_e18374: f64 = (assign13610_e18351 + assign13610_e18373);
        let assign13610_e18375: f64 = (0.5 * assign13610_e18374);
        let assign13610_e18377: f64 = (assign13610_e18375 - 1.0);
        let assign13610_e18378: f64 = (assign13610_e18343 * assign13610_e18377);
        let assign13610_e18381: f64 = (0.25 * 0.001);
        let assign13610_e18383: f64 = (assign13610_e18381 * 0.001);
        let assign13610_e18384: f64 = (assign13610_e18378 + assign13610_e18383);
        let assign13610_e18385: f64 = (assign13610_e18384).sqrt();
        let assign13610_e18386: f64 = (assign13610_e18309 - assign13610_e18385);
        let assign13610_e18387: f64 = (0.5 * assign13610_e18386);
        let assign13610_e18390: f64 = (0.25 * 0.001);
        let assign13610_e18391: f64 = (assign13610_e18387 + assign13610_e18390);
        (assign13610_e18391,)
    } else {
        (locals.var_wp2,)
    }
};
        locals.var_wp2 = assign13610_e18393;
        locals.var_wp2_rv = 0.0;

        let (assign13620_e18513,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13620_e18400: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18401: f64 = (1.001 * assign13620_e18400);
        let assign13620_e18403: f64 = assign13620_e18401;
        let assign13620_e18407: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18408: f64 = (1.001 * assign13620_e18407);
        let assign13620_e18410: f64 = assign13620_e18408;
        let assign13620_e18414: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18415: f64 = (1.001 * assign13620_e18414);
        let assign13620_e18417: f64 = assign13620_e18415;
        let assign13620_e18418: f64 = (assign13620_e18410 * assign13620_e18417);
        let assign13620_e18421: f64 = (0.25 * 0.001);
        let assign13620_e18423: f64 = (assign13620_e18421 * 0.001);
        let assign13620_e18424: f64 = (assign13620_e18418 + assign13620_e18423);
        let assign13620_e18425: f64 = (assign13620_e18424).sqrt();
        let assign13620_e18426: f64 = (assign13620_e18403 + assign13620_e18425);
        let assign13620_e18427: f64 = (0.5 * assign13620_e18426);
        let assign13620_e18429: f64 = (assign13620_e18427 + 1.0);
        let assign13620_e18434: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18435: f64 = (1.001 * assign13620_e18434);
        let assign13620_e18437: f64 = assign13620_e18435;
        let assign13620_e18441: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18442: f64 = (1.001 * assign13620_e18441);
        let assign13620_e18444: f64 = assign13620_e18442;
        let assign13620_e18448: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18449: f64 = (1.001 * assign13620_e18448);
        let assign13620_e18451: f64 = assign13620_e18449;
        let assign13620_e18452: f64 = (assign13620_e18444 * assign13620_e18451);
        let assign13620_e18455: f64 = (0.25 * 0.001);
        let assign13620_e18457: f64 = (assign13620_e18455 * 0.001);
        let assign13620_e18458: f64 = (assign13620_e18452 + assign13620_e18457);
        let assign13620_e18459: f64 = (assign13620_e18458).sqrt();
        let assign13620_e18460: f64 = (assign13620_e18437 + assign13620_e18459);
        let assign13620_e18461: f64 = (0.5 * assign13620_e18460);
        let assign13620_e18463: f64 = (assign13620_e18461 - 1.0);
        let assign13620_e18468: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18469: f64 = (1.001 * assign13620_e18468);
        let assign13620_e18471: f64 = assign13620_e18469;
        let assign13620_e18475: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18476: f64 = (1.001 * assign13620_e18475);
        let assign13620_e18478: f64 = assign13620_e18476;
        let assign13620_e18482: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18483: f64 = (1.001 * assign13620_e18482);
        let assign13620_e18485: f64 = assign13620_e18483;
        let assign13620_e18486: f64 = (assign13620_e18478 * assign13620_e18485);
        let assign13620_e18489: f64 = (0.25 * 0.001);
        let assign13620_e18491: f64 = (assign13620_e18489 * 0.001);
        let assign13620_e18492: f64 = (assign13620_e18486 + assign13620_e18491);
        let assign13620_e18493: f64 = (assign13620_e18492).sqrt();
        let assign13620_e18494: f64 = (assign13620_e18471 + assign13620_e18493);
        let assign13620_e18495: f64 = (0.5 * assign13620_e18494);
        let assign13620_e18497: f64 = (assign13620_e18495 - 1.0);
        let assign13620_e18498: f64 = (assign13620_e18463 * assign13620_e18497);
        let assign13620_e18501: f64 = (0.25 * 0.001);
        let assign13620_e18503: f64 = (assign13620_e18501 * 0.001);
        let assign13620_e18504: f64 = (assign13620_e18498 + assign13620_e18503);
        let assign13620_e18505: f64 = (assign13620_e18504).sqrt();
        let assign13620_e18506: f64 = (assign13620_e18429 - assign13620_e18505);
        let assign13620_e18507: f64 = (0.5 * assign13620_e18506);
        let assign13620_e18510: f64 = (0.25 * 0.001);
        let assign13620_e18511: f64 = (assign13620_e18507 + assign13620_e18510);
        (assign13620_e18511,)
    } else {
        (locals.var_tp2,)
    }
};
        locals.var_tp2 = assign13620_e18513;
        locals.var_tp2_rv = 0.0;

        let (assign13630_e18633,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13630_e18520: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18521: f64 = (1.001 * assign13630_e18520);
        let assign13630_e18523: f64 = assign13630_e18521;
        let assign13630_e18527: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18528: f64 = (1.001 * assign13630_e18527);
        let assign13630_e18530: f64 = assign13630_e18528;
        let assign13630_e18534: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18535: f64 = (1.001 * assign13630_e18534);
        let assign13630_e18537: f64 = assign13630_e18535;
        let assign13630_e18538: f64 = (assign13630_e18530 * assign13630_e18537);
        let assign13630_e18541: f64 = (0.25 * 0.001);
        let assign13630_e18543: f64 = (assign13630_e18541 * 0.001);
        let assign13630_e18544: f64 = (assign13630_e18538 + assign13630_e18543);
        let assign13630_e18545: f64 = (assign13630_e18544).sqrt();
        let assign13630_e18546: f64 = (assign13630_e18523 + assign13630_e18545);
        let assign13630_e18547: f64 = (0.5 * assign13630_e18546);
        let assign13630_e18549: f64 = (assign13630_e18547 + 1.0);
        let assign13630_e18554: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18555: f64 = (1.001 * assign13630_e18554);
        let assign13630_e18557: f64 = assign13630_e18555;
        let assign13630_e18561: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18562: f64 = (1.001 * assign13630_e18561);
        let assign13630_e18564: f64 = assign13630_e18562;
        let assign13630_e18568: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18569: f64 = (1.001 * assign13630_e18568);
        let assign13630_e18571: f64 = assign13630_e18569;
        let assign13630_e18572: f64 = (assign13630_e18564 * assign13630_e18571);
        let assign13630_e18575: f64 = (0.25 * 0.001);
        let assign13630_e18577: f64 = (assign13630_e18575 * 0.001);
        let assign13630_e18578: f64 = (assign13630_e18572 + assign13630_e18577);
        let assign13630_e18579: f64 = (assign13630_e18578).sqrt();
        let assign13630_e18580: f64 = (assign13630_e18557 + assign13630_e18579);
        let assign13630_e18581: f64 = (0.5 * assign13630_e18580);
        let assign13630_e18583: f64 = (assign13630_e18581 - 1.0);
        let assign13630_e18588: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18589: f64 = (1.001 * assign13630_e18588);
        let assign13630_e18591: f64 = assign13630_e18589;
        let assign13630_e18595: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18596: f64 = (1.001 * assign13630_e18595);
        let assign13630_e18598: f64 = assign13630_e18596;
        let assign13630_e18602: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18603: f64 = (1.001 * assign13630_e18602);
        let assign13630_e18605: f64 = assign13630_e18603;
        let assign13630_e18606: f64 = (assign13630_e18598 * assign13630_e18605);
        let assign13630_e18609: f64 = (0.25 * 0.001);
        let assign13630_e18611: f64 = (assign13630_e18609 * 0.001);
        let assign13630_e18612: f64 = (assign13630_e18606 + assign13630_e18611);
        let assign13630_e18613: f64 = (assign13630_e18612).sqrt();
        let assign13630_e18614: f64 = (assign13630_e18591 + assign13630_e18613);
        let assign13630_e18615: f64 = (0.5 * assign13630_e18614);
        let assign13630_e18617: f64 = (assign13630_e18615 - 1.0);
        let assign13630_e18618: f64 = (assign13630_e18583 * assign13630_e18617);
        let assign13630_e18621: f64 = (0.25 * 0.001);
        let assign13630_e18623: f64 = (assign13630_e18621 * 0.001);
        let assign13630_e18624: f64 = (assign13630_e18618 + assign13630_e18623);
        let assign13630_e18625: f64 = (assign13630_e18624).sqrt();
        let assign13630_e18626: f64 = (assign13630_e18549 - assign13630_e18625);
        let assign13630_e18627: f64 = (0.5 * assign13630_e18626);
        let assign13630_e18630: f64 = (0.25 * 0.001);
        let assign13630_e18631: f64 = (assign13630_e18627 + assign13630_e18630);
        (assign13630_e18631,)
    } else {
        (locals.var_wp3,)
    }
};
        locals.var_wp3 = assign13630_e18633;
        locals.var_wp3_rv = 0.0;

        let (assign13640_e18753,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13640_e18640: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18641: f64 = (1.001 * assign13640_e18640);
        let assign13640_e18643: f64 = assign13640_e18641;
        let assign13640_e18647: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18648: f64 = (1.001 * assign13640_e18647);
        let assign13640_e18650: f64 = assign13640_e18648;
        let assign13640_e18654: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18655: f64 = (1.001 * assign13640_e18654);
        let assign13640_e18657: f64 = assign13640_e18655;
        let assign13640_e18658: f64 = (assign13640_e18650 * assign13640_e18657);
        let assign13640_e18661: f64 = (0.25 * 0.001);
        let assign13640_e18663: f64 = (assign13640_e18661 * 0.001);
        let assign13640_e18664: f64 = (assign13640_e18658 + assign13640_e18663);
        let assign13640_e18665: f64 = (assign13640_e18664).sqrt();
        let assign13640_e18666: f64 = (assign13640_e18643 + assign13640_e18665);
        let assign13640_e18667: f64 = (0.5 * assign13640_e18666);
        let assign13640_e18669: f64 = (assign13640_e18667 + 1.0);
        let assign13640_e18674: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18675: f64 = (1.001 * assign13640_e18674);
        let assign13640_e18677: f64 = assign13640_e18675;
        let assign13640_e18681: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18682: f64 = (1.001 * assign13640_e18681);
        let assign13640_e18684: f64 = assign13640_e18682;
        let assign13640_e18688: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18689: f64 = (1.001 * assign13640_e18688);
        let assign13640_e18691: f64 = assign13640_e18689;
        let assign13640_e18692: f64 = (assign13640_e18684 * assign13640_e18691);
        let assign13640_e18695: f64 = (0.25 * 0.001);
        let assign13640_e18697: f64 = (assign13640_e18695 * 0.001);
        let assign13640_e18698: f64 = (assign13640_e18692 + assign13640_e18697);
        let assign13640_e18699: f64 = (assign13640_e18698).sqrt();
        let assign13640_e18700: f64 = (assign13640_e18677 + assign13640_e18699);
        let assign13640_e18701: f64 = (0.5 * assign13640_e18700);
        let assign13640_e18703: f64 = (assign13640_e18701 - 1.0);
        let assign13640_e18708: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18709: f64 = (1.001 * assign13640_e18708);
        let assign13640_e18711: f64 = assign13640_e18709;
        let assign13640_e18715: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18716: f64 = (1.001 * assign13640_e18715);
        let assign13640_e18718: f64 = assign13640_e18716;
        let assign13640_e18722: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18723: f64 = (1.001 * assign13640_e18722);
        let assign13640_e18725: f64 = assign13640_e18723;
        let assign13640_e18726: f64 = (assign13640_e18718 * assign13640_e18725);
        let assign13640_e18729: f64 = (0.25 * 0.001);
        let assign13640_e18731: f64 = (assign13640_e18729 * 0.001);
        let assign13640_e18732: f64 = (assign13640_e18726 + assign13640_e18731);
        let assign13640_e18733: f64 = (assign13640_e18732).sqrt();
        let assign13640_e18734: f64 = (assign13640_e18711 + assign13640_e18733);
        let assign13640_e18735: f64 = (0.5 * assign13640_e18734);
        let assign13640_e18737: f64 = (assign13640_e18735 - 1.0);
        let assign13640_e18738: f64 = (assign13640_e18703 * assign13640_e18737);
        let assign13640_e18741: f64 = (0.25 * 0.001);
        let assign13640_e18743: f64 = (assign13640_e18741 * 0.001);
        let assign13640_e18744: f64 = (assign13640_e18738 + assign13640_e18743);
        let assign13640_e18745: f64 = (assign13640_e18744).sqrt();
        let assign13640_e18746: f64 = (assign13640_e18669 - assign13640_e18745);
        let assign13640_e18747: f64 = (0.5 * assign13640_e18746);
        let assign13640_e18750: f64 = (0.25 * 0.001);
        let assign13640_e18751: f64 = (assign13640_e18747 + assign13640_e18750);
        (assign13640_e18751,)
    } else {
        (locals.var_tp3,)
    }
};
        locals.var_tp3 = assign13640_e18753;
        locals.var_tp3_rv = 0.0;

        let (assign13650_e18765,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13650_e18757: f64 = (locals.var_weff0).powf(locals.var_wp1);
        let assign13650_e18760: f64 = (locals.var_ach / locals.var_weff0);
        let assign13650_e18762: f64 = (assign13650_e18760).powf(locals.var_tp1);
        let assign13650_e18763: f64 = (assign13650_e18757 * assign13650_e18762);
        (assign13650_e18763,)
    } else {
        (locals.var_vnd1,)
    }
};
        locals.var_vnd1 = assign13650_e18765;
        locals.var_vnd1_rv = 0.0;

        let (assign13660_e18771,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13660_e18769: f64 = (locals.var_vnd1 / locals.var_cins);
        (assign13660_e18769,)
    } else {
        (locals.var_qndnf1,)
    }
};
        locals.var_qndnf1 = assign13660_e18771;
        locals.var_qndnf1_rv = 0.0;

        let (assign13670_e18783,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13670_e18775: f64 = (locals.var_weff0).powf(locals.var_wp2);
        let assign13670_e18778: f64 = (locals.var_ach / locals.var_weff0);
        let assign13670_e18780: f64 = (assign13670_e18778).powf(locals.var_tp2);
        let assign13670_e18781: f64 = (assign13670_e18775 * assign13670_e18780);
        (assign13670_e18781,)
    } else {
        (locals.var_vnd2,)
    }
};
        locals.var_vnd2 = assign13670_e18783;
        locals.var_vnd2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13680_e18789,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13680_e18787: f64 = (locals.var_vnd2 / locals.var_cins);
        (assign13680_e18787,)
    } else {
        (locals.var_qndnf2,)
    }
};
        locals.var_qndnf2 = assign13680_e18789;
        locals.var_qndnf2_rv = 0.0;

        let (assign13690_e18801,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13690_e18793: f64 = (locals.var_weff0).powf(locals.var_wp3);
        let assign13690_e18796: f64 = (locals.var_ach / locals.var_weff0);
        let assign13690_e18798: f64 = (assign13690_e18796).powf(locals.var_tp3);
        let assign13690_e18799: f64 = (assign13690_e18793 * assign13690_e18798);
        (assign13690_e18799,)
    } else {
        (locals.var_vnd3,)
    }
};
        locals.var_vnd3 = assign13690_e18801;
        locals.var_vnd3_rv = 0.0;

        let (assign13700_e18807,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13700_e18805: f64 = (locals.var_vnd3 / locals.var_cins);
        (assign13700_e18805,)
    } else {
        (locals.var_qndnf3,)
    }
};
        locals.var_qndnf3 = assign13700_e18807;
        locals.var_qndnf3_rv = 0.0;

        let (assign13710_e18863,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13710_e18815: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18816: f64 = (2.75 - assign13710_e18815);
        let assign13710_e18818: f64 = (assign13710_e18816 / 0.78);
        let assign13710_e18819: f64 = { let limited_exp_arg = assign13710_e18818; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18820: f64 = (1.0 + assign13710_e18819);
        let assign13710_e18821: f64 = (1.0 / assign13710_e18820);
        let assign13710_e18823: f64 = (assign13710_e18821 + 0.5);
        let assign13710_e18829: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18830: f64 = (2.75 - assign13710_e18829);
        let assign13710_e18832: f64 = (assign13710_e18830 / 0.78);
        let assign13710_e18833: f64 = { let limited_exp_arg = assign13710_e18832; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18834: f64 = (1.0 + assign13710_e18833);
        let assign13710_e18835: f64 = (1.0 / assign13710_e18834);
        let assign13710_e18837: f64 = (assign13710_e18835 - 0.5);
        let assign13710_e18843: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18844: f64 = (2.75 - assign13710_e18843);
        let assign13710_e18846: f64 = (assign13710_e18844 / 0.78);
        let assign13710_e18847: f64 = { let limited_exp_arg = assign13710_e18846; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18848: f64 = (1.0 + assign13710_e18847);
        let assign13710_e18849: f64 = (1.0 / assign13710_e18848);
        let assign13710_e18851: f64 = (assign13710_e18849 - 0.5);
        let assign13710_e18852: f64 = (assign13710_e18837 * assign13710_e18851);
        let assign13710_e18855: f64 = (0.25 * 0.003);
        let assign13710_e18857: f64 = (assign13710_e18855 * 0.003);
        let assign13710_e18858: f64 = (assign13710_e18852 + assign13710_e18857);
        let assign13710_e18859: f64 = (assign13710_e18858).sqrt();
        let assign13710_e18860: f64 = (assign13710_e18823 + assign13710_e18859);
        let assign13710_e18861: f64 = (0.5 * assign13710_e18860);
        (assign13710_e18861,)
    } else {
        (locals.var_nc3d0,)
    }
};
        locals.var_nc3d0 = assign13710_e18863;
        locals.var_nc3d0_rv = 0.0;

        let (assign13720_e18879,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13720_e18867: f64 = (1.0 - locals.var_nc3d0);
        let assign13720_e18870: f64 = (locals.var_d1 - locals.var_dimension1_i);
        let assign13720_e18871: f64 = (assign13720_e18867 * assign13720_e18870);
        let assign13720_e18874: f64 = (p.p1806 - locals.var_dimension1_i);
        let assign13720_e18875: f64 = (assign13720_e18871 / assign13720_e18874);
        let assign13720_e18877: f64 = (assign13720_e18875 + locals.var_nc3d0);
        (assign13720_e18877,)
    } else {
        (locals.var_nc3d,)
    }
};
        locals.var_nc3d = assign13720_e18879;
        locals.var_nc3d_rv = 0.0;

        let (assign13730_e18892,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13730_e18885: f64 = (locals.var_nc3d - 0.999);
        let assign13730_e18887: f64 = (assign13730_e18885 / 0.0001);
        let assign13730_e18888: f64 = { let limited_exp_arg = assign13730_e18887; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13730_e18889: f64 = (1.0 + assign13730_e18888);
        let assign13730_e18890: f64 = (1.0 / assign13730_e18889);
        (assign13730_e18890,)
    } else {
        (locals.var_ncq,)
    }
};
        locals.var_ncq = assign13730_e18892;
        locals.var_ncq_rv = 0.0;

        let (assign13740_e18910,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13740_e18896: f64 = (0.5 * p.p40);
        let assign13740_e18898: f64 = (assign13740_e18896 * p.p40);
        let assign13740_e18900: f64 = (assign13740_e18898 * 1e18);
        let assign13740_e18903: f64 = (1.5 * p.p40);
        let assign13740_e18905: f64 = (assign13740_e18903 * 1000000000.0);
        let assign13740_e18906: f64 = (assign13740_e18900 - assign13740_e18905);
        let assign13740_e18908: f64 = (assign13740_e18906 + 2.0);
        (assign13740_e18908,)
    } else {
        (locals.var_qt0,)
    }
};
        locals.var_qt0 = assign13740_e18910;
        locals.var_qt0_rv = 0.0;

        let (assign13750_e18937,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13750_e18915: f64 = (locals.var_qt0 + 4.0);
        let assign13750_e18918: f64 = (locals.var_qt0 - 4.0);
        let assign13750_e18921: f64 = (locals.var_qt0 - 4.0);
        let assign13750_e18922: f64 = (assign13750_e18918 * assign13750_e18921);
        let assign13750_e18925: f64 = (0.25 * 0.01);
        let assign13750_e18927: f64 = (assign13750_e18925 * 0.01);
        let assign13750_e18928: f64 = (assign13750_e18922 + assign13750_e18927);
        let assign13750_e18929: f64 = (assign13750_e18928).sqrt();
        let assign13750_e18930: f64 = (assign13750_e18915 - assign13750_e18929);
        let assign13750_e18931: f64 = (0.5 * assign13750_e18930);
        let assign13750_e18934: f64 = (0.25 * 0.01);
        let assign13750_e18935: f64 = (assign13750_e18931 + assign13750_e18934);
        (assign13750_e18935,)
    } else {
        (locals.var_qt1,)
    }
};
        locals.var_qt1 = assign13750_e18937;
        locals.var_qt1_rv = 0.0;

        let (assign13760_e19165, assign13760_e19165_d_n0, assign13760_e19165_d_n2, assign13760_e19165_d_n3, assign13760_e19165_d_n4, assign13760_e19165_d_n5, assign13760_e19165_d_n6, assign13760_e19165_d_n7, assign13760_e19165_d_n8, assign13760_e19165_d_n9, assign13760_e19165_d_n10, assign13760_e19165_d_n11, assign13760_e19165_d_n13, assign13760_e19165_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13760_e18944: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18946: f64 = (assign13760_e18944 - locals.var_qt1);
        let assign13760_e18948: f64 = (assign13760_e18946).powf(p.p1893);
        let assign13760_e18951: f64 = (924000.0 - 18100.0);
        let assign13760_e18952: f64 = (assign13760_e18948 * assign13760_e18951);
        let assign13760_e18955: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18956: f64 = (assign13760_e18952 / assign13760_e18955);
        let assign13760_e18957: f64 = (locals.var_t0 + assign13760_e18956);
        let assign13760_e18959: f64 = (assign13760_e18957 + 18100.0);
        let assign13760_e18963: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18965: f64 = (assign13760_e18963 - locals.var_qt1);
        let assign13760_e18967: f64 = (assign13760_e18965).powf(p.p1893);
        let assign13760_e18970: f64 = (924000.0 - 18100.0);
        let assign13760_e18971: f64 = (assign13760_e18967 * assign13760_e18970);
        let assign13760_e18974: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18975: f64 = (assign13760_e18971 / assign13760_e18974);
        let assign13760_e18976: f64 = (locals.var_t0 + assign13760_e18975);
        let assign13760_e18978: f64 = (assign13760_e18976 - 18100.0);
        let assign13760_e18982: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18984: f64 = (assign13760_e18982 - locals.var_qt1);
        let assign13760_e18986: f64 = (assign13760_e18984).powf(p.p1893);
        let assign13760_e18989: f64 = (924000.0 - 18100.0);
        let assign13760_e18990: f64 = (assign13760_e18986 * assign13760_e18989);
        let assign13760_e18993: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18994: f64 = (assign13760_e18990 / assign13760_e18993);
        let assign13760_e18995: f64 = (locals.var_t0 + assign13760_e18994);
        let assign13760_e18997: f64 = (assign13760_e18995 - 18100.0);
        let assign13760_e18998: f64 = (assign13760_e18978 * assign13760_e18997);
        let assign13760_e19001: f64 = (0.25 * 0.01);
        let assign13760_e19003: f64 = (assign13760_e19001 * 0.01);
        let assign13760_e19004: f64 = (assign13760_e18998 + assign13760_e19003);
        let assign13760_e19005: f64 = (assign13760_e19004).sqrt();
        let assign13760_e19006: f64 = (assign13760_e18959 + assign13760_e19005);
        let assign13760_e19007: f64 = (0.5 * assign13760_e19006);
        let assign13760_e19009: f64 = (assign13760_e19007 + 924000.0);
        let assign13760_e19014: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19016: f64 = (assign13760_e19014 - locals.var_qt1);
        let assign13760_e19018: f64 = (assign13760_e19016).powf(p.p1893);
        let assign13760_e19021: f64 = (924000.0 - 18100.0);
        let assign13760_e19022: f64 = (assign13760_e19018 * assign13760_e19021);
        let assign13760_e19025: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19026: f64 = (assign13760_e19022 / assign13760_e19025);
        let assign13760_e19027: f64 = (locals.var_t0 + assign13760_e19026);
        let assign13760_e19029: f64 = (assign13760_e19027 + 18100.0);
        let assign13760_e19033: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19035: f64 = (assign13760_e19033 - locals.var_qt1);
        let assign13760_e19037: f64 = (assign13760_e19035).powf(p.p1893);
        let assign13760_e19040: f64 = (924000.0 - 18100.0);
        let assign13760_e19041: f64 = (assign13760_e19037 * assign13760_e19040);
        let assign13760_e19044: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19045: f64 = (assign13760_e19041 / assign13760_e19044);
        let assign13760_e19046: f64 = (locals.var_t0 + assign13760_e19045);
        let assign13760_e19048: f64 = (assign13760_e19046 - 18100.0);
        let assign13760_e19052: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19054: f64 = (assign13760_e19052 - locals.var_qt1);
        let assign13760_e19056: f64 = (assign13760_e19054).powf(p.p1893);
        let assign13760_e19059: f64 = (924000.0 - 18100.0);
        let assign13760_e19060: f64 = (assign13760_e19056 * assign13760_e19059);
        let assign13760_e19063: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19064: f64 = (assign13760_e19060 / assign13760_e19063);
        let assign13760_e19065: f64 = (locals.var_t0 + assign13760_e19064);
        let assign13760_e19067: f64 = (assign13760_e19065 - 18100.0);
        let assign13760_e19068: f64 = (assign13760_e19048 * assign13760_e19067);
        let assign13760_e19071: f64 = (0.25 * 0.01);
        let assign13760_e19073: f64 = (assign13760_e19071 * 0.01);
        let assign13760_e19074: f64 = (assign13760_e19068 + assign13760_e19073);
        let assign13760_e19075: f64 = (assign13760_e19074).sqrt();
        let assign13760_e19076: f64 = (assign13760_e19029 + assign13760_e19075);
        let assign13760_e19077: f64 = (0.5 * assign13760_e19076);
        let assign13760_e19079: f64 = (assign13760_e19077 - 924000.0);
        let assign13760_e19084: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19086: f64 = (assign13760_e19084 - locals.var_qt1);
        let assign13760_e19088: f64 = (assign13760_e19086).powf(p.p1893);
        let assign13760_e19091: f64 = (924000.0 - 18100.0);
        let assign13760_e19092: f64 = (assign13760_e19088 * assign13760_e19091);
        let assign13760_e19095: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19096: f64 = (assign13760_e19092 / assign13760_e19095);
        let assign13760_e19097: f64 = (locals.var_t0 + assign13760_e19096);
        let assign13760_e19099: f64 = (assign13760_e19097 + 18100.0);
        let assign13760_e19103: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19105: f64 = (assign13760_e19103 - locals.var_qt1);
        let assign13760_e19107: f64 = (assign13760_e19105).powf(p.p1893);
        let assign13760_e19110: f64 = (924000.0 - 18100.0);
        let assign13760_e19111: f64 = (assign13760_e19107 * assign13760_e19110);
        let assign13760_e19114: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19115: f64 = (assign13760_e19111 / assign13760_e19114);
        let assign13760_e19116: f64 = (locals.var_t0 + assign13760_e19115);
        let assign13760_e19118: f64 = (assign13760_e19116 - 18100.0);
        let assign13760_e19122: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19124: f64 = (assign13760_e19122 - locals.var_qt1);
        let assign13760_e19126: f64 = (assign13760_e19124).powf(p.p1893);
        let assign13760_e19129: f64 = (924000.0 - 18100.0);
        let assign13760_e19130: f64 = (assign13760_e19126 * assign13760_e19129);
        let assign13760_e19133: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19134: f64 = (assign13760_e19130 / assign13760_e19133);
        let assign13760_e19135: f64 = (locals.var_t0 + assign13760_e19134);
        let assign13760_e19137: f64 = (assign13760_e19135 - 18100.0);
        let assign13760_e19138: f64 = (assign13760_e19118 * assign13760_e19137);
        let assign13760_e19141: f64 = (0.25 * 0.01);
        let assign13760_e19143: f64 = (assign13760_e19141 * 0.01);
        let assign13760_e19144: f64 = (assign13760_e19138 + assign13760_e19143);
        let assign13760_e19145: f64 = (assign13760_e19144).sqrt();
        let assign13760_e19146: f64 = (assign13760_e19099 + assign13760_e19145);
        let assign13760_e19147: f64 = (0.5 * assign13760_e19146);
        let assign13760_e19149: f64 = (assign13760_e19147 - 924000.0);
        let assign13760_e19150: f64 = (assign13760_e19079 * assign13760_e19149);
        let assign13760_e19153: f64 = (0.25 * 9240.0);
        let assign13760_e19155: f64 = (assign13760_e19153 * 9240.0);
        let assign13760_e19156: f64 = (assign13760_e19150 + assign13760_e19155);
        let assign13760_e19157: f64 = (assign13760_e19156).sqrt();
        let assign13760_e19158: f64 = (assign13760_e19009 - assign13760_e19157);
        let assign13760_e19159: f64 = (0.5 * assign13760_e19158);
        let assign13760_e19162: f64 = (0.25 * 9240.0);
        let assign13760_e19163: f64 = (assign13760_e19159 + assign13760_e19162);
        (assign13760_e19163, (0.5 * ((0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn0)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn0)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn0)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn2)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn2)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn2)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn3)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn3)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn3)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn4)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn4)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn4)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn5)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn5)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn5)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn6)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn6)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn6)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn7)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn7)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn7)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn8)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn8)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn8)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn9)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn9)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn9)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn10)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn10)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn10)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn11)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn11)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn11)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn13)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn13)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn13)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn14)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn14)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn14)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))),)
    } else {
        (locals.var_ne2h, locals.var_ne2h_dn0, locals.var_ne2h_dn2, locals.var_ne2h_dn3, locals.var_ne2h_dn4, locals.var_ne2h_dn5, locals.var_ne2h_dn6, locals.var_ne2h_dn7, locals.var_ne2h_dn8, locals.var_ne2h_dn9, locals.var_ne2h_dn10, locals.var_ne2h_dn11, locals.var_ne2h_dn13, locals.var_ne2h_dn14,)
    }
};
        locals.var_ne2h = assign13760_e19165;
        locals.var_ne2h_dn0 = assign13760_e19165_d_n0;
        locals.var_ne2h_dn2 = assign13760_e19165_d_n2;
        locals.var_ne2h_dn3 = assign13760_e19165_d_n3;
        locals.var_ne2h_dn4 = assign13760_e19165_d_n4;
        locals.var_ne2h_dn5 = assign13760_e19165_d_n5;
        locals.var_ne2h_dn6 = assign13760_e19165_d_n6;
        locals.var_ne2h_dn7 = assign13760_e19165_d_n7;
        locals.var_ne2h_dn8 = assign13760_e19165_d_n8;
        locals.var_ne2h_dn9 = assign13760_e19165_d_n9;
        locals.var_ne2h_dn10 = assign13760_e19165_d_n10;
        locals.var_ne2h_dn11 = assign13760_e19165_d_n11;
        locals.var_ne2h_dn13 = assign13760_e19165_d_n13;
        locals.var_ne2h_dn14 = assign13760_e19165_d_n14;
        locals.var_ne2h_rv = 0.0;

        let (assign13770_e19393,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13770_e19172: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19174: f64 = (assign13770_e19172 - locals.var_qt1);
        let assign13770_e19176: f64 = (assign13770_e19174).powf(p.p1894);
        let assign13770_e19179: f64 = (8.0 - 5.5);
        let assign13770_e19180: f64 = (assign13770_e19176 * assign13770_e19179);
        let assign13770_e19183: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19184: f64 = (assign13770_e19180 / assign13770_e19183);
        let assign13770_e19185: f64 = (5.5 + assign13770_e19184);
        let assign13770_e19187: f64 = assign13770_e19185;
        let assign13770_e19191: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19193: f64 = (assign13770_e19191 - locals.var_qt1);
        let assign13770_e19195: f64 = (assign13770_e19193).powf(p.p1894);
        let assign13770_e19198: f64 = (8.0 - 5.5);
        let assign13770_e19199: f64 = (assign13770_e19195 * assign13770_e19198);
        let assign13770_e19202: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19203: f64 = (assign13770_e19199 / assign13770_e19202);
        let assign13770_e19204: f64 = (5.5 + assign13770_e19203);
        let assign13770_e19206: f64 = assign13770_e19204;
        let assign13770_e19210: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19212: f64 = (assign13770_e19210 - locals.var_qt1);
        let assign13770_e19214: f64 = (assign13770_e19212).powf(p.p1894);
        let assign13770_e19217: f64 = (8.0 - 5.5);
        let assign13770_e19218: f64 = (assign13770_e19214 * assign13770_e19217);
        let assign13770_e19221: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19222: f64 = (assign13770_e19218 / assign13770_e19221);
        let assign13770_e19223: f64 = (5.5 + assign13770_e19222);
        let assign13770_e19225: f64 = assign13770_e19223;
        let assign13770_e19226: f64 = (assign13770_e19206 * assign13770_e19225);
        let assign13770_e19229: f64 = (0.25 * 0.01);
        let assign13770_e19231: f64 = (assign13770_e19229 * 0.01);
        let assign13770_e19232: f64 = (assign13770_e19226 + assign13770_e19231);
        let assign13770_e19233: f64 = (assign13770_e19232).sqrt();
        let assign13770_e19234: f64 = (assign13770_e19187 + assign13770_e19233);
        let assign13770_e19235: f64 = (0.5 * assign13770_e19234);
        let assign13770_e19237: f64 = (assign13770_e19235 + 8.0);
        let assign13770_e19242: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19244: f64 = (assign13770_e19242 - locals.var_qt1);
        let assign13770_e19246: f64 = (assign13770_e19244).powf(p.p1894);
        let assign13770_e19249: f64 = (8.0 - 5.5);
        let assign13770_e19250: f64 = (assign13770_e19246 * assign13770_e19249);
        let assign13770_e19253: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19254: f64 = (assign13770_e19250 / assign13770_e19253);
        let assign13770_e19255: f64 = (5.5 + assign13770_e19254);
        let assign13770_e19257: f64 = assign13770_e19255;
        let assign13770_e19261: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19263: f64 = (assign13770_e19261 - locals.var_qt1);
        let assign13770_e19265: f64 = (assign13770_e19263).powf(p.p1894);
        let assign13770_e19268: f64 = (8.0 - 5.5);
        let assign13770_e19269: f64 = (assign13770_e19265 * assign13770_e19268);
        let assign13770_e19272: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19273: f64 = (assign13770_e19269 / assign13770_e19272);
        let assign13770_e19274: f64 = (5.5 + assign13770_e19273);
        let assign13770_e19276: f64 = assign13770_e19274;
        let assign13770_e19280: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19282: f64 = (assign13770_e19280 - locals.var_qt1);
        let assign13770_e19284: f64 = (assign13770_e19282).powf(p.p1894);
        let assign13770_e19287: f64 = (8.0 - 5.5);
        let assign13770_e19288: f64 = (assign13770_e19284 * assign13770_e19287);
        let assign13770_e19291: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19292: f64 = (assign13770_e19288 / assign13770_e19291);
        let assign13770_e19293: f64 = (5.5 + assign13770_e19292);
        let assign13770_e19295: f64 = assign13770_e19293;
        let assign13770_e19296: f64 = (assign13770_e19276 * assign13770_e19295);
        let assign13770_e19299: f64 = (0.25 * 0.01);
        let assign13770_e19301: f64 = (assign13770_e19299 * 0.01);
        let assign13770_e19302: f64 = (assign13770_e19296 + assign13770_e19301);
        let assign13770_e19303: f64 = (assign13770_e19302).sqrt();
        let assign13770_e19304: f64 = (assign13770_e19257 + assign13770_e19303);
        let assign13770_e19305: f64 = (0.5 * assign13770_e19304);
        let assign13770_e19307: f64 = (assign13770_e19305 - 8.0);
        let assign13770_e19312: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19314: f64 = (assign13770_e19312 - locals.var_qt1);
        let assign13770_e19316: f64 = (assign13770_e19314).powf(p.p1894);
        let assign13770_e19319: f64 = (8.0 - 5.5);
        let assign13770_e19320: f64 = (assign13770_e19316 * assign13770_e19319);
        let assign13770_e19323: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19324: f64 = (assign13770_e19320 / assign13770_e19323);
        let assign13770_e19325: f64 = (5.5 + assign13770_e19324);
        let assign13770_e19327: f64 = assign13770_e19325;
        let assign13770_e19331: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19333: f64 = (assign13770_e19331 - locals.var_qt1);
        let assign13770_e19335: f64 = (assign13770_e19333).powf(p.p1894);
        let assign13770_e19338: f64 = (8.0 - 5.5);
        let assign13770_e19339: f64 = (assign13770_e19335 * assign13770_e19338);
        let assign13770_e19342: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19343: f64 = (assign13770_e19339 / assign13770_e19342);
        let assign13770_e19344: f64 = (5.5 + assign13770_e19343);
        let assign13770_e19346: f64 = assign13770_e19344;
        let assign13770_e19350: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19352: f64 = (assign13770_e19350 - locals.var_qt1);
        let assign13770_e19354: f64 = (assign13770_e19352).powf(p.p1894);
        let assign13770_e19357: f64 = (8.0 - 5.5);
        let assign13770_e19358: f64 = (assign13770_e19354 * assign13770_e19357);
        let assign13770_e19361: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19362: f64 = (assign13770_e19358 / assign13770_e19361);
        let assign13770_e19363: f64 = (5.5 + assign13770_e19362);
        let assign13770_e19365: f64 = assign13770_e19363;
        let assign13770_e19366: f64 = (assign13770_e19346 * assign13770_e19365);
        let assign13770_e19369: f64 = (0.25 * 0.01);
        let assign13770_e19371: f64 = (assign13770_e19369 * 0.01);
        let assign13770_e19372: f64 = (assign13770_e19366 + assign13770_e19371);
        let assign13770_e19373: f64 = (assign13770_e19372).sqrt();
        let assign13770_e19374: f64 = (assign13770_e19327 + assign13770_e19373);
        let assign13770_e19375: f64 = (0.5 * assign13770_e19374);
        let assign13770_e19377: f64 = (assign13770_e19375 - 8.0);
        let assign13770_e19378: f64 = (assign13770_e19307 * assign13770_e19377);
        let assign13770_e19381: f64 = (0.25 * 0.01);
        let assign13770_e19383: f64 = (assign13770_e19381 * 0.01);
        let assign13770_e19384: f64 = (assign13770_e19378 + assign13770_e19383);
        let assign13770_e19385: f64 = (assign13770_e19384).sqrt();
        let assign13770_e19386: f64 = (assign13770_e19237 - assign13770_e19385);
        let assign13770_e19387: f64 = (0.5 * assign13770_e19386);
        let assign13770_e19390: f64 = (0.25 * 0.01);
        let assign13770_e19391: f64 = (assign13770_e19387 + assign13770_e19390);
        (assign13770_e19391,)
    } else {
        (locals.var_pe2h,)
    }
};
        locals.var_pe2h = assign13770_e19393;
        locals.var_pe2h_rv = 0.0;

        let (assign13780_e19407,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13780_e19398: f64 = (4.0_f64).powf(p.p1895);
        let assign13780_e19399: f64 = (120.66 * assign13780_e19398);
        let assign13780_e19402: f64 = (p.p40 * 1000000000.0);
        let assign13780_e19404: f64 = (assign13780_e19402).powf(p.p1895);
        let assign13780_e19405: f64 = (assign13780_e19399 / assign13780_e19404);
        (assign13780_e19405,)
    } else {
        (locals.var_ne3h,)
    }
};
        locals.var_ne3h = assign13780_e19407;
        locals.var_ne3h_rv = 0.0;

        let (assign13790_e19421,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13790_e19412: f64 = (4.0_f64).powf(p.p1896);
        let assign13790_e19413: f64 = (2.0 * assign13790_e19412);
        let assign13790_e19416: f64 = (p.p40 * 1000000000.0);
        let assign13790_e19418: f64 = (assign13790_e19416).powf(p.p1896);
        let assign13790_e19419: f64 = (assign13790_e19413 / assign13790_e19418);
        (assign13790_e19419,)
    } else {
        (locals.var_pe3h,)
    }
};
        locals.var_pe3h = assign13790_e19421;
        locals.var_pe3h_rv = 0.0;

        let (assign13800_e19435,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13800_e19426: f64 = (4.0_f64).powf(p.p1897);
        let assign13800_e19427: f64 = (107.0 * assign13800_e19426);
        let assign13800_e19430: f64 = (p.p40 * 1000000000.0);
        let assign13800_e19432: f64 = (assign13800_e19430).powf(p.p1897);
        let assign13800_e19433: f64 = (assign13800_e19427 / assign13800_e19432);
        (assign13800_e19433,)
    } else {
        (locals.var_nc1l0,)
    }
};
        locals.var_nc1l0 = assign13800_e19435;
        locals.var_nc1l0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13810_e19609,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13810_e19442: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19444: f64 = (assign13810_e19442 - locals.var_qt1);
        let assign13810_e19446: f64 = (assign13810_e19444).powf(p.p1898);
        let assign13810_e19448: f64 = (assign13810_e19446 * 0.1);
        let assign13810_e19449: f64 = (0.7 + assign13810_e19448);
        let assign13810_e19451: f64 = (assign13810_e19449 + 0.5);
        let assign13810_e19455: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19457: f64 = (assign13810_e19455 - locals.var_qt1);
        let assign13810_e19459: f64 = (assign13810_e19457).powf(p.p1898);
        let assign13810_e19461: f64 = (assign13810_e19459 * 0.1);
        let assign13810_e19462: f64 = (0.7 + assign13810_e19461);
        let assign13810_e19464: f64 = (assign13810_e19462 - 0.5);
        let assign13810_e19468: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19470: f64 = (assign13810_e19468 - locals.var_qt1);
        let assign13810_e19472: f64 = (assign13810_e19470).powf(p.p1898);
        let assign13810_e19474: f64 = (assign13810_e19472 * 0.1);
        let assign13810_e19475: f64 = (0.7 + assign13810_e19474);
        let assign13810_e19477: f64 = (assign13810_e19475 - 0.5);
        let assign13810_e19478: f64 = (assign13810_e19464 * assign13810_e19477);
        let assign13810_e19481: f64 = (0.25 * 0.01);
        let assign13810_e19483: f64 = (assign13810_e19481 * 0.01);
        let assign13810_e19484: f64 = (assign13810_e19478 + assign13810_e19483);
        let assign13810_e19485: f64 = (assign13810_e19484).sqrt();
        let assign13810_e19486: f64 = (assign13810_e19451 + assign13810_e19485);
        let assign13810_e19487: f64 = (0.5 * assign13810_e19486);
        let assign13810_e19489: f64 = (assign13810_e19487 + 1.0);
        let assign13810_e19494: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19496: f64 = (assign13810_e19494 - locals.var_qt1);
        let assign13810_e19498: f64 = (assign13810_e19496).powf(p.p1898);
        let assign13810_e19500: f64 = (assign13810_e19498 * 0.1);
        let assign13810_e19501: f64 = (0.7 + assign13810_e19500);
        let assign13810_e19503: f64 = (assign13810_e19501 + 0.5);
        let assign13810_e19507: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19509: f64 = (assign13810_e19507 - locals.var_qt1);
        let assign13810_e19511: f64 = (assign13810_e19509).powf(p.p1898);
        let assign13810_e19513: f64 = (assign13810_e19511 * 0.1);
        let assign13810_e19514: f64 = (0.7 + assign13810_e19513);
        let assign13810_e19516: f64 = (assign13810_e19514 - 0.5);
        let assign13810_e19520: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19522: f64 = (assign13810_e19520 - locals.var_qt1);
        let assign13810_e19524: f64 = (assign13810_e19522).powf(p.p1898);
        let assign13810_e19526: f64 = (assign13810_e19524 * 0.1);
        let assign13810_e19527: f64 = (0.7 + assign13810_e19526);
        let assign13810_e19529: f64 = (assign13810_e19527 - 0.5);
        let assign13810_e19530: f64 = (assign13810_e19516 * assign13810_e19529);
        let assign13810_e19533: f64 = (0.25 * 0.01);
        let assign13810_e19535: f64 = (assign13810_e19533 * 0.01);
        let assign13810_e19536: f64 = (assign13810_e19530 + assign13810_e19535);
        let assign13810_e19537: f64 = (assign13810_e19536).sqrt();
        let assign13810_e19538: f64 = (assign13810_e19503 + assign13810_e19537);
        let assign13810_e19539: f64 = (0.5 * assign13810_e19538);
        let assign13810_e19541: f64 = (assign13810_e19539 - 1.0);
        let assign13810_e19546: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19548: f64 = (assign13810_e19546 - locals.var_qt1);
        let assign13810_e19550: f64 = (assign13810_e19548).powf(p.p1898);
        let assign13810_e19552: f64 = (assign13810_e19550 * 0.1);
        let assign13810_e19553: f64 = (0.7 + assign13810_e19552);
        let assign13810_e19555: f64 = (assign13810_e19553 + 0.5);
        let assign13810_e19559: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19561: f64 = (assign13810_e19559 - locals.var_qt1);
        let assign13810_e19563: f64 = (assign13810_e19561).powf(p.p1898);
        let assign13810_e19565: f64 = (assign13810_e19563 * 0.1);
        let assign13810_e19566: f64 = (0.7 + assign13810_e19565);
        let assign13810_e19568: f64 = (assign13810_e19566 - 0.5);
        let assign13810_e19572: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19574: f64 = (assign13810_e19572 - locals.var_qt1);
        let assign13810_e19576: f64 = (assign13810_e19574).powf(p.p1898);
        let assign13810_e19578: f64 = (assign13810_e19576 * 0.1);
        let assign13810_e19579: f64 = (0.7 + assign13810_e19578);
        let assign13810_e19581: f64 = (assign13810_e19579 - 0.5);
        let assign13810_e19582: f64 = (assign13810_e19568 * assign13810_e19581);
        let assign13810_e19585: f64 = (0.25 * 0.01);
        let assign13810_e19587: f64 = (assign13810_e19585 * 0.01);
        let assign13810_e19588: f64 = (assign13810_e19582 + assign13810_e19587);
        let assign13810_e19589: f64 = (assign13810_e19588).sqrt();
        let assign13810_e19590: f64 = (assign13810_e19555 + assign13810_e19589);
        let assign13810_e19591: f64 = (0.5 * assign13810_e19590);
        let assign13810_e19593: f64 = (assign13810_e19591 - 1.0);
        let assign13810_e19594: f64 = (assign13810_e19541 * assign13810_e19593);
        let assign13810_e19597: f64 = (0.25 * 0.01);
        let assign13810_e19599: f64 = (assign13810_e19597 * 0.01);
        let assign13810_e19600: f64 = (assign13810_e19594 + assign13810_e19599);
        let assign13810_e19601: f64 = (assign13810_e19600).sqrt();
        let assign13810_e19602: f64 = (assign13810_e19489 - assign13810_e19601);
        let assign13810_e19603: f64 = (0.5 * assign13810_e19602);
        let assign13810_e19606: f64 = (0.25 * 0.01);
        let assign13810_e19607: f64 = (assign13810_e19603 + assign13810_e19606);
        (assign13810_e19607,)
    } else {
        (locals.var_pnc1l,)
    }
};
        locals.var_pnc1l = assign13810_e19609;
        locals.var_pnc1l_rv = 0.0;

        let (assign13820_e19623,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13820_e19614: f64 = (4.0_f64).powf(p.p1899);
        let assign13820_e19615: f64 = (103.0 * assign13820_e19614);
        let assign13820_e19618: f64 = (p.p40 * 1000000000.0);
        let assign13820_e19620: f64 = (assign13820_e19618).powf(p.p1899);
        let assign13820_e19621: f64 = (assign13820_e19615 / assign13820_e19620);
        (assign13820_e19621,)
    } else {
        (locals.var_nc2l0,)
    }
};
        locals.var_nc2l0 = assign13820_e19623;
        locals.var_nc2l0_rv = 0.0;

        let (assign13830_e19637,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13830_e19628: f64 = (4.0_f64).powf(p.p1900);
        let assign13830_e19629: f64 = (1.5 * assign13830_e19628);
        let assign13830_e19632: f64 = (p.p40 * 1000000000.0);
        let assign13830_e19634: f64 = (assign13830_e19632).powf(p.p1900);
        let assign13830_e19635: f64 = (assign13830_e19629 / assign13830_e19634);
        (assign13830_e19635,)
    } else {
        (locals.var_pnc2l,)
    }
};
        locals.var_pnc2l = assign13830_e19637;
        locals.var_pnc2l_rv = 0.0;

        let (assign13840_e19651,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13840_e19642: f64 = (4.0_f64).powf(p.p1901);
        let assign13840_e19643: f64 = (833.0 * assign13840_e19642);
        let assign13840_e19646: f64 = (p.p40 * 1000000000.0);
        let assign13840_e19648: f64 = (assign13840_e19646).powf(p.p1901);
        let assign13840_e19649: f64 = (assign13840_e19643 / assign13840_e19648);
        (assign13840_e19649,)
    } else {
        (locals.var_nc3l0,)
    }
};
        locals.var_nc3l0 = assign13840_e19651;
        locals.var_nc3l0_rv = 0.0;

        let (assign13850_e19665,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13850_e19656: f64 = (4.0_f64).powf(p.p1902);
        let assign13850_e19657: f64 = (3.4 * assign13850_e19656);
        let assign13850_e19660: f64 = (p.p40 * 1000000000.0);
        let assign13850_e19662: f64 = (assign13850_e19660).powf(p.p1902);
        let assign13850_e19663: f64 = (assign13850_e19657 / assign13850_e19662);
        (assign13850_e19663,)
    } else {
        (locals.var_pnc3l,)
    }
};
        locals.var_pnc3l = assign13850_e19665;
        locals.var_pnc3l_rv = 0.0;

        let (assign13860_e19677, assign13860_e19677_d_n0, assign13860_e19677_d_n2, assign13860_e19677_d_n3, assign13860_e19677_d_n4, assign13860_e19677_d_n5, assign13860_e19677_d_n6, assign13860_e19677_d_n7, assign13860_e19677_d_n8, assign13860_e19677_d_n9, assign13860_e19677_d_n10, assign13860_e19677_d_n11, assign13860_e19677_d_n13, assign13860_e19677_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13860_e19670: f64 = (p.p1852 * 1000000000.0);
        let assign13860_e19673: f64 = (p.p1867 * locals.var_pe2h);
        let assign13860_e19674: f64 = (assign13860_e19670).powf(assign13860_e19673);
        let assign13860_e19675: f64 = (locals.var_ne2h / assign13860_e19674);
        (assign13860_e19675, (locals.var_ne2h_dn0 / assign13860_e19674), (locals.var_ne2h_dn2 / assign13860_e19674), (locals.var_ne2h_dn3 / assign13860_e19674), (locals.var_ne2h_dn4 / assign13860_e19674), (locals.var_ne2h_dn5 / assign13860_e19674), (locals.var_ne2h_dn6 / assign13860_e19674), (locals.var_ne2h_dn7 / assign13860_e19674), (locals.var_ne2h_dn8 / assign13860_e19674), (locals.var_ne2h_dn9 / assign13860_e19674), (locals.var_ne2h_dn10 / assign13860_e19674), (locals.var_ne2h_dn11 / assign13860_e19674), (locals.var_ne2h_dn13 / assign13860_e19674), (locals.var_ne2h_dn14 / assign13860_e19674),)
    } else {
        (locals.var_qe2n, locals.var_qe2n_dn0, locals.var_qe2n_dn2, locals.var_qe2n_dn3, locals.var_qe2n_dn4, locals.var_qe2n_dn5, locals.var_qe2n_dn6, locals.var_qe2n_dn7, locals.var_qe2n_dn8, locals.var_qe2n_dn9, locals.var_qe2n_dn10, locals.var_qe2n_dn11, locals.var_qe2n_dn13, locals.var_qe2n_dn14,)
    }
};
        locals.var_qe2n = assign13860_e19677;
        locals.var_qe2n_dn0 = assign13860_e19677_d_n0;
        locals.var_qe2n_dn2 = assign13860_e19677_d_n2;
        locals.var_qe2n_dn3 = assign13860_e19677_d_n3;
        locals.var_qe2n_dn4 = assign13860_e19677_d_n4;
        locals.var_qe2n_dn5 = assign13860_e19677_d_n5;
        locals.var_qe2n_dn6 = assign13860_e19677_d_n6;
        locals.var_qe2n_dn7 = assign13860_e19677_d_n7;
        locals.var_qe2n_dn8 = assign13860_e19677_d_n8;
        locals.var_qe2n_dn9 = assign13860_e19677_d_n9;
        locals.var_qe2n_dn10 = assign13860_e19677_d_n10;
        locals.var_qe2n_dn11 = assign13860_e19677_d_n11;
        locals.var_qe2n_dn13 = assign13860_e19677_d_n13;
        locals.var_qe2n_dn14 = assign13860_e19677_d_n14;
        locals.var_qe2n_rv = 0.0;

        let (assign13870_e19689,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13870_e19682: f64 = (p.p1852 * 1000000000.0);
        let assign13870_e19685: f64 = (p.p1868 * locals.var_pe3h);
        let assign13870_e19686: f64 = (assign13870_e19682).powf(assign13870_e19685);
        let assign13870_e19687: f64 = (locals.var_ne3h / assign13870_e19686);
        (assign13870_e19687,)
    } else {
        (locals.var_qe3n,)
    }
};
        locals.var_qe3n = assign13870_e19689;
        locals.var_qe3n_rv = 0.0;

        let (assign13880_e19754, assign13880_e19754_d_n0, assign13880_e19754_d_n2, assign13880_e19754_d_n3, assign13880_e19754_d_n4, assign13880_e19754_d_n5, assign13880_e19754_d_n6, assign13880_e19754_d_n7, assign13880_e19754_d_n8, assign13880_e19754_d_n9, assign13880_e19754_d_n10, assign13880_e19754_d_n11, assign13880_e19754_d_n13, assign13880_e19754_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13880_e19697: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19700: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19701: f64 = (assign13880_e19697).powf(assign13880_e19700);
        let assign13880_e19702: f64 = (locals.var_ne2h / assign13880_e19701);
        let assign13880_e19704: f64 = (assign13880_e19702 - locals.var_qe2n);
        let assign13880_e19705: f64 = (p.p1865 * assign13880_e19704);
        let assign13880_e19706: f64 = (locals.var_e2nom_i + assign13880_e19705);
        let assign13880_e19708: f64 = assign13880_e19706;
        let assign13880_e19714: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19717: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19718: f64 = (assign13880_e19714).powf(assign13880_e19717);
        let assign13880_e19719: f64 = (locals.var_ne2h / assign13880_e19718);
        let assign13880_e19721: f64 = (assign13880_e19719 - locals.var_qe2n);
        let assign13880_e19722: f64 = (p.p1865 * assign13880_e19721);
        let assign13880_e19723: f64 = (locals.var_e2nom_i + assign13880_e19722);
        let assign13880_e19725: f64 = assign13880_e19723;
        let assign13880_e19731: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19734: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19735: f64 = (assign13880_e19731).powf(assign13880_e19734);
        let assign13880_e19736: f64 = (locals.var_ne2h / assign13880_e19735);
        let assign13880_e19738: f64 = (assign13880_e19736 - locals.var_qe2n);
        let assign13880_e19739: f64 = (p.p1865 * assign13880_e19738);
        let assign13880_e19740: f64 = (locals.var_e2nom_i + assign13880_e19739);
        let assign13880_e19742: f64 = assign13880_e19740;
        let assign13880_e19743: f64 = (assign13880_e19725 * assign13880_e19742);
        let assign13880_e19746: f64 = (0.25 * 0.01);
        let assign13880_e19748: f64 = (assign13880_e19746 * 0.01);
        let assign13880_e19749: f64 = (assign13880_e19743 + assign13880_e19748);
        let assign13880_e19750: f64 = (assign13880_e19749).sqrt();
        let assign13880_e19751: f64 = (assign13880_e19708 + assign13880_e19750);
        let assign13880_e19752: f64 = (0.5 * assign13880_e19751);
        (assign13880_e19752, (0.5 * ((p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19701) - locals.var_qe2n_dn0)) + ((((p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19718) - locals.var_qe2n_dn0)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19735) - locals.var_qe2n_dn0)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19701) - locals.var_qe2n_dn2)) + ((((p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19718) - locals.var_qe2n_dn2)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19735) - locals.var_qe2n_dn2)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19701) - locals.var_qe2n_dn3)) + ((((p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19718) - locals.var_qe2n_dn3)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19735) - locals.var_qe2n_dn3)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19701) - locals.var_qe2n_dn4)) + ((((p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19718) - locals.var_qe2n_dn4)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19735) - locals.var_qe2n_dn4)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19701) - locals.var_qe2n_dn5)) + ((((p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19718) - locals.var_qe2n_dn5)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19735) - locals.var_qe2n_dn5)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19701) - locals.var_qe2n_dn6)) + ((((p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19718) - locals.var_qe2n_dn6)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19735) - locals.var_qe2n_dn6)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19701) - locals.var_qe2n_dn7)) + ((((p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19718) - locals.var_qe2n_dn7)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19735) - locals.var_qe2n_dn7)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19701) - locals.var_qe2n_dn8)) + ((((p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19718) - locals.var_qe2n_dn8)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19735) - locals.var_qe2n_dn8)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19701) - locals.var_qe2n_dn9)) + ((((p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19718) - locals.var_qe2n_dn9)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19735) - locals.var_qe2n_dn9)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19701) - locals.var_qe2n_dn10)) + ((((p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19718) - locals.var_qe2n_dn10)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19735) - locals.var_qe2n_dn10)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19701) - locals.var_qe2n_dn11)) + ((((p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19718) - locals.var_qe2n_dn11)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19735) - locals.var_qe2n_dn11)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19701) - locals.var_qe2n_dn13)) + ((((p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19718) - locals.var_qe2n_dn13)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19735) - locals.var_qe2n_dn13)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19701) - locals.var_qe2n_dn14)) + ((((p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19718) - locals.var_qe2n_dn14)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19735) - locals.var_qe2n_dn14)))) / (2.0 * assign13880_e19750)))),)
    } else {
        (locals.var_qe2, locals.var_qe2_dn0, locals.var_qe2_dn2, locals.var_qe2_dn3, locals.var_qe2_dn4, locals.var_qe2_dn5, locals.var_qe2_dn6, locals.var_qe2_dn7, locals.var_qe2_dn8, locals.var_qe2_dn9, locals.var_qe2_dn10, locals.var_qe2_dn11, locals.var_qe2_dn13, locals.var_qe2_dn14,)
    }
};
        locals.var_qe2 = assign13880_e19754;
        locals.var_qe2_dn0 = assign13880_e19754_d_n0;
        locals.var_qe2_dn2 = assign13880_e19754_d_n2;
        locals.var_qe2_dn3 = assign13880_e19754_d_n3;
        locals.var_qe2_dn4 = assign13880_e19754_d_n4;
        locals.var_qe2_dn5 = assign13880_e19754_d_n5;
        locals.var_qe2_dn6 = assign13880_e19754_d_n6;
        locals.var_qe2_dn7 = assign13880_e19754_d_n7;
        locals.var_qe2_dn8 = assign13880_e19754_d_n8;
        locals.var_qe2_dn9 = assign13880_e19754_d_n9;
        locals.var_qe2_dn10 = assign13880_e19754_d_n10;
        locals.var_qe2_dn11 = assign13880_e19754_d_n11;
        locals.var_qe2_dn13 = assign13880_e19754_d_n13;
        locals.var_qe2_dn14 = assign13880_e19754_d_n14;
        locals.var_qe2_rv = 0.0;

        let (assign13890_e19819,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13890_e19762: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19765: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19766: f64 = (assign13890_e19762).powf(assign13890_e19765);
        let assign13890_e19767: f64 = (locals.var_ne3h / assign13890_e19766);
        let assign13890_e19769: f64 = (assign13890_e19767 - locals.var_qe3n);
        let assign13890_e19770: f64 = (p.p1866 * assign13890_e19769);
        let assign13890_e19771: f64 = (locals.var_e3nom_i + assign13890_e19770);
        let assign13890_e19773: f64 = assign13890_e19771;
        let assign13890_e19779: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19782: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19783: f64 = (assign13890_e19779).powf(assign13890_e19782);
        let assign13890_e19784: f64 = (locals.var_ne3h / assign13890_e19783);
        let assign13890_e19786: f64 = (assign13890_e19784 - locals.var_qe3n);
        let assign13890_e19787: f64 = (p.p1866 * assign13890_e19786);
        let assign13890_e19788: f64 = (locals.var_e3nom_i + assign13890_e19787);
        let assign13890_e19790: f64 = assign13890_e19788;
        let assign13890_e19796: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19799: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19800: f64 = (assign13890_e19796).powf(assign13890_e19799);
        let assign13890_e19801: f64 = (locals.var_ne3h / assign13890_e19800);
        let assign13890_e19803: f64 = (assign13890_e19801 - locals.var_qe3n);
        let assign13890_e19804: f64 = (p.p1866 * assign13890_e19803);
        let assign13890_e19805: f64 = (locals.var_e3nom_i + assign13890_e19804);
        let assign13890_e19807: f64 = assign13890_e19805;
        let assign13890_e19808: f64 = (assign13890_e19790 * assign13890_e19807);
        let assign13890_e19811: f64 = (0.25 * 0.01);
        let assign13890_e19813: f64 = (assign13890_e19811 * 0.01);
        let assign13890_e19814: f64 = (assign13890_e19808 + assign13890_e19813);
        let assign13890_e19815: f64 = (assign13890_e19814).sqrt();
        let assign13890_e19816: f64 = (assign13890_e19773 + assign13890_e19815);
        let assign13890_e19817: f64 = (0.5 * assign13890_e19816);
        (assign13890_e19817,)
    } else {
        (locals.var_qe3,)
    }
};
        locals.var_qe3 = assign13890_e19819;
        locals.var_qe3_rv = 0.0;

        let (assign13900_e19884,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13900_e19827: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19830: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19831: f64 = (assign13900_e19827).powf(assign13900_e19830);
        let assign13900_e19832: f64 = (5.0 * assign13900_e19831);
        let assign13900_e19833: f64 = (1.0 + assign13900_e19832);
        let assign13900_e19835: f64 = (assign13900_e19833).powf(0.5);
        let assign13900_e19836: f64 = (locals.var_nc1l0 / assign13900_e19835);
        let assign13900_e19838: f64 = assign13900_e19836;
        let assign13900_e19844: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19847: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19848: f64 = (assign13900_e19844).powf(assign13900_e19847);
        let assign13900_e19849: f64 = (5.0 * assign13900_e19848);
        let assign13900_e19850: f64 = (1.0 + assign13900_e19849);
        let assign13900_e19852: f64 = (assign13900_e19850).powf(0.5);
        let assign13900_e19853: f64 = (locals.var_nc1l0 / assign13900_e19852);
        let assign13900_e19855: f64 = assign13900_e19853;
        let assign13900_e19861: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19864: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19865: f64 = (assign13900_e19861).powf(assign13900_e19864);
        let assign13900_e19866: f64 = (5.0 * assign13900_e19865);
        let assign13900_e19867: f64 = (1.0 + assign13900_e19866);
        let assign13900_e19869: f64 = (assign13900_e19867).powf(0.5);
        let assign13900_e19870: f64 = (locals.var_nc1l0 / assign13900_e19869);
        let assign13900_e19872: f64 = assign13900_e19870;
        let assign13900_e19873: f64 = (assign13900_e19855 * assign13900_e19872);
        let assign13900_e19876: f64 = (0.25 * 0.1);
        let assign13900_e19878: f64 = (assign13900_e19876 * 0.1);
        let assign13900_e19879: f64 = (assign13900_e19873 + assign13900_e19878);
        let assign13900_e19880: f64 = (assign13900_e19879).sqrt();
        let assign13900_e19881: f64 = (assign13900_e19838 + assign13900_e19880);
        let assign13900_e19882: f64 = (0.5 * assign13900_e19881);
        (assign13900_e19882,)
    } else {
        (locals.var_nc1l,)
    }
};
        locals.var_nc1l = assign13900_e19884;
        locals.var_nc1l_rv = 0.0;

        let (assign13910_e19949,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13910_e19892: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19895: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19896: f64 = (assign13910_e19892).powf(assign13910_e19895);
        let assign13910_e19897: f64 = (5.0 * assign13910_e19896);
        let assign13910_e19898: f64 = (1.0 + assign13910_e19897);
        let assign13910_e19900: f64 = (assign13910_e19898).powf(0.5);
        let assign13910_e19901: f64 = (locals.var_nc1l0 / assign13910_e19900);
        let assign13910_e19903: f64 = assign13910_e19901;
        let assign13910_e19909: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19912: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19913: f64 = (assign13910_e19909).powf(assign13910_e19912);
        let assign13910_e19914: f64 = (5.0 * assign13910_e19913);
        let assign13910_e19915: f64 = (1.0 + assign13910_e19914);
        let assign13910_e19917: f64 = (assign13910_e19915).powf(0.5);
        let assign13910_e19918: f64 = (locals.var_nc1l0 / assign13910_e19917);
        let assign13910_e19920: f64 = assign13910_e19918;
        let assign13910_e19926: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19929: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19930: f64 = (assign13910_e19926).powf(assign13910_e19929);
        let assign13910_e19931: f64 = (5.0 * assign13910_e19930);
        let assign13910_e19932: f64 = (1.0 + assign13910_e19931);
        let assign13910_e19934: f64 = (assign13910_e19932).powf(0.5);
        let assign13910_e19935: f64 = (locals.var_nc1l0 / assign13910_e19934);
        let assign13910_e19937: f64 = assign13910_e19935;
        let assign13910_e19938: f64 = (assign13910_e19920 * assign13910_e19937);
        let assign13910_e19941: f64 = (0.25 * 0.1);
        let assign13910_e19943: f64 = (assign13910_e19941 * 0.1);
        let assign13910_e19944: f64 = (assign13910_e19938 + assign13910_e19943);
        let assign13910_e19945: f64 = (assign13910_e19944).sqrt();
        let assign13910_e19946: f64 = (assign13910_e19903 + assign13910_e19945);
        let assign13910_e19947: f64 = (0.5 * assign13910_e19946);
        (assign13910_e19947,)
    } else {
        (locals.var_nc1ln,)
    }
};
        locals.var_nc1ln = assign13910_e19949;
        locals.var_nc1ln_rv = 0.0;

        let (assign13920_e19959,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13920_e19955: f64 = (locals.var_nc1l - locals.var_nc1ln);
        let assign13920_e19956: f64 = (p.p1887 * assign13920_e19955);
        let assign13920_e19957: f64 = (locals.var_mfq1nom_i + assign13920_e19956);
        (assign13920_e19957,)
    } else {
        (locals.var_nc1,)
    }
};
        locals.var_nc1 = assign13920_e19959;
        locals.var_nc1_rv = 0.0;

        let (assign13930_e20024,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13930_e19967: f64 = (p.p43 * 1000000000.0);
        let assign13930_e19970: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e19971: f64 = (assign13930_e19967).powf(assign13930_e19970);
        let assign13930_e19972: f64 = (5.0 * assign13930_e19971);
        let assign13930_e19973: f64 = (1.0 + assign13930_e19972);
        let assign13930_e19975: f64 = (assign13930_e19973).powf(0.5);
        let assign13930_e19976: f64 = (locals.var_nc2l0 / assign13930_e19975);
        let assign13930_e19978: f64 = assign13930_e19976;
        let assign13930_e19984: f64 = (p.p43 * 1000000000.0);
        let assign13930_e19987: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e19988: f64 = (assign13930_e19984).powf(assign13930_e19987);
        let assign13930_e19989: f64 = (5.0 * assign13930_e19988);
        let assign13930_e19990: f64 = (1.0 + assign13930_e19989);
        let assign13930_e19992: f64 = (assign13930_e19990).powf(0.5);
        let assign13930_e19993: f64 = (locals.var_nc2l0 / assign13930_e19992);
        let assign13930_e19995: f64 = assign13930_e19993;
        let assign13930_e20001: f64 = (p.p43 * 1000000000.0);
        let assign13930_e20004: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e20005: f64 = (assign13930_e20001).powf(assign13930_e20004);
        let assign13930_e20006: f64 = (5.0 * assign13930_e20005);
        let assign13930_e20007: f64 = (1.0 + assign13930_e20006);
        let assign13930_e20009: f64 = (assign13930_e20007).powf(0.5);
        let assign13930_e20010: f64 = (locals.var_nc2l0 / assign13930_e20009);
        let assign13930_e20012: f64 = assign13930_e20010;
        let assign13930_e20013: f64 = (assign13930_e19995 * assign13930_e20012);
        let assign13930_e20016: f64 = (0.25 * 0.1);
        let assign13930_e20018: f64 = (assign13930_e20016 * 0.1);
        let assign13930_e20019: f64 = (assign13930_e20013 + assign13930_e20018);
        let assign13930_e20020: f64 = (assign13930_e20019).sqrt();
        let assign13930_e20021: f64 = (assign13930_e19978 + assign13930_e20020);
        let assign13930_e20022: f64 = (0.5 * assign13930_e20021);
        (assign13930_e20022,)
    } else {
        (locals.var_nc2l,)
    }
};
        locals.var_nc2l = assign13930_e20024;
        locals.var_nc2l_rv = 0.0;

        let (assign13940_e20089,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13940_e20032: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20035: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20036: f64 = (assign13940_e20032).powf(assign13940_e20035);
        let assign13940_e20037: f64 = (5.0 * assign13940_e20036);
        let assign13940_e20038: f64 = (1.0 + assign13940_e20037);
        let assign13940_e20040: f64 = (assign13940_e20038).powf(0.5);
        let assign13940_e20041: f64 = (locals.var_nc2l0 / assign13940_e20040);
        let assign13940_e20043: f64 = assign13940_e20041;
        let assign13940_e20049: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20052: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20053: f64 = (assign13940_e20049).powf(assign13940_e20052);
        let assign13940_e20054: f64 = (5.0 * assign13940_e20053);
        let assign13940_e20055: f64 = (1.0 + assign13940_e20054);
        let assign13940_e20057: f64 = (assign13940_e20055).powf(0.5);
        let assign13940_e20058: f64 = (locals.var_nc2l0 / assign13940_e20057);
        let assign13940_e20060: f64 = assign13940_e20058;
        let assign13940_e20066: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20069: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20070: f64 = (assign13940_e20066).powf(assign13940_e20069);
        let assign13940_e20071: f64 = (5.0 * assign13940_e20070);
        let assign13940_e20072: f64 = (1.0 + assign13940_e20071);
        let assign13940_e20074: f64 = (assign13940_e20072).powf(0.5);
        let assign13940_e20075: f64 = (locals.var_nc2l0 / assign13940_e20074);
        let assign13940_e20077: f64 = assign13940_e20075;
        let assign13940_e20078: f64 = (assign13940_e20060 * assign13940_e20077);
        let assign13940_e20081: f64 = (0.25 * 0.1);
        let assign13940_e20083: f64 = (assign13940_e20081 * 0.1);
        let assign13940_e20084: f64 = (assign13940_e20078 + assign13940_e20083);
        let assign13940_e20085: f64 = (assign13940_e20084).sqrt();
        let assign13940_e20086: f64 = (assign13940_e20043 + assign13940_e20085);
        let assign13940_e20087: f64 = (0.5 * assign13940_e20086);
        (assign13940_e20087,)
    } else {
        (locals.var_nc2ln,)
    }
};
        locals.var_nc2ln = assign13940_e20089;
        locals.var_nc2ln_rv = 0.0;

        let (assign13950_e20099,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13950_e20095: f64 = (locals.var_nc2l - locals.var_nc2ln);
        let assign13950_e20096: f64 = (p.p1888 * assign13950_e20095);
        let assign13950_e20097: f64 = (locals.var_mfq2nom_i + assign13950_e20096);
        (assign13950_e20097,)
    } else {
        (locals.var_nc2,)
    }
};
        locals.var_nc2 = assign13950_e20099;
        locals.var_nc2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13960_e20164,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13960_e20107: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20110: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20111: f64 = (assign13960_e20107).powf(assign13960_e20110);
        let assign13960_e20112: f64 = (5.0 * assign13960_e20111);
        let assign13960_e20113: f64 = (1.0 + assign13960_e20112);
        let assign13960_e20115: f64 = (assign13960_e20113).powf(0.5);
        let assign13960_e20116: f64 = (locals.var_nc3l0 / assign13960_e20115);
        let assign13960_e20118: f64 = assign13960_e20116;
        let assign13960_e20124: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20127: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20128: f64 = (assign13960_e20124).powf(assign13960_e20127);
        let assign13960_e20129: f64 = (5.0 * assign13960_e20128);
        let assign13960_e20130: f64 = (1.0 + assign13960_e20129);
        let assign13960_e20132: f64 = (assign13960_e20130).powf(0.5);
        let assign13960_e20133: f64 = (locals.var_nc3l0 / assign13960_e20132);
        let assign13960_e20135: f64 = assign13960_e20133;
        let assign13960_e20141: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20144: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20145: f64 = (assign13960_e20141).powf(assign13960_e20144);
        let assign13960_e20146: f64 = (5.0 * assign13960_e20145);
        let assign13960_e20147: f64 = (1.0 + assign13960_e20146);
        let assign13960_e20149: f64 = (assign13960_e20147).powf(0.5);
        let assign13960_e20150: f64 = (locals.var_nc3l0 / assign13960_e20149);
        let assign13960_e20152: f64 = assign13960_e20150;
        let assign13960_e20153: f64 = (assign13960_e20135 * assign13960_e20152);
        let assign13960_e20156: f64 = (0.25 * 0.1);
        let assign13960_e20158: f64 = (assign13960_e20156 * 0.1);
        let assign13960_e20159: f64 = (assign13960_e20153 + assign13960_e20158);
        let assign13960_e20160: f64 = (assign13960_e20159).sqrt();
        let assign13960_e20161: f64 = (assign13960_e20118 + assign13960_e20160);
        let assign13960_e20162: f64 = (0.5 * assign13960_e20161);
        (assign13960_e20162,)
    } else {
        (locals.var_nc3l,)
    }
};
        locals.var_nc3l = assign13960_e20164;
        locals.var_nc3l_rv = 0.0;

        let (assign13970_e20229,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13970_e20172: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20175: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20176: f64 = (assign13970_e20172).powf(assign13970_e20175);
        let assign13970_e20177: f64 = (5.0 * assign13970_e20176);
        let assign13970_e20178: f64 = (1.0 + assign13970_e20177);
        let assign13970_e20180: f64 = (assign13970_e20178).powf(0.5);
        let assign13970_e20181: f64 = (locals.var_nc3l0 / assign13970_e20180);
        let assign13970_e20183: f64 = assign13970_e20181;
        let assign13970_e20189: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20192: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20193: f64 = (assign13970_e20189).powf(assign13970_e20192);
        let assign13970_e20194: f64 = (5.0 * assign13970_e20193);
        let assign13970_e20195: f64 = (1.0 + assign13970_e20194);
        let assign13970_e20197: f64 = (assign13970_e20195).powf(0.5);
        let assign13970_e20198: f64 = (locals.var_nc3l0 / assign13970_e20197);
        let assign13970_e20200: f64 = assign13970_e20198;
        let assign13970_e20206: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20209: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20210: f64 = (assign13970_e20206).powf(assign13970_e20209);
        let assign13970_e20211: f64 = (5.0 * assign13970_e20210);
        let assign13970_e20212: f64 = (1.0 + assign13970_e20211);
        let assign13970_e20214: f64 = (assign13970_e20212).powf(0.5);
        let assign13970_e20215: f64 = (locals.var_nc3l0 / assign13970_e20214);
        let assign13970_e20217: f64 = assign13970_e20215;
        let assign13970_e20218: f64 = (assign13970_e20200 * assign13970_e20217);
        let assign13970_e20221: f64 = (0.25 * 0.1);
        let assign13970_e20223: f64 = (assign13970_e20221 * 0.1);
        let assign13970_e20224: f64 = (assign13970_e20218 + assign13970_e20223);
        let assign13970_e20225: f64 = (assign13970_e20224).sqrt();
        let assign13970_e20226: f64 = (assign13970_e20183 + assign13970_e20225);
        let assign13970_e20227: f64 = (0.5 * assign13970_e20226);
        (assign13970_e20227,)
    } else {
        (locals.var_nc3ln,)
    }
};
        locals.var_nc3ln = assign13970_e20229;
        locals.var_nc3ln_rv = 0.0;

        let (assign13980_e20239,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13980_e20235: f64 = (locals.var_nc3l - locals.var_nc3ln);
        let assign13980_e20236: f64 = (p.p1889 * assign13980_e20235);
        let assign13980_e20237: f64 = (locals.var_mfq3nom_i + assign13980_e20236);
        (assign13980_e20237,)
    } else {
        (locals.var_nc3,)
    }
};
        locals.var_nc3 = assign13980_e20239;
        locals.var_nc3_rv = 0.0;

        let (assign13990_e20373,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13990_e20244: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20245: f64 = (1.60219e-19 * assign13990_e20244);
        let assign13990_e20249: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20250: f64 = (3.14_f64).powf(assign13990_e20249);
        let assign13990_e20253: f64 = (-4.6);
        let assign13990_e20257: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20258: f64 = (1.0 + assign13990_e20257);
        let assign13990_e20260: f64 = (assign13990_e20258 - 1.0);
        let assign13990_e20261: f64 = (assign13990_e20253 * assign13990_e20260);
        let assign13990_e20262: f64 = (assign13990_e20261).exp();
        let assign13990_e20263: f64 = (0.0385 * assign13990_e20262);
        let assign13990_e20269: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20270: f64 = (1.0 + assign13990_e20269);
        let assign13990_e20271: f64 = (2.0 * assign13990_e20270);
        let assign13990_e20273: f64 = (assign13990_e20271 - 3.0);
        let assign13990_e20275: f64 = (assign13990_e20273).powf(8.0);
        let assign13990_e20276: f64 = (7.5893e-7 * assign13990_e20275);
        let assign13990_e20277: f64 = (assign13990_e20263 + assign13990_e20276);
        let assign13990_e20283: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20284: f64 = (1.0 + assign13990_e20283);
        let assign13990_e20286: f64 = (assign13990_e20284 - 1.0);
        let assign13990_e20287: f64 = (2.0 * assign13990_e20286);
        let assign13990_e20289: f64 = (assign13990_e20287).powf(6.0);
        let assign13990_e20290: f64 = (6.9583e-5 * assign13990_e20289);
        let assign13990_e20291: f64 = (assign13990_e20277 + assign13990_e20290);
        let assign13990_e20297: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20298: f64 = (1.0 + assign13990_e20297);
        let assign13990_e20300: f64 = (assign13990_e20298 - 1.0);
        let assign13990_e20301: f64 = (2.0 * assign13990_e20300);
        let assign13990_e20303: f64 = (assign13990_e20301).powf(5.0);
        let assign13990_e20304: f64 = (0.0006583 * assign13990_e20303);
        let assign13990_e20305: f64 = (assign13990_e20291 - assign13990_e20304);
        let assign13990_e20311: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20312: f64 = (1.0 + assign13990_e20311);
        let assign13990_e20314: f64 = (assign13990_e20312 - 1.0);
        let assign13990_e20315: f64 = (2.0 * assign13990_e20314);
        let assign13990_e20317: f64 = (assign13990_e20315).powf(4.0);
        let assign13990_e20318: f64 = (0.0065 * assign13990_e20317);
        let assign13990_e20319: f64 = (assign13990_e20305 + assign13990_e20318);
        let assign13990_e20325: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20326: f64 = (1.0 + assign13990_e20325);
        let assign13990_e20328: f64 = (assign13990_e20326 - 1.0);
        let assign13990_e20329: f64 = (2.0 * assign13990_e20328);
        let assign13990_e20331: f64 = (assign13990_e20329).powf(3.0);
        let assign13990_e20332: f64 = (0.026 * assign13990_e20331);
        let assign13990_e20333: f64 = (assign13990_e20319 - assign13990_e20332);
        let assign13990_e20339: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20340: f64 = (1.0 + assign13990_e20339);
        let assign13990_e20342: f64 = (assign13990_e20340 - 1.0);
        let assign13990_e20343: f64 = (2.0 * assign13990_e20342);
        let assign13990_e20345: f64 = (assign13990_e20343).powf(2.0);
        let assign13990_e20346: f64 = (0.1371 * assign13990_e20345);
        let assign13990_e20347: f64 = (assign13990_e20333 + assign13990_e20346);
        let assign13990_e20350: f64 = (0.194 * 2.0);
        let assign13990_e20354: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20355: f64 = (1.0 + assign13990_e20354);
        let assign13990_e20357: f64 = (assign13990_e20355 - 1.0);
        let assign13990_e20358: f64 = (assign13990_e20350 * assign13990_e20357);
        let assign13990_e20359: f64 = (assign13990_e20347 - assign13990_e20358);
        let assign13990_e20361: f64 = (assign13990_e20359 + 0.959);
        let assign13990_e20362: f64 = (assign13990_e20250 / assign13990_e20361);
        let assign13990_e20363: f64 = (assign13990_e20245 * assign13990_e20362);
        let assign13990_e20366: f64 = (locals.var_nc1 * 1000000.0);
        let assign13990_e20368: f64 = (assign13990_e20366).powf(locals.var_d1);
        let assign13990_e20369: f64 = (assign13990_e20363 * assign13990_e20368);
        let assign13990_e20371: f64 = (assign13990_e20369 * locals.var_qndnf1);
        (assign13990_e20371,)
    } else {
        (locals.var_qnd10,)
    }
};
        locals.var_qnd10 = assign13990_e20373;
        locals.var_qnd10_rv = 0.0;

        let (assign14000_e20507,) = {
    if (locals.var_guard234 != 0.0) {
        let assign14000_e20378: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20379: f64 = (1.60219e-19 * assign14000_e20378);
        let assign14000_e20383: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20384: f64 = (3.14_f64).powf(assign14000_e20383);
        let assign14000_e20387: f64 = (-4.6);
        let assign14000_e20391: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20392: f64 = (1.0 + assign14000_e20391);
        let assign14000_e20394: f64 = (assign14000_e20392 - 1.0);
        let assign14000_e20395: f64 = (assign14000_e20387 * assign14000_e20394);
        let assign14000_e20396: f64 = (assign14000_e20395).exp();
        let assign14000_e20397: f64 = (0.0385 * assign14000_e20396);
        let assign14000_e20403: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20404: f64 = (1.0 + assign14000_e20403);
        let assign14000_e20405: f64 = (2.0 * assign14000_e20404);
        let assign14000_e20407: f64 = (assign14000_e20405 - 3.0);
        let assign14000_e20409: f64 = (assign14000_e20407).powf(8.0);
        let assign14000_e20410: f64 = (7.5893e-7 * assign14000_e20409);
        let assign14000_e20411: f64 = (assign14000_e20397 + assign14000_e20410);
        let assign14000_e20417: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20418: f64 = (1.0 + assign14000_e20417);
        let assign14000_e20420: f64 = (assign14000_e20418 - 1.0);
        let assign14000_e20421: f64 = (2.0 * assign14000_e20420);
        let assign14000_e20423: f64 = (assign14000_e20421).powf(6.0);
        let assign14000_e20424: f64 = (6.9583e-5 * assign14000_e20423);
        let assign14000_e20425: f64 = (assign14000_e20411 + assign14000_e20424);
        let assign14000_e20431: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20432: f64 = (1.0 + assign14000_e20431);
        let assign14000_e20434: f64 = (assign14000_e20432 - 1.0);
        let assign14000_e20435: f64 = (2.0 * assign14000_e20434);
        let assign14000_e20437: f64 = (assign14000_e20435).powf(5.0);
        let assign14000_e20438: f64 = (0.0006583 * assign14000_e20437);
        let assign14000_e20439: f64 = (assign14000_e20425 - assign14000_e20438);
        let assign14000_e20445: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20446: f64 = (1.0 + assign14000_e20445);
        let assign14000_e20448: f64 = (assign14000_e20446 - 1.0);
        let assign14000_e20449: f64 = (2.0 * assign14000_e20448);
        let assign14000_e20451: f64 = (assign14000_e20449).powf(4.0);
        let assign14000_e20452: f64 = (0.0065 * assign14000_e20451);
        let assign14000_e20453: f64 = (assign14000_e20439 + assign14000_e20452);
        let assign14000_e20459: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20460: f64 = (1.0 + assign14000_e20459);
        let assign14000_e20462: f64 = (assign14000_e20460 - 1.0);
        let assign14000_e20463: f64 = (2.0 * assign14000_e20462);
        let assign14000_e20465: f64 = (assign14000_e20463).powf(3.0);
        let assign14000_e20466: f64 = (0.026 * assign14000_e20465);
        let assign14000_e20467: f64 = (assign14000_e20453 - assign14000_e20466);
        let assign14000_e20473: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20474: f64 = (1.0 + assign14000_e20473);
        let assign14000_e20476: f64 = (assign14000_e20474 - 1.0);
        let assign14000_e20477: f64 = (2.0 * assign14000_e20476);
        let assign14000_e20479: f64 = (assign14000_e20477).powf(2.0);
        let assign14000_e20480: f64 = (0.1371 * assign14000_e20479);
        let assign14000_e20481: f64 = (assign14000_e20467 + assign14000_e20480);
        let assign14000_e20484: f64 = (0.194 * 2.0);
        let assign14000_e20488: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20489: f64 = (1.0 + assign14000_e20488);
        let assign14000_e20491: f64 = (assign14000_e20489 - 1.0);
        let assign14000_e20492: f64 = (assign14000_e20484 * assign14000_e20491);
        let assign14000_e20493: f64 = (assign14000_e20481 - assign14000_e20492);
        let assign14000_e20495: f64 = (assign14000_e20493 + 0.959);
        let assign14000_e20496: f64 = (assign14000_e20384 / assign14000_e20495);
        let assign14000_e20497: f64 = (assign14000_e20379 * assign14000_e20496);
        let assign14000_e20500: f64 = (locals.var_nc2 * 1000000.0);
        let assign14000_e20502: f64 = (assign14000_e20500).powf(locals.var_d2);
        let assign14000_e20503: f64 = (assign14000_e20497 * assign14000_e20502);
        let assign14000_e20505: f64 = (assign14000_e20503 * locals.var_qndnf2);
        (assign14000_e20505,)
    } else {
        (locals.var_qnd20,)
    }
};
        locals.var_qnd20 = assign14000_e20507;
        locals.var_qnd20_rv = 0.0;

        let (assign14010_e20641,) = {
    if (locals.var_guard234 != 0.0) {
        let assign14010_e20512: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20513: f64 = (1.60219e-19 * assign14010_e20512);
        let assign14010_e20517: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20518: f64 = (3.14_f64).powf(assign14010_e20517);
        let assign14010_e20521: f64 = (-4.6);
        let assign14010_e20525: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20526: f64 = (1.0 + assign14010_e20525);
        let assign14010_e20528: f64 = (assign14010_e20526 - 1.0);
        let assign14010_e20529: f64 = (assign14010_e20521 * assign14010_e20528);
        let assign14010_e20530: f64 = (assign14010_e20529).exp();
        let assign14010_e20531: f64 = (0.0385 * assign14010_e20530);
        let assign14010_e20537: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20538: f64 = (1.0 + assign14010_e20537);
        let assign14010_e20539: f64 = (2.0 * assign14010_e20538);
        let assign14010_e20541: f64 = (assign14010_e20539 - 3.0);
        let assign14010_e20543: f64 = (assign14010_e20541).powf(8.0);
        let assign14010_e20544: f64 = (7.5893e-7 * assign14010_e20543);
        let assign14010_e20545: f64 = (assign14010_e20531 + assign14010_e20544);
        let assign14010_e20551: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20552: f64 = (1.0 + assign14010_e20551);
        let assign14010_e20554: f64 = (assign14010_e20552 - 1.0);
        let assign14010_e20555: f64 = (2.0 * assign14010_e20554);
        let assign14010_e20557: f64 = (assign14010_e20555).powf(6.0);
        let assign14010_e20558: f64 = (6.9583e-5 * assign14010_e20557);
        let assign14010_e20559: f64 = (assign14010_e20545 + assign14010_e20558);
        let assign14010_e20565: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20566: f64 = (1.0 + assign14010_e20565);
        let assign14010_e20568: f64 = (assign14010_e20566 - 1.0);
        let assign14010_e20569: f64 = (2.0 * assign14010_e20568);
        let assign14010_e20571: f64 = (assign14010_e20569).powf(5.0);
        let assign14010_e20572: f64 = (0.0006583 * assign14010_e20571);
        let assign14010_e20573: f64 = (assign14010_e20559 - assign14010_e20572);
        let assign14010_e20579: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20580: f64 = (1.0 + assign14010_e20579);
        let assign14010_e20582: f64 = (assign14010_e20580 - 1.0);
        let assign14010_e20583: f64 = (2.0 * assign14010_e20582);
        let assign14010_e20585: f64 = (assign14010_e20583).powf(4.0);
        let assign14010_e20586: f64 = (0.0065 * assign14010_e20585);
        let assign14010_e20587: f64 = (assign14010_e20573 + assign14010_e20586);
        let assign14010_e20593: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20594: f64 = (1.0 + assign14010_e20593);
        let assign14010_e20596: f64 = (assign14010_e20594 - 1.0);
        let assign14010_e20597: f64 = (2.0 * assign14010_e20596);
        let assign14010_e20599: f64 = (assign14010_e20597).powf(3.0);
        let assign14010_e20600: f64 = (0.026 * assign14010_e20599);
        let assign14010_e20601: f64 = (assign14010_e20587 - assign14010_e20600);
        let assign14010_e20607: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20608: f64 = (1.0 + assign14010_e20607);
        let assign14010_e20610: f64 = (assign14010_e20608 - 1.0);
        let assign14010_e20611: f64 = (2.0 * assign14010_e20610);
        let assign14010_e20613: f64 = (assign14010_e20611).powf(2.0);
        let assign14010_e20614: f64 = (0.1371 * assign14010_e20613);
        let assign14010_e20615: f64 = (assign14010_e20601 + assign14010_e20614);
        let assign14010_e20618: f64 = (0.194 * 2.0);
        let assign14010_e20622: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20623: f64 = (1.0 + assign14010_e20622);
        let assign14010_e20625: f64 = (assign14010_e20623 - 1.0);
        let assign14010_e20626: f64 = (assign14010_e20618 * assign14010_e20625);
        let assign14010_e20627: f64 = (assign14010_e20615 - assign14010_e20626);
        let assign14010_e20629: f64 = (assign14010_e20627 + 0.959);
        let assign14010_e20630: f64 = (assign14010_e20518 / assign14010_e20629);
        let assign14010_e20631: f64 = (assign14010_e20513 * assign14010_e20630);
        let assign14010_e20634: f64 = (locals.var_nc3 * 1000000.0);
        let assign14010_e20636: f64 = (assign14010_e20634).powf(locals.var_d3);
        let assign14010_e20637: f64 = (assign14010_e20631 * assign14010_e20636);
        let assign14010_e20639: f64 = (assign14010_e20637 * locals.var_qndnf3);
        (assign14010_e20639,)
    } else {
        (locals.var_qnd30,)
    }
};
        locals.var_qnd30 = assign14010_e20641;
        locals.var_qnd30_rv = 0.0;

        let assign14020_e20644: f64 = if p.p58 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign14020_e20644;
        locals.var_guard235_rv = 0.0;

        let (assign14030_e20665,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14030_e20649: f64 = (locals.var_etamob_i - p.p889);
        let assign14030_e20652: f64 = (p.p890 * 1000000000.0);
        let assign14030_e20655: f64 = (p.p40 * 1000000000.0);
        let assign14030_e20656: f64 = (assign14030_e20652 - assign14030_e20655);
        let assign14030_e20658: f64 = (assign14030_e20656 / p.p891);
        let assign14030_e20659: f64 = { let limited_exp_arg = assign14030_e20658; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14030_e20661: f64 = (assign14030_e20659 + 1.0);
        let assign14030_e20662: f64 = (assign14030_e20649 / assign14030_e20661);
        let assign14030_e20663: f64 = (p.p889 + assign14030_e20662);
        (assign14030_e20663,)
    } else {
        (locals.var_etamob_i,)
    }
};
        locals.var_etamob_i = assign14030_e20665;
        locals.var_etamob_i_rv = 0.0;

        let (assign14040_e20677, assign14040_e20677_d_n0, assign14040_e20677_d_n2, assign14040_e20677_d_n3, assign14040_e20677_d_n4, assign14040_e20677_d_n5, assign14040_e20677_d_n6, assign14040_e20677_d_n7, assign14040_e20677_d_n8, assign14040_e20677_d_n9, assign14040_e20677_d_n10, assign14040_e20677_d_n11, assign14040_e20677_d_n13, assign14040_e20677_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14040_e20669: f64 = (locals.var_ua_i - p.p892);
        let assign14040_e20672: f64 = (p.p893 * 1000000000.0);
        let assign14040_e20674: f64 = (assign14040_e20672 * p.p894);
        let assign14040_e20675: f64 = (assign14040_e20669 - assign14040_e20674);
        (assign14040_e20675, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    } else {
        (locals.var_mut0, locals.var_mut0_dn0, locals.var_mut0_dn2, locals.var_mut0_dn3, locals.var_mut0_dn4, locals.var_mut0_dn5, locals.var_mut0_dn6, locals.var_mut0_dn7, locals.var_mut0_dn8, locals.var_mut0_dn9, locals.var_mut0_dn10, locals.var_mut0_dn11, locals.var_mut0_dn13, locals.var_mut0_dn14,)
    }
};
        locals.var_mut0 = assign14040_e20677;
        locals.var_mut0_dn0 = assign14040_e20677_d_n0;
        locals.var_mut0_dn2 = assign14040_e20677_d_n2;
        locals.var_mut0_dn3 = assign14040_e20677_d_n3;
        locals.var_mut0_dn4 = assign14040_e20677_d_n4;
        locals.var_mut0_dn5 = assign14040_e20677_d_n5;
        locals.var_mut0_dn6 = assign14040_e20677_d_n6;
        locals.var_mut0_dn7 = assign14040_e20677_d_n7;
        locals.var_mut0_dn8 = assign14040_e20677_d_n8;
        locals.var_mut0_dn9 = assign14040_e20677_d_n9;
        locals.var_mut0_dn10 = assign14040_e20677_d_n10;
        locals.var_mut0_dn11 = assign14040_e20677_d_n11;
        locals.var_mut0_dn13 = assign14040_e20677_d_n13;
        locals.var_mut0_dn14 = assign14040_e20677_d_n14;
        locals.var_mut0_rv = 0.0;

        let (assign14050_e20700, assign14050_e20700_d_n0, assign14050_e20700_d_n2, assign14050_e20700_d_n3, assign14050_e20700_d_n4, assign14050_e20700_d_n5, assign14050_e20700_d_n6, assign14050_e20700_d_n7, assign14050_e20700_d_n8, assign14050_e20700_d_n9, assign14050_e20700_d_n10, assign14050_e20700_d_n11, assign14050_e20700_d_n13, assign14050_e20700_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14050_e20682: f64 = (p.p40 * 1000000000.0);
        let assign14050_e20684: f64 = (assign14050_e20682 * p.p894);
        let assign14050_e20685: f64 = (locals.var_mut0 + assign14050_e20684);
        let assign14050_e20689: f64 = (p.p895 * 1000000000.0);
        let assign14050_e20692: f64 = (p.p40 * 1000000000.0);
        let assign14050_e20693: f64 = (assign14050_e20689 - assign14050_e20692);
        let assign14050_e20695: f64 = (assign14050_e20693 / p.p896);
        let assign14050_e20696: f64 = { let limited_exp_arg = assign14050_e20695; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14050_e20697: f64 = (1.0 + assign14050_e20696);
        let assign14050_e20698: f64 = (assign14050_e20685 / assign14050_e20697);
        (assign14050_e20698, (locals.var_mut0_dn0 / assign14050_e20697), (locals.var_mut0_dn2 / assign14050_e20697), (locals.var_mut0_dn3 / assign14050_e20697), (locals.var_mut0_dn4 / assign14050_e20697), (locals.var_mut0_dn5 / assign14050_e20697), (locals.var_mut0_dn6 / assign14050_e20697), (locals.var_mut0_dn7 / assign14050_e20697), (locals.var_mut0_dn8 / assign14050_e20697), (locals.var_mut0_dn9 / assign14050_e20697), (locals.var_mut0_dn10 / assign14050_e20697), (locals.var_mut0_dn11 / assign14050_e20697), (locals.var_mut0_dn13 / assign14050_e20697), (locals.var_mut0_dn14 / assign14050_e20697),)
    } else {
        (locals.var_mut1, locals.var_mut1_dn0, locals.var_mut1_dn2, locals.var_mut1_dn3, locals.var_mut1_dn4, locals.var_mut1_dn5, locals.var_mut1_dn6, locals.var_mut1_dn7, locals.var_mut1_dn8, locals.var_mut1_dn9, locals.var_mut1_dn10, locals.var_mut1_dn11, locals.var_mut1_dn13, locals.var_mut1_dn14,)
    }
};
        locals.var_mut1 = assign14050_e20700;
        locals.var_mut1_dn0 = assign14050_e20700_d_n0;
        locals.var_mut1_dn2 = assign14050_e20700_d_n2;
        locals.var_mut1_dn3 = assign14050_e20700_d_n3;
        locals.var_mut1_dn4 = assign14050_e20700_d_n4;
        locals.var_mut1_dn5 = assign14050_e20700_d_n5;
        locals.var_mut1_dn6 = assign14050_e20700_d_n6;
        locals.var_mut1_dn7 = assign14050_e20700_d_n7;
        locals.var_mut1_dn8 = assign14050_e20700_d_n8;
        locals.var_mut1_dn9 = assign14050_e20700_d_n9;
        locals.var_mut1_dn10 = assign14050_e20700_d_n10;
        locals.var_mut1_dn11 = assign14050_e20700_d_n11;
        locals.var_mut1_dn13 = assign14050_e20700_d_n13;
        locals.var_mut1_dn14 = assign14050_e20700_d_n14;
        locals.var_mut1_rv = 0.0;

        let (assign14060_e20735, assign14060_e20735_d_n0, assign14060_e20735_d_n2, assign14060_e20735_d_n3, assign14060_e20735_d_n4, assign14060_e20735_d_n5, assign14060_e20735_d_n6, assign14060_e20735_d_n7, assign14060_e20735_d_n8, assign14060_e20735_d_n9, assign14060_e20735_d_n10, assign14060_e20735_d_n11, assign14060_e20735_d_n13, assign14060_e20735_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14060_e20705: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20708: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20709: f64 = (assign14060_e20705 + assign14060_e20708);
        let assign14060_e20712: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20715: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20716: f64 = (assign14060_e20712 - assign14060_e20715);
        let assign14060_e20719: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20722: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20723: f64 = (assign14060_e20719 - assign14060_e20722);
        let assign14060_e20724: f64 = (assign14060_e20716 * assign14060_e20723);
        let assign14060_e20727: f64 = (0.25 * 0.6);
        let assign14060_e20729: f64 = (assign14060_e20727 * 0.6);
        let assign14060_e20730: f64 = (assign14060_e20724 + assign14060_e20729);
        let assign14060_e20731: f64 = (assign14060_e20730).sqrt();
        let assign14060_e20732: f64 = (assign14060_e20709 - assign14060_e20731);
        let assign14060_e20733: f64 = (0.5 * assign14060_e20732);
        (assign14060_e20733, (0.5 * ((locals.var_mut1_dn0 + locals.var_ua_i_dn0) - ((((locals.var_mut1_dn0 - locals.var_ua_i_dn0) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn0 - locals.var_ua_i_dn0))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn2 + locals.var_ua_i_dn2) - ((((locals.var_mut1_dn2 - locals.var_ua_i_dn2) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn2 - locals.var_ua_i_dn2))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn3 + locals.var_ua_i_dn3) - ((((locals.var_mut1_dn3 - locals.var_ua_i_dn3) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn3 - locals.var_ua_i_dn3))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn4 + locals.var_ua_i_dn4) - ((((locals.var_mut1_dn4 - locals.var_ua_i_dn4) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn4 - locals.var_ua_i_dn4))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn5 + locals.var_ua_i_dn5) - ((((locals.var_mut1_dn5 - locals.var_ua_i_dn5) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn5 - locals.var_ua_i_dn5))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn6 + locals.var_ua_i_dn6) - ((((locals.var_mut1_dn6 - locals.var_ua_i_dn6) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn6 - locals.var_ua_i_dn6))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn7 + locals.var_ua_i_dn7) - ((((locals.var_mut1_dn7 - locals.var_ua_i_dn7) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn7 - locals.var_ua_i_dn7))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn8 + locals.var_ua_i_dn8) - ((((locals.var_mut1_dn8 - locals.var_ua_i_dn8) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn8 - locals.var_ua_i_dn8))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn9 + locals.var_ua_i_dn9) - ((((locals.var_mut1_dn9 - locals.var_ua_i_dn9) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn9 - locals.var_ua_i_dn9))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn10 + locals.var_ua_i_dn10) - ((((locals.var_mut1_dn10 - locals.var_ua_i_dn10) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn10 - locals.var_ua_i_dn10))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn11 + locals.var_ua_i_dn11) - ((((locals.var_mut1_dn11 - locals.var_ua_i_dn11) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn11 - locals.var_ua_i_dn11))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn13 + locals.var_ua_i_dn13) - ((((locals.var_mut1_dn13 - locals.var_ua_i_dn13) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn13 - locals.var_ua_i_dn13))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn14 + locals.var_ua_i_dn14) - ((((locals.var_mut1_dn14 - locals.var_ua_i_dn14) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn14 - locals.var_ua_i_dn14))) / (2.0 * assign14060_e20731)))),)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    }
};
        locals.var_ua_i = assign14060_e20735;
        locals.var_ua_i_dn0 = assign14060_e20735_d_n0;
        locals.var_ua_i_dn2 = assign14060_e20735_d_n2;
        locals.var_ua_i_dn3 = assign14060_e20735_d_n3;
        locals.var_ua_i_dn4 = assign14060_e20735_d_n4;
        locals.var_ua_i_dn5 = assign14060_e20735_d_n5;
        locals.var_ua_i_dn6 = assign14060_e20735_d_n6;
        locals.var_ua_i_dn7 = assign14060_e20735_d_n7;
        locals.var_ua_i_dn8 = assign14060_e20735_d_n8;
        locals.var_ua_i_dn9 = assign14060_e20735_d_n9;
        locals.var_ua_i_dn10 = assign14060_e20735_d_n10;
        locals.var_ua_i_dn11 = assign14060_e20735_d_n11;
        locals.var_ua_i_dn13 = assign14060_e20735_d_n13;
        locals.var_ua_i_dn14 = assign14060_e20735_d_n14;
        locals.var_ua_i_rv = 0.0;

        let (assign14070_e20768, assign14070_e20768_d_n0, assign14070_e20768_d_n2, assign14070_e20768_d_n3, assign14070_e20768_d_n4, assign14070_e20768_d_n5, assign14070_e20768_d_n6, assign14070_e20768_d_n7, assign14070_e20768_d_n8, assign14070_e20768_d_n9, assign14070_e20768_d_n10, assign14070_e20768_d_n11, assign14070_e20768_d_n13, assign14070_e20768_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14070_e20739: f64 = (p.p897 - locals.var_eu_i);
        let assign14070_e20741: f64 = (assign14070_e20739 * 370.0);
        let assign14070_e20744: f64 = (p.p40 * 1000000000.0);
        let assign14070_e20746: f64 = (assign14070_e20744).powf(p.p898);
        let assign14070_e20747: f64 = (assign14070_e20741 / assign14070_e20746);
        let assign14070_e20750: f64 = (p.p897 - locals.var_eu_i);
        let assign14070_e20754: f64 = (p.p40 * 1000000000.0);
        let assign14070_e20757: f64 = (p.p899 * 1000000000.0);
        let assign14070_e20758: f64 = (assign14070_e20754 - assign14070_e20757);
        let assign14070_e20760: f64 = (assign14070_e20758 / p.p900);
        let assign14070_e20761: f64 = { let limited_exp_arg = assign14070_e20760; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14070_e20762: f64 = (1.0 + assign14070_e20761);
        let assign14070_e20763: f64 = (assign14070_e20750 / assign14070_e20762);
        let assign14070_e20764: f64 = (assign14070_e20747 + assign14070_e20763);
        let assign14070_e20766: f64 = (assign14070_e20764 + locals.var_eu_i);
        (assign14070_e20766, (((((-locals.var_eu_i_dn0) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn0) / assign14070_e20762)) + locals.var_eu_i_dn0), (((((-locals.var_eu_i_dn2) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn2) / assign14070_e20762)) + locals.var_eu_i_dn2), (((((-locals.var_eu_i_dn3) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn3) / assign14070_e20762)) + locals.var_eu_i_dn3), (((((-locals.var_eu_i_dn4) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn4) / assign14070_e20762)) + locals.var_eu_i_dn4), (((((-locals.var_eu_i_dn5) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn5) / assign14070_e20762)) + locals.var_eu_i_dn5), (((((-locals.var_eu_i_dn6) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn6) / assign14070_e20762)) + locals.var_eu_i_dn6), (((((-locals.var_eu_i_dn7) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn7) / assign14070_e20762)) + locals.var_eu_i_dn7), (((((-locals.var_eu_i_dn8) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn8) / assign14070_e20762)) + locals.var_eu_i_dn8), (((((-locals.var_eu_i_dn9) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn9) / assign14070_e20762)) + locals.var_eu_i_dn9), (((((-locals.var_eu_i_dn10) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn10) / assign14070_e20762)) + locals.var_eu_i_dn10), (((((-locals.var_eu_i_dn11) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn11) / assign14070_e20762)) + locals.var_eu_i_dn11), (((((-locals.var_eu_i_dn13) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn13) / assign14070_e20762)) + locals.var_eu_i_dn13), (((((-locals.var_eu_i_dn14) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn14) / assign14070_e20762)) + locals.var_eu_i_dn14),)
    } else {
        (locals.var_mut2, locals.var_mut2_dn0, locals.var_mut2_dn2, locals.var_mut2_dn3, locals.var_mut2_dn4, locals.var_mut2_dn5, locals.var_mut2_dn6, locals.var_mut2_dn7, locals.var_mut2_dn8, locals.var_mut2_dn9, locals.var_mut2_dn10, locals.var_mut2_dn11, locals.var_mut2_dn13, locals.var_mut2_dn14,)
    }
};
        locals.var_mut2 = assign14070_e20768;
        locals.var_mut2_dn0 = assign14070_e20768_d_n0;
        locals.var_mut2_dn2 = assign14070_e20768_d_n2;
        locals.var_mut2_dn3 = assign14070_e20768_d_n3;
        locals.var_mut2_dn4 = assign14070_e20768_d_n4;
        locals.var_mut2_dn5 = assign14070_e20768_d_n5;
        locals.var_mut2_dn6 = assign14070_e20768_d_n6;
        locals.var_mut2_dn7 = assign14070_e20768_d_n7;
        locals.var_mut2_dn8 = assign14070_e20768_d_n8;
        locals.var_mut2_dn9 = assign14070_e20768_d_n9;
        locals.var_mut2_dn10 = assign14070_e20768_d_n10;
        locals.var_mut2_dn11 = assign14070_e20768_d_n11;
        locals.var_mut2_dn13 = assign14070_e20768_d_n13;
        locals.var_mut2_dn14 = assign14070_e20768_d_n14;
        locals.var_mut2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign14080_e20791, assign14080_e20791_d_n0, assign14080_e20791_d_n2, assign14080_e20791_d_n3, assign14080_e20791_d_n4, assign14080_e20791_d_n5, assign14080_e20791_d_n6, assign14080_e20791_d_n7, assign14080_e20791_d_n8, assign14080_e20791_d_n9, assign14080_e20791_d_n10, assign14080_e20791_d_n11, assign14080_e20791_d_n13, assign14080_e20791_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14080_e20773: f64 = (locals.var_mut2 + p.p897);
        let assign14080_e20776: f64 = (locals.var_mut2 - p.p897);
        let assign14080_e20779: f64 = (locals.var_mut2 - p.p897);
        let assign14080_e20780: f64 = (assign14080_e20776 * assign14080_e20779);
        let assign14080_e20783: f64 = (0.25 * 0.2);
        let assign14080_e20785: f64 = (assign14080_e20783 * 0.2);
        let assign14080_e20786: f64 = (assign14080_e20780 + assign14080_e20785);
        let assign14080_e20787: f64 = (assign14080_e20786).sqrt();
        let assign14080_e20788: f64 = (assign14080_e20773 - assign14080_e20787);
        let assign14080_e20789: f64 = (0.5 * assign14080_e20788);
        (assign14080_e20789, (0.5 * (locals.var_mut2_dn0 - (((locals.var_mut2_dn0 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn0)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn2 - (((locals.var_mut2_dn2 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn2)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn3 - (((locals.var_mut2_dn3 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn3)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn4 - (((locals.var_mut2_dn4 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn4)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn5 - (((locals.var_mut2_dn5 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn5)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn6 - (((locals.var_mut2_dn6 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn6)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn7 - (((locals.var_mut2_dn7 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn7)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn8 - (((locals.var_mut2_dn8 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn8)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn9 - (((locals.var_mut2_dn9 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn9)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn10 - (((locals.var_mut2_dn10 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn10)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn11 - (((locals.var_mut2_dn11 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn11)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn13 - (((locals.var_mut2_dn13 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn13)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn14 - (((locals.var_mut2_dn14 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn14)) / (2.0 * assign14080_e20787)))),)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    }
};
        locals.var_eu_i = assign14080_e20791;
        locals.var_eu_i_dn0 = assign14080_e20791_d_n0;
        locals.var_eu_i_dn2 = assign14080_e20791_d_n2;
        locals.var_eu_i_dn3 = assign14080_e20791_d_n3;
        locals.var_eu_i_dn4 = assign14080_e20791_d_n4;
        locals.var_eu_i_dn5 = assign14080_e20791_d_n5;
        locals.var_eu_i_dn6 = assign14080_e20791_d_n6;
        locals.var_eu_i_dn7 = assign14080_e20791_d_n7;
        locals.var_eu_i_dn8 = assign14080_e20791_d_n8;
        locals.var_eu_i_dn9 = assign14080_e20791_d_n9;
        locals.var_eu_i_dn10 = assign14080_e20791_d_n10;
        locals.var_eu_i_dn11 = assign14080_e20791_d_n11;
        locals.var_eu_i_dn13 = assign14080_e20791_d_n13;
        locals.var_eu_i_dn14 = assign14080_e20791_d_n14;
        locals.var_eu_i_rv = 0.0;

        let (assign14090_e20799,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14090_e20796: f64 = (p.p43 + p.p40);
        let assign14090_e20797: f64 = (p.p43 / assign14090_e20796);
        (assign14090_e20797,)
    } else {
        (locals.var_mut3,)
    }
};
        locals.var_mut3 = assign14090_e20799;
        locals.var_mut3_rv = 0.0;

        let (assign14100_e20813,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14100_e20803: f64 = (p.p905 * p.p40);
        let assign14100_e20805: f64 = (assign14100_e20803 * p.p40);
        let assign14100_e20807: f64 = (assign14100_e20805 * 1e18);
        let assign14100_e20810: f64 = (p.p906 * 0.001);
        let assign14100_e20811: f64 = (assign14100_e20807 - assign14100_e20810);
        (assign14100_e20811,)
    } else {
        (locals.var_mut4,)
    }
};
        locals.var_mut4 = assign14100_e20813;
        locals.var_mut4_rv = 0.0;

        let (assign14110_e20851,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14110_e20818: f64 = (locals.var_mut4 * locals.var_mut4);
        let assign14110_e20821: f64 = (4.0 * p.p906);
        let assign14110_e20823: f64 = (assign14110_e20821 * 0.001);
        let assign14110_e20826: f64 = (p.p905 + 0.24);
        let assign14110_e20827: f64 = (assign14110_e20823 * assign14110_e20826);
        let assign14110_e20829: f64 = (assign14110_e20827 * p.p40);
        let assign14110_e20831: f64 = (assign14110_e20829 * p.p40);
        let assign14110_e20833: f64 = (assign14110_e20831 * 1e18);
        let assign14110_e20834: f64 = (assign14110_e20818 + assign14110_e20833);
        let assign14110_e20836: f64 = (assign14110_e20834).powf(0.5);
        let assign14110_e20837: f64 = (locals.var_mut4 + assign14110_e20836);
        let assign14110_e20841: f64 = (p.p905 + 0.24);
        let assign14110_e20842: f64 = (2.0 * assign14110_e20841);
        let assign14110_e20844: f64 = (assign14110_e20842 * p.p40);
        let assign14110_e20846: f64 = (assign14110_e20844 * p.p40);
        let assign14110_e20848: f64 = (assign14110_e20846 * 1e18);
        let assign14110_e20849: f64 = (assign14110_e20837 / assign14110_e20848);
        (assign14110_e20849,)
    } else {
        (locals.var_mut5,)
    }
};
        locals.var_mut5 = assign14110_e20851;
        locals.var_mut5_rv = 0.0;

        let (assign14120_e20898,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14120_e20857: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20860: f64 = (p.p907 * 1e-5);
        let assign14120_e20861: f64 = (assign14120_e20857 - assign14120_e20860);
        let assign14120_e20862: f64 = (0.0001 / assign14120_e20861);
        let assign14120_e20864: f64 = (assign14120_e20862 + 1.0);
        let assign14120_e20868: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20871: f64 = (p.p907 * 1e-5);
        let assign14120_e20872: f64 = (assign14120_e20868 - assign14120_e20871);
        let assign14120_e20873: f64 = (0.0001 / assign14120_e20872);
        let assign14120_e20875: f64 = (assign14120_e20873 - 1.0);
        let assign14120_e20879: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20882: f64 = (p.p907 * 1e-5);
        let assign14120_e20883: f64 = (assign14120_e20879 - assign14120_e20882);
        let assign14120_e20884: f64 = (0.0001 / assign14120_e20883);
        let assign14120_e20886: f64 = (assign14120_e20884 - 1.0);
        let assign14120_e20887: f64 = (assign14120_e20875 * assign14120_e20886);
        let assign14120_e20890: f64 = (0.25 * 0.06);
        let assign14120_e20892: f64 = (assign14120_e20890 * 0.06);
        let assign14120_e20893: f64 = (assign14120_e20887 + assign14120_e20892);
        let assign14120_e20894: f64 = (assign14120_e20893).sqrt();
        let assign14120_e20895: f64 = (assign14120_e20864 - assign14120_e20894);
        let assign14120_e20896: f64 = (0.5 * assign14120_e20895);
        (assign14120_e20896,)
    } else {
        (locals.var_mut6,)
    }
};
        locals.var_mut6 = assign14120_e20898;
        locals.var_mut6_rv = 0.0;

        let (assign14130_e20912, assign14130_e20912_d_n0, assign14130_e20912_d_n2, assign14130_e20912_d_n3, assign14130_e20912_d_n4, assign14130_e20912_d_n5, assign14130_e20912_d_n6, assign14130_e20912_d_n7, assign14130_e20912_d_n8, assign14130_e20912_d_n9, assign14130_e20912_d_n10, assign14130_e20912_d_n11, assign14130_e20912_d_n13, assign14130_e20912_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14130_e20905: f64 = (1.0 - locals.var_mut3);
        let assign14130_e20906: f64 = (p.p904 * assign14130_e20905);
        let assign14130_e20907: f64 = (locals.var_mut3 + assign14130_e20906);
        let assign14130_e20908: f64 = (locals.var_u0_i * assign14130_e20907);
        let assign14130_e20910: f64 = (assign14130_e20908 * locals.var_mut6);
        (assign14130_e20910, ((locals.var_u0_i_dn0 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn2 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn3 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn4 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn5 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn6 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn7 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn8 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn9 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn10 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn11 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn13 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn14 * assign14130_e20907) * locals.var_mut6),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign14130_e20912;
        locals.var_u0_i_dn0 = assign14130_e20912_d_n0;
        locals.var_u0_i_dn2 = assign14130_e20912_d_n2;
        locals.var_u0_i_dn3 = assign14130_e20912_d_n3;
        locals.var_u0_i_dn4 = assign14130_e20912_d_n4;
        locals.var_u0_i_dn5 = assign14130_e20912_d_n5;
        locals.var_u0_i_dn6 = assign14130_e20912_d_n6;
        locals.var_u0_i_dn7 = assign14130_e20912_d_n7;
        locals.var_u0_i_dn8 = assign14130_e20912_d_n8;
        locals.var_u0_i_dn9 = assign14130_e20912_d_n9;
        locals.var_u0_i_dn10 = assign14130_e20912_d_n10;
        locals.var_u0_i_dn11 = assign14130_e20912_d_n11;
        locals.var_u0_i_dn13 = assign14130_e20912_d_n13;
        locals.var_u0_i_dn14 = assign14130_e20912_d_n14;
        locals.var_u0_i_rv = 0.0;

        let (assign14140_e20961, assign14140_e20961_d_n0, assign14140_e20961_d_n2, assign14140_e20961_d_n3, assign14140_e20961_d_n4, assign14140_e20961_d_n5, assign14140_e20961_d_n6, assign14140_e20961_d_n7, assign14140_e20961_d_n8, assign14140_e20961_d_n9, assign14140_e20961_d_n10, assign14140_e20961_d_n11, assign14140_e20961_d_n13, assign14140_e20961_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14140_e20916: f64 = (p.p901 - locals.var_ud_i);
        let assign14140_e20920: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20923: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20924: f64 = (assign14140_e20920 - assign14140_e20923);
        let assign14140_e20926: f64 = assign14140_e20924;
        let assign14140_e20929: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20932: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20933: f64 = (assign14140_e20929 - assign14140_e20932);
        let assign14140_e20935: f64 = assign14140_e20933;
        let assign14140_e20938: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20941: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20942: f64 = (assign14140_e20938 - assign14140_e20941);
        let assign14140_e20944: f64 = assign14140_e20942;
        let assign14140_e20945: f64 = (assign14140_e20935 * assign14140_e20944);
        let assign14140_e20948: f64 = 0.25;
        let assign14140_e20950: f64 = assign14140_e20948;
        let assign14140_e20951: f64 = (assign14140_e20945 + assign14140_e20950);
        let assign14140_e20952: f64 = (assign14140_e20951).sqrt();
        let assign14140_e20953: f64 = (assign14140_e20926 + assign14140_e20952);
        let assign14140_e20954: f64 = (0.5 * assign14140_e20953);
        let assign14140_e20956: f64 = (assign14140_e20954).powf(p.p903);
        let assign14140_e20957: f64 = (assign14140_e20916 * assign14140_e20956);
        let assign14140_e20959: f64 = (assign14140_e20957 + locals.var_ud_i);
        (assign14140_e20959, (((-locals.var_ud_i_dn0) * assign14140_e20956) + locals.var_ud_i_dn0), (((-locals.var_ud_i_dn2) * assign14140_e20956) + locals.var_ud_i_dn2), (((-locals.var_ud_i_dn3) * assign14140_e20956) + locals.var_ud_i_dn3), (((-locals.var_ud_i_dn4) * assign14140_e20956) + locals.var_ud_i_dn4), (((-locals.var_ud_i_dn5) * assign14140_e20956) + locals.var_ud_i_dn5), (((-locals.var_ud_i_dn6) * assign14140_e20956) + locals.var_ud_i_dn6), (((-locals.var_ud_i_dn7) * assign14140_e20956) + locals.var_ud_i_dn7), (((-locals.var_ud_i_dn8) * assign14140_e20956) + locals.var_ud_i_dn8), (((-locals.var_ud_i_dn9) * assign14140_e20956) + locals.var_ud_i_dn9), (((-locals.var_ud_i_dn10) * assign14140_e20956) + locals.var_ud_i_dn10), (((-locals.var_ud_i_dn11) * assign14140_e20956) + locals.var_ud_i_dn11), (((-locals.var_ud_i_dn13) * assign14140_e20956) + locals.var_ud_i_dn13), (((-locals.var_ud_i_dn14) * assign14140_e20956) + locals.var_ud_i_dn14),)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn13, locals.var_ud_i_dn14,)
    }
};
        locals.var_ud_i = assign14140_e20961;
        locals.var_ud_i_dn0 = assign14140_e20961_d_n0;
        locals.var_ud_i_dn2 = assign14140_e20961_d_n2;
        locals.var_ud_i_dn3 = assign14140_e20961_d_n3;
        locals.var_ud_i_dn4 = assign14140_e20961_d_n4;
        locals.var_ud_i_dn5 = assign14140_e20961_d_n5;
        locals.var_ud_i_dn6 = assign14140_e20961_d_n6;
        locals.var_ud_i_dn7 = assign14140_e20961_d_n7;
        locals.var_ud_i_dn8 = assign14140_e20961_d_n8;
        locals.var_ud_i_dn9 = assign14140_e20961_d_n9;
        locals.var_ud_i_dn10 = assign14140_e20961_d_n10;
        locals.var_ud_i_dn11 = assign14140_e20961_d_n11;
        locals.var_ud_i_dn13 = assign14140_e20961_d_n13;
        locals.var_ud_i_dn14 = assign14140_e20961_d_n14;
        locals.var_ud_i_rv = 0.0;

        let assign14150_e20968: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard236 = assign14150_e20968;
        locals.var_guard236_rv = 0.0;

        let (assign14160_e20976, assign14160_e20976_d_n4,) = {
    if (locals.var_guard236 != 0.0) {
        let assign14160_e20970: f64 = ctx_temp;
        let assign14160_e20972: f64 = (assign14160_e20970 + (nv4 - 0.0));
        let assign14160_e20974: f64 = (assign14160_e20972 + p.p22);
        (assign14160_e20974, 1.0,)
    } else {
        (locals.var_devtemp, locals.var_devtemp_dn4,)
    }
};
        locals.var_devtemp = assign14160_e20976;
        locals.var_devtemp_dn4 = assign14160_e20976_d_n4;
        locals.var_devtemp_rv = 0.0;

        let (assign14170_e20983, assign14170_e20983_d_n4,) = {
    if (locals.var_guard236 == 0.0) {
        let assign14170_e20979: f64 = ctx_temp;
        let assign14170_e20981: f64 = (assign14170_e20979 + p.p22);
        (assign14170_e20981, 0.0,)
    } else {
        (locals.var_devtemp, locals.var_devtemp_dn4,)
    }
};
        locals.var_devtemp = assign14170_e20983;
        locals.var_devtemp_dn4 = assign14170_e20983_d_n4;
        locals.var_devtemp_rv = 0.0;

        let assign14180_e20986: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign14180_e20986;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_rv = 0.0;

        let assign14190_e20989: f64 = (locals.var_tratio - 1.0);
        locals.var_tratio_m1 = assign14190_e20989;
        locals.var_tratio_m1_dn4 = locals.var_tratio_dn4;
        locals.var_tratio_m1_rv = 0.0;

        let assign14200_e20992: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign14200_e20992;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_rv = 0.0;

        let assign14210_e20995: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vtm = assign14210_e20995;
        locals.var_vtm_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);
        locals.var_vtm_rv = 0.0;

        let assign14220_e20998: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign14220_e20998;
        locals.var_vtm0_rv = 0.0;

        locals.var_tlow = p.p1786;
        locals.var_tlow_rv = 0.0;

        let assign14240_e21002: f64 = if p.p80 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign14240_e21002;
        locals.var_guard237_rv = 0.0;

        let (assign14250_e21025, assign14250_e21025_d_n4,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14250_e21007: f64 = (locals.var_devtemp + locals.var_tlow);
        let assign14250_e21010: f64 = (locals.var_devtemp - locals.var_tlow);
        let assign14250_e21013: f64 = (locals.var_devtemp - locals.var_tlow);
        let assign14250_e21014: f64 = (assign14250_e21010 * assign14250_e21013);
        let assign14250_e21017: f64 = (0.25 * p.p1788);
        let assign14250_e21019: f64 = (assign14250_e21017 * p.p1788);
        let assign14250_e21020: f64 = (assign14250_e21014 + assign14250_e21019);
        let assign14250_e21021: f64 = (assign14250_e21020).sqrt();
        let assign14250_e21022: f64 = (assign14250_e21007 + assign14250_e21021);
        let assign14250_e21023: f64 = (0.5 * assign14250_e21022);
        (assign14250_e21023, (0.5 * (locals.var_devtemp_dn4 + (((locals.var_devtemp_dn4 * assign14250_e21013) + (assign14250_e21010 * locals.var_devtemp_dn4)) / (2.0 * assign14250_e21021)))),)
    } else {
        (locals.var_devtemplow0, locals.var_devtemplow0_dn4,)
    }
};
        locals.var_devtemplow0 = assign14250_e21025;
        locals.var_devtemplow0_dn4 = assign14250_e21025_d_n4;
        locals.var_devtemplow0_rv = 0.0;

        let (assign14260_e21063, assign14260_e21063_d_n4,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14260_e21029: f64 = (-p.p1790);
        let assign14260_e21032: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21033: f64 = (assign14260_e21029 * assign14260_e21032);
        let assign14260_e21035: f64 = assign14260_e21033;
        let assign14260_e21037: f64 = (-p.p1790);
        let assign14260_e21040: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21041: f64 = (assign14260_e21037 * assign14260_e21040);
        let assign14260_e21043: f64 = assign14260_e21041;
        let assign14260_e21045: f64 = (-p.p1790);
        let assign14260_e21048: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21049: f64 = (assign14260_e21045 * assign14260_e21048);
        let assign14260_e21051: f64 = assign14260_e21049;
        let assign14260_e21052: f64 = (assign14260_e21043 * assign14260_e21051);
        let assign14260_e21055: f64 = (0.25 * p.p1789);
        let assign14260_e21057: f64 = (assign14260_e21055 * p.p1789);
        let assign14260_e21058: f64 = (assign14260_e21052 + assign14260_e21057);
        let assign14260_e21059: f64 = (assign14260_e21058).sqrt();
        let assign14260_e21060: f64 = (assign14260_e21035 + assign14260_e21059);
        let assign14260_e21061: f64 = (0.5 * assign14260_e21060);
        (assign14260_e21061, (0.5 * ((assign14260_e21029 * locals.var_devtemp_dn4) + ((((assign14260_e21037 * locals.var_devtemp_dn4) * assign14260_e21051) + (assign14260_e21043 * (assign14260_e21045 * locals.var_devtemp_dn4))) / (2.0 * assign14260_e21059)))),)
    } else {
        (locals.var_devtemplow1, locals.var_devtemplow1_dn4,)
    }
};
        locals.var_devtemplow1 = assign14260_e21063;
        locals.var_devtemplow1_dn4 = assign14260_e21063_d_n4;
        locals.var_devtemplow1_rv = 0.0;

        let assign14270_e21066: f64 = if p.p80 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign14270_e21066;
        locals.var_guard238_rv = 0.0;

        let (assign14280_e21091, assign14280_e21091_d_n0, assign14280_e21091_d_n2, assign14280_e21091_d_n3, assign14280_e21091_d_n4, assign14280_e21091_d_n5, assign14280_e21091_d_n6, assign14280_e21091_d_n7, assign14280_e21091_d_n8, assign14280_e21091_d_n9, assign14280_e21091_d_n10, assign14280_e21091_d_n11, assign14280_e21091_d_n13, assign14280_e21091_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14280_e21073: f64 = (locals.var_tnom + locals.var_tlow);
        let assign14280_e21076: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14280_e21079: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14280_e21080: f64 = (assign14280_e21076 * assign14280_e21079);
        let assign14280_e21083: f64 = (0.25 * p.p1788);
        let assign14280_e21085: f64 = (assign14280_e21083 * p.p1788);
        let assign14280_e21086: f64 = (assign14280_e21080 + assign14280_e21085);
        let assign14280_e21087: f64 = (assign14280_e21086).sqrt();
        let assign14280_e21088: f64 = (assign14280_e21073 + assign14280_e21087);
        let assign14280_e21089: f64 = (0.5 * assign14280_e21088);
        (assign14280_e21089, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14280_e21091;
        locals.var_t1_dn0 = assign14280_e21091_d_n0;
        locals.var_t1_dn2 = assign14280_e21091_d_n2;
        locals.var_t1_dn3 = assign14280_e21091_d_n3;
        locals.var_t1_dn4 = assign14280_e21091_d_n4;
        locals.var_t1_dn5 = assign14280_e21091_d_n5;
        locals.var_t1_dn6 = assign14280_e21091_d_n6;
        locals.var_t1_dn7 = assign14280_e21091_d_n7;
        locals.var_t1_dn8 = assign14280_e21091_d_n8;
        locals.var_t1_dn9 = assign14280_e21091_d_n9;
        locals.var_t1_dn10 = assign14280_e21091_d_n10;
        locals.var_t1_dn11 = assign14280_e21091_d_n11;
        locals.var_t1_dn13 = assign14280_e21091_d_n13;
        locals.var_t1_dn14 = assign14280_e21091_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14290_e21131, assign14290_e21131_d_n0, assign14290_e21131_d_n2, assign14290_e21131_d_n3, assign14290_e21131_d_n4, assign14290_e21131_d_n5, assign14290_e21131_d_n6, assign14290_e21131_d_n7, assign14290_e21131_d_n8, assign14290_e21131_d_n9, assign14290_e21131_d_n10, assign14290_e21131_d_n11, assign14290_e21131_d_n13, assign14290_e21131_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14290_e21097: f64 = (-p.p1790);
        let assign14290_e21100: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21101: f64 = (assign14290_e21097 * assign14290_e21100);
        let assign14290_e21103: f64 = assign14290_e21101;
        let assign14290_e21105: f64 = (-p.p1790);
        let assign14290_e21108: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21109: f64 = (assign14290_e21105 * assign14290_e21108);
        let assign14290_e21111: f64 = assign14290_e21109;
        let assign14290_e21113: f64 = (-p.p1790);
        let assign14290_e21116: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21117: f64 = (assign14290_e21113 * assign14290_e21116);
        let assign14290_e21119: f64 = assign14290_e21117;
        let assign14290_e21120: f64 = (assign14290_e21111 * assign14290_e21119);
        let assign14290_e21123: f64 = (0.25 * p.p1789);
        let assign14290_e21125: f64 = (assign14290_e21123 * p.p1789);
        let assign14290_e21126: f64 = (assign14290_e21120 + assign14290_e21125);
        let assign14290_e21127: f64 = (assign14290_e21126).sqrt();
        let assign14290_e21128: f64 = (assign14290_e21103 + assign14290_e21127);
        let assign14290_e21129: f64 = (0.5 * assign14290_e21128);
        (assign14290_e21129, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14290_e21131;
        locals.var_t2_dn0 = assign14290_e21131_d_n0;
        locals.var_t2_dn2 = assign14290_e21131_d_n2;
        locals.var_t2_dn3 = assign14290_e21131_d_n3;
        locals.var_t2_dn4 = assign14290_e21131_d_n4;
        locals.var_t2_dn5 = assign14290_e21131_d_n5;
        locals.var_t2_dn6 = assign14290_e21131_d_n6;
        locals.var_t2_dn7 = assign14290_e21131_d_n7;
        locals.var_t2_dn8 = assign14290_e21131_d_n8;
        locals.var_t2_dn9 = assign14290_e21131_d_n9;
        locals.var_t2_dn10 = assign14290_e21131_d_n10;
        locals.var_t2_dn11 = assign14290_e21131_d_n11;
        locals.var_t2_dn13 = assign14290_e21131_d_n13;
        locals.var_t2_dn14 = assign14290_e21131_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14300_e21134: f64 = if locals.var_tnom > locals.var_tlow { 1.0 } else { 0.0 };
        locals.var_guard239 = assign14300_e21134;
        locals.var_guard239_rv = 0.0;

        let (assign14310_e21150, assign14310_e21150_d_n0, assign14310_e21150_d_n2, assign14310_e21150_d_n3, assign14310_e21150_d_n4, assign14310_e21150_d_n5, assign14310_e21150_d_n6, assign14310_e21150_d_n7, assign14310_e21150_d_n8, assign14310_e21150_d_n9, assign14310_e21150_d_n10, assign14310_e21150_d_n11, assign14310_e21150_d_n13, assign14310_e21150_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
        let assign14310_e21142: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14310_e21144: f64 = (assign14310_e21142 - locals.var_t1);
        let assign14310_e21146: f64 = (assign14310_e21144 - locals.var_t2);
        let assign14310_e21148: f64 = (assign14310_e21146 + locals.var_tnom);
        (assign14310_e21148, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14310_e21150;
        locals.var_t3_dn0 = assign14310_e21150_d_n0;
        locals.var_t3_dn2 = assign14310_e21150_d_n2;
        locals.var_t3_dn3 = assign14310_e21150_d_n3;
        locals.var_t3_dn4 = assign14310_e21150_d_n4;
        locals.var_t3_dn5 = assign14310_e21150_d_n5;
        locals.var_t3_dn6 = assign14310_e21150_d_n6;
        locals.var_t3_dn7 = assign14310_e21150_d_n7;
        locals.var_t3_dn8 = assign14310_e21150_d_n8;
        locals.var_t3_dn9 = assign14310_e21150_d_n9;
        locals.var_t3_dn10 = assign14310_e21150_d_n10;
        locals.var_t3_dn11 = assign14310_e21150_d_n11;
        locals.var_t3_dn13 = assign14310_e21150_d_n13;
        locals.var_t3_dn14 = assign14310_e21150_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14320_e21167, assign14320_e21167_d_n0, assign14320_e21167_d_n2, assign14320_e21167_d_n3, assign14320_e21167_d_n4, assign14320_e21167_d_n5, assign14320_e21167_d_n6, assign14320_e21167_d_n7, assign14320_e21167_d_n8, assign14320_e21167_d_n9, assign14320_e21167_d_n10, assign14320_e21167_d_n11, assign14320_e21167_d_n13, assign14320_e21167_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
        let assign14320_e21159: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14320_e21161: f64 = (assign14320_e21159 - locals.var_t1);
        let assign14320_e21163: f64 = (assign14320_e21161 - locals.var_t2);
        let assign14320_e21165: f64 = (assign14320_e21163 + locals.var_tlow);
        (assign14320_e21165, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14320_e21167;
        locals.var_t3_dn0 = assign14320_e21167_d_n0;
        locals.var_t3_dn2 = assign14320_e21167_d_n2;
        locals.var_t3_dn3 = assign14320_e21167_d_n3;
        locals.var_t3_dn4 = assign14320_e21167_d_n4;
        locals.var_t3_dn5 = assign14320_e21167_d_n5;
        locals.var_t3_dn6 = assign14320_e21167_d_n6;
        locals.var_t3_dn7 = assign14320_e21167_d_n7;
        locals.var_t3_dn8 = assign14320_e21167_d_n8;
        locals.var_t3_dn9 = assign14320_e21167_d_n9;
        locals.var_t3_dn10 = assign14320_e21167_d_n10;
        locals.var_t3_dn11 = assign14320_e21167_d_n11;
        locals.var_t3_dn13 = assign14320_e21167_d_n13;
        locals.var_t3_dn14 = assign14320_e21167_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14330_e21192, assign14330_e21192_d_n0, assign14330_e21192_d_n2, assign14330_e21192_d_n3, assign14330_e21192_d_n4, assign14330_e21192_d_n5, assign14330_e21192_d_n6, assign14330_e21192_d_n7, assign14330_e21192_d_n8, assign14330_e21192_d_n9, assign14330_e21192_d_n10, assign14330_e21192_d_n11, assign14330_e21192_d_n13, assign14330_e21192_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14330_e21174: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14330_e21177: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14330_e21180: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14330_e21181: f64 = (assign14330_e21177 * assign14330_e21180);
        let assign14330_e21184: f64 = (0.25 * 0.2);
        let assign14330_e21186: f64 = (assign14330_e21184 * 0.2);
        let assign14330_e21187: f64 = (assign14330_e21181 + assign14330_e21186);
        let assign14330_e21188: f64 = (assign14330_e21187).sqrt();
        let assign14330_e21189: f64 = (assign14330_e21174 + assign14330_e21188);
        let assign14330_e21190: f64 = (0.5 * assign14330_e21189);
        (assign14330_e21190, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn0))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn2))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn3))) / (2.0 * assign14330_e21188)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14330_e21180) + (assign14330_e21177 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn5))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn6))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn7))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn8))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn9))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn10))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn11))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn13))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn14))) / (2.0 * assign14330_e21188)))),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14330_e21192;
        locals.var_devtempeff_dn0 = assign14330_e21192_d_n0;
        locals.var_devtempeff_dn2 = assign14330_e21192_d_n2;
        locals.var_devtempeff_dn3 = assign14330_e21192_d_n3;
        locals.var_devtempeff_dn4 = assign14330_e21192_d_n4;
        locals.var_devtempeff_dn5 = assign14330_e21192_d_n5;
        locals.var_devtempeff_dn6 = assign14330_e21192_d_n6;
        locals.var_devtempeff_dn7 = assign14330_e21192_d_n7;
        locals.var_devtempeff_dn8 = assign14330_e21192_d_n8;
        locals.var_devtempeff_dn9 = assign14330_e21192_d_n9;
        locals.var_devtempeff_dn10 = assign14330_e21192_d_n10;
        locals.var_devtempeff_dn11 = assign14330_e21192_d_n11;
        locals.var_devtempeff_dn13 = assign14330_e21192_d_n13;
        locals.var_devtempeff_dn14 = assign14330_e21192_d_n14;
        locals.var_devtempeff_rv = 0.0;

        let assign14340_e21195: f64 = if locals.var_tlow > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign14340_e21195;
        locals.var_guard240_rv = 0.0;

        let (assign14350_e21204,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard240 != 0.0)) {
        (210.0,)
    } else {
        (locals.var_tlow,)
    }
};
        locals.var_tlow = assign14350_e21204;
        locals.var_tlow_rv = 0.0;

        let (assign14360_e21220, assign14360_e21220_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14360_e21214: f64 = (locals.var_devtemp - 210.0);
        let assign14360_e21215: f64 = (0.5 * assign14360_e21214);
        let assign14360_e21216: f64 = (assign14360_e21215).tanh();
        let assign14360_e21217: f64 = (0.5 * assign14360_e21216);
        let assign14360_e21218: f64 = (0.5 + assign14360_e21217);
        (assign14360_e21218, (0.5 * ((0.5 * locals.var_devtemp_dn4) / ((assign14360_e21215).cosh() * (assign14360_e21215).cosh()))),)
    } else {
        (locals.var_wh, locals.var_wh_dn4,)
    }
};
        locals.var_wh = assign14360_e21220;
        locals.var_wh_dn4 = assign14360_e21220_d_n4;
        locals.var_wh_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14370_e21229, assign14370_e21229_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14370_e21227: f64 = (1.0 - locals.var_wh);
        (assign14370_e21227, (-locals.var_wh_dn4),)
    } else {
        (locals.var_wl, locals.var_wl_dn4,)
    }
};
        locals.var_wl = assign14370_e21229;
        locals.var_wl_dn4 = assign14370_e21229_d_n4;
        locals.var_wl_rv = 0.0;

        let assign14380_e21232: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign14380_e21232;
        locals.var_guard241_rv = 0.0;

        let (assign14390_e21260, assign14390_e21260_d_n0, assign14390_e21260_d_n2, assign14390_e21260_d_n3, assign14390_e21260_d_n4, assign14390_e21260_d_n5, assign14390_e21260_d_n6, assign14390_e21260_d_n7, assign14390_e21260_d_n8, assign14390_e21260_d_n9, assign14390_e21260_d_n10, assign14390_e21260_d_n11, assign14390_e21260_d_n13, assign14390_e21260_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14390_e21242: f64 = (210.0 + locals.var_tlow);
        let assign14390_e21245: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21248: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21249: f64 = (assign14390_e21245 * assign14390_e21248);
        let assign14390_e21252: f64 = (0.25 * p.p1788);
        let assign14390_e21254: f64 = (assign14390_e21252 * p.p1788);
        let assign14390_e21255: f64 = (assign14390_e21249 + assign14390_e21254);
        let assign14390_e21256: f64 = (assign14390_e21255).sqrt();
        let assign14390_e21257: f64 = (assign14390_e21242 + assign14390_e21256);
        let assign14390_e21258: f64 = (0.5 * assign14390_e21257);
        (assign14390_e21258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14390_e21260;
        locals.var_t1_dn0 = assign14390_e21260_d_n0;
        locals.var_t1_dn2 = assign14390_e21260_d_n2;
        locals.var_t1_dn3 = assign14390_e21260_d_n3;
        locals.var_t1_dn4 = assign14390_e21260_d_n4;
        locals.var_t1_dn5 = assign14390_e21260_d_n5;
        locals.var_t1_dn6 = assign14390_e21260_d_n6;
        locals.var_t1_dn7 = assign14390_e21260_d_n7;
        locals.var_t1_dn8 = assign14390_e21260_d_n8;
        locals.var_t1_dn9 = assign14390_e21260_d_n9;
        locals.var_t1_dn10 = assign14390_e21260_d_n10;
        locals.var_t1_dn11 = assign14390_e21260_d_n11;
        locals.var_t1_dn13 = assign14390_e21260_d_n13;
        locals.var_t1_dn14 = assign14390_e21260_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14400_e21303, assign14400_e21303_d_n0, assign14400_e21303_d_n2, assign14400_e21303_d_n3, assign14400_e21303_d_n4, assign14400_e21303_d_n5, assign14400_e21303_d_n6, assign14400_e21303_d_n7, assign14400_e21303_d_n8, assign14400_e21303_d_n9, assign14400_e21303_d_n10, assign14400_e21303_d_n11, assign14400_e21303_d_n13, assign14400_e21303_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14400_e21269: f64 = (-p.p1790);
        let assign14400_e21272: f64 = (210.0 - p.p1787);
        let assign14400_e21273: f64 = (assign14400_e21269 * assign14400_e21272);
        let assign14400_e21275: f64 = assign14400_e21273;
        let assign14400_e21277: f64 = (-p.p1790);
        let assign14400_e21280: f64 = (210.0 - p.p1787);
        let assign14400_e21281: f64 = (assign14400_e21277 * assign14400_e21280);
        let assign14400_e21283: f64 = assign14400_e21281;
        let assign14400_e21285: f64 = (-p.p1790);
        let assign14400_e21288: f64 = (210.0 - p.p1787);
        let assign14400_e21289: f64 = (assign14400_e21285 * assign14400_e21288);
        let assign14400_e21291: f64 = assign14400_e21289;
        let assign14400_e21292: f64 = (assign14400_e21283 * assign14400_e21291);
        let assign14400_e21295: f64 = (0.25 * p.p1789);
        let assign14400_e21297: f64 = (assign14400_e21295 * p.p1789);
        let assign14400_e21298: f64 = (assign14400_e21292 + assign14400_e21297);
        let assign14400_e21299: f64 = (assign14400_e21298).sqrt();
        let assign14400_e21300: f64 = (assign14400_e21275 + assign14400_e21299);
        let assign14400_e21301: f64 = (0.5 * assign14400_e21300);
        (assign14400_e21301, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14400_e21303;
        locals.var_t2_dn0 = assign14400_e21303_d_n0;
        locals.var_t2_dn2 = assign14400_e21303_d_n2;
        locals.var_t2_dn3 = assign14400_e21303_d_n3;
        locals.var_t2_dn4 = assign14400_e21303_d_n4;
        locals.var_t2_dn5 = assign14400_e21303_d_n5;
        locals.var_t2_dn6 = assign14400_e21303_d_n6;
        locals.var_t2_dn7 = assign14400_e21303_d_n7;
        locals.var_t2_dn8 = assign14400_e21303_d_n8;
        locals.var_t2_dn9 = assign14400_e21303_d_n9;
        locals.var_t2_dn10 = assign14400_e21303_d_n10;
        locals.var_t2_dn11 = assign14400_e21303_d_n11;
        locals.var_t2_dn13 = assign14400_e21303_d_n13;
        locals.var_t2_dn14 = assign14400_e21303_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign14410_e21320, assign14410_e21320_d_n0, assign14410_e21320_d_n2, assign14410_e21320_d_n3, assign14410_e21320_d_n4, assign14410_e21320_d_n5, assign14410_e21320_d_n6, assign14410_e21320_d_n7, assign14410_e21320_d_n8, assign14410_e21320_d_n9, assign14410_e21320_d_n10, assign14410_e21320_d_n11, assign14410_e21320_d_n13, assign14410_e21320_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14410_e21312: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14410_e21314: f64 = (assign14410_e21312 - locals.var_t1);
        let assign14410_e21316: f64 = (assign14410_e21314 - locals.var_t2);
        let assign14410_e21318: f64 = (assign14410_e21316 + 210.0);
        (assign14410_e21318, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14410_e21320;
        locals.var_t3_dn0 = assign14410_e21320_d_n0;
        locals.var_t3_dn2 = assign14410_e21320_d_n2;
        locals.var_t3_dn3 = assign14410_e21320_d_n3;
        locals.var_t3_dn4 = assign14410_e21320_d_n4;
        locals.var_t3_dn5 = assign14410_e21320_d_n5;
        locals.var_t3_dn6 = assign14410_e21320_d_n6;
        locals.var_t3_dn7 = assign14410_e21320_d_n7;
        locals.var_t3_dn8 = assign14410_e21320_d_n8;
        locals.var_t3_dn9 = assign14410_e21320_d_n9;
        locals.var_t3_dn10 = assign14410_e21320_d_n10;
        locals.var_t3_dn11 = assign14410_e21320_d_n11;
        locals.var_t3_dn13 = assign14410_e21320_d_n13;
        locals.var_t3_dn14 = assign14410_e21320_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14420_e21348, assign14420_e21348_d_n0, assign14420_e21348_d_n2, assign14420_e21348_d_n3, assign14420_e21348_d_n4, assign14420_e21348_d_n5, assign14420_e21348_d_n6, assign14420_e21348_d_n7, assign14420_e21348_d_n8, assign14420_e21348_d_n9, assign14420_e21348_d_n10, assign14420_e21348_d_n11, assign14420_e21348_d_n13, assign14420_e21348_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14420_e21330: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14420_e21333: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21336: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21337: f64 = (assign14420_e21333 * assign14420_e21336);
        let assign14420_e21340: f64 = (0.25 * 0.2);
        let assign14420_e21342: f64 = (assign14420_e21340 * 0.2);
        let assign14420_e21343: f64 = (assign14420_e21337 + assign14420_e21342);
        let assign14420_e21344: f64 = (assign14420_e21343).sqrt();
        let assign14420_e21345: f64 = (assign14420_e21330 + assign14420_e21344);
        let assign14420_e21346: f64 = (0.5 * assign14420_e21345);
        (assign14420_e21346, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn0))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn2))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn3))) / (2.0 * assign14420_e21344)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14420_e21336) + (assign14420_e21333 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn5))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn6))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn7))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn8))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn9))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn10))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn11))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn13))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn14))) / (2.0 * assign14420_e21344)))),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14420_e21348;
        locals.var_devtempeff_dn0 = assign14420_e21348_d_n0;
        locals.var_devtempeff_dn2 = assign14420_e21348_d_n2;
        locals.var_devtempeff_dn3 = assign14420_e21348_d_n3;
        locals.var_devtempeff_dn4 = assign14420_e21348_d_n4;
        locals.var_devtempeff_dn5 = assign14420_e21348_d_n5;
        locals.var_devtempeff_dn6 = assign14420_e21348_d_n6;
        locals.var_devtempeff_dn7 = assign14420_e21348_d_n7;
        locals.var_devtempeff_dn8 = assign14420_e21348_d_n8;
        locals.var_devtempeff_dn9 = assign14420_e21348_d_n9;
        locals.var_devtempeff_dn10 = assign14420_e21348_d_n10;
        locals.var_devtempeff_dn11 = assign14420_e21348_d_n11;
        locals.var_devtempeff_dn13 = assign14420_e21348_d_n13;
        locals.var_devtempeff_dn14 = assign14420_e21348_d_n14;
        locals.var_devtempeff_rv = 0.0;

        let (assign14430_e21377, assign14430_e21377_d_n0, assign14430_e21377_d_n2, assign14430_e21377_d_n3, assign14430_e21377_d_n4, assign14430_e21377_d_n5, assign14430_e21377_d_n6, assign14430_e21377_d_n7, assign14430_e21377_d_n8, assign14430_e21377_d_n9, assign14430_e21377_d_n10, assign14430_e21377_d_n11, assign14430_e21377_d_n13, assign14430_e21377_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14430_e21359: f64 = (locals.var_tnom + locals.var_tlow);
        let assign14430_e21362: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21365: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21366: f64 = (assign14430_e21362 * assign14430_e21365);
        let assign14430_e21369: f64 = (0.25 * p.p1788);
        let assign14430_e21371: f64 = (assign14430_e21369 * p.p1788);
        let assign14430_e21372: f64 = (assign14430_e21366 + assign14430_e21371);
        let assign14430_e21373: f64 = (assign14430_e21372).sqrt();
        let assign14430_e21374: f64 = (assign14430_e21359 + assign14430_e21373);
        let assign14430_e21375: f64 = (0.5 * assign14430_e21374);
        (assign14430_e21375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14430_e21377;
        locals.var_t1_dn0 = assign14430_e21377_d_n0;
        locals.var_t1_dn2 = assign14430_e21377_d_n2;
        locals.var_t1_dn3 = assign14430_e21377_d_n3;
        locals.var_t1_dn4 = assign14430_e21377_d_n4;
        locals.var_t1_dn5 = assign14430_e21377_d_n5;
        locals.var_t1_dn6 = assign14430_e21377_d_n6;
        locals.var_t1_dn7 = assign14430_e21377_d_n7;
        locals.var_t1_dn8 = assign14430_e21377_d_n8;
        locals.var_t1_dn9 = assign14430_e21377_d_n9;
        locals.var_t1_dn10 = assign14430_e21377_d_n10;
        locals.var_t1_dn11 = assign14430_e21377_d_n11;
        locals.var_t1_dn13 = assign14430_e21377_d_n13;
        locals.var_t1_dn14 = assign14430_e21377_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14440_e21421, assign14440_e21421_d_n0, assign14440_e21421_d_n2, assign14440_e21421_d_n3, assign14440_e21421_d_n4, assign14440_e21421_d_n5, assign14440_e21421_d_n6, assign14440_e21421_d_n7, assign14440_e21421_d_n8, assign14440_e21421_d_n9, assign14440_e21421_d_n10, assign14440_e21421_d_n11, assign14440_e21421_d_n13, assign14440_e21421_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14440_e21387: f64 = (-p.p1790);
        let assign14440_e21390: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21391: f64 = (assign14440_e21387 * assign14440_e21390);
        let assign14440_e21393: f64 = assign14440_e21391;
        let assign14440_e21395: f64 = (-p.p1790);
        let assign14440_e21398: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21399: f64 = (assign14440_e21395 * assign14440_e21398);
        let assign14440_e21401: f64 = assign14440_e21399;
        let assign14440_e21403: f64 = (-p.p1790);
        let assign14440_e21406: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21407: f64 = (assign14440_e21403 * assign14440_e21406);
        let assign14440_e21409: f64 = assign14440_e21407;
        let assign14440_e21410: f64 = (assign14440_e21401 * assign14440_e21409);
        let assign14440_e21413: f64 = (0.25 * p.p1789);
        let assign14440_e21415: f64 = (assign14440_e21413 * p.p1789);
        let assign14440_e21416: f64 = (assign14440_e21410 + assign14440_e21415);
        let assign14440_e21417: f64 = (assign14440_e21416).sqrt();
        let assign14440_e21418: f64 = (assign14440_e21393 + assign14440_e21417);
        let assign14440_e21419: f64 = (0.5 * assign14440_e21418);
        (assign14440_e21419, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14440_e21421;
        locals.var_t2_dn0 = assign14440_e21421_d_n0;
        locals.var_t2_dn2 = assign14440_e21421_d_n2;
        locals.var_t2_dn3 = assign14440_e21421_d_n3;
        locals.var_t2_dn4 = assign14440_e21421_d_n4;
        locals.var_t2_dn5 = assign14440_e21421_d_n5;
        locals.var_t2_dn6 = assign14440_e21421_d_n6;
        locals.var_t2_dn7 = assign14440_e21421_d_n7;
        locals.var_t2_dn8 = assign14440_e21421_d_n8;
        locals.var_t2_dn9 = assign14440_e21421_d_n9;
        locals.var_t2_dn10 = assign14440_e21421_d_n10;
        locals.var_t2_dn11 = assign14440_e21421_d_n11;
        locals.var_t2_dn13 = assign14440_e21421_d_n13;
        locals.var_t2_dn14 = assign14440_e21421_d_n14;
        locals.var_t2_rv = 0.0;

        let assign14450_e21424: f64 = if locals.var_tnom > locals.var_tlow { 1.0 } else { 0.0 };
        locals.var_guard242 = assign14450_e21424;
        locals.var_guard242_rv = 0.0;

        let (assign14460_e21444, assign14460_e21444_d_n0, assign14460_e21444_d_n2, assign14460_e21444_d_n3, assign14460_e21444_d_n4, assign14460_e21444_d_n5, assign14460_e21444_d_n6, assign14460_e21444_d_n7, assign14460_e21444_d_n8, assign14460_e21444_d_n9, assign14460_e21444_d_n10, assign14460_e21444_d_n11, assign14460_e21444_d_n13, assign14460_e21444_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        let assign14460_e21436: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14460_e21438: f64 = (assign14460_e21436 - locals.var_t1);
        let assign14460_e21440: f64 = (assign14460_e21438 - locals.var_t2);
        let assign14460_e21442: f64 = (assign14460_e21440 + locals.var_tnom);
        (assign14460_e21442, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14460_e21444;
        locals.var_t3_dn0 = assign14460_e21444_d_n0;
        locals.var_t3_dn2 = assign14460_e21444_d_n2;
        locals.var_t3_dn3 = assign14460_e21444_d_n3;
        locals.var_t3_dn4 = assign14460_e21444_d_n4;
        locals.var_t3_dn5 = assign14460_e21444_d_n5;
        locals.var_t3_dn6 = assign14460_e21444_d_n6;
        locals.var_t3_dn7 = assign14460_e21444_d_n7;
        locals.var_t3_dn8 = assign14460_e21444_d_n8;
        locals.var_t3_dn9 = assign14460_e21444_d_n9;
        locals.var_t3_dn10 = assign14460_e21444_d_n10;
        locals.var_t3_dn11 = assign14460_e21444_d_n11;
        locals.var_t3_dn13 = assign14460_e21444_d_n13;
        locals.var_t3_dn14 = assign14460_e21444_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14470_e21465, assign14470_e21465_d_n0, assign14470_e21465_d_n2, assign14470_e21465_d_n3, assign14470_e21465_d_n4, assign14470_e21465_d_n5, assign14470_e21465_d_n6, assign14470_e21465_d_n7, assign14470_e21465_d_n8, assign14470_e21465_d_n9, assign14470_e21465_d_n10, assign14470_e21465_d_n11, assign14470_e21465_d_n13, assign14470_e21465_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        let assign14470_e21457: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14470_e21459: f64 = (assign14470_e21457 - locals.var_t1);
        let assign14470_e21461: f64 = (assign14470_e21459 - locals.var_t2);
        let assign14470_e21463: f64 = (assign14470_e21461 + locals.var_tlow);
        (assign14470_e21463, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14470_e21465;
        locals.var_t3_dn0 = assign14470_e21465_d_n0;
        locals.var_t3_dn2 = assign14470_e21465_d_n2;
        locals.var_t3_dn3 = assign14470_e21465_d_n3;
        locals.var_t3_dn4 = assign14470_e21465_d_n4;
        locals.var_t3_dn5 = assign14470_e21465_d_n5;
        locals.var_t3_dn6 = assign14470_e21465_d_n6;
        locals.var_t3_dn7 = assign14470_e21465_d_n7;
        locals.var_t3_dn8 = assign14470_e21465_d_n8;
        locals.var_t3_dn9 = assign14470_e21465_d_n9;
        locals.var_t3_dn10 = assign14470_e21465_d_n10;
        locals.var_t3_dn11 = assign14470_e21465_d_n11;
        locals.var_t3_dn13 = assign14470_e21465_d_n13;
        locals.var_t3_dn14 = assign14470_e21465_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign14480_e21494, assign14480_e21494_d_n0, assign14480_e21494_d_n2, assign14480_e21494_d_n3, assign14480_e21494_d_n4, assign14480_e21494_d_n5, assign14480_e21494_d_n6, assign14480_e21494_d_n7, assign14480_e21494_d_n8, assign14480_e21494_d_n9, assign14480_e21494_d_n10, assign14480_e21494_d_n11, assign14480_e21494_d_n13, assign14480_e21494_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14480_e21476: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14480_e21479: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21482: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21483: f64 = (assign14480_e21479 * assign14480_e21482);
        let assign14480_e21486: f64 = (0.25 * 0.2);
        let assign14480_e21488: f64 = (assign14480_e21486 * 0.2);
        let assign14480_e21489: f64 = (assign14480_e21483 + assign14480_e21488);
        let assign14480_e21490: f64 = (assign14480_e21489).sqrt();
        let assign14480_e21491: f64 = (assign14480_e21476 + assign14480_e21490);
        let assign14480_e21492: f64 = (0.5 * assign14480_e21491);
        (assign14480_e21492, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn0))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn2))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn3))) / (2.0 * assign14480_e21490)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14480_e21482) + (assign14480_e21479 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn5))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn6))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn7))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn8))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn9))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn10))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn11))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn13))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn14))) / (2.0 * assign14480_e21490)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14480_e21494;
        locals.var_t4_dn0 = assign14480_e21494_d_n0;
        locals.var_t4_dn2 = assign14480_e21494_d_n2;
        locals.var_t4_dn3 = assign14480_e21494_d_n3;
        locals.var_t4_dn4 = assign14480_e21494_d_n4;
        locals.var_t4_dn5 = assign14480_e21494_d_n5;
        locals.var_t4_dn6 = assign14480_e21494_d_n6;
        locals.var_t4_dn7 = assign14480_e21494_d_n7;
        locals.var_t4_dn8 = assign14480_e21494_d_n8;
        locals.var_t4_dn9 = assign14480_e21494_d_n9;
        locals.var_t4_dn10 = assign14480_e21494_d_n10;
        locals.var_t4_dn11 = assign14480_e21494_d_n11;
        locals.var_t4_dn13 = assign14480_e21494_d_n13;
        locals.var_t4_dn14 = assign14480_e21494_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign14490_e21510, assign14490_e21510_d_n0, assign14490_e21510_d_n2, assign14490_e21510_d_n3, assign14490_e21510_d_n4, assign14490_e21510_d_n5, assign14490_e21510_d_n6, assign14490_e21510_d_n7, assign14490_e21510_d_n8, assign14490_e21510_d_n9, assign14490_e21510_d_n10, assign14490_e21510_d_n11, assign14490_e21510_d_n13, assign14490_e21510_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14490_e21504: f64 = (locals.var_wl * locals.var_t4);
        let assign14490_e21507: f64 = (locals.var_wh * locals.var_devtemp);
        let assign14490_e21508: f64 = (assign14490_e21504 + assign14490_e21507);
        (assign14490_e21508, (locals.var_wl * locals.var_t4_dn0), (locals.var_wl * locals.var_t4_dn2), (locals.var_wl * locals.var_t4_dn3), (((locals.var_wl_dn4 * locals.var_t4) + (locals.var_wl * locals.var_t4_dn4)) + ((locals.var_wh_dn4 * locals.var_devtemp) + (locals.var_wh * locals.var_devtemp_dn4))), (locals.var_wl * locals.var_t4_dn5), (locals.var_wl * locals.var_t4_dn6), (locals.var_wl * locals.var_t4_dn7), (locals.var_wl * locals.var_t4_dn8), (locals.var_wl * locals.var_t4_dn9), (locals.var_wl * locals.var_t4_dn10), (locals.var_wl * locals.var_t4_dn11), (locals.var_wl * locals.var_t4_dn13), (locals.var_wl * locals.var_t4_dn14),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14490_e21510;
        locals.var_devtempeff_dn0 = assign14490_e21510_d_n0;
        locals.var_devtempeff_dn2 = assign14490_e21510_d_n2;
        locals.var_devtempeff_dn3 = assign14490_e21510_d_n3;
        locals.var_devtempeff_dn4 = assign14490_e21510_d_n4;
        locals.var_devtempeff_dn5 = assign14490_e21510_d_n5;
        locals.var_devtempeff_dn6 = assign14490_e21510_d_n6;
        locals.var_devtempeff_dn7 = assign14490_e21510_d_n7;
        locals.var_devtempeff_dn8 = assign14490_e21510_d_n8;
        locals.var_devtempeff_dn9 = assign14490_e21510_d_n9;
        locals.var_devtempeff_dn10 = assign14490_e21510_d_n10;
        locals.var_devtempeff_dn11 = assign14490_e21510_d_n11;
        locals.var_devtempeff_dn13 = assign14490_e21510_d_n13;
        locals.var_devtempeff_dn14 = assign14490_e21510_d_n14;
        locals.var_devtempeff_rv = 0.0;

        let (assign14500_e21536, assign14500_e21536_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14500_e21518: f64 = (locals.var_devtemp + 210.0);
        let assign14500_e21521: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21524: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21525: f64 = (assign14500_e21521 * assign14500_e21524);
        let assign14500_e21528: f64 = (0.25 * 0.2);
        let assign14500_e21530: f64 = (assign14500_e21528 * 0.2);
        let assign14500_e21531: f64 = (assign14500_e21525 + assign14500_e21530);
        let assign14500_e21532: f64 = (assign14500_e21531).sqrt();
        let assign14500_e21533: f64 = (assign14500_e21518 - assign14500_e21532);
        let assign14500_e21534: f64 = (0.5 * assign14500_e21533);
        (assign14500_e21534, (0.5 * (locals.var_devtemp_dn4 - (((locals.var_devtemp_dn4 * assign14500_e21524) + (assign14500_e21521 * locals.var_devtemp_dn4)) / (2.0 * assign14500_e21532)))),)
    } else {
        (locals.var_devtemp1, locals.var_devtemp1_dn4,)
    }
};
        locals.var_devtemp1 = assign14500_e21536;
        locals.var_devtemp1_dn4 = assign14500_e21536_d_n4;
        locals.var_devtemp1_rv = 0.0;

        let (assign14510_e21564, assign14510_e21564_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14510_e21545: f64 = (locals.var_tnom + 210.0);
        let assign14510_e21548: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21551: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21552: f64 = (assign14510_e21548 * assign14510_e21551);
        let assign14510_e21555: f64 = (0.25 * 0.2);
        let assign14510_e21557: f64 = (assign14510_e21555 * 0.2);
        let assign14510_e21558: f64 = (assign14510_e21552 + assign14510_e21557);
        let assign14510_e21559: f64 = (assign14510_e21558).sqrt();
        let assign14510_e21560: f64 = (assign14510_e21545 - assign14510_e21559);
        let assign14510_e21561: f64 = (0.5 * assign14510_e21560);
        let assign14510_e21562: f64 = (locals.var_devtemp1 - assign14510_e21561);
        (assign14510_e21562, locals.var_devtemp1_dn4,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign14510_e21564;
        locals.var_deltemp1_dn4 = assign14510_e21564_d_n4;
        locals.var_deltemp1_rv = 0.0;

        let (assign14520_e21575, assign14520_e21575_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14520_e21571: f64 = (locals.var_devtemp1 - 210.0);
        let assign14520_e21573: f64 = (assign14520_e21571 / locals.var_tnom);
        (assign14520_e21573, (locals.var_devtemp1_dn4 / locals.var_tnom),)
    } else {
        (locals.var_deltratio1, locals.var_deltratio1_dn4,)
    }
};
        locals.var_deltratio1 = assign14520_e21575;
        locals.var_deltratio1_dn4 = assign14520_e21575_d_n4;
        locals.var_deltratio1_rv = 0.0;

        let (assign14530_e21581, assign14530_e21581_d_n0, assign14530_e21581_d_n2, assign14530_e21581_d_n3, assign14530_e21581_d_n4, assign14530_e21581_d_n5, assign14530_e21581_d_n6, assign14530_e21581_d_n7, assign14530_e21581_d_n8, assign14530_e21581_d_n9, assign14530_e21581_d_n10, assign14530_e21581_d_n11, assign14530_e21581_d_n13, assign14530_e21581_d_n14,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14530_e21579: f64 = (8.617087e-5 * locals.var_devtempeff);
        (assign14530_e21579, (8.617087e-5 * locals.var_devtempeff_dn0), (8.617087e-5 * locals.var_devtempeff_dn2), (8.617087e-5 * locals.var_devtempeff_dn3), (8.617087e-5 * locals.var_devtempeff_dn4), (8.617087e-5 * locals.var_devtempeff_dn5), (8.617087e-5 * locals.var_devtempeff_dn6), (8.617087e-5 * locals.var_devtempeff_dn7), (8.617087e-5 * locals.var_devtempeff_dn8), (8.617087e-5 * locals.var_devtempeff_dn9), (8.617087e-5 * locals.var_devtempeff_dn10), (8.617087e-5 * locals.var_devtempeff_dn11), (8.617087e-5 * locals.var_devtempeff_dn13), (8.617087e-5 * locals.var_devtempeff_dn14),)
    } else {
        (locals.var_vtmeff, locals.var_vtmeff_dn0, locals.var_vtmeff_dn2, locals.var_vtmeff_dn3, locals.var_vtmeff_dn4, locals.var_vtmeff_dn5, locals.var_vtmeff_dn6, locals.var_vtmeff_dn7, locals.var_vtmeff_dn8, locals.var_vtmeff_dn9, locals.var_vtmeff_dn10, locals.var_vtmeff_dn11, locals.var_vtmeff_dn13, locals.var_vtmeff_dn14,)
    }
};
        locals.var_vtmeff = assign14530_e21581;
        locals.var_vtmeff_dn0 = assign14530_e21581_d_n0;
        locals.var_vtmeff_dn2 = assign14530_e21581_d_n2;
        locals.var_vtmeff_dn3 = assign14530_e21581_d_n3;
        locals.var_vtmeff_dn4 = assign14530_e21581_d_n4;
        locals.var_vtmeff_dn5 = assign14530_e21581_d_n5;
        locals.var_vtmeff_dn6 = assign14530_e21581_d_n6;
        locals.var_vtmeff_dn7 = assign14530_e21581_d_n7;
        locals.var_vtmeff_dn8 = assign14530_e21581_d_n8;
        locals.var_vtmeff_dn9 = assign14530_e21581_d_n9;
        locals.var_vtmeff_dn10 = assign14530_e21581_d_n10;
        locals.var_vtmeff_dn11 = assign14530_e21581_d_n11;
        locals.var_vtmeff_dn13 = assign14530_e21581_d_n13;
        locals.var_vtmeff_dn14 = assign14530_e21581_d_n14;
        locals.var_vtmeff_rv = 0.0;

        let assign14540_e21585: f64 = (p.p1718 * locals.var_devtemp);
        let assign14540_e21587: f64 = (assign14540_e21585 * locals.var_devtemp);
        let assign14540_e21590: f64 = (locals.var_devtemp + p.p1719);
        let assign14540_e21591: f64 = (assign14540_e21587 / assign14540_e21590);
        let assign14540_e21592: f64 = (p.p106 - assign14540_e21591);
        locals.var_eg = assign14540_e21592;
        locals.var_eg_dn4 = (-((((((p.p1718 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign14540_e21585 * locals.var_devtemp_dn4)) * assign14540_e21590) - (assign14540_e21587 * locals.var_devtemp_dn4)) / (assign14540_e21590 * assign14540_e21590)));
        locals.var_eg_rv = 0.0;

        let assign14550_e21596: f64 = (p.p1718 * locals.var_tnom);
        let assign14550_e21598: f64 = (assign14550_e21596 * locals.var_tnom);
        let assign14550_e21601: f64 = (locals.var_tnom + p.p1719);
        let assign14550_e21602: f64 = (assign14550_e21598 / assign14550_e21601);
        let assign14550_e21603: f64 = (p.p106 - assign14550_e21602);
        locals.var_eg0 = assign14550_e21603;
        locals.var_eg0_rv = 0.0;

        let __rspice_inv_cse_0: f64 = 1.0 / 300.15;
        let assign14560_e21606: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21609: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21610: f64 = (assign14560_e21609).sqrt();
        let assign14560_e21611: f64 = (assign14560_e21606 * assign14560_e21610);
        locals.var_t1 = assign14560_e21611;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / 300.15) * assign14560_e21610) + (assign14560_e21606 * ((locals.var_devtemp_dn4 / 300.15) / (2.0 * assign14560_e21610))));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14570_e21614: f64 = (p.p105 * locals.var_t1);
        let assign14570_e21618: f64 = (2.0 * 8.617087e-5);
        let assign14570_e21620: f64 = (assign14570_e21618 * 300.15);
        let assign14570_e21621: f64 = (p.p106 / assign14570_e21620);
        let assign14570_e21625: f64 = (2.0 * locals.var_vtm);
        let assign14570_e21626: f64 = (locals.var_eg / assign14570_e21625);
        let assign14570_e21627: f64 = (assign14570_e21621 - assign14570_e21626);
        let assign14570_e21628: f64 = { let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14570_e21629: f64 = (assign14570_e21614 * assign14570_e21628);
        locals.var_ni = assign14570_e21629;
        locals.var_ni_dn0 = ((p.p105 * locals.var_t1_dn0) * assign14570_e21628);
        locals.var_ni_dn2 = ((p.p105 * locals.var_t1_dn2) * assign14570_e21628);
        locals.var_ni_dn3 = ((p.p105 * locals.var_t1_dn3) * assign14570_e21628);
        locals.var_ni_dn4 = (((p.p105 * locals.var_t1_dn4) * assign14570_e21628) + (assign14570_e21614 * ({ let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-(((locals.var_eg_dn4 * assign14570_e21625) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14570_e21625 * assign14570_e21625))))));
        locals.var_ni_dn5 = ((p.p105 * locals.var_t1_dn5) * assign14570_e21628);
        locals.var_ni_dn6 = ((p.p105 * locals.var_t1_dn6) * assign14570_e21628);
        locals.var_ni_dn7 = ((p.p105 * locals.var_t1_dn7) * assign14570_e21628);
        locals.var_ni_dn8 = ((p.p105 * locals.var_t1_dn8) * assign14570_e21628);
        locals.var_ni_dn9 = ((p.p105 * locals.var_t1_dn9) * assign14570_e21628);
        locals.var_ni_dn10 = ((p.p105 * locals.var_t1_dn10) * assign14570_e21628);
        locals.var_ni_dn11 = ((p.p105 * locals.var_t1_dn11) * assign14570_e21628);
        locals.var_ni_dn13 = ((p.p105 * locals.var_t1_dn13) * assign14570_e21628);
        locals.var_ni_dn14 = ((p.p105 * locals.var_t1_dn14) * assign14570_e21628);
        locals.var_ni_rv = 0.0;

        let assign14580_e21632: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign14580_e21632;
        locals.var_guard243_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14590_e21638, assign14590_e21638_d_n0, assign14590_e21638_d_n2, assign14590_e21638_d_n3, assign14590_e21638_d_n4, assign14590_e21638_d_n5, assign14590_e21638_d_n6, assign14590_e21638_d_n7, assign14590_e21638_d_n8, assign14590_e21638_d_n9, assign14590_e21638_d_n10, assign14590_e21638_d_n11, assign14590_e21638_d_n13, assign14590_e21638_d_n14,) = {
    if (locals.var_guard243 != 0.0) {
        let assign14590_e21636: f64 = (p.p107 * locals.var_t1);
        (assign14590_e21636, (p.p107 * locals.var_t1_dn0), (p.p107 * locals.var_t1_dn2), (p.p107 * locals.var_t1_dn3), (p.p107 * locals.var_t1_dn4), (p.p107 * locals.var_t1_dn5), (p.p107 * locals.var_t1_dn6), (p.p107 * locals.var_t1_dn7), (p.p107 * locals.var_t1_dn8), (p.p107 * locals.var_t1_dn9), (p.p107 * locals.var_t1_dn10), (p.p107 * locals.var_t1_dn11), (p.p107 * locals.var_t1_dn13), (p.p107 * locals.var_t1_dn14),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14590_e21638;
        locals.var_nc_dn0 = assign14590_e21638_d_n0;
        locals.var_nc_dn2 = assign14590_e21638_d_n2;
        locals.var_nc_dn3 = assign14590_e21638_d_n3;
        locals.var_nc_dn4 = assign14590_e21638_d_n4;
        locals.var_nc_dn5 = assign14590_e21638_d_n5;
        locals.var_nc_dn6 = assign14590_e21638_d_n6;
        locals.var_nc_dn7 = assign14590_e21638_d_n7;
        locals.var_nc_dn8 = assign14590_e21638_d_n8;
        locals.var_nc_dn9 = assign14590_e21638_d_n9;
        locals.var_nc_dn10 = assign14590_e21638_d_n10;
        locals.var_nc_dn11 = assign14590_e21638_d_n11;
        locals.var_nc_dn13 = assign14590_e21638_d_n13;
        locals.var_nc_dn14 = assign14590_e21638_d_n14;
        locals.var_nc_rv = 0.0;

        let (assign14600_e21652, assign14600_e21652_d_n0, assign14600_e21652_d_n2, assign14600_e21652_d_n3, assign14600_e21652_d_n4, assign14600_e21652_d_n5, assign14600_e21652_d_n6, assign14600_e21652_d_n7, assign14600_e21652_d_n8, assign14600_e21652_d_n9, assign14600_e21652_d_n10, assign14600_e21652_d_n11, assign14600_e21652_d_n13, assign14600_e21652_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14600_e21644: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21645: f64 = (p.p107 * assign14600_e21644);
        let assign14600_e21648: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21649: f64 = (assign14600_e21648).sqrt();
        let assign14600_e21650: f64 = (assign14600_e21645 * assign14600_e21649);
        (assign14600_e21650, (((p.p107 * (locals.var_devtempeff_dn0 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn0 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn2 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn2 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn3 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn3 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn4 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn4 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn5 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn5 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn6 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn6 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn7 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn7 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn8 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn8 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn9 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn9 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn10 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn10 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn11 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn11 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn13 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn13 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn14 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn14 / 300.15) / (2.0 * assign14600_e21649)))),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14600_e21652;
        locals.var_nc_dn0 = assign14600_e21652_d_n0;
        locals.var_nc_dn2 = assign14600_e21652_d_n2;
        locals.var_nc_dn3 = assign14600_e21652_d_n3;
        locals.var_nc_dn4 = assign14600_e21652_d_n4;
        locals.var_nc_dn5 = assign14600_e21652_d_n5;
        locals.var_nc_dn6 = assign14600_e21652_d_n6;
        locals.var_nc_dn7 = assign14600_e21652_d_n7;
        locals.var_nc_dn8 = assign14600_e21652_d_n8;
        locals.var_nc_dn9 = assign14600_e21652_d_n9;
        locals.var_nc_dn10 = assign14600_e21652_d_n10;
        locals.var_nc_dn11 = assign14600_e21652_d_n11;
        locals.var_nc_dn13 = assign14600_e21652_d_n13;
        locals.var_nc_dn14 = assign14600_e21652_d_n14;
        locals.var_nc_rv = 0.0;

        let (assign14610_e21690, assign14610_e21690_d_n0, assign14610_e21690_d_n2, assign14610_e21690_d_n3, assign14610_e21690_d_n4, assign14610_e21690_d_n5, assign14610_e21690_d_n6, assign14610_e21690_d_n7, assign14610_e21690_d_n8, assign14610_e21690_d_n9, assign14610_e21690_d_n10, assign14610_e21690_d_n11, assign14610_e21690_d_n13, assign14610_e21690_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14610_e21657: f64 = (p.p105 * locals.var_t1);
        let (assign14610_e21674, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, assign14610_e21674_d_n4, assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,) = {
            if (!(assign14610_e21657 > 1e-38)) {
                let assign14610_e21662: f64 = (-87.498233534);
                (assign14610_e21662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign14610_e21665: f64 = (p.p105 * locals.var_t1);
                let (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,) = {
                    if (assign14610_e21665 > 1e-38) {
                        let assign14610_e21670: f64 = (p.p105 * locals.var_t1);
                        let assign14610_e21671: f64 = (assign14610_e21670).ln();
                        (assign14610_e21671, ((p.p105 * locals.var_t1_dn0) / assign14610_e21670), ((p.p105 * locals.var_t1_dn2) / assign14610_e21670), ((p.p105 * locals.var_t1_dn3) / assign14610_e21670), ((p.p105 * locals.var_t1_dn4) / assign14610_e21670), ((p.p105 * locals.var_t1_dn5) / assign14610_e21670), ((p.p105 * locals.var_t1_dn6) / assign14610_e21670), ((p.p105 * locals.var_t1_dn7) / assign14610_e21670), ((p.p105 * locals.var_t1_dn8) / assign14610_e21670), ((p.p105 * locals.var_t1_dn9) / assign14610_e21670), ((p.p105 * locals.var_t1_dn10) / assign14610_e21670), ((p.p105 * locals.var_t1_dn11) / assign14610_e21670), ((p.p105 * locals.var_t1_dn13) / assign14610_e21670), ((p.p105 * locals.var_t1_dn14) / assign14610_e21670),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,)
            }
        };
        let assign14610_e21678: f64 = (2.0 * 8.617087e-5);
        let assign14610_e21680: f64 = (assign14610_e21678 * 300.15);
        let assign14610_e21681: f64 = (p.p106 / assign14610_e21680);
        let assign14610_e21682: f64 = (assign14610_e21674 + assign14610_e21681);
        let assign14610_e21686: f64 = (2.0 * locals.var_vtm);
        let assign14610_e21687: f64 = (locals.var_eg / assign14610_e21686);
        let assign14610_e21688: f64 = (assign14610_e21682 - assign14610_e21687);
        (assign14610_e21688, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, (assign14610_e21674_d_n4 - (((locals.var_eg_dn4 * assign14610_e21686) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14610_e21686 * assign14610_e21686))), assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,)
    } else {
        (locals.var_niln, locals.var_niln_dn0, locals.var_niln_dn2, locals.var_niln_dn3, locals.var_niln_dn4, locals.var_niln_dn5, locals.var_niln_dn6, locals.var_niln_dn7, locals.var_niln_dn8, locals.var_niln_dn9, locals.var_niln_dn10, locals.var_niln_dn11, locals.var_niln_dn13, locals.var_niln_dn14,)
    }
};
        locals.var_niln = assign14610_e21690;
        locals.var_niln_dn0 = assign14610_e21690_d_n0;
        locals.var_niln_dn2 = assign14610_e21690_d_n2;
        locals.var_niln_dn3 = assign14610_e21690_d_n3;
        locals.var_niln_dn4 = assign14610_e21690_d_n4;
        locals.var_niln_dn5 = assign14610_e21690_d_n5;
        locals.var_niln_dn6 = assign14610_e21690_d_n6;
        locals.var_niln_dn7 = assign14610_e21690_d_n7;
        locals.var_niln_dn8 = assign14610_e21690_d_n8;
        locals.var_niln_dn9 = assign14610_e21690_d_n9;
        locals.var_niln_dn10 = assign14610_e21690_d_n10;
        locals.var_niln_dn11 = assign14610_e21690_d_n11;
        locals.var_niln_dn13 = assign14610_e21690_d_n13;
        locals.var_niln_dn14 = assign14610_e21690_d_n14;
        locals.var_niln_rv = 0.0;

        let assign14620_e21694: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21695: f64 = (1.0 + assign14620_e21694);
        let assign14620_e21697: f64 = (assign14620_e21695 - 1e-6);
        let assign14620_e21699: f64 = (-10000.0);
        let assign14620_e21701: f64 = (assign14620_e21699 * 0.001);
        let (assign14620_e21762, assign14620_e21762_d_n4,) = {
    if (!(assign14620_e21697 < assign14620_e21701)) {
        let assign14620_e21708: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21709: f64 = (1.0 + assign14620_e21708);
        let assign14620_e21711: f64 = (assign14620_e21709 - 1e-6);
        let assign14620_e21715: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21716: f64 = (1.0 + assign14620_e21715);
        let assign14620_e21718: f64 = (assign14620_e21716 - 1e-6);
        let assign14620_e21722: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21723: f64 = (1.0 + assign14620_e21722);
        let assign14620_e21725: f64 = (assign14620_e21723 - 1e-6);
        let assign14620_e21726: f64 = (assign14620_e21718 * assign14620_e21725);
        let assign14620_e21729: f64 = (4.0 * 0.001);
        let assign14620_e21731: f64 = (assign14620_e21729 * 0.001);
        let assign14620_e21732: f64 = (assign14620_e21726 + assign14620_e21731);
        let assign14620_e21733: f64 = (assign14620_e21732).sqrt();
        let assign14620_e21734: f64 = (assign14620_e21711 + assign14620_e21733);
        let assign14620_e21735: f64 = (0.5 * assign14620_e21734);
        (assign14620_e21735, (0.5 * ((locals.var_tss_i * locals.var_deltemp_dn4) + ((((locals.var_tss_i * locals.var_deltemp_dn4) * assign14620_e21725) + (assign14620_e21718 * (locals.var_tss_i * locals.var_deltemp_dn4))) / (2.0 * assign14620_e21733)))),)
    } else {
        let assign14620_e21739: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21740: f64 = (1.0 + assign14620_e21739);
        let assign14620_e21742: f64 = (assign14620_e21740 - 1e-6);
        let assign14620_e21744: f64 = (-10000.0);
        let assign14620_e21746: f64 = (assign14620_e21744 * 0.001);
        let (assign14620_e21761, assign14620_e21761_d_n4,) = {
            if (assign14620_e21742 < assign14620_e21746) {
                let assign14620_e21749: f64 = (-0.001);
                let assign14620_e21751: f64 = (assign14620_e21749 * 0.001);
                let assign14620_e21755: f64 = (locals.var_tss_i * locals.var_deltemp);
                let assign14620_e21756: f64 = (1.0 + assign14620_e21755);
                let assign14620_e21758: f64 = (assign14620_e21756 - 1e-6);
                let assign14620_e21759: f64 = (assign14620_e21751 / assign14620_e21758);
                (assign14620_e21759, (-((assign14620_e21751 * (locals.var_tss_i * locals.var_deltemp_dn4)) / (assign14620_e21758 * assign14620_e21758))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14620_e21761, assign14620_e21761_d_n4,)
    }
};
        locals.var_thetass = assign14620_e21762;
        locals.var_thetass_dn4 = assign14620_e21762_d_n4;
        locals.var_thetass_rv = 0.0;

        let assign14630_e21765: f64 = (locals.var_vtm * 1.60219e-19);
        locals.var_kt = assign14630_e21765;
        locals.var_kt_dn4 = (locals.var_vtm_dn4 * 1.60219e-19);
        locals.var_kt_rv = 0.0;

        let assign14640_e21768: f64 = (1.05457e-34 * 3.141592653589793);
        let assign14640_e21771: f64 = (2.0 * locals.var_ach);
        let assign14640_e21773: f64 = (assign14640_e21771 / locals.var_weff_ufcm);
        let assign14640_e21774: f64 = (assign14640_e21768 / assign14640_e21773);
        locals.var_t0 = assign14640_e21774;
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
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign14650_e21777: f64 = (locals.var_t0 * locals.var_t0);
        let assign14650_e21780: f64 = (2.0 * locals.var_mx);
        let assign14650_e21781: f64 = (assign14650_e21777 / assign14650_e21780);
        locals.var_e0_1 = assign14650_e21781;
        locals.var_e0_1_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14650_e21780);
        locals.var_e0_1_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14650_e21780);
        locals.var_e0_1_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14650_e21780);
        locals.var_e0_1_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14650_e21780);
        locals.var_e0_1_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14650_e21780);
        locals.var_e0_1_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14650_e21780);
        locals.var_e0_1_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14650_e21780);
        locals.var_e0_1_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14650_e21780);
        locals.var_e0_1_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14650_e21780);
        locals.var_e0_1_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14650_e21780);
        locals.var_e0_1_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14650_e21780);
        locals.var_e0_1_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14650_e21780);
        locals.var_e0_1_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14650_e21780);
        locals.var_e0_1_rv = 0.0;

        let assign14660_e21784: f64 = (locals.var_t0 * locals.var_t0);
        let assign14660_e21787: f64 = (2.0 * locals.var_mxprime);
        let assign14660_e21788: f64 = (assign14660_e21784 / assign14660_e21787);
        locals.var_e0prime = assign14660_e21788;
        locals.var_e0prime_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14660_e21787);
        locals.var_e0prime_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14660_e21787);
        locals.var_e0prime_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14660_e21787);
        locals.var_e0prime_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14660_e21787);
        locals.var_e0prime_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14660_e21787);
        locals.var_e0prime_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14660_e21787);
        locals.var_e0prime_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14660_e21787);
        locals.var_e0prime_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14660_e21787);
        locals.var_e0prime_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14660_e21787);
        locals.var_e0prime_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14660_e21787);
        locals.var_e0prime_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14660_e21787);
        locals.var_e0prime_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14660_e21787);
        locals.var_e0prime_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14660_e21787);
        locals.var_e0prime_rv = 0.0;

        let assign14670_e21791: f64 = (4.0 * locals.var_e0_1);
        locals.var_e1_1 = assign14670_e21791;
        locals.var_e1_1_dn0 = (4.0 * locals.var_e0_1_dn0);
        locals.var_e1_1_dn2 = (4.0 * locals.var_e0_1_dn2);
        locals.var_e1_1_dn3 = (4.0 * locals.var_e0_1_dn3);
        locals.var_e1_1_dn4 = (4.0 * locals.var_e0_1_dn4);
        locals.var_e1_1_dn5 = (4.0 * locals.var_e0_1_dn5);
        locals.var_e1_1_dn6 = (4.0 * locals.var_e0_1_dn6);
        locals.var_e1_1_dn7 = (4.0 * locals.var_e0_1_dn7);
        locals.var_e1_1_dn8 = (4.0 * locals.var_e0_1_dn8);
        locals.var_e1_1_dn9 = (4.0 * locals.var_e0_1_dn9);
        locals.var_e1_1_dn10 = (4.0 * locals.var_e0_1_dn10);
        locals.var_e1_1_dn11 = (4.0 * locals.var_e0_1_dn11);
        locals.var_e1_1_dn13 = (4.0 * locals.var_e0_1_dn13);
        locals.var_e1_1_dn14 = (4.0 * locals.var_e0_1_dn14);
        locals.var_e1_1_rv = 0.0;

        let assign14680_e21794: f64 = (4.0 * locals.var_e0prime);
        locals.var_e1prime = assign14680_e21794;
        locals.var_e1prime_dn0 = (4.0 * locals.var_e0prime_dn0);
        locals.var_e1prime_dn2 = (4.0 * locals.var_e0prime_dn2);
        locals.var_e1prime_dn3 = (4.0 * locals.var_e0prime_dn3);
        locals.var_e1prime_dn4 = (4.0 * locals.var_e0prime_dn4);
        locals.var_e1prime_dn5 = (4.0 * locals.var_e0prime_dn5);
        locals.var_e1prime_dn6 = (4.0 * locals.var_e0prime_dn6);
        locals.var_e1prime_dn7 = (4.0 * locals.var_e0prime_dn7);
        locals.var_e1prime_dn8 = (4.0 * locals.var_e0prime_dn8);
        locals.var_e1prime_dn9 = (4.0 * locals.var_e0prime_dn9);
        locals.var_e1prime_dn10 = (4.0 * locals.var_e0prime_dn10);
        locals.var_e1prime_dn11 = (4.0 * locals.var_e0prime_dn11);
        locals.var_e1prime_dn13 = (4.0 * locals.var_e0prime_dn13);
        locals.var_e1prime_dn14 = (4.0 * locals.var_e0prime_dn14);
        locals.var_e1prime_rv = 0.0;

        let assign14690_e21797: f64 = (locals.var_gprime * locals.var_mdprime);
        let assign14690_e21800: f64 = (locals.var_gfactor * locals.var_md);
        let assign14690_e21801: f64 = (assign14690_e21797 / assign14690_e21800);
        locals.var_t1 = assign14690_e21801;
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
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14700_e21806: f64 = (locals.var_e0_1 - locals.var_e0prime);
        let assign14700_e21808: f64 = (assign14700_e21806 / locals.var_kt);
        let assign14700_e21809: f64 = { let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14700_e21810: f64 = (locals.var_t1 * assign14700_e21809);
        let assign14700_e21811: f64 = (1.0 + assign14700_e21810);
        locals.var_gam0 = assign14700_e21811;
        locals.var_gam0_dn0 = ((locals.var_t1_dn0 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e0prime_dn0) / locals.var_kt))));
        locals.var_gam0_dn2 = ((locals.var_t1_dn2 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e0prime_dn2) / locals.var_kt))));
        locals.var_gam0_dn3 = ((locals.var_t1_dn3 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e0prime_dn3) / locals.var_kt))));
        locals.var_gam0_dn4 = ((locals.var_t1_dn4 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e0prime_dn4) * locals.var_kt) - (assign14700_e21806 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))));
        locals.var_gam0_dn5 = ((locals.var_t1_dn5 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e0prime_dn5) / locals.var_kt))));
        locals.var_gam0_dn6 = ((locals.var_t1_dn6 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e0prime_dn6) / locals.var_kt))));
        locals.var_gam0_dn7 = ((locals.var_t1_dn7 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e0prime_dn7) / locals.var_kt))));
        locals.var_gam0_dn8 = ((locals.var_t1_dn8 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e0prime_dn8) / locals.var_kt))));
        locals.var_gam0_dn9 = ((locals.var_t1_dn9 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e0prime_dn9) / locals.var_kt))));
        locals.var_gam0_dn10 = ((locals.var_t1_dn10 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e0prime_dn10) / locals.var_kt))));
        locals.var_gam0_dn11 = ((locals.var_t1_dn11 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e0prime_dn11) / locals.var_kt))));
        locals.var_gam0_dn13 = ((locals.var_t1_dn13 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e0prime_dn13) / locals.var_kt))));
        locals.var_gam0_dn14 = ((locals.var_t1_dn14 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e0prime_dn14) / locals.var_kt))));
        locals.var_gam0_rv = 0.0;

        let assign14710_e21815: f64 = (locals.var_e0_1 - locals.var_e1_1);
        let assign14710_e21817: f64 = (assign14710_e21815 / locals.var_kt);
        let assign14710_e21818: f64 = { let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21819: f64 = (locals.var_gam0 + assign14710_e21818);
        let assign14710_e21823: f64 = (locals.var_e0_1 - locals.var_e1prime);
        let assign14710_e21825: f64 = (assign14710_e21823 / locals.var_kt);
        let assign14710_e21826: f64 = { let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21827: f64 = (locals.var_t1 * assign14710_e21826);
        let assign14710_e21828: f64 = (assign14710_e21819 + assign14710_e21827);
        locals.var_gam1 = assign14710_e21828;
        locals.var_gam1_dn0 = ((locals.var_gam0_dn0 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1_1_dn0) / locals.var_kt))) + ((locals.var_t1_dn0 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1prime_dn0) / locals.var_kt)))));
        locals.var_gam1_dn2 = ((locals.var_gam0_dn2 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1_1_dn2) / locals.var_kt))) + ((locals.var_t1_dn2 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1prime_dn2) / locals.var_kt)))));
        locals.var_gam1_dn3 = ((locals.var_gam0_dn3 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1_1_dn3) / locals.var_kt))) + ((locals.var_t1_dn3 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1prime_dn3) / locals.var_kt)))));
        locals.var_gam1_dn4 = ((locals.var_gam0_dn4 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1_1_dn4) * locals.var_kt) - (assign14710_e21815 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))) + ((locals.var_t1_dn4 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1prime_dn4) * locals.var_kt) - (assign14710_e21823 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt))))));
        locals.var_gam1_dn5 = ((locals.var_gam0_dn5 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1_1_dn5) / locals.var_kt))) + ((locals.var_t1_dn5 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1prime_dn5) / locals.var_kt)))));
        locals.var_gam1_dn6 = ((locals.var_gam0_dn6 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1_1_dn6) / locals.var_kt))) + ((locals.var_t1_dn6 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1prime_dn6) / locals.var_kt)))));
        locals.var_gam1_dn7 = ((locals.var_gam0_dn7 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1_1_dn7) / locals.var_kt))) + ((locals.var_t1_dn7 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1prime_dn7) / locals.var_kt)))));
        locals.var_gam1_dn8 = ((locals.var_gam0_dn8 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1_1_dn8) / locals.var_kt))) + ((locals.var_t1_dn8 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1prime_dn8) / locals.var_kt)))));
        locals.var_gam1_dn9 = ((locals.var_gam0_dn9 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1_1_dn9) / locals.var_kt))) + ((locals.var_t1_dn9 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1prime_dn9) / locals.var_kt)))));
        locals.var_gam1_dn10 = ((locals.var_gam0_dn10 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1_1_dn10) / locals.var_kt))) + ((locals.var_t1_dn10 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1prime_dn10) / locals.var_kt)))));
        locals.var_gam1_dn11 = ((locals.var_gam0_dn11 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1_1_dn11) / locals.var_kt))) + ((locals.var_t1_dn11 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1prime_dn11) / locals.var_kt)))));
        locals.var_gam1_dn13 = ((locals.var_gam0_dn13 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1_1_dn13) / locals.var_kt))) + ((locals.var_t1_dn13 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1prime_dn13) / locals.var_kt)))));
        locals.var_gam1_dn14 = ((locals.var_gam0_dn14 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1_1_dn14) / locals.var_kt))) + ((locals.var_t1_dn14 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1prime_dn14) / locals.var_kt)))));
        locals.var_gam1_rv = 0.0;

        let assign14720_e21830: f64 = (-locals.var_vtm);
        let assign14720_e21833: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21836: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21838: f64 = (assign14720_e21836 * 1.05457e-34);
        let assign14720_e21840: f64 = (assign14720_e21838 * locals.var_nc);
        let assign14720_e21841: f64 = (assign14720_e21833 / assign14720_e21840);
        let assign14720_e21843: f64 = (assign14720_e21841 * locals.var_kt);
        let assign14720_e21846: f64 = (2.0 * locals.var_ach);
        let assign14720_e21848: f64 = (assign14720_e21846 / locals.var_weff_ufcm);
        let assign14720_e21849: f64 = (assign14720_e21843 / assign14720_e21848);
        let assign14720_e21851: f64 = (assign14720_e21849 * locals.var_gam1);
        let (assign14720_e21904, assign14720_e21904_d_n0, assign14720_e21904_d_n2, assign14720_e21904_d_n3, assign14720_e21904_d_n4, assign14720_e21904_d_n5, assign14720_e21904_d_n6, assign14720_e21904_d_n7, assign14720_e21904_d_n8, assign14720_e21904_d_n9, assign14720_e21904_d_n10, assign14720_e21904_d_n11, assign14720_e21904_d_n13, assign14720_e21904_d_n14,) = {
    if (!(assign14720_e21851 > 1e-38)) {
        let assign14720_e21856: f64 = (-87.498233534);
        (assign14720_e21856, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign14720_e21859: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21862: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21864: f64 = (assign14720_e21862 * 1.05457e-34);
        let assign14720_e21866: f64 = (assign14720_e21864 * locals.var_nc);
        let assign14720_e21867: f64 = (assign14720_e21859 / assign14720_e21866);
        let assign14720_e21869: f64 = (assign14720_e21867 * locals.var_kt);
        let assign14720_e21872: f64 = (2.0 * locals.var_ach);
        let assign14720_e21874: f64 = (assign14720_e21872 / locals.var_weff_ufcm);
        let assign14720_e21875: f64 = (assign14720_e21869 / assign14720_e21874);
        let assign14720_e21877: f64 = (assign14720_e21875 * locals.var_gam1);
        let (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,) = {
            if (assign14720_e21877 > 1e-38) {
                let assign14720_e21882: f64 = (locals.var_gfactor * locals.var_md);
                let assign14720_e21885: f64 = (3.141592653589793 * 1.05457e-34);
                let assign14720_e21887: f64 = (assign14720_e21885 * 1.05457e-34);
                let assign14720_e21889: f64 = (assign14720_e21887 * locals.var_nc);
                let assign14720_e21890: f64 = (assign14720_e21882 / assign14720_e21889);
                let assign14720_e21892: f64 = (assign14720_e21890 * locals.var_kt);
                let assign14720_e21895: f64 = (2.0 * locals.var_ach);
                let assign14720_e21897: f64 = (assign14720_e21895 / locals.var_weff_ufcm);
                let assign14720_e21898: f64 = (assign14720_e21892 / assign14720_e21897);
                let assign14720_e21900: f64 = (assign14720_e21898 * locals.var_gam1);
                let assign14720_e21901: f64 = (assign14720_e21900).ln();
                (assign14720_e21901, ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn0)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn0)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn2)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn2)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn3)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn3)) / assign14720_e21900), (((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn4)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) + (assign14720_e21890 * locals.var_kt_dn4)) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn4)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn5)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn5)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn6)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn6)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn7)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn7)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn8)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn8)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn9)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn9)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn10)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn10)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn11)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn11)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn13)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn13)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn14)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn14)) / assign14720_e21900),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,)
    }
};
        let assign14720_e21905: f64 = (assign14720_e21830 * assign14720_e21904);
        locals.var_t2 = assign14720_e21905;
        locals.var_t2_dn0 = (assign14720_e21830 * assign14720_e21904_d_n0);
        locals.var_t2_dn2 = (assign14720_e21830 * assign14720_e21904_d_n2);
        locals.var_t2_dn3 = (assign14720_e21830 * assign14720_e21904_d_n3);
        locals.var_t2_dn4 = (((-locals.var_vtm_dn4) * assign14720_e21904) + (assign14720_e21830 * assign14720_e21904_d_n4));
        locals.var_t2_dn5 = (assign14720_e21830 * assign14720_e21904_d_n5);
        locals.var_t2_dn6 = (assign14720_e21830 * assign14720_e21904_d_n6);
        locals.var_t2_dn7 = (assign14720_e21830 * assign14720_e21904_d_n7);
        locals.var_t2_dn8 = (assign14720_e21830 * assign14720_e21904_d_n8);
        locals.var_t2_dn9 = (assign14720_e21830 * assign14720_e21904_d_n9);
        locals.var_t2_dn10 = (assign14720_e21830 * assign14720_e21904_d_n10);
        locals.var_t2_dn11 = (assign14720_e21830 * assign14720_e21904_d_n11);
        locals.var_t2_dn13 = (assign14720_e21830 * assign14720_e21904_d_n13);
        locals.var_t2_dn14 = (assign14720_e21830 * assign14720_e21904_d_n14);
        locals.var_t2_rv = 0.0;

        let assign14730_e21909: f64 = (locals.var_e0_1 / 1.60219e-19);
        let assign14730_e21911: f64 = (assign14730_e21909 + locals.var_t2);
        let assign14730_e21912: f64 = (locals.var_qmfactor_i * assign14730_e21911);
        locals.var_dvch_qm = assign14730_e21912;
        locals.var_dvch_qm_dn0 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn0 / 1.60219e-19) + locals.var_t2_dn0));
        locals.var_dvch_qm_dn2 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn2 / 1.60219e-19) + locals.var_t2_dn2));
        locals.var_dvch_qm_dn3 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn3 / 1.60219e-19) + locals.var_t2_dn3));
        locals.var_dvch_qm_dn4 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn4 / 1.60219e-19) + locals.var_t2_dn4));
        locals.var_dvch_qm_dn5 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn5 / 1.60219e-19) + locals.var_t2_dn5));
        locals.var_dvch_qm_dn6 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn6 / 1.60219e-19) + locals.var_t2_dn6));
        locals.var_dvch_qm_dn7 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn7 / 1.60219e-19) + locals.var_t2_dn7));
        locals.var_dvch_qm_dn8 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn8 / 1.60219e-19) + locals.var_t2_dn8));
        locals.var_dvch_qm_dn9 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn9 / 1.60219e-19) + locals.var_t2_dn9));
        locals.var_dvch_qm_dn10 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn10 / 1.60219e-19) + locals.var_t2_dn10));
        locals.var_dvch_qm_dn11 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn11 / 1.60219e-19) + locals.var_t2_dn11));
        locals.var_dvch_qm_dn13 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn13 / 1.60219e-19) + locals.var_t2_dn13));
        locals.var_dvch_qm_dn14 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn14 / 1.60219e-19) + locals.var_t2_dn14));
        locals.var_dvch_qm_rv = 0.0;

        let assign14740_e21914: f64 = (locals.var_tratio).ln();
        locals.var_trat_ln = assign14740_e21914;
        locals.var_trat_ln_dn4 = (locals.var_tratio_dn4 / locals.var_tratio);
        locals.var_trat_ln_rv = 0.0;

        let assign14750_e21917: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign14750_e21917;
        locals.var_guard244_rv = 0.0;

        let (assign14760_e21926, assign14760_e21926_d_n0, assign14760_e21926_d_n2, assign14760_e21926_d_n3, assign14760_e21926_d_n4, assign14760_e21926_d_n5, assign14760_e21926_d_n6, assign14760_e21926_d_n7, assign14760_e21926_d_n8, assign14760_e21926_d_n9, assign14760_e21926_d_n10, assign14760_e21926_d_n11, assign14760_e21926_d_n13, assign14760_e21926_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14760_e21922: f64 = (locals.var_ute_i * locals.var_trat_ln);
        let assign14760_e21923: f64 = (assign14760_e21922).exp();
        let assign14760_e21924: f64 = (locals.var_u0_i * assign14760_e21923);
        (assign14760_e21924, (locals.var_u0_i_dn0 * assign14760_e21923), (locals.var_u0_i_dn2 * assign14760_e21923), (locals.var_u0_i_dn3 * assign14760_e21923), ((locals.var_u0_i_dn4 * assign14760_e21923) + (locals.var_u0_i * (assign14760_e21923 * (locals.var_ute_i * locals.var_trat_ln_dn4)))), (locals.var_u0_i_dn5 * assign14760_e21923), (locals.var_u0_i_dn6 * assign14760_e21923), (locals.var_u0_i_dn7 * assign14760_e21923), (locals.var_u0_i_dn8 * assign14760_e21923), (locals.var_u0_i_dn9 * assign14760_e21923), (locals.var_u0_i_dn10 * assign14760_e21923), (locals.var_u0_i_dn11 * assign14760_e21923), (locals.var_u0_i_dn13 * assign14760_e21923), (locals.var_u0_i_dn14 * assign14760_e21923),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14760_e21926;
        locals.var_t1_dn0 = assign14760_e21926_d_n0;
        locals.var_t1_dn2 = assign14760_e21926_d_n2;
        locals.var_t1_dn3 = assign14760_e21926_d_n3;
        locals.var_t1_dn4 = assign14760_e21926_d_n4;
        locals.var_t1_dn5 = assign14760_e21926_d_n5;
        locals.var_t1_dn6 = assign14760_e21926_d_n6;
        locals.var_t1_dn7 = assign14760_e21926_d_n7;
        locals.var_t1_dn8 = assign14760_e21926_d_n8;
        locals.var_t1_dn9 = assign14760_e21926_d_n9;
        locals.var_t1_dn10 = assign14760_e21926_d_n10;
        locals.var_t1_dn11 = assign14760_e21926_d_n11;
        locals.var_t1_dn13 = assign14760_e21926_d_n13;
        locals.var_t1_dn14 = assign14760_e21926_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign14770_e21980, assign14770_e21980_d_n0, assign14770_e21980_d_n2, assign14770_e21980_d_n3, assign14770_e21980_d_n4, assign14770_e21980_d_n5, assign14770_e21980_d_n6, assign14770_e21980_d_n7, assign14770_e21980_d_n8, assign14770_e21980_d_n9, assign14770_e21980_d_n10, assign14770_e21980_d_n11, assign14770_e21980_d_n13, assign14770_e21980_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14770_e21930: f64 = (-0.9);
        let assign14770_e21932: f64 = (assign14770_e21930 * locals.var_t1);
        let assign14770_e21936: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21938: f64 = (-0.9);
        let assign14770_e21940: f64 = (assign14770_e21938 * locals.var_t1);
        let assign14770_e21941: f64 = (assign14770_e21936 - assign14770_e21940);
        let assign14770_e21943: f64 = (assign14770_e21941 - 0.0001);
        let assign14770_e21946: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21948: f64 = (-0.9);
        let assign14770_e21950: f64 = (assign14770_e21948 * locals.var_t1);
        let assign14770_e21951: f64 = (assign14770_e21946 - assign14770_e21950);
        let assign14770_e21953: f64 = (assign14770_e21951 - 0.0001);
        let assign14770_e21956: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21958: f64 = (-0.9);
        let assign14770_e21960: f64 = (assign14770_e21958 * locals.var_t1);
        let assign14770_e21961: f64 = (assign14770_e21956 - assign14770_e21960);
        let assign14770_e21963: f64 = (assign14770_e21961 - 0.0001);
        let assign14770_e21964: f64 = (assign14770_e21953 * assign14770_e21963);
        let assign14770_e21967: f64 = (-0.9);
        let assign14770_e21969: f64 = (assign14770_e21967 * locals.var_t1);
        let assign14770_e21970: f64 = (4.0 * assign14770_e21969);
        let assign14770_e21972: f64 = (assign14770_e21970 * 0.0001);
        let assign14770_e21973: f64 = (assign14770_e21964 - assign14770_e21972);
        let assign14770_e21974: f64 = (assign14770_e21973).sqrt();
        let assign14770_e21975: f64 = (assign14770_e21943 + assign14770_e21974);
        let assign14770_e21976: f64 = (0.5 * assign14770_e21975);
        let assign14770_e21977: f64 = (assign14770_e21932 + assign14770_e21976);
        let assign14770_e21978: f64 = (locals.var_t1 + assign14770_e21977);
        (assign14770_e21978, (locals.var_t1_dn0 + ((assign14770_e21930 * locals.var_t1_dn0) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn0)) + (((((-(assign14770_e21948 * locals.var_t1_dn0)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn0)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn2 + ((assign14770_e21930 * locals.var_t1_dn2) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn2)) + (((((-(assign14770_e21948 * locals.var_t1_dn2)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn2)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn3 + ((assign14770_e21930 * locals.var_t1_dn3) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn3)) + (((((-(assign14770_e21948 * locals.var_t1_dn3)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn3)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn4 + ((assign14770_e21930 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21938 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21948 * locals.var_t1_dn4)) * assign14770_e21963) + (assign14770_e21953 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21958 * locals.var_t1_dn4)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn5 + ((assign14770_e21930 * locals.var_t1_dn5) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn5)) + (((((-(assign14770_e21948 * locals.var_t1_dn5)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn5)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn6 + ((assign14770_e21930 * locals.var_t1_dn6) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn6)) + (((((-(assign14770_e21948 * locals.var_t1_dn6)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn6)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn7 + ((assign14770_e21930 * locals.var_t1_dn7) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn7)) + (((((-(assign14770_e21948 * locals.var_t1_dn7)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn7)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn8 + ((assign14770_e21930 * locals.var_t1_dn8) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn8)) + (((((-(assign14770_e21948 * locals.var_t1_dn8)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn8)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn9 + ((assign14770_e21930 * locals.var_t1_dn9) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn9)) + (((((-(assign14770_e21948 * locals.var_t1_dn9)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn9)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn10 + ((assign14770_e21930 * locals.var_t1_dn10) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn10)) + (((((-(assign14770_e21948 * locals.var_t1_dn10)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn10)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn11 + ((assign14770_e21930 * locals.var_t1_dn11) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn11)) + (((((-(assign14770_e21948 * locals.var_t1_dn11)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn11)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn13 + ((assign14770_e21930 * locals.var_t1_dn13) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn13)) + (((((-(assign14770_e21948 * locals.var_t1_dn13)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn13)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn14 + ((assign14770_e21930 * locals.var_t1_dn14) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn14)) + (((((-(assign14770_e21948 * locals.var_t1_dn14)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn14)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14770_e21974)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign14770_e21980;
        locals.var_u0_v_dn0 = assign14770_e21980_d_n0;
        locals.var_u0_v_dn2 = assign14770_e21980_d_n2;
        locals.var_u0_v_dn3 = assign14770_e21980_d_n3;
        locals.var_u0_v_dn4 = assign14770_e21980_d_n4;
        locals.var_u0_v_dn5 = assign14770_e21980_d_n5;
        locals.var_u0_v_dn6 = assign14770_e21980_d_n6;
        locals.var_u0_v_dn7 = assign14770_e21980_d_n7;
        locals.var_u0_v_dn8 = assign14770_e21980_d_n8;
        locals.var_u0_v_dn9 = assign14770_e21980_d_n9;
        locals.var_u0_v_dn10 = assign14770_e21980_d_n10;
        locals.var_u0_v_dn11 = assign14770_e21980_d_n11;
        locals.var_u0_v_dn13 = assign14770_e21980_d_n13;
        locals.var_u0_v_dn14 = assign14770_e21980_d_n14;
        locals.var_u0_v_rv = 0.0;

        let assign14780_e21983: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign14780_e21983;
        locals.var_guard245_rv = 0.0;

        let (assign14790_e21994, assign14790_e21994_d_n0, assign14790_e21994_d_n2, assign14790_e21994_d_n3, assign14790_e21994_d_n4, assign14790_e21994_d_n5, assign14790_e21994_d_n6, assign14790_e21994_d_n7, assign14790_e21994_d_n8, assign14790_e21994_d_n9, assign14790_e21994_d_n10, assign14790_e21994_d_n11, assign14790_e21994_d_n13, assign14790_e21994_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14790_e21990: f64 = (locals.var_uter_i * locals.var_trat_ln);
        let assign14790_e21991: f64 = (assign14790_e21990).exp();
        let assign14790_e21992: f64 = (locals.var_u0r_i * assign14790_e21991);
        (assign14790_e21992, (locals.var_u0r_i_dn0 * assign14790_e21991), (locals.var_u0r_i_dn2 * assign14790_e21991), (locals.var_u0r_i_dn3 * assign14790_e21991), ((locals.var_u0r_i_dn4 * assign14790_e21991) + (locals.var_u0r_i * (assign14790_e21991 * (locals.var_uter_i * locals.var_trat_ln_dn4)))), (locals.var_u0r_i_dn5 * assign14790_e21991), (locals.var_u0r_i_dn6 * assign14790_e21991), (locals.var_u0r_i_dn7 * assign14790_e21991), (locals.var_u0r_i_dn8 * assign14790_e21991), (locals.var_u0r_i_dn9 * assign14790_e21991), (locals.var_u0r_i_dn10 * assign14790_e21991), (locals.var_u0r_i_dn11 * assign14790_e21991), (locals.var_u0r_i_dn13 * assign14790_e21991), (locals.var_u0r_i_dn14 * assign14790_e21991),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14790_e21994;
        locals.var_t1_dn0 = assign14790_e21994_d_n0;
        locals.var_t1_dn2 = assign14790_e21994_d_n2;
        locals.var_t1_dn3 = assign14790_e21994_d_n3;
        locals.var_t1_dn4 = assign14790_e21994_d_n4;
        locals.var_t1_dn5 = assign14790_e21994_d_n5;
        locals.var_t1_dn6 = assign14790_e21994_d_n6;
        locals.var_t1_dn7 = assign14790_e21994_d_n7;
        locals.var_t1_dn8 = assign14790_e21994_d_n8;
        locals.var_t1_dn9 = assign14790_e21994_d_n9;
        locals.var_t1_dn10 = assign14790_e21994_d_n10;
        locals.var_t1_dn11 = assign14790_e21994_d_n11;
        locals.var_t1_dn13 = assign14790_e21994_d_n13;
        locals.var_t1_dn14 = assign14790_e21994_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14800_e22050, assign14800_e22050_d_n0, assign14800_e22050_d_n2, assign14800_e22050_d_n3, assign14800_e22050_d_n4, assign14800_e22050_d_n5, assign14800_e22050_d_n6, assign14800_e22050_d_n7, assign14800_e22050_d_n8, assign14800_e22050_d_n9, assign14800_e22050_d_n10, assign14800_e22050_d_n11, assign14800_e22050_d_n13, assign14800_e22050_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14800_e22000: f64 = (-0.9);
        let assign14800_e22002: f64 = (assign14800_e22000 * locals.var_t1);
        let assign14800_e22006: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22008: f64 = (-0.9);
        let assign14800_e22010: f64 = (assign14800_e22008 * locals.var_t1);
        let assign14800_e22011: f64 = (assign14800_e22006 - assign14800_e22010);
        let assign14800_e22013: f64 = (assign14800_e22011 - 0.0001);
        let assign14800_e22016: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22018: f64 = (-0.9);
        let assign14800_e22020: f64 = (assign14800_e22018 * locals.var_t1);
        let assign14800_e22021: f64 = (assign14800_e22016 - assign14800_e22020);
        let assign14800_e22023: f64 = (assign14800_e22021 - 0.0001);
        let assign14800_e22026: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22028: f64 = (-0.9);
        let assign14800_e22030: f64 = (assign14800_e22028 * locals.var_t1);
        let assign14800_e22031: f64 = (assign14800_e22026 - assign14800_e22030);
        let assign14800_e22033: f64 = (assign14800_e22031 - 0.0001);
        let assign14800_e22034: f64 = (assign14800_e22023 * assign14800_e22033);
        let assign14800_e22037: f64 = (-0.9);
        let assign14800_e22039: f64 = (assign14800_e22037 * locals.var_t1);
        let assign14800_e22040: f64 = (4.0 * assign14800_e22039);
        let assign14800_e22042: f64 = (assign14800_e22040 * 0.0001);
        let assign14800_e22043: f64 = (assign14800_e22034 - assign14800_e22042);
        let assign14800_e22044: f64 = (assign14800_e22043).sqrt();
        let assign14800_e22045: f64 = (assign14800_e22013 + assign14800_e22044);
        let assign14800_e22046: f64 = (0.5 * assign14800_e22045);
        let assign14800_e22047: f64 = (assign14800_e22002 + assign14800_e22046);
        let assign14800_e22048: f64 = (locals.var_t1 + assign14800_e22047);
        (assign14800_e22048, (locals.var_t1_dn0 + ((assign14800_e22000 * locals.var_t1_dn0) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn0)) + (((((-(assign14800_e22018 * locals.var_t1_dn0)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn0)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn2 + ((assign14800_e22000 * locals.var_t1_dn2) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn2)) + (((((-(assign14800_e22018 * locals.var_t1_dn2)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn2)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn3 + ((assign14800_e22000 * locals.var_t1_dn3) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn3)) + (((((-(assign14800_e22018 * locals.var_t1_dn3)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn3)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn4 + ((assign14800_e22000 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22008 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22018 * locals.var_t1_dn4)) * assign14800_e22033) + (assign14800_e22023 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22028 * locals.var_t1_dn4)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn5 + ((assign14800_e22000 * locals.var_t1_dn5) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn5)) + (((((-(assign14800_e22018 * locals.var_t1_dn5)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn5)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn6 + ((assign14800_e22000 * locals.var_t1_dn6) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn6)) + (((((-(assign14800_e22018 * locals.var_t1_dn6)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn6)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn7 + ((assign14800_e22000 * locals.var_t1_dn7) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn7)) + (((((-(assign14800_e22018 * locals.var_t1_dn7)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn7)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn8 + ((assign14800_e22000 * locals.var_t1_dn8) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn8)) + (((((-(assign14800_e22018 * locals.var_t1_dn8)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn8)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn9 + ((assign14800_e22000 * locals.var_t1_dn9) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn9)) + (((((-(assign14800_e22018 * locals.var_t1_dn9)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn9)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn10 + ((assign14800_e22000 * locals.var_t1_dn10) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn10)) + (((((-(assign14800_e22018 * locals.var_t1_dn10)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn10)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn11 + ((assign14800_e22000 * locals.var_t1_dn11) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn11)) + (((((-(assign14800_e22018 * locals.var_t1_dn11)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn11)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn13 + ((assign14800_e22000 * locals.var_t1_dn13) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn13)) + (((((-(assign14800_e22018 * locals.var_t1_dn13)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn13)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn14 + ((assign14800_e22000 * locals.var_t1_dn14) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn14)) + (((((-(assign14800_e22018 * locals.var_t1_dn14)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn14)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14800_e22044)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign14800_e22050;
        locals.var_u0r_t_dn0 = assign14800_e22050_d_n0;
        locals.var_u0r_t_dn2 = assign14800_e22050_d_n2;
        locals.var_u0r_t_dn3 = assign14800_e22050_d_n3;
        locals.var_u0r_t_dn4 = assign14800_e22050_d_n4;
        locals.var_u0r_t_dn5 = assign14800_e22050_d_n5;
        locals.var_u0r_t_dn6 = assign14800_e22050_d_n6;
        locals.var_u0r_t_dn7 = assign14800_e22050_d_n7;
        locals.var_u0r_t_dn8 = assign14800_e22050_d_n8;
        locals.var_u0r_t_dn9 = assign14800_e22050_d_n9;
        locals.var_u0r_t_dn10 = assign14800_e22050_d_n10;
        locals.var_u0r_t_dn11 = assign14800_e22050_d_n11;
        locals.var_u0r_t_dn13 = assign14800_e22050_d_n13;
        locals.var_u0r_t_dn14 = assign14800_e22050_d_n14;
        locals.var_u0r_t_rv = 0.0;

        let (assign14810_e22056, assign14810_e22056_d_n0, assign14810_e22056_d_n2, assign14810_e22056_d_n3, assign14810_e22056_d_n4, assign14810_e22056_d_n5, assign14810_e22056_d_n6, assign14810_e22056_d_n7, assign14810_e22056_d_n8, assign14810_e22056_d_n9, assign14810_e22056_d_n10, assign14810_e22056_d_n11, assign14810_e22056_d_n13, assign14810_e22056_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign14810_e22056;
        locals.var_u0r_v_dn0 = assign14810_e22056_d_n0;
        locals.var_u0r_v_dn2 = assign14810_e22056_d_n2;
        locals.var_u0r_v_dn3 = assign14810_e22056_d_n3;
        locals.var_u0r_v_dn4 = assign14810_e22056_d_n4;
        locals.var_u0r_v_dn5 = assign14810_e22056_d_n5;
        locals.var_u0r_v_dn6 = assign14810_e22056_d_n6;
        locals.var_u0r_v_dn7 = assign14810_e22056_d_n7;
        locals.var_u0r_v_dn8 = assign14810_e22056_d_n8;
        locals.var_u0r_v_dn9 = assign14810_e22056_d_n9;
        locals.var_u0r_v_dn10 = assign14810_e22056_d_n10;
        locals.var_u0r_v_dn11 = assign14810_e22056_d_n11;
        locals.var_u0r_v_dn13 = assign14810_e22056_d_n13;
        locals.var_u0r_v_dn14 = assign14810_e22056_d_n14;
        locals.var_u0r_v_rv = 0.0;

        let (assign14820_e22100, assign14820_e22100_d_n0, assign14820_e22100_d_n2, assign14820_e22100_d_n3, assign14820_e22100_d_n4, assign14820_e22100_d_n5, assign14820_e22100_d_n6, assign14820_e22100_d_n7, assign14820_e22100_d_n8, assign14820_e22100_d_n9, assign14820_e22100_d_n10, assign14820_e22100_d_n11, assign14820_e22100_d_n13, assign14820_e22100_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14820_e22060: f64 = (-locals.var_ua_i);
        let assign14820_e22064: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22066: f64 = (-locals.var_ua_i);
        let assign14820_e22067: f64 = (assign14820_e22064 - assign14820_e22066);
        let assign14820_e22069: f64 = (assign14820_e22067 - 1e-6);
        let assign14820_e22072: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22074: f64 = (-locals.var_ua_i);
        let assign14820_e22075: f64 = (assign14820_e22072 - assign14820_e22074);
        let assign14820_e22077: f64 = (assign14820_e22075 - 1e-6);
        let assign14820_e22080: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22082: f64 = (-locals.var_ua_i);
        let assign14820_e22083: f64 = (assign14820_e22080 - assign14820_e22082);
        let assign14820_e22085: f64 = (assign14820_e22083 - 1e-6);
        let assign14820_e22086: f64 = (assign14820_e22077 * assign14820_e22085);
        let assign14820_e22089: f64 = (-locals.var_ua_i);
        let assign14820_e22090: f64 = (4.0 * assign14820_e22089);
        let assign14820_e22092: f64 = (assign14820_e22090 * 1e-6);
        let assign14820_e22093: f64 = (assign14820_e22086 - assign14820_e22092);
        let assign14820_e22094: f64 = (assign14820_e22093).sqrt();
        let assign14820_e22095: f64 = (assign14820_e22069 + assign14820_e22094);
        let assign14820_e22096: f64 = (0.5 * assign14820_e22095);
        let assign14820_e22097: f64 = (assign14820_e22060 + assign14820_e22096);
        let assign14820_e22098: f64 = (locals.var_ua_i + assign14820_e22097);
        (assign14820_e22098, (locals.var_ua_i_dn0 + ((-locals.var_ua_i_dn0) + (0.5 * ((-(-locals.var_ua_i_dn0)) + (((((-(-locals.var_ua_i_dn0)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn0)))) - ((4.0 * (-locals.var_ua_i_dn0)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn2 + ((-locals.var_ua_i_dn2) + (0.5 * ((-(-locals.var_ua_i_dn2)) + (((((-(-locals.var_ua_i_dn2)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn2)))) - ((4.0 * (-locals.var_ua_i_dn2)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn3 + ((-locals.var_ua_i_dn3) + (0.5 * ((-(-locals.var_ua_i_dn3)) + (((((-(-locals.var_ua_i_dn3)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn3)))) - ((4.0 * (-locals.var_ua_i_dn3)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn4 + ((-locals.var_ua_i_dn4) + (0.5 * (((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) + ((((((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) * assign14820_e22085) + (assign14820_e22077 * ((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)))) - ((4.0 * (-locals.var_ua_i_dn4)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn5 + ((-locals.var_ua_i_dn5) + (0.5 * ((-(-locals.var_ua_i_dn5)) + (((((-(-locals.var_ua_i_dn5)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn5)))) - ((4.0 * (-locals.var_ua_i_dn5)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn6 + ((-locals.var_ua_i_dn6) + (0.5 * ((-(-locals.var_ua_i_dn6)) + (((((-(-locals.var_ua_i_dn6)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn6)))) - ((4.0 * (-locals.var_ua_i_dn6)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn7 + ((-locals.var_ua_i_dn7) + (0.5 * ((-(-locals.var_ua_i_dn7)) + (((((-(-locals.var_ua_i_dn7)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn7)))) - ((4.0 * (-locals.var_ua_i_dn7)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn8 + ((-locals.var_ua_i_dn8) + (0.5 * ((-(-locals.var_ua_i_dn8)) + (((((-(-locals.var_ua_i_dn8)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn8)))) - ((4.0 * (-locals.var_ua_i_dn8)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn9 + ((-locals.var_ua_i_dn9) + (0.5 * ((-(-locals.var_ua_i_dn9)) + (((((-(-locals.var_ua_i_dn9)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn9)))) - ((4.0 * (-locals.var_ua_i_dn9)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn10 + ((-locals.var_ua_i_dn10) + (0.5 * ((-(-locals.var_ua_i_dn10)) + (((((-(-locals.var_ua_i_dn10)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn10)))) - ((4.0 * (-locals.var_ua_i_dn10)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn11 + ((-locals.var_ua_i_dn11) + (0.5 * ((-(-locals.var_ua_i_dn11)) + (((((-(-locals.var_ua_i_dn11)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn11)))) - ((4.0 * (-locals.var_ua_i_dn11)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn13 + ((-locals.var_ua_i_dn13) + (0.5 * ((-(-locals.var_ua_i_dn13)) + (((((-(-locals.var_ua_i_dn13)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn13)))) - ((4.0 * (-locals.var_ua_i_dn13)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn14 + ((-locals.var_ua_i_dn14) + (0.5 * ((-(-locals.var_ua_i_dn14)) + (((((-(-locals.var_ua_i_dn14)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn14)))) - ((4.0 * (-locals.var_ua_i_dn14)) * 1e-6)) / (2.0 * assign14820_e22094)))))),)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign14820_e22100;
        locals.var_ua_t_dn0 = assign14820_e22100_d_n0;
        locals.var_ua_t_dn2 = assign14820_e22100_d_n2;
        locals.var_ua_t_dn3 = assign14820_e22100_d_n3;
        locals.var_ua_t_dn4 = assign14820_e22100_d_n4;
        locals.var_ua_t_dn5 = assign14820_e22100_d_n5;
        locals.var_ua_t_dn6 = assign14820_e22100_d_n6;
        locals.var_ua_t_dn7 = assign14820_e22100_d_n7;
        locals.var_ua_t_dn8 = assign14820_e22100_d_n8;
        locals.var_ua_t_dn9 = assign14820_e22100_d_n9;
        locals.var_ua_t_dn10 = assign14820_e22100_d_n10;
        locals.var_ua_t_dn11 = assign14820_e22100_d_n11;
        locals.var_ua_t_dn13 = assign14820_e22100_d_n13;
        locals.var_ua_t_dn14 = assign14820_e22100_d_n14;
        locals.var_ua_t_rv = 0.0;

        let (assign14830_e22104, assign14830_e22104_d_n0, assign14830_e22104_d_n2, assign14830_e22104_d_n3, assign14830_e22104_d_n4, assign14830_e22104_d_n5, assign14830_e22104_d_n6, assign14830_e22104_d_n7, assign14830_e22104_d_n8, assign14830_e22104_d_n9, assign14830_e22104_d_n10, assign14830_e22104_d_n11, assign14830_e22104_d_n13, assign14830_e22104_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign14830_e22104;
        locals.var_eu_t_dn0 = assign14830_e22104_d_n0;
        locals.var_eu_t_dn2 = assign14830_e22104_d_n2;
        locals.var_eu_t_dn3 = assign14830_e22104_d_n3;
        locals.var_eu_t_dn4 = assign14830_e22104_d_n4;
        locals.var_eu_t_dn5 = assign14830_e22104_d_n5;
        locals.var_eu_t_dn6 = assign14830_e22104_d_n6;
        locals.var_eu_t_dn7 = assign14830_e22104_d_n7;
        locals.var_eu_t_dn8 = assign14830_e22104_d_n8;
        locals.var_eu_t_dn9 = assign14830_e22104_d_n9;
        locals.var_eu_t_dn10 = assign14830_e22104_d_n10;
        locals.var_eu_t_dn11 = assign14830_e22104_d_n11;
        locals.var_eu_t_dn13 = assign14830_e22104_d_n13;
        locals.var_eu_t_dn14 = assign14830_e22104_d_n14;
        locals.var_eu_t_rv = 0.0;

        let assign14840_e22107: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign14840_e22107;
        locals.var_guard246_rv = 0.0;

        let (assign14850_e22153, assign14850_e22153_d_n0, assign14850_e22153_d_n2, assign14850_e22153_d_n3, assign14850_e22153_d_n4, assign14850_e22153_d_n5, assign14850_e22153_d_n6, assign14850_e22153_d_n7, assign14850_e22153_d_n8, assign14850_e22153_d_n9, assign14850_e22153_d_n10, assign14850_e22153_d_n11, assign14850_e22153_d_n13, assign14850_e22153_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard246 != 0.0)) {
        let assign14850_e22113: f64 = (-locals.var_uar_i);
        let assign14850_e22117: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22119: f64 = (-locals.var_uar_i);
        let assign14850_e22120: f64 = (assign14850_e22117 - assign14850_e22119);
        let assign14850_e22122: f64 = (assign14850_e22120 - 1e-6);
        let assign14850_e22125: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22127: f64 = (-locals.var_uar_i);
        let assign14850_e22128: f64 = (assign14850_e22125 - assign14850_e22127);
        let assign14850_e22130: f64 = (assign14850_e22128 - 1e-6);
        let assign14850_e22133: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22135: f64 = (-locals.var_uar_i);
        let assign14850_e22136: f64 = (assign14850_e22133 - assign14850_e22135);
        let assign14850_e22138: f64 = (assign14850_e22136 - 1e-6);
        let assign14850_e22139: f64 = (assign14850_e22130 * assign14850_e22138);
        let assign14850_e22142: f64 = (-locals.var_uar_i);
        let assign14850_e22143: f64 = (4.0 * assign14850_e22142);
        let assign14850_e22145: f64 = (assign14850_e22143 * 1e-6);
        let assign14850_e22146: f64 = (assign14850_e22139 - assign14850_e22145);
        let assign14850_e22147: f64 = (assign14850_e22146).sqrt();
        let assign14850_e22148: f64 = (assign14850_e22122 + assign14850_e22147);
        let assign14850_e22149: f64 = (0.5 * assign14850_e22148);
        let assign14850_e22150: f64 = (assign14850_e22113 + assign14850_e22149);
        let assign14850_e22151: f64 = (locals.var_uar_i + assign14850_e22150);
        (assign14850_e22151, (locals.var_uar_i_dn0 + ((-locals.var_uar_i_dn0) + (0.5 * ((-(-locals.var_uar_i_dn0)) + (((((-(-locals.var_uar_i_dn0)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn0)))) - ((4.0 * (-locals.var_uar_i_dn0)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn2 + ((-locals.var_uar_i_dn2) + (0.5 * ((-(-locals.var_uar_i_dn2)) + (((((-(-locals.var_uar_i_dn2)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn2)))) - ((4.0 * (-locals.var_uar_i_dn2)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn3 + ((-locals.var_uar_i_dn3) + (0.5 * ((-(-locals.var_uar_i_dn3)) + (((((-(-locals.var_uar_i_dn3)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn3)))) - ((4.0 * (-locals.var_uar_i_dn3)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn4 + ((-locals.var_uar_i_dn4) + (0.5 * (((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) + ((((((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) * assign14850_e22138) + (assign14850_e22130 * ((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)))) - ((4.0 * (-locals.var_uar_i_dn4)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn5 + ((-locals.var_uar_i_dn5) + (0.5 * ((-(-locals.var_uar_i_dn5)) + (((((-(-locals.var_uar_i_dn5)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn5)))) - ((4.0 * (-locals.var_uar_i_dn5)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn6 + ((-locals.var_uar_i_dn6) + (0.5 * ((-(-locals.var_uar_i_dn6)) + (((((-(-locals.var_uar_i_dn6)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn6)))) - ((4.0 * (-locals.var_uar_i_dn6)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn7 + ((-locals.var_uar_i_dn7) + (0.5 * ((-(-locals.var_uar_i_dn7)) + (((((-(-locals.var_uar_i_dn7)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn7)))) - ((4.0 * (-locals.var_uar_i_dn7)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn8 + ((-locals.var_uar_i_dn8) + (0.5 * ((-(-locals.var_uar_i_dn8)) + (((((-(-locals.var_uar_i_dn8)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn8)))) - ((4.0 * (-locals.var_uar_i_dn8)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn9 + ((-locals.var_uar_i_dn9) + (0.5 * ((-(-locals.var_uar_i_dn9)) + (((((-(-locals.var_uar_i_dn9)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn9)))) - ((4.0 * (-locals.var_uar_i_dn9)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn10 + ((-locals.var_uar_i_dn10) + (0.5 * ((-(-locals.var_uar_i_dn10)) + (((((-(-locals.var_uar_i_dn10)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn10)))) - ((4.0 * (-locals.var_uar_i_dn10)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn11 + ((-locals.var_uar_i_dn11) + (0.5 * ((-(-locals.var_uar_i_dn11)) + (((((-(-locals.var_uar_i_dn11)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn11)))) - ((4.0 * (-locals.var_uar_i_dn11)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn13 + ((-locals.var_uar_i_dn13) + (0.5 * ((-(-locals.var_uar_i_dn13)) + (((((-(-locals.var_uar_i_dn13)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn13)))) - ((4.0 * (-locals.var_uar_i_dn13)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn14 + ((-locals.var_uar_i_dn14) + (0.5 * ((-(-locals.var_uar_i_dn14)) + (((((-(-locals.var_uar_i_dn14)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn14)))) - ((4.0 * (-locals.var_uar_i_dn14)) * 1e-6)) / (2.0 * assign14850_e22147)))))),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign14850_e22153;
        locals.var_uar_t_dn0 = assign14850_e22153_d_n0;
        locals.var_uar_t_dn2 = assign14850_e22153_d_n2;
        locals.var_uar_t_dn3 = assign14850_e22153_d_n3;
        locals.var_uar_t_dn4 = assign14850_e22153_d_n4;
        locals.var_uar_t_dn5 = assign14850_e22153_d_n5;
        locals.var_uar_t_dn6 = assign14850_e22153_d_n6;
        locals.var_uar_t_dn7 = assign14850_e22153_d_n7;
        locals.var_uar_t_dn8 = assign14850_e22153_d_n8;
        locals.var_uar_t_dn9 = assign14850_e22153_d_n9;
        locals.var_uar_t_dn10 = assign14850_e22153_d_n10;
        locals.var_uar_t_dn11 = assign14850_e22153_d_n11;
        locals.var_uar_t_dn13 = assign14850_e22153_d_n13;
        locals.var_uar_t_dn14 = assign14850_e22153_d_n14;
        locals.var_uar_t_rv = 0.0;

        let (assign14860_e22162, assign14860_e22162_d_n0, assign14860_e22162_d_n2, assign14860_e22162_d_n3, assign14860_e22162_d_n4, assign14860_e22162_d_n5, assign14860_e22162_d_n6, assign14860_e22162_d_n7, assign14860_e22162_d_n8, assign14860_e22162_d_n9, assign14860_e22162_d_n10, assign14860_e22162_d_n11, assign14860_e22162_d_n13, assign14860_e22162_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14860_e22158: f64 = (locals.var_ud1_i * locals.var_trat_ln);
        let assign14860_e22159: f64 = (assign14860_e22158).exp();
        let assign14860_e22160: f64 = (locals.var_ud_i * assign14860_e22159);
        (assign14860_e22160, (locals.var_ud_i_dn0 * assign14860_e22159), (locals.var_ud_i_dn2 * assign14860_e22159), (locals.var_ud_i_dn3 * assign14860_e22159), ((locals.var_ud_i_dn4 * assign14860_e22159) + (locals.var_ud_i * (assign14860_e22159 * (locals.var_ud1_i * locals.var_trat_ln_dn4)))), (locals.var_ud_i_dn5 * assign14860_e22159), (locals.var_ud_i_dn6 * assign14860_e22159), (locals.var_ud_i_dn7 * assign14860_e22159), (locals.var_ud_i_dn8 * assign14860_e22159), (locals.var_ud_i_dn9 * assign14860_e22159), (locals.var_ud_i_dn10 * assign14860_e22159), (locals.var_ud_i_dn11 * assign14860_e22159), (locals.var_ud_i_dn13 * assign14860_e22159), (locals.var_ud_i_dn14 * assign14860_e22159),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign14860_e22162;
        locals.var_ud_t_dn0 = assign14860_e22162_d_n0;
        locals.var_ud_t_dn2 = assign14860_e22162_d_n2;
        locals.var_ud_t_dn3 = assign14860_e22162_d_n3;
        locals.var_ud_t_dn4 = assign14860_e22162_d_n4;
        locals.var_ud_t_dn5 = assign14860_e22162_d_n5;
        locals.var_ud_t_dn6 = assign14860_e22162_d_n6;
        locals.var_ud_t_dn7 = assign14860_e22162_d_n7;
        locals.var_ud_t_dn8 = assign14860_e22162_d_n8;
        locals.var_ud_t_dn9 = assign14860_e22162_d_n9;
        locals.var_ud_t_dn10 = assign14860_e22162_d_n10;
        locals.var_ud_t_dn11 = assign14860_e22162_d_n11;
        locals.var_ud_t_dn13 = assign14860_e22162_d_n13;
        locals.var_ud_t_dn14 = assign14860_e22162_d_n14;
        locals.var_ud_t_rv = 0.0;

        let assign14870_e22165: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign14870_e22165;
        locals.var_guard247_rv = 0.0;

        let (assign14880_e22176, assign14880_e22176_d_n0, assign14880_e22176_d_n2, assign14880_e22176_d_n3, assign14880_e22176_d_n4, assign14880_e22176_d_n5, assign14880_e22176_d_n6, assign14880_e22176_d_n7, assign14880_e22176_d_n8, assign14880_e22176_d_n9, assign14880_e22176_d_n10, assign14880_e22176_d_n11, assign14880_e22176_d_n13, assign14880_e22176_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard247 != 0.0)) {
        let assign14880_e22172: f64 = (locals.var_ud1r_i * locals.var_trat_ln);
        let assign14880_e22173: f64 = (assign14880_e22172).exp();
        let assign14880_e22174: f64 = (locals.var_udr_i * assign14880_e22173);
        (assign14880_e22174, (locals.var_udr_i_dn0 * assign14880_e22173), (locals.var_udr_i_dn2 * assign14880_e22173), (locals.var_udr_i_dn3 * assign14880_e22173), ((locals.var_udr_i_dn4 * assign14880_e22173) + (locals.var_udr_i * (assign14880_e22173 * (locals.var_ud1r_i * locals.var_trat_ln_dn4)))), (locals.var_udr_i_dn5 * assign14880_e22173), (locals.var_udr_i_dn6 * assign14880_e22173), (locals.var_udr_i_dn7 * assign14880_e22173), (locals.var_udr_i_dn8 * assign14880_e22173), (locals.var_udr_i_dn9 * assign14880_e22173), (locals.var_udr_i_dn10 * assign14880_e22173), (locals.var_udr_i_dn11 * assign14880_e22173), (locals.var_udr_i_dn13 * assign14880_e22173), (locals.var_udr_i_dn14 * assign14880_e22173),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign14880_e22176;
        locals.var_udr_t_dn0 = assign14880_e22176_d_n0;
        locals.var_udr_t_dn2 = assign14880_e22176_d_n2;
        locals.var_udr_t_dn3 = assign14880_e22176_d_n3;
        locals.var_udr_t_dn4 = assign14880_e22176_d_n4;
        locals.var_udr_t_dn5 = assign14880_e22176_d_n5;
        locals.var_udr_t_dn6 = assign14880_e22176_d_n6;
        locals.var_udr_t_dn7 = assign14880_e22176_d_n7;
        locals.var_udr_t_dn8 = assign14880_e22176_d_n8;
        locals.var_udr_t_dn9 = assign14880_e22176_d_n9;
        locals.var_udr_t_dn10 = assign14880_e22176_d_n10;
        locals.var_udr_t_dn11 = assign14880_e22176_d_n11;
        locals.var_udr_t_dn13 = assign14880_e22176_d_n13;
        locals.var_udr_t_dn14 = assign14880_e22176_d_n14;
        locals.var_udr_t_rv = 0.0;

        let (assign14890_e22185, assign14890_e22185_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14890_e22181: f64 = (locals.var_ucste_i * locals.var_trat_ln);
        let assign14890_e22182: f64 = (assign14890_e22181).exp();
        let assign14890_e22183: f64 = (locals.var_ucs_i * assign14890_e22182);
        (assign14890_e22183, (locals.var_ucs_i * (assign14890_e22182 * (locals.var_ucste_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign14890_e22185;
        locals.var_ucs_t_dn4 = assign14890_e22185_d_n4;
        locals.var_ucs_t_rv = 0.0;

        let (assign14900_e22260, assign14900_e22260_d_n0, assign14900_e22260_d_n2, assign14900_e22260_d_n3, assign14900_e22260_d_n4, assign14900_e22260_d_n5, assign14900_e22260_d_n6, assign14900_e22260_d_n7, assign14900_e22260_d_n8, assign14900_e22260_d_n9, assign14900_e22260_d_n10, assign14900_e22260_d_n11, assign14900_e22260_d_n13, assign14900_e22260_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14900_e22190: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign14900_e22191: f64 = (1.0 + assign14900_e22190);
        let assign14900_e22193: f64 = (assign14900_e22191 - 1e-6);
        let assign14900_e22195: f64 = (-10000.0);
        let assign14900_e22197: f64 = (assign14900_e22195 * 0.001);
        let (assign14900_e22258, assign14900_e22258_d_n4,) = {
            if (!(assign14900_e22193 < assign14900_e22197)) {
                let assign14900_e22204: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22205: f64 = (1.0 + assign14900_e22204);
                let assign14900_e22207: f64 = (assign14900_e22205 - 1e-6);
                let assign14900_e22211: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22212: f64 = (1.0 + assign14900_e22211);
                let assign14900_e22214: f64 = (assign14900_e22212 - 1e-6);
                let assign14900_e22218: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22219: f64 = (1.0 + assign14900_e22218);
                let assign14900_e22221: f64 = (assign14900_e22219 - 1e-6);
                let assign14900_e22222: f64 = (assign14900_e22214 * assign14900_e22221);
                let assign14900_e22225: f64 = (4.0 * 0.001);
                let assign14900_e22227: f64 = (assign14900_e22225 * 0.001);
                let assign14900_e22228: f64 = (assign14900_e22222 + assign14900_e22227);
                let assign14900_e22229: f64 = (assign14900_e22228).sqrt();
                let assign14900_e22230: f64 = (assign14900_e22207 + assign14900_e22229);
                let assign14900_e22231: f64 = (0.5 * assign14900_e22230);
                (assign14900_e22231, (0.5 * ((locals.var_prt_i * locals.var_deltemp_dn4) + ((((locals.var_prt_i * locals.var_deltemp_dn4) * assign14900_e22221) + (assign14900_e22214 * (locals.var_prt_i * locals.var_deltemp_dn4))) / (2.0 * assign14900_e22229)))),)
            } else {
                let assign14900_e22235: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22236: f64 = (1.0 + assign14900_e22235);
                let assign14900_e22238: f64 = (assign14900_e22236 - 1e-6);
                let assign14900_e22240: f64 = (-10000.0);
                let assign14900_e22242: f64 = (assign14900_e22240 * 0.001);
                let (assign14900_e22257, assign14900_e22257_d_n4,) = {
                    if (assign14900_e22238 < assign14900_e22242) {
                        let assign14900_e22245: f64 = (-0.001);
                        let assign14900_e22247: f64 = (assign14900_e22245 * 0.001);
                        let assign14900_e22251: f64 = (locals.var_prt_i * locals.var_deltemp);
                        let assign14900_e22252: f64 = (1.0 + assign14900_e22251);
                        let assign14900_e22254: f64 = (assign14900_e22252 - 1e-6);
                        let assign14900_e22255: f64 = (assign14900_e22247 / assign14900_e22254);
                        (assign14900_e22255, (-((assign14900_e22247 * (locals.var_prt_i * locals.var_deltemp_dn4)) / (assign14900_e22254 * assign14900_e22254))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14900_e22257, assign14900_e22257_d_n4,)
            }
        };
        (assign14900_e22258, 0.0, 0.0, 0.0, assign14900_e22258_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign14900_e22260;
        locals.var_rdstemp_dn0 = assign14900_e22260_d_n0;
        locals.var_rdstemp_dn2 = assign14900_e22260_d_n2;
        locals.var_rdstemp_dn3 = assign14900_e22260_d_n3;
        locals.var_rdstemp_dn4 = assign14900_e22260_d_n4;
        locals.var_rdstemp_dn5 = assign14900_e22260_d_n5;
        locals.var_rdstemp_dn6 = assign14900_e22260_d_n6;
        locals.var_rdstemp_dn7 = assign14900_e22260_d_n7;
        locals.var_rdstemp_dn8 = assign14900_e22260_d_n8;
        locals.var_rdstemp_dn9 = assign14900_e22260_d_n9;
        locals.var_rdstemp_dn10 = assign14900_e22260_d_n10;
        locals.var_rdstemp_dn11 = assign14900_e22260_d_n11;
        locals.var_rdstemp_dn13 = assign14900_e22260_d_n13;
        locals.var_rdstemp_dn14 = assign14900_e22260_d_n14;
        locals.var_rdstemp_rv = 0.0;

        let assign14910_e22263: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign14910_e22263;
        locals.var_guard248_rv = 0.0;

        let (assign14920_e22312, assign14920_e22312_d_n0, assign14920_e22312_d_n2, assign14920_e22312_d_n3, assign14920_e22312_d_n4, assign14920_e22312_d_n5, assign14920_e22312_d_n6, assign14920_e22312_d_n7, assign14920_e22312_d_n8, assign14920_e22312_d_n9, assign14920_e22312_d_n10, assign14920_e22312_d_n11, assign14920_e22312_d_n13, assign14920_e22312_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 != 0.0)) {
        let assign14920_e22269: f64 = (-locals.var_vsat_i);
        let assign14920_e22272: f64 = (-locals.var_at_i);
        let assign14920_e22274: f64 = (assign14920_e22272 * locals.var_deltemp);
        let assign14920_e22276: f64 = (-locals.var_vsat_i);
        let assign14920_e22277: f64 = (assign14920_e22274 - assign14920_e22276);
        let assign14920_e22279: f64 = (assign14920_e22277 - 1e-6);
        let assign14920_e22281: f64 = (-locals.var_at_i);
        let assign14920_e22283: f64 = (assign14920_e22281 * locals.var_deltemp);
        let assign14920_e22285: f64 = (-locals.var_vsat_i);
        let assign14920_e22286: f64 = (assign14920_e22283 - assign14920_e22285);
        let assign14920_e22288: f64 = (assign14920_e22286 - 1e-6);
        let assign14920_e22290: f64 = (-locals.var_at_i);
        let assign14920_e22292: f64 = (assign14920_e22290 * locals.var_deltemp);
        let assign14920_e22294: f64 = (-locals.var_vsat_i);
        let assign14920_e22295: f64 = (assign14920_e22292 - assign14920_e22294);
        let assign14920_e22297: f64 = (assign14920_e22295 - 1e-6);
        let assign14920_e22298: f64 = (assign14920_e22288 * assign14920_e22297);
        let assign14920_e22301: f64 = (-locals.var_vsat_i);
        let assign14920_e22302: f64 = (4.0 * assign14920_e22301);
        let assign14920_e22304: f64 = (assign14920_e22302 * 1e-6);
        let assign14920_e22305: f64 = (assign14920_e22298 - assign14920_e22304);
        let assign14920_e22306: f64 = (assign14920_e22305).sqrt();
        let assign14920_e22307: f64 = (assign14920_e22279 + assign14920_e22306);
        let assign14920_e22308: f64 = (0.5 * assign14920_e22307);
        let assign14920_e22309: f64 = (assign14920_e22269 + assign14920_e22308);
        let assign14920_e22310: f64 = (locals.var_vsat_i + assign14920_e22309);
        (assign14920_e22310, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * (((assign14920_e22272 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) + ((((((assign14920_e22281 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) * assign14920_e22297) + (assign14920_e22288 * ((assign14920_e22290 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign14920_e22306)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14920_e22312;
        locals.var_vsat_t_dn0 = assign14920_e22312_d_n0;
        locals.var_vsat_t_dn2 = assign14920_e22312_d_n2;
        locals.var_vsat_t_dn3 = assign14920_e22312_d_n3;
        locals.var_vsat_t_dn4 = assign14920_e22312_d_n4;
        locals.var_vsat_t_dn5 = assign14920_e22312_d_n5;
        locals.var_vsat_t_dn6 = assign14920_e22312_d_n6;
        locals.var_vsat_t_dn7 = assign14920_e22312_d_n7;
        locals.var_vsat_t_dn8 = assign14920_e22312_d_n8;
        locals.var_vsat_t_dn9 = assign14920_e22312_d_n9;
        locals.var_vsat_t_dn10 = assign14920_e22312_d_n10;
        locals.var_vsat_t_dn11 = assign14920_e22312_d_n11;
        locals.var_vsat_t_dn13 = assign14920_e22312_d_n13;
        locals.var_vsat_t_dn14 = assign14920_e22312_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let (assign14930_e22398, assign14930_e22398_d_n0, assign14930_e22398_d_n2, assign14930_e22398_d_n3, assign14930_e22398_d_n4, assign14930_e22398_d_n5, assign14930_e22398_d_n6, assign14930_e22398_d_n7, assign14930_e22398_d_n8, assign14930_e22398_d_n9, assign14930_e22398_d_n10, assign14930_e22398_d_n11, assign14930_e22398_d_n13, assign14930_e22398_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 == 0.0)) {
        let assign14930_e22320: f64 = (-locals.var_at_i);
        let assign14930_e22322: f64 = (assign14930_e22320 * locals.var_deltemp);
        let assign14930_e22323: f64 = (1.0 + assign14930_e22322);
        let assign14930_e22325: f64 = (assign14930_e22323 - 1e-6);
        let assign14930_e22327: f64 = (-10000.0);
        let assign14930_e22329: f64 = (assign14930_e22327 * 0.001);
        let (assign14930_e22395, assign14930_e22395_d_n4,) = {
            if (!(assign14930_e22325 < assign14930_e22329)) {
                let assign14930_e22335: f64 = (-locals.var_at_i);
                let assign14930_e22337: f64 = (assign14930_e22335 * locals.var_deltemp);
                let assign14930_e22338: f64 = (1.0 + assign14930_e22337);
                let assign14930_e22340: f64 = (assign14930_e22338 - 1e-6);
                let assign14930_e22343: f64 = (-locals.var_at_i);
                let assign14930_e22345: f64 = (assign14930_e22343 * locals.var_deltemp);
                let assign14930_e22346: f64 = (1.0 + assign14930_e22345);
                let assign14930_e22348: f64 = (assign14930_e22346 - 1e-6);
                let assign14930_e22351: f64 = (-locals.var_at_i);
                let assign14930_e22353: f64 = (assign14930_e22351 * locals.var_deltemp);
                let assign14930_e22354: f64 = (1.0 + assign14930_e22353);
                let assign14930_e22356: f64 = (assign14930_e22354 - 1e-6);
                let assign14930_e22357: f64 = (assign14930_e22348 * assign14930_e22356);
                let assign14930_e22360: f64 = (4.0 * 0.001);
                let assign14930_e22362: f64 = (assign14930_e22360 * 0.001);
                let assign14930_e22363: f64 = (assign14930_e22357 + assign14930_e22362);
                let assign14930_e22364: f64 = (assign14930_e22363).sqrt();
                let assign14930_e22365: f64 = (assign14930_e22340 + assign14930_e22364);
                let assign14930_e22366: f64 = (0.5 * assign14930_e22365);
                (assign14930_e22366, (0.5 * ((assign14930_e22335 * locals.var_deltemp_dn4) + ((((assign14930_e22343 * locals.var_deltemp_dn4) * assign14930_e22356) + (assign14930_e22348 * (assign14930_e22351 * locals.var_deltemp_dn4))) / (2.0 * assign14930_e22364)))),)
            } else {
                let assign14930_e22369: f64 = (-locals.var_at_i);
                let assign14930_e22371: f64 = (assign14930_e22369 * locals.var_deltemp);
                let assign14930_e22372: f64 = (1.0 + assign14930_e22371);
                let assign14930_e22374: f64 = (assign14930_e22372 - 1e-6);
                let assign14930_e22376: f64 = (-10000.0);
                let assign14930_e22378: f64 = (assign14930_e22376 * 0.001);
                let (assign14930_e22394, assign14930_e22394_d_n4,) = {
                    if (assign14930_e22374 < assign14930_e22378) {
                        let assign14930_e22381: f64 = (-0.001);
                        let assign14930_e22383: f64 = (assign14930_e22381 * 0.001);
                        let assign14930_e22386: f64 = (-locals.var_at_i);
                        let assign14930_e22388: f64 = (assign14930_e22386 * locals.var_deltemp);
                        let assign14930_e22389: f64 = (1.0 + assign14930_e22388);
                        let assign14930_e22391: f64 = (assign14930_e22389 - 1e-6);
                        let assign14930_e22392: f64 = (assign14930_e22383 / assign14930_e22391);
                        (assign14930_e22392, (-((assign14930_e22383 * (assign14930_e22386 * locals.var_deltemp_dn4)) / (assign14930_e22391 * assign14930_e22391))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14930_e22394, assign14930_e22394_d_n4,)
            }
        };
        let assign14930_e22396: f64 = (locals.var_vsat_i * assign14930_e22395);
        (assign14930_e22396, (locals.var_vsat_i_dn0 * assign14930_e22395), (locals.var_vsat_i_dn2 * assign14930_e22395), (locals.var_vsat_i_dn3 * assign14930_e22395), ((locals.var_vsat_i_dn4 * assign14930_e22395) + (locals.var_vsat_i * assign14930_e22395_d_n4)), (locals.var_vsat_i_dn5 * assign14930_e22395), (locals.var_vsat_i_dn6 * assign14930_e22395), (locals.var_vsat_i_dn7 * assign14930_e22395), (locals.var_vsat_i_dn8 * assign14930_e22395), (locals.var_vsat_i_dn9 * assign14930_e22395), (locals.var_vsat_i_dn10 * assign14930_e22395), (locals.var_vsat_i_dn11 * assign14930_e22395), (locals.var_vsat_i_dn13 * assign14930_e22395), (locals.var_vsat_i_dn14 * assign14930_e22395),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14930_e22398;
        locals.var_vsat_t_dn0 = assign14930_e22398_d_n0;
        locals.var_vsat_t_dn2 = assign14930_e22398_d_n2;
        locals.var_vsat_t_dn3 = assign14930_e22398_d_n3;
        locals.var_vsat_t_dn4 = assign14930_e22398_d_n4;
        locals.var_vsat_t_dn5 = assign14930_e22398_d_n5;
        locals.var_vsat_t_dn6 = assign14930_e22398_d_n6;
        locals.var_vsat_t_dn7 = assign14930_e22398_d_n7;
        locals.var_vsat_t_dn8 = assign14930_e22398_d_n8;
        locals.var_vsat_t_dn9 = assign14930_e22398_d_n9;
        locals.var_vsat_t_dn10 = assign14930_e22398_d_n10;
        locals.var_vsat_t_dn11 = assign14930_e22398_d_n11;
        locals.var_vsat_t_dn13 = assign14930_e22398_d_n13;
        locals.var_vsat_t_dn14 = assign14930_e22398_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let assign14940_e22401: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign14940_e22401;
        locals.var_guard249_rv = 0.0;

        let assign14950_e22404: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign14950_e22404;
        locals.var_guard250_rv = 0.0;

        let (assign14960_e22455, assign14960_e22455_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign14960_e22412: f64 = (-locals.var_vsatr_i);
        let assign14960_e22415: f64 = (-locals.var_atr_i);
        let assign14960_e22417: f64 = (assign14960_e22415 * locals.var_deltemp);
        let assign14960_e22419: f64 = (-locals.var_vsatr_i);
        let assign14960_e22420: f64 = (assign14960_e22417 - assign14960_e22419);
        let assign14960_e22422: f64 = (assign14960_e22420 - 1e-6);
        let assign14960_e22424: f64 = (-locals.var_atr_i);
        let assign14960_e22426: f64 = (assign14960_e22424 * locals.var_deltemp);
        let assign14960_e22428: f64 = (-locals.var_vsatr_i);
        let assign14960_e22429: f64 = (assign14960_e22426 - assign14960_e22428);
        let assign14960_e22431: f64 = (assign14960_e22429 - 1e-6);
        let assign14960_e22433: f64 = (-locals.var_atr_i);
        let assign14960_e22435: f64 = (assign14960_e22433 * locals.var_deltemp);
        let assign14960_e22437: f64 = (-locals.var_vsatr_i);
        let assign14960_e22438: f64 = (assign14960_e22435 - assign14960_e22437);
        let assign14960_e22440: f64 = (assign14960_e22438 - 1e-6);
        let assign14960_e22441: f64 = (assign14960_e22431 * assign14960_e22440);
        let assign14960_e22444: f64 = (-locals.var_vsatr_i);
        let assign14960_e22445: f64 = (4.0 * assign14960_e22444);
        let assign14960_e22447: f64 = (assign14960_e22445 * 1e-6);
        let assign14960_e22448: f64 = (assign14960_e22441 - assign14960_e22447);
        let assign14960_e22449: f64 = (assign14960_e22448).sqrt();
        let assign14960_e22450: f64 = (assign14960_e22422 + assign14960_e22449);
        let assign14960_e22451: f64 = (0.5 * assign14960_e22450);
        let assign14960_e22452: f64 = (assign14960_e22412 + assign14960_e22451);
        let assign14960_e22453: f64 = (locals.var_vsatr_i + assign14960_e22452);
        (assign14960_e22453, (0.5 * ((assign14960_e22415 * locals.var_deltemp_dn4) + ((((assign14960_e22424 * locals.var_deltemp_dn4) * assign14960_e22440) + (assign14960_e22431 * (assign14960_e22433 * locals.var_deltemp_dn4))) / (2.0 * assign14960_e22449)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14960_e22455;
        locals.var_vsatr_t_dn4 = assign14960_e22455_d_n4;
        locals.var_vsatr_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14970_e22543, assign14970_e22543_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign14970_e22465: f64 = (-locals.var_atr_i);
        let assign14970_e22467: f64 = (assign14970_e22465 * locals.var_deltemp);
        let assign14970_e22468: f64 = (1.0 + assign14970_e22467);
        let assign14970_e22470: f64 = (assign14970_e22468 - 1e-6);
        let assign14970_e22472: f64 = (-10000.0);
        let assign14970_e22474: f64 = (assign14970_e22472 * 0.001);
        let (assign14970_e22540, assign14970_e22540_d_n4,) = {
            if (!(assign14970_e22470 < assign14970_e22474)) {
                let assign14970_e22480: f64 = (-locals.var_atr_i);
                let assign14970_e22482: f64 = (assign14970_e22480 * locals.var_deltemp);
                let assign14970_e22483: f64 = (1.0 + assign14970_e22482);
                let assign14970_e22485: f64 = (assign14970_e22483 - 1e-6);
                let assign14970_e22488: f64 = (-locals.var_atr_i);
                let assign14970_e22490: f64 = (assign14970_e22488 * locals.var_deltemp);
                let assign14970_e22491: f64 = (1.0 + assign14970_e22490);
                let assign14970_e22493: f64 = (assign14970_e22491 - 1e-6);
                let assign14970_e22496: f64 = (-locals.var_atr_i);
                let assign14970_e22498: f64 = (assign14970_e22496 * locals.var_deltemp);
                let assign14970_e22499: f64 = (1.0 + assign14970_e22498);
                let assign14970_e22501: f64 = (assign14970_e22499 - 1e-6);
                let assign14970_e22502: f64 = (assign14970_e22493 * assign14970_e22501);
                let assign14970_e22505: f64 = (4.0 * 0.001);
                let assign14970_e22507: f64 = (assign14970_e22505 * 0.001);
                let assign14970_e22508: f64 = (assign14970_e22502 + assign14970_e22507);
                let assign14970_e22509: f64 = (assign14970_e22508).sqrt();
                let assign14970_e22510: f64 = (assign14970_e22485 + assign14970_e22509);
                let assign14970_e22511: f64 = (0.5 * assign14970_e22510);
                (assign14970_e22511, (0.5 * ((assign14970_e22480 * locals.var_deltemp_dn4) + ((((assign14970_e22488 * locals.var_deltemp_dn4) * assign14970_e22501) + (assign14970_e22493 * (assign14970_e22496 * locals.var_deltemp_dn4))) / (2.0 * assign14970_e22509)))),)
            } else {
                let assign14970_e22514: f64 = (-locals.var_atr_i);
                let assign14970_e22516: f64 = (assign14970_e22514 * locals.var_deltemp);
                let assign14970_e22517: f64 = (1.0 + assign14970_e22516);
                let assign14970_e22519: f64 = (assign14970_e22517 - 1e-6);
                let assign14970_e22521: f64 = (-10000.0);
                let assign14970_e22523: f64 = (assign14970_e22521 * 0.001);
                let (assign14970_e22539, assign14970_e22539_d_n4,) = {
                    if (assign14970_e22519 < assign14970_e22523) {
                        let assign14970_e22526: f64 = (-0.001);
                        let assign14970_e22528: f64 = (assign14970_e22526 * 0.001);
                        let assign14970_e22531: f64 = (-locals.var_atr_i);
                        let assign14970_e22533: f64 = (assign14970_e22531 * locals.var_deltemp);
                        let assign14970_e22534: f64 = (1.0 + assign14970_e22533);
                        let assign14970_e22536: f64 = (assign14970_e22534 - 1e-6);
                        let assign14970_e22537: f64 = (assign14970_e22528 / assign14970_e22536);
                        (assign14970_e22537, (-((assign14970_e22528 * (assign14970_e22531 * locals.var_deltemp_dn4)) / (assign14970_e22536 * assign14970_e22536))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14970_e22539, assign14970_e22539_d_n4,)
            }
        };
        let assign14970_e22541: f64 = (locals.var_vsatr_i * assign14970_e22540);
        (assign14970_e22541, (locals.var_vsatr_i * assign14970_e22540_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14970_e22543;
        locals.var_vsatr_t_dn4 = assign14970_e22543_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign14980_e22546: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign14980_e22546;
        locals.var_guard251_rv = 0.0;

        let (assign14990_e22554, assign14990_e22554_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard251 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14990_e22554;
        locals.var_vsatr_t_dn4 = assign14990_e22554_d_n4;
        locals.var_vsatr_t_rv = 0.0;

        let assign15000_e22557: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign15000_e22557;
        locals.var_guard252_rv = 0.0;

        let (assign15010_e22568, assign15010_e22568_d_n0, assign15010_e22568_d_n2, assign15010_e22568_d_n3, assign15010_e22568_d_n4, assign15010_e22568_d_n5, assign15010_e22568_d_n6, assign15010_e22568_d_n7, assign15010_e22568_d_n8, assign15010_e22568_d_n9, assign15010_e22568_d_n10, assign15010_e22568_d_n11, assign15010_e22568_d_n13, assign15010_e22568_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15010_e22564: f64 = (locals.var_utecv_i * locals.var_trat_ln);
        let assign15010_e22565: f64 = (assign15010_e22564).exp();
        let assign15010_e22566: f64 = (locals.var_u0cv_i * assign15010_e22565);
        (assign15010_e22566, (locals.var_u0cv_i_dn0 * assign15010_e22565), (locals.var_u0cv_i_dn2 * assign15010_e22565), (locals.var_u0cv_i_dn3 * assign15010_e22565), ((locals.var_u0cv_i_dn4 * assign15010_e22565) + (locals.var_u0cv_i * (assign15010_e22565 * (locals.var_utecv_i * locals.var_trat_ln_dn4)))), (locals.var_u0cv_i_dn5 * assign15010_e22565), (locals.var_u0cv_i_dn6 * assign15010_e22565), (locals.var_u0cv_i_dn7 * assign15010_e22565), (locals.var_u0cv_i_dn8 * assign15010_e22565), (locals.var_u0cv_i_dn9 * assign15010_e22565), (locals.var_u0cv_i_dn10 * assign15010_e22565), (locals.var_u0cv_i_dn11 * assign15010_e22565), (locals.var_u0cv_i_dn13 * assign15010_e22565), (locals.var_u0cv_i_dn14 * assign15010_e22565),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15010_e22568;
        locals.var_t1_dn0 = assign15010_e22568_d_n0;
        locals.var_t1_dn2 = assign15010_e22568_d_n2;
        locals.var_t1_dn3 = assign15010_e22568_d_n3;
        locals.var_t1_dn4 = assign15010_e22568_d_n4;
        locals.var_t1_dn5 = assign15010_e22568_d_n5;
        locals.var_t1_dn6 = assign15010_e22568_d_n6;
        locals.var_t1_dn7 = assign15010_e22568_d_n7;
        locals.var_t1_dn8 = assign15010_e22568_d_n8;
        locals.var_t1_dn9 = assign15010_e22568_d_n9;
        locals.var_t1_dn10 = assign15010_e22568_d_n10;
        locals.var_t1_dn11 = assign15010_e22568_d_n11;
        locals.var_t1_dn13 = assign15010_e22568_d_n13;
        locals.var_t1_dn14 = assign15010_e22568_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15020_e22624, assign15020_e22624_d_n0, assign15020_e22624_d_n2, assign15020_e22624_d_n3, assign15020_e22624_d_n4, assign15020_e22624_d_n5, assign15020_e22624_d_n6, assign15020_e22624_d_n7, assign15020_e22624_d_n8, assign15020_e22624_d_n9, assign15020_e22624_d_n10, assign15020_e22624_d_n11, assign15020_e22624_d_n13, assign15020_e22624_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15020_e22574: f64 = (-0.9);
        let assign15020_e22576: f64 = (assign15020_e22574 * locals.var_t1);
        let assign15020_e22580: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22582: f64 = (-0.9);
        let assign15020_e22584: f64 = (assign15020_e22582 * locals.var_t1);
        let assign15020_e22585: f64 = (assign15020_e22580 - assign15020_e22584);
        let assign15020_e22587: f64 = (assign15020_e22585 - 0.0001);
        let assign15020_e22590: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22592: f64 = (-0.9);
        let assign15020_e22594: f64 = (assign15020_e22592 * locals.var_t1);
        let assign15020_e22595: f64 = (assign15020_e22590 - assign15020_e22594);
        let assign15020_e22597: f64 = (assign15020_e22595 - 0.0001);
        let assign15020_e22600: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22602: f64 = (-0.9);
        let assign15020_e22604: f64 = (assign15020_e22602 * locals.var_t1);
        let assign15020_e22605: f64 = (assign15020_e22600 - assign15020_e22604);
        let assign15020_e22607: f64 = (assign15020_e22605 - 0.0001);
        let assign15020_e22608: f64 = (assign15020_e22597 * assign15020_e22607);
        let assign15020_e22611: f64 = (-0.9);
        let assign15020_e22613: f64 = (assign15020_e22611 * locals.var_t1);
        let assign15020_e22614: f64 = (4.0 * assign15020_e22613);
        let assign15020_e22616: f64 = (assign15020_e22614 * 0.0001);
        let assign15020_e22617: f64 = (assign15020_e22608 - assign15020_e22616);
        let assign15020_e22618: f64 = (assign15020_e22617).sqrt();
        let assign15020_e22619: f64 = (assign15020_e22587 + assign15020_e22618);
        let assign15020_e22620: f64 = (0.5 * assign15020_e22619);
        let assign15020_e22621: f64 = (assign15020_e22576 + assign15020_e22620);
        let assign15020_e22622: f64 = (locals.var_t1 + assign15020_e22621);
        (assign15020_e22622, (locals.var_t1_dn0 + ((assign15020_e22574 * locals.var_t1_dn0) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn0)) + (((((-(assign15020_e22592 * locals.var_t1_dn0)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn0)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn2 + ((assign15020_e22574 * locals.var_t1_dn2) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn2)) + (((((-(assign15020_e22592 * locals.var_t1_dn2)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn2)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn3 + ((assign15020_e22574 * locals.var_t1_dn3) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn3)) + (((((-(assign15020_e22592 * locals.var_t1_dn3)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn3)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn4 + ((assign15020_e22574 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22582 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22592 * locals.var_t1_dn4)) * assign15020_e22607) + (assign15020_e22597 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22602 * locals.var_t1_dn4)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn5 + ((assign15020_e22574 * locals.var_t1_dn5) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn5)) + (((((-(assign15020_e22592 * locals.var_t1_dn5)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn5)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn6 + ((assign15020_e22574 * locals.var_t1_dn6) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn6)) + (((((-(assign15020_e22592 * locals.var_t1_dn6)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn6)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn7 + ((assign15020_e22574 * locals.var_t1_dn7) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn7)) + (((((-(assign15020_e22592 * locals.var_t1_dn7)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn7)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn8 + ((assign15020_e22574 * locals.var_t1_dn8) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn8)) + (((((-(assign15020_e22592 * locals.var_t1_dn8)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn8)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn9 + ((assign15020_e22574 * locals.var_t1_dn9) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn9)) + (((((-(assign15020_e22592 * locals.var_t1_dn9)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn9)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn10 + ((assign15020_e22574 * locals.var_t1_dn10) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn10)) + (((((-(assign15020_e22592 * locals.var_t1_dn10)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn10)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn11 + ((assign15020_e22574 * locals.var_t1_dn11) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn11)) + (((((-(assign15020_e22592 * locals.var_t1_dn11)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn11)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn13 + ((assign15020_e22574 * locals.var_t1_dn13) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn13)) + (((((-(assign15020_e22592 * locals.var_t1_dn13)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn13)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn14 + ((assign15020_e22574 * locals.var_t1_dn14) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn14)) + (((((-(assign15020_e22592 * locals.var_t1_dn14)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn14)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15020_e22618)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign15020_e22624;
        locals.var_u0_cv_dn0 = assign15020_e22624_d_n0;
        locals.var_u0_cv_dn2 = assign15020_e22624_d_n2;
        locals.var_u0_cv_dn3 = assign15020_e22624_d_n3;
        locals.var_u0_cv_dn4 = assign15020_e22624_d_n4;
        locals.var_u0_cv_dn5 = assign15020_e22624_d_n5;
        locals.var_u0_cv_dn6 = assign15020_e22624_d_n6;
        locals.var_u0_cv_dn7 = assign15020_e22624_d_n7;
        locals.var_u0_cv_dn8 = assign15020_e22624_d_n8;
        locals.var_u0_cv_dn9 = assign15020_e22624_d_n9;
        locals.var_u0_cv_dn10 = assign15020_e22624_d_n10;
        locals.var_u0_cv_dn11 = assign15020_e22624_d_n11;
        locals.var_u0_cv_dn13 = assign15020_e22624_d_n13;
        locals.var_u0_cv_dn14 = assign15020_e22624_d_n14;
        locals.var_u0_cv_rv = 0.0;

        let (assign15030_e22670, assign15030_e22670_d_n0, assign15030_e22670_d_n2, assign15030_e22670_d_n3, assign15030_e22670_d_n4, assign15030_e22670_d_n5, assign15030_e22670_d_n6, assign15030_e22670_d_n7, assign15030_e22670_d_n8, assign15030_e22670_d_n9, assign15030_e22670_d_n10, assign15030_e22670_d_n11, assign15030_e22670_d_n13, assign15030_e22670_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15030_e22630: f64 = (-locals.var_uacv_i);
        let assign15030_e22634: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22636: f64 = (-locals.var_uacv_i);
        let assign15030_e22637: f64 = (assign15030_e22634 - assign15030_e22636);
        let assign15030_e22639: f64 = (assign15030_e22637 - 1e-6);
        let assign15030_e22642: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22644: f64 = (-locals.var_uacv_i);
        let assign15030_e22645: f64 = (assign15030_e22642 - assign15030_e22644);
        let assign15030_e22647: f64 = (assign15030_e22645 - 1e-6);
        let assign15030_e22650: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22652: f64 = (-locals.var_uacv_i);
        let assign15030_e22653: f64 = (assign15030_e22650 - assign15030_e22652);
        let assign15030_e22655: f64 = (assign15030_e22653 - 1e-6);
        let assign15030_e22656: f64 = (assign15030_e22647 * assign15030_e22655);
        let assign15030_e22659: f64 = (-locals.var_uacv_i);
        let assign15030_e22660: f64 = (4.0 * assign15030_e22659);
        let assign15030_e22662: f64 = (assign15030_e22660 * 1e-6);
        let assign15030_e22663: f64 = (assign15030_e22656 - assign15030_e22662);
        let assign15030_e22664: f64 = (assign15030_e22663).sqrt();
        let assign15030_e22665: f64 = (assign15030_e22639 + assign15030_e22664);
        let assign15030_e22666: f64 = (0.5 * assign15030_e22665);
        let assign15030_e22667: f64 = (assign15030_e22630 + assign15030_e22666);
        let assign15030_e22668: f64 = (locals.var_uacv_i + assign15030_e22667);
        (assign15030_e22668, 0.0, 0.0, 0.0, (0.5 * ((locals.var_ua1cv_i * locals.var_deltemp_dn4) + ((((locals.var_ua1cv_i * locals.var_deltemp_dn4) * assign15030_e22655) + (assign15030_e22647 * (locals.var_ua1cv_i * locals.var_deltemp_dn4))) / (2.0 * assign15030_e22664)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign15030_e22670;
        locals.var_uacv_t_dn0 = assign15030_e22670_d_n0;
        locals.var_uacv_t_dn2 = assign15030_e22670_d_n2;
        locals.var_uacv_t_dn3 = assign15030_e22670_d_n3;
        locals.var_uacv_t_dn4 = assign15030_e22670_d_n4;
        locals.var_uacv_t_dn5 = assign15030_e22670_d_n5;
        locals.var_uacv_t_dn6 = assign15030_e22670_d_n6;
        locals.var_uacv_t_dn7 = assign15030_e22670_d_n7;
        locals.var_uacv_t_dn8 = assign15030_e22670_d_n8;
        locals.var_uacv_t_dn9 = assign15030_e22670_d_n9;
        locals.var_uacv_t_dn10 = assign15030_e22670_d_n10;
        locals.var_uacv_t_dn11 = assign15030_e22670_d_n11;
        locals.var_uacv_t_dn13 = assign15030_e22670_d_n13;
        locals.var_uacv_t_dn14 = assign15030_e22670_d_n14;
        locals.var_uacv_t_rv = 0.0;

        let (assign15040_e22681, assign15040_e22681_d_n4,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15040_e22677: f64 = (locals.var_ud1cv_i * locals.var_trat_ln);
        let assign15040_e22678: f64 = (assign15040_e22677).exp();
        let assign15040_e22679: f64 = (locals.var_udcv_i * assign15040_e22678);
        (assign15040_e22679, (locals.var_udcv_i * (assign15040_e22678 * (locals.var_ud1cv_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign15040_e22681;
        locals.var_udcv_t_dn4 = assign15040_e22681_d_n4;
        locals.var_udcv_t_rv = 0.0;

        let assign15050_e22684: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign15050_e22684;
        locals.var_guard253_rv = 0.0;

        let (assign15060_e22733, assign15060_e22733_d_n0, assign15060_e22733_d_n2, assign15060_e22733_d_n3, assign15060_e22733_d_n4, assign15060_e22733_d_n5, assign15060_e22733_d_n6, assign15060_e22733_d_n7, assign15060_e22733_d_n8, assign15060_e22733_d_n9, assign15060_e22733_d_n10, assign15060_e22733_d_n11, assign15060_e22733_d_n13, assign15060_e22733_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 != 0.0)) {
        let assign15060_e22690: f64 = (-locals.var_vsat1_i);
        let assign15060_e22693: f64 = (-locals.var_at_i);
        let assign15060_e22695: f64 = (assign15060_e22693 * locals.var_deltemp);
        let assign15060_e22697: f64 = (-locals.var_vsat1_i);
        let assign15060_e22698: f64 = (assign15060_e22695 - assign15060_e22697);
        let assign15060_e22700: f64 = (assign15060_e22698 - 1e-6);
        let assign15060_e22702: f64 = (-locals.var_at_i);
        let assign15060_e22704: f64 = (assign15060_e22702 * locals.var_deltemp);
        let assign15060_e22706: f64 = (-locals.var_vsat1_i);
        let assign15060_e22707: f64 = (assign15060_e22704 - assign15060_e22706);
        let assign15060_e22709: f64 = (assign15060_e22707 - 1e-6);
        let assign15060_e22711: f64 = (-locals.var_at_i);
        let assign15060_e22713: f64 = (assign15060_e22711 * locals.var_deltemp);
        let assign15060_e22715: f64 = (-locals.var_vsat1_i);
        let assign15060_e22716: f64 = (assign15060_e22713 - assign15060_e22715);
        let assign15060_e22718: f64 = (assign15060_e22716 - 1e-6);
        let assign15060_e22719: f64 = (assign15060_e22709 * assign15060_e22718);
        let assign15060_e22722: f64 = (-locals.var_vsat1_i);
        let assign15060_e22723: f64 = (4.0 * assign15060_e22722);
        let assign15060_e22725: f64 = (assign15060_e22723 * 1e-6);
        let assign15060_e22726: f64 = (assign15060_e22719 - assign15060_e22725);
        let assign15060_e22727: f64 = (assign15060_e22726).sqrt();
        let assign15060_e22728: f64 = (assign15060_e22700 + assign15060_e22727);
        let assign15060_e22729: f64 = (0.5 * assign15060_e22728);
        let assign15060_e22730: f64 = (assign15060_e22690 + assign15060_e22729);
        let assign15060_e22731: f64 = (locals.var_vsat1_i + assign15060_e22730);
        (assign15060_e22731, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * (((assign15060_e22693 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) + ((((((assign15060_e22702 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) * assign15060_e22718) + (assign15060_e22709 * ((assign15060_e22711 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign15060_e22727)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15060_e22733;
        locals.var_vsat1_t_dn0 = assign15060_e22733_d_n0;
        locals.var_vsat1_t_dn2 = assign15060_e22733_d_n2;
        locals.var_vsat1_t_dn3 = assign15060_e22733_d_n3;
        locals.var_vsat1_t_dn4 = assign15060_e22733_d_n4;
        locals.var_vsat1_t_dn5 = assign15060_e22733_d_n5;
        locals.var_vsat1_t_dn6 = assign15060_e22733_d_n6;
        locals.var_vsat1_t_dn7 = assign15060_e22733_d_n7;
        locals.var_vsat1_t_dn8 = assign15060_e22733_d_n8;
        locals.var_vsat1_t_dn9 = assign15060_e22733_d_n9;
        locals.var_vsat1_t_dn10 = assign15060_e22733_d_n10;
        locals.var_vsat1_t_dn11 = assign15060_e22733_d_n11;
        locals.var_vsat1_t_dn13 = assign15060_e22733_d_n13;
        locals.var_vsat1_t_dn14 = assign15060_e22733_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let (assign15070_e22819, assign15070_e22819_d_n0, assign15070_e22819_d_n2, assign15070_e22819_d_n3, assign15070_e22819_d_n4, assign15070_e22819_d_n5, assign15070_e22819_d_n6, assign15070_e22819_d_n7, assign15070_e22819_d_n8, assign15070_e22819_d_n9, assign15070_e22819_d_n10, assign15070_e22819_d_n11, assign15070_e22819_d_n13, assign15070_e22819_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 == 0.0)) {
        let assign15070_e22741: f64 = (-locals.var_at_i);
        let assign15070_e22743: f64 = (assign15070_e22741 * locals.var_deltemp);
        let assign15070_e22744: f64 = (1.0 + assign15070_e22743);
        let assign15070_e22746: f64 = (assign15070_e22744 - 1e-6);
        let assign15070_e22748: f64 = (-10000.0);
        let assign15070_e22750: f64 = (assign15070_e22748 * 0.001);
        let (assign15070_e22816, assign15070_e22816_d_n4,) = {
            if (!(assign15070_e22746 < assign15070_e22750)) {
                let assign15070_e22756: f64 = (-locals.var_at_i);
                let assign15070_e22758: f64 = (assign15070_e22756 * locals.var_deltemp);
                let assign15070_e22759: f64 = (1.0 + assign15070_e22758);
                let assign15070_e22761: f64 = (assign15070_e22759 - 1e-6);
                let assign15070_e22764: f64 = (-locals.var_at_i);
                let assign15070_e22766: f64 = (assign15070_e22764 * locals.var_deltemp);
                let assign15070_e22767: f64 = (1.0 + assign15070_e22766);
                let assign15070_e22769: f64 = (assign15070_e22767 - 1e-6);
                let assign15070_e22772: f64 = (-locals.var_at_i);
                let assign15070_e22774: f64 = (assign15070_e22772 * locals.var_deltemp);
                let assign15070_e22775: f64 = (1.0 + assign15070_e22774);
                let assign15070_e22777: f64 = (assign15070_e22775 - 1e-6);
                let assign15070_e22778: f64 = (assign15070_e22769 * assign15070_e22777);
                let assign15070_e22781: f64 = (4.0 * 0.001);
                let assign15070_e22783: f64 = (assign15070_e22781 * 0.001);
                let assign15070_e22784: f64 = (assign15070_e22778 + assign15070_e22783);
                let assign15070_e22785: f64 = (assign15070_e22784).sqrt();
                let assign15070_e22786: f64 = (assign15070_e22761 + assign15070_e22785);
                let assign15070_e22787: f64 = (0.5 * assign15070_e22786);
                (assign15070_e22787, (0.5 * ((assign15070_e22756 * locals.var_deltemp_dn4) + ((((assign15070_e22764 * locals.var_deltemp_dn4) * assign15070_e22777) + (assign15070_e22769 * (assign15070_e22772 * locals.var_deltemp_dn4))) / (2.0 * assign15070_e22785)))),)
            } else {
                let assign15070_e22790: f64 = (-locals.var_at_i);
                let assign15070_e22792: f64 = (assign15070_e22790 * locals.var_deltemp);
                let assign15070_e22793: f64 = (1.0 + assign15070_e22792);
                let assign15070_e22795: f64 = (assign15070_e22793 - 1e-6);
                let assign15070_e22797: f64 = (-10000.0);
                let assign15070_e22799: f64 = (assign15070_e22797 * 0.001);
                let (assign15070_e22815, assign15070_e22815_d_n4,) = {
                    if (assign15070_e22795 < assign15070_e22799) {
                        let assign15070_e22802: f64 = (-0.001);
                        let assign15070_e22804: f64 = (assign15070_e22802 * 0.001);
                        let assign15070_e22807: f64 = (-locals.var_at_i);
                        let assign15070_e22809: f64 = (assign15070_e22807 * locals.var_deltemp);
                        let assign15070_e22810: f64 = (1.0 + assign15070_e22809);
                        let assign15070_e22812: f64 = (assign15070_e22810 - 1e-6);
                        let assign15070_e22813: f64 = (assign15070_e22804 / assign15070_e22812);
                        (assign15070_e22813, (-((assign15070_e22804 * (assign15070_e22807 * locals.var_deltemp_dn4)) / (assign15070_e22812 * assign15070_e22812))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15070_e22815, assign15070_e22815_d_n4,)
            }
        };
        let assign15070_e22817: f64 = (locals.var_vsat1_i * assign15070_e22816);
        (assign15070_e22817, (locals.var_vsat1_i_dn0 * assign15070_e22816), (locals.var_vsat1_i_dn2 * assign15070_e22816), (locals.var_vsat1_i_dn3 * assign15070_e22816), ((locals.var_vsat1_i_dn4 * assign15070_e22816) + (locals.var_vsat1_i * assign15070_e22816_d_n4)), (locals.var_vsat1_i_dn5 * assign15070_e22816), (locals.var_vsat1_i_dn6 * assign15070_e22816), (locals.var_vsat1_i_dn7 * assign15070_e22816), (locals.var_vsat1_i_dn8 * assign15070_e22816), (locals.var_vsat1_i_dn9 * assign15070_e22816), (locals.var_vsat1_i_dn10 * assign15070_e22816), (locals.var_vsat1_i_dn11 * assign15070_e22816), (locals.var_vsat1_i_dn13 * assign15070_e22816), (locals.var_vsat1_i_dn14 * assign15070_e22816),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15070_e22819;
        locals.var_vsat1_t_dn0 = assign15070_e22819_d_n0;
        locals.var_vsat1_t_dn2 = assign15070_e22819_d_n2;
        locals.var_vsat1_t_dn3 = assign15070_e22819_d_n3;
        locals.var_vsat1_t_dn4 = assign15070_e22819_d_n4;
        locals.var_vsat1_t_dn5 = assign15070_e22819_d_n5;
        locals.var_vsat1_t_dn6 = assign15070_e22819_d_n6;
        locals.var_vsat1_t_dn7 = assign15070_e22819_d_n7;
        locals.var_vsat1_t_dn8 = assign15070_e22819_d_n8;
        locals.var_vsat1_t_dn9 = assign15070_e22819_d_n9;
        locals.var_vsat1_t_dn10 = assign15070_e22819_d_n10;
        locals.var_vsat1_t_dn11 = assign15070_e22819_d_n11;
        locals.var_vsat1_t_dn13 = assign15070_e22819_d_n13;
        locals.var_vsat1_t_dn14 = assign15070_e22819_d_n14;
        locals.var_vsat1_t_rv = 0.0;

        let assign15080_e22822: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign15080_e22822;
        locals.var_guard254_rv = 0.0;

        let assign15090_e22825: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign15090_e22825;
        locals.var_guard255_rv = 0.0;

        let (assign15100_e22876, assign15100_e22876_d_n0, assign15100_e22876_d_n2, assign15100_e22876_d_n3, assign15100_e22876_d_n4, assign15100_e22876_d_n5, assign15100_e22876_d_n6, assign15100_e22876_d_n7, assign15100_e22876_d_n8, assign15100_e22876_d_n9, assign15100_e22876_d_n10, assign15100_e22876_d_n11, assign15100_e22876_d_n13, assign15100_e22876_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 != 0.0)) {
        let assign15100_e22833: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22836: f64 = (-locals.var_at_i);
        let assign15100_e22838: f64 = (assign15100_e22836 * locals.var_deltemp);
        let assign15100_e22840: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22841: f64 = (assign15100_e22838 - assign15100_e22840);
        let assign15100_e22843: f64 = (assign15100_e22841 - 1e-6);
        let assign15100_e22845: f64 = (-locals.var_at_i);
        let assign15100_e22847: f64 = (assign15100_e22845 * locals.var_deltemp);
        let assign15100_e22849: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22850: f64 = (assign15100_e22847 - assign15100_e22849);
        let assign15100_e22852: f64 = (assign15100_e22850 - 1e-6);
        let assign15100_e22854: f64 = (-locals.var_at_i);
        let assign15100_e22856: f64 = (assign15100_e22854 * locals.var_deltemp);
        let assign15100_e22858: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22859: f64 = (assign15100_e22856 - assign15100_e22858);
        let assign15100_e22861: f64 = (assign15100_e22859 - 1e-6);
        let assign15100_e22862: f64 = (assign15100_e22852 * assign15100_e22861);
        let assign15100_e22865: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22866: f64 = (4.0 * assign15100_e22865);
        let assign15100_e22868: f64 = (assign15100_e22866 * 1e-6);
        let assign15100_e22869: f64 = (assign15100_e22862 - assign15100_e22868);
        let assign15100_e22870: f64 = (assign15100_e22869).sqrt();
        let assign15100_e22871: f64 = (assign15100_e22843 + assign15100_e22870);
        let assign15100_e22872: f64 = (0.5 * assign15100_e22871);
        let assign15100_e22873: f64 = (assign15100_e22833 + assign15100_e22872);
        let assign15100_e22874: f64 = (locals.var_vsat1r_i + assign15100_e22873);
        (assign15100_e22874, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * (((assign15100_e22836 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) + ((((((assign15100_e22845 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) * assign15100_e22861) + (assign15100_e22852 * ((assign15100_e22854 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign15100_e22870)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15100_e22876;
        locals.var_vsat1r_t_dn0 = assign15100_e22876_d_n0;
        locals.var_vsat1r_t_dn2 = assign15100_e22876_d_n2;
        locals.var_vsat1r_t_dn3 = assign15100_e22876_d_n3;
        locals.var_vsat1r_t_dn4 = assign15100_e22876_d_n4;
        locals.var_vsat1r_t_dn5 = assign15100_e22876_d_n5;
        locals.var_vsat1r_t_dn6 = assign15100_e22876_d_n6;
        locals.var_vsat1r_t_dn7 = assign15100_e22876_d_n7;
        locals.var_vsat1r_t_dn8 = assign15100_e22876_d_n8;
        locals.var_vsat1r_t_dn9 = assign15100_e22876_d_n9;
        locals.var_vsat1r_t_dn10 = assign15100_e22876_d_n10;
        locals.var_vsat1r_t_dn11 = assign15100_e22876_d_n11;
        locals.var_vsat1r_t_dn13 = assign15100_e22876_d_n13;
        locals.var_vsat1r_t_dn14 = assign15100_e22876_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let (assign15110_e22964, assign15110_e22964_d_n0, assign15110_e22964_d_n2, assign15110_e22964_d_n3, assign15110_e22964_d_n4, assign15110_e22964_d_n5, assign15110_e22964_d_n6, assign15110_e22964_d_n7, assign15110_e22964_d_n8, assign15110_e22964_d_n9, assign15110_e22964_d_n10, assign15110_e22964_d_n11, assign15110_e22964_d_n13, assign15110_e22964_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 == 0.0)) {
        let assign15110_e22886: f64 = (-locals.var_at_i);
        let assign15110_e22888: f64 = (assign15110_e22886 * locals.var_deltemp);
        let assign15110_e22889: f64 = (1.0 + assign15110_e22888);
        let assign15110_e22891: f64 = (assign15110_e22889 - 1e-6);
        let assign15110_e22893: f64 = (-10000.0);
        let assign15110_e22895: f64 = (assign15110_e22893 * 0.001);
        let (assign15110_e22961, assign15110_e22961_d_n4,) = {
            if (!(assign15110_e22891 < assign15110_e22895)) {
                let assign15110_e22901: f64 = (-locals.var_at_i);
                let assign15110_e22903: f64 = (assign15110_e22901 * locals.var_deltemp);
                let assign15110_e22904: f64 = (1.0 + assign15110_e22903);
                let assign15110_e22906: f64 = (assign15110_e22904 - 1e-6);
                let assign15110_e22909: f64 = (-locals.var_at_i);
                let assign15110_e22911: f64 = (assign15110_e22909 * locals.var_deltemp);
                let assign15110_e22912: f64 = (1.0 + assign15110_e22911);
                let assign15110_e22914: f64 = (assign15110_e22912 - 1e-6);
                let assign15110_e22917: f64 = (-locals.var_at_i);
                let assign15110_e22919: f64 = (assign15110_e22917 * locals.var_deltemp);
                let assign15110_e22920: f64 = (1.0 + assign15110_e22919);
                let assign15110_e22922: f64 = (assign15110_e22920 - 1e-6);
                let assign15110_e22923: f64 = (assign15110_e22914 * assign15110_e22922);
                let assign15110_e22926: f64 = (4.0 * 0.001);
                let assign15110_e22928: f64 = (assign15110_e22926 * 0.001);
                let assign15110_e22929: f64 = (assign15110_e22923 + assign15110_e22928);
                let assign15110_e22930: f64 = (assign15110_e22929).sqrt();
                let assign15110_e22931: f64 = (assign15110_e22906 + assign15110_e22930);
                let assign15110_e22932: f64 = (0.5 * assign15110_e22931);
                (assign15110_e22932, (0.5 * ((assign15110_e22901 * locals.var_deltemp_dn4) + ((((assign15110_e22909 * locals.var_deltemp_dn4) * assign15110_e22922) + (assign15110_e22914 * (assign15110_e22917 * locals.var_deltemp_dn4))) / (2.0 * assign15110_e22930)))),)
            } else {
                let assign15110_e22935: f64 = (-locals.var_at_i);
                let assign15110_e22937: f64 = (assign15110_e22935 * locals.var_deltemp);
                let assign15110_e22938: f64 = (1.0 + assign15110_e22937);
                let assign15110_e22940: f64 = (assign15110_e22938 - 1e-6);
                let assign15110_e22942: f64 = (-10000.0);
                let assign15110_e22944: f64 = (assign15110_e22942 * 0.001);
                let (assign15110_e22960, assign15110_e22960_d_n4,) = {
                    if (assign15110_e22940 < assign15110_e22944) {
                        let assign15110_e22947: f64 = (-0.001);
                        let assign15110_e22949: f64 = (assign15110_e22947 * 0.001);
                        let assign15110_e22952: f64 = (-locals.var_at_i);
                        let assign15110_e22954: f64 = (assign15110_e22952 * locals.var_deltemp);
                        let assign15110_e22955: f64 = (1.0 + assign15110_e22954);
                        let assign15110_e22957: f64 = (assign15110_e22955 - 1e-6);
                        let assign15110_e22958: f64 = (assign15110_e22949 / assign15110_e22957);
                        (assign15110_e22958, (-((assign15110_e22949 * (assign15110_e22952 * locals.var_deltemp_dn4)) / (assign15110_e22957 * assign15110_e22957))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15110_e22960, assign15110_e22960_d_n4,)
            }
        };
        let assign15110_e22962: f64 = (locals.var_vsat1r_i * assign15110_e22961);
        (assign15110_e22962, (locals.var_vsat1r_i_dn0 * assign15110_e22961), (locals.var_vsat1r_i_dn2 * assign15110_e22961), (locals.var_vsat1r_i_dn3 * assign15110_e22961), ((locals.var_vsat1r_i_dn4 * assign15110_e22961) + (locals.var_vsat1r_i * assign15110_e22961_d_n4)), (locals.var_vsat1r_i_dn5 * assign15110_e22961), (locals.var_vsat1r_i_dn6 * assign15110_e22961), (locals.var_vsat1r_i_dn7 * assign15110_e22961), (locals.var_vsat1r_i_dn8 * assign15110_e22961), (locals.var_vsat1r_i_dn9 * assign15110_e22961), (locals.var_vsat1r_i_dn10 * assign15110_e22961), (locals.var_vsat1r_i_dn11 * assign15110_e22961), (locals.var_vsat1r_i_dn13 * assign15110_e22961), (locals.var_vsat1r_i_dn14 * assign15110_e22961),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15110_e22964;
        locals.var_vsat1r_t_dn0 = assign15110_e22964_d_n0;
        locals.var_vsat1r_t_dn2 = assign15110_e22964_d_n2;
        locals.var_vsat1r_t_dn3 = assign15110_e22964_d_n3;
        locals.var_vsat1r_t_dn4 = assign15110_e22964_d_n4;
        locals.var_vsat1r_t_dn5 = assign15110_e22964_d_n5;
        locals.var_vsat1r_t_dn6 = assign15110_e22964_d_n6;
        locals.var_vsat1r_t_dn7 = assign15110_e22964_d_n7;
        locals.var_vsat1r_t_dn8 = assign15110_e22964_d_n8;
        locals.var_vsat1r_t_dn9 = assign15110_e22964_d_n9;
        locals.var_vsat1r_t_dn10 = assign15110_e22964_d_n10;
        locals.var_vsat1r_t_dn11 = assign15110_e22964_d_n11;
        locals.var_vsat1r_t_dn13 = assign15110_e22964_d_n13;
        locals.var_vsat1r_t_dn14 = assign15110_e22964_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

        let assign15120_e22967: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign15120_e22967;
        locals.var_guard256_rv = 0.0;

        let (assign15130_e22975, assign15130_e22975_d_n0, assign15130_e22975_d_n2, assign15130_e22975_d_n3, assign15130_e22975_d_n4, assign15130_e22975_d_n5, assign15130_e22975_d_n6, assign15130_e22975_d_n7, assign15130_e22975_d_n8, assign15130_e22975_d_n9, assign15130_e22975_d_n10, assign15130_e22975_d_n11, assign15130_e22975_d_n13, assign15130_e22975_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard256 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15130_e22975;
        locals.var_vsat1r_t_dn0 = assign15130_e22975_d_n0;
        locals.var_vsat1r_t_dn2 = assign15130_e22975_d_n2;
        locals.var_vsat1r_t_dn3 = assign15130_e22975_d_n3;
        locals.var_vsat1r_t_dn4 = assign15130_e22975_d_n4;
        locals.var_vsat1r_t_dn5 = assign15130_e22975_d_n5;
        locals.var_vsat1r_t_dn6 = assign15130_e22975_d_n6;
        locals.var_vsat1r_t_dn7 = assign15130_e22975_d_n7;
        locals.var_vsat1r_t_dn8 = assign15130_e22975_d_n8;
        locals.var_vsat1r_t_dn9 = assign15130_e22975_d_n9;
        locals.var_vsat1r_t_dn10 = assign15130_e22975_d_n10;
        locals.var_vsat1r_t_dn11 = assign15130_e22975_d_n11;
        locals.var_vsat1r_t_dn13 = assign15130_e22975_d_n13;
        locals.var_vsat1r_t_dn14 = assign15130_e22975_d_n14;
        locals.var_vsat1r_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign15140_e22978: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign15140_e22978;
        locals.var_guard257_rv = 0.0;

        let (assign15150_e23027, assign15150_e23027_d_n0, assign15150_e23027_d_n2, assign15150_e23027_d_n3, assign15150_e23027_d_n4, assign15150_e23027_d_n5, assign15150_e23027_d_n6, assign15150_e23027_d_n7, assign15150_e23027_d_n8, assign15150_e23027_d_n9, assign15150_e23027_d_n10, assign15150_e23027_d_n11, assign15150_e23027_d_n13, assign15150_e23027_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 != 0.0)) {
        let assign15150_e22984: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22987: f64 = (-locals.var_atcv_i);
        let assign15150_e22989: f64 = (assign15150_e22987 * locals.var_deltemp);
        let assign15150_e22991: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22992: f64 = (assign15150_e22989 - assign15150_e22991);
        let assign15150_e22994: f64 = (assign15150_e22992 - 1e-6);
        let assign15150_e22996: f64 = (-locals.var_atcv_i);
        let assign15150_e22998: f64 = (assign15150_e22996 * locals.var_deltemp);
        let assign15150_e23000: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23001: f64 = (assign15150_e22998 - assign15150_e23000);
        let assign15150_e23003: f64 = (assign15150_e23001 - 1e-6);
        let assign15150_e23005: f64 = (-locals.var_atcv_i);
        let assign15150_e23007: f64 = (assign15150_e23005 * locals.var_deltemp);
        let assign15150_e23009: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23010: f64 = (assign15150_e23007 - assign15150_e23009);
        let assign15150_e23012: f64 = (assign15150_e23010 - 1e-6);
        let assign15150_e23013: f64 = (assign15150_e23003 * assign15150_e23012);
        let assign15150_e23016: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23017: f64 = (4.0 * assign15150_e23016);
        let assign15150_e23019: f64 = (assign15150_e23017 * 1e-6);
        let assign15150_e23020: f64 = (assign15150_e23013 - assign15150_e23019);
        let assign15150_e23021: f64 = (assign15150_e23020).sqrt();
        let assign15150_e23022: f64 = (assign15150_e22994 + assign15150_e23021);
        let assign15150_e23023: f64 = (0.5 * assign15150_e23022);
        let assign15150_e23024: f64 = (assign15150_e22984 + assign15150_e23023);
        let assign15150_e23025: f64 = (locals.var_vsatcv_i + assign15150_e23024);
        (assign15150_e23025, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * (((assign15150_e22987 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) + ((((((assign15150_e22996 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) * assign15150_e23012) + (assign15150_e23003 * ((assign15150_e23005 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign15150_e23021)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15150_e23027;
        locals.var_vsatcv_t_dn0 = assign15150_e23027_d_n0;
        locals.var_vsatcv_t_dn2 = assign15150_e23027_d_n2;
        locals.var_vsatcv_t_dn3 = assign15150_e23027_d_n3;
        locals.var_vsatcv_t_dn4 = assign15150_e23027_d_n4;
        locals.var_vsatcv_t_dn5 = assign15150_e23027_d_n5;
        locals.var_vsatcv_t_dn6 = assign15150_e23027_d_n6;
        locals.var_vsatcv_t_dn7 = assign15150_e23027_d_n7;
        locals.var_vsatcv_t_dn8 = assign15150_e23027_d_n8;
        locals.var_vsatcv_t_dn9 = assign15150_e23027_d_n9;
        locals.var_vsatcv_t_dn10 = assign15150_e23027_d_n10;
        locals.var_vsatcv_t_dn11 = assign15150_e23027_d_n11;
        locals.var_vsatcv_t_dn13 = assign15150_e23027_d_n13;
        locals.var_vsatcv_t_dn14 = assign15150_e23027_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15160_e23113, assign15160_e23113_d_n0, assign15160_e23113_d_n2, assign15160_e23113_d_n3, assign15160_e23113_d_n4, assign15160_e23113_d_n5, assign15160_e23113_d_n6, assign15160_e23113_d_n7, assign15160_e23113_d_n8, assign15160_e23113_d_n9, assign15160_e23113_d_n10, assign15160_e23113_d_n11, assign15160_e23113_d_n13, assign15160_e23113_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 == 0.0)) {
        let assign15160_e23035: f64 = (-locals.var_atcv_i);
        let assign15160_e23037: f64 = (assign15160_e23035 * locals.var_deltemp);
        let assign15160_e23038: f64 = (1.0 + assign15160_e23037);
        let assign15160_e23040: f64 = (assign15160_e23038 - 1e-6);
        let assign15160_e23042: f64 = (-10000.0);
        let assign15160_e23044: f64 = (assign15160_e23042 * 0.001);
        let (assign15160_e23110, assign15160_e23110_d_n4,) = {
            if (!(assign15160_e23040 < assign15160_e23044)) {
                let assign15160_e23050: f64 = (-locals.var_atcv_i);
                let assign15160_e23052: f64 = (assign15160_e23050 * locals.var_deltemp);
                let assign15160_e23053: f64 = (1.0 + assign15160_e23052);
                let assign15160_e23055: f64 = (assign15160_e23053 - 1e-6);
                let assign15160_e23058: f64 = (-locals.var_atcv_i);
                let assign15160_e23060: f64 = (assign15160_e23058 * locals.var_deltemp);
                let assign15160_e23061: f64 = (1.0 + assign15160_e23060);
                let assign15160_e23063: f64 = (assign15160_e23061 - 1e-6);
                let assign15160_e23066: f64 = (-locals.var_atcv_i);
                let assign15160_e23068: f64 = (assign15160_e23066 * locals.var_deltemp);
                let assign15160_e23069: f64 = (1.0 + assign15160_e23068);
                let assign15160_e23071: f64 = (assign15160_e23069 - 1e-6);
                let assign15160_e23072: f64 = (assign15160_e23063 * assign15160_e23071);
                let assign15160_e23075: f64 = (4.0 * 0.001);
                let assign15160_e23077: f64 = (assign15160_e23075 * 0.001);
                let assign15160_e23078: f64 = (assign15160_e23072 + assign15160_e23077);
                let assign15160_e23079: f64 = (assign15160_e23078).sqrt();
                let assign15160_e23080: f64 = (assign15160_e23055 + assign15160_e23079);
                let assign15160_e23081: f64 = (0.5 * assign15160_e23080);
                (assign15160_e23081, (0.5 * ((assign15160_e23050 * locals.var_deltemp_dn4) + ((((assign15160_e23058 * locals.var_deltemp_dn4) * assign15160_e23071) + (assign15160_e23063 * (assign15160_e23066 * locals.var_deltemp_dn4))) / (2.0 * assign15160_e23079)))),)
            } else {
                let assign15160_e23084: f64 = (-locals.var_atcv_i);
                let assign15160_e23086: f64 = (assign15160_e23084 * locals.var_deltemp);
                let assign15160_e23087: f64 = (1.0 + assign15160_e23086);
                let assign15160_e23089: f64 = (assign15160_e23087 - 1e-6);
                let assign15160_e23091: f64 = (-10000.0);
                let assign15160_e23093: f64 = (assign15160_e23091 * 0.001);
                let (assign15160_e23109, assign15160_e23109_d_n4,) = {
                    if (assign15160_e23089 < assign15160_e23093) {
                        let assign15160_e23096: f64 = (-0.001);
                        let assign15160_e23098: f64 = (assign15160_e23096 * 0.001);
                        let assign15160_e23101: f64 = (-locals.var_atcv_i);
                        let assign15160_e23103: f64 = (assign15160_e23101 * locals.var_deltemp);
                        let assign15160_e23104: f64 = (1.0 + assign15160_e23103);
                        let assign15160_e23106: f64 = (assign15160_e23104 - 1e-6);
                        let assign15160_e23107: f64 = (assign15160_e23098 / assign15160_e23106);
                        (assign15160_e23107, (-((assign15160_e23098 * (assign15160_e23101 * locals.var_deltemp_dn4)) / (assign15160_e23106 * assign15160_e23106))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15160_e23109, assign15160_e23109_d_n4,)
            }
        };
        let assign15160_e23111: f64 = (locals.var_vsatcv_i * assign15160_e23110);
        (assign15160_e23111, (locals.var_vsatcv_i_dn0 * assign15160_e23110), (locals.var_vsatcv_i_dn2 * assign15160_e23110), (locals.var_vsatcv_i_dn3 * assign15160_e23110), ((locals.var_vsatcv_i_dn4 * assign15160_e23110) + (locals.var_vsatcv_i * assign15160_e23110_d_n4)), (locals.var_vsatcv_i_dn5 * assign15160_e23110), (locals.var_vsatcv_i_dn6 * assign15160_e23110), (locals.var_vsatcv_i_dn7 * assign15160_e23110), (locals.var_vsatcv_i_dn8 * assign15160_e23110), (locals.var_vsatcv_i_dn9 * assign15160_e23110), (locals.var_vsatcv_i_dn10 * assign15160_e23110), (locals.var_vsatcv_i_dn11 * assign15160_e23110), (locals.var_vsatcv_i_dn13 * assign15160_e23110), (locals.var_vsatcv_i_dn14 * assign15160_e23110),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15160_e23113;
        locals.var_vsatcv_t_dn0 = assign15160_e23113_d_n0;
        locals.var_vsatcv_t_dn2 = assign15160_e23113_d_n2;
        locals.var_vsatcv_t_dn3 = assign15160_e23113_d_n3;
        locals.var_vsatcv_t_dn4 = assign15160_e23113_d_n4;
        locals.var_vsatcv_t_dn5 = assign15160_e23113_d_n5;
        locals.var_vsatcv_t_dn6 = assign15160_e23113_d_n6;
        locals.var_vsatcv_t_dn7 = assign15160_e23113_d_n7;
        locals.var_vsatcv_t_dn8 = assign15160_e23113_d_n8;
        locals.var_vsatcv_t_dn9 = assign15160_e23113_d_n9;
        locals.var_vsatcv_t_dn10 = assign15160_e23113_d_n10;
        locals.var_vsatcv_t_dn11 = assign15160_e23113_d_n11;
        locals.var_vsatcv_t_dn13 = assign15160_e23113_d_n13;
        locals.var_vsatcv_t_dn14 = assign15160_e23113_d_n14;
        locals.var_vsatcv_t_rv = 0.0;

        let (assign15170_e23202, assign15170_e23202_d_n0, assign15170_e23202_d_n2, assign15170_e23202_d_n3, assign15170_e23202_d_n4, assign15170_e23202_d_n5, assign15170_e23202_d_n6, assign15170_e23202_d_n7, assign15170_e23202_d_n8, assign15170_e23202_d_n9, assign15170_e23202_d_n10, assign15170_e23202_d_n11, assign15170_e23202_d_n13, assign15170_e23202_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15170_e23119: f64 = (p.p450 * locals.var_deltemp);
        let assign15170_e23120: f64 = (1.0 + assign15170_e23119);
        let assign15170_e23121: f64 = (locals.var_mexp_i * assign15170_e23120);
        let assign15170_e23123: f64 = (assign15170_e23121 - 2.0);
        let assign15170_e23125: f64 = (-10000.0);
        let assign15170_e23127: f64 = (assign15170_e23125 * 0.001);
        let (assign15170_e23198, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,) = {
            if (!(assign15170_e23123 < assign15170_e23127)) {
                let assign15170_e23135: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23136: f64 = (1.0 + assign15170_e23135);
                let assign15170_e23137: f64 = (locals.var_mexp_i * assign15170_e23136);
                let assign15170_e23139: f64 = (assign15170_e23137 - 2.0);
                let assign15170_e23144: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23145: f64 = (1.0 + assign15170_e23144);
                let assign15170_e23146: f64 = (locals.var_mexp_i * assign15170_e23145);
                let assign15170_e23148: f64 = (assign15170_e23146 - 2.0);
                let assign15170_e23153: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23154: f64 = (1.0 + assign15170_e23153);
                let assign15170_e23155: f64 = (locals.var_mexp_i * assign15170_e23154);
                let assign15170_e23157: f64 = (assign15170_e23155 - 2.0);
                let assign15170_e23158: f64 = (assign15170_e23148 * assign15170_e23157);
                let assign15170_e23161: f64 = (4.0 * 0.001);
                let assign15170_e23163: f64 = (assign15170_e23161 * 0.001);
                let assign15170_e23164: f64 = (assign15170_e23158 + assign15170_e23163);
                let assign15170_e23165: f64 = (assign15170_e23164).sqrt();
                let assign15170_e23166: f64 = (assign15170_e23139 + assign15170_e23165);
                let assign15170_e23167: f64 = (0.5 * assign15170_e23166);
                (assign15170_e23167, (0.5 * ((locals.var_mexp_i_dn0 * assign15170_e23136) + ((((locals.var_mexp_i_dn0 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn0 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn2 * assign15170_e23136) + ((((locals.var_mexp_i_dn2 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn2 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn3 * assign15170_e23136) + ((((locals.var_mexp_i_dn3 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn3 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * (((locals.var_mexp_i_dn4 * assign15170_e23136) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) + (((((locals.var_mexp_i_dn4 * assign15170_e23145) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) * assign15170_e23157) + (assign15170_e23148 * ((locals.var_mexp_i_dn4 * assign15170_e23154) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn5 * assign15170_e23136) + ((((locals.var_mexp_i_dn5 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn5 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn6 * assign15170_e23136) + ((((locals.var_mexp_i_dn6 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn6 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn7 * assign15170_e23136) + ((((locals.var_mexp_i_dn7 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn7 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn8 * assign15170_e23136) + ((((locals.var_mexp_i_dn8 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn8 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn9 * assign15170_e23136) + ((((locals.var_mexp_i_dn9 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn9 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn10 * assign15170_e23136) + ((((locals.var_mexp_i_dn10 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn10 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn11 * assign15170_e23136) + ((((locals.var_mexp_i_dn11 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn11 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn13 * assign15170_e23136) + ((((locals.var_mexp_i_dn13 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn13 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn14 * assign15170_e23136) + ((((locals.var_mexp_i_dn14 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn14 * assign15170_e23154))) / (2.0 * assign15170_e23165)))),)
            } else {
                let assign15170_e23172: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23173: f64 = (1.0 + assign15170_e23172);
                let assign15170_e23174: f64 = (locals.var_mexp_i * assign15170_e23173);
                let assign15170_e23176: f64 = (assign15170_e23174 - 2.0);
                let assign15170_e23178: f64 = (-10000.0);
                let assign15170_e23180: f64 = (assign15170_e23178 * 0.001);
                let (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,) = {
                    if (assign15170_e23176 < assign15170_e23180) {
                        let assign15170_e23183: f64 = (-0.001);
                        let assign15170_e23185: f64 = (assign15170_e23183 * 0.001);
                        let assign15170_e23190: f64 = (p.p450 * locals.var_deltemp);
                        let assign15170_e23191: f64 = (1.0 + assign15170_e23190);
                        let assign15170_e23192: f64 = (locals.var_mexp_i * assign15170_e23191);
                        let assign15170_e23194: f64 = (assign15170_e23192 - 2.0);
                        let assign15170_e23195: f64 = (assign15170_e23185 / assign15170_e23194);
                        (assign15170_e23195, (-((assign15170_e23185 * (locals.var_mexp_i_dn0 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn2 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn3 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * ((locals.var_mexp_i_dn4 * assign15170_e23191) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4)))) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn5 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn6 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn7 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn8 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn9 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn10 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn11 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn13 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn14 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,)
            }
        };
        let assign15170_e23200: f64 = (assign15170_e23198 + 2.0);
        (assign15170_e23200, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,)
    } else {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    }
};
        locals.var_mexp_t = assign15170_e23202;
        locals.var_mexp_t_dn0 = assign15170_e23202_d_n0;
        locals.var_mexp_t_dn2 = assign15170_e23202_d_n2;
        locals.var_mexp_t_dn3 = assign15170_e23202_d_n3;
        locals.var_mexp_t_dn4 = assign15170_e23202_d_n4;
        locals.var_mexp_t_dn5 = assign15170_e23202_d_n5;
        locals.var_mexp_t_dn6 = assign15170_e23202_d_n6;
        locals.var_mexp_t_dn7 = assign15170_e23202_d_n7;
        locals.var_mexp_t_dn8 = assign15170_e23202_d_n8;
        locals.var_mexp_t_dn9 = assign15170_e23202_d_n9;
        locals.var_mexp_t_dn10 = assign15170_e23202_d_n10;
        locals.var_mexp_t_dn11 = assign15170_e23202_d_n11;
        locals.var_mexp_t_dn13 = assign15170_e23202_d_n13;
        locals.var_mexp_t_dn14 = assign15170_e23202_d_n14;
        locals.var_mexp_t_rv = 0.0;

        let assign15180_e23205: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign15180_e23205;
        locals.var_guard258_rv = 0.0;

        let (assign15190_e23296, assign15190_e23296_d_n0, assign15190_e23296_d_n2, assign15190_e23296_d_n3, assign15190_e23296_d_n4, assign15190_e23296_d_n5, assign15190_e23296_d_n6, assign15190_e23296_d_n7, assign15190_e23296_d_n8, assign15190_e23296_d_n9, assign15190_e23296_d_n10, assign15190_e23296_d_n11, assign15190_e23296_d_n13, assign15190_e23296_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign15190_e23213: f64 = (p.p452 * locals.var_deltemp);
        let assign15190_e23214: f64 = (1.0 + assign15190_e23213);
        let assign15190_e23215: f64 = (locals.var_mexpr_i * assign15190_e23214);
        let assign15190_e23217: f64 = (assign15190_e23215 - 2.0);
        let assign15190_e23219: f64 = (-10000.0);
        let assign15190_e23221: f64 = (assign15190_e23219 * 0.001);
        let (assign15190_e23292, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,) = {
            if (!(assign15190_e23217 < assign15190_e23221)) {
                let assign15190_e23229: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23230: f64 = (1.0 + assign15190_e23229);
                let assign15190_e23231: f64 = (locals.var_mexpr_i * assign15190_e23230);
                let assign15190_e23233: f64 = (assign15190_e23231 - 2.0);
                let assign15190_e23238: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23239: f64 = (1.0 + assign15190_e23238);
                let assign15190_e23240: f64 = (locals.var_mexpr_i * assign15190_e23239);
                let assign15190_e23242: f64 = (assign15190_e23240 - 2.0);
                let assign15190_e23247: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23248: f64 = (1.0 + assign15190_e23247);
                let assign15190_e23249: f64 = (locals.var_mexpr_i * assign15190_e23248);
                let assign15190_e23251: f64 = (assign15190_e23249 - 2.0);
                let assign15190_e23252: f64 = (assign15190_e23242 * assign15190_e23251);
                let assign15190_e23255: f64 = (4.0 * 0.001);
                let assign15190_e23257: f64 = (assign15190_e23255 * 0.001);
                let assign15190_e23258: f64 = (assign15190_e23252 + assign15190_e23257);
                let assign15190_e23259: f64 = (assign15190_e23258).sqrt();
                let assign15190_e23260: f64 = (assign15190_e23233 + assign15190_e23259);
                let assign15190_e23261: f64 = (0.5 * assign15190_e23260);
                (assign15190_e23261, (0.5 * ((locals.var_mexpr_i_dn0 * assign15190_e23230) + ((((locals.var_mexpr_i_dn0 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn0 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn2 * assign15190_e23230) + ((((locals.var_mexpr_i_dn2 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn2 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn3 * assign15190_e23230) + ((((locals.var_mexpr_i_dn3 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn3 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * (((locals.var_mexpr_i_dn4 * assign15190_e23230) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) + (((((locals.var_mexpr_i_dn4 * assign15190_e23239) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) * assign15190_e23251) + (assign15190_e23242 * ((locals.var_mexpr_i_dn4 * assign15190_e23248) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn5 * assign15190_e23230) + ((((locals.var_mexpr_i_dn5 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn5 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn6 * assign15190_e23230) + ((((locals.var_mexpr_i_dn6 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn6 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn7 * assign15190_e23230) + ((((locals.var_mexpr_i_dn7 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn7 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn8 * assign15190_e23230) + ((((locals.var_mexpr_i_dn8 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn8 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn9 * assign15190_e23230) + ((((locals.var_mexpr_i_dn9 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn9 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn10 * assign15190_e23230) + ((((locals.var_mexpr_i_dn10 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn10 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn11 * assign15190_e23230) + ((((locals.var_mexpr_i_dn11 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn11 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn13 * assign15190_e23230) + ((((locals.var_mexpr_i_dn13 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn13 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn14 * assign15190_e23230) + ((((locals.var_mexpr_i_dn14 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn14 * assign15190_e23248))) / (2.0 * assign15190_e23259)))),)
            } else {
                let assign15190_e23266: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23267: f64 = (1.0 + assign15190_e23266);
                let assign15190_e23268: f64 = (locals.var_mexpr_i * assign15190_e23267);
                let assign15190_e23270: f64 = (assign15190_e23268 - 2.0);
                let assign15190_e23272: f64 = (-10000.0);
                let assign15190_e23274: f64 = (assign15190_e23272 * 0.001);
                let (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,) = {
                    if (assign15190_e23270 < assign15190_e23274) {
                        let assign15190_e23277: f64 = (-0.001);
                        let assign15190_e23279: f64 = (assign15190_e23277 * 0.001);
                        let assign15190_e23284: f64 = (p.p452 * locals.var_deltemp);
                        let assign15190_e23285: f64 = (1.0 + assign15190_e23284);
                        let assign15190_e23286: f64 = (locals.var_mexpr_i * assign15190_e23285);
                        let assign15190_e23288: f64 = (assign15190_e23286 - 2.0);
                        let assign15190_e23289: f64 = (assign15190_e23279 / assign15190_e23288);
                        (assign15190_e23289, (-((assign15190_e23279 * (locals.var_mexpr_i_dn0 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn2 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn3 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * ((locals.var_mexpr_i_dn4 * assign15190_e23285) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4)))) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn5 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn6 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn7 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn8 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn9 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn10 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn11 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn13 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn14 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,)
            }
        };
        let assign15190_e23294: f64 = (assign15190_e23292 + 2.0);
        (assign15190_e23294, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,)
    } else {
        (locals.var_mexpr_t, locals.var_mexpr_t_dn0, locals.var_mexpr_t_dn2, locals.var_mexpr_t_dn3, locals.var_mexpr_t_dn4, locals.var_mexpr_t_dn5, locals.var_mexpr_t_dn6, locals.var_mexpr_t_dn7, locals.var_mexpr_t_dn8, locals.var_mexpr_t_dn9, locals.var_mexpr_t_dn10, locals.var_mexpr_t_dn11, locals.var_mexpr_t_dn13, locals.var_mexpr_t_dn14,)
    }
};
        locals.var_mexpr_t = assign15190_e23296;
        locals.var_mexpr_t_dn0 = assign15190_e23296_d_n0;
        locals.var_mexpr_t_dn2 = assign15190_e23296_d_n2;
        locals.var_mexpr_t_dn3 = assign15190_e23296_d_n3;
        locals.var_mexpr_t_dn4 = assign15190_e23296_d_n4;
        locals.var_mexpr_t_dn5 = assign15190_e23296_d_n5;
        locals.var_mexpr_t_dn6 = assign15190_e23296_d_n6;
        locals.var_mexpr_t_dn7 = assign15190_e23296_d_n7;
        locals.var_mexpr_t_dn8 = assign15190_e23296_d_n8;
        locals.var_mexpr_t_dn9 = assign15190_e23296_d_n9;
        locals.var_mexpr_t_dn10 = assign15190_e23296_d_n10;
        locals.var_mexpr_t_dn11 = assign15190_e23296_d_n11;
        locals.var_mexpr_t_dn13 = assign15190_e23296_d_n13;
        locals.var_mexpr_t_dn14 = assign15190_e23296_d_n14;
        locals.var_mexpr_t_rv = 0.0;

        let (assign15200_e23300, assign15200_e23300_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_ksativ_i, 0.0,)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign15200_e23300;
        locals.var_ksativ_t_dn4 = assign15200_e23300_d_n4;
        locals.var_ksativ_t_rv = 0.0;

        let (assign15210_e23304, assign15210_e23304_d_n0, assign15210_e23304_d_n2, assign15210_e23304_d_n3, assign15210_e23304_d_n4, assign15210_e23304_d_n5, assign15210_e23304_d_n6, assign15210_e23304_d_n7, assign15210_e23304_d_n8, assign15210_e23304_d_n9, assign15210_e23304_d_n10, assign15210_e23304_d_n11, assign15210_e23304_d_n13, assign15210_e23304_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14,)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign15210_e23304;
        locals.var_pclm_t_dn0 = assign15210_e23304_d_n0;
        locals.var_pclm_t_dn2 = assign15210_e23304_d_n2;
        locals.var_pclm_t_dn3 = assign15210_e23304_d_n3;
        locals.var_pclm_t_dn4 = assign15210_e23304_d_n4;
        locals.var_pclm_t_dn5 = assign15210_e23304_d_n5;
        locals.var_pclm_t_dn6 = assign15210_e23304_d_n6;
        locals.var_pclm_t_dn7 = assign15210_e23304_d_n7;
        locals.var_pclm_t_dn8 = assign15210_e23304_d_n8;
        locals.var_pclm_t_dn9 = assign15210_e23304_d_n9;
        locals.var_pclm_t_dn10 = assign15210_e23304_d_n10;
        locals.var_pclm_t_dn11 = assign15210_e23304_d_n11;
        locals.var_pclm_t_dn13 = assign15210_e23304_d_n13;
        locals.var_pclm_t_dn14 = assign15210_e23304_d_n14;
        locals.var_pclm_t_rv = 0.0;

        let (assign15220_e23314, assign15220_e23314_d_n0, assign15220_e23314_d_n2, assign15220_e23314_d_n3, assign15220_e23314_d_n4, assign15220_e23314_d_n5, assign15220_e23314_d_n6, assign15220_e23314_d_n7, assign15220_e23314_d_n8, assign15220_e23314_d_n9, assign15220_e23314_d_n10, assign15220_e23314_d_n11, assign15220_e23314_d_n13, assign15220_e23314_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15220_e23309: f64 = (p.p1720 / locals.var_leff_1);
        let assign15220_e23310: f64 = (locals.var_kt1_i + assign15220_e23309);
        let assign15220_e23312: f64 = (assign15220_e23310 * locals.var_tratio_m1);
        (assign15220_e23312, ((-((p.p1720 * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), (((-((p.p1720 * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) + (assign15220_e23310 * locals.var_tratio_m1_dn4)), ((-((p.p1720 * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign15220_e23314;
        locals.var_dvth_temp_dn0 = assign15220_e23314_d_n0;
        locals.var_dvth_temp_dn2 = assign15220_e23314_d_n2;
        locals.var_dvth_temp_dn3 = assign15220_e23314_d_n3;
        locals.var_dvth_temp_dn4 = assign15220_e23314_d_n4;
        locals.var_dvth_temp_dn5 = assign15220_e23314_d_n5;
        locals.var_dvth_temp_dn6 = assign15220_e23314_d_n6;
        locals.var_dvth_temp_dn7 = assign15220_e23314_d_n7;
        locals.var_dvth_temp_dn8 = assign15220_e23314_d_n8;
        locals.var_dvth_temp_dn9 = assign15220_e23314_d_n9;
        locals.var_dvth_temp_dn10 = assign15220_e23314_d_n10;
        locals.var_dvth_temp_dn11 = assign15220_e23314_d_n11;
        locals.var_dvth_temp_dn13 = assign15220_e23314_d_n13;
        locals.var_dvth_temp_dn14 = assign15220_e23314_d_n14;
        locals.var_dvth_temp_rv = 0.0;

        let assign15230_e23317: f64 = if p.p80 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign15230_e23317;
        locals.var_guard259_rv = 0.0;

        let (assign15240_e23333, assign15240_e23333_d_n0, assign15240_e23333_d_n2, assign15240_e23333_d_n3, assign15240_e23333_d_n4, assign15240_e23333_d_n5, assign15240_e23333_d_n6, assign15240_e23333_d_n7, assign15240_e23333_d_n8, assign15240_e23333_d_n9, assign15240_e23333_d_n10, assign15240_e23333_d_n11, assign15240_e23333_d_n13, assign15240_e23333_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15240_e23326: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15240_e23327: f64 = (locals.var_ute_i + assign15240_e23326);
        let assign15240_e23329: f64 = (assign15240_e23327 * locals.var_trat_ln);
        let assign15240_e23330: f64 = (assign15240_e23329).exp();
        let assign15240_e23331: f64 = (locals.var_u0_i * assign15240_e23330);
        (assign15240_e23331, (locals.var_u0_i_dn0 * assign15240_e23330), (locals.var_u0_i_dn2 * assign15240_e23330), (locals.var_u0_i_dn3 * assign15240_e23330), ((locals.var_u0_i_dn4 * assign15240_e23330) + (locals.var_u0_i * (assign15240_e23330 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15240_e23327 * locals.var_trat_ln_dn4))))), (locals.var_u0_i_dn5 * assign15240_e23330), (locals.var_u0_i_dn6 * assign15240_e23330), (locals.var_u0_i_dn7 * assign15240_e23330), (locals.var_u0_i_dn8 * assign15240_e23330), (locals.var_u0_i_dn9 * assign15240_e23330), (locals.var_u0_i_dn10 * assign15240_e23330), (locals.var_u0_i_dn11 * assign15240_e23330), (locals.var_u0_i_dn13 * assign15240_e23330), (locals.var_u0_i_dn14 * assign15240_e23330),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15240_e23333;
        locals.var_t1_dn0 = assign15240_e23333_d_n0;
        locals.var_t1_dn2 = assign15240_e23333_d_n2;
        locals.var_t1_dn3 = assign15240_e23333_d_n3;
        locals.var_t1_dn4 = assign15240_e23333_d_n4;
        locals.var_t1_dn5 = assign15240_e23333_d_n5;
        locals.var_t1_dn6 = assign15240_e23333_d_n6;
        locals.var_t1_dn7 = assign15240_e23333_d_n7;
        locals.var_t1_dn8 = assign15240_e23333_d_n8;
        locals.var_t1_dn9 = assign15240_e23333_d_n9;
        locals.var_t1_dn10 = assign15240_e23333_d_n10;
        locals.var_t1_dn11 = assign15240_e23333_d_n11;
        locals.var_t1_dn13 = assign15240_e23333_d_n13;
        locals.var_t1_dn14 = assign15240_e23333_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15250_e23390, assign15250_e23390_d_n0, assign15250_e23390_d_n2, assign15250_e23390_d_n3, assign15250_e23390_d_n4, assign15250_e23390_d_n5, assign15250_e23390_d_n6, assign15250_e23390_d_n7, assign15250_e23390_d_n8, assign15250_e23390_d_n9, assign15250_e23390_d_n10, assign15250_e23390_d_n11, assign15250_e23390_d_n13, assign15250_e23390_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15250_e23340: f64 = (-0.9);
        let assign15250_e23342: f64 = (assign15250_e23340 * locals.var_t1);
        let assign15250_e23346: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23348: f64 = (-0.9);
        let assign15250_e23350: f64 = (assign15250_e23348 * locals.var_t1);
        let assign15250_e23351: f64 = (assign15250_e23346 - assign15250_e23350);
        let assign15250_e23353: f64 = (assign15250_e23351 - 0.0001);
        let assign15250_e23356: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23358: f64 = (-0.9);
        let assign15250_e23360: f64 = (assign15250_e23358 * locals.var_t1);
        let assign15250_e23361: f64 = (assign15250_e23356 - assign15250_e23360);
        let assign15250_e23363: f64 = (assign15250_e23361 - 0.0001);
        let assign15250_e23366: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23368: f64 = (-0.9);
        let assign15250_e23370: f64 = (assign15250_e23368 * locals.var_t1);
        let assign15250_e23371: f64 = (assign15250_e23366 - assign15250_e23370);
        let assign15250_e23373: f64 = (assign15250_e23371 - 0.0001);
        let assign15250_e23374: f64 = (assign15250_e23363 * assign15250_e23373);
        let assign15250_e23377: f64 = (-0.9);
        let assign15250_e23379: f64 = (assign15250_e23377 * locals.var_t1);
        let assign15250_e23380: f64 = (4.0 * assign15250_e23379);
        let assign15250_e23382: f64 = (assign15250_e23380 * 0.0001);
        let assign15250_e23383: f64 = (assign15250_e23374 - assign15250_e23382);
        let assign15250_e23384: f64 = (assign15250_e23383).sqrt();
        let assign15250_e23385: f64 = (assign15250_e23353 + assign15250_e23384);
        let assign15250_e23386: f64 = (0.5 * assign15250_e23385);
        let assign15250_e23387: f64 = (assign15250_e23342 + assign15250_e23386);
        let assign15250_e23388: f64 = (locals.var_t1 + assign15250_e23387);
        (assign15250_e23388, (locals.var_t1_dn0 + ((assign15250_e23340 * locals.var_t1_dn0) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn0)) + (((((-(assign15250_e23358 * locals.var_t1_dn0)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn0)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn2 + ((assign15250_e23340 * locals.var_t1_dn2) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn2)) + (((((-(assign15250_e23358 * locals.var_t1_dn2)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn2)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn3 + ((assign15250_e23340 * locals.var_t1_dn3) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn3)) + (((((-(assign15250_e23358 * locals.var_t1_dn3)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn3)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn4 + ((assign15250_e23340 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23348 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23358 * locals.var_t1_dn4)) * assign15250_e23373) + (assign15250_e23363 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23368 * locals.var_t1_dn4)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn5 + ((assign15250_e23340 * locals.var_t1_dn5) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn5)) + (((((-(assign15250_e23358 * locals.var_t1_dn5)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn5)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn6 + ((assign15250_e23340 * locals.var_t1_dn6) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn6)) + (((((-(assign15250_e23358 * locals.var_t1_dn6)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn6)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn7 + ((assign15250_e23340 * locals.var_t1_dn7) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn7)) + (((((-(assign15250_e23358 * locals.var_t1_dn7)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn7)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn8 + ((assign15250_e23340 * locals.var_t1_dn8) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn8)) + (((((-(assign15250_e23358 * locals.var_t1_dn8)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn8)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn9 + ((assign15250_e23340 * locals.var_t1_dn9) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn9)) + (((((-(assign15250_e23358 * locals.var_t1_dn9)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn9)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn10 + ((assign15250_e23340 * locals.var_t1_dn10) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn10)) + (((((-(assign15250_e23358 * locals.var_t1_dn10)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn10)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn11 + ((assign15250_e23340 * locals.var_t1_dn11) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn11)) + (((((-(assign15250_e23358 * locals.var_t1_dn11)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn11)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn13 + ((assign15250_e23340 * locals.var_t1_dn13) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn13)) + (((((-(assign15250_e23358 * locals.var_t1_dn13)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn13)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn14 + ((assign15250_e23340 * locals.var_t1_dn14) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn14)) + (((((-(assign15250_e23358 * locals.var_t1_dn14)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn14)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15250_e23384)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign15250_e23390;
        locals.var_u0_v_dn0 = assign15250_e23390_d_n0;
        locals.var_u0_v_dn2 = assign15250_e23390_d_n2;
        locals.var_u0_v_dn3 = assign15250_e23390_d_n3;
        locals.var_u0_v_dn4 = assign15250_e23390_d_n4;
        locals.var_u0_v_dn5 = assign15250_e23390_d_n5;
        locals.var_u0_v_dn6 = assign15250_e23390_d_n6;
        locals.var_u0_v_dn7 = assign15250_e23390_d_n7;
        locals.var_u0_v_dn8 = assign15250_e23390_d_n8;
        locals.var_u0_v_dn9 = assign15250_e23390_d_n9;
        locals.var_u0_v_dn10 = assign15250_e23390_d_n10;
        locals.var_u0_v_dn11 = assign15250_e23390_d_n11;
        locals.var_u0_v_dn13 = assign15250_e23390_d_n13;
        locals.var_u0_v_dn14 = assign15250_e23390_d_n14;
        locals.var_u0_v_rv = 0.0;

        let assign15260_e23393: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign15260_e23393;
        locals.var_guard260_rv = 0.0;

        let (assign15270_e23411, assign15270_e23411_d_n0, assign15270_e23411_d_n2, assign15270_e23411_d_n3, assign15270_e23411_d_n4, assign15270_e23411_d_n5, assign15270_e23411_d_n6, assign15270_e23411_d_n7, assign15270_e23411_d_n8, assign15270_e23411_d_n9, assign15270_e23411_d_n10, assign15270_e23411_d_n11, assign15270_e23411_d_n13, assign15270_e23411_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15270_e23404: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15270_e23405: f64 = (locals.var_uter_i + assign15270_e23404);
        let assign15270_e23407: f64 = (assign15270_e23405 * locals.var_trat_ln);
        let assign15270_e23408: f64 = (assign15270_e23407).exp();
        let assign15270_e23409: f64 = (locals.var_u0r_i * assign15270_e23408);
        (assign15270_e23409, (locals.var_u0r_i_dn0 * assign15270_e23408), (locals.var_u0r_i_dn2 * assign15270_e23408), (locals.var_u0r_i_dn3 * assign15270_e23408), ((locals.var_u0r_i_dn4 * assign15270_e23408) + (locals.var_u0r_i * (assign15270_e23408 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15270_e23405 * locals.var_trat_ln_dn4))))), (locals.var_u0r_i_dn5 * assign15270_e23408), (locals.var_u0r_i_dn6 * assign15270_e23408), (locals.var_u0r_i_dn7 * assign15270_e23408), (locals.var_u0r_i_dn8 * assign15270_e23408), (locals.var_u0r_i_dn9 * assign15270_e23408), (locals.var_u0r_i_dn10 * assign15270_e23408), (locals.var_u0r_i_dn11 * assign15270_e23408), (locals.var_u0r_i_dn13 * assign15270_e23408), (locals.var_u0r_i_dn14 * assign15270_e23408),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15270_e23411;
        locals.var_t1_dn0 = assign15270_e23411_d_n0;
        locals.var_t1_dn2 = assign15270_e23411_d_n2;
        locals.var_t1_dn3 = assign15270_e23411_d_n3;
        locals.var_t1_dn4 = assign15270_e23411_d_n4;
        locals.var_t1_dn5 = assign15270_e23411_d_n5;
        locals.var_t1_dn6 = assign15270_e23411_d_n6;
        locals.var_t1_dn7 = assign15270_e23411_d_n7;
        locals.var_t1_dn8 = assign15270_e23411_d_n8;
        locals.var_t1_dn9 = assign15270_e23411_d_n9;
        locals.var_t1_dn10 = assign15270_e23411_d_n10;
        locals.var_t1_dn11 = assign15270_e23411_d_n11;
        locals.var_t1_dn13 = assign15270_e23411_d_n13;
        locals.var_t1_dn14 = assign15270_e23411_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15280_e23470, assign15280_e23470_d_n0, assign15280_e23470_d_n2, assign15280_e23470_d_n3, assign15280_e23470_d_n4, assign15280_e23470_d_n5, assign15280_e23470_d_n6, assign15280_e23470_d_n7, assign15280_e23470_d_n8, assign15280_e23470_d_n9, assign15280_e23470_d_n10, assign15280_e23470_d_n11, assign15280_e23470_d_n13, assign15280_e23470_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15280_e23420: f64 = (-0.9);
        let assign15280_e23422: f64 = (assign15280_e23420 * locals.var_t1);
        let assign15280_e23426: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23428: f64 = (-0.9);
        let assign15280_e23430: f64 = (assign15280_e23428 * locals.var_t1);
        let assign15280_e23431: f64 = (assign15280_e23426 - assign15280_e23430);
        let assign15280_e23433: f64 = (assign15280_e23431 - 0.0001);
        let assign15280_e23436: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23438: f64 = (-0.9);
        let assign15280_e23440: f64 = (assign15280_e23438 * locals.var_t1);
        let assign15280_e23441: f64 = (assign15280_e23436 - assign15280_e23440);
        let assign15280_e23443: f64 = (assign15280_e23441 - 0.0001);
        let assign15280_e23446: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23448: f64 = (-0.9);
        let assign15280_e23450: f64 = (assign15280_e23448 * locals.var_t1);
        let assign15280_e23451: f64 = (assign15280_e23446 - assign15280_e23450);
        let assign15280_e23453: f64 = (assign15280_e23451 - 0.0001);
        let assign15280_e23454: f64 = (assign15280_e23443 * assign15280_e23453);
        let assign15280_e23457: f64 = (-0.9);
        let assign15280_e23459: f64 = (assign15280_e23457 * locals.var_t1);
        let assign15280_e23460: f64 = (4.0 * assign15280_e23459);
        let assign15280_e23462: f64 = (assign15280_e23460 * 0.0001);
        let assign15280_e23463: f64 = (assign15280_e23454 - assign15280_e23462);
        let assign15280_e23464: f64 = (assign15280_e23463).sqrt();
        let assign15280_e23465: f64 = (assign15280_e23433 + assign15280_e23464);
        let assign15280_e23466: f64 = (0.5 * assign15280_e23465);
        let assign15280_e23467: f64 = (assign15280_e23422 + assign15280_e23466);
        let assign15280_e23468: f64 = (locals.var_t1 + assign15280_e23467);
        (assign15280_e23468, (locals.var_t1_dn0 + ((assign15280_e23420 * locals.var_t1_dn0) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn0)) + (((((-(assign15280_e23438 * locals.var_t1_dn0)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn0)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn2 + ((assign15280_e23420 * locals.var_t1_dn2) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn2)) + (((((-(assign15280_e23438 * locals.var_t1_dn2)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn2)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn3 + ((assign15280_e23420 * locals.var_t1_dn3) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn3)) + (((((-(assign15280_e23438 * locals.var_t1_dn3)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn3)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn4 + ((assign15280_e23420 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23428 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23438 * locals.var_t1_dn4)) * assign15280_e23453) + (assign15280_e23443 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23448 * locals.var_t1_dn4)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn5 + ((assign15280_e23420 * locals.var_t1_dn5) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn5)) + (((((-(assign15280_e23438 * locals.var_t1_dn5)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn5)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn6 + ((assign15280_e23420 * locals.var_t1_dn6) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn6)) + (((((-(assign15280_e23438 * locals.var_t1_dn6)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn6)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn7 + ((assign15280_e23420 * locals.var_t1_dn7) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn7)) + (((((-(assign15280_e23438 * locals.var_t1_dn7)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn7)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn8 + ((assign15280_e23420 * locals.var_t1_dn8) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn8)) + (((((-(assign15280_e23438 * locals.var_t1_dn8)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn8)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn9 + ((assign15280_e23420 * locals.var_t1_dn9) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn9)) + (((((-(assign15280_e23438 * locals.var_t1_dn9)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn9)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn10 + ((assign15280_e23420 * locals.var_t1_dn10) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn10)) + (((((-(assign15280_e23438 * locals.var_t1_dn10)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn10)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn11 + ((assign15280_e23420 * locals.var_t1_dn11) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn11)) + (((((-(assign15280_e23438 * locals.var_t1_dn11)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn11)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn13 + ((assign15280_e23420 * locals.var_t1_dn13) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn13)) + (((((-(assign15280_e23438 * locals.var_t1_dn13)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn13)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn14 + ((assign15280_e23420 * locals.var_t1_dn14) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn14)) + (((((-(assign15280_e23438 * locals.var_t1_dn14)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn14)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15280_e23464)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign15280_e23470;
        locals.var_u0r_t_dn0 = assign15280_e23470_d_n0;
        locals.var_u0r_t_dn2 = assign15280_e23470_d_n2;
        locals.var_u0r_t_dn3 = assign15280_e23470_d_n3;
        locals.var_u0r_t_dn4 = assign15280_e23470_d_n4;
        locals.var_u0r_t_dn5 = assign15280_e23470_d_n5;
        locals.var_u0r_t_dn6 = assign15280_e23470_d_n6;
        locals.var_u0r_t_dn7 = assign15280_e23470_d_n7;
        locals.var_u0r_t_dn8 = assign15280_e23470_d_n8;
        locals.var_u0r_t_dn9 = assign15280_e23470_d_n9;
        locals.var_u0r_t_dn10 = assign15280_e23470_d_n10;
        locals.var_u0r_t_dn11 = assign15280_e23470_d_n11;
        locals.var_u0r_t_dn13 = assign15280_e23470_d_n13;
        locals.var_u0r_t_dn14 = assign15280_e23470_d_n14;
        locals.var_u0r_t_rv = 0.0;

    }
}
