#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_24(
        locals: &mut StampLocals,
    ) {
        let (assign43820_e56599, assign43820_e56599_d_n4, assign43820_e56599_d_n6, assign43820_e56599_d_n7, assign43820_e56599_d_n8, assign43820_e56599_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) {
        let assign43820_e56597: f64 = (1.0 - locals.var_temp__blk949);
        (assign43820_e56597, (-locals.var_temp__blk949_dn4), (-locals.var_temp__blk949_dn6), (-locals.var_temp__blk949_dn7), (-locals.var_temp__blk949_dn8), (-locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43820_e56599;
        locals.var_temp1_dn4 = assign43820_e56599_d_n4;
        locals.var_temp1_dn6 = assign43820_e56599_d_n6;
        locals.var_temp1_dn7 = assign43820_e56599_d_n7;
        locals.var_temp1_dn8 = assign43820_e56599_d_n8;
        locals.var_temp1_dn9 = assign43820_e56599_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign43830_e56602: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign43830_e56602;
        locals.var_guard1217_rv = 0.0;

        let (assign43840_e56612, assign43840_e56612_d_n4, assign43840_e56612_d_n6, assign43840_e56612_d_n7, assign43840_e56612_d_n8, assign43840_e56612_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) && (locals.var_guard1217 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43840_e56612;
        locals.var_temp2_dn4 = assign43840_e56612_d_n4;
        locals.var_temp2_dn6 = assign43840_e56612_d_n6;
        locals.var_temp2_dn7 = assign43840_e56612_d_n7;
        locals.var_temp2_dn8 = assign43840_e56612_d_n8;
        locals.var_temp2_dn9 = assign43840_e56612_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign43850_e56626, assign43850_e56626_d_n4, assign43850_e56626_d_n6, assign43850_e56626_d_n7, assign43850_e56626_d_n8, assign43850_e56626_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) && (locals.var_guard1217 == 0.0)) {
        let assign43850_e56623: f64 = (locals.var_temp1).sqrt();
        let assign43850_e56624: f64 = (1.0 - assign43850_e56623);
        (assign43850_e56624, (-(locals.var_temp1_dn4 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn6 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn7 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn8 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn9 / (2.0 * assign43850_e56623))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43850_e56626;
        locals.var_temp2_dn4 = assign43850_e56626_d_n4;
        locals.var_temp2_dn6 = assign43850_e56626_d_n6;
        locals.var_temp2_dn7 = assign43850_e56626_d_n7;
        locals.var_temp2_dn8 = assign43850_e56626_d_n8;
        locals.var_temp2_dn9 = assign43850_e56626_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign43860_e56637, assign43860_e56637_d_n4, assign43860_e56637_d_n6, assign43860_e56637_d_n7, assign43860_e56637_d_n8, assign43860_e56637_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 == 0.0)) {
        let assign43860_e56635: f64 = (0.5 * locals.var_temp__blk949);
        (assign43860_e56635, (0.5 * locals.var_temp__blk949_dn4), (0.5 * locals.var_temp__blk949_dn6), (0.5 * locals.var_temp__blk949_dn7), (0.5 * locals.var_temp__blk949_dn8), (0.5 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43860_e56637;
        locals.var_temp2_dn4 = assign43860_e56637_d_n4;
        locals.var_temp2_dn6 = assign43860_e56637_d_n6;
        locals.var_temp2_dn7 = assign43860_e56637_d_n7;
        locals.var_temp2_dn8 = assign43860_e56637_d_n8;
        locals.var_temp2_dn9 = assign43860_e56637_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign43870_e56645, assign43870_e56645_d_n4, assign43870_e56645_d_n6, assign43870_e56645_d_n7, assign43870_e56645_d_n8, assign43870_e56645_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43870_e56643: f64 = (locals.var_temp2 * locals.var_asat);
        (assign43870_e56643, ((locals.var_temp2_dn4 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn4)), ((locals.var_temp2_dn6 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn6)), ((locals.var_temp2_dn7 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn7)), ((locals.var_temp2_dn8 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn8)), ((locals.var_temp2_dn9 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn9)),)
    } else {
        (locals.var_x_inf0, locals.var_x_inf0_dn4, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, locals.var_x_inf0_dn9,)
    }
};
        locals.var_x_inf0 = assign43870_e56645;
        locals.var_x_inf0_dn4 = assign43870_e56645_d_n4;
        locals.var_x_inf0_dn6 = assign43870_e56645_d_n6;
        locals.var_x_inf0_dn7 = assign43870_e56645_d_n7;
        locals.var_x_inf0_dn8 = assign43870_e56645_d_n8;
        locals.var_x_inf0_dn9 = assign43870_e56645_d_n9;
        locals.var_x_inf0_rv = 0.0;

        let assign43880_e56652: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign43880_e56652;
        locals.var_guard1218_rv = 0.0;

        let (assign43890_e56664, assign43890_e56664_d_n4, assign43890_e56664_d_n6, assign43890_e56664_d_n7, assign43890_e56664_d_n8, assign43890_e56664_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43890_e56660: f64 = (0.475 * locals.var_phit1);
        let assign43890_e56662: f64 = (assign43890_e56660 * locals.var_x_inf0);
        (assign43890_e56662, (((0.475 * locals.var_phit1_dn4) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn4)), (((0.475 * locals.var_phit1_dn6) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn6)), (((0.475 * locals.var_phit1_dn7) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn7)), (((0.475 * locals.var_phit1_dn8) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn8)), (((0.475 * locals.var_phit1_dn9) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn9)),)
    } else {
        (locals.var_midphi0, locals.var_midphi0_dn4, locals.var_midphi0_dn6, locals.var_midphi0_dn7, locals.var_midphi0_dn8, locals.var_midphi0_dn9,)
    }
};
        locals.var_midphi0 = assign43890_e56664;
        locals.var_midphi0_dn4 = assign43890_e56664_d_n4;
        locals.var_midphi0_dn6 = assign43890_e56664_d_n6;
        locals.var_midphi0_dn7 = assign43890_e56664_d_n7;
        locals.var_midphi0_dn8 = assign43890_e56664_d_n8;
        locals.var_midphi0_dn9 = assign43890_e56664_d_n9;
        locals.var_midphi0_rv = 0.0;

        let (assign43900_e56676, assign43900_e56676_d_n4, assign43900_e56676_d_n6, assign43900_e56676_d_n7, assign43900_e56676_d_n8, assign43900_e56676_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43900_e56673: f64 = (locals.var_alphas * locals.var_midphi0);
        let assign43900_e56674: f64 = (locals.var_qis - assign43900_e56673);
        (assign43900_e56674, (locals.var_qis_dn4 - ((locals.var_alphas_dn4 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn4))), (locals.var_qis_dn6 - ((locals.var_alphas_dn6 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn6))), (locals.var_qis_dn7 - ((locals.var_alphas_dn7 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn7))), (locals.var_qis_dn8 - ((locals.var_alphas_dn8 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn8))), (locals.var_qis_dn9 - ((locals.var_alphas_dn9 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn9))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43900_e56676;
        locals.var_temp__blk949_dn4 = assign43900_e56676_d_n4;
        locals.var_temp__blk949_dn6 = assign43900_e56676_d_n6;
        locals.var_temp__blk949_dn7 = assign43900_e56676_d_n7;
        locals.var_temp__blk949_dn8 = assign43900_e56676_d_n8;
        locals.var_temp__blk949_dn9 = assign43900_e56676_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign43910_e56693, assign43910_e56693_d_n4, assign43910_e56693_d_n6, assign43910_e56693_d_n7, assign43910_e56693_d_n8, assign43910_e56693_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43910_e56686: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign43910_e56688: f64 = (assign43910_e56686 + 1e-12);
        let assign43910_e56689: f64 = (assign43910_e56688).sqrt();
        let assign43910_e56690: f64 = (locals.var_temp__blk949 + assign43910_e56689);
        let assign43910_e56691: f64 = (0.5 * assign43910_e56690);
        (assign43910_e56691, (0.5 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign43910_e56689)))),)
    } else {
        (locals.var_qisat, locals.var_qisat_dn4, locals.var_qisat_dn6, locals.var_qisat_dn7, locals.var_qisat_dn8, locals.var_qisat_dn9,)
    }
};
        locals.var_qisat = assign43910_e56693;
        locals.var_qisat_dn4 = assign43910_e56693_d_n4;
        locals.var_qisat_dn6 = assign43910_e56693_d_n6;
        locals.var_qisat_dn7 = assign43910_e56693_d_n7;
        locals.var_qisat_dn8 = assign43910_e56693_d_n8;
        locals.var_qisat_dn9 = assign43910_e56693_d_n9;
        locals.var_qisat_rv = 0.0;

        let (assign43920_e56711, assign43920_e56711_d_n4, assign43920_e56711_d_n6, assign43920_e56711_d_n7, assign43920_e56711_d_n8, assign43920_e56711_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43920_e56701: f64 = (locals.var_phit1 * locals.var_xgs);
        let assign43920_e56703: f64 = (assign43920_e56701 - locals.var_qis);
        let assign43920_e56706: f64 = (locals.var_alphas - 1.0);
        let assign43920_e56708: f64 = (assign43920_e56706 * locals.var_midphi0);
        let assign43920_e56709: f64 = (assign43920_e56703 + assign43920_e56708);
        (assign43920_e56709, ((((locals.var_phit1_dn4 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn4)) - locals.var_qis_dn4) + ((locals.var_alphas_dn4 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn4))), ((((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6)) - locals.var_qis_dn6) + ((locals.var_alphas_dn6 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn6))), ((((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7)) - locals.var_qis_dn7) + ((locals.var_alphas_dn7 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn7))), ((((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8)) - locals.var_qis_dn8) + ((locals.var_alphas_dn8 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn8))), ((((locals.var_phit1_dn9 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn9)) - locals.var_qis_dn9) + ((locals.var_alphas_dn9 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn9))),)
    } else {
        (locals.var_qbsat, locals.var_qbsat_dn4, locals.var_qbsat_dn6, locals.var_qbsat_dn7, locals.var_qbsat_dn8, locals.var_qbsat_dn9,)
    }
};
        locals.var_qbsat = assign43920_e56711;
        locals.var_qbsat_dn4 = assign43920_e56711_d_n4;
        locals.var_qbsat_dn6 = assign43920_e56711_d_n6;
        locals.var_qbsat_dn7 = assign43920_e56711_d_n7;
        locals.var_qbsat_dn8 = assign43920_e56711_d_n8;
        locals.var_qbsat_dn9 = assign43920_e56711_d_n9;
        locals.var_qbsat_rv = 0.0;

        let (assign43930_e56727, assign43930_e56727_d_n4, assign43930_e56727_d_n6, assign43930_e56727_d_n7, assign43930_e56727_d_n8, assign43930_e56727_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43930_e56720: f64 = (0.5 * locals.var_gf2);
        let assign43930_e56722: f64 = (assign43930_e56720 * locals.var_phit1);
        let assign43930_e56724: f64 = (assign43930_e56722 / locals.var_qbsat);
        let assign43930_e56725: f64 = (1.0 + assign43930_e56724);
        (assign43930_e56725, ((((((0.5 * locals.var_gf2_dn4) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn4)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn6) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn6)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn7) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn7)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn8) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn8)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn9) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn9)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_alphasat, locals.var_alphasat_dn4, locals.var_alphasat_dn6, locals.var_alphasat_dn7, locals.var_alphasat_dn8, locals.var_alphasat_dn9,)
    }
};
        locals.var_alphasat = assign43930_e56727;
        locals.var_alphasat_dn4 = assign43930_e56727_d_n4;
        locals.var_alphasat_dn6 = assign43930_e56727_d_n6;
        locals.var_alphasat_dn7 = assign43930_e56727_d_n7;
        locals.var_alphasat_dn8 = assign43930_e56727_d_n8;
        locals.var_alphasat_dn9 = assign43930_e56727_d_n9;
        locals.var_alphasat_rv = 0.0;

        let (assign43940_e56739, assign43940_e56739_d_n4, assign43940_e56739_d_n6, assign43940_e56739_d_n7, assign43940_e56739_d_n8, assign43940_e56739_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43940_e56736: f64 = (locals.var_eta_mu * locals.var_qisat);
        let assign43940_e56737: f64 = (locals.var_qbsat + assign43940_e56736);
        (assign43940_e56737, (locals.var_qbsat_dn4 + (locals.var_eta_mu * locals.var_qisat_dn4)), (locals.var_qbsat_dn6 + (locals.var_eta_mu * locals.var_qisat_dn6)), (locals.var_qbsat_dn7 + (locals.var_eta_mu * locals.var_qisat_dn7)), (locals.var_qbsat_dn8 + (locals.var_eta_mu * locals.var_qisat_dn8)), (locals.var_qbsat_dn9 + (locals.var_eta_mu * locals.var_qisat_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43940_e56739;
        locals.var_temp__blk949_dn4 = assign43940_e56739_d_n4;
        locals.var_temp__blk949_dn6 = assign43940_e56739_d_n6;
        locals.var_temp__blk949_dn7 = assign43940_e56739_d_n7;
        locals.var_temp__blk949_dn8 = assign43940_e56739_d_n8;
        locals.var_temp__blk949_dn9 = assign43940_e56739_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign43950_e56753, assign43950_e56753_d_n4, assign43950_e56753_d_n6, assign43950_e56753_d_n7, assign43950_e56753_d_n8, assign43950_e56753_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43950_e56747: f64 = (locals.var_e_eff0 * locals.var_temp__blk949);
        let assign43950_e56749: f64 = (assign43950_e56747 * locals.var_mue_t);
        let assign43950_e56751: f64 = (assign43950_e56749).powf(locals.var_themu_t);
        (assign43950_e56751, if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * (((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign43950_e56747 * locals.var_mue_t_dn4)))) } } else { (assign43950_e56751 * ((locals.var_themu_t_dn4 * (assign43950_e56749).ln()) + (locals.var_themu_t * ((((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign43950_e56747 * locals.var_mue_t_dn4)) / assign43950_e56749)))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t) / assign43950_e56749))) },)
    } else {
        (locals.var_gmobmusat, locals.var_gmobmusat_dn4, locals.var_gmobmusat_dn6, locals.var_gmobmusat_dn7, locals.var_gmobmusat_dn8, locals.var_gmobmusat_dn9,)
    }
};
        locals.var_gmobmusat = assign43950_e56753;
        locals.var_gmobmusat_dn4 = assign43950_e56753_d_n4;
        locals.var_gmobmusat_dn6 = assign43950_e56753_d_n6;
        locals.var_gmobmusat_dn7 = assign43950_e56753_d_n7;
        locals.var_gmobmusat_dn8 = assign43950_e56753_d_n8;
        locals.var_gmobmusat_dn9 = assign43950_e56753_d_n9;
        locals.var_gmobmusat_rv = 0.0;

        let (assign43960_e56773, assign43960_e56773_d_n4, assign43960_e56773_d_n6, assign43960_e56773_d_n7, assign43960_e56773_d_n8, assign43960_e56773_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43960_e56763: f64 = (1.0 - locals.var_eta_mu);
        let assign43960_e56764: f64 = (locals.var_alphasat * assign43960_e56763);
        let assign43960_e56766: f64 = (assign43960_e56764 - 1.0);
        let assign43960_e56767: f64 = (locals.var_themu_t * assign43960_e56766);
        let assign43960_e56769: f64 = (assign43960_e56767 / locals.var_temp__blk949);
        let assign43960_e56771: f64 = (assign43960_e56769 * locals.var_gmobmusat);
        (assign43960_e56771, (((((((locals.var_themu_t_dn4 * assign43960_e56766) + (locals.var_themu_t * (locals.var_alphasat_dn4 * assign43960_e56763))) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn4)), ((((((locals.var_themu_t * (locals.var_alphasat_dn6 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat_dn7 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat_dn8 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn8)), ((((((locals.var_themu_t * (locals.var_alphasat_dn9 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43960_e56773;
        locals.var_temp1_dn4 = assign43960_e56773_d_n4;
        locals.var_temp1_dn6 = assign43960_e56773_d_n6;
        locals.var_temp1_dn7 = assign43960_e56773_d_n7;
        locals.var_temp1_dn8 = assign43960_e56773_d_n8;
        locals.var_temp1_dn9 = assign43960_e56773_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign43970_e56783, assign43970_e56783_d_n4, assign43970_e56783_d_n6, assign43970_e56783_d_n7, assign43970_e56783_d_n8, assign43970_e56783_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43970_e56781: f64 = (locals.var_qisat / locals.var_qbsat);
        (assign43970_e56781, (((locals.var_qisat_dn4 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn6 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn7 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn8 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn9 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43970_e56783;
        locals.var_temp__blk949_dn4 = assign43970_e56783_d_n4;
        locals.var_temp__blk949_dn6 = assign43970_e56783_d_n6;
        locals.var_temp__blk949_dn7 = assign43970_e56783_d_n7;
        locals.var_temp__blk949_dn8 = assign43970_e56783_d_n8;
        locals.var_temp__blk949_dn9 = assign43970_e56783_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign43980_e56798, assign43980_e56798_d_n4, assign43980_e56798_d_n6, assign43980_e56798_d_n7, assign43980_e56798_d_n8, assign43980_e56798_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43980_e56792: f64 = (1.0 + locals.var_temp__blk949);
        let assign43980_e56794: f64 = (-locals.var_thecs_t);
        let assign43980_e56795: f64 = (assign43980_e56792).powf(assign43980_e56794);
        let assign43980_e56796: f64 = (locals.var_cs_t * assign43980_e56795);
        (assign43980_e56796, ((locals.var_cs_t_dn4 * assign43980_e56795) + (locals.var_cs_t * if (-locals.var_thecs_t_dn4) == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn4)) } } else { (assign43980_e56795 * (((-locals.var_thecs_t_dn4) * (assign43980_e56792).ln()) + (assign43980_e56794 * (locals.var_temp__blk949_dn4 / assign43980_e56792)))) })), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn6)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn6 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn7)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn7 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn8)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn8 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn9)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn9 / assign43980_e56792))) }),)
    } else {
        (locals.var_gmobcssat, locals.var_gmobcssat_dn4, locals.var_gmobcssat_dn6, locals.var_gmobcssat_dn7, locals.var_gmobcssat_dn8, locals.var_gmobcssat_dn9,)
    }
};
        locals.var_gmobcssat = assign43980_e56798;
        locals.var_gmobcssat_dn4 = assign43980_e56798_d_n4;
        locals.var_gmobcssat_dn6 = assign43980_e56798_d_n6;
        locals.var_gmobcssat_dn7 = assign43980_e56798_d_n7;
        locals.var_gmobcssat_dn8 = assign43980_e56798_d_n8;
        locals.var_gmobcssat_dn9 = assign43980_e56798_d_n9;
        locals.var_gmobcssat_rv = 0.0;

        let (assign43990_e56820, assign43990_e56820_d_n4, assign43990_e56820_d_n6, assign43990_e56820_d_n7, assign43990_e56820_d_n8, assign43990_e56820_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43990_e56807: f64 = (locals.var_alphasat - 1.0);
        let assign43990_e56811: f64 = (locals.var_temp__blk949 + 1.0);
        let assign43990_e56812: f64 = (1.0 / assign43990_e56811);
        let assign43990_e56813: f64 = (assign43990_e56807 + assign43990_e56812);
        let assign43990_e56814: f64 = (locals.var_thecs_t * assign43990_e56813);
        let assign43990_e56816: f64 = (assign43990_e56814 / locals.var_qbsat);
        let assign43990_e56818: f64 = (assign43990_e56816 * locals.var_gmobcssat);
        (assign43990_e56818, (((((((locals.var_thecs_t_dn4 * assign43990_e56813) + (locals.var_thecs_t * (locals.var_alphasat_dn4 + (-(locals.var_temp__blk949_dn4 / (assign43990_e56811 * assign43990_e56811)))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn4)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn6 + (-(locals.var_temp__blk949_dn6 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn7 + (-(locals.var_temp__blk949_dn7 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn8 + (-(locals.var_temp__blk949_dn8 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn8)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn9 + (-(locals.var_temp__blk949_dn9 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43990_e56820;
        locals.var_temp2_dn4 = assign43990_e56820_d_n4;
        locals.var_temp2_dn6 = assign43990_e56820_d_n6;
        locals.var_temp2_dn7 = assign43990_e56820_d_n7;
        locals.var_temp2_dn8 = assign43990_e56820_d_n8;
        locals.var_temp2_dn9 = assign43990_e56820_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign44000_e56834, assign44000_e56834_d_n4, assign44000_e56834_d_n6, assign44000_e56834_d_n7, assign44000_e56834_d_n8, assign44000_e56834_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44000_e56828: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign44000_e56830: f64 = (assign44000_e56828 * locals.var_rhog);
        let assign44000_e56832: f64 = (assign44000_e56830 * locals.var_qisat);
        (assign44000_e56832, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn4)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn6)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn7)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn8)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn9)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn9)),)
    } else {
        (locals.var_grsat, locals.var_grsat_dn4, locals.var_grsat_dn6, locals.var_grsat_dn7, locals.var_grsat_dn8, locals.var_grsat_dn9,)
    }
};
        locals.var_grsat = assign44000_e56834;
        locals.var_grsat_dn4 = assign44000_e56834_d_n4;
        locals.var_grsat_dn6 = assign44000_e56834_d_n6;
        locals.var_grsat_dn7 = assign44000_e56834_d_n7;
        locals.var_grsat_dn8 = assign44000_e56834_d_n8;
        locals.var_grsat_dn9 = assign44000_e56834_d_n9;
        locals.var_grsat_rv = 0.0;

        let (assign44010_e56854, assign44010_e56854_d_n4, assign44010_e56854_d_n6, assign44010_e56854_d_n7, assign44010_e56854_d_n8, assign44010_e56854_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44010_e56844: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign44010_e56846: f64 = (assign44010_e56844 * locals.var_rhog);
        let assign44010_e56848: f64 = (assign44010_e56846 * locals.var_alphasat);
        let assign44010_e56849: f64 = (locals.var_temp1 - assign44010_e56848);
        let assign44010_e56851: f64 = (assign44010_e56849 / locals.var_temp2);
        let assign44010_e56852: f64 = (1.0 + assign44010_e56851);
        (assign44010_e56852, ((((locals.var_temp1_dn4 - ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn4)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn4))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn6)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn6))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn7)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn7))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn8)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn8))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn9 - (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn9)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn9))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44010_e56854;
        locals.var_temp__blk949_dn4 = assign44010_e56854_d_n4;
        locals.var_temp__blk949_dn6 = assign44010_e56854_d_n6;
        locals.var_temp__blk949_dn7 = assign44010_e56854_d_n7;
        locals.var_temp__blk949_dn8 = assign44010_e56854_d_n8;
        locals.var_temp__blk949_dn9 = assign44010_e56854_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign44020_e56857: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign44020_e56857;
        locals.var_guard1219_rv = 0.0;

        let (assign44030_e56875, assign44030_e56875_d_n4, assign44030_e56875_d_n6, assign44030_e56875_d_n7, assign44030_e56875_d_n8, assign44030_e56875_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) && (locals.var_guard1219 != 0.0)) {
        let assign44030_e56869: f64 = (2.0 * locals.var_temp__blk949);
        let assign44030_e56870: f64 = (assign44030_e56869).exp();
        let assign44030_e56871: f64 = (1.0 + assign44030_e56870);
        let assign44030_e56872: f64 = (assign44030_e56871).ln();
        let assign44030_e56873: f64 = (0.5 * assign44030_e56872);
        (assign44030_e56873, (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn4)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn6)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn7)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn8)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn9)) / assign44030_e56871)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44030_e56875;
        locals.var_temp1_dn4 = assign44030_e56875_d_n4;
        locals.var_temp1_dn6 = assign44030_e56875_d_n6;
        locals.var_temp1_dn7 = assign44030_e56875_d_n7;
        locals.var_temp1_dn8 = assign44030_e56875_d_n8;
        locals.var_temp1_dn9 = assign44030_e56875_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign44040_e56886, assign44040_e56886_d_n4, assign44040_e56886_d_n6, assign44040_e56886_d_n7, assign44040_e56886_d_n8, assign44040_e56886_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) && (locals.var_guard1219 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44040_e56886;
        locals.var_temp1_dn4 = assign44040_e56886_d_n4;
        locals.var_temp1_dn6 = assign44040_e56886_d_n6;
        locals.var_temp1_dn7 = assign44040_e56886_d_n7;
        locals.var_temp1_dn8 = assign44040_e56886_d_n8;
        locals.var_temp1_dn9 = assign44040_e56886_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign44050_e56907, assign44050_e56907_d_n4, assign44050_e56907_d_n6, assign44050_e56907_d_n7, assign44050_e56907_d_n8, assign44050_e56907_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44050_e56893: f64 = (-locals.var_midphi0);
        let assign44050_e56895: f64 = (assign44050_e56893 * locals.var_temp2);
        let assign44050_e56897: f64 = (assign44050_e56895 * locals.var_temp1);
        let assign44050_e56900: f64 = (1.0 + locals.var_gmobmusat);
        let assign44050_e56902: f64 = (assign44050_e56900 + locals.var_gmobcssat);
        let assign44050_e56904: f64 = (assign44050_e56902 + locals.var_grsat);
        let assign44050_e56905: f64 = (assign44050_e56897 / assign44050_e56904);
        (assign44050_e56905, ((((((((-locals.var_midphi0_dn4) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn4)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn4)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn4 + locals.var_gmobcssat_dn4) + locals.var_grsat_dn4))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn6) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn6)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn6 + locals.var_gmobcssat_dn6) + locals.var_grsat_dn6))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn7) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn7)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn7 + locals.var_gmobcssat_dn7) + locals.var_grsat_dn7))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn8) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn8)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn8 + locals.var_gmobcssat_dn8) + locals.var_grsat_dn8))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn9) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn9)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn9)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn9 + locals.var_gmobcssat_dn9) + locals.var_grsat_dn9))) / (assign44050_e56904 * assign44050_e56904)),)
    } else {
        (locals.var_delta_gmob, locals.var_delta_gmob_dn4, locals.var_delta_gmob_dn6, locals.var_delta_gmob_dn7, locals.var_delta_gmob_dn8, locals.var_delta_gmob_dn9,)
    }
};
        locals.var_delta_gmob = assign44050_e56907;
        locals.var_delta_gmob_dn4 = assign44050_e56907_d_n4;
        locals.var_delta_gmob_dn6 = assign44050_e56907_d_n6;
        locals.var_delta_gmob_dn7 = assign44050_e56907_d_n7;
        locals.var_delta_gmob_dn8 = assign44050_e56907_d_n8;
        locals.var_delta_gmob_dn9 = assign44050_e56907_d_n9;
        locals.var_delta_gmob_rv = 0.0;

        let (assign44060_e56928, assign44060_e56928_d_n4, assign44060_e56928_d_n6, assign44060_e56928_d_n7, assign44060_e56928_d_n8, assign44060_e56928_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44060_e56920: f64 = (locals.var_delta_gmob * locals.var_delta_gmob);
        let assign44060_e56921: f64 = (1.0 + assign44060_e56920);
        let assign44060_e56922: f64 = (assign44060_e56921).sqrt();
        let assign44060_e56923: f64 = (1.0 + assign44060_e56922);
        let assign44060_e56924: f64 = (locals.var_delta_gmob / assign44060_e56923);
        let assign44060_e56925: f64 = (1.0 + assign44060_e56924);
        let assign44060_e56926: f64 = (locals.var_x_inf0 * assign44060_e56925);
        (assign44060_e56926, ((locals.var_x_inf0_dn4 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn4 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn4 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn4)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn6 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn6 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn6 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn6)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn7 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn7 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn7 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn7)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn8 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn8 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn8 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn8)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn9 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn9 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn9 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn9)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))),)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn4, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, locals.var_x_inf_dn9,)
    }
};
        locals.var_x_inf = assign44060_e56928;
        locals.var_x_inf_dn4 = assign44060_e56928_d_n4;
        locals.var_x_inf_dn6 = assign44060_e56928_d_n6;
        locals.var_x_inf_dn7 = assign44060_e56928_d_n7;
        locals.var_x_inf_dn8 = assign44060_e56928_d_n8;
        locals.var_x_inf_dn9 = assign44060_e56928_d_n9;
        locals.var_x_inf_rv = 0.0;

        let (assign44070_e56937, assign44070_e56937_d_n4, assign44070_e56937_d_n6, assign44070_e56937_d_n7, assign44070_e56937_d_n8, assign44070_e56937_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        (locals.var_x_inf0, locals.var_x_inf0_dn4, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, locals.var_x_inf0_dn9,)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn4, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, locals.var_x_inf_dn9,)
    }
};
        locals.var_x_inf = assign44070_e56937;
        locals.var_x_inf_dn4 = assign44070_e56937_d_n4;
        locals.var_x_inf_dn6 = assign44070_e56937_d_n6;
        locals.var_x_inf_dn7 = assign44070_e56937_d_n7;
        locals.var_x_inf_dn8 = assign44070_e56937_d_n8;
        locals.var_x_inf_dn9 = assign44070_e56937_d_n9;
        locals.var_x_inf_rv = 0.0;

        let (assign44080_e56949, assign44080_e56949_d_n4, assign44080_e56949_d_n6, assign44080_e56949_d_n7, assign44080_e56949_d_n8, assign44080_e56949_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44080_e56943: f64 = (locals.var_phit1 * locals.var_thesat1);
        let assign44080_e56945: f64 = (assign44080_e56943 * locals.var_x_inf);
        let assign44080_e56947: f64 = (assign44080_e56945 * 0.7071067811865475);
        (assign44080_e56947, (((((locals.var_phit1_dn4 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn4)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn4)) * 0.7071067811865475), (((((locals.var_phit1_dn6 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn6)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn6)) * 0.7071067811865475), (((((locals.var_phit1_dn7 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn7)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn7)) * 0.7071067811865475), (((((locals.var_phit1_dn8 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn8)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn8)) * 0.7071067811865475), (((((locals.var_phit1_dn9 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn9)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn9)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn4, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, locals.var_ysat_dn9,)
    }
};
        locals.var_ysat = assign44080_e56949;
        locals.var_ysat_dn4 = assign44080_e56949_d_n4;
        locals.var_ysat_dn6 = assign44080_e56949_d_n6;
        locals.var_ysat_dn7 = assign44080_e56949_d_n7;
        locals.var_ysat_dn8 = assign44080_e56949_d_n8;
        locals.var_ysat_dn9 = assign44080_e56949_d_n9;
        locals.var_ysat_rv = 0.0;

        let assign44090_e56952: f64 = (-1.0);
        let assign44090_e56953: f64 = if locals.var_chnl_type == assign44090_e56952 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign44090_e56953;
        locals.var_guard1220_rv = 0.0;

        let (assign44100_e56966, assign44100_e56966_d_n4, assign44100_e56966_d_n6, assign44100_e56966_d_n7, assign44100_e56966_d_n8, assign44100_e56966_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1220 != 0.0)) {
        let assign44100_e56962: f64 = (1.0 + locals.var_ysat);
        let assign44100_e56963: f64 = (assign44100_e56962).sqrt();
        let assign44100_e56964: f64 = (locals.var_ysat / assign44100_e56963);
        (assign44100_e56964, (((locals.var_ysat_dn4 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn4 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn6 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn6 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn7 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn7 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn8 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn8 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn9 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn9 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn4, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, locals.var_ysat_dn9,)
    }
};
        locals.var_ysat = assign44100_e56966;
        locals.var_ysat_dn4 = assign44100_e56966_d_n4;
        locals.var_ysat_dn6 = assign44100_e56966_d_n6;
        locals.var_ysat_dn7 = assign44100_e56966_d_n7;
        locals.var_ysat_dn8 = assign44100_e56966_d_n8;
        locals.var_ysat_dn9 = assign44100_e56966_d_n9;
        locals.var_ysat_rv = 0.0;

        let (assign44110_e56981, assign44110_e56981_d_n4, assign44110_e56981_d_n6, assign44110_e56981_d_n7, assign44110_e56981_d_n8, assign44110_e56981_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44110_e56975: f64 = (4.0 * locals.var_ysat);
        let assign44110_e56976: f64 = (1.0 + assign44110_e56975);
        let assign44110_e56977: f64 = (assign44110_e56976).sqrt();
        let assign44110_e56978: f64 = (1.0 + assign44110_e56977);
        let assign44110_e56979: f64 = (2.0 / assign44110_e56978);
        (assign44110_e56979, (-((2.0 * ((4.0 * locals.var_ysat_dn4) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn6) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn7) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn8) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn9) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))),)
    } else {
        (locals.var_za, locals.var_za_dn4, locals.var_za_dn6, locals.var_za_dn7, locals.var_za_dn8, locals.var_za_dn9,)
    }
};
        locals.var_za = assign44110_e56981;
        locals.var_za_dn4 = assign44110_e56981_d_n4;
        locals.var_za_dn6 = assign44110_e56981_d_n6;
        locals.var_za_dn7 = assign44110_e56981_d_n7;
        locals.var_za_dn8 = assign44110_e56981_d_n8;
        locals.var_za_dn9 = assign44110_e56981_d_n9;
        locals.var_za_rv = 0.0;

        let (assign44120_e56989, assign44120_e56989_d_n4, assign44120_e56989_d_n6, assign44120_e56989_d_n7, assign44120_e56989_d_n8, assign44120_e56989_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44120_e56987: f64 = (locals.var_za * locals.var_ysat);
        (assign44120_e56987, ((locals.var_za_dn4 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn4)), ((locals.var_za_dn6 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn6)), ((locals.var_za_dn7 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn7)), ((locals.var_za_dn8 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn8)), ((locals.var_za_dn9 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44120_e56989;
        locals.var_temp__blk949_dn4 = assign44120_e56989_d_n4;
        locals.var_temp__blk949_dn6 = assign44120_e56989_d_n6;
        locals.var_temp__blk949_dn7 = assign44120_e56989_d_n7;
        locals.var_temp__blk949_dn8 = assign44120_e56989_d_n8;
        locals.var_temp__blk949_dn9 = assign44120_e56989_d_n9;
        locals.var_temp__blk949_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign44130_e57019, assign44130_e57019_d_n4, assign44130_e57019_d_n6, assign44130_e57019_d_n7, assign44130_e57019_d_n8, assign44130_e57019_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44130_e56995: f64 = (locals.var_x_inf * locals.var_za);
        let assign44130_e56999: f64 = (0.86 * locals.var_temp__blk949);
        let assign44130_e57003: f64 = (locals.var_temp__blk949 * locals.var_za);
        let assign44130_e57004: f64 = (1.0 - assign44130_e57003);
        let assign44130_e57005: f64 = (assign44130_e56999 * assign44130_e57004);
        let assign44130_e57009: f64 = (4.0 * locals.var_temp__blk949);
        let assign44130_e57011: f64 = (assign44130_e57009 * locals.var_temp__blk949);
        let assign44130_e57013: f64 = (assign44130_e57011 * locals.var_za);
        let assign44130_e57014: f64 = (1.0 + assign44130_e57013);
        let assign44130_e57015: f64 = (assign44130_e57005 / assign44130_e57014);
        let assign44130_e57016: f64 = (1.0 + assign44130_e57015);
        let assign44130_e57017: f64 = (assign44130_e56995 * assign44130_e57016);
        (assign44130_e57017, ((((locals.var_x_inf_dn4 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn4)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn4) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn4 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn4))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn4)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn4)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn6 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn6)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn6) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn6 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn6))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn6)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn6)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn7 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn7)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn7) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn7 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn7))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn7)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn7)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn8 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn8)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn8) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn8 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn8))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn8)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn8)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn9 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn9)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn9) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn9 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn9))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn9)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn9)))) / (assign44130_e57014 * assign44130_e57014)))),)
    } else {
        (locals.var_x_0, locals.var_x_0_dn4, locals.var_x_0_dn6, locals.var_x_0_dn7, locals.var_x_0_dn8, locals.var_x_0_dn9,)
    }
};
        locals.var_x_0 = assign44130_e57019;
        locals.var_x_0_dn4 = assign44130_e57019_d_n4;
        locals.var_x_0_dn6 = assign44130_e57019_d_n6;
        locals.var_x_0_dn7 = assign44130_e57019_d_n7;
        locals.var_x_0_dn8 = assign44130_e57019_d_n8;
        locals.var_x_0_dn9 = assign44130_e57019_d_n9;
        locals.var_x_0_rv = 0.0;

        let (assign44140_e57027, assign44140_e57027_d_n4, assign44140_e57027_d_n6, assign44140_e57027_d_n7, assign44140_e57027_d_n8, assign44140_e57027_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44140_e57025: f64 = (0.99 * locals.var_x_0);
        (assign44140_e57025, (0.99 * locals.var_x_0_dn4), (0.99 * locals.var_x_0_dn6), (0.99 * locals.var_x_0_dn7), (0.99 * locals.var_x_0_dn8), (0.99 * locals.var_x_0_dn9),)
    } else {
        (locals.var_x_sat, locals.var_x_sat_dn4, locals.var_x_sat_dn6, locals.var_x_sat_dn7, locals.var_x_sat_dn8, locals.var_x_sat_dn9,)
    }
};
        locals.var_x_sat = assign44140_e57027;
        locals.var_x_sat_dn4 = assign44140_e57027_d_n4;
        locals.var_x_sat_dn6 = assign44140_e57027_d_n6;
        locals.var_x_sat_dn7 = assign44140_e57027_d_n7;
        locals.var_x_sat_dn8 = assign44140_e57027_d_n8;
        locals.var_x_sat_dn9 = assign44140_e57027_d_n9;
        locals.var_x_sat_rv = 0.0;

        let (assign44150_e57043, assign44150_e57043_d_n4, assign44150_e57043_d_n6, assign44150_e57043_d_n7, assign44150_e57043_d_n8, assign44150_e57043_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44150_e57035: f64 = (2.0 * locals.var_asat);
        let assign44150_e57036: f64 = (locals.var_x_sat - assign44150_e57035);
        let assign44150_e57037: f64 = (locals.var_x_sat * assign44150_e57036);
        let assign44150_e57039: f64 = (assign44150_e57037 * locals.var_inv_gf2);
        let assign44150_e57041: f64 = (assign44150_e57039 / locals.var_ds);
        (assign44150_e57041, (((((((locals.var_x_sat_dn4 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn4 - (2.0 * locals.var_asat_dn4)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn4)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn4)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn6 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn6 - (2.0 * locals.var_asat_dn6)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn6)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn7 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn7 - (2.0 * locals.var_asat_dn7)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn7)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn8 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn8 - (2.0 * locals.var_asat_dn8)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn8)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn9 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn9 - (2.0 * locals.var_asat_dn9)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn9)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn9)) / (locals.var_ds * locals.var_ds)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44150_e57043;
        locals.var_temp__blk949_dn4 = assign44150_e57043_d_n4;
        locals.var_temp__blk949_dn6 = assign44150_e57043_d_n6;
        locals.var_temp__blk949_dn7 = assign44150_e57043_d_n7;
        locals.var_temp__blk949_dn8 = assign44150_e57043_d_n8;
        locals.var_temp__blk949_dn9 = assign44150_e57043_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44160_e57063, assign44160_e57063_d_n4, assign44160_e57063_d_n6, assign44160_e57063_d_n7, assign44160_e57063_d_n8, assign44160_e57063_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44160_e57052: f64 = (-0.99);
        let (assign44160_e57057, assign44160_e57057_d_n4, assign44160_e57057_d_n6, assign44160_e57057_d_n7, assign44160_e57057_d_n8, assign44160_e57057_d_n9,) = {
            if (locals.var_temp__blk949 > assign44160_e57052) {
                (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
            } else {
                let assign44160_e57056: f64 = (-0.99);
                (assign44160_e57056, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign44160_e57058: f64 = (1.0 + assign44160_e57057);
        let assign44160_e57059: f64 = (assign44160_e57058).ln();
        let assign44160_e57060: f64 = (locals.var_x_sat - assign44160_e57059);
        let assign44160_e57061: f64 = (locals.var_phit1 * assign44160_e57060);
        (assign44160_e57061, ((locals.var_phit1_dn4 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn4 - (assign44160_e57057_d_n4 / assign44160_e57058)))), ((locals.var_phit1_dn6 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn6 - (assign44160_e57057_d_n6 / assign44160_e57058)))), ((locals.var_phit1_dn7 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn7 - (assign44160_e57057_d_n7 / assign44160_e57058)))), ((locals.var_phit1_dn8 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn8 - (assign44160_e57057_d_n8 / assign44160_e57058)))), ((locals.var_phit1_dn9 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn9 - (assign44160_e57057_d_n9 / assign44160_e57058)))),)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn4, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, locals.var_v_dsat_dn9,)
    }
};
        locals.var_v_dsat = assign44160_e57063;
        locals.var_v_dsat_dn4 = assign44160_e57063_d_n4;
        locals.var_v_dsat_dn6 = assign44160_e57063_d_n6;
        locals.var_v_dsat_dn7 = assign44160_e57063_d_n7;
        locals.var_v_dsat_dn8 = assign44160_e57063_d_n8;
        locals.var_v_dsat_dn9 = assign44160_e57063_d_n9;
        locals.var_v_dsat_rv = 0.0;

        let (assign44170_e57070, assign44170_e57070_d_n4, assign44170_e57070_d_n6, assign44170_e57070_d_n7, assign44170_e57070_d_n8, assign44170_e57070_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 == 0.0)) {
        (locals.var_vdsat_lim, locals.var_vdsat_lim_dn4, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, locals.var_vdsat_lim_dn9,)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn4, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, locals.var_v_dsat_dn9,)
    }
};
        locals.var_v_dsat = assign44170_e57070;
        locals.var_v_dsat_dn4 = assign44170_e57070_d_n4;
        locals.var_v_dsat_dn6 = assign44170_e57070_d_n6;
        locals.var_v_dsat_dn7 = assign44170_e57070_d_n7;
        locals.var_v_dsat_dn8 = assign44170_e57070_d_n8;
        locals.var_v_dsat_dn9 = assign44170_e57070_d_n9;
        locals.var_v_dsat_rv = 0.0;

        let (assign44180_e57076, assign44180_e57076_d_n4, assign44180_e57076_d_n6, assign44180_e57076_d_n7, assign44180_e57076_d_n8, assign44180_e57076_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44180_e57074: f64 = (1.0 + locals.var_arloc);
        (assign44180_e57074, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44180_e57076;
        locals.var_temp__blk949_dn4 = assign44180_e57076_d_n4;
        locals.var_temp__blk949_dn6 = assign44180_e57076_d_n6;
        locals.var_temp__blk949_dn7 = assign44180_e57076_d_n7;
        locals.var_temp__blk949_dn8 = assign44180_e57076_d_n8;
        locals.var_temp__blk949_dn9 = assign44180_e57076_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44190_e57085, assign44190_e57085_d_n4, assign44190_e57085_d_n6, assign44190_e57085_d_n7, assign44190_e57085_d_n8, assign44190_e57085_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44190_e57079: f64 = (locals.var_temp__blk949).sqrt();
        let assign44190_e57081: f64 = (assign44190_e57079 * locals.var_v_ds);
        let assign44190_e57083: f64 = (assign44190_e57081 / locals.var_v_dsat);
        (assign44190_e57083, (((((locals.var_temp__blk949_dn4 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn4)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn6)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign44190_e57079)) * locals.var_v_ds) + (assign44190_e57079 * locals.var_v_ds_dn7)) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn7)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign44190_e57079)) * locals.var_v_ds) + (assign44190_e57079 * locals.var_v_ds_dn8)) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn8)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn9)) / (locals.var_v_dsat * locals.var_v_dsat)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44190_e57085;
        locals.var_temp1_dn4 = assign44190_e57085_d_n4;
        locals.var_temp1_dn6 = assign44190_e57085_d_n6;
        locals.var_temp1_dn7 = assign44190_e57085_d_n7;
        locals.var_temp1_dn8 = assign44190_e57085_d_n8;
        locals.var_temp1_dn9 = assign44190_e57085_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign44200_e57093, assign44200_e57093_d_n4, assign44200_e57093_d_n6, assign44200_e57093_d_n7, assign44200_e57093_d_n8, assign44200_e57093_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44200_e57089: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign44200_e57091: f64 = (assign44200_e57089 + locals.var_temp__blk949);
        (assign44200_e57091, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign44200_e57093;
        locals.var_temp2_dn4 = assign44200_e57093_d_n4;
        locals.var_temp2_dn6 = assign44200_e57093_d_n6;
        locals.var_temp2_dn7 = assign44200_e57093_d_n7;
        locals.var_temp2_dn8 = assign44200_e57093_d_n8;
        locals.var_temp2_dn9 = assign44200_e57093_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign44210_e57099, assign44210_e57099_d_n4, assign44210_e57099_d_n6, assign44210_e57099_d_n7, assign44210_e57099_d_n8, assign44210_e57099_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44210_e57097: f64 = (2.0 * locals.var_temp1);
        (assign44210_e57097, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44210_e57099;
        locals.var_temp__blk949_dn4 = assign44210_e57099_d_n4;
        locals.var_temp__blk949_dn6 = assign44210_e57099_d_n6;
        locals.var_temp__blk949_dn7 = assign44210_e57099_d_n7;
        locals.var_temp__blk949_dn8 = assign44210_e57099_d_n8;
        locals.var_temp__blk949_dn9 = assign44210_e57099_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44220_e57115, assign44220_e57115_d_n4, assign44220_e57115_d_n6, assign44220_e57115_d_n7, assign44220_e57115_d_n8, assign44220_e57115_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44220_e57103: f64 = (locals.var_v_dsat * locals.var_temp__blk949);
        let assign44220_e57106: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign44220_e57107: f64 = (assign44220_e57106).sqrt();
        let assign44220_e57110: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign44220_e57111: f64 = (assign44220_e57110).sqrt();
        let assign44220_e57112: f64 = (assign44220_e57107 + assign44220_e57111);
        let assign44220_e57113: f64 = (assign44220_e57103 / assign44220_e57112);
        (assign44220_e57113, (((((locals.var_v_dsat_dn4 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn4)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn6 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn6)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn7 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn7)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn8 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn8)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn9 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn9)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn4, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8, locals.var_vdse_dn9,)
    }
};
        locals.var_vdse = assign44220_e57115;
        locals.var_vdse_dn4 = assign44220_e57115_d_n4;
        locals.var_vdse_dn6 = assign44220_e57115_d_n6;
        locals.var_vdse_dn7 = assign44220_e57115_d_n7;
        locals.var_vdse_dn8 = assign44220_e57115_d_n8;
        locals.var_vdse_dn9 = assign44220_e57115_d_n9;
        locals.var_vdse_rv = 0.0;

        let (assign44230_e57121, assign44230_e57121_d_n4, assign44230_e57121_d_n6, assign44230_e57121_d_n7, assign44230_e57121_d_n8, assign44230_e57121_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44230_e57119: f64 = (locals.var_vdse * locals.var_inv_phit1);
        (assign44230_e57119, ((locals.var_vdse_dn4 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn4)), ((locals.var_vdse_dn6 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn6)), ((locals.var_vdse_dn7 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn7)), ((locals.var_vdse_dn8 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn8)), ((locals.var_vdse_dn9 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn9)),)
    } else {
        (locals.var_udse, locals.var_udse_dn4, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, locals.var_udse_dn9,)
    }
};
        locals.var_udse = assign44230_e57121;
        locals.var_udse_dn4 = assign44230_e57121_d_n4;
        locals.var_udse_dn6 = assign44230_e57121_d_n6;
        locals.var_udse_dn7 = assign44230_e57121_d_n7;
        locals.var_udse_dn8 = assign44230_e57121_d_n8;
        locals.var_udse_dn9 = assign44230_e57121_d_n9;
        locals.var_udse_rv = 0.0;

        let (assign44240_e57127, assign44240_e57127_d_n4, assign44240_e57127_d_n6, assign44240_e57127_d_n7, assign44240_e57127_d_n8, assign44240_e57127_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44240_e57125: f64 = (locals.var_xn_s + locals.var_udse);
        (assign44240_e57125, (locals.var_xn_s_dn4 + locals.var_udse_dn4), (locals.var_xn_s_dn6 + locals.var_udse_dn6), (locals.var_xn_s_dn7 + locals.var_udse_dn7), (locals.var_xn_s_dn8 + locals.var_udse_dn8), (locals.var_xn_s_dn9 + locals.var_udse_dn9),)
    } else {
        (locals.var_xn_d, locals.var_xn_d_dn4, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, locals.var_xn_d_dn9,)
    }
};
        locals.var_xn_d = assign44240_e57127;
        locals.var_xn_d_dn4 = assign44240_e57127_d_n4;
        locals.var_xn_d_dn6 = assign44240_e57127_d_n6;
        locals.var_xn_d_dn7 = assign44240_e57127_d_n7;
        locals.var_xn_d_dn8 = assign44240_e57127_d_n8;
        locals.var_xn_d_dn9 = assign44240_e57127_d_n9;
        locals.var_xn_d_rv = 0.0;

        let assign44250_e57130: f64 = if locals.var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign44250_e57130;
        locals.var_guard1221_rv = 0.0;

        let (assign44260_e57138, assign44260_e57138_d_n4, assign44260_e57138_d_n6, assign44260_e57138_d_n7, assign44260_e57138_d_n8, assign44260_e57138_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1221 != 0.0)) {
        let assign44260_e57135: f64 = (-locals.var_udse);
        let assign44260_e57136: f64 = (assign44260_e57135).exp();
        (assign44260_e57136, (assign44260_e57136 * (-locals.var_udse_dn4)), (assign44260_e57136 * (-locals.var_udse_dn6)), (assign44260_e57136 * (-locals.var_udse_dn7)), (assign44260_e57136 * (-locals.var_udse_dn8)), (assign44260_e57136 * (-locals.var_udse_dn9)),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn4, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, locals.var_k_ds_dn9,)
    }
};
        locals.var_k_ds = assign44260_e57138;
        locals.var_k_ds_dn4 = assign44260_e57138_d_n4;
        locals.var_k_ds_dn6 = assign44260_e57138_d_n6;
        locals.var_k_ds_dn7 = assign44260_e57138_d_n7;
        locals.var_k_ds_dn8 = assign44260_e57138_d_n8;
        locals.var_k_ds_dn9 = assign44260_e57138_d_n9;
        locals.var_k_ds_rv = 0.0;

        let (assign44270_e57167, assign44270_e57167_d_n4, assign44270_e57167_d_n6, assign44270_e57167_d_n7, assign44270_e57167_d_n8, assign44270_e57167_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1221 == 0.0)) {
        let assign44270_e57147: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57152: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57156: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57158: f64 = (assign44270_e57156 * 0.3333333333333333);
        let assign44270_e57159: f64 = (1.0 + assign44270_e57158);
        let assign44270_e57160: f64 = (assign44270_e57152 * assign44270_e57159);
        let assign44270_e57161: f64 = (0.5 * assign44270_e57160);
        let assign44270_e57162: f64 = (1.0 + assign44270_e57161);
        let assign44270_e57163: f64 = (assign44270_e57147 * assign44270_e57162);
        let assign44270_e57164: f64 = (1.0 + assign44270_e57163);
        let assign44270_e57165: f64 = (1e-200 / assign44270_e57164);
        (assign44270_e57165, (-((1e-200 * ((locals.var_udse_dn4 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn4 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn4 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn6 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn6 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn6 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn7 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn7 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn7 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn8 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn8 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn8 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn9 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn9 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn9 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn4, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, locals.var_k_ds_dn9,)
    }
};
        locals.var_k_ds = assign44270_e57167;
        locals.var_k_ds_dn4 = assign44270_e57167_d_n4;
        locals.var_k_ds_dn6 = assign44270_e57167_d_n6;
        locals.var_k_ds_dn7 = assign44270_e57167_d_n7;
        locals.var_k_ds_dn8 = assign44270_e57167_d_n8;
        locals.var_k_ds_dn9 = assign44270_e57167_d_n9;
        locals.var_k_ds_rv = 0.0;

        let (assign44280_e57173, assign44280_e57173_d_n4, assign44280_e57173_d_n6, assign44280_e57173_d_n7, assign44280_e57173_d_n8, assign44280_e57173_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44280_e57171: f64 = (locals.var_delta_ns * locals.var_k_ds);
        (assign44280_e57171, ((locals.var_delta_ns_dn4 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn4)), ((locals.var_delta_ns_dn6 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn6)), ((locals.var_delta_ns_dn7 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn7)), ((locals.var_delta_ns_dn8 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn8)), ((locals.var_delta_ns_dn9 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn9)),)
    } else {
        (locals.var_delta_nd, locals.var_delta_nd_dn4, locals.var_delta_nd_dn6, locals.var_delta_nd_dn7, locals.var_delta_nd_dn8, locals.var_delta_nd_dn9,)
    }
};
        locals.var_delta_nd = assign44280_e57173;
        locals.var_delta_nd_dn4 = assign44280_e57173_d_n4;
        locals.var_delta_nd_dn6 = assign44280_e57173_d_n6;
        locals.var_delta_nd_dn7 = assign44280_e57173_d_n7;
        locals.var_delta_nd_dn8 = assign44280_e57173_d_n8;
        locals.var_delta_nd_dn9 = assign44280_e57173_d_n9;
        locals.var_delta_nd_rv = 0.0;

        let assign44290_e57175: f64 = (locals.var_xg).abs();
        let assign44290_e57177: f64 = if assign44290_e57175 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign44290_e57177;
        locals.var_guard1222_rv = 0.0;

        let (assign44300_e57189, assign44300_e57189_d_n4, assign44300_e57189_d_n6, assign44300_e57189_d_n7, assign44300_e57189_d_n8, assign44300_e57189_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign44300_e57183: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign44300_e57185: f64 = (assign44300_e57183 * 0.16666666666666666);
        let assign44300_e57187: f64 = (assign44300_e57185 * 0.7071067811865475);
        (assign44300_e57187, ((((locals.var_inv_xi_dn4 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn9 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign44300_e57189;
        locals.var_sp_s_temp1_dn4 = assign44300_e57189_d_n4;
        locals.var_sp_s_temp1_dn6 = assign44300_e57189_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44300_e57189_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44300_e57189_d_n8;
        locals.var_sp_s_temp1_dn9 = assign44300_e57189_d_n9;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign44310_e57209, assign44310_e57209_d_n4, assign44310_e57209_d_n6, assign44310_e57209_d_n7, assign44310_e57209_d_n8, assign44310_e57209_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign44310_e57195: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign44310_e57200: f64 = (1.0 - locals.var_delta_nd);
        let assign44310_e57201: f64 = (locals.var_xg * assign44310_e57200);
        let assign44310_e57203: f64 = (assign44310_e57201 * locals.var_gf);
        let assign44310_e57205: f64 = (assign44310_e57203 * locals.var_sp_s_temp1);
        let assign44310_e57206: f64 = (1.0 + assign44310_e57205);
        let assign44310_e57207: f64 = (assign44310_e57195 * assign44310_e57206);
        (assign44310_e57207, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn4 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn4))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn4)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn6 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn6))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn7 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn7))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn8 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn8))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn9 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn9))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn9)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn9)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44310_e57209;
        locals.var_x_d_dn4 = assign44310_e57209_d_n4;
        locals.var_x_d_dn6 = assign44310_e57209_d_n6;
        locals.var_x_d_dn7 = assign44310_e57209_d_n7;
        locals.var_x_d_dn8 = assign44310_e57209_d_n8;
        locals.var_x_d_dn9 = assign44310_e57209_d_n9;
        locals.var_x_d_rv = 0.0;

        let (assign44320_e57218, assign44320_e57218_d_n4, assign44320_e57218_d_n6, assign44320_e57218_d_n7, assign44320_e57218_d_n8, assign44320_e57218_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44320_e57216: f64 = (locals.var_xn_d + 3.0);
        (assign44320_e57216, locals.var_xn_d_dn4, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, locals.var_xn_d_dn9,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9,)
    }
};
        locals.var_sp_s_bx = assign44320_e57218;
        locals.var_sp_s_bx_dn4 = assign44320_e57218_d_n4;
        locals.var_sp_s_bx_dn6 = assign44320_e57218_d_n6;
        locals.var_sp_s_bx_dn7 = assign44320_e57218_d_n7;
        locals.var_sp_s_bx_dn8 = assign44320_e57218_d_n8;
        locals.var_sp_s_bx_dn9 = assign44320_e57218_d_n9;
        locals.var_sp_s_bx_rv = 0.0;

        let (assign44330_e57251, assign44330_e57251_d_n4, assign44330_e57251_d_n6, assign44330_e57251_d_n7, assign44330_e57251_d_n8, assign44330_e57251_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44330_e57226: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign44330_e57229: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44330_e57232: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44330_e57233: f64 = (assign44330_e57229 * assign44330_e57232);
        let assign44330_e57235: f64 = (assign44330_e57233 + 5.0);
        let assign44330_e57236: f64 = (assign44330_e57235).sqrt();
        let assign44330_e57237: f64 = (assign44330_e57226 - assign44330_e57236);
        let assign44330_e57238: f64 = (0.5 * assign44330_e57237);
        let assign44330_e57243: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign44330_e57245: f64 = (assign44330_e57243 + 5.0);
        let assign44330_e57246: f64 = (assign44330_e57245).sqrt();
        let assign44330_e57247: f64 = (locals.var_sp_s_bx - assign44330_e57246);
        let assign44330_e57248: f64 = (0.5 * assign44330_e57247);
        let assign44330_e57249: f64 = (assign44330_e57238 - assign44330_e57248);
        (assign44330_e57249, ((0.5 * ((locals.var_sp_s_x1_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn4 - (((locals.var_sp_s_bx_dn4 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn4)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn9 - (((locals.var_sp_s_bx_dn9 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn9)) / (2.0 * assign44330_e57246))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign44330_e57251;
        locals.var_sp_s_eta_dn4 = assign44330_e57251_d_n4;
        locals.var_sp_s_eta_dn6 = assign44330_e57251_d_n6;
        locals.var_sp_s_eta_dn7 = assign44330_e57251_d_n7;
        locals.var_sp_s_eta_dn8 = assign44330_e57251_d_n8;
        locals.var_sp_s_eta_dn9 = assign44330_e57251_d_n9;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign44340_e57260, assign44340_e57260_d_n4, assign44340_e57260_d_n6, assign44340_e57260_d_n7, assign44340_e57260_d_n8, assign44340_e57260_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44340_e57258: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign44340_e57258, (locals.var_xg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_xg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44340_e57260;
        locals.var_sp_s_temp_dn4 = assign44340_e57260_d_n4;
        locals.var_sp_s_temp_dn6 = assign44340_e57260_d_n6;
        locals.var_sp_s_temp_dn7 = assign44340_e57260_d_n7;
        locals.var_sp_s_temp_dn8 = assign44340_e57260_d_n8;
        locals.var_sp_s_temp_dn9 = assign44340_e57260_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign44350_e57269, assign44350_e57269_d_n4, assign44350_e57269_d_n6, assign44350_e57269_d_n7, assign44350_e57269_d_n8, assign44350_e57269_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44350_e57266: f64 = (-locals.var_sp_s_eta);
        let assign44350_e57267: f64 = (assign44350_e57266).exp();
        (assign44350_e57267, (assign44350_e57267 * (-locals.var_sp_s_eta_dn4)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn6)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn7)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn8)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign44350_e57269;
        locals.var_sp_s_temp1_dn4 = assign44350_e57269_d_n4;
        locals.var_sp_s_temp1_dn6 = assign44350_e57269_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44350_e57269_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44350_e57269_d_n8;
        locals.var_sp_s_temp1_dn9 = assign44350_e57269_d_n9;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign44360_e57282, assign44360_e57282_d_n4, assign44360_e57282_d_n6, assign44360_e57282_d_n7, assign44360_e57282_d_n8, assign44360_e57282_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44360_e57278: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44360_e57279: f64 = (2.0 + assign44360_e57278);
        let assign44360_e57280: f64 = (1.0 / assign44360_e57279);
        (assign44360_e57280, (-(((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) / (assign44360_e57279 * assign44360_e57279))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn4, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn9,)
    }
};
        locals.var_sp_s_temp2 = assign44360_e57282;
        locals.var_sp_s_temp2_dn4 = assign44360_e57282_d_n4;
        locals.var_sp_s_temp2_dn6 = assign44360_e57282_d_n6;
        locals.var_sp_s_temp2_dn7 = assign44360_e57282_d_n7;
        locals.var_sp_s_temp2_dn8 = assign44360_e57282_d_n8;
        locals.var_sp_s_temp2_dn9 = assign44360_e57282_d_n9;
        locals.var_sp_s_temp2_rv = 0.0;

        let (assign44370_e57293, assign44370_e57293_d_n4, assign44370_e57293_d_n6, assign44370_e57293_d_n7, assign44370_e57293_d_n8, assign44370_e57293_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44370_e57289: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44370_e57291: f64 = (assign44370_e57289 * locals.var_sp_s_temp2);
        (assign44370_e57291, ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn4)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign44370_e57293;
        locals.var_sp_s_xi0_dn4 = assign44370_e57293_d_n4;
        locals.var_sp_s_xi0_dn6 = assign44370_e57293_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44370_e57293_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44370_e57293_d_n8;
        locals.var_sp_s_xi0_dn9 = assign44370_e57293_d_n9;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign44380_e57306, assign44380_e57306_d_n4, assign44380_e57306_d_n6, assign44380_e57306_d_n7, assign44380_e57306_d_n8, assign44380_e57306_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44380_e57301: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign44380_e57303: f64 = (assign44380_e57301 * locals.var_sp_s_temp2);
        let assign44380_e57304: f64 = (4.0 * assign44380_e57303);
        (assign44380_e57304, (4.0 * ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn4))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign44380_e57306;
        locals.var_sp_s_xi1_dn4 = assign44380_e57306_d_n4;
        locals.var_sp_s_xi1_dn6 = assign44380_e57306_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44380_e57306_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44380_e57306_d_n8;
        locals.var_sp_s_xi1_dn9 = assign44380_e57306_d_n9;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign44390_e57323, assign44390_e57323_d_n4, assign44390_e57323_d_n6, assign44390_e57323_d_n7, assign44390_e57323_d_n8, assign44390_e57323_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44390_e57313: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign44390_e57316: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44390_e57317: f64 = (assign44390_e57313 - assign44390_e57316);
        let assign44390_e57319: f64 = (assign44390_e57317 * locals.var_sp_s_temp2);
        let assign44390_e57321: f64 = (assign44390_e57319 * locals.var_sp_s_temp2);
        (assign44390_e57321, ((((((8.0 * locals.var_sp_s_temp2_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn4)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign44390_e57323;
        locals.var_sp_s_xi2_dn4 = assign44390_e57323_d_n4;
        locals.var_sp_s_xi2_dn6 = assign44390_e57323_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44390_e57323_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44390_e57323_d_n8;
        locals.var_sp_s_xi2_dn9 = assign44390_e57323_d_n9;
        locals.var_sp_s_xi2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        locals: &mut StampLocals,
    ) {
        let (assign44400_e57371, assign44400_e57371_d_n4, assign44400_e57371_d_n6, assign44400_e57371_d_n7, assign44400_e57371_d_n8, assign44400_e57371_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44400_e57331: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44400_e57335: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign44400_e57337: f64 = (assign44400_e57335 - 1.0);
        let assign44400_e57341: f64 = (locals.var_sp_s_eta + 1.0);
        let assign44400_e57343: f64 = (assign44400_e57341 + locals.var_sp_s_xi0);
        let assign44400_e57344: f64 = (locals.var_delta_nd * assign44400_e57343);
        let assign44400_e57345: f64 = (assign44400_e57337 - assign44400_e57344);
        let assign44400_e57346: f64 = (locals.var_gf2 * assign44400_e57345);
        let assign44400_e57347: f64 = (assign44400_e57331 - assign44400_e57346);
        let (assign44400_e57369, assign44400_e57369_d_n4, assign44400_e57369_d_n6, assign44400_e57369_d_n7, assign44400_e57369_d_n8, assign44400_e57369_d_n9,) = {
            if (1e-40 > assign44400_e57347) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44400_e57352: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign44400_e57356: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign44400_e57358: f64 = (assign44400_e57356 - 1.0);
                let assign44400_e57362: f64 = (locals.var_sp_s_eta + 1.0);
                let assign44400_e57364: f64 = (assign44400_e57362 + locals.var_sp_s_xi0);
                let assign44400_e57365: f64 = (locals.var_delta_nd * assign44400_e57364);
                let assign44400_e57366: f64 = (assign44400_e57358 - assign44400_e57365);
                let assign44400_e57367: f64 = (locals.var_gf2 * assign44400_e57366);
                let assign44400_e57368: f64 = (assign44400_e57352 - assign44400_e57367);
                (assign44400_e57368, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn4 + locals.var_sp_s_eta_dn4) - ((locals.var_delta_nd_dn4 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_nd_dn6 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_nd_dn7 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_nd_dn8 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn9 + locals.var_sp_s_eta_dn9) - ((locals.var_delta_nd_dn9 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn9 + locals.var_sp_s_xi0_dn9))))))),)
            }
        };
        (assign44400_e57369, assign44400_e57369_d_n4, assign44400_e57369_d_n6, assign44400_e57369_d_n7, assign44400_e57369_d_n8, assign44400_e57369_d_n9,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign44400_e57371;
        locals.var_sp_s_a_dn4 = assign44400_e57371_d_n4;
        locals.var_sp_s_a_dn6 = assign44400_e57371_d_n6;
        locals.var_sp_s_a_dn7 = assign44400_e57371_d_n7;
        locals.var_sp_s_a_dn8 = assign44400_e57371_d_n8;
        locals.var_sp_s_a_dn9 = assign44400_e57371_d_n9;
        locals.var_sp_s_a_rv = 0.0;

        let (assign44410_e57388, assign44410_e57388_d_n4, assign44410_e57388_d_n6, assign44410_e57388_d_n7, assign44410_e57388_d_n8, assign44410_e57388_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44410_e57382: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44410_e57383: f64 = (locals.var_sp_s_temp1 - assign44410_e57382);
        let assign44410_e57384: f64 = (locals.var_gf2 * assign44410_e57383);
        let assign44410_e57385: f64 = (0.5 * assign44410_e57384);
        let assign44410_e57386: f64 = (1.0 - assign44410_e57385);
        (assign44410_e57386, (-(0.5 * ((locals.var_gf2_dn4 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn4 - ((locals.var_delta_nd_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn4))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gf2_dn9 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn9 - ((locals.var_delta_nd_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn9))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn4, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn9,)
    }
};
        locals.var_sp_s_b = assign44410_e57388;
        locals.var_sp_s_b_dn4 = assign44410_e57388_d_n4;
        locals.var_sp_s_b_dn6 = assign44410_e57388_d_n6;
        locals.var_sp_s_b_dn7 = assign44410_e57388_d_n7;
        locals.var_sp_s_b_dn8 = assign44410_e57388_d_n8;
        locals.var_sp_s_b_dn9 = assign44410_e57388_d_n9;
        locals.var_sp_s_b_rv = 0.0;

        let (assign44420_e57409, assign44420_e57409_d_n4, assign44420_e57409_d_n6, assign44420_e57409_d_n7, assign44420_e57409_d_n8, assign44420_e57409_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44420_e57395: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44420_e57399: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign44420_e57403: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44420_e57404: f64 = (locals.var_delta_nd * assign44420_e57403);
        let assign44420_e57405: f64 = (assign44420_e57399 - assign44420_e57404);
        let assign44420_e57406: f64 = (locals.var_gf2 * assign44420_e57405);
        let assign44420_e57407: f64 = (assign44420_e57395 + assign44420_e57406);
        (assign44420_e57407, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn4) - ((locals.var_delta_nd_dn4 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_nd_dn6 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_nd_dn7 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_nd_dn8 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn9) - ((locals.var_delta_nd_dn9 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign44420_e57409;
        locals.var_sp_s_c_dn4 = assign44420_e57409_d_n4;
        locals.var_sp_s_c_dn6 = assign44420_e57409_d_n6;
        locals.var_sp_s_c_dn7 = assign44420_e57409_d_n7;
        locals.var_sp_s_c_dn8 = assign44420_e57409_d_n8;
        locals.var_sp_s_c_dn9 = assign44420_e57409_d_n9;
        locals.var_sp_s_c_rv = 0.0;

        let (assign44430_e57423, assign44430_e57423_d_n4, assign44430_e57423_d_n6, assign44430_e57423_d_n7, assign44430_e57423_d_n8, assign44430_e57423_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44430_e57416: f64 = (locals.var_xn_d - locals.var_sp_s_eta);
        let assign44430_e57419: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign44430_e57420: f64 = (assign44430_e57419).ln();
        let assign44430_e57421: f64 = (assign44430_e57416 + assign44430_e57420);
        (assign44430_e57421, ((locals.var_xn_d_dn4 - locals.var_sp_s_eta_dn4) + ((((locals.var_sp_s_a_dn4 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn4)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn9 - locals.var_sp_s_eta_dn9) + ((((locals.var_sp_s_a_dn9 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn9)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign44430_e57423;
        locals.var_sp_s_tau_dn4 = assign44430_e57423_d_n4;
        locals.var_sp_s_tau_dn6 = assign44430_e57423_d_n6;
        locals.var_sp_s_tau_dn7 = assign44430_e57423_d_n7;
        locals.var_sp_s_tau_dn8 = assign44430_e57423_d_n8;
        locals.var_sp_s_tau_dn9 = assign44430_e57423_d_n9;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign44440_e57432, assign44440_e57432_d_n4, assign44440_e57432_d_n6, assign44440_e57432_d_n7, assign44440_e57432_d_n8, assign44440_e57432_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44440_e57430: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign44440_e57430, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign44440_e57432;
        locals.var_nu_dn4 = assign44440_e57432_d_n4;
        locals.var_nu_dn6 = assign44440_e57432_d_n6;
        locals.var_nu_dn7 = assign44440_e57432_d_n7;
        locals.var_nu_dn8 = assign44440_e57432_d_n8;
        locals.var_nu_dn9 = assign44440_e57432_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign44450_e57453, assign44450_e57453_d_n4, assign44450_e57453_d_n6, assign44450_e57453_d_n7, assign44450_e57453_d_n8, assign44450_e57453_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44450_e57439: f64 = (locals.var_nu * locals.var_nu);
        let assign44450_e57444: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44450_e57445: f64 = (0.5 * assign44450_e57444);
        let assign44450_e57448: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44450_e57449: f64 = (assign44450_e57445 - assign44450_e57448);
        let assign44450_e57450: f64 = (locals.var_sp_s_tau * assign44450_e57449);
        let assign44450_e57451: f64 = (assign44450_e57439 + assign44450_e57450);
        (assign44450_e57451, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign44450_e57453;
        locals.var_mutau_dn4 = assign44450_e57453_d_n4;
        locals.var_mutau_dn6 = assign44450_e57453_d_n6;
        locals.var_mutau_dn7 = assign44450_e57453_d_n7;
        locals.var_mutau_dn8 = assign44450_e57453_d_n8;
        locals.var_mutau_dn9 = assign44450_e57453_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign44460_e57488, assign44460_e57488_d_n4, assign44460_e57488_d_n6, assign44460_e57488_d_n7, assign44460_e57488_d_n8, assign44460_e57488_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44460_e57461: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign44460_e57463: f64 = (assign44460_e57461 * locals.var_sp_s_tau);
        let assign44460_e57467: f64 = (locals.var_nu / locals.var_mutau);
        let assign44460_e57469: f64 = (assign44460_e57467 * locals.var_sp_s_tau);
        let assign44460_e57471: f64 = (assign44460_e57469 * locals.var_sp_s_tau);
        let assign44460_e57473: f64 = (assign44460_e57471 * locals.var_sp_s_c);
        let assign44460_e57476: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44460_e57478: f64 = (assign44460_e57476 * 0.3333333333333333);
        let assign44460_e57481: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44460_e57482: f64 = (assign44460_e57478 - assign44460_e57481);
        let assign44460_e57483: f64 = (assign44460_e57473 * assign44460_e57482);
        let assign44460_e57484: f64 = (locals.var_mutau + assign44460_e57483);
        let assign44460_e57485: f64 = (assign44460_e57463 / assign44460_e57484);
        let assign44460_e57486: f64 = (locals.var_sp_s_eta + assign44460_e57485);
        (assign44460_e57486, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn4)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn4)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn6)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn6)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn7)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn7)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn8)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn8)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn9)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn9)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))))) / (assign44460_e57484 * assign44460_e57484))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn4, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn9,)
    }
};
        locals.var_sp_s_x0 = assign44460_e57488;
        locals.var_sp_s_x0_dn4 = assign44460_e57488_d_n4;
        locals.var_sp_s_x0_dn6 = assign44460_e57488_d_n6;
        locals.var_sp_s_x0_dn7 = assign44460_e57488_d_n7;
        locals.var_sp_s_x0_dn8 = assign44460_e57488_d_n8;
        locals.var_sp_s_x0_dn9 = assign44460_e57488_d_n9;
        locals.var_sp_s_x0_rv = 0.0;

        let assign44470_e57491: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign44470_e57491;
        locals.var_guard1223_rv = 0.0;

        let (assign44480_e57501, assign44480_e57501_d_n4, assign44480_e57501_d_n6, assign44480_e57501_d_n7, assign44480_e57501_d_n8, assign44480_e57501_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44480_e57499: f64 = (locals.var_sp_s_x0).exp();
        (assign44480_e57499, (assign44480_e57499 * locals.var_sp_s_x0_dn4), (assign44480_e57499 * locals.var_sp_s_x0_dn6), (assign44480_e57499 * locals.var_sp_s_x0_dn7), (assign44480_e57499 * locals.var_sp_s_x0_dn8), (assign44480_e57499 * locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44480_e57501;
        locals.var_sp_s_delta0_dn4 = assign44480_e57501_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44480_e57501_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44480_e57501_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44480_e57501_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44480_e57501_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign44490_e57512, assign44490_e57512_d_n4, assign44490_e57512_d_n6, assign44490_e57512_d_n7, assign44490_e57512_d_n8, assign44490_e57512_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44490_e57510: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign44490_e57510, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44490_e57512;
        locals.var_sp_s_delta1_dn4 = assign44490_e57512_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44490_e57512_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44490_e57512_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44490_e57512_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44490_e57512_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign44500_e57523, assign44500_e57523_d_n4, assign44500_e57523_d_n6, assign44500_e57523_d_n7, assign44500_e57523_d_n8, assign44500_e57523_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44500_e57521: f64 = (locals.var_delta_nd * locals.var_sp_s_delta0);
        (assign44500_e57521, ((locals.var_delta_nd_dn4 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn4)), ((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)), ((locals.var_delta_nd_dn9 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44500_e57523;
        locals.var_sp_s_delta0_dn4 = assign44500_e57523_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44500_e57523_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44500_e57523_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44500_e57523_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44500_e57523_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let assign44510_e57527: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44510_e57528: f64 = if locals.var_sp_s_x0 > assign44510_e57527 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign44510_e57528;
        locals.var_guard1224_rv = 0.0;

        let (assign44520_e57543, assign44520_e57543_d_n4, assign44520_e57543_d_n6, assign44520_e57543_d_n7, assign44520_e57543_d_n8, assign44520_e57543_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 != 0.0)) {
        let assign44520_e57540: f64 = (locals.var_sp_s_x0 - locals.var_xn_d);
        let assign44520_e57541: f64 = (assign44520_e57540).exp();
        (assign44520_e57541, (assign44520_e57541 * (locals.var_sp_s_x0_dn4 - locals.var_xn_d_dn4)), (assign44520_e57541 * (locals.var_sp_s_x0_dn6 - locals.var_xn_d_dn6)), (assign44520_e57541 * (locals.var_sp_s_x0_dn7 - locals.var_xn_d_dn7)), (assign44520_e57541 * (locals.var_sp_s_x0_dn8 - locals.var_xn_d_dn8)), (assign44520_e57541 * (locals.var_sp_s_x0_dn9 - locals.var_xn_d_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44520_e57543;
        locals.var_sp_s_delta0_dn4 = assign44520_e57543_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44520_e57543_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44520_e57543_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44520_e57543_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44520_e57543_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign44530_e57557, assign44530_e57557_d_n4, assign44530_e57557_d_n6, assign44530_e57557_d_n7, assign44530_e57557_d_n8, assign44530_e57557_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 != 0.0)) {
        let assign44530_e57555: f64 = (locals.var_delta_nd / locals.var_sp_s_delta0);
        (assign44530_e57555, (((locals.var_delta_nd_dn4 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn4)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn9 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn9)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44530_e57557;
        locals.var_sp_s_delta1_dn4 = assign44530_e57557_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44530_e57557_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44530_e57557_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44530_e57557_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44530_e57557_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign44540_e57598, assign44540_e57598_d_n4, assign44540_e57598_d_n6, assign44540_e57598_d_n7, assign44540_e57598_d_n8, assign44540_e57598_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 == 0.0)) {
        let assign44540_e57572: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57574: f64 = (assign44540_e57572 - 230.25850929940458);
        let assign44540_e57579: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57581: f64 = (assign44540_e57579 - 230.25850929940458);
        let assign44540_e57585: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57587: f64 = (assign44540_e57585 - 230.25850929940458);
        let assign44540_e57589: f64 = (assign44540_e57587 * 0.3333333333333333);
        let assign44540_e57590: f64 = (1.0 + assign44540_e57589);
        let assign44540_e57591: f64 = (assign44540_e57581 * assign44540_e57590);
        let assign44540_e57592: f64 = (0.5 * assign44540_e57591);
        let assign44540_e57593: f64 = (1.0 + assign44540_e57592);
        let assign44540_e57594: f64 = (assign44540_e57574 * assign44540_e57593);
        let assign44540_e57595: f64 = (1.0 + assign44540_e57594);
        let assign44540_e57596: f64 = (1e-100 / assign44540_e57595);
        (assign44540_e57596, (-((1e-100 * (((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44540_e57598;
        locals.var_sp_s_delta0_dn4 = assign44540_e57598_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44540_e57598_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44540_e57598_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44540_e57598_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44540_e57598_d_n9;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign44550_e57633, assign44550_e57633_d_n4, assign44550_e57633_d_n6, assign44550_e57633_d_n7, assign44550_e57633_d_n8, assign44550_e57633_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 == 0.0)) {
        let assign44550_e57613: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57618: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57622: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57624: f64 = (assign44550_e57622 * 0.3333333333333333);
        let assign44550_e57625: f64 = (1.0 + assign44550_e57624);
        let assign44550_e57626: f64 = (assign44550_e57618 * assign44550_e57625);
        let assign44550_e57627: f64 = (0.5 * assign44550_e57626);
        let assign44550_e57628: f64 = (1.0 + assign44550_e57627);
        let assign44550_e57629: f64 = (assign44550_e57613 * assign44550_e57628);
        let assign44550_e57630: f64 = (1.0 + assign44550_e57629);
        let assign44550_e57631: f64 = (1e-100 / assign44550_e57630);
        (assign44550_e57631, (-((1e-100 * ((locals.var_sp_s_x0_dn4 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn4 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn4 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn9 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn9 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn9 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44550_e57633;
        locals.var_sp_s_delta1_dn4 = assign44550_e57633_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44550_e57633_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44550_e57633_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44550_e57633_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44550_e57633_d_n9;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign44560_e57646, assign44560_e57646_d_n4, assign44560_e57646_d_n6, assign44560_e57646_d_n7, assign44560_e57646_d_n8, assign44560_e57646_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44560_e57642: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44560_e57643: f64 = (2.0 + assign44560_e57642);
        let assign44560_e57644: f64 = (1.0 / assign44560_e57643);
        (assign44560_e57644, (-(((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) / (assign44560_e57643 * assign44560_e57643))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44560_e57646;
        locals.var_sp_s_temp_dn4 = assign44560_e57646_d_n4;
        locals.var_sp_s_temp_dn6 = assign44560_e57646_d_n6;
        locals.var_sp_s_temp_dn7 = assign44560_e57646_d_n7;
        locals.var_sp_s_temp_dn8 = assign44560_e57646_d_n8;
        locals.var_sp_s_temp_dn9 = assign44560_e57646_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign44570_e57657, assign44570_e57657_d_n4, assign44570_e57657_d_n6, assign44570_e57657_d_n7, assign44570_e57657_d_n8, assign44570_e57657_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44570_e57653: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44570_e57655: f64 = (assign44570_e57653 * locals.var_sp_s_temp);
        (assign44570_e57655, ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign44570_e57657;
        locals.var_sp_s_xi0_dn4 = assign44570_e57657_d_n4;
        locals.var_sp_s_xi0_dn6 = assign44570_e57657_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44570_e57657_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44570_e57657_d_n8;
        locals.var_sp_s_xi0_dn9 = assign44570_e57657_d_n9;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign44580_e57670, assign44580_e57670_d_n4, assign44580_e57670_d_n6, assign44580_e57670_d_n7, assign44580_e57670_d_n8, assign44580_e57670_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44580_e57665: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign44580_e57667: f64 = (assign44580_e57665 * locals.var_sp_s_temp);
        let assign44580_e57668: f64 = (4.0 * assign44580_e57667);
        (assign44580_e57668, (4.0 * ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign44580_e57670;
        locals.var_sp_s_xi1_dn4 = assign44580_e57670_d_n4;
        locals.var_sp_s_xi1_dn6 = assign44580_e57670_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44580_e57670_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44580_e57670_d_n8;
        locals.var_sp_s_xi1_dn9 = assign44580_e57670_d_n9;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign44590_e57687, assign44590_e57687_d_n4, assign44590_e57687_d_n6, assign44590_e57687_d_n7, assign44590_e57687_d_n8, assign44590_e57687_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44590_e57677: f64 = (8.0 * locals.var_sp_s_temp);
        let assign44590_e57680: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44590_e57681: f64 = (assign44590_e57677 - assign44590_e57680);
        let assign44590_e57683: f64 = (assign44590_e57681 * locals.var_sp_s_temp);
        let assign44590_e57685: f64 = (assign44590_e57683 * locals.var_sp_s_temp);
        (assign44590_e57685, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign44590_e57687;
        locals.var_sp_s_xi2_dn4 = assign44590_e57687_d_n4;
        locals.var_sp_s_xi2_dn6 = assign44590_e57687_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44590_e57687_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44590_e57687_d_n8;
        locals.var_sp_s_xi2_dn9 = assign44590_e57687_d_n9;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign44600_e57696, assign44600_e57696_d_n4, assign44600_e57696_d_n6, assign44600_e57696_d_n7, assign44600_e57696_d_n8, assign44600_e57696_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44600_e57694: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign44600_e57694, (locals.var_xg_dn4 - locals.var_sp_s_x0_dn4), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), (locals.var_xg_dn9 - locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44600_e57696;
        locals.var_sp_s_temp_dn4 = assign44600_e57696_d_n4;
        locals.var_sp_s_temp_dn6 = assign44600_e57696_d_n6;
        locals.var_sp_s_temp_dn7 = assign44600_e57696_d_n7;
        locals.var_sp_s_temp_dn8 = assign44600_e57696_d_n8;
        locals.var_sp_s_temp_dn9 = assign44600_e57696_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign44610_e57719, assign44610_e57719_d_n4, assign44610_e57719_d_n6, assign44610_e57719_d_n7, assign44610_e57719_d_n8, assign44610_e57719_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44610_e57703: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44610_e57707: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign44610_e57709: f64 = (assign44610_e57707 + locals.var_sp_s_delta0);
        let assign44610_e57713: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44610_e57714: f64 = (locals.var_delta_nd * assign44610_e57713);
        let assign44610_e57715: f64 = (assign44610_e57709 - assign44610_e57714);
        let assign44610_e57716: f64 = (locals.var_gf2 * assign44610_e57715);
        let assign44610_e57717: f64 = (assign44610_e57703 + assign44610_e57716);
        (assign44610_e57717, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign44610_e57719;
        locals.var_sp_s_pc_dn4 = assign44610_e57719_d_n4;
        locals.var_sp_s_pc_dn6 = assign44610_e57719_d_n6;
        locals.var_sp_s_pc_dn7 = assign44610_e57719_d_n7;
        locals.var_sp_s_pc_dn8 = assign44610_e57719_d_n8;
        locals.var_sp_s_pc_dn9 = assign44610_e57719_d_n9;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign44620_e57746, assign44620_e57746_d_n4, assign44620_e57746_d_n6, assign44620_e57746_d_n7, assign44620_e57746_d_n8, assign44620_e57746_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44620_e57726: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44620_e57730: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign44620_e57732: f64 = (assign44620_e57730 - 1.0);
        let assign44620_e57734: f64 = (assign44620_e57732 + locals.var_sp_s_delta0);
        let assign44620_e57738: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign44620_e57740: f64 = (assign44620_e57738 + locals.var_sp_s_xi0);
        let assign44620_e57741: f64 = (locals.var_delta_nd * assign44620_e57740);
        let assign44620_e57742: f64 = (assign44620_e57734 - assign44620_e57741);
        let assign44620_e57743: f64 = (locals.var_gf2 * assign44620_e57742);
        let assign44620_e57744: f64 = (assign44620_e57726 - assign44620_e57743);
        (assign44620_e57744, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_x0_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_x0_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn9 + locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign44620_e57746;
        locals.var_sp_s_qc_dn4 = assign44620_e57746_d_n4;
        locals.var_sp_s_qc_dn6 = assign44620_e57746_d_n6;
        locals.var_sp_s_qc_dn7 = assign44620_e57746_d_n7;
        locals.var_sp_s_qc_dn8 = assign44620_e57746_d_n8;
        locals.var_sp_s_qc_dn9 = assign44620_e57746_d_n9;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign44630_e57763, assign44630_e57763_d_n4, assign44630_e57763_d_n6, assign44630_e57763_d_n7, assign44630_e57763_d_n8, assign44630_e57763_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44630_e57755: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign44630_e57758: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44630_e57759: f64 = (assign44630_e57755 - assign44630_e57758);
        let assign44630_e57760: f64 = (locals.var_gf2 * assign44630_e57759);
        let assign44630_e57761: f64 = (2.0 - assign44630_e57760);
        (assign44630_e57761, (-((locals.var_gf2_dn4 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44630_e57763;
        locals.var_sp_s_temp_dn4 = assign44630_e57763_d_n4;
        locals.var_sp_s_temp_dn6 = assign44630_e57763_d_n6;
        locals.var_sp_s_temp_dn7 = assign44630_e57763_d_n7;
        locals.var_sp_s_temp_dn8 = assign44630_e57763_d_n8;
        locals.var_sp_s_temp_dn9 = assign44630_e57763_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign44640_e57778, assign44640_e57778_d_n4, assign44640_e57778_d_n6, assign44640_e57778_d_n7, assign44640_e57778_d_n8, assign44640_e57778_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44640_e57770: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign44640_e57774: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign44640_e57775: f64 = (2.0 * assign44640_e57774);
        let assign44640_e57776: f64 = (assign44640_e57770 - assign44640_e57775);
        (assign44640_e57776, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44640_e57778;
        locals.var_sp_s_temp_dn4 = assign44640_e57778_d_n4;
        locals.var_sp_s_temp_dn6 = assign44640_e57778_d_n6;
        locals.var_sp_s_temp_dn7 = assign44640_e57778_d_n7;
        locals.var_sp_s_temp_dn8 = assign44640_e57778_d_n8;
        locals.var_sp_s_temp_dn9 = assign44640_e57778_d_n9;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign44650_e57794, assign44650_e57794_d_n4, assign44650_e57794_d_n6, assign44650_e57794_d_n7, assign44650_e57794_d_n8, assign44650_e57794_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44650_e57788: f64 = (locals.var_sp_s_temp).sqrt();
        let assign44650_e57789: f64 = (locals.var_sp_s_pc + assign44650_e57788);
        let assign44650_e57790: f64 = (locals.var_sp_s_qc / assign44650_e57789);
        let assign44650_e57791: f64 = (2.0 * assign44650_e57790);
        let assign44650_e57792: f64 = (locals.var_sp_s_x0 + assign44650_e57791);
        (assign44650_e57792, (locals.var_sp_s_x0_dn4 + (2.0 * (((locals.var_sp_s_qc_dn4 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn9 + (2.0 * (((locals.var_sp_s_qc_dn9 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44650_e57794;
        locals.var_x_d_dn4 = assign44650_e57794_d_n4;
        locals.var_x_d_dn6 = assign44650_e57794_d_n6;
        locals.var_x_d_dn7 = assign44650_e57794_d_n7;
        locals.var_x_d_dn8 = assign44650_e57794_d_n8;
        locals.var_x_d_dn9 = assign44650_e57794_d_n9;
        locals.var_x_d_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        locals: &mut StampLocals,
    ) {
        let (assign44660_e57800, assign44660_e57800_d_n4, assign44660_e57800_d_n6, assign44660_e57800_d_n7, assign44660_e57800_d_n8, assign44660_e57800_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44660_e57798: f64 = (locals.var_x_d - locals.var_x_s);
        (assign44660_e57798, (locals.var_x_d_dn4 - locals.var_x_s_dn4), (locals.var_x_d_dn6 - locals.var_x_s_dn6), (locals.var_x_d_dn7 - locals.var_x_s_dn7), (locals.var_x_d_dn8 - locals.var_x_s_dn8), (locals.var_x_d_dn9 - locals.var_x_s_dn9),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign44660_e57800;
        locals.var_x_ds_dn4 = assign44660_e57800_d_n4;
        locals.var_x_ds_dn6 = assign44660_e57800_d_n6;
        locals.var_x_ds_dn7 = assign44660_e57800_d_n7;
        locals.var_x_ds_dn8 = assign44660_e57800_d_n8;
        locals.var_x_ds_dn9 = assign44660_e57800_d_n9;
        locals.var_x_ds_rv = 0.0;

        let assign44670_e57803: f64 = if locals.var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign44670_e57803;
        locals.var_guard1225_rv = 0.0;

        let (assign44680_e57829, assign44680_e57829_d_n4, assign44680_e57829_d_n6, assign44680_e57829_d_n7, assign44680_e57829_d_n8, assign44680_e57829_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44680_e57810: f64 = (locals.var_xg - locals.var_x_s);
        let assign44680_e57811: f64 = (2.0 * assign44680_e57810);
        let assign44680_e57815: f64 = (1.0 - locals.var_es);
        let assign44680_e57818: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44680_e57819: f64 = (assign44680_e57815 + assign44680_e57818);
        let assign44680_e57823: f64 = (1.0 + locals.var_xi1s);
        let assign44680_e57824: f64 = (locals.var_delta_nd * assign44680_e57823);
        let assign44680_e57825: f64 = (assign44680_e57819 - assign44680_e57824);
        let assign44680_e57826: f64 = (locals.var_gf2 * assign44680_e57825);
        let assign44680_e57827: f64 = (assign44680_e57811 + assign44680_e57826);
        (assign44680_e57827, ((2.0 * (locals.var_xg_dn4 - locals.var_x_s_dn4)) + ((locals.var_gf2_dn4 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn4) + ((locals.var_delta_1s_dn4 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn4))) - ((locals.var_delta_nd_dn4 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn4)))))), ((2.0 * (locals.var_xg_dn6 - locals.var_x_s_dn6)) + ((locals.var_gf2_dn6 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn6) + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn6)))))), ((2.0 * (locals.var_xg_dn7 - locals.var_x_s_dn7)) + ((locals.var_gf2_dn7 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn7) + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn7)))))), ((2.0 * (locals.var_xg_dn8 - locals.var_x_s_dn8)) + ((locals.var_gf2_dn8 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn8) + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn8)))))), ((2.0 * (locals.var_xg_dn9 - locals.var_x_s_dn9)) + ((locals.var_gf2_dn9 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn9) + ((locals.var_delta_1s_dn9 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn9))) - ((locals.var_delta_nd_dn9 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn9)))))),)
    } else {
        (locals.var_pc, locals.var_pc_dn4, locals.var_pc_dn6, locals.var_pc_dn7, locals.var_pc_dn8, locals.var_pc_dn9,)
    }
};
        locals.var_pc = assign44680_e57829;
        locals.var_pc_dn4 = assign44680_e57829_d_n4;
        locals.var_pc_dn6 = assign44680_e57829_d_n6;
        locals.var_pc_dn7 = assign44680_e57829_d_n7;
        locals.var_pc_dn8 = assign44680_e57829_d_n8;
        locals.var_pc_dn9 = assign44680_e57829_d_n9;
        locals.var_pc_rv = 0.0;

        let (assign44690_e57841, assign44690_e57841_d_n4, assign44690_e57841_d_n6, assign44690_e57841_d_n7, assign44690_e57841_d_n8, assign44690_e57841_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44690_e57836: f64 = (1.0 - locals.var_k_ds);
        let assign44690_e57837: f64 = (locals.var_gf2 * assign44690_e57836);
        let assign44690_e57839: f64 = (assign44690_e57837 * locals.var_ds);
        (assign44690_e57839, ((((locals.var_gf2_dn4 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn4))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn4)), ((((locals.var_gf2_dn6 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn6))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn6)), ((((locals.var_gf2_dn7 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn7))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn7)), ((((locals.var_gf2_dn8 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn8))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn8)), ((((locals.var_gf2_dn9 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn9))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn9)),)
    } else {
        (locals.var_qc, locals.var_qc_dn4, locals.var_qc_dn6, locals.var_qc_dn7, locals.var_qc_dn8, locals.var_qc_dn9,)
    }
};
        locals.var_qc = assign44690_e57841;
        locals.var_qc_dn4 = assign44690_e57841_d_n4;
        locals.var_qc_dn6 = assign44690_e57841_d_n6;
        locals.var_qc_dn7 = assign44690_e57841_d_n7;
        locals.var_qc_dn8 = assign44690_e57841_d_n8;
        locals.var_qc_dn9 = assign44690_e57841_d_n9;
        locals.var_qc_rv = 0.0;

        let (assign44700_e57859, assign44700_e57859_d_n4, assign44700_e57859_d_n6, assign44700_e57859_d_n7, assign44700_e57859_d_n8, assign44700_e57859_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44700_e57850: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44700_e57851: f64 = (locals.var_es + assign44700_e57850);
        let assign44700_e57854: f64 = (locals.var_delta_nd * locals.var_xi2s);
        let assign44700_e57855: f64 = (assign44700_e57851 - assign44700_e57854);
        let assign44700_e57856: f64 = (locals.var_gf2 * assign44700_e57855);
        let assign44700_e57857: f64 = (2.0 - assign44700_e57856);
        (assign44700_e57857, (-((locals.var_gf2_dn4 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn4 + ((locals.var_delta_1s_dn4 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn4))) - ((locals.var_delta_nd_dn4 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn4)))))), (-((locals.var_gf2_dn6 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn6 + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn6)))))), (-((locals.var_gf2_dn7 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn7 + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn7)))))), (-((locals.var_gf2_dn8 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn8 + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn8)))))), (-((locals.var_gf2_dn9 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn9 + ((locals.var_delta_1s_dn9 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn9))) - ((locals.var_delta_nd_dn9 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn9)))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44700_e57859;
        locals.var_temp__blk949_dn4 = assign44700_e57859_d_n4;
        locals.var_temp__blk949_dn6 = assign44700_e57859_d_n6;
        locals.var_temp__blk949_dn7 = assign44700_e57859_d_n7;
        locals.var_temp__blk949_dn8 = assign44700_e57859_d_n8;
        locals.var_temp__blk949_dn9 = assign44700_e57859_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44710_e57873, assign44710_e57873_d_n4, assign44710_e57873_d_n6, assign44710_e57873_d_n7, assign44710_e57873_d_n8, assign44710_e57873_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44710_e57865: f64 = (locals.var_pc * locals.var_pc);
        let assign44710_e57869: f64 = (locals.var_temp__blk949 * locals.var_qc);
        let assign44710_e57870: f64 = (2.0 * assign44710_e57869);
        let assign44710_e57871: f64 = (assign44710_e57865 - assign44710_e57870);
        (assign44710_e57871, (((locals.var_pc_dn4 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn4)) - (2.0 * ((locals.var_temp__blk949_dn4 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn4)))), (((locals.var_pc_dn6 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn6)) - (2.0 * ((locals.var_temp__blk949_dn6 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn6)))), (((locals.var_pc_dn7 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn7)) - (2.0 * ((locals.var_temp__blk949_dn7 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn7)))), (((locals.var_pc_dn8 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn8)) - (2.0 * ((locals.var_temp__blk949_dn8 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn8)))), (((locals.var_pc_dn9 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn9)) - (2.0 * ((locals.var_temp__blk949_dn9 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44710_e57873;
        locals.var_temp__blk949_dn4 = assign44710_e57873_d_n4;
        locals.var_temp__blk949_dn6 = assign44710_e57873_d_n6;
        locals.var_temp__blk949_dn7 = assign44710_e57873_d_n7;
        locals.var_temp__blk949_dn8 = assign44710_e57873_d_n8;
        locals.var_temp__blk949_dn9 = assign44710_e57873_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44720_e57886, assign44720_e57886_d_n4, assign44720_e57886_d_n6, assign44720_e57886_d_n7, assign44720_e57886_d_n8, assign44720_e57886_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44720_e57881: f64 = (locals.var_temp__blk949).sqrt();
        let assign44720_e57882: f64 = (locals.var_pc + assign44720_e57881);
        let assign44720_e57883: f64 = (locals.var_qc / assign44720_e57882);
        let assign44720_e57884: f64 = (2.0 * assign44720_e57883);
        (assign44720_e57884, (2.0 * (((locals.var_qc_dn4 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn4 + (locals.var_temp__blk949_dn4 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn6 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn6 + (locals.var_temp__blk949_dn6 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn7 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn7 + (locals.var_temp__blk949_dn7 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn8 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn8 + (locals.var_temp__blk949_dn8 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn9 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn9 + (locals.var_temp__blk949_dn9 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign44720_e57886;
        locals.var_x_ds_dn4 = assign44720_e57886_d_n4;
        locals.var_x_ds_dn6 = assign44720_e57886_d_n6;
        locals.var_x_ds_dn7 = assign44720_e57886_d_n7;
        locals.var_x_ds_dn8 = assign44720_e57886_d_n8;
        locals.var_x_ds_dn9 = assign44720_e57886_d_n9;
        locals.var_x_ds_rv = 0.0;

        let (assign44730_e57894, assign44730_e57894_d_n4, assign44730_e57894_d_n6, assign44730_e57894_d_n7, assign44730_e57894_d_n8, assign44730_e57894_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44730_e57892: f64 = (locals.var_x_s + locals.var_x_ds);
        (assign44730_e57892, (locals.var_x_s_dn4 + locals.var_x_ds_dn4), (locals.var_x_s_dn6 + locals.var_x_ds_dn6), (locals.var_x_s_dn7 + locals.var_x_ds_dn7), (locals.var_x_s_dn8 + locals.var_x_ds_dn8), (locals.var_x_s_dn9 + locals.var_x_ds_dn9),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44730_e57894;
        locals.var_x_d_dn4 = assign44730_e57894_d_n4;
        locals.var_x_d_dn6 = assign44730_e57894_d_n6;
        locals.var_x_d_dn7 = assign44730_e57894_d_n7;
        locals.var_x_d_dn8 = assign44730_e57894_d_n8;
        locals.var_x_d_dn9 = assign44730_e57894_d_n9;
        locals.var_x_d_rv = 0.0;

        let (assign44740_e57900, assign44740_e57900_d_n4, assign44740_e57900_d_n6, assign44740_e57900_d_n7, assign44740_e57900_d_n8, assign44740_e57900_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44740_e57898: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign44740_e57898, ((locals.var_x_ds_dn4 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn4)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), ((locals.var_x_ds_dn9 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn9)),)
    } else {
        (locals.var_dps, locals.var_dps_dn4, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9,)
    }
};
        locals.var_dps = assign44740_e57900;
        locals.var_dps_dn4 = assign44740_e57900_d_n4;
        locals.var_dps_dn6 = assign44740_e57900_d_n6;
        locals.var_dps_dn7 = assign44740_e57900_d_n7;
        locals.var_dps_dn8 = assign44740_e57900_d_n8;
        locals.var_dps_dn9 = assign44740_e57900_d_n9;
        locals.var_dps_rv = 0.0;

        let (assign44750_e57912, assign44750_e57912_d_n4, assign44750_e57912_d_n6, assign44750_e57912_d_n7, assign44750_e57912_d_n8, assign44750_e57912_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44750_e57904: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44750_e57908: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44750_e57909: f64 = (2.0 + assign44750_e57908);
        let assign44750_e57910: f64 = (assign44750_e57904 / assign44750_e57909);
        (assign44750_e57910, (((((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)))) / (assign44750_e57909 * assign44750_e57909)),)
    } else {
        (locals.var_xi0d, locals.var_xi0d_dn4, locals.var_xi0d_dn6, locals.var_xi0d_dn7, locals.var_xi0d_dn8, locals.var_xi0d_dn9,)
    }
};
        locals.var_xi0d = assign44750_e57912;
        locals.var_xi0d_dn4 = assign44750_e57912_d_n4;
        locals.var_xi0d_dn6 = assign44750_e57912_d_n6;
        locals.var_xi0d_dn7 = assign44750_e57912_d_n7;
        locals.var_xi0d_dn8 = assign44750_e57912_d_n8;
        locals.var_xi0d_dn9 = assign44750_e57912_d_n9;
        locals.var_xi0d_rv = 0.0;

        let assign44760_e57915: f64 = if locals.var_x_d < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign44760_e57915;
        locals.var_guard1226_rv = 0.0;

        let (assign44770_e57923, assign44770_e57923_d_n4, assign44770_e57923_d_n6, assign44770_e57923_d_n7, assign44770_e57923_d_n8, assign44770_e57923_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign44770_e57920: f64 = (-locals.var_x_d);
        let assign44770_e57921: f64 = (assign44770_e57920).exp();
        (assign44770_e57921, (assign44770_e57921 * (-locals.var_x_d_dn4)), (assign44770_e57921 * (-locals.var_x_d_dn6)), (assign44770_e57921 * (-locals.var_x_d_dn7)), (assign44770_e57921 * (-locals.var_x_d_dn8)), (assign44770_e57921 * (-locals.var_x_d_dn9)),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44770_e57923;
        locals.var_ed_dn4 = assign44770_e57923_d_n4;
        locals.var_ed_dn6 = assign44770_e57923_d_n6;
        locals.var_ed_dn7 = assign44770_e57923_d_n7;
        locals.var_ed_dn8 = assign44770_e57923_d_n8;
        locals.var_ed_dn9 = assign44770_e57923_d_n9;
        locals.var_ed_rv = 0.0;

        let assign44780_e57926: f64 = if locals.var_x_d < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign44780_e57926;
        locals.var_guard1227_rv = 0.0;

        let (assign44790_e57950, assign44790_e57950_d_n4, assign44790_e57950_d_n6, assign44790_e57950_d_n7, assign44790_e57950_d_n8, assign44790_e57950_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44790_e57935: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44790_e57942: f64 = (0.25 * locals.var_x_d);
        let assign44790_e57943: f64 = (1.0 - assign44790_e57942);
        let assign44790_e57944: f64 = (locals.var_x_d * assign44790_e57943);
        let assign44790_e57945: f64 = (0.3333333333333333 * assign44790_e57944);
        let assign44790_e57946: f64 = (1.0 - assign44790_e57945);
        let assign44790_e57947: f64 = (assign44790_e57935 * assign44790_e57946);
        let assign44790_e57948: f64 = (0.5 * assign44790_e57947);
        (assign44790_e57948, (0.5 * ((((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn4 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn4))))))))), (0.5 * ((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6))))))))), (0.5 * ((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7))))))))), (0.5 * ((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8))))))))), (0.5 * ((((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn9 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn9))))))))),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44790_e57950;
        locals.var_pd_dn4 = assign44790_e57950_d_n4;
        locals.var_pd_dn6 = assign44790_e57950_d_n6;
        locals.var_pd_dn7 = assign44790_e57950_d_n7;
        locals.var_pd_dn8 = assign44790_e57950_d_n8;
        locals.var_pd_dn9 = assign44790_e57950_d_n9;
        locals.var_pd_rv = 0.0;

        let (assign44800_e57969, assign44800_e57969_d_n4, assign44800_e57969_d_n6, assign44800_e57969_d_n7, assign44800_e57969_d_n8, assign44800_e57969_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44800_e57962: f64 = (0.25 * locals.var_x_d);
        let assign44800_e57963: f64 = (1.0 - assign44800_e57962);
        let assign44800_e57964: f64 = (locals.var_x_d * assign44800_e57963);
        let assign44800_e57965: f64 = (0.3333333333333333 * assign44800_e57964);
        let assign44800_e57966: f64 = (1.0 - assign44800_e57965);
        let assign44800_e57967: f64 = (assign44800_e57966).sqrt();
        (assign44800_e57967, ((-(0.3333333333333333 * ((locals.var_x_d_dn4 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn4)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn9 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn9)))))) / (2.0 * assign44800_e57967)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44800_e57969;
        locals.var_temp__blk949_dn4 = assign44800_e57969_d_n4;
        locals.var_temp__blk949_dn6 = assign44800_e57969_d_n6;
        locals.var_temp__blk949_dn7 = assign44800_e57969_d_n7;
        locals.var_temp__blk949_dn8 = assign44800_e57969_d_n8;
        locals.var_temp__blk949_dn9 = assign44800_e57969_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44810_e57981, assign44810_e57981_d_n4, assign44810_e57981_d_n6, assign44810_e57981_d_n7, assign44810_e57981_d_n8, assign44810_e57981_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44810_e57978: f64 = (locals.var_x_d * locals.var_temp__blk949);
        let assign44810_e57979: f64 = (0.7071067811865475 * assign44810_e57978);
        (assign44810_e57979, (0.7071067811865475 * ((locals.var_x_d_dn4 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_d_dn6 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_d_dn7 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_d_dn8 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_d_dn9 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44810_e57981;
        locals.var_sqd_dn4 = assign44810_e57981_d_n4;
        locals.var_sqd_dn6 = assign44810_e57981_d_n6;
        locals.var_sqd_dn7 = assign44810_e57981_d_n7;
        locals.var_sqd_dn8 = assign44810_e57981_d_n8;
        locals.var_sqd_dn9 = assign44810_e57981_d_n9;
        locals.var_sqd_rv = 0.0;

        let (assign44820_e58003, assign44820_e58003_d_n4, assign44820_e58003_d_n6, assign44820_e58003_d_n7, assign44820_e58003_d_n8, assign44820_e58003_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44820_e57989: f64 = (0.16666666666666666 * locals.var_delta_nd);
        let assign44820_e57991: f64 = (assign44820_e57989 * locals.var_x_d);
        let assign44820_e57993: f64 = (assign44820_e57991 * locals.var_x_d);
        let assign44820_e57995: f64 = (assign44820_e57993 * locals.var_x_d);
        let assign44820_e57999: f64 = (1.75 * locals.var_x_d);
        let assign44820_e58000: f64 = (1.0 + assign44820_e57999);
        let assign44820_e58001: f64 = (assign44820_e57995 * assign44820_e58000);
        (assign44820_e58001, (((((((((0.16666666666666666 * locals.var_delta_nd_dn4) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn4)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn4)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn4)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn4))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn6) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn6)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn7) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn7)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn8) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn8)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn8))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn9) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn9)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn9)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn9)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn9))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44820_e58003;
        locals.var_dd_dn4 = assign44820_e58003_d_n4;
        locals.var_dd_dn6 = assign44820_e58003_d_n6;
        locals.var_dd_dn7 = assign44820_e58003_d_n7;
        locals.var_dd_dn8 = assign44820_e58003_d_n8;
        locals.var_dd_dn9 = assign44820_e58003_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign44830_e58016, assign44830_e58016_d_n4, assign44830_e58016_d_n6, assign44830_e58016_d_n7, assign44830_e58016_d_n8, assign44830_e58016_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44830_e58012: f64 = (locals.var_x_d - 1.0);
        let assign44830_e58014: f64 = (assign44830_e58012 + locals.var_ed);
        (assign44830_e58014, (locals.var_x_d_dn4 + locals.var_ed_dn4), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), (locals.var_x_d_dn9 + locals.var_ed_dn9),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44830_e58016;
        locals.var_pd_dn4 = assign44830_e58016_d_n4;
        locals.var_pd_dn6 = assign44830_e58016_d_n6;
        locals.var_pd_dn7 = assign44830_e58016_d_n7;
        locals.var_pd_dn8 = assign44830_e58016_d_n8;
        locals.var_pd_dn9 = assign44830_e58016_d_n9;
        locals.var_pd_rv = 0.0;

        let (assign44840_e58026, assign44840_e58026_d_n4, assign44840_e58026_d_n6, assign44840_e58026_d_n7, assign44840_e58026_d_n8, assign44840_e58026_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44840_e58024: f64 = (locals.var_pd).sqrt();
        (assign44840_e58024, (locals.var_pd_dn4 / (2.0 * assign44840_e58024)), (locals.var_pd_dn6 / (2.0 * assign44840_e58024)), (locals.var_pd_dn7 / (2.0 * assign44840_e58024)), (locals.var_pd_dn8 / (2.0 * assign44840_e58024)), (locals.var_pd_dn9 / (2.0 * assign44840_e58024)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44840_e58026;
        locals.var_sqd_dn4 = assign44840_e58026_d_n4;
        locals.var_sqd_dn6 = assign44840_e58026_d_n6;
        locals.var_sqd_dn7 = assign44840_e58026_d_n7;
        locals.var_sqd_dn8 = assign44840_e58026_d_n8;
        locals.var_sqd_dn9 = assign44840_e58026_d_n9;
        locals.var_sqd_rv = 0.0;

        let (assign44850_e58045, assign44850_e58045_d_n4, assign44850_e58045_d_n6, assign44850_e58045_d_n7, assign44850_e58045_d_n8, assign44850_e58045_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44850_e58036: f64 = (1.0 / locals.var_ed);
        let assign44850_e58038: f64 = (assign44850_e58036 - locals.var_x_d);
        let assign44850_e58040: f64 = (assign44850_e58038 - 1.0);
        let assign44850_e58042: f64 = (assign44850_e58040 - locals.var_xi0d);
        let assign44850_e58043: f64 = (locals.var_delta_nd * assign44850_e58042);
        (assign44850_e58043, ((locals.var_delta_nd_dn4 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn4 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn4) - locals.var_xi0d_dn4))), ((locals.var_delta_nd_dn6 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn6 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn6) - locals.var_xi0d_dn6))), ((locals.var_delta_nd_dn7 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn7 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn7) - locals.var_xi0d_dn7))), ((locals.var_delta_nd_dn8 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn8 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn8) - locals.var_xi0d_dn8))), ((locals.var_delta_nd_dn9 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn9 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn9) - locals.var_xi0d_dn9))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44850_e58045;
        locals.var_dd_dn4 = assign44850_e58045_d_n4;
        locals.var_dd_dn6 = assign44850_e58045_d_n6;
        locals.var_dd_dn7 = assign44850_e58045_d_n7;
        locals.var_dd_dn8 = assign44850_e58045_d_n8;
        locals.var_dd_dn9 = assign44850_e58045_d_n9;
        locals.var_dd_rv = 0.0;

        let assign44860_e58049: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44860_e58050: f64 = if locals.var_x_d > assign44860_e58049 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign44860_e58050;
        locals.var_guard1228_rv = 0.0;

        let (assign44870_e58062, assign44870_e58062_d_n4, assign44870_e58062_d_n6, assign44870_e58062_d_n7, assign44870_e58062_d_n8, assign44870_e58062_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44870_e58059: f64 = (locals.var_x_d - locals.var_xn_d);
        let assign44870_e58060: f64 = (assign44870_e58059).exp();
        (assign44870_e58060, (assign44870_e58060 * (locals.var_x_d_dn4 - locals.var_xn_d_dn4)), (assign44870_e58060 * (locals.var_x_d_dn6 - locals.var_xn_d_dn6)), (assign44870_e58060 * (locals.var_x_d_dn7 - locals.var_xn_d_dn7)), (assign44870_e58060 * (locals.var_x_d_dn8 - locals.var_xn_d_dn8)), (assign44870_e58060 * (locals.var_x_d_dn9 - locals.var_xn_d_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44870_e58062;
        locals.var_temp__blk949_dn4 = assign44870_e58062_d_n4;
        locals.var_temp__blk949_dn6 = assign44870_e58062_d_n6;
        locals.var_temp__blk949_dn7 = assign44870_e58062_d_n7;
        locals.var_temp__blk949_dn8 = assign44870_e58062_d_n8;
        locals.var_temp__blk949_dn9 = assign44870_e58062_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44880_e58073, assign44880_e58073_d_n4, assign44880_e58073_d_n6, assign44880_e58073_d_n7, assign44880_e58073_d_n8, assign44880_e58073_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44880_e58071: f64 = (locals.var_delta_nd / locals.var_temp__blk949);
        (assign44880_e58071, (((locals.var_delta_nd_dn4 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn6 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn7 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn8 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn9 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44880_e58073;
        locals.var_ed_dn4 = assign44880_e58073_d_n4;
        locals.var_ed_dn6 = assign44880_e58073_d_n6;
        locals.var_ed_dn7 = assign44880_e58073_d_n7;
        locals.var_ed_dn8 = assign44880_e58073_d_n8;
        locals.var_ed_dn9 = assign44880_e58073_d_n9;
        locals.var_ed_rv = 0.0;

        let (assign44890_e58090, assign44890_e58090_d_n4, assign44890_e58090_d_n6, assign44890_e58090_d_n7, assign44890_e58090_d_n8, assign44890_e58090_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44890_e58084: f64 = (locals.var_x_d + 1.0);
        let assign44890_e58086: f64 = (assign44890_e58084 + locals.var_xi0d);
        let assign44890_e58087: f64 = (locals.var_delta_nd * assign44890_e58086);
        let assign44890_e58088: f64 = (locals.var_temp__blk949 - assign44890_e58087);
        (assign44890_e58088, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd_dn4 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn4 + locals.var_xi0d_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd_dn6 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd_dn7 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd_dn8 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd_dn9 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn9 + locals.var_xi0d_dn9)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44890_e58090;
        locals.var_dd_dn4 = assign44890_e58090_d_n4;
        locals.var_dd_dn6 = assign44890_e58090_d_n6;
        locals.var_dd_dn7 = assign44890_e58090_d_n7;
        locals.var_dd_dn8 = assign44890_e58090_d_n8;
        locals.var_dd_dn9 = assign44890_e58090_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign44900_e58122, assign44900_e58122_d_n4, assign44900_e58122_d_n6, assign44900_e58122_d_n7, assign44900_e58122_d_n8, assign44900_e58122_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44900_e58102: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58107: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58111: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58113: f64 = (assign44900_e58111 * 0.3333333333333333);
        let assign44900_e58114: f64 = (1.0 + assign44900_e58113);
        let assign44900_e58115: f64 = (assign44900_e58107 * assign44900_e58114);
        let assign44900_e58116: f64 = (0.5 * assign44900_e58115);
        let assign44900_e58117: f64 = (1.0 + assign44900_e58116);
        let assign44900_e58118: f64 = (assign44900_e58102 * assign44900_e58117);
        let assign44900_e58119: f64 = (1.0 + assign44900_e58118);
        let assign44900_e58120: f64 = (1e-100 / assign44900_e58119);
        (assign44900_e58120, (-((1e-100 * ((locals.var_x_d_dn4 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn4 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn4 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn6 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn6 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn6 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn7 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn7 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn7 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn8 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn8 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn8 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn9 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn9 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn9 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44900_e58122;
        locals.var_ed_dn4 = assign44900_e58122_d_n4;
        locals.var_ed_dn6 = assign44900_e58122_d_n6;
        locals.var_ed_dn7 = assign44900_e58122_d_n7;
        locals.var_ed_dn8 = assign44900_e58122_d_n8;
        locals.var_ed_dn9 = assign44900_e58122_d_n9;
        locals.var_ed_rv = 0.0;

        let (assign44910_e58160, assign44910_e58160_d_n4, assign44910_e58160_d_n6, assign44910_e58160_d_n7, assign44910_e58160_d_n8, assign44910_e58160_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44910_e58134: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58136: f64 = (assign44910_e58134 - 230.25850929940458);
        let assign44910_e58141: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58143: f64 = (assign44910_e58141 - 230.25850929940458);
        let assign44910_e58147: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58149: f64 = (assign44910_e58147 - 230.25850929940458);
        let assign44910_e58151: f64 = (assign44910_e58149 * 0.3333333333333333);
        let assign44910_e58152: f64 = (1.0 + assign44910_e58151);
        let assign44910_e58153: f64 = (assign44910_e58143 * assign44910_e58152);
        let assign44910_e58154: f64 = (0.5 * assign44910_e58153);
        let assign44910_e58155: f64 = (1.0 + assign44910_e58154);
        let assign44910_e58156: f64 = (assign44910_e58136 * assign44910_e58155);
        let assign44910_e58157: f64 = (1.0 + assign44910_e58156);
        let assign44910_e58158: f64 = (1e-100 / assign44910_e58157);
        (assign44910_e58158, (-((1e-100 * (((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44910_e58160;
        locals.var_temp__blk949_dn4 = assign44910_e58160_d_n4;
        locals.var_temp__blk949_dn6 = assign44910_e58160_d_n6;
        locals.var_temp__blk949_dn7 = assign44910_e58160_d_n7;
        locals.var_temp__blk949_dn8 = assign44910_e58160_d_n8;
        locals.var_temp__blk949_dn9 = assign44910_e58160_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign44920_e58178, assign44920_e58178_d_n4, assign44920_e58178_d_n6, assign44920_e58178_d_n7, assign44920_e58178_d_n8, assign44920_e58178_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44920_e58172: f64 = (locals.var_x_d + 1.0);
        let assign44920_e58174: f64 = (assign44920_e58172 + locals.var_xi0d);
        let assign44920_e58175: f64 = (locals.var_delta_nd * assign44920_e58174);
        let assign44920_e58176: f64 = (locals.var_temp__blk949 - assign44920_e58175);
        (assign44920_e58176, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd_dn4 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn4 + locals.var_xi0d_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd_dn6 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd_dn7 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd_dn8 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd_dn9 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn9 + locals.var_xi0d_dn9)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44920_e58178;
        locals.var_dd_dn4 = assign44920_e58178_d_n4;
        locals.var_dd_dn6 = assign44920_e58178_d_n6;
        locals.var_dd_dn7 = assign44920_e58178_d_n7;
        locals.var_dd_dn8 = assign44920_e58178_d_n8;
        locals.var_dd_dn9 = assign44920_e58178_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign44930_e58189, assign44930_e58189_d_n4, assign44930_e58189_d_n6, assign44930_e58189_d_n7, assign44930_e58189_d_n8, assign44930_e58189_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign44930_e58185: f64 = (locals.var_x_d - 1.0);
        let assign44930_e58187: f64 = (assign44930_e58185 + locals.var_ed);
        (assign44930_e58187, (locals.var_x_d_dn4 + locals.var_ed_dn4), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), (locals.var_x_d_dn9 + locals.var_ed_dn9),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44930_e58189;
        locals.var_pd_dn4 = assign44930_e58189_d_n4;
        locals.var_pd_dn6 = assign44930_e58189_d_n6;
        locals.var_pd_dn7 = assign44930_e58189_d_n7;
        locals.var_pd_dn8 = assign44930_e58189_d_n8;
        locals.var_pd_dn9 = assign44930_e58189_d_n9;
        locals.var_pd_rv = 0.0;

        let (assign44940_e58197, assign44940_e58197_d_n4, assign44940_e58197_d_n6, assign44940_e58197_d_n7, assign44940_e58197_d_n8, assign44940_e58197_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign44940_e58195: f64 = (locals.var_pd).sqrt();
        (assign44940_e58195, (locals.var_pd_dn4 / (2.0 * assign44940_e58195)), (locals.var_pd_dn6 / (2.0 * assign44940_e58195)), (locals.var_pd_dn7 / (2.0 * assign44940_e58195)), (locals.var_pd_dn8 / (2.0 * assign44940_e58195)), (locals.var_pd_dn9 / (2.0 * assign44940_e58195)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44940_e58197;
        locals.var_sqd_dn4 = assign44940_e58197_d_n4;
        locals.var_sqd_dn6 = assign44940_e58197_d_n6;
        locals.var_sqd_dn7 = assign44940_e58197_d_n7;
        locals.var_sqd_dn8 = assign44940_e58197_d_n8;
        locals.var_sqd_dn9 = assign44940_e58197_d_n9;
        locals.var_sqd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign44950_e58205, assign44950_e58205_d_n4, assign44950_e58205_d_n6, assign44950_e58205_d_n7, assign44950_e58205_d_n8, assign44950_e58205_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44950_e58201: f64 = (locals.var_sqd * locals.var_gf);
        let assign44950_e58203: f64 = (assign44950_e58201 * locals.var_phit1);
        (assign44950_e58203, ((((locals.var_sqd_dn4 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn4)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn4)), ((((locals.var_sqd_dn6 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn6)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn6)), ((((locals.var_sqd_dn7 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn7)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn7)), ((((locals.var_sqd_dn8 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn8)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn8)), ((((locals.var_sqd_dn9 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn9)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn4, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9,)
    }
};
        locals.var_qbd = assign44950_e58205;
        locals.var_qbd_dn4 = assign44950_e58205_d_n4;
        locals.var_qbd_dn6 = assign44950_e58205_d_n6;
        locals.var_qbd_dn7 = assign44950_e58205_d_n7;
        locals.var_qbd_dn8 = assign44950_e58205_d_n8;
        locals.var_qbd_dn9 = assign44950_e58205_d_n9;
        locals.var_qbd_rv = 0.0;

        let (assign44960_e58213, assign44960_e58213_d_n4, assign44960_e58213_d_n6, assign44960_e58213_d_n7, assign44960_e58213_d_n8, assign44960_e58213_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44960_e58210: f64 = (locals.var_x_s + locals.var_x_d);
        let assign44960_e58211: f64 = (0.5 * assign44960_e58210);
        (assign44960_e58211, (0.5 * (locals.var_x_s_dn4 + locals.var_x_d_dn4)), (0.5 * (locals.var_x_s_dn6 + locals.var_x_d_dn6)), (0.5 * (locals.var_x_s_dn7 + locals.var_x_d_dn7)), (0.5 * (locals.var_x_s_dn8 + locals.var_x_d_dn8)), (0.5 * (locals.var_x_s_dn9 + locals.var_x_d_dn9)),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign44960_e58213;
        locals.var_x_m_dn4 = assign44960_e58213_d_n4;
        locals.var_x_m_dn6 = assign44960_e58213_d_n6;
        locals.var_x_m_dn7 = assign44960_e58213_d_n7;
        locals.var_x_m_dn8 = assign44960_e58213_d_n8;
        locals.var_x_m_dn9 = assign44960_e58213_d_n9;
        locals.var_x_m_rv = 0.0;

        let (assign44970_e58217, assign44970_e58217_d_n4, assign44970_e58217_d_n6, assign44970_e58217_d_n7, assign44970_e58217_d_n8, assign44970_e58217_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign44970_e58217;
        locals.var_em_dn4 = assign44970_e58217_d_n4;
        locals.var_em_dn6 = assign44970_e58217_d_n6;
        locals.var_em_dn7 = assign44970_e58217_d_n7;
        locals.var_em_dn8 = assign44970_e58217_d_n8;
        locals.var_em_dn9 = assign44970_e58217_d_n9;
        locals.var_em_rv = 0.0;

        let (assign44980_e58223, assign44980_e58223_d_n4, assign44980_e58223_d_n6, assign44980_e58223_d_n7, assign44980_e58223_d_n8, assign44980_e58223_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44980_e58221: f64 = (locals.var_ed * locals.var_es);
        (assign44980_e58221, ((locals.var_ed_dn4 * locals.var_es) + (locals.var_ed * locals.var_es_dn4)), ((locals.var_ed_dn6 * locals.var_es) + (locals.var_ed * locals.var_es_dn6)), ((locals.var_ed_dn7 * locals.var_es) + (locals.var_ed * locals.var_es_dn7)), ((locals.var_ed_dn8 * locals.var_es) + (locals.var_ed * locals.var_es_dn8)), ((locals.var_ed_dn9 * locals.var_es) + (locals.var_ed * locals.var_es_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44980_e58223;
        locals.var_temp__blk949_dn4 = assign44980_e58223_d_n4;
        locals.var_temp__blk949_dn6 = assign44980_e58223_d_n6;
        locals.var_temp__blk949_dn7 = assign44980_e58223_d_n7;
        locals.var_temp__blk949_dn8 = assign44980_e58223_d_n8;
        locals.var_temp__blk949_dn9 = assign44980_e58223_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign44990_e58226: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign44990_e58226;
        locals.var_guard1229_rv = 0.0;

        let (assign45000_e58233, assign45000_e58233_d_n4, assign45000_e58233_d_n6, assign45000_e58233_d_n7, assign45000_e58233_d_n8, assign45000_e58233_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1229 != 0.0)) {
        let assign45000_e58231: f64 = (locals.var_temp__blk949).sqrt();
        (assign45000_e58231, (locals.var_temp__blk949_dn4 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn6 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn7 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn8 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn9 / (2.0 * assign45000_e58231)),)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign45000_e58233;
        locals.var_em_dn4 = assign45000_e58233_d_n4;
        locals.var_em_dn6 = assign45000_e58233_d_n6;
        locals.var_em_dn7 = assign45000_e58233_d_n7;
        locals.var_em_dn8 = assign45000_e58233_d_n8;
        locals.var_em_dn9 = assign45000_e58233_d_n9;
        locals.var_em_rv = 0.0;

        let (assign45010_e58241, assign45010_e58241_d_n4, assign45010_e58241_d_n6, assign45010_e58241_d_n7, assign45010_e58241_d_n8, assign45010_e58241_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45010_e58238: f64 = (locals.var_ds + locals.var_dd);
        let assign45010_e58239: f64 = (0.5 * assign45010_e58238);
        (assign45010_e58239, (0.5 * (locals.var_ds_dn4 + locals.var_dd_dn4)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)), (0.5 * (locals.var_ds_dn9 + locals.var_dd_dn9)),)
    } else {
        (locals.var_d_bar, locals.var_d_bar_dn4, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8, locals.var_d_bar_dn9,)
    }
};
        locals.var_d_bar = assign45010_e58241;
        locals.var_d_bar_dn4 = assign45010_e58241_d_n4;
        locals.var_d_bar_dn6 = assign45010_e58241_d_n6;
        locals.var_d_bar_dn7 = assign45010_e58241_d_n7;
        locals.var_d_bar_dn8 = assign45010_e58241_d_n8;
        locals.var_d_bar_dn9 = assign45010_e58241_d_n9;
        locals.var_d_bar_rv = 0.0;

        let (assign45020_e58257, assign45020_e58257_d_n4, assign45020_e58257_d_n6, assign45020_e58257_d_n7, assign45020_e58257_d_n8, assign45020_e58257_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45020_e58247: f64 = (locals.var_x_ds * locals.var_x_ds);
        let assign45020_e58251: f64 = (2.0 * locals.var_inv_gf2);
        let assign45020_e58252: f64 = (locals.var_em - assign45020_e58251);
        let assign45020_e58253: f64 = (assign45020_e58247 * assign45020_e58252);
        let assign45020_e58254: f64 = (0.125 * assign45020_e58253);
        let assign45020_e58255: f64 = (locals.var_d_bar + assign45020_e58254);
        (assign45020_e58255, (locals.var_d_bar_dn4 + (0.125 * ((((locals.var_x_ds_dn4 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn4)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn4 - (2.0 * locals.var_inv_gf2_dn4)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gf2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gf2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gf2_dn8)))))), (locals.var_d_bar_dn9 + (0.125 * ((((locals.var_x_ds_dn9 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn9)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn9 - (2.0 * locals.var_inv_gf2_dn9)))))),)
    } else {
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9,)
    }
};
        locals.var_dm = assign45020_e58257;
        locals.var_dm_dn4 = assign45020_e58257_d_n4;
        locals.var_dm_dn6 = assign45020_e58257_d_n6;
        locals.var_dm_dn7 = assign45020_e58257_d_n7;
        locals.var_dm_dn8 = assign45020_e58257_d_n8;
        locals.var_dm_dn9 = assign45020_e58257_d_n9;
        locals.var_dm_rv = 0.0;

        let assign45030_e58260: f64 = if locals.var_x_m < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign45030_e58260;
        locals.var_guard1230_rv = 0.0;

        let (assign45040_e58282, assign45040_e58282_d_n4, assign45040_e58282_d_n6, assign45040_e58282_d_n7, assign45040_e58282_d_n8, assign45040_e58282_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45040_e58267: f64 = (locals.var_x_m * locals.var_x_m);
        let assign45040_e58274: f64 = (0.25 * locals.var_x_m);
        let assign45040_e58275: f64 = (1.0 - assign45040_e58274);
        let assign45040_e58276: f64 = (locals.var_x_m * assign45040_e58275);
        let assign45040_e58277: f64 = (0.3333333333333333 * assign45040_e58276);
        let assign45040_e58278: f64 = (1.0 - assign45040_e58277);
        let assign45040_e58279: f64 = (assign45040_e58267 * assign45040_e58278);
        let assign45040_e58280: f64 = (0.5 * assign45040_e58279);
        (assign45040_e58280, (0.5 * ((((locals.var_x_m_dn4 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn4)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn4 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn4))))))))), (0.5 * ((((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6))))))))), (0.5 * ((((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7))))))))), (0.5 * ((((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8))))))))), (0.5 * ((((locals.var_x_m_dn9 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn9)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn9 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn9))))))))),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45040_e58282;
        locals.var_pm_dn4 = assign45040_e58282_d_n4;
        locals.var_pm_dn6 = assign45040_e58282_d_n6;
        locals.var_pm_dn7 = assign45040_e58282_d_n7;
        locals.var_pm_dn8 = assign45040_e58282_d_n8;
        locals.var_pm_dn9 = assign45040_e58282_d_n9;
        locals.var_pm_rv = 0.0;

        let (assign45050_e58293, assign45050_e58293_d_n4, assign45050_e58293_d_n6, assign45050_e58293_d_n7, assign45050_e58293_d_n8, assign45050_e58293_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45050_e58289: f64 = (locals.var_dm + locals.var_pm);
        let assign45050_e58290: f64 = (assign45050_e58289).sqrt();
        let assign45050_e58291: f64 = (locals.var_gf * assign45050_e58290);
        (assign45050_e58291, ((locals.var_gf_dn4 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn6 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn7 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn8 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn9 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45050_e58290)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45050_e58293;
        locals.var_xgm_dn4 = assign45050_e58293_d_n4;
        locals.var_xgm_dn6 = assign45050_e58293_d_n6;
        locals.var_xgm_dn7 = assign45050_e58293_d_n7;
        locals.var_xgm_dn8 = assign45050_e58293_d_n8;
        locals.var_xgm_dn9 = assign45050_e58293_d_n9;
        locals.var_xgm_rv = 0.0;

        let assign45060_e58296: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign45060_e58296;
        locals.var_guard1231_rv = 0.0;

        let (assign45070_e58311, assign45070_e58311_d_n4, assign45070_e58311_d_n6, assign45070_e58311_d_n7, assign45070_e58311_d_n8, assign45070_e58311_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign45070_e58306: f64 = (locals.var_kp * locals.var_xgm);
        let assign45070_e58307: f64 = (1.0 + assign45070_e58306);
        let assign45070_e58308: f64 = (assign45070_e58307).sqrt();
        let assign45070_e58309: f64 = (1.0 / assign45070_e58308);
        (assign45070_e58309, (-((((locals.var_kp_dn4 * locals.var_xgm) + (locals.var_kp * locals.var_xgm_dn4)) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn9) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn4, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, locals.var_eta_p_dn9,)
    }
};
        locals.var_eta_p = assign45070_e58311;
        locals.var_eta_p_dn4 = assign45070_e58311_d_n4;
        locals.var_eta_p_dn6 = assign45070_e58311_d_n6;
        locals.var_eta_p_dn7 = assign45070_e58311_d_n7;
        locals.var_eta_p_dn8 = assign45070_e58311_d_n8;
        locals.var_eta_p_dn9 = assign45070_e58311_d_n9;
        locals.var_eta_p_rv = 0.0;

        let (assign45080_e58328, assign45080_e58328_d_n4, assign45080_e58328_d_n6, assign45080_e58328_d_n7, assign45080_e58328_d_n8, assign45080_e58328_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45080_e58321: f64 = (0.25 * locals.var_x_m);
        let assign45080_e58322: f64 = (1.0 - assign45080_e58321);
        let assign45080_e58323: f64 = (locals.var_x_m * assign45080_e58322);
        let assign45080_e58324: f64 = (0.3333333333333333 * assign45080_e58323);
        let assign45080_e58325: f64 = (1.0 - assign45080_e58324);
        let assign45080_e58326: f64 = (assign45080_e58325).sqrt();
        (assign45080_e58326, ((-(0.3333333333333333 * ((locals.var_x_m_dn4 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn4)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn9 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn9)))))) / (2.0 * assign45080_e58326)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45080_e58328;
        locals.var_temp__blk949_dn4 = assign45080_e58328_d_n4;
        locals.var_temp__blk949_dn6 = assign45080_e58328_d_n6;
        locals.var_temp__blk949_dn7 = assign45080_e58328_d_n7;
        locals.var_temp__blk949_dn8 = assign45080_e58328_d_n8;
        locals.var_temp__blk949_dn9 = assign45080_e58328_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign45090_e58338, assign45090_e58338_d_n4, assign45090_e58338_d_n6, assign45090_e58338_d_n7, assign45090_e58338_d_n8, assign45090_e58338_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45090_e58335: f64 = (locals.var_x_m * locals.var_temp__blk949);
        let assign45090_e58336: f64 = (0.7071067811865475 * assign45090_e58335);
        (assign45090_e58336, (0.7071067811865475 * ((locals.var_x_m_dn4 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_m_dn6 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_m_dn7 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_m_dn8 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_m_dn9 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn4, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9,)
    }
};
        locals.var_sqm = assign45090_e58338;
        locals.var_sqm_dn4 = assign45090_e58338_d_n4;
        locals.var_sqm_dn6 = assign45090_e58338_d_n6;
        locals.var_sqm_dn7 = assign45090_e58338_d_n7;
        locals.var_sqm_dn8 = assign45090_e58338_d_n8;
        locals.var_sqm_dn9 = assign45090_e58338_d_n9;
        locals.var_sqm_rv = 0.0;

        let (assign45100_e58362, assign45100_e58362_d_n4, assign45100_e58362_d_n6, assign45100_e58362_d_n7, assign45100_e58362_d_n8, assign45100_e58362_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45100_e58348: f64 = (0.5 * locals.var_x_m);
        let assign45100_e58349: f64 = (1.0 - assign45100_e58348);
        let assign45100_e58353: f64 = (locals.var_x_m * locals.var_x_m);
        let assign45100_e58354: f64 = (0.16666666666666666 * assign45100_e58353);
        let assign45100_e58355: f64 = (assign45100_e58349 + assign45100_e58354);
        let assign45100_e58356: f64 = (locals.var_gf * assign45100_e58355);
        let assign45100_e58358: f64 = (assign45100_e58356 / locals.var_temp__blk949);
        let assign45100_e58359: f64 = (0.7071067811865475 * assign45100_e58358);
        let assign45100_e58360: f64 = (locals.var_eta_p + assign45100_e58359);
        (assign45100_e58360, (locals.var_eta_p_dn4 + (0.7071067811865475 * (((((locals.var_gf_dn4 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn4)) + (0.16666666666666666 * ((locals.var_x_m_dn4 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn4)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn6 + (0.7071067811865475 * (((((locals.var_gf_dn6 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn6)) + (0.16666666666666666 * ((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn7 + (0.7071067811865475 * (((((locals.var_gf_dn7 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn7)) + (0.16666666666666666 * ((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn8 + (0.7071067811865475 * (((((locals.var_gf_dn8 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn8)) + (0.16666666666666666 * ((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn9 + (0.7071067811865475 * (((((locals.var_gf_dn9 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn9)) + (0.16666666666666666 * ((locals.var_x_m_dn9 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn9)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn4, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9,)
    }
};
        locals.var_alpha = assign45100_e58362;
        locals.var_alpha_dn4 = assign45100_e58362_d_n4;
        locals.var_alpha_dn6 = assign45100_e58362_d_n6;
        locals.var_alpha_dn7 = assign45100_e58362_d_n7;
        locals.var_alpha_dn8 = assign45100_e58362_d_n8;
        locals.var_alpha_dn9 = assign45100_e58362_d_n9;
        locals.var_alpha_rv = 0.0;

        let (assign45110_e58373, assign45110_e58373_d_n4, assign45110_e58373_d_n6, assign45110_e58373_d_n7, assign45110_e58373_d_n8, assign45110_e58373_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45110_e58369: f64 = (locals.var_x_m - 1.0);
        let assign45110_e58371: f64 = (assign45110_e58369 + locals.var_em);
        (assign45110_e58371, (locals.var_x_m_dn4 + locals.var_em_dn4), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), (locals.var_x_m_dn9 + locals.var_em_dn9),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45110_e58373;
        locals.var_pm_dn4 = assign45110_e58373_d_n4;
        locals.var_pm_dn6 = assign45110_e58373_d_n6;
        locals.var_pm_dn7 = assign45110_e58373_d_n7;
        locals.var_pm_dn8 = assign45110_e58373_d_n8;
        locals.var_pm_dn9 = assign45110_e58373_d_n9;
        locals.var_pm_rv = 0.0;

        let (assign45120_e58385, assign45120_e58385_d_n4, assign45120_e58385_d_n6, assign45120_e58385_d_n7, assign45120_e58385_d_n8, assign45120_e58385_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45120_e58381: f64 = (locals.var_dm + locals.var_pm);
        let assign45120_e58382: f64 = (assign45120_e58381).sqrt();
        let assign45120_e58383: f64 = (locals.var_gf * assign45120_e58382);
        (assign45120_e58383, ((locals.var_gf_dn4 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn6 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn7 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn8 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn9 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45120_e58382)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45120_e58385;
        locals.var_xgm_dn4 = assign45120_e58385_d_n4;
        locals.var_xgm_dn6 = assign45120_e58385_d_n6;
        locals.var_xgm_dn7 = assign45120_e58385_d_n7;
        locals.var_xgm_dn8 = assign45120_e58385_d_n8;
        locals.var_xgm_dn9 = assign45120_e58385_d_n9;
        locals.var_xgm_rv = 0.0;

        let assign45130_e58388: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign45130_e58388;
        locals.var_guard1232_rv = 0.0;

        let (assign45140_e58405, assign45140_e58405_d_n4, assign45140_e58405_d_n6, assign45140_e58405_d_n7, assign45140_e58405_d_n8, assign45140_e58405_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45140_e58397: f64 = (1.0 - locals.var_em);
        let assign45140_e58401: f64 = (locals.var_xgm * locals.var_inv_gf2);
        let assign45140_e58402: f64 = (2.0 * assign45140_e58401);
        let assign45140_e58403: f64 = (assign45140_e58397 + assign45140_e58402);
        (assign45140_e58403, ((-locals.var_em_dn4) + (2.0 * ((locals.var_xgm_dn4 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn4)))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn8)))), ((-locals.var_em_dn9) + (2.0 * ((locals.var_xgm_dn9 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn9)))),)
    } else {
        (locals.var_d0, locals.var_d0_dn4, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8, locals.var_d0_dn9,)
    }
};
        locals.var_d0 = assign45140_e58405;
        locals.var_d0_dn4 = assign45140_e58405_d_n4;
        locals.var_d0_dn6 = assign45140_e58405_d_n6;
        locals.var_d0_dn7 = assign45140_e58405_d_n7;
        locals.var_d0_dn8 = assign45140_e58405_d_n8;
        locals.var_d0_dn9 = assign45140_e58405_d_n9;
        locals.var_d0_rv = 0.0;

        let (assign45150_e58421, assign45150_e58421_d_n4, assign45150_e58421_d_n6, assign45150_e58421_d_n7, assign45150_e58421_d_n8, assign45150_e58421_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45150_e58416: f64 = (locals.var_kp * locals.var_xgm);
        let assign45150_e58417: f64 = (1.0 + assign45150_e58416);
        let assign45150_e58418: f64 = (assign45150_e58417).sqrt();
        let assign45150_e58419: f64 = (1.0 / assign45150_e58418);
        (assign45150_e58419, (-((((locals.var_kp_dn4 * locals.var_xgm) + (locals.var_kp * locals.var_xgm_dn4)) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn9) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn4, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, locals.var_eta_p_dn9,)
    }
};
        locals.var_eta_p = assign45150_e58421;
        locals.var_eta_p_dn4 = assign45150_e58421_d_n4;
        locals.var_eta_p_dn6 = assign45150_e58421_d_n6;
        locals.var_eta_p_dn7 = assign45150_e58421_d_n7;
        locals.var_eta_p_dn8 = assign45150_e58421_d_n8;
        locals.var_eta_p_dn9 = assign45150_e58421_d_n9;
        locals.var_eta_p_rv = 0.0;

        let (assign45160_e58434, assign45160_e58434_d_n4, assign45160_e58434_d_n6, assign45160_e58434_d_n7, assign45160_e58434_d_n8, assign45160_e58434_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45160_e58431: f64 = (locals.var_eta_p + 1.0);
        let assign45160_e58432: f64 = (locals.var_eta_p / assign45160_e58431);
        (assign45160_e58432, (((locals.var_eta_p_dn4 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn4)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn6 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn7 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn8 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn9 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn9)) / (assign45160_e58431 * assign45160_e58431)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45160_e58434;
        locals.var_temp__blk949_dn4 = assign45160_e58434_d_n4;
        locals.var_temp__blk949_dn6 = assign45160_e58434_d_n6;
        locals.var_temp__blk949_dn7 = assign45160_e58434_d_n7;
        locals.var_temp__blk949_dn8 = assign45160_e58434_d_n8;
        locals.var_temp__blk949_dn9 = assign45160_e58434_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign45170_e58451, assign45170_e58451_d_n4, assign45170_e58451_d_n6, assign45170_e58451_d_n7, assign45170_e58451_d_n8, assign45170_e58451_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45170_e58444: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign45170_e58446: f64 = (assign45170_e58444 * locals.var_gf2);
        let assign45170_e58448: f64 = (assign45170_e58446 * locals.var_dm);
        let assign45170_e58449: f64 = (locals.var_kp * assign45170_e58448);
        (assign45170_e58449, ((locals.var_kp_dn4 * assign45170_e58448) + (locals.var_kp * ((((((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn4)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn4)))), (locals.var_kp * ((((((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn6)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn6))), (locals.var_kp * ((((((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn7)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn7))), (locals.var_kp * ((((((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn8)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn8))), (locals.var_kp * ((((((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn9)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn9))),)
    } else {
        (locals.var_x_pm, locals.var_x_pm_dn4, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8, locals.var_x_pm_dn9,)
    }
};
        locals.var_x_pm = assign45170_e58451;
        locals.var_x_pm_dn4 = assign45170_e58451_d_n4;
        locals.var_x_pm_dn6 = assign45170_e58451_d_n6;
        locals.var_x_pm_dn7 = assign45170_e58451_d_n7;
        locals.var_x_pm_dn8 = assign45170_e58451_d_n8;
        locals.var_x_pm_dn9 = assign45170_e58451_d_n9;
        locals.var_x_pm_rv = 0.0;

        let (assign45180_e58472, assign45180_e58472_d_n4, assign45180_e58472_d_n6, assign45180_e58472_d_n7, assign45180_e58472_d_n8, assign45180_e58472_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45180_e58461: f64 = (locals.var_xgm - locals.var_x_pm);
        let assign45180_e58462: f64 = (2.0 * assign45180_e58461);
        let assign45180_e58466: f64 = (1.0 - locals.var_em);
        let assign45180_e58468: f64 = (assign45180_e58466 + locals.var_dm);
        let assign45180_e58469: f64 = (locals.var_gf2 * assign45180_e58468);
        let assign45180_e58470: f64 = (assign45180_e58462 + assign45180_e58469);
        (assign45180_e58470, ((2.0 * (locals.var_xgm_dn4 - locals.var_x_pm_dn4)) + ((locals.var_gf2_dn4 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn4) + locals.var_dm_dn4)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gf2_dn6 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gf2_dn7 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gf2_dn8 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))), ((2.0 * (locals.var_xgm_dn9 - locals.var_x_pm_dn9)) + ((locals.var_gf2_dn9 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn9) + locals.var_dm_dn9)))),)
    } else {
        (locals.var_p_pd, locals.var_p_pd_dn4, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8, locals.var_p_pd_dn9,)
    }
};
        locals.var_p_pd = assign45180_e58472;
        locals.var_p_pd_dn4 = assign45180_e58472_d_n4;
        locals.var_p_pd_dn6 = assign45180_e58472_d_n6;
        locals.var_p_pd_dn7 = assign45180_e58472_d_n7;
        locals.var_p_pd_dn8 = assign45180_e58472_d_n8;
        locals.var_p_pd_dn9 = assign45180_e58472_d_n9;
        locals.var_p_pd_rv = 0.0;

        let (assign45190_e58487, assign45190_e58487_d_n4, assign45190_e58487_d_n6, assign45190_e58487_d_n7, assign45190_e58487_d_n8, assign45190_e58487_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45190_e58483: f64 = (2.0 * locals.var_xgm);
        let assign45190_e58484: f64 = (locals.var_x_pm - assign45190_e58483);
        let assign45190_e58485: f64 = (locals.var_x_pm * assign45190_e58484);
        (assign45190_e58485, ((locals.var_x_pm_dn4 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn4 - (2.0 * locals.var_xgm_dn4)))), ((locals.var_x_pm_dn6 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))), ((locals.var_x_pm_dn9 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn9 - (2.0 * locals.var_xgm_dn9)))),)
    } else {
        (locals.var_q_pd, locals.var_q_pd_dn4, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8, locals.var_q_pd_dn9,)
    }
};
        locals.var_q_pd = assign45190_e58487;
        locals.var_q_pd_dn4 = assign45190_e58487_d_n4;
        locals.var_q_pd_dn6 = assign45190_e58487_d_n6;
        locals.var_q_pd_dn7 = assign45190_e58487_d_n7;
        locals.var_q_pd_dn8 = assign45190_e58487_d_n8;
        locals.var_q_pd_dn9 = assign45190_e58487_d_n9;
        locals.var_q_pd_rv = 0.0;

        let (assign45200_e58504, assign45200_e58504_d_n4, assign45200_e58504_d_n6, assign45200_e58504_d_n7, assign45200_e58504_d_n8, assign45200_e58504_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45200_e58499: f64 = (locals.var_em + locals.var_dm);
        let assign45200_e58500: f64 = (locals.var_gf2 * assign45200_e58499);
        let assign45200_e58501: f64 = (0.5 * assign45200_e58500);
        let assign45200_e58502: f64 = (1.0 - assign45200_e58501);
        (assign45200_e58502, (-(0.5 * ((locals.var_gf2_dn4 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn4 + locals.var_dm_dn4))))), (-(0.5 * ((locals.var_gf2_dn6 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gf2_dn7 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gf2_dn8 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn8 + locals.var_dm_dn8))))), (-(0.5 * ((locals.var_gf2_dn9 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn9 + locals.var_dm_dn9))))),)
    } else {
        (locals.var_xi_pd, locals.var_xi_pd_dn4, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8, locals.var_xi_pd_dn9,)
    }
};
        locals.var_xi_pd = assign45200_e58504;
        locals.var_xi_pd_dn4 = assign45200_e58504_d_n4;
        locals.var_xi_pd_dn6 = assign45200_e58504_d_n6;
        locals.var_xi_pd_dn7 = assign45200_e58504_d_n7;
        locals.var_xi_pd_dn8 = assign45200_e58504_d_n8;
        locals.var_xi_pd_dn9 = assign45200_e58504_d_n9;
        locals.var_xi_pd_rv = 0.0;

        let (assign45210_e58523, assign45210_e58523_d_n4, assign45210_e58523_d_n6, assign45210_e58523_d_n7, assign45210_e58523_d_n8, assign45210_e58523_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45210_e58513: f64 = (locals.var_q_pd * locals.var_p_pd);
        let assign45210_e58516: f64 = (locals.var_p_pd * locals.var_p_pd);
        let assign45210_e58519: f64 = (locals.var_xi_pd * locals.var_q_pd);
        let assign45210_e58520: f64 = (assign45210_e58516 - assign45210_e58519);
        let assign45210_e58521: f64 = (assign45210_e58513 / assign45210_e58520);
        (assign45210_e58521, (((((locals.var_q_pd_dn4 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn4)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn4 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn4)) - ((locals.var_xi_pd_dn4 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn4))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn9 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn9)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn9 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn9)) - ((locals.var_xi_pd_dn9 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn9))))) / (assign45210_e58520 * assign45210_e58520)),)
    } else {
        (locals.var_u_pd, locals.var_u_pd_dn4, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8, locals.var_u_pd_dn9,)
    }
};
        locals.var_u_pd = assign45210_e58523;
        locals.var_u_pd_dn4 = assign45210_e58523_d_n4;
        locals.var_u_pd_dn6 = assign45210_e58523_d_n6;
        locals.var_u_pd_dn7 = assign45210_e58523_d_n7;
        locals.var_u_pd_dn8 = assign45210_e58523_d_n8;
        locals.var_u_pd_dn9 = assign45210_e58523_d_n9;
        locals.var_u_pd_rv = 0.0;

        let (assign45220_e58534, assign45220_e58534_d_n4, assign45220_e58534_d_n6, assign45220_e58534_d_n7, assign45220_e58534_d_n8, assign45220_e58534_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45220_e58532: f64 = (locals.var_x_m + locals.var_u_pd);
        (assign45220_e58532, (locals.var_x_m_dn4 + locals.var_u_pd_dn4), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8), (locals.var_x_m_dn9 + locals.var_u_pd_dn9),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign45220_e58534;
        locals.var_x_m_dn4 = assign45220_e58534_d_n4;
        locals.var_x_m_dn6 = assign45220_e58534_d_n6;
        locals.var_x_m_dn7 = assign45220_e58534_d_n7;
        locals.var_x_m_dn8 = assign45220_e58534_d_n8;
        locals.var_x_m_dn9 = assign45220_e58534_d_n9;
        locals.var_x_m_rv = 0.0;

        let (assign45230_e58544, assign45230_e58544_d_n4, assign45230_e58544_d_n6, assign45230_e58544_d_n7, assign45230_e58544_d_n8, assign45230_e58544_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45230_e58542: f64 = (locals.var_u_pd).exp();
        (assign45230_e58542, (assign45230_e58542 * locals.var_u_pd_dn4), (assign45230_e58542 * locals.var_u_pd_dn6), (assign45230_e58542 * locals.var_u_pd_dn7), (assign45230_e58542 * locals.var_u_pd_dn8), (assign45230_e58542 * locals.var_u_pd_dn9),)
    } else {
        (locals.var_km, locals.var_km_dn4, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8, locals.var_km_dn9,)
    }
};
        locals.var_km = assign45230_e58544;
        locals.var_km_dn4 = assign45230_e58544_d_n4;
        locals.var_km_dn6 = assign45230_e58544_d_n6;
        locals.var_km_dn7 = assign45230_e58544_d_n7;
        locals.var_km_dn8 = assign45230_e58544_d_n8;
        locals.var_km_dn9 = assign45230_e58544_d_n9;
        locals.var_km_rv = 0.0;

        let (assign45240_e58555, assign45240_e58555_d_n4, assign45240_e58555_d_n6, assign45240_e58555_d_n7, assign45240_e58555_d_n8, assign45240_e58555_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45240_e58553: f64 = (locals.var_em / locals.var_km);
        (assign45240_e58553, (((locals.var_em_dn4 * locals.var_km) - (locals.var_em * locals.var_km_dn4)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn9 * locals.var_km) - (locals.var_em * locals.var_km_dn9)) / (locals.var_km * locals.var_km)),)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign45240_e58555;
        locals.var_em_dn4 = assign45240_e58555_d_n4;
        locals.var_em_dn6 = assign45240_e58555_d_n6;
        locals.var_em_dn7 = assign45240_e58555_d_n7;
        locals.var_em_dn8 = assign45240_e58555_d_n8;
        locals.var_em_dn9 = assign45240_e58555_d_n9;
        locals.var_em_rv = 0.0;

        let (assign45250_e58566, assign45250_e58566_d_n4, assign45250_e58566_d_n6, assign45250_e58566_d_n7, assign45250_e58566_d_n8, assign45250_e58566_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45250_e58564: f64 = (locals.var_dm * locals.var_km);
        (assign45250_e58564, ((locals.var_dm_dn4 * locals.var_km) + (locals.var_dm * locals.var_km_dn4)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)), ((locals.var_dm_dn9 * locals.var_km) + (locals.var_dm * locals.var_km_dn9)),)
    } else {
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9,)
    }
};
        locals.var_dm = assign45250_e58566;
        locals.var_dm_dn4 = assign45250_e58566_d_n4;
        locals.var_dm_dn6 = assign45250_e58566_d_n6;
        locals.var_dm_dn7 = assign45250_e58566_d_n7;
        locals.var_dm_dn8 = assign45250_e58566_d_n8;
        locals.var_dm_dn9 = assign45250_e58566_d_n9;
        locals.var_dm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign45260_e58579, assign45260_e58579_d_n4, assign45260_e58579_d_n6, assign45260_e58579_d_n7, assign45260_e58579_d_n8, assign45260_e58579_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45260_e58575: f64 = (locals.var_x_m - 1.0);
        let assign45260_e58577: f64 = (assign45260_e58575 + locals.var_em);
        (assign45260_e58577, (locals.var_x_m_dn4 + locals.var_em_dn4), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), (locals.var_x_m_dn9 + locals.var_em_dn9),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45260_e58579;
        locals.var_pm_dn4 = assign45260_e58579_d_n4;
        locals.var_pm_dn6 = assign45260_e58579_d_n6;
        locals.var_pm_dn7 = assign45260_e58579_d_n7;
        locals.var_pm_dn8 = assign45260_e58579_d_n8;
        locals.var_pm_dn9 = assign45260_e58579_d_n9;
        locals.var_pm_rv = 0.0;

        let (assign45270_e58593, assign45270_e58593_d_n4, assign45270_e58593_d_n6, assign45270_e58593_d_n7, assign45270_e58593_d_n8, assign45270_e58593_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45270_e58589: f64 = (locals.var_dm + locals.var_pm);
        let assign45270_e58590: f64 = (assign45270_e58589).sqrt();
        let assign45270_e58591: f64 = (locals.var_gf * assign45270_e58590);
        (assign45270_e58591, ((locals.var_gf_dn4 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn6 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn7 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn8 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn9 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45270_e58590)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45270_e58593;
        locals.var_xgm_dn4 = assign45270_e58593_d_n4;
        locals.var_xgm_dn6 = assign45270_e58593_d_n6;
        locals.var_xgm_dn7 = assign45270_e58593_d_n7;
        locals.var_xgm_dn8 = assign45270_e58593_d_n8;
        locals.var_xgm_dn9 = assign45270_e58593_d_n9;
        locals.var_xgm_rv = 0.0;

        let (assign45280_e58612, assign45280_e58612_d_n4, assign45280_e58612_d_n6, assign45280_e58612_d_n7, assign45280_e58612_d_n8, assign45280_e58612_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45280_e58602: f64 = (1.0 - locals.var_em);
        let assign45280_e58606: f64 = (locals.var_xgm * locals.var_eta_p);
        let assign45280_e58608: f64 = (assign45280_e58606 * locals.var_inv_gf2);
        let assign45280_e58609: f64 = (2.0 * assign45280_e58608);
        let assign45280_e58610: f64 = (assign45280_e58602 + assign45280_e58609);
        (assign45280_e58610, ((-locals.var_em_dn4) + (2.0 * ((((locals.var_xgm_dn4 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn4)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn4)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn8)))), ((-locals.var_em_dn9) + (2.0 * ((((locals.var_xgm_dn9 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn9)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn9)))),)
    } else {
        (locals.var_km0, locals.var_km0_dn4, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8, locals.var_km0_dn9,)
    }
};
        locals.var_km0 = assign45280_e58612;
        locals.var_km0_dn4 = assign45280_e58612_d_n4;
        locals.var_km0_dn6 = assign45280_e58612_d_n6;
        locals.var_km0_dn7 = assign45280_e58612_d_n7;
        locals.var_km0_dn8 = assign45280_e58612_d_n8;
        locals.var_km0_dn9 = assign45280_e58612_d_n9;
        locals.var_km0_rv = 0.0;

        let (assign45290_e58633, assign45290_e58633_d_n4, assign45290_e58633_d_n6, assign45290_e58633_d_n7, assign45290_e58633_d_n8, assign45290_e58633_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45290_e58621: f64 = (locals.var_x_ds * locals.var_km);
        let assign45290_e58624: f64 = (locals.var_d0 + locals.var_d_bar);
        let assign45290_e58625: f64 = (assign45290_e58621 * assign45290_e58624);
        let assign45290_e58629: f64 = (locals.var_km * locals.var_d_bar);
        let assign45290_e58630: f64 = (locals.var_km0 + assign45290_e58629);
        let assign45290_e58631: f64 = (assign45290_e58625 / assign45290_e58630);
        (assign45290_e58631, (((((((locals.var_x_ds_dn4 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn4)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn4 + locals.var_d_bar_dn4))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn4 + ((locals.var_km_dn4 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn4))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn9 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn9)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn9 + locals.var_d_bar_dn9))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn9 + ((locals.var_km_dn9 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn9))))) / (assign45290_e58630 * assign45290_e58630)),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign45290_e58633;
        locals.var_x_ds_dn4 = assign45290_e58633_d_n4;
        locals.var_x_ds_dn6 = assign45290_e58633_d_n6;
        locals.var_x_ds_dn7 = assign45290_e58633_d_n7;
        locals.var_x_ds_dn8 = assign45290_e58633_d_n8;
        locals.var_x_ds_dn9 = assign45290_e58633_d_n9;
        locals.var_x_ds_rv = 0.0;

        let (assign45300_e58644, assign45300_e58644_d_n4, assign45300_e58644_d_n6, assign45300_e58644_d_n7, assign45300_e58644_d_n8, assign45300_e58644_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45300_e58642: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign45300_e58642, ((locals.var_x_ds_dn4 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn4)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), ((locals.var_x_ds_dn9 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn9)),)
    } else {
        (locals.var_dps, locals.var_dps_dn4, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9,)
    }
};
        locals.var_dps = assign45300_e58644;
        locals.var_dps_dn4 = assign45300_e58644_d_n4;
        locals.var_dps_dn6 = assign45300_e58644_d_n6;
        locals.var_dps_dn7 = assign45300_e58644_d_n7;
        locals.var_dps_dn8 = assign45300_e58644_d_n8;
        locals.var_dps_dn9 = assign45300_e58644_d_n9;
        locals.var_dps_rv = 0.0;

        let (assign45310_e58652, assign45310_e58652_d_n4, assign45310_e58652_d_n6, assign45310_e58652_d_n7, assign45310_e58652_d_n8, assign45310_e58652_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45310_e58650: f64 = (locals.var_pm).sqrt();
        (assign45310_e58650, (locals.var_pm_dn4 / (2.0 * assign45310_e58650)), (locals.var_pm_dn6 / (2.0 * assign45310_e58650)), (locals.var_pm_dn7 / (2.0 * assign45310_e58650)), (locals.var_pm_dn8 / (2.0 * assign45310_e58650)), (locals.var_pm_dn9 / (2.0 * assign45310_e58650)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn4, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9,)
    }
};
        locals.var_sqm = assign45310_e58652;
        locals.var_sqm_dn4 = assign45310_e58652_d_n4;
        locals.var_sqm_dn6 = assign45310_e58652_d_n6;
        locals.var_sqm_dn7 = assign45310_e58652_d_n7;
        locals.var_sqm_dn8 = assign45310_e58652_d_n8;
        locals.var_sqm_dn9 = assign45310_e58652_d_n9;
        locals.var_sqm_rv = 0.0;

        let (assign45320_e58669, assign45320_e58669_d_n4, assign45320_e58669_d_n6, assign45320_e58669_d_n7, assign45320_e58669_d_n8, assign45320_e58669_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45320_e58662: f64 = (1.0 - locals.var_em);
        let assign45320_e58663: f64 = (locals.var_gf * assign45320_e58662);
        let assign45320_e58665: f64 = (assign45320_e58663 / locals.var_sqm);
        let assign45320_e58666: f64 = (0.5 * assign45320_e58665);
        let assign45320_e58667: f64 = (locals.var_eta_p + assign45320_e58666);
        (assign45320_e58667, (locals.var_eta_p_dn4 + (0.5 * (((((locals.var_gf_dn4 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn4))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn4)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn6 + (0.5 * (((((locals.var_gf_dn6 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn6))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn6)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn7 + (0.5 * (((((locals.var_gf_dn7 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn7))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn7)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn8 + (0.5 * (((((locals.var_gf_dn8 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn8))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn8)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn9 + (0.5 * (((((locals.var_gf_dn9 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn9))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn9)) / (locals.var_sqm * locals.var_sqm)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn4, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9,)
    }
};
        locals.var_alpha = assign45320_e58669;
        locals.var_alpha_dn4 = assign45320_e58669_d_n4;
        locals.var_alpha_dn6 = assign45320_e58669_d_n6;
        locals.var_alpha_dn7 = assign45320_e58669_d_n7;
        locals.var_alpha_dn8 = assign45320_e58669_d_n8;
        locals.var_alpha_dn9 = assign45320_e58669_d_n9;
        locals.var_alpha_rv = 0.0;

        let (assign45330_e58683, assign45330_e58683_d_n4, assign45330_e58683_d_n6, assign45330_e58683_d_n7, assign45330_e58683_d_n8, assign45330_e58683_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45330_e58674: f64 = (locals.var_gf2 * locals.var_dm);
        let assign45330_e58678: f64 = (locals.var_gf * locals.var_sqm);
        let assign45330_e58679: f64 = (locals.var_xgm + assign45330_e58678);
        let assign45330_e58680: f64 = (assign45330_e58674 / assign45330_e58679);
        let assign45330_e58681: f64 = (locals.var_phit1 * assign45330_e58680);
        (assign45330_e58681, ((locals.var_phit1_dn4 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn4 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn4)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn4 + ((locals.var_gf_dn4 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn4))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn6 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn6 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn6)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn6 + ((locals.var_gf_dn6 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn6))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn7 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn7 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn7)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn7 + ((locals.var_gf_dn7 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn7))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn8 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn8 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn8)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn8 + ((locals.var_gf_dn8 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn8))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn9 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn9 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn9)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn9 + ((locals.var_gf_dn9 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn9))))) / (assign45330_e58679 * assign45330_e58679)))),)
    } else {
        (locals.var_qim, locals.var_qim_dn4, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, locals.var_qim_dn9,)
    }
};
        locals.var_qim = assign45330_e58683;
        locals.var_qim_dn4 = assign45330_e58683_d_n4;
        locals.var_qim_dn6 = assign45330_e58683_d_n6;
        locals.var_qim_dn7 = assign45330_e58683_d_n7;
        locals.var_qim_dn8 = assign45330_e58683_d_n8;
        locals.var_qim_dn9 = assign45330_e58683_d_n9;
        locals.var_qim_rv = 0.0;

        let (assign45340_e58691, assign45340_e58691_d_n4, assign45340_e58691_d_n6, assign45340_e58691_d_n7, assign45340_e58691_d_n8, assign45340_e58691_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45340_e58688: f64 = (locals.var_phit1 * locals.var_alpha);
        let assign45340_e58689: f64 = (locals.var_qim + assign45340_e58688);
        (assign45340_e58689, (locals.var_qim_dn4 + ((locals.var_phit1_dn4 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn4))), (locals.var_qim_dn6 + ((locals.var_phit1_dn6 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn6))), (locals.var_qim_dn7 + ((locals.var_phit1_dn7 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn7))), (locals.var_qim_dn8 + ((locals.var_phit1_dn8 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn8))), (locals.var_qim_dn9 + ((locals.var_phit1_dn9 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn9))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn4, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, locals.var_qim1_dn9,)
    }
};
        locals.var_qim1 = assign45340_e58691;
        locals.var_qim1_dn4 = assign45340_e58691_d_n4;
        locals.var_qim1_dn6 = assign45340_e58691_d_n6;
        locals.var_qim1_dn7 = assign45340_e58691_d_n7;
        locals.var_qim1_dn8 = assign45340_e58691_d_n8;
        locals.var_qim1_dn9 = assign45340_e58691_d_n9;
        locals.var_qim1_rv = 0.0;

        let (assign45350_e58699, assign45350_e58699_d_n4, assign45350_e58699_d_n6, assign45350_e58699_d_n7, assign45350_e58699_d_n8, assign45350_e58699_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45350_e58695: f64 = (locals.var_sqm * locals.var_gf);
        let assign45350_e58697: f64 = (assign45350_e58695 * locals.var_phit1);
        (assign45350_e58697, ((((locals.var_sqm_dn4 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn4)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn4)), ((((locals.var_sqm_dn6 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn6)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn6)), ((((locals.var_sqm_dn7 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn7)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn7)), ((((locals.var_sqm_dn8 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn8)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn8)), ((((locals.var_sqm_dn9 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn9)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbm, locals.var_qbm_dn4, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, locals.var_qbm_dn9,)
    }
};
        locals.var_qbm = assign45350_e58699;
        locals.var_qbm_dn4 = assign45350_e58699_d_n4;
        locals.var_qbm_dn6 = assign45350_e58699_d_n6;
        locals.var_qbm_dn7 = assign45350_e58699_d_n7;
        locals.var_qbm_dn8 = assign45350_e58699_d_n8;
        locals.var_qbm_dn9 = assign45350_e58699_d_n9;
        locals.var_qbm_rv = 0.0;

        let assign45360_e58702: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign45360_e58702;
        locals.var_guard1233_rv = 0.0;

        let (assign45370_e58712, assign45370_e58712_d_n4, assign45370_e58712_d_n6, assign45370_e58712_d_n7, assign45370_e58712_d_n8, assign45370_e58712_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1233 != 0.0)) {
        let assign45370_e58709: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45370_e58710: f64 = (1.0 - assign45370_e58709);
        (assign45370_e58710, (-(locals.var_rsg_i * locals.var_qim_dn4)), (-(locals.var_rsg_i * locals.var_qim_dn6)), (-(locals.var_rsg_i * locals.var_qim_dn7)), (-(locals.var_rsg_i * locals.var_qim_dn8)), (-(locals.var_rsg_i * locals.var_qim_dn9)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign45370_e58712;
        locals.var_rhog_dn4 = assign45370_e58712_d_n4;
        locals.var_rhog_dn6 = assign45370_e58712_d_n6;
        locals.var_rhog_dn7 = assign45370_e58712_d_n7;
        locals.var_rhog_dn8 = assign45370_e58712_d_n8;
        locals.var_rhog_dn9 = assign45370_e58712_d_n9;
        locals.var_rhog_rv = 0.0;

        let (assign45380_e58725, assign45380_e58725_d_n4, assign45380_e58725_d_n6, assign45380_e58725_d_n7, assign45380_e58725_d_n8, assign45380_e58725_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1233 == 0.0)) {
        let assign45380_e58721: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45380_e58722: f64 = (1.0 + assign45380_e58721);
        let assign45380_e58723: f64 = (1.0 / assign45380_e58722);
        (assign45380_e58723, (-((locals.var_rsg_i * locals.var_qim_dn4) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn6) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn7) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn8) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn9) / (assign45380_e58722 * assign45380_e58722))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign45380_e58725;
        locals.var_rhog_dn4 = assign45380_e58725_d_n4;
        locals.var_rhog_dn6 = assign45380_e58725_d_n6;
        locals.var_rhog_dn7 = assign45380_e58725_d_n7;
        locals.var_rhog_dn8 = assign45380_e58725_d_n8;
        locals.var_rhog_dn9 = assign45380_e58725_d_n9;
        locals.var_rhog_rv = 0.0;

        let (assign45390_e58735, assign45390_e58735_d_n4, assign45390_e58735_d_n6, assign45390_e58735_d_n7, assign45390_e58735_d_n8, assign45390_e58735_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45390_e58729: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign45390_e58731: f64 = (assign45390_e58729 * locals.var_rhog);
        let assign45390_e58733: f64 = (assign45390_e58731 * locals.var_qim);
        (assign45390_e58733, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn4)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn6)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn7)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn8)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn9)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn9)),)
    } else {
        (locals.var_gr, locals.var_gr_dn4, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, locals.var_gr_dn9,)
    }
};
        locals.var_gr = assign45390_e58735;
        locals.var_gr_dn4 = assign45390_e58735_d_n4;
        locals.var_gr_dn6 = assign45390_e58735_d_n6;
        locals.var_gr_dn7 = assign45390_e58735_d_n7;
        locals.var_gr_dn8 = assign45390_e58735_d_n8;
        locals.var_gr_dn9 = assign45390_e58735_d_n9;
        locals.var_gr_rv = 0.0;

        let (assign45400_e58743, assign45400_e58743_d_n4, assign45400_e58743_d_n6, assign45400_e58743_d_n7, assign45400_e58743_d_n8, assign45400_e58743_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45400_e58740: f64 = (locals.var_eta_mu * locals.var_qim);
        let assign45400_e58741: f64 = (locals.var_qbm + assign45400_e58740);
        (assign45400_e58741, (locals.var_qbm_dn4 + (locals.var_eta_mu * locals.var_qim_dn4)), (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8)), (locals.var_qbm_dn9 + (locals.var_eta_mu * locals.var_qim_dn9)),)
    } else {
        (locals.var_qeff, locals.var_qeff_dn4, locals.var_qeff_dn6, locals.var_qeff_dn7, locals.var_qeff_dn8, locals.var_qeff_dn9,)
    }
};
        locals.var_qeff = assign45400_e58743;
        locals.var_qeff_dn4 = assign45400_e58743_d_n4;
        locals.var_qeff_dn6 = assign45400_e58743_d_n6;
        locals.var_qeff_dn7 = assign45400_e58743_d_n7;
        locals.var_qeff_dn8 = assign45400_e58743_d_n8;
        locals.var_qeff_dn9 = assign45400_e58743_d_n9;
        locals.var_qeff_rv = 0.0;

        let (assign45410_e58751, assign45410_e58751_d_n4, assign45410_e58751_d_n6, assign45410_e58751_d_n7, assign45410_e58751_d_n8, assign45410_e58751_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45410_e58748: f64 = (locals.var_eta_mu1 * locals.var_qim);
        let assign45410_e58749: f64 = (locals.var_qbm + assign45410_e58748);
        (assign45410_e58749, (locals.var_qbm_dn4 + (locals.var_eta_mu1 * locals.var_qim_dn4)), (locals.var_qbm_dn6 + (locals.var_eta_mu1 * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu1 * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu1 * locals.var_qim_dn8)), (locals.var_qbm_dn9 + (locals.var_eta_mu1 * locals.var_qim_dn9)),)
    } else {
        (locals.var_qeff1, locals.var_qeff1_dn4, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, locals.var_qeff1_dn9,)
    }
};
        locals.var_qeff1 = assign45410_e58751;
        locals.var_qeff1_dn4 = assign45410_e58751_d_n4;
        locals.var_qeff1_dn6 = assign45410_e58751_d_n6;
        locals.var_qeff1_dn7 = assign45410_e58751_d_n7;
        locals.var_qeff1_dn8 = assign45410_e58751_d_n8;
        locals.var_qeff1_dn9 = assign45410_e58751_d_n9;
        locals.var_qeff1_rv = 0.0;

        let (assign45420_e58757, assign45420_e58757_d_n4, assign45420_e58757_d_n6, assign45420_e58757_d_n7, assign45420_e58757_d_n8, assign45420_e58757_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45420_e58755: f64 = (locals.var_e_eff0 * locals.var_qeff);
        (assign45420_e58755, (locals.var_e_eff0 * locals.var_qeff_dn4), (locals.var_e_eff0 * locals.var_qeff_dn6), (locals.var_e_eff0 * locals.var_qeff_dn7), (locals.var_e_eff0 * locals.var_qeff_dn8), (locals.var_e_eff0 * locals.var_qeff_dn9),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn4, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, locals.var_eeffm_dn9,)
    }
};
        locals.var_eeffm = assign45420_e58757;
        locals.var_eeffm_dn4 = assign45420_e58757_d_n4;
        locals.var_eeffm_dn6 = assign45420_e58757_d_n6;
        locals.var_eeffm_dn7 = assign45420_e58757_d_n7;
        locals.var_eeffm_dn8 = assign45420_e58757_d_n8;
        locals.var_eeffm_dn9 = assign45420_e58757_d_n9;
        locals.var_eeffm_rv = 0.0;

        let (assign45430_e58768, assign45430_e58768_d_n4, assign45430_e58768_d_n6, assign45430_e58768_d_n7, assign45430_e58768_d_n8, assign45430_e58768_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45430_e58762: f64 = (locals.var_pm + locals.var_dm);
        let assign45430_e58764: f64 = (assign45430_e58762 + 1e-14);
        let assign45430_e58765: f64 = (locals.var_pm / assign45430_e58764);
        let assign45430_e58766: f64 = (assign45430_e58765).ln();
        (assign45430_e58766, ((((locals.var_pm_dn4 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn4 + locals.var_dm_dn4))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn6 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn6 + locals.var_dm_dn6))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn7 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn7 + locals.var_dm_dn7))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn8 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn8 + locals.var_dm_dn8))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn9 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn9 + locals.var_dm_dn9))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45430_e58768;
        locals.var_temp1_dn4 = assign45430_e58768_d_n4;
        locals.var_temp1_dn6 = assign45430_e58768_d_n6;
        locals.var_temp1_dn7 = assign45430_e58768_d_n7;
        locals.var_temp1_dn8 = assign45430_e58768_d_n8;
        locals.var_temp1_dn9 = assign45430_e58768_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign45440_e58785, assign45440_e58785_d_n4, assign45440_e58785_d_n6, assign45440_e58785_d_n7, assign45440_e58785_d_n8, assign45440_e58785_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45440_e58772: f64 = (locals.var_eeffm * locals.var_mue_t);
        let assign45440_e58774: f64 = (assign45440_e58772).powf(locals.var_themu_t);
        let assign45440_e58778: f64 = (0.5 * locals.var_thecs_t);
        let assign45440_e58780: f64 = (assign45440_e58778 * locals.var_temp1);
        let assign45440_e58781: f64 = (assign45440_e58780).exp();
        let assign45440_e58782: f64 = (locals.var_cs_t * assign45440_e58781);
        let assign45440_e58783: f64 = (assign45440_e58774 + assign45440_e58782);
        (assign45440_e58783, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffm_dn4 * locals.var_mue_t) + (locals.var_eeffm * locals.var_mue_t_dn4)))) } } else { (assign45440_e58774 * ((locals.var_themu_t_dn4 * (assign45440_e58772).ln()) + (locals.var_themu_t * (((locals.var_eeffm_dn4 * locals.var_mue_t) + (locals.var_eeffm * locals.var_mue_t_dn4)) / assign45440_e58772)))) } + ((locals.var_cs_t_dn4 * assign45440_e58781) + (locals.var_cs_t * (assign45440_e58781 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign45440_e58778 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn6 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn6 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn7 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn7 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn8 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn8 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn9 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn9 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn4, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, locals.var_mutmp_dn9,)
    }
};
        locals.var_mutmp = assign45440_e58785;
        locals.var_mutmp_dn4 = assign45440_e58785_d_n4;
        locals.var_mutmp_dn6 = assign45440_e58785_d_n6;
        locals.var_mutmp_dn7 = assign45440_e58785_d_n7;
        locals.var_mutmp_dn8 = assign45440_e58785_d_n8;
        locals.var_mutmp_dn9 = assign45440_e58785_d_n9;
        locals.var_mutmp_rv = 0.0;

        let (assign45450_e58795, assign45450_e58795_d_n4, assign45450_e58795_d_n6, assign45450_e58795_d_n7, assign45450_e58795_d_n8, assign45450_e58795_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45450_e58789: f64 = (1.0 + locals.var_mutmp);
        let assign45450_e58791: f64 = (assign45450_e58789 + locals.var_gr);
        let assign45450_e58793: f64 = (assign45450_e58791 * locals.var_rxcor);
        (assign45450_e58793, (((locals.var_mutmp_dn4 + locals.var_gr_dn4) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn4)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn8)), (((locals.var_mutmp_dn9 + locals.var_gr_dn9) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn9)),)
    } else {
        (locals.var_gmob, locals.var_gmob_dn4, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8, locals.var_gmob_dn9,)
    }
};
        locals.var_gmob = assign45450_e58795;
        locals.var_gmob_dn4 = assign45450_e58795_d_n4;
        locals.var_gmob_dn6 = assign45450_e58795_d_n6;
        locals.var_gmob_dn7 = assign45450_e58795_d_n7;
        locals.var_gmob_dn8 = assign45450_e58795_d_n8;
        locals.var_gmob_dn9 = assign45450_e58795_d_n9;
        locals.var_gmob_rv = 0.0;

        let (assign45460_e58814, assign45460_e58814_d_n4, assign45460_e58814_d_n6, assign45460_e58814_d_n7, assign45460_e58814_d_n8, assign45460_e58814_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45460_e58800: f64 = (locals.var_v_ds - locals.var_dps);
        let assign45460_e58802: f64 = (assign45460_e58800 * locals.var_inv_vp);
        let assign45460_e58803: f64 = (1.0 + assign45460_e58802);
        let assign45460_e58807: f64 = (locals.var_vdse - locals.var_dps);
        let assign45460_e58809: f64 = (assign45460_e58807 * locals.var_inv_vp);
        let assign45460_e58810: f64 = (1.0 + assign45460_e58809);
        let assign45460_e58811: f64 = (assign45460_e58803 / assign45460_e58810);
        let assign45460_e58812: f64 = (assign45460_e58811).ln();
        (assign45460_e58812, ((((((-locals.var_dps_dn4) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn4 - locals.var_dps_dn4) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((-locals.var_dps_dn6) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((locals.var_v_ds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((locals.var_v_ds_dn8 - locals.var_dps_dn8) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((-locals.var_dps_dn9) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn9 - locals.var_dps_dn9) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign45460_e58814;
        locals.var_s1_dn4 = assign45460_e58814_d_n4;
        locals.var_s1_dn6 = assign45460_e58814_d_n6;
        locals.var_s1_dn7 = assign45460_e58814_d_n7;
        locals.var_s1_dn8 = assign45460_e58814_d_n8;
        locals.var_s1_dn9 = assign45460_e58814_d_n9;
        locals.var_s1_rv = 0.0;

        let (assign45470_e58820, assign45470_e58820_d_n4, assign45470_e58820_d_n6, assign45470_e58820_d_n7, assign45470_e58820_d_n8, assign45470_e58820_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45470_e58818: f64 = (locals.var_qim * locals.var_xitsb);
        (assign45470_e58818, ((locals.var_qim_dn4 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn4)), ((locals.var_qim_dn6 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn6)), ((locals.var_qim_dn7 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn7)), ((locals.var_qim_dn8 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn8)), ((locals.var_qim_dn9 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign45470_e58820;
        locals.var_temp2_dn4 = assign45470_e58820_d_n4;
        locals.var_temp2_dn6 = assign45470_e58820_d_n6;
        locals.var_temp2_dn7 = assign45470_e58820_d_n7;
        locals.var_temp2_dn8 = assign45470_e58820_d_n8;
        locals.var_temp2_dn9 = assign45470_e58820_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign45480_e58828, assign45480_e58828_d_n4, assign45480_e58828_d_n6, assign45480_e58828_d_n7, assign45480_e58828_d_n8, assign45480_e58828_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45480_e58825: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign45480_e58826: f64 = (locals.var_temp2 / assign45480_e58825);
        (assign45480_e58826, (((locals.var_temp2_dn4 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn6 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn7 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn8 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn9 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign45480_e58825 * assign45480_e58825)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn4, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, locals.var_wsat_dn9,)
    }
};
        locals.var_wsat = assign45480_e58828;
        locals.var_wsat_dn4 = assign45480_e58828_d_n4;
        locals.var_wsat_dn6 = assign45480_e58828_d_n6;
        locals.var_wsat_dn7 = assign45480_e58828_d_n7;
        locals.var_wsat_dn8 = assign45480_e58828_d_n8;
        locals.var_wsat_dn9 = assign45480_e58828_d_n9;
        locals.var_wsat_rv = 0.0;

        let assign45490_e58831: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign45490_e58831;
        locals.var_guard1234_rv = 0.0;

        let (assign45500_e58843, assign45500_e58843_d_n4, assign45500_e58843_d_n6, assign45500_e58843_d_n7, assign45500_e58843_d_n8, assign45500_e58843_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1234 != 0.0)) {
        let assign45500_e58839: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45500_e58840: f64 = (1.0 - assign45500_e58839);
        let assign45500_e58841: f64 = (1.0 / assign45500_e58840);
        (assign45500_e58841, (-((-(locals.var_thesatg_i * locals.var_wsat_dn4)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn9)) / (assign45500_e58840 * assign45500_e58840))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign45500_e58843;
        locals.var_factheta_dn4 = assign45500_e58843_d_n4;
        locals.var_factheta_dn6 = assign45500_e58843_d_n6;
        locals.var_factheta_dn7 = assign45500_e58843_d_n7;
        locals.var_factheta_dn8 = assign45500_e58843_d_n8;
        locals.var_factheta_dn9 = assign45500_e58843_d_n9;
        locals.var_factheta_rv = 0.0;

        let (assign45510_e58854, assign45510_e58854_d_n4, assign45510_e58854_d_n6, assign45510_e58854_d_n7, assign45510_e58854_d_n8, assign45510_e58854_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1234 == 0.0)) {
        let assign45510_e58851: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45510_e58852: f64 = (1.0 + assign45510_e58851);
        (assign45510_e58852, (locals.var_thesatg_i * locals.var_wsat_dn4), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), (locals.var_thesatg_i * locals.var_wsat_dn9),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign45510_e58854;
        locals.var_factheta_dn4 = assign45510_e58854_d_n4;
        locals.var_factheta_dn6 = assign45510_e58854_d_n6;
        locals.var_factheta_dn7 = assign45510_e58854_d_n7;
        locals.var_factheta_dn8 = assign45510_e58854_d_n8;
        locals.var_factheta_dn9 = assign45510_e58854_d_n9;
        locals.var_factheta_rv = 0.0;

        let (assign45520_e58860, assign45520_e58860_d_n4, assign45520_e58860_d_n6, assign45520_e58860_d_n7, assign45520_e58860_d_n8, assign45520_e58860_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45520_e58858: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign45520_e58858, ((locals.var_thesatloc_dn4 * locals.var_factheta) + (locals.var_thesatloc * locals.var_factheta_dn4)), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), (locals.var_thesatloc * locals.var_factheta_dn9),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn4, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, locals.var_thesateff_dn9,)
    }
};
        locals.var_thesateff = assign45520_e58860;
        locals.var_thesateff_dn4 = assign45520_e58860_d_n4;
        locals.var_thesateff_dn6 = assign45520_e58860_d_n6;
        locals.var_thesateff_dn7 = assign45520_e58860_d_n7;
        locals.var_thesateff_dn8 = assign45520_e58860_d_n8;
        locals.var_thesateff_dn9 = assign45520_e58860_d_n9;
        locals.var_thesateff_rv = 0.0;

        let (assign45530_e58866, assign45530_e58866_d_n4, assign45530_e58866_d_n6, assign45530_e58866_d_n7, assign45530_e58866_d_n8, assign45530_e58866_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45530_e58864: f64 = (locals.var_xgm * locals.var_phit1);
        (assign45530_e58864, ((locals.var_xgm_dn4 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn4)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)), ((locals.var_xgm_dn9 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn9)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn4, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9,)
    }
};
        locals.var_voxm = assign45530_e58866;
        locals.var_voxm_dn4 = assign45530_e58866_d_n4;
        locals.var_voxm_dn6 = assign45530_e58866_d_n6;
        locals.var_voxm_dn7 = assign45530_e58866_d_n7;
        locals.var_voxm_dn8 = assign45530_e58866_d_n8;
        locals.var_voxm_dn9 = assign45530_e58866_d_n9;
        locals.var_voxm_rv = 0.0;

        locals.var_vdsat_lim_dc = locals.var_vdsat_lim;
        locals.var_vdsat_lim_dc_dn4 = locals.var_vdsat_lim_dn4;
        locals.var_vdsat_lim_dc_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_vdsat_lim_dc_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_vdsat_lim_dc_dn8 = locals.var_vdsat_lim_dn8;
        locals.var_vdsat_lim_dc_dn9 = locals.var_vdsat_lim_dn9;
        locals.var_vdsat_lim_dc_rv = 0.0;

        locals.var_vdse_dc = locals.var_vdse;
        locals.var_vdse_dc_dn4 = locals.var_vdse_dn4;
        locals.var_vdse_dc_dn6 = locals.var_vdse_dn6;
        locals.var_vdse_dc_dn7 = locals.var_vdse_dn7;
        locals.var_vdse_dc_dn8 = locals.var_vdse_dn8;
        locals.var_vdse_dc_dn9 = locals.var_vdse_dn9;
        locals.var_vdse_dc_rv = 0.0;

        locals.var_udse_dc = locals.var_udse;
        locals.var_udse_dc_dn4 = locals.var_udse_dn4;
        locals.var_udse_dc_dn6 = locals.var_udse_dn6;
        locals.var_udse_dc_dn7 = locals.var_udse_dn7;
        locals.var_udse_dc_dn8 = locals.var_udse_dn8;
        locals.var_udse_dc_dn9 = locals.var_udse_dn9;
        locals.var_udse_dc_rv = 0.0;

        locals.var_x_ds_dc = locals.var_x_ds;
        locals.var_x_ds_dc_dn4 = locals.var_x_ds_dn4;
        locals.var_x_ds_dc_dn6 = locals.var_x_ds_dn6;
        locals.var_x_ds_dc_dn7 = locals.var_x_ds_dn7;
        locals.var_x_ds_dc_dn8 = locals.var_x_ds_dn8;
        locals.var_x_ds_dc_dn9 = locals.var_x_ds_dn9;
        locals.var_x_ds_dc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_dps_dc = locals.var_dps;
        locals.var_dps_dc_dn4 = locals.var_dps_dn4;
        locals.var_dps_dc_dn6 = locals.var_dps_dn6;
        locals.var_dps_dc_dn7 = locals.var_dps_dn7;
        locals.var_dps_dc_dn8 = locals.var_dps_dn8;
        locals.var_dps_dc_dn9 = locals.var_dps_dn9;
        locals.var_dps_dc_rv = 0.0;

        locals.var_x_m_dc = locals.var_x_m;
        locals.var_x_m_dc_dn4 = locals.var_x_m_dn4;
        locals.var_x_m_dc_dn6 = locals.var_x_m_dn6;
        locals.var_x_m_dc_dn7 = locals.var_x_m_dn7;
        locals.var_x_m_dc_dn8 = locals.var_x_m_dn8;
        locals.var_x_m_dc_dn9 = locals.var_x_m_dn9;
        locals.var_x_m_dc_rv = 0.0;

        locals.var_qbd_dc = locals.var_qbd;
        locals.var_qbd_dc_dn4 = locals.var_qbd_dn4;
        locals.var_qbd_dc_dn6 = locals.var_qbd_dn6;
        locals.var_qbd_dc_dn7 = locals.var_qbd_dn7;
        locals.var_qbd_dc_dn8 = locals.var_qbd_dn8;
        locals.var_qbd_dc_dn9 = locals.var_qbd_dn9;
        locals.var_qbd_dc_rv = 0.0;

        locals.var_eta_p_dc = locals.var_eta_p;
        locals.var_eta_p_dc_dn4 = locals.var_eta_p_dn4;
        locals.var_eta_p_dc_dn6 = locals.var_eta_p_dn6;
        locals.var_eta_p_dc_dn7 = locals.var_eta_p_dn7;
        locals.var_eta_p_dc_dn8 = locals.var_eta_p_dn8;
        locals.var_eta_p_dc_dn9 = locals.var_eta_p_dn9;
        locals.var_eta_p_dc_rv = 0.0;

        locals.var_alpha_dc = locals.var_alpha;
        locals.var_alpha_dc_dn4 = locals.var_alpha_dn4;
        locals.var_alpha_dc_dn6 = locals.var_alpha_dn6;
        locals.var_alpha_dc_dn7 = locals.var_alpha_dn7;
        locals.var_alpha_dc_dn8 = locals.var_alpha_dn8;
        locals.var_alpha_dc_dn9 = locals.var_alpha_dn9;
        locals.var_alpha_dc_rv = 0.0;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn4 = locals.var_qim_dn4;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;
        locals.var_qim_dc_dn9 = locals.var_qim_dn9;
        locals.var_qim_dc_rv = 0.0;

        locals.var_qim1_dc = locals.var_qim1;
        locals.var_qim1_dc_dn4 = locals.var_qim1_dn4;
        locals.var_qim1_dc_dn6 = locals.var_qim1_dn6;
        locals.var_qim1_dc_dn7 = locals.var_qim1_dn7;
        locals.var_qim1_dc_dn8 = locals.var_qim1_dn8;
        locals.var_qim1_dc_dn9 = locals.var_qim1_dn9;
        locals.var_qim1_dc_rv = 0.0;

        locals.var_qbm_dc = locals.var_qbm;
        locals.var_qbm_dc_dn4 = locals.var_qbm_dn4;
        locals.var_qbm_dc_dn6 = locals.var_qbm_dn6;
        locals.var_qbm_dc_dn7 = locals.var_qbm_dn7;
        locals.var_qbm_dc_dn8 = locals.var_qbm_dn8;
        locals.var_qbm_dc_dn9 = locals.var_qbm_dn9;
        locals.var_qbm_dc_rv = 0.0;

        locals.var_qeff1_dc = locals.var_qeff1;
        locals.var_qeff1_dc_dn4 = locals.var_qeff1_dn4;
        locals.var_qeff1_dc_dn6 = locals.var_qeff1_dn6;
        locals.var_qeff1_dc_dn7 = locals.var_qeff1_dn7;
        locals.var_qeff1_dc_dn8 = locals.var_qeff1_dn8;
        locals.var_qeff1_dc_dn9 = locals.var_qeff1_dn9;
        locals.var_qeff1_dc_rv = 0.0;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn4 = locals.var_gmob_dn4;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;
        locals.var_gmob_dc_dn9 = locals.var_gmob_dn9;
        locals.var_gmob_dc_rv = 0.0;

        locals.var_s1_dc = locals.var_s1;
        locals.var_s1_dc_dn4 = locals.var_s1_dn4;
        locals.var_s1_dc_dn6 = locals.var_s1_dn6;
        locals.var_s1_dc_dn7 = locals.var_s1_dn7;
        locals.var_s1_dc_dn8 = locals.var_s1_dn8;
        locals.var_s1_dc_dn9 = locals.var_s1_dn9;
        locals.var_s1_dc_rv = 0.0;

        locals.var_thesateff_dc = locals.var_thesateff;
        locals.var_thesateff_dc_dn4 = locals.var_thesateff_dn4;
        locals.var_thesateff_dc_dn6 = locals.var_thesateff_dn6;
        locals.var_thesateff_dc_dn7 = locals.var_thesateff_dn7;
        locals.var_thesateff_dc_dn8 = locals.var_thesateff_dn8;
        locals.var_thesateff_dc_dn9 = locals.var_thesateff_dn9;
        locals.var_thesateff_dc_rv = 0.0;

        locals.var_voxm_dc = locals.var_voxm;
        locals.var_voxm_dc_dn4 = locals.var_voxm_dn4;
        locals.var_voxm_dc_dn6 = locals.var_voxm_dn6;
        locals.var_voxm_dc_dn7 = locals.var_voxm_dn7;
        locals.var_voxm_dc_dn8 = locals.var_voxm_dn8;
        locals.var_voxm_dc_dn9 = locals.var_voxm_dn9;
        locals.var_voxm_dc_rv = 0.0;

        locals.var_gdl_dc = 1.0;
        locals.var_gdl_dc_dn4 = 0.0;
        locals.var_gdl_dc_dn6 = 0.0;
        locals.var_gdl_dc_dn7 = 0.0;
        locals.var_gdl_dc_dn8 = 0.0;
        locals.var_gdl_dc_dn9 = 0.0;
        locals.var_gdl_dc_rv = 0.0;

        locals.var_gmob_dl_dc = 1.0;
        locals.var_gmob_dl_dc_dn4 = 0.0;
        locals.var_gmob_dl_dc_dn6 = 0.0;
        locals.var_gmob_dl_dc_dn7 = 0.0;
        locals.var_gmob_dl_dc_dn8 = 0.0;
        locals.var_gmob_dl_dc_dn9 = 0.0;
        locals.var_gmob_dl_dc_rv = 0.0;

        locals.var_gvsatinv_dc = 1.0;
        locals.var_gvsatinv_dc_dn4 = 0.0;
        locals.var_gvsatinv_dc_dn6 = 0.0;
        locals.var_gvsatinv_dc_dn7 = 0.0;
        locals.var_gvsatinv_dc_dn8 = 0.0;
        locals.var_gvsatinv_dc_dn9 = 0.0;
        locals.var_gvsatinv_dc_rv = 0.0;

        locals.var_h_dc = 1.0;
        locals.var_h_dc_dn4 = 0.0;
        locals.var_h_dc_dn6 = 0.0;
        locals.var_h_dc_dn7 = 0.0;
        locals.var_h_dc_dn8 = 0.0;
        locals.var_h_dc_dn9 = 0.0;
        locals.var_h_dc_rv = 0.0;

        locals.var_i_ds = 0.0;
        locals.var_i_ds_dn4 = 0.0;
        locals.var_i_ds_dn6 = 0.0;
        locals.var_i_ds_dn7 = 0.0;
        locals.var_i_ds_dn8 = 0.0;
        locals.var_i_ds_dn9 = 0.0;
        locals.var_i_ds_rv = 0.0;

        let assign45870_e58940: f64 = if locals.var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign45870_e58940;
        locals.var_guard1235_rv = 0.0;

        let (assign45880_e58949, assign45880_e58949_d_n7, assign45880_e58949_d_n8,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45880_e58945: f64 = (locals.var_vdsx * locals.var_inv_vp);
        let assign45880_e58946: f64 = (1.0 + assign45880_e58945);
        let assign45880_e58947: f64 = (assign45880_e58946).ln();
        (assign45880_e58947, ((locals.var_vdsx_dn7 * locals.var_inv_vp) / assign45880_e58946), ((locals.var_vdsx_dn8 * locals.var_inv_vp) / assign45880_e58946),)
    } else {
        (locals.var_s2, locals.var_s2_dn7, locals.var_s2_dn8,)
    }
};
        locals.var_s2 = assign45880_e58949;
        locals.var_s2_dn7 = assign45880_e58949_d_n7;
        locals.var_s2_dn8 = assign45880_e58949_d_n8;
        locals.var_s2_rv = 0.0;

        let (assign45890_e58957, assign45890_e58957_d_n4, assign45890_e58957_d_n6, assign45890_e58957_d_n7, assign45890_e58957_d_n8, assign45890_e58957_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45890_e58953: f64 = (locals.var_phit1_dc * locals.var_alpha_dc);
        let assign45890_e58955: f64 = (assign45890_e58953 / locals.var_qim1_dc);
        (assign45890_e58955, (((((locals.var_phit1_dc_dn4 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn4)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn4)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn6 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn6)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn7 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn7)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn8 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn8)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn9 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn9)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn9)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45890_e58957;
        locals.var_temp__blk949_dn4 = assign45890_e58957_d_n4;
        locals.var_temp__blk949_dn6 = assign45890_e58957_d_n6;
        locals.var_temp__blk949_dn7 = assign45890_e58957_d_n7;
        locals.var_temp__blk949_dn8 = assign45890_e58957_d_n8;
        locals.var_temp__blk949_dn9 = assign45890_e58957_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign45900_e58981, assign45900_e58981_d_n4, assign45900_e58981_d_n6, assign45900_e58981_d_n7, assign45900_e58981_d_n8, assign45900_e58981_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45900_e58962: f64 = (locals.var_alp1_i / locals.var_qim1_dc);
        let assign45900_e58963: f64 = (locals.var_alp_i + assign45900_e58962);
        let assign45900_e58965: f64 = (assign45900_e58963 * locals.var_qim_dc);
        let assign45900_e58967: f64 = (assign45900_e58965 / locals.var_qim1_dc);
        let assign45900_e58969: f64 = (assign45900_e58967 * locals.var_s1_dc);
        let assign45900_e58972: f64 = (locals.var_alp2_i * locals.var_qbm_dc);
        let assign45900_e58974: f64 = (assign45900_e58972 * locals.var_temp__blk949);
        let assign45900_e58976: f64 = (assign45900_e58974 * locals.var_temp__blk949);
        let assign45900_e58978: f64 = (assign45900_e58976 * locals.var_s2);
        let assign45900_e58979: f64 = (assign45900_e58969 + assign45900_e58978);
        (assign45900_e58979, (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn4) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn4)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn4)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn4)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn4) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn4)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn6) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn6)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn6)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn6) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn6)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn7) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn7)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn7)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn7) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn7)) * locals.var_s2) + (assign45900_e58976 * locals.var_s2_dn7))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn8) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn8)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn8)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn8) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn8)) * locals.var_s2) + (assign45900_e58976 * locals.var_s2_dn8))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn9) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn9)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn9)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn9)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn9) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn9)) * locals.var_s2)),)
    } else {
        (locals.var_dl, locals.var_dl_dn4, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8, locals.var_dl_dn9,)
    }
};
        locals.var_dl = assign45900_e58981;
        locals.var_dl_dn4 = assign45900_e58981_d_n4;
        locals.var_dl_dn6 = assign45900_e58981_d_n6;
        locals.var_dl_dn7 = assign45900_e58981_d_n7;
        locals.var_dl_dn8 = assign45900_e58981_d_n8;
        locals.var_dl_dn9 = assign45900_e58981_d_n9;
        locals.var_dl_rv = 0.0;

        let (assign45910_e58993, assign45910_e58993_d_n4, assign45910_e58993_d_n6, assign45910_e58993_d_n7, assign45910_e58993_d_n8, assign45910_e58993_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45910_e58986: f64 = (1.0 + locals.var_dl);
        let assign45910_e58989: f64 = (locals.var_dl * locals.var_dl);
        let assign45910_e58990: f64 = (assign45910_e58986 + assign45910_e58989);
        let assign45910_e58991: f64 = (1.0 / assign45910_e58990);
        (assign45910_e58991, (-((locals.var_dl_dn4 + ((locals.var_dl_dn4 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn4))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn9 + ((locals.var_dl_dn9 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn9))) / (assign45910_e58990 * assign45910_e58990))),)
    } else {
        (locals.var_gdl_dc, locals.var_gdl_dc_dn4, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8, locals.var_gdl_dc_dn9,)
    }
};
        locals.var_gdl_dc = assign45910_e58993;
        locals.var_gdl_dc_dn4 = assign45910_e58993_d_n4;
        locals.var_gdl_dc_dn6 = assign45910_e58993_d_n6;
        locals.var_gdl_dc_dn7 = assign45910_e58993_d_n7;
        locals.var_gdl_dc_dn8 = assign45910_e58993_d_n8;
        locals.var_gdl_dc_dn9 = assign45910_e58993_d_n9;
        locals.var_gdl_dc_rv = 0.0;

        let (assign45920_e58999, assign45920_e58999_d_n4, assign45920_e58999_d_n6, assign45920_e58999_d_n7, assign45920_e58999_d_n8, assign45920_e58999_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45920_e58997: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        (assign45920_e58997, ((locals.var_gmob_dc_dn4 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn4)), ((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)), ((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)), ((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)), ((locals.var_gmob_dc_dn9 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn9)),)
    } else {
        (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn4, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8, locals.var_gmob_dl_dc_dn9,)
    }
};
        locals.var_gmob_dl_dc = assign45920_e58999;
        locals.var_gmob_dl_dc_dn4 = assign45920_e58999_d_n4;
        locals.var_gmob_dl_dc_dn6 = assign45920_e58999_d_n6;
        locals.var_gmob_dl_dc_dn7 = assign45920_e58999_d_n7;
        locals.var_gmob_dl_dc_dn8 = assign45920_e58999_d_n8;
        locals.var_gmob_dl_dc_dn9 = assign45920_e58999_d_n9;
        locals.var_gmob_dl_dc_rv = 0.0;

        let (assign45930_e59005, assign45930_e59005_d_n4, assign45930_e59005_d_n6, assign45930_e59005_d_n7, assign45930_e59005_d_n8, assign45930_e59005_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45930_e59003: f64 = (locals.var_thesateff_dc / locals.var_gmob_dl_dc);
        (assign45930_e59003, (((locals.var_thesateff_dc_dn4 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn4)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn6)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn7)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn8)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn9 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn9)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)),)
    } else {
        (locals.var_thesat1_dc, locals.var_thesat1_dc_dn4, locals.var_thesat1_dc_dn6, locals.var_thesat1_dc_dn7, locals.var_thesat1_dc_dn8, locals.var_thesat1_dc_dn9,)
    }
};
        locals.var_thesat1_dc = assign45930_e59005;
        locals.var_thesat1_dc_dn4 = assign45930_e59005_d_n4;
        locals.var_thesat1_dc_dn6 = assign45930_e59005_d_n6;
        locals.var_thesat1_dc_dn7 = assign45930_e59005_d_n7;
        locals.var_thesat1_dc_dn8 = assign45930_e59005_d_n8;
        locals.var_thesat1_dc_dn9 = assign45930_e59005_d_n9;
        locals.var_thesat1_dc_rv = 0.0;

        let (assign45940_e59015, assign45940_e59015_d_n4, assign45940_e59015_d_n6, assign45940_e59015_d_n7, assign45940_e59015_d_n8, assign45940_e59015_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45940_e59009: f64 = (locals.var_thesat1_dc * locals.var_thesat1_dc);
        let assign45940_e59011: f64 = (assign45940_e59009 * locals.var_dps_dc);
        let assign45940_e59013: f64 = (assign45940_e59011 * locals.var_dps_dc);
        (assign45940_e59013, ((((((locals.var_thesat1_dc_dn4 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn4)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn4)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn4)), ((((((locals.var_thesat1_dc_dn6 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn6)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_dc_dn7 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn7)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_dc_dn8 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn8)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn8)), ((((((locals.var_thesat1_dc_dn9 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn9)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn9)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn9)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn4, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9,)
    }
};
        locals.var_zsat = assign45940_e59015;
        locals.var_zsat_dn4 = assign45940_e59015_d_n4;
        locals.var_zsat_dn6 = assign45940_e59015_d_n6;
        locals.var_zsat_dn7 = assign45940_e59015_d_n7;
        locals.var_zsat_dn8 = assign45940_e59015_d_n8;
        locals.var_zsat_dn9 = assign45940_e59015_d_n9;
        locals.var_zsat_rv = 0.0;

        let assign45950_e59018: f64 = (-1.0);
        let assign45950_e59019: f64 = if locals.var_chnl_type == assign45950_e59018 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign45950_e59019;
        locals.var_guard1236_rv = 0.0;

        let (assign45960_e59031, assign45960_e59031_d_n4, assign45960_e59031_d_n6, assign45960_e59031_d_n7, assign45960_e59031_d_n8, assign45960_e59031_d_n9,) = {
    if ((locals.var_guard1235 != 0.0) && (locals.var_guard1236 != 0.0)) {
        let assign45960_e59027: f64 = (locals.var_thesat1_dc * locals.var_dps_dc);
        let assign45960_e59028: f64 = (1.0 + assign45960_e59027);
        let assign45960_e59029: f64 = (locals.var_zsat / assign45960_e59028);
        (assign45960_e59029, (((locals.var_zsat_dn4 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn4 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn4)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn6 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn6)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn7 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn7)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn8 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn8)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn9 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn9 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn9)))) / (assign45960_e59028 * assign45960_e59028)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn4, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9,)
    }
};
        locals.var_zsat = assign45960_e59031;
        locals.var_zsat_dn4 = assign45960_e59031_d_n4;
        locals.var_zsat_dn6 = assign45960_e59031_d_n6;
        locals.var_zsat_dn7 = assign45960_e59031_d_n7;
        locals.var_zsat_dn8 = assign45960_e59031_d_n8;
        locals.var_zsat_dn9 = assign45960_e59031_d_n9;
        locals.var_zsat_rv = 0.0;

        let (assign45970_e59046, assign45970_e59046_d_n4, assign45970_e59046_d_n6, assign45970_e59046_d_n7, assign45970_e59046_d_n8, assign45970_e59046_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45970_e59039: f64 = (2.0 * locals.var_zsat);
        let assign45970_e59040: f64 = (1.0 + assign45970_e59039);
        let assign45970_e59041: f64 = (assign45970_e59040).sqrt();
        let assign45970_e59042: f64 = (1.0 + assign45970_e59041);
        let assign45970_e59043: f64 = (locals.var_gmob_dl_dc * assign45970_e59042);
        let assign45970_e59044: f64 = (0.5 * assign45970_e59043);
        (assign45970_e59044, (0.5 * ((locals.var_gmob_dl_dc_dn4 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn4) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn6 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn7 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn8 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn9 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn9) / (2.0 * assign45970_e59041))))),)
    } else {
        (locals.var_gvsat, locals.var_gvsat_dn4, locals.var_gvsat_dn6, locals.var_gvsat_dn7, locals.var_gvsat_dn8, locals.var_gvsat_dn9,)
    }
};
        locals.var_gvsat = assign45970_e59046;
        locals.var_gvsat_dn4 = assign45970_e59046_d_n4;
        locals.var_gvsat_dn6 = assign45970_e59046_d_n6;
        locals.var_gvsat_dn7 = assign45970_e59046_d_n7;
        locals.var_gvsat_dn8 = assign45970_e59046_d_n8;
        locals.var_gvsat_dn9 = assign45970_e59046_d_n9;
        locals.var_gvsat_rv = 0.0;

        let (assign45980_e59052, assign45980_e59052_d_n4, assign45980_e59052_d_n6, assign45980_e59052_d_n7, assign45980_e59052_d_n8, assign45980_e59052_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45980_e59050: f64 = (1.0 / locals.var_gvsat);
        (assign45980_e59050, (-(locals.var_gvsat_dn4 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn6 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn7 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn8 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn9 / (locals.var_gvsat * locals.var_gvsat))),)
    } else {
        (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn4, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8, locals.var_gvsatinv_dc_dn9,)
    }
};
        locals.var_gvsatinv_dc = assign45980_e59052;
        locals.var_gvsatinv_dc_dn4 = assign45980_e59052_d_n4;
        locals.var_gvsatinv_dc_dn6 = assign45980_e59052_d_n6;
        locals.var_gvsatinv_dc_dn7 = assign45980_e59052_d_n7;
        locals.var_gvsatinv_dc_dn8 = assign45980_e59052_d_n8;
        locals.var_gvsatinv_dc_dn9 = assign45980_e59052_d_n9;
        locals.var_gvsatinv_dc_rv = 0.0;

        let (assign45990_e59058, assign45990_e59058_d_n4, assign45990_e59058_d_n6, assign45990_e59058_d_n7, assign45990_e59058_d_n8, assign45990_e59058_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45990_e59056: f64 = (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc);
        (assign45990_e59056, ((locals.var_gmob_dl_dc_dn4 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn4)), ((locals.var_gmob_dl_dc_dn6 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn6)), ((locals.var_gmob_dl_dc_dn7 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn7)), ((locals.var_gmob_dl_dc_dn8 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn8)), ((locals.var_gmob_dl_dc_dn9 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45990_e59058;
        locals.var_temp__blk949_dn4 = assign45990_e59058_d_n4;
        locals.var_temp__blk949_dn6 = assign45990_e59058_d_n6;
        locals.var_temp__blk949_dn7 = assign45990_e59058_d_n7;
        locals.var_temp__blk949_dn8 = assign45990_e59058_d_n8;
        locals.var_temp__blk949_dn9 = assign45990_e59058_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46000_e59072, assign46000_e59072_d_n4, assign46000_e59072_d_n6, assign46000_e59072_d_n7, assign46000_e59072_d_n8, assign46000_e59072_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46000_e59065: f64 = (locals.var_zsat * locals.var_temp__blk949);
        let assign46000_e59067: f64 = (assign46000_e59065 * locals.var_temp__blk949);
        let assign46000_e59068: f64 = (0.5 * assign46000_e59067);
        let assign46000_e59069: f64 = (1.0 + assign46000_e59068);
        let assign46000_e59070: f64 = (locals.var_alpha_dc * assign46000_e59069);
        (assign46000_e59070, ((locals.var_alpha_dc_dn4 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn4 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn4))))), ((locals.var_alpha_dc_dn6 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn6 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn6))))), ((locals.var_alpha_dc_dn7 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn7 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn7))))), ((locals.var_alpha_dc_dn8 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn8 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn8))))), ((locals.var_alpha_dc_dn9 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn9 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn9))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn4, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9,)
    }
};
        locals.var_alpha1 = assign46000_e59072;
        locals.var_alpha1_dn4 = assign46000_e59072_d_n4;
        locals.var_alpha1_dn6 = assign46000_e59072_d_n6;
        locals.var_alpha1_dn7 = assign46000_e59072_d_n7;
        locals.var_alpha1_dn8 = assign46000_e59072_d_n8;
        locals.var_alpha1_dn9 = assign46000_e59072_d_n9;
        locals.var_alpha1_rv = 0.0;

        let (assign46010_e59080, assign46010_e59080_d_n4, assign46010_e59080_d_n6, assign46010_e59080_d_n7, assign46010_e59080_d_n8, assign46010_e59080_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46010_e59076: f64 = (locals.var_temp__blk949 * locals.var_qim1_dc);
        let assign46010_e59078: f64 = (assign46010_e59076 / locals.var_alpha1);
        (assign46010_e59078, (((((locals.var_temp__blk949_dn4 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn4)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn4)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn6 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn6)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn7 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn7)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn8 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn8)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn9 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn9)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn9)) / (locals.var_alpha1 * locals.var_alpha1)),)
    } else {
        (locals.var_h_dc, locals.var_h_dc_dn4, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8, locals.var_h_dc_dn9,)
    }
};
        locals.var_h_dc = assign46010_e59080;
        locals.var_h_dc_dn4 = assign46010_e59080_d_n4;
        locals.var_h_dc_dn6 = assign46010_e59080_d_n6;
        locals.var_h_dc_dn7 = assign46010_e59080_d_n7;
        locals.var_h_dc_dn8 = assign46010_e59080_d_n8;
        locals.var_h_dc_dn9 = assign46010_e59080_d_n9;
        locals.var_h_dc_rv = 0.0;

        let (assign46020_e59090, assign46020_e59090_d_n4, assign46020_e59090_d_n6, assign46020_e59090_d_n7, assign46020_e59090_d_n8, assign46020_e59090_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46020_e59084: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign46020_e59086: f64 = (assign46020_e59084 * locals.var_dps_dc);
        let assign46020_e59088: f64 = (assign46020_e59086 * locals.var_gvsatinv_dc);
        (assign46020_e59088, ((((((locals.var_bet_i_dn4 * locals.var_qim1_dc) + (locals.var_bet_i * locals.var_qim1_dc_dn4)) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn4)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn4)), (((((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn6)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn6)), (((((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn7)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn7)), (((((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn8)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn8)), (((((locals.var_bet_i * locals.var_qim1_dc_dn9) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn9)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn9)),)
    } else {
        (locals.var_i_ds, locals.var_i_ds_dn4, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8, locals.var_i_ds_dn9,)
    }
};
        locals.var_i_ds = assign46020_e59090;
        locals.var_i_ds_dn4 = assign46020_e59090_d_n4;
        locals.var_i_ds_dn6 = assign46020_e59090_d_n6;
        locals.var_i_ds_dn7 = assign46020_e59090_d_n7;
        locals.var_i_ds_dn8 = assign46020_e59090_d_n8;
        locals.var_i_ds_dn9 = assign46020_e59090_d_n9;
        locals.var_i_ds_rv = 0.0;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;
        locals.var_xs_ov_dn8 = 0.0;
        locals.var_xs_ov_rv = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;
        locals.var_xd_ov_dn8 = 0.0;
        locals.var_xd_ov_rv = 0.0;

        locals.var_vovs = 0.0;
        locals.var_vovs_dn6 = 0.0;
        locals.var_vovs_dn7 = 0.0;
        locals.var_vovs_dn8 = 0.0;
        locals.var_vovs_rv = 0.0;

        locals.var_vovd = 0.0;
        locals.var_vovd_dn6 = 0.0;
        locals.var_vovd_dn7 = 0.0;
        locals.var_vovd_dn8 = 0.0;
        locals.var_vovd_rv = 0.0;

        let assign46070_e59125: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign46070_e59125;
        locals.var_guard1237_rv = 0.0;

        let (assign46080_e59138, assign46080_e59138_d_n6, assign46080_e59138_d_n7, assign46080_e59138_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46080_e59131: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
        let assign46080_e59133: f64 = (assign46080_e59131 + locals.var_sp_ov_eps2_s);
        let assign46080_e59134: f64 = (assign46080_e59133).sqrt();
        let assign46080_e59135: f64 = (locals.var_xgs_ov + assign46080_e59134);
        let assign46080_e59136: f64 = (0.5 * assign46080_e59135);
        (assign46080_e59136, (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign46080_e59134)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign46080_e59134)))), (0.5 * (locals.var_xgs_ov_dn8 + (((locals.var_xgs_ov_dn8 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn8)) / (2.0 * assign46080_e59134)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, locals.var_sp_ov_xg_dn8,)
    }
};
        locals.var_sp_ov_xg = assign46080_e59138;
        locals.var_sp_ov_xg_dn6 = assign46080_e59138_d_n6;
        locals.var_sp_ov_xg_dn7 = assign46080_e59138_d_n7;
        locals.var_sp_ov_xg_dn8 = assign46080_e59138_d_n8;
        locals.var_sp_ov_xg_rv = 0.0;

        let (assign46090_e59160, assign46090_e59160_d_n6, assign46090_e59160_d_n7, assign46090_e59160_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46090_e59141: f64 = (-locals.var_sp_ov_xg);
        let assign46090_e59144: f64 = (locals.var_gov2_s * 0.5);
        let assign46090_e59145: f64 = (assign46090_e59141 - assign46090_e59144);
        let assign46090_e59150: f64 = (locals.var_gov2_s * 0.25);
        let assign46090_e59151: f64 = (locals.var_sp_ov_xg + assign46090_e59150);
        let assign46090_e59153: f64 = (assign46090_e59151 + locals.var_sp_ov_a_s);
        let assign46090_e59154: f64 = (assign46090_e59153).sqrt();
        let assign46090_e59155: f64 = (locals.var_gov_s * assign46090_e59154);
        let assign46090_e59156: f64 = (assign46090_e59145 + assign46090_e59155);
        let assign46090_e59158: f64 = (assign46090_e59156 + locals.var_sp_ov_delta1_s);
        (assign46090_e59158, ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign46090_e59154)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign46090_e59154)))), ((-locals.var_sp_ov_xg_dn8) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn8 / (2.0 * assign46090_e59154)))),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8,)
    }
};
        locals.var_xs_ov = assign46090_e59160;
        locals.var_xs_ov_dn6 = assign46090_e59160_d_n6;
        locals.var_xs_ov_dn7 = assign46090_e59160_d_n7;
        locals.var_xs_ov_dn8 = assign46090_e59160_d_n8;
        locals.var_xs_ov_rv = 0.0;

        let (assign46100_e59173, assign46100_e59173_d_n6, assign46100_e59173_d_n7, assign46100_e59173_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46100_e59166: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
        let assign46100_e59168: f64 = (assign46100_e59166 + locals.var_sp_ov_eps2_d);
        let assign46100_e59169: f64 = (assign46100_e59168).sqrt();
        let assign46100_e59170: f64 = (locals.var_xgd_ov + assign46100_e59169);
        let assign46100_e59171: f64 = (0.5 * assign46100_e59170);
        (assign46100_e59171, (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign46100_e59169)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign46100_e59169)))), (0.5 * (locals.var_xgd_ov_dn8 + (((locals.var_xgd_ov_dn8 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn8)) / (2.0 * assign46100_e59169)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, locals.var_sp_ov_xg_dn8,)
    }
};
        locals.var_sp_ov_xg = assign46100_e59173;
        locals.var_sp_ov_xg_dn6 = assign46100_e59173_d_n6;
        locals.var_sp_ov_xg_dn7 = assign46100_e59173_d_n7;
        locals.var_sp_ov_xg_dn8 = assign46100_e59173_d_n8;
        locals.var_sp_ov_xg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign46110_e59195, assign46110_e59195_d_n6, assign46110_e59195_d_n7, assign46110_e59195_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46110_e59176: f64 = (-locals.var_sp_ov_xg);
        let assign46110_e59179: f64 = (locals.var_gov2_d * 0.5);
        let assign46110_e59180: f64 = (assign46110_e59176 - assign46110_e59179);
        let assign46110_e59185: f64 = (locals.var_gov2_d * 0.25);
        let assign46110_e59186: f64 = (locals.var_sp_ov_xg + assign46110_e59185);
        let assign46110_e59188: f64 = (assign46110_e59186 + locals.var_sp_ov_a_d);
        let assign46110_e59189: f64 = (assign46110_e59188).sqrt();
        let assign46110_e59190: f64 = (locals.var_gov_d * assign46110_e59189);
        let assign46110_e59191: f64 = (assign46110_e59180 + assign46110_e59190);
        let assign46110_e59193: f64 = (assign46110_e59191 + locals.var_sp_ov_delta1_d);
        (assign46110_e59193, ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign46110_e59189)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign46110_e59189)))), ((-locals.var_sp_ov_xg_dn8) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn8 / (2.0 * assign46110_e59189)))),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8,)
    }
};
        locals.var_xd_ov = assign46110_e59195;
        locals.var_xd_ov_dn6 = assign46110_e59195_d_n6;
        locals.var_xd_ov_dn7 = assign46110_e59195_d_n7;
        locals.var_xd_ov_dn8 = assign46110_e59195_d_n8;
        locals.var_xd_ov_rv = 0.0;

        let (assign46120_e59204, assign46120_e59204_d_n6, assign46120_e59204_d_n7, assign46120_e59204_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46120_e59198: f64 = (-locals.var_phita);
        let assign46120_e59201: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign46120_e59202: f64 = (assign46120_e59198 * assign46120_e59201);
        (assign46120_e59202, (assign46120_e59198 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign46120_e59198 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)), (assign46120_e59198 * (locals.var_xgs_ov_dn8 + locals.var_xs_ov_dn8)),)
    } else {
        (locals.var_vovs, locals.var_vovs_dn6, locals.var_vovs_dn7, locals.var_vovs_dn8,)
    }
};
        locals.var_vovs = assign46120_e59204;
        locals.var_vovs_dn6 = assign46120_e59204_d_n6;
        locals.var_vovs_dn7 = assign46120_e59204_d_n7;
        locals.var_vovs_dn8 = assign46120_e59204_d_n8;
        locals.var_vovs_rv = 0.0;

        let (assign46130_e59213, assign46130_e59213_d_n6, assign46130_e59213_d_n7, assign46130_e59213_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46130_e59207: f64 = (-locals.var_phita);
        let assign46130_e59210: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign46130_e59211: f64 = (assign46130_e59207 * assign46130_e59210);
        (assign46130_e59211, (assign46130_e59207 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign46130_e59207 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)), (assign46130_e59207 * (locals.var_xgd_ov_dn8 + locals.var_xd_ov_dn8)),)
    } else {
        (locals.var_vovd, locals.var_vovd_dn6, locals.var_vovd_dn7, locals.var_vovd_dn8,)
    }
};
        locals.var_vovd = assign46130_e59213;
        locals.var_vovd_dn6 = assign46130_e59213_d_n6;
        locals.var_vovd_dn7 = assign46130_e59213_d_n7;
        locals.var_vovd_dn8 = assign46130_e59213_d_n8;
        locals.var_vovd_rv = 0.0;

        let assign46200_e59222: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1238 = assign46200_e59222;
        locals.var_guard1238_rv = 0.0;

        let assign46210_e59225: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign46210_e59225;
        locals.var_guard1239_rv = 0.0;

        let (assign46220_e59238, assign46220_e59238_d_n4, assign46220_e59238_d_n6, assign46220_e59238_d_n7, assign46220_e59238_d_n8, assign46220_e59238_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46220_e59231: f64 = (locals.var_vovs * locals.var_vovs);
        let assign46220_e59233: f64 = (assign46220_e59231 + 1e-6);
        let assign46220_e59234: f64 = (assign46220_e59233).sqrt();
        let assign46220_e59236: f64 = (assign46220_e59234 * locals.var_inv_chib);
        (assign46220_e59236, 0.0, ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46220_e59238;
        locals.var_zg_dn4 = assign46220_e59238_d_n4;
        locals.var_zg_dn6 = assign46220_e59238_d_n6;
        locals.var_zg_dn7 = assign46220_e59238_d_n7;
        locals.var_zg_dn8 = assign46220_e59238_d_n8;
        locals.var_zg_dn9 = assign46220_e59238_d_n9;
        locals.var_zg_rv = 0.0;

        let assign46230_e59241: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign46230_e59241;
        locals.var_guard1240_rv = 0.0;

        let (assign46240_e59264, assign46240_e59264_d_n4, assign46240_e59264_d_n6, assign46240_e59264_d_n7, assign46240_e59264_d_n8, assign46240_e59264_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) && (locals.var_guard1240 != 0.0)) {
        let assign46240_e59250: f64 = (locals.var_zg + locals.var_gcqov);
        let assign46240_e59253: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46240_e59256: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46240_e59257: f64 = (assign46240_e59253 * assign46240_e59256);
        let assign46240_e59259: f64 = (assign46240_e59257 + 1e-6);
        let assign46240_e59260: f64 = (assign46240_e59259).sqrt();
        let assign46240_e59261: f64 = (assign46240_e59250 - assign46240_e59260);
        let assign46240_e59262: f64 = (0.5 * assign46240_e59261);
        (assign46240_e59262, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn4)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn6)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn7)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn8)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn9)) / (2.0 * assign46240_e59260)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46240_e59264;
        locals.var_zg_dn4 = assign46240_e59264_d_n4;
        locals.var_zg_dn6 = assign46240_e59264_d_n6;
        locals.var_zg_dn7 = assign46240_e59264_d_n7;
        locals.var_zg_dn8 = assign46240_e59264_d_n8;
        locals.var_zg_dn9 = assign46240_e59264_d_n9;
        locals.var_zg_rv = 0.0;

        let (assign46250_e59281, assign46250_e59281_d_n4, assign46250_e59281_d_n6, assign46250_e59281_d_n7, assign46250_e59281_d_n8, assign46250_e59281_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46250_e59270: f64 = (-1.5);
        let assign46250_e59275: f64 = (locals.var_gc3ov_i * locals.var_zg);
        let assign46250_e59276: f64 = (locals.var_gc2ov_i + assign46250_e59275);
        let assign46250_e59277: f64 = (locals.var_zg * assign46250_e59276);
        let assign46250_e59278: f64 = (assign46250_e59270 + assign46250_e59277);
        let assign46250_e59279: f64 = (locals.var_bov * assign46250_e59278);
        (assign46250_e59279, (locals.var_bov * ((locals.var_zg_dn4 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn4)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))), (locals.var_bov * ((locals.var_zg_dn9 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46250_e59281;
        locals.var_temp__blk949_dn4 = assign46250_e59281_d_n4;
        locals.var_temp__blk949_dn6 = assign46250_e59281_d_n6;
        locals.var_temp__blk949_dn7 = assign46250_e59281_d_n7;
        locals.var_temp__blk949_dn8 = assign46250_e59281_d_n8;
        locals.var_temp__blk949_dn9 = assign46250_e59281_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46310_e59367, assign46310_e59367_d_n6, assign46310_e59367_d_n7, assign46310_e59367_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46310_e59365: f64 = (3.0 + locals.var_xs_ov);
        (assign46310_e59365, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn6, locals.var_fs1_dn7, locals.var_fs1_dn8,)
    }
};
        locals.var_fs1 = assign46310_e59367;
        locals.var_fs1_dn6 = assign46310_e59367_d_n6;
        locals.var_fs1_dn7 = assign46310_e59367_d_n7;
        locals.var_fs1_dn8 = assign46310_e59367_d_n8;
        locals.var_fs1_rv = 0.0;

        let (assign46320_e59376,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46320_e59372: f64 = (-3.0);
        let assign46320_e59374: f64 = (assign46320_e59372 - locals.var_gco_i);
        (assign46320_e59374,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46320_e59376;
        locals.var_fs2_rv = 0.0;

        let (assign46330_e59384, assign46330_e59384_d_n6, assign46330_e59384_d_n7, assign46330_e59384_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46330_e59382: f64 = (30.0 * locals.var_vgsprime);
        (assign46330_e59382, (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7), (30.0 * locals.var_vgsprime_dn8),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn6, locals.var_fs3_dn7, locals.var_fs3_dn8,)
    }
};
        locals.var_fs3 = assign46330_e59384;
        locals.var_fs3_dn6 = assign46330_e59384_d_n6;
        locals.var_fs3_dn7 = assign46330_e59384_d_n7;
        locals.var_fs3_dn8 = assign46330_e59384_d_n8;
        locals.var_fs3_rv = 0.0;

        let (assign46340_e59392,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46340_e59390: f64 = (4.0 - 0.9);
        (assign46340_e59390,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46340_e59392;
        locals.var_tme1_rv = 0.0;

        let (assign46350_e59400, assign46350_e59400_d_n4, assign46350_e59400_d_n6, assign46350_e59400_d_n7, assign46350_e59400_d_n8, assign46350_e59400_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46350_e59398: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46350_e59398, 0.0, (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), (locals.var_fs1_dn8 + locals.var_fs3_dn8), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46350_e59400;
        locals.var_tme2_dn4 = assign46350_e59400_d_n4;
        locals.var_tme2_dn6 = assign46350_e59400_d_n6;
        locals.var_tme2_dn7 = assign46350_e59400_d_n7;
        locals.var_tme2_dn8 = assign46350_e59400_d_n8;
        locals.var_tme2_dn9 = assign46350_e59400_d_n9;
        locals.var_tme2_rv = 0.0;

        let (assign46360_e59421, assign46360_e59421_d_n4, assign46360_e59421_d_n6, assign46360_e59421_d_n7, assign46360_e59421_d_n8, assign46360_e59421_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46360_e59406: f64 = (2.0 / locals.var_tme1);
        let assign46360_e59410: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46360_e59413: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46360_e59415: f64 = (assign46360_e59413 * locals.var_fs3);
        let assign46360_e59416: f64 = (assign46360_e59410 - assign46360_e59415);
        let assign46360_e59417: f64 = (assign46360_e59416).sqrt();
        let assign46360_e59418: f64 = (locals.var_tme2 - assign46360_e59417);
        let assign46360_e59419: f64 = (assign46360_e59406 * assign46360_e59418);
        (assign46360_e59419, (assign46360_e59406 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn6))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn7))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn8))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46360_e59417)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46360_e59421;
        locals.var_temp__blk949_dn4 = assign46360_e59421_d_n4;
        locals.var_temp__blk949_dn6 = assign46360_e59421_d_n6;
        locals.var_temp__blk949_dn7 = assign46360_e59421_d_n7;
        locals.var_temp__blk949_dn8 = assign46360_e59421_d_n8;
        locals.var_temp__blk949_dn9 = assign46360_e59421_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46370_e59429,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46370_e59427: f64 = (4.0 - 0.3);
        (assign46370_e59427,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46370_e59429;
        locals.var_tme1_rv = 0.0;

        let (assign46380_e59437, assign46380_e59437_d_n4, assign46380_e59437_d_n6, assign46380_e59437_d_n7, assign46380_e59437_d_n8, assign46380_e59437_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46380_e59435: f64 = (locals.var_fs2 + locals.var_temp__blk949);
        (assign46380_e59435, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46380_e59437;
        locals.var_tme2_dn4 = assign46380_e59437_d_n4;
        locals.var_tme2_dn6 = assign46380_e59437_d_n6;
        locals.var_tme2_dn7 = assign46380_e59437_d_n7;
        locals.var_tme2_dn8 = assign46380_e59437_d_n8;
        locals.var_tme2_dn9 = assign46380_e59437_d_n9;
        locals.var_tme2_rv = 0.0;

        let assign46410_e59471: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46410_e59471;
        locals.var_guard1243_rv = 0.0;

        let (assign46420_e59484, assign46420_e59484_d_n4, assign46420_e59484_d_n6, assign46420_e59484_d_n7, assign46420_e59484_d_n8, assign46420_e59484_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46420_e59477: f64 = (locals.var_vovd * locals.var_vovd);
        let assign46420_e59479: f64 = (assign46420_e59477 + 1e-6);
        let assign46420_e59480: f64 = (assign46420_e59479).sqrt();
        let assign46420_e59482: f64 = (assign46420_e59480 * locals.var_inv_chib);
        (assign46420_e59482, 0.0, ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46420_e59484;
        locals.var_zg_dn4 = assign46420_e59484_d_n4;
        locals.var_zg_dn6 = assign46420_e59484_d_n6;
        locals.var_zg_dn7 = assign46420_e59484_d_n7;
        locals.var_zg_dn8 = assign46420_e59484_d_n8;
        locals.var_zg_dn9 = assign46420_e59484_d_n9;
        locals.var_zg_rv = 0.0;

        let assign46430_e59487: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign46430_e59487;
        locals.var_guard1244_rv = 0.0;

        let (assign46440_e59510, assign46440_e59510_d_n4, assign46440_e59510_d_n6, assign46440_e59510_d_n7, assign46440_e59510_d_n8, assign46440_e59510_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1244 != 0.0)) {
        let assign46440_e59496: f64 = (locals.var_zg + locals.var_gcqovd);
        let assign46440_e59499: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46440_e59502: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46440_e59503: f64 = (assign46440_e59499 * assign46440_e59502);
        let assign46440_e59505: f64 = (assign46440_e59503 + 1e-6);
        let assign46440_e59506: f64 = (assign46440_e59505).sqrt();
        let assign46440_e59507: f64 = (assign46440_e59496 - assign46440_e59506);
        let assign46440_e59508: f64 = (0.5 * assign46440_e59507);
        (assign46440_e59508, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn4)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn6)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn7)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn8)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn9)) / (2.0 * assign46440_e59506)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46440_e59510;
        locals.var_zg_dn4 = assign46440_e59510_d_n4;
        locals.var_zg_dn6 = assign46440_e59510_d_n6;
        locals.var_zg_dn7 = assign46440_e59510_d_n7;
        locals.var_zg_dn8 = assign46440_e59510_d_n8;
        locals.var_zg_dn9 = assign46440_e59510_d_n9;
        locals.var_zg_rv = 0.0;

        let (assign46450_e59527, assign46450_e59527_d_n4, assign46450_e59527_d_n6, assign46450_e59527_d_n7, assign46450_e59527_d_n8, assign46450_e59527_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46450_e59516: f64 = (-1.5);
        let assign46450_e59521: f64 = (locals.var_gc3ovd_i * locals.var_zg);
        let assign46450_e59522: f64 = (locals.var_gc2ovd_i + assign46450_e59521);
        let assign46450_e59523: f64 = (locals.var_zg * assign46450_e59522);
        let assign46450_e59524: f64 = (assign46450_e59516 + assign46450_e59523);
        let assign46450_e59525: f64 = (locals.var_bov_d * assign46450_e59524);
        (assign46450_e59525, (locals.var_bov_d * ((locals.var_zg_dn4 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn4)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))), (locals.var_bov_d * ((locals.var_zg_dn9 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46450_e59527;
        locals.var_temp__blk949_dn4 = assign46450_e59527_d_n4;
        locals.var_temp__blk949_dn6 = assign46450_e59527_d_n6;
        locals.var_temp__blk949_dn7 = assign46450_e59527_d_n7;
        locals.var_temp__blk949_dn8 = assign46450_e59527_d_n8;
        locals.var_temp__blk949_dn9 = assign46450_e59527_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46510_e59613, assign46510_e59613_d_n6, assign46510_e59613_d_n7, assign46510_e59613_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46510_e59611: f64 = (3.0 + locals.var_xd_ov);
        (assign46510_e59611, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn6, locals.var_fs1_dn7, locals.var_fs1_dn8,)
    }
};
        locals.var_fs1 = assign46510_e59613;
        locals.var_fs1_dn6 = assign46510_e59613_d_n6;
        locals.var_fs1_dn7 = assign46510_e59613_d_n7;
        locals.var_fs1_dn8 = assign46510_e59613_d_n8;
        locals.var_fs1_rv = 0.0;

        let (assign46520_e59622,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46520_e59618: f64 = (-3.0);
        let assign46520_e59620: f64 = (assign46520_e59618 - locals.var_gco_i);
        (assign46520_e59620,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46520_e59622;
        locals.var_fs2_rv = 0.0;

        let (assign46530_e59630, assign46530_e59630_d_n6, assign46530_e59630_d_n7, assign46530_e59630_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46530_e59628: f64 = (30.0 * locals.var_vgdprime);
        (assign46530_e59628, (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7), (30.0 * locals.var_vgdprime_dn8),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn6, locals.var_fs3_dn7, locals.var_fs3_dn8,)
    }
};
        locals.var_fs3 = assign46530_e59630;
        locals.var_fs3_dn6 = assign46530_e59630_d_n6;
        locals.var_fs3_dn7 = assign46530_e59630_d_n7;
        locals.var_fs3_dn8 = assign46530_e59630_d_n8;
        locals.var_fs3_rv = 0.0;

        let (assign46540_e59638,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46540_e59636: f64 = (4.0 - 0.9);
        (assign46540_e59636,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46540_e59638;
        locals.var_tme1_rv = 0.0;

        let (assign46550_e59646, assign46550_e59646_d_n4, assign46550_e59646_d_n6, assign46550_e59646_d_n7, assign46550_e59646_d_n8, assign46550_e59646_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46550_e59644: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46550_e59644, 0.0, (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), (locals.var_fs1_dn8 + locals.var_fs3_dn8), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46550_e59646;
        locals.var_tme2_dn4 = assign46550_e59646_d_n4;
        locals.var_tme2_dn6 = assign46550_e59646_d_n6;
        locals.var_tme2_dn7 = assign46550_e59646_d_n7;
        locals.var_tme2_dn8 = assign46550_e59646_d_n8;
        locals.var_tme2_dn9 = assign46550_e59646_d_n9;
        locals.var_tme2_rv = 0.0;

        let (assign46560_e59667, assign46560_e59667_d_n4, assign46560_e59667_d_n6, assign46560_e59667_d_n7, assign46560_e59667_d_n8, assign46560_e59667_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46560_e59652: f64 = (2.0 / locals.var_tme1);
        let assign46560_e59656: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46560_e59659: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46560_e59661: f64 = (assign46560_e59659 * locals.var_fs3);
        let assign46560_e59662: f64 = (assign46560_e59656 - assign46560_e59661);
        let assign46560_e59663: f64 = (assign46560_e59662).sqrt();
        let assign46560_e59664: f64 = (locals.var_tme2 - assign46560_e59663);
        let assign46560_e59665: f64 = (assign46560_e59652 * assign46560_e59664);
        (assign46560_e59665, (assign46560_e59652 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn6))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn7))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn8))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46560_e59663)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46560_e59667;
        locals.var_temp__blk949_dn4 = assign46560_e59667_d_n4;
        locals.var_temp__blk949_dn6 = assign46560_e59667_d_n6;
        locals.var_temp__blk949_dn7 = assign46560_e59667_d_n7;
        locals.var_temp__blk949_dn8 = assign46560_e59667_d_n8;
        locals.var_temp__blk949_dn9 = assign46560_e59667_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46570_e59675,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46570_e59673: f64 = (4.0 - 0.3);
        (assign46570_e59673,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46570_e59675;
        locals.var_tme1_rv = 0.0;

        let (assign46580_e59683, assign46580_e59683_d_n4, assign46580_e59683_d_n6, assign46580_e59683_d_n7, assign46580_e59683_d_n8, assign46580_e59683_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46580_e59681: f64 = (locals.var_fs2 + locals.var_temp__blk949);
        (assign46580_e59681, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46580_e59683;
        locals.var_tme2_dn4 = assign46580_e59683_d_n4;
        locals.var_tme2_dn6 = assign46580_e59683_d_n6;
        locals.var_tme2_dn7 = assign46580_e59683_d_n7;
        locals.var_tme2_dn8 = assign46580_e59683_d_n8;
        locals.var_tme2_dn9 = assign46580_e59683_d_n9;
        locals.var_tme2_rv = 0.0;

        let assign46610_e59717: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign46610_e59717;
        locals.var_guard1247_rv = 0.0;

        let assign46620_e59720: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign46620_e59720;
        locals.var_guard1248_rv = 0.0;

        let (assign46630_e59730, assign46630_e59730_d_n4, assign46630_e59730_d_n6, assign46630_e59730_d_n7, assign46630_e59730_d_n8, assign46630_e59730_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46630_e59728: f64 = (1.0 + locals.var_ar);
        (assign46630_e59728, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46630_e59730;
        locals.var_temp__blk949_dn4 = assign46630_e59730_d_n4;
        locals.var_temp__blk949_dn6 = assign46630_e59730_d_n6;
        locals.var_temp__blk949_dn7 = assign46630_e59730_d_n7;
        locals.var_temp__blk949_dn8 = assign46630_e59730_d_n8;
        locals.var_temp__blk949_dn9 = assign46630_e59730_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46640_e59743, assign46640_e59743_d_n4, assign46640_e59743_d_n6, assign46640_e59743_d_n7, assign46640_e59743_d_n8, assign46640_e59743_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46640_e59737: f64 = (locals.var_temp__blk949).sqrt();
        let assign46640_e59739: f64 = (assign46640_e59737 * locals.var_v_ds);
        let assign46640_e59741: f64 = (assign46640_e59739 / locals.var_vdsat_lim_dc);
        (assign46640_e59741, (((((locals.var_temp__blk949_dn4 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn4)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn8)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn9)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign46640_e59743;
        locals.var_temp1_dn4 = assign46640_e59743_d_n4;
        locals.var_temp1_dn6 = assign46640_e59743_d_n6;
        locals.var_temp1_dn7 = assign46640_e59743_d_n7;
        locals.var_temp1_dn8 = assign46640_e59743_d_n8;
        locals.var_temp1_dn9 = assign46640_e59743_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign46650_e59755, assign46650_e59755_d_n4, assign46650_e59755_d_n6, assign46650_e59755_d_n7, assign46650_e59755_d_n8, assign46650_e59755_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46650_e59751: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign46650_e59753: f64 = (assign46650_e59751 + locals.var_temp__blk949);
        (assign46650_e59753, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign46650_e59755;
        locals.var_temp2_dn4 = assign46650_e59755_d_n4;
        locals.var_temp2_dn6 = assign46650_e59755_d_n6;
        locals.var_temp2_dn7 = assign46650_e59755_d_n7;
        locals.var_temp2_dn8 = assign46650_e59755_d_n8;
        locals.var_temp2_dn9 = assign46650_e59755_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign46660_e59765, assign46660_e59765_d_n4, assign46660_e59765_d_n6, assign46660_e59765_d_n7, assign46660_e59765_d_n8, assign46660_e59765_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46660_e59763: f64 = (2.0 * locals.var_temp1);
        (assign46660_e59763, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46660_e59765;
        locals.var_temp__blk949_dn4 = assign46660_e59765_d_n4;
        locals.var_temp__blk949_dn6 = assign46660_e59765_d_n6;
        locals.var_temp__blk949_dn7 = assign46660_e59765_d_n7;
        locals.var_temp__blk949_dn8 = assign46660_e59765_d_n8;
        locals.var_temp__blk949_dn9 = assign46660_e59765_d_n9;
        locals.var_temp__blk949_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign46670_e59787, assign46670_e59787_d_n4, assign46670_e59787_d_n6, assign46670_e59787_d_n7, assign46670_e59787_d_n8, assign46670_e59787_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46670_e59773: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
        let assign46670_e59775: f64 = (assign46670_e59773 * locals.var_temp__blk949);
        let assign46670_e59778: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign46670_e59779: f64 = (assign46670_e59778).sqrt();
        let assign46670_e59782: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign46670_e59783: f64 = (assign46670_e59782).sqrt();
        let assign46670_e59784: f64 = (assign46670_e59779 + assign46670_e59783);
        let assign46670_e59785: f64 = (assign46670_e59775 / assign46670_e59784);
        (assign46670_e59785, (((((((locals.var_vdsat_lim_dc_dn4 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn4)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn4)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn6)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn7)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn8)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn9 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn9)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn9)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)),)
    } else {
        (locals.var_udse_dc, locals.var_udse_dc_dn4, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8, locals.var_udse_dc_dn9,)
    }
};
        locals.var_udse_dc = assign46670_e59787;
        locals.var_udse_dc_dn4 = assign46670_e59787_d_n4;
        locals.var_udse_dc_dn6 = assign46670_e59787_d_n6;
        locals.var_udse_dc_dn7 = assign46670_e59787_d_n7;
        locals.var_udse_dc_dn8 = assign46670_e59787_d_n8;
        locals.var_udse_dc_dn9 = assign46670_e59787_d_n9;
        locals.var_udse_dc_rv = 0.0;

        let assign46680_e59790: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46680_e59792: f64 = (-230.25850929940458);
        let assign46680_e59793: f64 = if assign46680_e59790 > assign46680_e59792 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign46680_e59793;
        locals.var_guard1249_rv = 0.0;

        let (assign46690_e59804, assign46690_e59804_d_n4, assign46690_e59804_d_n6, assign46690_e59804_d_n7, assign46690_e59804_d_n8, assign46690_e59804_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1249 != 0.0)) {
        let assign46690_e59801: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46690_e59802: f64 = (assign46690_e59801).exp();
        (assign46690_e59802, (assign46690_e59802 * (locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)), (assign46690_e59802 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46690_e59802 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46690_e59802 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)), (assign46690_e59802 * (locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46690_e59804;
        locals.var_temp__blk949_dn4 = assign46690_e59804_d_n4;
        locals.var_temp__blk949_dn6 = assign46690_e59804_d_n6;
        locals.var_temp__blk949_dn7 = assign46690_e59804_d_n7;
        locals.var_temp__blk949_dn8 = assign46690_e59804_d_n8;
        locals.var_temp__blk949_dn9 = assign46690_e59804_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46700_e59844, assign46700_e59844_d_n4, assign46700_e59844_d_n6, assign46700_e59844_d_n7, assign46700_e59844_d_n8, assign46700_e59844_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1249 == 0.0)) {
        let assign46700_e59814: f64 = (-230.25850929940458);
        let assign46700_e59817: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46700_e59818: f64 = (assign46700_e59814 - assign46700_e59817);
        let assign46700_e59822: f64 = (-230.25850929940458);
        let assign46700_e59825: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46700_e59826: f64 = (assign46700_e59822 - assign46700_e59825);
        let assign46700_e59829: f64 = (-230.25850929940458);
        let assign46700_e59832: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46700_e59833: f64 = (assign46700_e59829 - assign46700_e59832);
        let assign46700_e59835: f64 = (assign46700_e59833 * 0.3333333333333333);
        let assign46700_e59836: f64 = (1.0 + assign46700_e59835);
        let assign46700_e59837: f64 = (assign46700_e59826 * assign46700_e59836);
        let assign46700_e59838: f64 = (0.5 * assign46700_e59837);
        let assign46700_e59839: f64 = (1.0 + assign46700_e59838);
        let assign46700_e59840: f64 = (assign46700_e59818 * assign46700_e59839);
        let assign46700_e59841: f64 = (1.0 + assign46700_e59840);
        let assign46700_e59842: f64 = (1e-100 / assign46700_e59841);
        (assign46700_e59842, (-((1e-100 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46700_e59844;
        locals.var_temp__blk949_dn4 = assign46700_e59844_d_n4;
        locals.var_temp__blk949_dn6 = assign46700_e59844_d_n6;
        locals.var_temp__blk949_dn7 = assign46700_e59844_d_n7;
        locals.var_temp__blk949_dn8 = assign46700_e59844_d_n8;
        locals.var_temp__blk949_dn9 = assign46700_e59844_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46710_e59863, assign46710_e59863_d_n4, assign46710_e59863_d_n6, assign46710_e59863_d_n7, assign46710_e59863_d_n8, assign46710_e59863_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46710_e59852: f64 = (0.5 * locals.var_x_ds_dc);
        let assign46710_e59856: f64 = (1.0 + locals.var_temp__blk949);
        let assign46710_e59857: f64 = (0.5 * assign46710_e59856);
        let assign46710_e59858: f64 = (assign46710_e59857).ln();
        let assign46710_e59859: f64 = (assign46710_e59852 - assign46710_e59858);
        let assign46710_e59860: f64 = (locals.var_phit1_dc * assign46710_e59859);
        let assign46710_e59861: f64 = (locals.var_vsbstar_dc + assign46710_e59860);
        (assign46710_e59861, (locals.var_vsbstar_dc_dn4 + ((locals.var_phit1_dc_dn4 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn4) - ((0.5 * locals.var_temp__blk949_dn4) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk949_dn6) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk949_dn7) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk949_dn8) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn9 + ((locals.var_phit1_dc_dn9 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn9) - ((0.5 * locals.var_temp__blk949_dn9) / assign46710_e59857))))),)
    } else {
        (locals.var_vm, locals.var_vm_dn4, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, locals.var_vm_dn9,)
    }
};
        locals.var_vm = assign46710_e59863;
        locals.var_vm_dn4 = assign46710_e59863_d_n4;
        locals.var_vm_dn6 = assign46710_e59863_d_n6;
        locals.var_vm_dn7 = assign46710_e59863_d_n7;
        locals.var_vm_dn8 = assign46710_e59863_d_n8;
        locals.var_vm_dn9 = assign46710_e59863_d_n9;
        locals.var_vm_rv = 0.0;

        let (assign46720_e59871, assign46720_e59871_d_n4, assign46720_e59871_d_n6, assign46720_e59871_d_n7, assign46720_e59871_d_n8, assign46720_e59871_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46720_e59869: f64 = (locals.var_gco_i * locals.var_phit1_dc);
        (assign46720_e59869, (locals.var_gco_i * locals.var_phit1_dc_dn4), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8), (locals.var_gco_i * locals.var_phit1_dc_dn9),)
    } else {
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9,)
    }
};
        locals.var_dch = assign46720_e59871;
        locals.var_dch_dn4 = assign46720_e59871_d_n4;
        locals.var_dch_dn6 = assign46720_e59871_d_n6;
        locals.var_dch_dn7 = assign46720_e59871_d_n7;
        locals.var_dch_dn8 = assign46720_e59871_d_n8;
        locals.var_dch_dn9 = assign46720_e59871_d_n9;
        locals.var_dch_rv = 0.0;

        let (assign46730_e59879, assign46730_e59879_d_n4, assign46730_e59879_d_n6, assign46730_e59879_d_n7, assign46730_e59879_d_n8, assign46730_e59879_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46730_e59877: f64 = (locals.var_voxm_dc + locals.var_dch);
        (assign46730_e59877, (locals.var_voxm_dc_dn4 + locals.var_dch_dn4), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8), (locals.var_voxm_dc_dn9 + locals.var_dch_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign46730_e59879;
        locals.var_arg2mina_dn4 = assign46730_e59879_d_n4;
        locals.var_arg2mina_dn6 = assign46730_e59879_d_n6;
        locals.var_arg2mina_dn7 = assign46730_e59879_d_n7;
        locals.var_arg2mina_dn8 = assign46730_e59879_d_n8;
        locals.var_arg2mina_dn9 = assign46730_e59879_d_n9;
        locals.var_arg2mina_rv = 0.0;

        let (assign46740_e59900, assign46740_e59900_d_n4, assign46740_e59900_d_n6, assign46740_e59900_d_n7, assign46740_e59900_d_n8, assign46740_e59900_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46740_e59886: f64 = locals.var_arg2mina;
        let assign46740_e59889: f64 = (-locals.var_arg2mina);
        let assign46740_e59892: f64 = (-locals.var_arg2mina);
        let assign46740_e59893: f64 = (assign46740_e59889 * assign46740_e59892);
        let assign46740_e59895: f64 = (assign46740_e59893 + 0.01);
        let assign46740_e59896: f64 = (assign46740_e59895).sqrt();
        let assign46740_e59897: f64 = (assign46740_e59886 - assign46740_e59896);
        let assign46740_e59898: f64 = (0.5 * assign46740_e59897);
        (assign46740_e59898, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn4))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn9))) / (2.0 * assign46740_e59896)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign46740_e59900;
        locals.var_psi_t_dn4 = assign46740_e59900_d_n4;
        locals.var_psi_t_dn6 = assign46740_e59900_d_n6;
        locals.var_psi_t_dn7 = assign46740_e59900_d_n7;
        locals.var_psi_t_dn8 = assign46740_e59900_d_n8;
        locals.var_psi_t_dn9 = assign46740_e59900_d_n9;
        locals.var_psi_t_rv = 0.0;

        let (assign46750_e59913, assign46750_e59913_d_n4, assign46750_e59913_d_n6, assign46750_e59913_d_n7, assign46750_e59913_d_n8, assign46750_e59913_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46750_e59906: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
        let assign46750_e59908: f64 = (assign46750_e59906 + 1e-6);
        let assign46750_e59909: f64 = (assign46750_e59908).sqrt();
        let assign46750_e59911: f64 = (assign46750_e59909 * locals.var_inv_chib);
        (assign46750_e59911, ((((locals.var_voxm_dc_dn4 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn4)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn9 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn9)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46750_e59913;
        locals.var_zg_dn4 = assign46750_e59913_d_n4;
        locals.var_zg_dn6 = assign46750_e59913_d_n6;
        locals.var_zg_dn7 = assign46750_e59913_d_n7;
        locals.var_zg_dn8 = assign46750_e59913_d_n8;
        locals.var_zg_dn9 = assign46750_e59913_d_n9;
        locals.var_zg_rv = 0.0;

        let assign46760_e59916: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign46760_e59916;
        locals.var_guard1250_rv = 0.0;

        let (assign46770_e59939, assign46770_e59939_d_n4, assign46770_e59939_d_n6, assign46770_e59939_d_n7, assign46770_e59939_d_n8, assign46770_e59939_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1250 != 0.0)) {
        let assign46770_e59925: f64 = (locals.var_zg + locals.var_gcq);
        let assign46770_e59928: f64 = (locals.var_zg - locals.var_gcq);
        let assign46770_e59931: f64 = (locals.var_zg - locals.var_gcq);
        let assign46770_e59932: f64 = (assign46770_e59928 * assign46770_e59931);
        let assign46770_e59934: f64 = (assign46770_e59932 + 1e-6);
        let assign46770_e59935: f64 = (assign46770_e59934).sqrt();
        let assign46770_e59936: f64 = (assign46770_e59925 - assign46770_e59935);
        let assign46770_e59937: f64 = (0.5 * assign46770_e59936);
        (assign46770_e59937, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn4)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn6)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn7)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn8)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn9)) / (2.0 * assign46770_e59935)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46770_e59939;
        locals.var_zg_dn4 = assign46770_e59939_d_n4;
        locals.var_zg_dn6 = assign46770_e59939_d_n6;
        locals.var_zg_dn7 = assign46770_e59939_d_n7;
        locals.var_zg_dn8 = assign46770_e59939_d_n8;
        locals.var_zg_dn9 = assign46770_e59939_d_n9;
        locals.var_zg_rv = 0.0;

        let (assign46780_e59953, assign46780_e59953_d_n4, assign46780_e59953_d_n6, assign46780_e59953_d_n7, assign46780_e59953_d_n8, assign46780_e59953_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46780_e59946: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign46780_e59948: f64 = (assign46780_e59946 - locals.var_vm);
        let assign46780_e59950: f64 = (assign46780_e59948 * locals.var_inv_phit1_dc);
        let assign46780_e59951: f64 = (locals.var_x_m_dc + assign46780_e59950);
        (assign46780_e59951, (locals.var_x_m_dc_dn4 + ((((locals.var_psi_t_dn4 - locals.var_alpha_b_dn4) - locals.var_vm_dn4) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn4))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn8))), (locals.var_x_m_dc_dn9 + (((locals.var_psi_t_dn9 - locals.var_vm_dn9) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign46780_e59953;
        locals.var_arg1_dn4 = assign46780_e59953_d_n4;
        locals.var_arg1_dn6 = assign46780_e59953_d_n6;
        locals.var_arg1_dn7 = assign46780_e59953_d_n7;
        locals.var_arg1_dn8 = assign46780_e59953_d_n8;
        locals.var_arg1_dn9 = assign46780_e59953_d_n9;
        locals.var_arg1_rv = 0.0;

        let (assign46840_e60052, assign46840_e60052_d_n4, assign46840_e60052_d_n6, assign46840_e60052_d_n7, assign46840_e60052_d_n8, assign46840_e60052_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46840_e60045: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
        let assign46840_e60047: f64 = (assign46840_e60045 - locals.var_vm);
        let assign46840_e60048: f64 = (-assign46840_e60047);
        let assign46840_e60050: f64 = (assign46840_e60048 * locals.var_inv_phit1_dc);
        (assign46840_e60050, (((-(locals.var_vsbstar_dc_dn4 - locals.var_vm_dn4)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn4)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn7)), (((-((locals.var_v_gs_dn8 + locals.var_vsbstar_dc_dn8) - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn8)), (((-(locals.var_vsbstar_dc_dn9 - locals.var_vm_dn9)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign46840_e60052;
        locals.var_arg1_dn4 = assign46840_e60052_d_n4;
        locals.var_arg1_dn6 = assign46840_e60052_d_n6;
        locals.var_arg1_dn7 = assign46840_e60052_d_n7;
        locals.var_arg1_dn8 = assign46840_e60052_d_n8;
        locals.var_arg1_dn9 = assign46840_e60052_d_n9;
        locals.var_arg1_rv = 0.0;

        let assign46850_e60054: f64 = (locals.var_arg1).abs();
        let assign46850_e60056: f64 = if assign46850_e60054 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign46850_e60056;
        locals.var_guard1253_rv = 0.0;

        let (assign46860_e60065, assign46860_e60065_d_n4, assign46860_e60065_d_n6, assign46860_e60065_d_n7, assign46860_e60065_d_n8, assign46860_e60065_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 != 0.0)) {
        let assign46860_e60063: f64 = (locals.var_arg1).exp();
        (assign46860_e60063, (assign46860_e60063 * locals.var_arg1_dn4), (assign46860_e60063 * locals.var_arg1_dn6), (assign46860_e60063 * locals.var_arg1_dn7), (assign46860_e60063 * locals.var_arg1_dn8), (assign46860_e60063 * locals.var_arg1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46860_e60065;
        locals.var_temp__blk949_dn4 = assign46860_e60065_d_n4;
        locals.var_temp__blk949_dn6 = assign46860_e60065_d_n6;
        locals.var_temp__blk949_dn7 = assign46860_e60065_d_n7;
        locals.var_temp__blk949_dn8 = assign46860_e60065_d_n8;
        locals.var_temp__blk949_dn9 = assign46860_e60065_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign46870_e60068: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign46870_e60068;
        locals.var_guard1254_rv = 0.0;

        let (assign46880_e60104, assign46880_e60104_d_n4, assign46880_e60104_d_n6, assign46880_e60104_d_n7, assign46880_e60104_d_n8, assign46880_e60104_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 != 0.0)) {
        let assign46880_e60080: f64 = (-230.25850929940458);
        let assign46880_e60082: f64 = (assign46880_e60080 - locals.var_arg1);
        let assign46880_e60086: f64 = (-230.25850929940458);
        let assign46880_e60088: f64 = (assign46880_e60086 - locals.var_arg1);
        let assign46880_e60091: f64 = (-230.25850929940458);
        let assign46880_e60093: f64 = (assign46880_e60091 - locals.var_arg1);
        let assign46880_e60095: f64 = (assign46880_e60093 * 0.3333333333333333);
        let assign46880_e60096: f64 = (1.0 + assign46880_e60095);
        let assign46880_e60097: f64 = (assign46880_e60088 * assign46880_e60096);
        let assign46880_e60098: f64 = (0.5 * assign46880_e60097);
        let assign46880_e60099: f64 = (1.0 + assign46880_e60098);
        let assign46880_e60100: f64 = (assign46880_e60082 * assign46880_e60099);
        let assign46880_e60101: f64 = (1.0 + assign46880_e60100);
        let assign46880_e60102: f64 = (1e-100 / assign46880_e60101);
        (assign46880_e60102, (-((1e-100 * (((-locals.var_arg1_dn4) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn4) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn4) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn6) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn7) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn8) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn9) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn9) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn9) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46880_e60104;
        locals.var_temp__blk949_dn4 = assign46880_e60104_d_n4;
        locals.var_temp__blk949_dn6 = assign46880_e60104_d_n6;
        locals.var_temp__blk949_dn7 = assign46880_e60104_d_n7;
        locals.var_temp__blk949_dn8 = assign46880_e60104_d_n8;
        locals.var_temp__blk949_dn9 = assign46880_e60104_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46890_e60138, assign46890_e60138_d_n4, assign46890_e60138_d_n6, assign46890_e60138_d_n7, assign46890_e60138_d_n8, assign46890_e60138_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 == 0.0)) {
        let assign46890_e60118: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46890_e60123: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46890_e60127: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46890_e60129: f64 = (assign46890_e60127 * 0.3333333333333333);
        let assign46890_e60130: f64 = (1.0 + assign46890_e60129);
        let assign46890_e60131: f64 = (assign46890_e60123 * assign46890_e60130);
        let assign46890_e60132: f64 = (0.5 * assign46890_e60131);
        let assign46890_e60133: f64 = (1.0 + assign46890_e60132);
        let assign46890_e60134: f64 = (assign46890_e60118 * assign46890_e60133);
        let assign46890_e60135: f64 = (1.0 + assign46890_e60134);
        let assign46890_e60136: f64 = (1e100 * assign46890_e60135);
        (assign46890_e60136, (1e100 * ((locals.var_arg1_dn4 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn4 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn6 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn7 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn8 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn9 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn9 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46890_e60138;
        locals.var_temp__blk949_dn4 = assign46890_e60138_d_n4;
        locals.var_temp__blk949_dn6 = assign46890_e60138_d_n6;
        locals.var_temp__blk949_dn7 = assign46890_e60138_d_n7;
        locals.var_temp__blk949_dn8 = assign46890_e60138_d_n8;
        locals.var_temp__blk949_dn9 = assign46890_e60138_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign46910_e60163, assign46910_e60163_d_n4, assign46910_e60163_d_n6, assign46910_e60163_d_n7, assign46910_e60163_d_n8, assign46910_e60163_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46910_e60152: f64 = (-1.5);
        let assign46910_e60157: f64 = (locals.var_gc3_i * locals.var_zg);
        let assign46910_e60158: f64 = (locals.var_gc2_i + assign46910_e60157);
        let assign46910_e60159: f64 = (locals.var_zg * assign46910_e60158);
        let assign46910_e60160: f64 = (assign46910_e60152 + assign46910_e60159);
        let assign46910_e60161: f64 = (locals.var_bch * assign46910_e60160);
        (assign46910_e60161, (locals.var_bch * ((locals.var_zg_dn4 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn4)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))), (locals.var_bch * ((locals.var_zg_dn9 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46910_e60163;
        locals.var_temp__blk949_dn4 = assign46910_e60163_d_n4;
        locals.var_temp__blk949_dn6 = assign46910_e60163_d_n6;
        locals.var_temp__blk949_dn7 = assign46910_e60163_d_n7;
        locals.var_temp__blk949_dn8 = assign46910_e60163_d_n8;
        locals.var_temp__blk949_dn9 = assign46910_e60163_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign46980_e60269: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign46980_e60269;
        locals.var_guard1257_rv = 0.0;

        let (assign47010_e60300, assign47010_e60300_d_n4, assign47010_e60300_d_n6, assign47010_e60300_d_n7, assign47010_e60300_d_n8, assign47010_e60300_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47010_e60295: f64 = (2.0 * locals.var_gc3_i);
        let assign47010_e60297: f64 = (assign47010_e60295 * locals.var_zg);
        let assign47010_e60298: f64 = (locals.var_gc2_i + assign47010_e60297);
        (assign47010_e60298, (assign47010_e60295 * locals.var_zg_dn4), (assign47010_e60295 * locals.var_zg_dn6), (assign47010_e60295 * locals.var_zg_dn7), (assign47010_e60295 * locals.var_zg_dn8), (assign47010_e60295 * locals.var_zg_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47010_e60300;
        locals.var_temp__blk949_dn4 = assign47010_e60300_d_n4;
        locals.var_temp__blk949_dn6 = assign47010_e60300_d_n6;
        locals.var_temp__blk949_dn7 = assign47010_e60300_d_n7;
        locals.var_temp__blk949_dn8 = assign47010_e60300_d_n8;
        locals.var_temp__blk949_dn9 = assign47010_e60300_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign47020_e60313, assign47020_e60313_d_n4, assign47020_e60313_d_n6, assign47020_e60313_d_n7, assign47020_e60313_d_n8, assign47020_e60313_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47020_e60310: f64 = (locals.var_temp__blk949 * locals.var_bch);
        let assign47020_e60311: f64 = (locals.var_chib_i / assign47020_e60310);
        (assign47020_e60311, (-((locals.var_chib_i * (locals.var_temp__blk949_dn4 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn6 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn7 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn8 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn9 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))),)
    } else {
        (locals.var_u0, locals.var_u0_dn4, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, locals.var_u0_dn9,)
    }
};
        locals.var_u0 = assign47020_e60313;
        locals.var_u0_dn4 = assign47020_e60313_d_n4;
        locals.var_u0_dn6 = assign47020_e60313_d_n6;
        locals.var_u0_dn7 = assign47020_e60313_d_n7;
        locals.var_u0_dn8 = assign47020_e60313_d_n8;
        locals.var_u0_dn9 = assign47020_e60313_d_n9;
        locals.var_u0_rv = 0.0;

        let (assign47030_e60326, assign47030_e60326_d_n4, assign47030_e60326_d_n6, assign47030_e60326_d_n7, assign47030_e60326_d_n8, assign47030_e60326_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47030_e60323: f64 = (locals.var_dps_dc / locals.var_u0);
        let assign47030_e60324: f64 = (0.5 * assign47030_e60323);
        (assign47030_e60324, (0.5 * (((locals.var_dps_dc_dn4 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn4)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn9 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn9)) / (locals.var_u0 * locals.var_u0))),)
    } else {
        (locals.var_x, locals.var_x_dn4, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign47030_e60326;
        locals.var_x_dn4 = assign47030_e60326_d_n4;
        locals.var_x_dn6 = assign47030_e60326_d_n6;
        locals.var_x_dn7 = assign47030_e60326_d_n7;
        locals.var_x_dn8 = assign47030_e60326_d_n8;
        locals.var_x_dn9 = assign47030_e60326_d_n9;
        locals.var_x_rv = 0.0;

        let assign47070_e60368: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign47070_e60368;
        locals.var_guard1258_rv = 0.0;

        let assign47120_e60461: f64 = (locals.var_x).abs();
        let assign47120_e60463: f64 = if assign47120_e60461 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign47120_e60463;
        locals.var_guard1259_rv = 0.0;

        let (assign47130_e60478, assign47130_e60478_d_n4, assign47130_e60478_d_n6, assign47130_e60478_d_n7, assign47130_e60478_d_n8, assign47130_e60478_d_n9,) = {
    if (((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 != 0.0)) {
        let assign47130_e60476: f64 = (locals.var_x).exp();
        (assign47130_e60476, (assign47130_e60476 * locals.var_x_dn4), (assign47130_e60476 * locals.var_x_dn6), (assign47130_e60476 * locals.var_x_dn7), (assign47130_e60476 * locals.var_x_dn8), (assign47130_e60476 * locals.var_x_dn9),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47130_e60478;
        locals.var_ex_dn4 = assign47130_e60478_d_n4;
        locals.var_ex_dn6 = assign47130_e60478_d_n6;
        locals.var_ex_dn7 = assign47130_e60478_d_n7;
        locals.var_ex_dn8 = assign47130_e60478_d_n8;
        locals.var_ex_dn9 = assign47130_e60478_d_n9;
        locals.var_ex_rv = 0.0;

        let assign47140_e60481: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign47140_e60481;
        locals.var_guard1260_rv = 0.0;

        let (assign47150_e60523, assign47150_e60523_d_n4, assign47150_e60523_d_n6, assign47150_e60523_d_n7, assign47150_e60523_d_n8, assign47150_e60523_d_n9,) = {
    if ((((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 == 0.0)) && (locals.var_guard1260 != 0.0)) {
        let assign47150_e60499: f64 = (-230.25850929940458);
        let assign47150_e60501: f64 = (assign47150_e60499 - locals.var_x);
        let assign47150_e60505: f64 = (-230.25850929940458);
        let assign47150_e60507: f64 = (assign47150_e60505 - locals.var_x);
        let assign47150_e60510: f64 = (-230.25850929940458);
        let assign47150_e60512: f64 = (assign47150_e60510 - locals.var_x);
        let assign47150_e60514: f64 = (assign47150_e60512 * 0.3333333333333333);
        let assign47150_e60515: f64 = (1.0 + assign47150_e60514);
        let assign47150_e60516: f64 = (assign47150_e60507 * assign47150_e60515);
        let assign47150_e60517: f64 = (0.5 * assign47150_e60516);
        let assign47150_e60518: f64 = (1.0 + assign47150_e60517);
        let assign47150_e60519: f64 = (assign47150_e60501 * assign47150_e60518);
        let assign47150_e60520: f64 = (1.0 + assign47150_e60519);
        let assign47150_e60521: f64 = (1e-100 / assign47150_e60520);
        (assign47150_e60521, (-((1e-100 * (((-locals.var_x_dn4) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn4) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn4) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn6) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn6) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn7) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn7) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn8) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn8) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn9) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn9) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn9) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47150_e60523;
        locals.var_ex_dn4 = assign47150_e60523_d_n4;
        locals.var_ex_dn6 = assign47150_e60523_d_n6;
        locals.var_ex_dn7 = assign47150_e60523_d_n7;
        locals.var_ex_dn8 = assign47150_e60523_d_n8;
        locals.var_ex_dn9 = assign47150_e60523_d_n9;
        locals.var_ex_rv = 0.0;

        let (assign47160_e60563, assign47160_e60563_d_n4, assign47160_e60563_d_n6, assign47160_e60563_d_n7, assign47160_e60563_d_n8, assign47160_e60563_d_n9,) = {
    if ((((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 == 0.0)) && (locals.var_guard1260 == 0.0)) {
        let assign47160_e60543: f64 = (locals.var_x - 230.25850929940458);
        let assign47160_e60548: f64 = (locals.var_x - 230.25850929940458);
        let assign47160_e60552: f64 = (locals.var_x - 230.25850929940458);
        let assign47160_e60554: f64 = (assign47160_e60552 * 0.3333333333333333);
        let assign47160_e60555: f64 = (1.0 + assign47160_e60554);
        let assign47160_e60556: f64 = (assign47160_e60548 * assign47160_e60555);
        let assign47160_e60557: f64 = (0.5 * assign47160_e60556);
        let assign47160_e60558: f64 = (1.0 + assign47160_e60557);
        let assign47160_e60559: f64 = (assign47160_e60543 * assign47160_e60558);
        let assign47160_e60560: f64 = (1.0 + assign47160_e60559);
        let assign47160_e60561: f64 = (1e100 * assign47160_e60560);
        (assign47160_e60561, (1e100 * ((locals.var_x_dn4 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn4 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn6 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn7 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn8 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn9 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn9 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47160_e60563;
        locals.var_ex_dn4 = assign47160_e60563_d_n4;
        locals.var_ex_dn6 = assign47160_e60563_d_n6;
        locals.var_ex_dn7 = assign47160_e60563_d_n7;
        locals.var_ex_dn8 = assign47160_e60563_d_n8;
        locals.var_ex_dn9 = assign47160_e60563_d_n9;
        locals.var_ex_rv = 0.0;

        let (assign47170_e60577, assign47170_e60577_d_n4, assign47170_e60577_d_n6, assign47170_e60577_d_n7, assign47170_e60577_d_n8, assign47170_e60577_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47170_e60575: f64 = (1.0 / locals.var_ex);
        (assign47170_e60575, (-(locals.var_ex_dn4 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn9 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn4, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, locals.var_inv_ex_dn9,)
    }
};
        locals.var_inv_ex = assign47170_e60577;
        locals.var_inv_ex_dn4 = assign47170_e60577_d_n4;
        locals.var_inv_ex_dn6 = assign47170_e60577_d_n6;
        locals.var_inv_ex_dn7 = assign47170_e60577_d_n7;
        locals.var_inv_ex_dn8 = assign47170_e60577_d_n8;
        locals.var_inv_ex_dn9 = assign47170_e60577_d_n9;
        locals.var_inv_ex_rv = 0.0;

        let (assign47180_e60591, assign47180_e60591_d_n4, assign47180_e60591_d_n6, assign47180_e60591_d_n7, assign47180_e60591_d_n8, assign47180_e60591_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47180_e60589: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign47180_e60589, (locals.var_ex_dn4 - locals.var_inv_ex_dn4), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), (locals.var_ex_dn9 - locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47180_e60591;
        locals.var_temp__blk949_dn4 = assign47180_e60591_d_n4;
        locals.var_temp__blk949_dn6 = assign47180_e60591_d_n6;
        locals.var_temp__blk949_dn7 = assign47180_e60591_d_n7;
        locals.var_temp__blk949_dn8 = assign47180_e60591_d_n8;
        locals.var_temp__blk949_dn9 = assign47180_e60591_d_n9;
        locals.var_temp__blk949_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47190_e60605, assign47190_e60605_d_n4, assign47190_e60605_d_n6, assign47190_e60605_d_n7, assign47190_e60605_d_n8, assign47190_e60605_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47190_e60603: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign47190_e60603, (locals.var_ex_dn4 + locals.var_inv_ex_dn4), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), (locals.var_ex_dn9 + locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47190_e60605;
        locals.var_temp2_dn4 = assign47190_e60605_d_n4;
        locals.var_temp2_dn6 = assign47190_e60605_d_n6;
        locals.var_temp2_dn7 = assign47190_e60605_d_n7;
        locals.var_temp2_dn8 = assign47190_e60605_d_n8;
        locals.var_temp2_dn9 = assign47190_e60605_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign47290_e60721: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign47290_e60721;
        locals.var_guard1261_rv = 0.0;

        let assign47300_e60728: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign47300_e60728;
        locals.var_guard1262_rv = 0.0;

        let (assign47310_e60747, assign47310_e60747_d_n6, assign47310_e60747_d_n7, assign47310_e60747_d_n8, assign47310_e60747_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
        let assign47310_e60734: f64 = (locals.var_vovd * locals.var_vovd);
        let assign47310_e60737: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign47310_e60740: f64 = (locals.var_vdbprime * locals.var_vdbprime);
        let assign47310_e60741: f64 = (assign47310_e60737 * assign47310_e60740);
        let assign47310_e60742: f64 = (assign47310_e60734 + assign47310_e60741);
        let assign47310_e60744: f64 = (assign47310_e60742 + 1e-6);
        let assign47310_e60745: f64 = (assign47310_e60744).sqrt();
        (assign47310_e60745, (((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47310_e60737 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) + (assign47310_e60737 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8)))) / (2.0 * assign47310_e60745)), ((assign47310_e60737 * ((locals.var_vdbprime_dn9 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn9))) / (2.0 * assign47310_e60745)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, locals.var_vtovd_dn9,)
    }
};
        locals.var_vtovd = assign47310_e60747;
        locals.var_vtovd_dn6 = assign47310_e60747_d_n6;
        locals.var_vtovd_dn7 = assign47310_e60747_d_n7;
        locals.var_vtovd_dn8 = assign47310_e60747_d_n8;
        locals.var_vtovd_dn9 = assign47310_e60747_d_n9;
        locals.var_vtovd_rv = 0.0;

        let (assign47320_e60756, assign47320_e60756_d_n4, assign47320_e60756_d_n6, assign47320_e60756_d_n7, assign47320_e60756_d_n8, assign47320_e60756_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
        let assign47320_e60752: f64 = (-locals.var_bgidlds);
        let assign47320_e60754: f64 = (assign47320_e60752 / locals.var_vtovd);
        (assign47320_e60754, 0.0, (-((assign47320_e60752 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn9) / (locals.var_vtovd * locals.var_vtovd))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47320_e60756;
        locals.var_temp__blk949_dn4 = assign47320_e60756_d_n4;
        locals.var_temp__blk949_dn6 = assign47320_e60756_d_n6;
        locals.var_temp__blk949_dn7 = assign47320_e60756_d_n7;
        locals.var_temp__blk949_dn8 = assign47320_e60756_d_n8;
        locals.var_temp__blk949_dn9 = assign47320_e60756_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign47330_e60759: f64 = (-230.25850929940458);
        let assign47330_e60760: f64 = if locals.var_temp__blk949 > assign47330_e60759 { 1.0 } else { 0.0 };
        locals.var_guard1263 = assign47330_e60760;
        locals.var_guard1263_rv = 0.0;

        let (assign47340_e60769, assign47340_e60769_d_n4, assign47340_e60769_d_n6, assign47340_e60769_d_n7, assign47340_e60769_d_n8, assign47340_e60769_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 != 0.0)) {
        let assign47340_e60767: f64 = (locals.var_temp__blk949).exp();
        (assign47340_e60767, (assign47340_e60767 * locals.var_temp__blk949_dn4), (assign47340_e60767 * locals.var_temp__blk949_dn6), (assign47340_e60767 * locals.var_temp__blk949_dn7), (assign47340_e60767 * locals.var_temp__blk949_dn8), (assign47340_e60767 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47340_e60769;
        locals.var_temp2_dn4 = assign47340_e60769_d_n4;
        locals.var_temp2_dn6 = assign47340_e60769_d_n6;
        locals.var_temp2_dn7 = assign47340_e60769_d_n7;
        locals.var_temp2_dn8 = assign47340_e60769_d_n8;
        locals.var_temp2_dn9 = assign47340_e60769_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign47350_e60803, assign47350_e60803_d_n4, assign47350_e60803_d_n6, assign47350_e60803_d_n7, assign47350_e60803_d_n8, assign47350_e60803_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 == 0.0)) {
        let assign47350_e60779: f64 = (-230.25850929940458);
        let assign47350_e60781: f64 = (assign47350_e60779 - locals.var_temp__blk949);
        let assign47350_e60785: f64 = (-230.25850929940458);
        let assign47350_e60787: f64 = (assign47350_e60785 - locals.var_temp__blk949);
        let assign47350_e60790: f64 = (-230.25850929940458);
        let assign47350_e60792: f64 = (assign47350_e60790 - locals.var_temp__blk949);
        let assign47350_e60794: f64 = (assign47350_e60792 * 0.3333333333333333);
        let assign47350_e60795: f64 = (1.0 + assign47350_e60794);
        let assign47350_e60796: f64 = (assign47350_e60787 * assign47350_e60795);
        let assign47350_e60797: f64 = (0.5 * assign47350_e60796);
        let assign47350_e60798: f64 = (1.0 + assign47350_e60797);
        let assign47350_e60799: f64 = (assign47350_e60781 * assign47350_e60798);
        let assign47350_e60800: f64 = (1.0 + assign47350_e60799);
        let assign47350_e60801: f64 = (1e-100 / assign47350_e60800);
        (assign47350_e60801, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47350_e60803;
        locals.var_temp2_dn4 = assign47350_e60803_d_n4;
        locals.var_temp2_dn6 = assign47350_e60803_d_n6;
        locals.var_temp2_dn7 = assign47350_e60803_d_n7;
        locals.var_temp2_dn8 = assign47350_e60803_d_n8;
        locals.var_temp2_dn9 = assign47350_e60803_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign47370_e60825: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1264 = assign47370_e60825;
        locals.var_guard1264_rv = 0.0;

        let (assign47380_e60844, assign47380_e60844_d_n6, assign47380_e60844_d_n7, assign47380_e60844_d_n8, assign47380_e60844_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
        let assign47380_e60831: f64 = (locals.var_vovs * locals.var_vovs);
        let assign47380_e60834: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign47380_e60837: f64 = (locals.var_vsbprime * locals.var_vsbprime);
        let assign47380_e60838: f64 = (assign47380_e60834 * assign47380_e60837);
        let assign47380_e60839: f64 = (assign47380_e60831 + assign47380_e60838);
        let assign47380_e60841: f64 = (assign47380_e60839 + 1e-6);
        let assign47380_e60842: f64 = (assign47380_e60841).sqrt();
        (assign47380_e60842, (((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47380_e60834 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) + (assign47380_e60834 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8)))) / (2.0 * assign47380_e60842)), ((assign47380_e60834 * ((locals.var_vsbprime_dn9 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn9))) / (2.0 * assign47380_e60842)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, locals.var_vtovs_dn9,)
    }
};
        locals.var_vtovs = assign47380_e60844;
        locals.var_vtovs_dn6 = assign47380_e60844_d_n6;
        locals.var_vtovs_dn7 = assign47380_e60844_d_n7;
        locals.var_vtovs_dn8 = assign47380_e60844_d_n8;
        locals.var_vtovs_dn9 = assign47380_e60844_d_n9;
        locals.var_vtovs_rv = 0.0;

        let (assign47390_e60853, assign47390_e60853_d_n4, assign47390_e60853_d_n6, assign47390_e60853_d_n7, assign47390_e60853_d_n8, assign47390_e60853_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
        let assign47390_e60849: f64 = (-locals.var_bgidls);
        let assign47390_e60851: f64 = (assign47390_e60849 / locals.var_vtovs);
        (assign47390_e60851, 0.0, (-((assign47390_e60849 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn9) / (locals.var_vtovs * locals.var_vtovs))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47390_e60853;
        locals.var_temp__blk949_dn4 = assign47390_e60853_d_n4;
        locals.var_temp__blk949_dn6 = assign47390_e60853_d_n6;
        locals.var_temp__blk949_dn7 = assign47390_e60853_d_n7;
        locals.var_temp__blk949_dn8 = assign47390_e60853_d_n8;
        locals.var_temp__blk949_dn9 = assign47390_e60853_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign47400_e60856: f64 = (-230.25850929940458);
        let assign47400_e60857: f64 = if locals.var_temp__blk949 > assign47400_e60856 { 1.0 } else { 0.0 };
        locals.var_guard1265 = assign47400_e60857;
        locals.var_guard1265_rv = 0.0;

        let (assign47410_e60866, assign47410_e60866_d_n4, assign47410_e60866_d_n6, assign47410_e60866_d_n7, assign47410_e60866_d_n8, assign47410_e60866_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 != 0.0)) {
        let assign47410_e60864: f64 = (locals.var_temp__blk949).exp();
        (assign47410_e60864, (assign47410_e60864 * locals.var_temp__blk949_dn4), (assign47410_e60864 * locals.var_temp__blk949_dn6), (assign47410_e60864 * locals.var_temp__blk949_dn7), (assign47410_e60864 * locals.var_temp__blk949_dn8), (assign47410_e60864 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47410_e60866;
        locals.var_temp2_dn4 = assign47410_e60866_d_n4;
        locals.var_temp2_dn6 = assign47410_e60866_d_n6;
        locals.var_temp2_dn7 = assign47410_e60866_d_n7;
        locals.var_temp2_dn8 = assign47410_e60866_d_n8;
        locals.var_temp2_dn9 = assign47410_e60866_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign47420_e60900, assign47420_e60900_d_n4, assign47420_e60900_d_n6, assign47420_e60900_d_n7, assign47420_e60900_d_n8, assign47420_e60900_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 == 0.0)) {
        let assign47420_e60876: f64 = (-230.25850929940458);
        let assign47420_e60878: f64 = (assign47420_e60876 - locals.var_temp__blk949);
        let assign47420_e60882: f64 = (-230.25850929940458);
        let assign47420_e60884: f64 = (assign47420_e60882 - locals.var_temp__blk949);
        let assign47420_e60887: f64 = (-230.25850929940458);
        let assign47420_e60889: f64 = (assign47420_e60887 - locals.var_temp__blk949);
        let assign47420_e60891: f64 = (assign47420_e60889 * 0.3333333333333333);
        let assign47420_e60892: f64 = (1.0 + assign47420_e60891);
        let assign47420_e60893: f64 = (assign47420_e60884 * assign47420_e60892);
        let assign47420_e60894: f64 = (0.5 * assign47420_e60893);
        let assign47420_e60895: f64 = (1.0 + assign47420_e60894);
        let assign47420_e60896: f64 = (assign47420_e60878 * assign47420_e60895);
        let assign47420_e60897: f64 = (1.0 + assign47420_e60896);
        let assign47420_e60898: f64 = (1e-100 / assign47420_e60897);
        (assign47420_e60898, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47420_e60900;
        locals.var_temp2_dn4 = assign47420_e60900_d_n4;
        locals.var_temp2_dn6 = assign47420_e60900_d_n6;
        locals.var_temp2_dn7 = assign47420_e60900_d_n7;
        locals.var_temp2_dn8 = assign47420_e60900_d_n8;
        locals.var_temp2_dn9 = assign47420_e60900_d_n9;
        locals.var_temp2_rv = 0.0;

        locals.var_phit1edge = locals.var_phit;
        locals.var_phit1edge_dn4 = locals.var_phit_dn4;
        locals.var_phit1edge_dn6 = 0.0;
        locals.var_phit1edge_dn7 = 0.0;
        locals.var_phit1edge_dn8 = 0.0;
        locals.var_phit1edge_dn9 = 0.0;
        locals.var_phit1edge_rv = 0.0;

        locals.var_xgedge = 0.0;
        locals.var_xgedge_dn4 = 0.0;
        locals.var_xgedge_dn6 = 0.0;
        locals.var_xgedge_dn7 = 0.0;
        locals.var_xgedge_dn8 = 0.0;
        locals.var_xgedge_dn9 = 0.0;
        locals.var_xgedge_rv = 0.0;

        locals.var_qdseffedge = 0.0;
        locals.var_qdseffedge_dn4 = 0.0;
        locals.var_qdseffedge_dn6 = 0.0;
        locals.var_qdseffedge_dn7 = 0.0;
        locals.var_qdseffedge_dn8 = 0.0;
        locals.var_qdseffedge_dn9 = 0.0;
        locals.var_qdseffedge_rv = 0.0;

        locals.var_qmeffedge = 0.0;
        locals.var_qmeffedge_dn4 = 0.0;
        locals.var_qmeffedge_dn6 = 0.0;
        locals.var_qmeffedge_dn7 = 0.0;
        locals.var_qmeffedge_dn8 = 0.0;
        locals.var_qmeffedge_dn9 = 0.0;
        locals.var_qmeffedge_rv = 0.0;

        locals.var_dsqredge = 1e-40;
        locals.var_dsqredge_dn4 = 0.0;
        locals.var_dsqredge_dn6 = 0.0;
        locals.var_dsqredge_dn7 = 0.0;
        locals.var_dsqredge_dn8 = 0.0;
        locals.var_dsqredge_dn9 = 0.0;
        locals.var_dsqredge_rv = 0.0;

        locals.var_alphabmedge = 1.0;
        locals.var_alphabmedge_dn4 = 0.0;
        locals.var_alphabmedge_dn6 = 0.0;
        locals.var_alphabmedge_dn7 = 0.0;
        locals.var_alphabmedge_dn8 = 0.0;
        locals.var_alphabmedge_dn9 = 0.0;
        locals.var_alphabmedge_rv = 0.0;

        locals.var_i_dsedge = 0.0;
        locals.var_i_dsedge_dn4 = 0.0;
        locals.var_i_dsedge_dn6 = 0.0;
        locals.var_i_dsedge_dn7 = 0.0;
        locals.var_i_dsedge_dn8 = 0.0;
        locals.var_i_dsedge_dn9 = 0.0;
        locals.var_i_dsedge_rv = 0.0;

        let assign47510_e60929: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1266 = assign47510_e60929;
        locals.var_guard1266_rv = 0.0;

        let (assign47520_e60950, assign47520_e60950_d_n4, assign47520_e60950_d_n6, assign47520_e60950_d_n7, assign47520_e60950_d_n8, assign47520_e60950_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47520_e60934: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign47520_e60937: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47520_e60940: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47520_e60941: f64 = (assign47520_e60937 * assign47520_e60940);
        let assign47520_e60943: f64 = (assign47520_e60941 + locals.var_bphiedge);
        let assign47520_e60944: f64 = (assign47520_e60943).sqrt();
        let assign47520_e60945: f64 = (assign47520_e60934 - assign47520_e60944);
        let assign47520_e60946: f64 = (0.5 * assign47520_e60945);
        let assign47520_e60948: f64 = (assign47520_e60946 + locals.var_phixedge);
        (assign47520_e60948, ((0.5 * (-(locals.var_bphiedge_dn4 / (2.0 * assign47520_e60944)))) + locals.var_phixedge_dn4), 0.0, (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign47520_e60944)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47520_e60950;
        locals.var_temp__blk949_dn4 = assign47520_e60950_d_n4;
        locals.var_temp__blk949_dn6 = assign47520_e60950_d_n6;
        locals.var_temp__blk949_dn7 = assign47520_e60950_d_n7;
        locals.var_temp__blk949_dn8 = assign47520_e60950_d_n8;
        locals.var_temp__blk949_dn9 = assign47520_e60950_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign47530_e60973, assign47530_e60973_d_n4, assign47530_e60973_d_n6, assign47530_e60973_d_n7, assign47530_e60973_d_n8, assign47530_e60973_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47530_e60956: f64 = locals.var_temp__blk949;
        let assign47530_e60959: f64 = locals.var_temp__blk949;
        let assign47530_e60962: f64 = locals.var_temp__blk949;
        let assign47530_e60963: f64 = (assign47530_e60959 * assign47530_e60962);
        let assign47530_e60965: f64 = (assign47530_e60963 + locals.var_aphiedge);
        let assign47530_e60966: f64 = (assign47530_e60965).sqrt();
        let assign47530_e60967: f64 = (assign47530_e60956 - assign47530_e60966);
        let assign47530_e60968: f64 = (0.5 * assign47530_e60967);
        let assign47530_e60969: f64 = (locals.var_v_sb - assign47530_e60968);
        let assign47530_e60971: f64 = (assign47530_e60969 + locals.var_phix1edge);
        (assign47530_e60971, ((-(0.5 * (locals.var_temp__blk949_dn4 - ((((locals.var_temp__blk949_dn4 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn4)) + locals.var_aphiedge_dn4) / (2.0 * assign47530_e60966))))) + locals.var_phix1edge_dn4), (-(0.5 * (locals.var_temp__blk949_dn6 - (((locals.var_temp__blk949_dn6 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn6)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk949_dn7 - (((locals.var_temp__blk949_dn7 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn7)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk949_dn8 - (((locals.var_temp__blk949_dn8 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn8)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_temp__blk949_dn9 - (((locals.var_temp__blk949_dn9 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn9)) / (2.0 * assign47530_e60966))))),)
    } else {
        (locals.var_vsbstaredge, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8, locals.var_vsbstaredge_dn9,)
    }
};
        locals.var_vsbstaredge = assign47530_e60973;
        locals.var_vsbstaredge_dn4 = assign47530_e60973_d_n4;
        locals.var_vsbstaredge_dn6 = assign47530_e60973_d_n6;
        locals.var_vsbstaredge_dn7 = assign47530_e60973_d_n7;
        locals.var_vsbstaredge_dn8 = assign47530_e60973_d_n8;
        locals.var_vsbstaredge_dn9 = assign47530_e60973_d_n9;
        locals.var_vsbstaredge_rv = 0.0;

        let (assign47540_e60983, assign47540_e60983_d_n4, assign47540_e60983_d_n6, assign47540_e60983_d_n7, assign47540_e60983_d_n8, assign47540_e60983_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47540_e60979: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign47540_e60980: f64 = (0.5 * assign47540_e60979);
        let assign47540_e60981: f64 = (locals.var_vsbstaredge + assign47540_e60980);
        (assign47540_e60981, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstaredge_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstaredge_dn9,)
    } else {
        (locals.var_vsbxedge, locals.var_vsbxedge_dn4, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8, locals.var_vsbxedge_dn9,)
    }
};
        locals.var_vsbxedge = assign47540_e60983;
        locals.var_vsbxedge_dn4 = assign47540_e60983_d_n4;
        locals.var_vsbxedge_dn6 = assign47540_e60983_d_n6;
        locals.var_vsbxedge_dn7 = assign47540_e60983_d_n7;
        locals.var_vsbxedge_dn8 = assign47540_e60983_d_n8;
        locals.var_vsbxedge_dn9 = assign47540_e60983_d_n9;
        locals.var_vsbxedge_rv = 0.0;

        let (assign47550_e60999, assign47550_e60999_d_n4, assign47550_e60999_d_n6, assign47550_e60999_d_n7, assign47550_e60999_d_n8, assign47550_e60999_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47550_e60989: f64 = (locals.var_pscededge_i * locals.var_vdsx);
        let assign47550_e60990: f64 = (1.0 + assign47550_e60989);
        let assign47550_e60991: f64 = (locals.var_psceedge_i * assign47550_e60990);
        let assign47550_e60995: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
        let assign47550_e60996: f64 = (1.0 + assign47550_e60995);
        let assign47550_e60997: f64 = (assign47550_e60991 * assign47550_e60996);
        (assign47550_e60997, (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn4)), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn8)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8))), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn9)),)
    } else {
        (locals.var_dphit1edge, locals.var_dphit1edge_dn4, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8, locals.var_dphit1edge_dn9,)
    }
};
        locals.var_dphit1edge = assign47550_e60999;
        locals.var_dphit1edge_dn4 = assign47550_e60999_d_n4;
        locals.var_dphit1edge_dn6 = assign47550_e60999_d_n6;
        locals.var_dphit1edge_dn7 = assign47550_e60999_d_n7;
        locals.var_dphit1edge_dn8 = assign47550_e60999_d_n8;
        locals.var_dphit1edge_dn9 = assign47550_e60999_d_n9;
        locals.var_dphit1edge_rv = 0.0;

        let (assign47560_e61007, assign47560_e61007_d_n4, assign47560_e61007_d_n6, assign47560_e61007_d_n7, assign47560_e61007_d_n8, assign47560_e61007_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47560_e61004: f64 = (1.0 + locals.var_dphit1edge);
        let assign47560_e61005: f64 = (locals.var_phit0edge * assign47560_e61004);
        (assign47560_e61005, ((locals.var_phit0edge_dn4 * assign47560_e61004) + (locals.var_phit0edge * locals.var_dphit1edge_dn4)), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8), (locals.var_phit0edge * locals.var_dphit1edge_dn9),)
    } else {
        (locals.var_phit1edge, locals.var_phit1edge_dn4, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, locals.var_phit1edge_dn9,)
    }
};
        locals.var_phit1edge = assign47560_e61007;
        locals.var_phit1edge_dn4 = assign47560_e61007_d_n4;
        locals.var_phit1edge_dn6 = assign47560_e61007_d_n6;
        locals.var_phit1edge_dn7 = assign47560_e61007_d_n7;
        locals.var_phit1edge_dn8 = assign47560_e61007_d_n8;
        locals.var_phit1edge_dn9 = assign47560_e61007_d_n9;
        locals.var_phit1edge_rv = 0.0;

        let (assign47570_e61013, assign47570_e61013_d_n4, assign47570_e61013_d_n6, assign47570_e61013_d_n7, assign47570_e61013_d_n8, assign47570_e61013_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47570_e61011: f64 = (1.0 / locals.var_phit1edge);
        (assign47570_e61011, (-(locals.var_phit1edge_dn4 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn9 / (locals.var_phit1edge * locals.var_phit1edge))),)
    } else {
        (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn4, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8, locals.var_inv_phit1edge_dn9,)
    }
};
        locals.var_inv_phit1edge = assign47570_e61013;
        locals.var_inv_phit1edge_dn4 = assign47570_e61013_d_n4;
        locals.var_inv_phit1edge_dn6 = assign47570_e61013_d_n6;
        locals.var_inv_phit1edge_dn7 = assign47570_e61013_d_n7;
        locals.var_inv_phit1edge_dn8 = assign47570_e61013_d_n8;
        locals.var_inv_phit1edge_dn9 = assign47570_e61013_d_n9;
        locals.var_inv_phit1edge_rv = 0.0;

        let (assign47580_e61028, assign47580_e61028_d_n7, assign47580_e61028_d_n8,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47580_e61017: f64 = (2.0 * locals.var_vdsx);
        let assign47580_e61022: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
        let assign47580_e61023: f64 = (1.0 + assign47580_e61022);
        let assign47580_e61024: f64 = (assign47580_e61023).sqrt();
        let assign47580_e61025: f64 = (1.0 + assign47580_e61024);
        let assign47580_e61026: f64 = (assign47580_e61017 / assign47580_e61025);
        (assign47580_e61026, ((((2.0 * locals.var_vdsx_dn7) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)), ((((2.0 * locals.var_vdsx_dn8) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn8) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)),)
    } else {
        (locals.var_vdspedge, locals.var_vdspedge_dn7, locals.var_vdspedge_dn8,)
    }
};
        locals.var_vdspedge = assign47580_e61028;
        locals.var_vdspedge_dn7 = assign47580_e61028_d_n7;
        locals.var_vdspedge_dn8 = assign47580_e61028_d_n8;
        locals.var_vdspedge_rv = 0.0;

        let (assign47590_e61040, assign47590_e61040_d_n4, assign47590_e61040_d_n6, assign47590_e61040_d_n7, assign47590_e61040_d_n8, assign47590_e61040_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47590_e61032: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
        let assign47590_e61036: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
        let assign47590_e61037: f64 = (1.0 + assign47590_e61036);
        let assign47590_e61038: f64 = (assign47590_e61032 * assign47590_e61037);
        (assign47590_e61038, (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn4)), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6)), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (((locals.var_cfedge_i * locals.var_vdspedge_dn8) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8))), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn9)),)
    } else {
        (locals.var_delvgedge, locals.var_delvgedge_dn4, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8, locals.var_delvgedge_dn9,)
    }
};
        locals.var_delvgedge = assign47590_e61040;
        locals.var_delvgedge_dn4 = assign47590_e61040_d_n4;
        locals.var_delvgedge_dn6 = assign47590_e61040_d_n6;
        locals.var_delvgedge_dn7 = assign47590_e61040_d_n7;
        locals.var_delvgedge_dn8 = assign47590_e61040_d_n8;
        locals.var_delvgedge_dn9 = assign47590_e61040_d_n9;
        locals.var_delvgedge_rv = 0.0;

        let (assign47600_e61050, assign47600_e61050_d_n4, assign47600_e61050_d_n6, assign47600_e61050_d_n7, assign47600_e61050_d_n8, assign47600_e61050_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47600_e61045: f64 = (locals.var_vgb + locals.var_delvgedge);
        let assign47600_e61047: f64 = (assign47600_e61045 - locals.var_vfbedge_t);
        let assign47600_e61048: f64 = (locals.var_inv_phit1edge * assign47600_e61047);
        (assign47600_e61048, ((locals.var_inv_phit1edge_dn4 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_delvgedge_dn4 - locals.var_vfbedge_t_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn9 + locals.var_delvgedge_dn9))),)
    } else {
        (locals.var_xgedge, locals.var_xgedge_dn4, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, locals.var_xgedge_dn9,)
    }
};
        locals.var_xgedge = assign47600_e61050;
        locals.var_xgedge_dn4 = assign47600_e61050_d_n4;
        locals.var_xgedge_dn6 = assign47600_e61050_d_n6;
        locals.var_xgedge_dn7 = assign47600_e61050_d_n7;
        locals.var_xgedge_dn8 = assign47600_e61050_d_n8;
        locals.var_xgedge_dn9 = assign47600_e61050_d_n9;
        locals.var_xgedge_rv = 0.0;

        let (assign47610_e61056, assign47610_e61056_d_n4, assign47610_e61056_d_n6, assign47610_e61056_d_n7, assign47610_e61056_d_n8, assign47610_e61056_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47610_e61054: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
        (assign47610_e61054, ((locals.var_inv_phit1edge_dn4 * locals.var_phibedge) + (locals.var_inv_phit1edge * locals.var_phibedge_dn4)), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge), (locals.var_inv_phit1edge_dn9 * locals.var_phibedge),)
    } else {
        (locals.var_xbedge, locals.var_xbedge_dn4, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8, locals.var_xbedge_dn9,)
    }
};
        locals.var_xbedge = assign47610_e61056;
        locals.var_xbedge_dn4 = assign47610_e61056_d_n4;
        locals.var_xbedge_dn6 = assign47610_e61056_d_n6;
        locals.var_xbedge_dn7 = assign47610_e61056_d_n7;
        locals.var_xbedge_dn8 = assign47610_e61056_d_n8;
        locals.var_xbedge_dn9 = assign47610_e61056_d_n9;
        locals.var_xbedge_rv = 0.0;

        let (assign47620_e61068, assign47620_e61068_d_n4, assign47620_e61068_d_n6, assign47620_e61068_d_n7, assign47620_e61068_d_n8, assign47620_e61068_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47620_e61061: f64 = (locals.var_xbedge / locals.var_gfedge);
        let assign47620_e61063: f64 = (locals.var_xbedge).sqrt();
        let assign47620_e61064: f64 = (assign47620_e61061 + assign47620_e61063);
        let assign47620_e61065: f64 = (assign47620_e61064).ln();
        let assign47620_e61066: f64 = (2.0 * assign47620_e61065);
        (assign47620_e61066, (2.0 * (((((locals.var_xbedge_dn4 * locals.var_gfedge) - (locals.var_xbedge * locals.var_gfedge_dn4)) / (locals.var_gfedge * locals.var_gfedge)) + (locals.var_xbedge_dn4 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn9 / locals.var_gfedge) + (locals.var_xbedge_dn9 / (2.0 * assign47620_e61063))) / assign47620_e61064)),)
    } else {
        (locals.var_dxthedge, locals.var_dxthedge_dn4, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8, locals.var_dxthedge_dn9,)
    }
};
        locals.var_dxthedge = assign47620_e61068;
        locals.var_dxthedge_dn4 = assign47620_e61068_d_n4;
        locals.var_dxthedge_dn6 = assign47620_e61068_d_n6;
        locals.var_dxthedge_dn7 = assign47620_e61068_d_n7;
        locals.var_dxthedge_dn8 = assign47620_e61068_d_n8;
        locals.var_dxthedge_dn9 = assign47620_e61068_d_n9;
        locals.var_dxthedge_rv = 0.0;

        let (assign47630_e61074, assign47630_e61074_d_n4, assign47630_e61074_d_n6, assign47630_e61074_d_n7, assign47630_e61074_d_n8, assign47630_e61074_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47630_e61072: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
        (assign47630_e61072, ((locals.var_inv_phit1edge_dn4 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn4)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)), ((locals.var_inv_phit1edge_dn9 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn9)),)
    } else {
        (locals.var_xnedge_s, locals.var_xnedge_s_dn4, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8, locals.var_xnedge_s_dn9,)
    }
};
        locals.var_xnedge_s = assign47630_e61074;
        locals.var_xnedge_s_dn4 = assign47630_e61074_d_n4;
        locals.var_xnedge_s_dn6 = assign47630_e61074_d_n6;
        locals.var_xnedge_s_dn7 = assign47630_e61074_d_n7;
        locals.var_xnedge_s_dn8 = assign47630_e61074_d_n8;
        locals.var_xnedge_s_dn9 = assign47630_e61074_d_n9;
        locals.var_xnedge_s_rv = 0.0;

        let (assign47640_e61080, assign47640_e61080_d_n4, assign47640_e61080_d_n6, assign47640_e61080_d_n7, assign47640_e61080_d_n8, assign47640_e61080_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47640_e61078: f64 = (locals.var_xbedge + locals.var_xnedge_s);
        (assign47640_e61078, (locals.var_xbedge_dn4 + locals.var_xnedge_s_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_s_dn9),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9,)
    }
};
        locals.var_q_edge_xsth = assign47640_e61080;
        locals.var_q_edge_xsth_dn4 = assign47640_e61080_d_n4;
        locals.var_q_edge_xsth_dn6 = assign47640_e61080_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47640_e61080_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47640_e61080_d_n8;
        locals.var_q_edge_xsth_dn9 = assign47640_e61080_d_n9;
        locals.var_q_edge_xsth_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        locals: &mut StampLocals,
    ) {
        let (assign47650_e61089, assign47650_e61089_d_n4, assign47650_e61089_d_n6, assign47650_e61089_d_n7, assign47650_e61089_d_n8, assign47650_e61089_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47650_e61085: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47650_e61086: f64 = (locals.var_gfedge * assign47650_e61085);
        let assign47650_e61087: f64 = (locals.var_q_edge_xsth + assign47650_e61086);
        (assign47650_e61087, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47650_e61085) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47650_e61085))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47650_e61085)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9,)
    }
};
        locals.var_q_edge_xth0 = assign47650_e61089;
        locals.var_q_edge_xth0_dn4 = assign47650_e61089_d_n4;
        locals.var_q_edge_xth0_dn6 = assign47650_e61089_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47650_e61089_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47650_e61089_d_n8;
        locals.var_q_edge_xth0_dn9 = assign47650_e61089_d_n9;
        locals.var_q_edge_xth0_rv = 0.0;

        let (assign47660_e61095, assign47660_e61095_d_n4, assign47660_e61095_d_n6, assign47660_e61095_d_n7, assign47660_e61095_d_n8, assign47660_e61095_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47660_e61093: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47660_e61093, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9,)
    }
};
        locals.var_q_edge_xth = assign47660_e61095;
        locals.var_q_edge_xth_dn4 = assign47660_e61095_d_n4;
        locals.var_q_edge_xth_dn6 = assign47660_e61095_d_n6;
        locals.var_q_edge_xth_dn7 = assign47660_e61095_d_n7;
        locals.var_q_edge_xth_dn8 = assign47660_e61095_d_n8;
        locals.var_q_edge_xth_dn9 = assign47660_e61095_d_n9;
        locals.var_q_edge_xth_rv = 0.0;

        let (assign47670_e61106, assign47670_e61106_d_n4, assign47670_e61106_d_n6, assign47670_e61106_d_n7, assign47670_e61106_d_n8, assign47670_e61106_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47670_e61101: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47670_e61102: f64 = (2.0 * assign47670_e61101);
        let assign47670_e61103: f64 = (locals.var_gfedge / assign47670_e61102);
        let assign47670_e61104: f64 = (1.0 + assign47670_e61103);
        (assign47670_e61104, (((locals.var_gfedge_dn4 * assign47670_e61102) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47670_e61101))))) / (assign47670_e61102 * assign47670_e61102)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9,)
    }
};
        locals.var_q_edge_n = assign47670_e61106;
        locals.var_q_edge_n_dn4 = assign47670_e61106_d_n4;
        locals.var_q_edge_n_dn6 = assign47670_e61106_d_n6;
        locals.var_q_edge_n_dn7 = assign47670_e61106_d_n7;
        locals.var_q_edge_n_dn8 = assign47670_e61106_d_n8;
        locals.var_q_edge_n_dn9 = assign47670_e61106_d_n9;
        locals.var_q_edge_n_rv = 0.0;

        let (assign47680_e61112, assign47680_e61112_d_n4, assign47680_e61112_d_n6, assign47680_e61112_d_n7, assign47680_e61112_d_n8, assign47680_e61112_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47680_e61110: f64 = (1.0 / locals.var_q_edge_n);
        (assign47680_e61110, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9,)
    }
};
        locals.var_q_edge_n_inv = assign47680_e61112;
        locals.var_q_edge_n_inv_dn4 = assign47680_e61112_d_n4;
        locals.var_q_edge_n_inv_dn6 = assign47680_e61112_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47680_e61112_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47680_e61112_d_n8;
        locals.var_q_edge_n_inv_dn9 = assign47680_e61112_d_n9;
        locals.var_q_edge_n_inv_rv = 0.0;

        let (assign47690_e61118, assign47690_e61118_d_n4, assign47690_e61118_d_n6, assign47690_e61118_d_n7, assign47690_e61118_d_n8, assign47690_e61118_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47690_e61116: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47690_e61116, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    }
};
        locals.var_q_edge_xgt = assign47690_e61118;
        locals.var_q_edge_xgt_dn4 = assign47690_e61118_d_n4;
        locals.var_q_edge_xgt_dn6 = assign47690_e61118_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47690_e61118_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47690_e61118_d_n8;
        locals.var_q_edge_xgt_dn9 = assign47690_e61118_d_n9;
        locals.var_q_edge_xgt_rv = 0.0;

        let assign47700_e61121: f64 = (-12.0);
        let assign47700_e61122: f64 = if locals.var_q_edge_xgt > assign47700_e61121 { 1.0 } else { 0.0 };
        locals.var_guard1267 = assign47700_e61122;
        locals.var_guard1267_rv = 0.0;

        let (assign47710_e61132, assign47710_e61132_d_n4, assign47710_e61132_d_n6, assign47710_e61132_d_n7, assign47710_e61132_d_n8, assign47710_e61132_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47710_e61128: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47710_e61130: f64 = (assign47710_e61128 - 1.0);
        (assign47710_e61130, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9,)
    }
};
        locals.var_q_edge_xgt0 = assign47710_e61132;
        locals.var_q_edge_xgt0_dn4 = assign47710_e61132_d_n4;
        locals.var_q_edge_xgt0_dn6 = assign47710_e61132_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47710_e61132_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47710_e61132_d_n8;
        locals.var_q_edge_xgt0_dn9 = assign47710_e61132_d_n9;
        locals.var_q_edge_xgt0_rv = 0.0;

        let (assign47720_e61147, assign47720_e61147_d_n4, assign47720_e61147_d_n6, assign47720_e61147_d_n7, assign47720_e61147_d_n8, assign47720_e61147_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47720_e61140: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47720_e61142: f64 = (assign47720_e61140 + 10.0);
        let assign47720_e61143: f64 = (assign47720_e61142).sqrt();
        let assign47720_e61144: f64 = (locals.var_q_edge_xgt0 + assign47720_e61143);
        let assign47720_e61145: f64 = (0.5 * assign47720_e61144);
        (assign47720_e61145, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign47720_e61143)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9,)
    }
};
        locals.var_q_edge_xgt0e = assign47720_e61147;
        locals.var_q_edge_xgt0e_dn4 = assign47720_e61147_d_n4;
        locals.var_q_edge_xgt0e_dn6 = assign47720_e61147_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47720_e61147_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47720_e61147_d_n8;
        locals.var_q_edge_xgt0e_dn9 = assign47720_e61147_d_n9;
        locals.var_q_edge_xgt0e_rv = 0.0;

        let (assign47730_e61160, assign47730_e61160_d_n4, assign47730_e61160_d_n6, assign47730_e61160_d_n7, assign47730_e61160_d_n8, assign47730_e61160_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47730_e61154: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47730_e61155: f64 = (locals.var_q_edge_n * assign47730_e61154);
        let assign47730_e61156: f64 = (locals.var_q_edge_xgt - assign47730_e61155);
        let assign47730_e61158: f64 = (assign47730_e61156 + locals.var_lngfedge2);
        (assign47730_e61158, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9,)
    }
};
        locals.var_q_edge_qi0si = assign47730_e61160;
        locals.var_q_edge_qi0si_dn4 = assign47730_e61160_d_n4;
        locals.var_q_edge_qi0si_dn6 = assign47730_e61160_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47730_e61160_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47730_e61160_d_n8;
        locals.var_q_edge_qi0si_dn9 = assign47730_e61160_d_n9;
        locals.var_q_edge_qi0si_rv = 0.0;

        let (assign47740_e61175, assign47740_e61175_d_n4, assign47740_e61175_d_n6, assign47740_e61175_d_n7, assign47740_e61175_d_n8, assign47740_e61175_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47740_e61168: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47740_e61170: f64 = (assign47740_e61168 + 2.0);
        let assign47740_e61171: f64 = (assign47740_e61170).sqrt();
        let assign47740_e61172: f64 = (locals.var_q_edge_qi0si + assign47740_e61171);
        let assign47740_e61173: f64 = (0.5 * assign47740_e61172);
        (assign47740_e61173, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign47740_e61171)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9,)
    }
};
        locals.var_q_edge_qi0 = assign47740_e61175;
        locals.var_q_edge_qi0_dn4 = assign47740_e61175_d_n4;
        locals.var_q_edge_qi0_dn6 = assign47740_e61175_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47740_e61175_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47740_e61175_d_n8;
        locals.var_q_edge_qi0_dn9 = assign47740_e61175_d_n9;
        locals.var_q_edge_qi0_rv = 0.0;

        let assign47750_e61178: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47750_e61180: f64 = if assign47750_e61178 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1268 = assign47750_e61180;
        locals.var_guard1268_rv = 0.0;

        let (assign47760_e61191, assign47760_e61191_d_n4, assign47760_e61191_d_n6, assign47760_e61191_d_n7, assign47760_e61191_d_n8, assign47760_e61191_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 != 0.0)) {
        let assign47760_e61188: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47760_e61189: f64 = (assign47760_e61188).exp();
        (assign47760_e61189, (assign47760_e61189 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign47760_e61191;
        locals.var_q_edge_exp_x_dn4 = assign47760_e61191_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign47760_e61191_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47760_e61191_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47760_e61191_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign47760_e61191_d_n9;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign47770_e61228, assign47770_e61228_d_n4, assign47770_e61228_d_n6, assign47770_e61228_d_n7, assign47770_e61228_d_n8, assign47770_e61228_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 == 0.0)) {
        let assign47770_e61202: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61204: f64 = (assign47770_e61202 - 230.25850929940458);
        let assign47770_e61209: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61211: f64 = (assign47770_e61209 - 230.25850929940458);
        let assign47770_e61215: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61217: f64 = (assign47770_e61215 - 230.25850929940458);
        let assign47770_e61219: f64 = (assign47770_e61217 * 0.3333333333333333);
        let assign47770_e61220: f64 = (1.0 + assign47770_e61219);
        let assign47770_e61221: f64 = (assign47770_e61211 * assign47770_e61220);
        let assign47770_e61222: f64 = (0.5 * assign47770_e61221);
        let assign47770_e61223: f64 = (1.0 + assign47770_e61222);
        let assign47770_e61224: f64 = (assign47770_e61204 * assign47770_e61223);
        let assign47770_e61225: f64 = (1.0 + assign47770_e61224);
        let assign47770_e61226: f64 = (1e100 * assign47770_e61225);
        (assign47770_e61226, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign47770_e61228;
        locals.var_q_edge_exp_x_dn4 = assign47770_e61228_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign47770_e61228_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47770_e61228_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47770_e61228_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign47770_e61228_d_n9;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign47780_e61236, assign47780_e61236_d_n4, assign47780_e61236_d_n6, assign47780_e61236_d_n7, assign47780_e61236_d_n8, assign47780_e61236_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47780_e61234: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47780_e61234, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9,)
    }
};
        locals.var_q_edge_d0 = assign47780_e61236;
        locals.var_q_edge_d0_dn4 = assign47780_e61236_d_n4;
        locals.var_q_edge_d0_dn6 = assign47780_e61236_d_n6;
        locals.var_q_edge_d0_dn7 = assign47780_e61236_d_n7;
        locals.var_q_edge_d0_dn8 = assign47780_e61236_d_n8;
        locals.var_q_edge_d0_dn9 = assign47780_e61236_d_n9;
        locals.var_q_edge_d0_rv = 0.0;

        let (assign47790_e61244, assign47790_e61244_d_n4, assign47790_e61244_d_n6, assign47790_e61244_d_n7, assign47790_e61244_d_n8, assign47790_e61244_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47790_e61242: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47790_e61242, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9,)
    }
};
        locals.var_q_edge_d0p = assign47790_e61244;
        locals.var_q_edge_d0p_dn4 = assign47790_e61244_d_n4;
        locals.var_q_edge_d0p_dn6 = assign47790_e61244_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47790_e61244_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47790_e61244_d_n8;
        locals.var_q_edge_d0p_dn9 = assign47790_e61244_d_n9;
        locals.var_q_edge_d0p_rv = 0.0;

        let (assign47800_e61262, assign47800_e61262_d_n4, assign47800_e61262_d_n6, assign47800_e61262_d_n7, assign47800_e61262_d_n8, assign47800_e61262_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47800_e61250: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47800_e61254: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47800_e61255: f64 = (2.0 * assign47800_e61254);
        let assign47800_e61257: f64 = (assign47800_e61255 - locals.var_q_edge_d0p);
        let assign47800_e61259: f64 = (assign47800_e61257 * locals.var_q_edge_d0p);
        let assign47800_e61260: f64 = (assign47800_e61250 + assign47800_e61259);
        (assign47800_e61260, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn9))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9,)
    }
};
        locals.var_q_edge_sqerr = assign47800_e61262;
        locals.var_q_edge_sqerr_dn4 = assign47800_e61262_d_n4;
        locals.var_q_edge_sqerr_dn6 = assign47800_e61262_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47800_e61262_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47800_e61262_d_n8;
        locals.var_q_edge_sqerr_dn9 = assign47800_e61262_d_n9;
        locals.var_q_edge_sqerr_rv = 0.0;

        let (assign47810_e61277, assign47810_e61277_d_n4, assign47810_e61277_d_n6, assign47810_e61277_d_n7, assign47810_e61277_d_n8, assign47810_e61277_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47810_e61268: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47810_e61270: f64 = (assign47810_e61268 - locals.var_q_edge_n);
        let assign47810_e61272: f64 = (assign47810_e61270 / locals.var_q_edge_d0p);
        let assign47810_e61274: f64 = (assign47810_e61272 - 1.0);
        let assign47810_e61275: f64 = (locals.var_q_edge_n * assign47810_e61274);
        (assign47810_e61275, ((locals.var_q_edge_n_dn4 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9,)
    }
};
        locals.var_q_edge_errq = assign47810_e61277;
        locals.var_q_edge_errq_dn4 = assign47810_e61277_d_n4;
        locals.var_q_edge_errq_dn6 = assign47810_e61277_d_n6;
        locals.var_q_edge_errq_dn7 = assign47810_e61277_d_n7;
        locals.var_q_edge_errq_dn8 = assign47810_e61277_d_n8;
        locals.var_q_edge_errq_dn9 = assign47810_e61277_d_n9;
        locals.var_q_edge_errq_rv = 0.0;

        let (assign47820_e61285, assign47820_e61285_d_n4, assign47820_e61285_d_n6, assign47820_e61285_d_n7, assign47820_e61285_d_n8, assign47820_e61285_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47820_e61283: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47820_e61283, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47820_e61285;
        locals.var_qseffedge_dn4 = assign47820_e61285_d_n4;
        locals.var_qseffedge_dn6 = assign47820_e61285_d_n6;
        locals.var_qseffedge_dn7 = assign47820_e61285_d_n7;
        locals.var_qseffedge_dn8 = assign47820_e61285_d_n8;
        locals.var_qseffedge_dn9 = assign47820_e61285_d_n9;
        locals.var_qseffedge_rv = 0.0;

        let assign47830_e61289: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47830_e61290: f64 = (locals.var_q_edge_n_inv * assign47830_e61289);
        let assign47830_e61292: f64 = (-230.25850929940458);
        let assign47830_e61293: f64 = if assign47830_e61290 > assign47830_e61292 { 1.0 } else { 0.0 };
        locals.var_guard1269 = assign47830_e61293;
        locals.var_guard1269_rv = 0.0;

        let (assign47840_e61307, assign47840_e61307_d_n4, assign47840_e61307_d_n6, assign47840_e61307_d_n7, assign47840_e61307_d_n8, assign47840_e61307_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 != 0.0)) {
        let assign47840_e61303: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47840_e61304: f64 = (locals.var_q_edge_n_inv * assign47840_e61303);
        let assign47840_e61305: f64 = (assign47840_e61304).exp();
        (assign47840_e61305, (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn4 * assign47840_e61303) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn6 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn7 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn8 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn9 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47840_e61307;
        locals.var_qseffedge_dn4 = assign47840_e61307_d_n4;
        locals.var_qseffedge_dn6 = assign47840_e61307_d_n6;
        locals.var_qseffedge_dn7 = assign47840_e61307_d_n7;
        locals.var_qseffedge_dn8 = assign47840_e61307_d_n8;
        locals.var_qseffedge_dn9 = assign47840_e61307_d_n9;
        locals.var_qseffedge_rv = 0.0;

        let (assign47850_e61354, assign47850_e61354_d_n4, assign47850_e61354_d_n6, assign47850_e61354_d_n7, assign47850_e61354_d_n8, assign47850_e61354_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 == 0.0)) {
        let assign47850_e61318: f64 = (-230.25850929940458);
        let assign47850_e61322: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61323: f64 = (locals.var_q_edge_n_inv * assign47850_e61322);
        let assign47850_e61324: f64 = (assign47850_e61318 - assign47850_e61323);
        let assign47850_e61328: f64 = (-230.25850929940458);
        let assign47850_e61332: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61333: f64 = (locals.var_q_edge_n_inv * assign47850_e61332);
        let assign47850_e61334: f64 = (assign47850_e61328 - assign47850_e61333);
        let assign47850_e61337: f64 = (-230.25850929940458);
        let assign47850_e61341: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61342: f64 = (locals.var_q_edge_n_inv * assign47850_e61341);
        let assign47850_e61343: f64 = (assign47850_e61337 - assign47850_e61342);
        let assign47850_e61345: f64 = (assign47850_e61343 * 0.3333333333333333);
        let assign47850_e61346: f64 = (1.0 + assign47850_e61345);
        let assign47850_e61347: f64 = (assign47850_e61334 * assign47850_e61346);
        let assign47850_e61348: f64 = (0.5 * assign47850_e61347);
        let assign47850_e61349: f64 = (1.0 + assign47850_e61348);
        let assign47850_e61350: f64 = (assign47850_e61324 * assign47850_e61349);
        let assign47850_e61351: f64 = (1.0 + assign47850_e61350);
        let assign47850_e61352: f64 = (1e-100 / assign47850_e61351);
        (assign47850_e61352, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61322) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61332) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61341) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47850_e61354;
        locals.var_qseffedge_dn4 = assign47850_e61354_d_n4;
        locals.var_qseffedge_dn6 = assign47850_e61354_d_n6;
        locals.var_qseffedge_dn7 = assign47850_e61354_d_n7;
        locals.var_qseffedge_dn8 = assign47850_e61354_d_n8;
        locals.var_qseffedge_dn9 = assign47850_e61354_d_n9;
        locals.var_qseffedge_rv = 0.0;

        let (assign47860_e61362, assign47860_e61362_d_n4, assign47860_e61362_d_n6, assign47860_e61362_d_n7, assign47860_e61362_d_n8, assign47860_e61362_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47860_e61359: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
        let assign47860_e61360: f64 = (locals.var_inv_phit1edge * assign47860_e61359);
        (assign47860_e61360, ((locals.var_inv_phit1edge_dn4 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn4 + locals.var_vsbstaredge_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn9 + locals.var_vsbstaredge_dn9))),)
    } else {
        (locals.var_xnedge_d, locals.var_xnedge_d_dn4, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8, locals.var_xnedge_d_dn9,)
    }
};
        locals.var_xnedge_d = assign47860_e61362;
        locals.var_xnedge_d_dn4 = assign47860_e61362_d_n4;
        locals.var_xnedge_d_dn6 = assign47860_e61362_d_n6;
        locals.var_xnedge_d_dn7 = assign47860_e61362_d_n7;
        locals.var_xnedge_d_dn8 = assign47860_e61362_d_n8;
        locals.var_xnedge_d_dn9 = assign47860_e61362_d_n9;
        locals.var_xnedge_d_rv = 0.0;

        let assign47870_e61369: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1270 = assign47870_e61369;
        locals.var_guard1270_rv = 0.0;

        let assign47880_e61371: f64 = (-locals.var_xnedge_d);
        let assign47880_e61373: f64 = (assign47880_e61371 + locals.var_xnedge_s);
        let assign47880_e61375: f64 = (-230.25850929940458);
        let assign47880_e61376: f64 = if assign47880_e61373 > assign47880_e61375 { 1.0 } else { 0.0 };
        locals.var_guard1271 = assign47880_e61376;
        locals.var_guard1271_rv = 0.0;

        let (assign47890_e61388, assign47890_e61388_d_n4, assign47890_e61388_d_n6, assign47890_e61388_d_n7, assign47890_e61388_d_n8, assign47890_e61388_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 != 0.0)) {
        let assign47890_e61383: f64 = (-locals.var_xnedge_d);
        let assign47890_e61385: f64 = (assign47890_e61383 + locals.var_xnedge_s);
        let assign47890_e61386: f64 = (assign47890_e61385).exp();
        (assign47890_e61386, (assign47890_e61386 * ((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47890_e61388;
        locals.var_temp__blk949_dn4 = assign47890_e61388_d_n4;
        locals.var_temp__blk949_dn6 = assign47890_e61388_d_n6;
        locals.var_temp__blk949_dn7 = assign47890_e61388_d_n7;
        locals.var_temp__blk949_dn8 = assign47890_e61388_d_n8;
        locals.var_temp__blk949_dn9 = assign47890_e61388_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign47900_e61431, assign47900_e61431_d_n4, assign47900_e61431_d_n6, assign47900_e61431_d_n7, assign47900_e61431_d_n8, assign47900_e61431_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 == 0.0)) {
        let assign47900_e61398: f64 = (-230.25850929940458);
        let assign47900_e61400: f64 = (-locals.var_xnedge_d);
        let assign47900_e61402: f64 = (assign47900_e61400 + locals.var_xnedge_s);
        let assign47900_e61403: f64 = (assign47900_e61398 - assign47900_e61402);
        let assign47900_e61407: f64 = (-230.25850929940458);
        let assign47900_e61409: f64 = (-locals.var_xnedge_d);
        let assign47900_e61411: f64 = (assign47900_e61409 + locals.var_xnedge_s);
        let assign47900_e61412: f64 = (assign47900_e61407 - assign47900_e61411);
        let assign47900_e61415: f64 = (-230.25850929940458);
        let assign47900_e61417: f64 = (-locals.var_xnedge_d);
        let assign47900_e61419: f64 = (assign47900_e61417 + locals.var_xnedge_s);
        let assign47900_e61420: f64 = (assign47900_e61415 - assign47900_e61419);
        let assign47900_e61422: f64 = (assign47900_e61420 * 0.3333333333333333);
        let assign47900_e61423: f64 = (1.0 + assign47900_e61422);
        let assign47900_e61424: f64 = (assign47900_e61412 * assign47900_e61423);
        let assign47900_e61425: f64 = (0.5 * assign47900_e61424);
        let assign47900_e61426: f64 = (1.0 + assign47900_e61425);
        let assign47900_e61427: f64 = (assign47900_e61403 * assign47900_e61426);
        let assign47900_e61428: f64 = (1.0 + assign47900_e61427);
        let assign47900_e61429: f64 = (1e-100 / assign47900_e61428);
        (assign47900_e61429, (-((1e-100 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47900_e61431;
        locals.var_temp__blk949_dn4 = assign47900_e61431_d_n4;
        locals.var_temp__blk949_dn6 = assign47900_e61431_d_n6;
        locals.var_temp__blk949_dn7 = assign47900_e61431_d_n7;
        locals.var_temp__blk949_dn8 = assign47900_e61431_d_n8;
        locals.var_temp__blk949_dn9 = assign47900_e61431_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign47910_e61441, assign47910_e61441_d_n4, assign47910_e61441_d_n6, assign47910_e61441_d_n7, assign47910_e61441_d_n8, assign47910_e61441_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
        let assign47910_e61438: f64 = (locals.var_temp__blk949 - 1.0);
        let assign47910_e61439: f64 = (locals.var_qseffedge * assign47910_e61438);
        (assign47910_e61439, ((locals.var_qseffedge_dn4 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn4)), ((locals.var_qseffedge_dn6 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn6)), ((locals.var_qseffedge_dn7 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn7)), ((locals.var_qseffedge_dn8 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn8)), ((locals.var_qseffedge_dn9 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9,)
    }
};
        locals.var_qdseffedge = assign47910_e61441;
        locals.var_qdseffedge_dn4 = assign47910_e61441_d_n4;
        locals.var_qdseffedge_dn6 = assign47910_e61441_d_n6;
        locals.var_qdseffedge_dn7 = assign47910_e61441_d_n7;
        locals.var_qdseffedge_dn8 = assign47910_e61441_d_n8;
        locals.var_qdseffedge_dn9 = assign47910_e61441_d_n9;
        locals.var_qdseffedge_rv = 0.0;

        let (assign47920_e61449, assign47920_e61449_d_n4, assign47920_e61449_d_n6, assign47920_e61449_d_n7, assign47920_e61449_d_n8, assign47920_e61449_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
        let assign47920_e61447: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
        (assign47920_e61447, (locals.var_qdseffedge_dn4 + locals.var_qseffedge_dn4), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8), (locals.var_qdseffedge_dn9 + locals.var_qseffedge_dn9),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign47920_e61449;
        locals.var_qdeffedge_dn4 = assign47920_e61449_d_n4;
        locals.var_qdeffedge_dn6 = assign47920_e61449_d_n6;
        locals.var_qdeffedge_dn7 = assign47920_e61449_d_n7;
        locals.var_qdeffedge_dn8 = assign47920_e61449_d_n8;
        locals.var_qdeffedge_dn9 = assign47920_e61449_d_n9;
        locals.var_qdeffedge_rv = 0.0;

        let (assign47930_e61458, assign47930_e61458_d_n4, assign47930_e61458_d_n6, assign47930_e61458_d_n7, assign47930_e61458_d_n8, assign47930_e61458_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47930_e61456: f64 = (locals.var_xbedge + locals.var_xnedge_d);
        (assign47930_e61456, (locals.var_xbedge_dn4 + locals.var_xnedge_d_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_d_dn9),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9,)
    }
};
        locals.var_q_edge_xsth = assign47930_e61458;
        locals.var_q_edge_xsth_dn4 = assign47930_e61458_d_n4;
        locals.var_q_edge_xsth_dn6 = assign47930_e61458_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47930_e61458_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47930_e61458_d_n8;
        locals.var_q_edge_xsth_dn9 = assign47930_e61458_d_n9;
        locals.var_q_edge_xsth_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47940_e61470, assign47940_e61470_d_n4, assign47940_e61470_d_n6, assign47940_e61470_d_n7, assign47940_e61470_d_n8, assign47940_e61470_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47940_e61466: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47940_e61467: f64 = (locals.var_gfedge * assign47940_e61466);
        let assign47940_e61468: f64 = (locals.var_q_edge_xsth + assign47940_e61467);
        (assign47940_e61468, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47940_e61466) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47940_e61466))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47940_e61466)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9,)
    }
};
        locals.var_q_edge_xth0 = assign47940_e61470;
        locals.var_q_edge_xth0_dn4 = assign47940_e61470_d_n4;
        locals.var_q_edge_xth0_dn6 = assign47940_e61470_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47940_e61470_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47940_e61470_d_n8;
        locals.var_q_edge_xth0_dn9 = assign47940_e61470_d_n9;
        locals.var_q_edge_xth0_rv = 0.0;

        let (assign47950_e61479, assign47950_e61479_d_n4, assign47950_e61479_d_n6, assign47950_e61479_d_n7, assign47950_e61479_d_n8, assign47950_e61479_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47950_e61477: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47950_e61477, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9,)
    }
};
        locals.var_q_edge_xth = assign47950_e61479;
        locals.var_q_edge_xth_dn4 = assign47950_e61479_d_n4;
        locals.var_q_edge_xth_dn6 = assign47950_e61479_d_n6;
        locals.var_q_edge_xth_dn7 = assign47950_e61479_d_n7;
        locals.var_q_edge_xth_dn8 = assign47950_e61479_d_n8;
        locals.var_q_edge_xth_dn9 = assign47950_e61479_d_n9;
        locals.var_q_edge_xth_rv = 0.0;

        let (assign47960_e61493, assign47960_e61493_d_n4, assign47960_e61493_d_n6, assign47960_e61493_d_n7, assign47960_e61493_d_n8, assign47960_e61493_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47960_e61488: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47960_e61489: f64 = (2.0 * assign47960_e61488);
        let assign47960_e61490: f64 = (locals.var_gfedge / assign47960_e61489);
        let assign47960_e61491: f64 = (1.0 + assign47960_e61490);
        (assign47960_e61491, (((locals.var_gfedge_dn4 * assign47960_e61489) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47960_e61488))))) / (assign47960_e61489 * assign47960_e61489)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9,)
    }
};
        locals.var_q_edge_n = assign47960_e61493;
        locals.var_q_edge_n_dn4 = assign47960_e61493_d_n4;
        locals.var_q_edge_n_dn6 = assign47960_e61493_d_n6;
        locals.var_q_edge_n_dn7 = assign47960_e61493_d_n7;
        locals.var_q_edge_n_dn8 = assign47960_e61493_d_n8;
        locals.var_q_edge_n_dn9 = assign47960_e61493_d_n9;
        locals.var_q_edge_n_rv = 0.0;

        let (assign47970_e61502, assign47970_e61502_d_n4, assign47970_e61502_d_n6, assign47970_e61502_d_n7, assign47970_e61502_d_n8, assign47970_e61502_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47970_e61500: f64 = (1.0 / locals.var_q_edge_n);
        (assign47970_e61500, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9,)
    }
};
        locals.var_q_edge_n_inv = assign47970_e61502;
        locals.var_q_edge_n_inv_dn4 = assign47970_e61502_d_n4;
        locals.var_q_edge_n_inv_dn6 = assign47970_e61502_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47970_e61502_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47970_e61502_d_n8;
        locals.var_q_edge_n_inv_dn9 = assign47970_e61502_d_n9;
        locals.var_q_edge_n_inv_rv = 0.0;

        let (assign47980_e61511, assign47980_e61511_d_n4, assign47980_e61511_d_n6, assign47980_e61511_d_n7, assign47980_e61511_d_n8, assign47980_e61511_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47980_e61509: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47980_e61509, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    }
};
        locals.var_q_edge_xgt = assign47980_e61511;
        locals.var_q_edge_xgt_dn4 = assign47980_e61511_d_n4;
        locals.var_q_edge_xgt_dn6 = assign47980_e61511_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47980_e61511_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47980_e61511_d_n8;
        locals.var_q_edge_xgt_dn9 = assign47980_e61511_d_n9;
        locals.var_q_edge_xgt_rv = 0.0;

        let assign47990_e61514: f64 = (-12.0);
        let assign47990_e61515: f64 = if locals.var_q_edge_xgt > assign47990_e61514 { 1.0 } else { 0.0 };
        locals.var_guard1272 = assign47990_e61515;
        locals.var_guard1272_rv = 0.0;

        let (assign48000_e61528, assign48000_e61528_d_n4, assign48000_e61528_d_n6, assign48000_e61528_d_n7, assign48000_e61528_d_n8, assign48000_e61528_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48000_e61524: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48000_e61526: f64 = (assign48000_e61524 - 1.0);
        (assign48000_e61526, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9,)
    }
};
        locals.var_q_edge_xgt0 = assign48000_e61528;
        locals.var_q_edge_xgt0_dn4 = assign48000_e61528_d_n4;
        locals.var_q_edge_xgt0_dn6 = assign48000_e61528_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign48000_e61528_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign48000_e61528_d_n8;
        locals.var_q_edge_xgt0_dn9 = assign48000_e61528_d_n9;
        locals.var_q_edge_xgt0_rv = 0.0;

        let (assign48010_e61546, assign48010_e61546_d_n4, assign48010_e61546_d_n6, assign48010_e61546_d_n7, assign48010_e61546_d_n8, assign48010_e61546_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48010_e61539: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign48010_e61541: f64 = (assign48010_e61539 + 10.0);
        let assign48010_e61542: f64 = (assign48010_e61541).sqrt();
        let assign48010_e61543: f64 = (locals.var_q_edge_xgt0 + assign48010_e61542);
        let assign48010_e61544: f64 = (0.5 * assign48010_e61543);
        (assign48010_e61544, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign48010_e61542)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9,)
    }
};
        locals.var_q_edge_xgt0e = assign48010_e61546;
        locals.var_q_edge_xgt0e_dn4 = assign48010_e61546_d_n4;
        locals.var_q_edge_xgt0e_dn6 = assign48010_e61546_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign48010_e61546_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign48010_e61546_d_n8;
        locals.var_q_edge_xgt0e_dn9 = assign48010_e61546_d_n9;
        locals.var_q_edge_xgt0e_rv = 0.0;

        let (assign48020_e61562, assign48020_e61562_d_n4, assign48020_e61562_d_n6, assign48020_e61562_d_n7, assign48020_e61562_d_n8, assign48020_e61562_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48020_e61556: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign48020_e61557: f64 = (locals.var_q_edge_n * assign48020_e61556);
        let assign48020_e61558: f64 = (locals.var_q_edge_xgt - assign48020_e61557);
        let assign48020_e61560: f64 = (assign48020_e61558 + locals.var_lngfedge2);
        (assign48020_e61560, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9,)
    }
};
        locals.var_q_edge_qi0si = assign48020_e61562;
        locals.var_q_edge_qi0si_dn4 = assign48020_e61562_d_n4;
        locals.var_q_edge_qi0si_dn6 = assign48020_e61562_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign48020_e61562_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign48020_e61562_d_n8;
        locals.var_q_edge_qi0si_dn9 = assign48020_e61562_d_n9;
        locals.var_q_edge_qi0si_rv = 0.0;

        let (assign48030_e61580, assign48030_e61580_d_n4, assign48030_e61580_d_n6, assign48030_e61580_d_n7, assign48030_e61580_d_n8, assign48030_e61580_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48030_e61573: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign48030_e61575: f64 = (assign48030_e61573 + 2.0);
        let assign48030_e61576: f64 = (assign48030_e61575).sqrt();
        let assign48030_e61577: f64 = (locals.var_q_edge_qi0si + assign48030_e61576);
        let assign48030_e61578: f64 = (0.5 * assign48030_e61577);
        (assign48030_e61578, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign48030_e61576)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9,)
    }
};
        locals.var_q_edge_qi0 = assign48030_e61580;
        locals.var_q_edge_qi0_dn4 = assign48030_e61580_d_n4;
        locals.var_q_edge_qi0_dn6 = assign48030_e61580_d_n6;
        locals.var_q_edge_qi0_dn7 = assign48030_e61580_d_n7;
        locals.var_q_edge_qi0_dn8 = assign48030_e61580_d_n8;
        locals.var_q_edge_qi0_dn9 = assign48030_e61580_d_n9;
        locals.var_q_edge_qi0_rv = 0.0;

        let assign48040_e61583: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48040_e61585: f64 = if assign48040_e61583 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1273 = assign48040_e61585;
        locals.var_guard1273_rv = 0.0;

        let (assign48050_e61599, assign48050_e61599_d_n4, assign48050_e61599_d_n6, assign48050_e61599_d_n7, assign48050_e61599_d_n8, assign48050_e61599_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 != 0.0)) {
        let assign48050_e61596: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48050_e61597: f64 = (assign48050_e61596).exp();
        (assign48050_e61597, (assign48050_e61597 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign48050_e61599;
        locals.var_q_edge_exp_x_dn4 = assign48050_e61599_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign48050_e61599_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign48050_e61599_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign48050_e61599_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign48050_e61599_d_n9;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign48060_e61639, assign48060_e61639_d_n4, assign48060_e61639_d_n6, assign48060_e61639_d_n7, assign48060_e61639_d_n8, assign48060_e61639_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 == 0.0)) {
        let assign48060_e61613: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61615: f64 = (assign48060_e61613 - 230.25850929940458);
        let assign48060_e61620: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61622: f64 = (assign48060_e61620 - 230.25850929940458);
        let assign48060_e61626: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61628: f64 = (assign48060_e61626 - 230.25850929940458);
        let assign48060_e61630: f64 = (assign48060_e61628 * 0.3333333333333333);
        let assign48060_e61631: f64 = (1.0 + assign48060_e61630);
        let assign48060_e61632: f64 = (assign48060_e61622 * assign48060_e61631);
        let assign48060_e61633: f64 = (0.5 * assign48060_e61632);
        let assign48060_e61634: f64 = (1.0 + assign48060_e61633);
        let assign48060_e61635: f64 = (assign48060_e61615 * assign48060_e61634);
        let assign48060_e61636: f64 = (1.0 + assign48060_e61635);
        let assign48060_e61637: f64 = (1e100 * assign48060_e61636);
        (assign48060_e61637, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign48060_e61639;
        locals.var_q_edge_exp_x_dn4 = assign48060_e61639_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign48060_e61639_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign48060_e61639_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign48060_e61639_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign48060_e61639_d_n9;
        locals.var_q_edge_exp_x_rv = 0.0;

        let (assign48070_e61650, assign48070_e61650_d_n4, assign48070_e61650_d_n6, assign48070_e61650_d_n7, assign48070_e61650_d_n8, assign48070_e61650_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48070_e61648: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign48070_e61648, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9,)
    }
};
        locals.var_q_edge_d0 = assign48070_e61650;
        locals.var_q_edge_d0_dn4 = assign48070_e61650_d_n4;
        locals.var_q_edge_d0_dn6 = assign48070_e61650_d_n6;
        locals.var_q_edge_d0_dn7 = assign48070_e61650_d_n7;
        locals.var_q_edge_d0_dn8 = assign48070_e61650_d_n8;
        locals.var_q_edge_d0_dn9 = assign48070_e61650_d_n9;
        locals.var_q_edge_d0_rv = 0.0;

        let (assign48080_e61661, assign48080_e61661_d_n4, assign48080_e61661_d_n6, assign48080_e61661_d_n7, assign48080_e61661_d_n8, assign48080_e61661_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48080_e61659: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign48080_e61659, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9,)
    }
};
        locals.var_q_edge_d0p = assign48080_e61661;
        locals.var_q_edge_d0p_dn4 = assign48080_e61661_d_n4;
        locals.var_q_edge_d0p_dn6 = assign48080_e61661_d_n6;
        locals.var_q_edge_d0p_dn7 = assign48080_e61661_d_n7;
        locals.var_q_edge_d0p_dn8 = assign48080_e61661_d_n8;
        locals.var_q_edge_d0p_dn9 = assign48080_e61661_d_n9;
        locals.var_q_edge_d0p_rv = 0.0;

        let (assign48090_e61682, assign48090_e61682_d_n4, assign48090_e61682_d_n6, assign48090_e61682_d_n7, assign48090_e61682_d_n8, assign48090_e61682_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48090_e61670: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign48090_e61674: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign48090_e61675: f64 = (2.0 * assign48090_e61674);
        let assign48090_e61677: f64 = (assign48090_e61675 - locals.var_q_edge_d0p);
        let assign48090_e61679: f64 = (assign48090_e61677 * locals.var_q_edge_d0p);
        let assign48090_e61680: f64 = (assign48090_e61670 + assign48090_e61679);
        (assign48090_e61680, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn9))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9,)
    }
};
        locals.var_q_edge_sqerr = assign48090_e61682;
        locals.var_q_edge_sqerr_dn4 = assign48090_e61682_d_n4;
        locals.var_q_edge_sqerr_dn6 = assign48090_e61682_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign48090_e61682_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign48090_e61682_d_n8;
        locals.var_q_edge_sqerr_dn9 = assign48090_e61682_d_n9;
        locals.var_q_edge_sqerr_rv = 0.0;

        let (assign48100_e61700, assign48100_e61700_d_n4, assign48100_e61700_d_n6, assign48100_e61700_d_n7, assign48100_e61700_d_n8, assign48100_e61700_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48100_e61691: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign48100_e61693: f64 = (assign48100_e61691 - locals.var_q_edge_n);
        let assign48100_e61695: f64 = (assign48100_e61693 / locals.var_q_edge_d0p);
        let assign48100_e61697: f64 = (assign48100_e61695 - 1.0);
        let assign48100_e61698: f64 = (locals.var_q_edge_n * assign48100_e61697);
        (assign48100_e61698, ((locals.var_q_edge_n_dn4 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9,)
    }
};
        locals.var_q_edge_errq = assign48100_e61700;
        locals.var_q_edge_errq_dn4 = assign48100_e61700_d_n4;
        locals.var_q_edge_errq_dn6 = assign48100_e61700_d_n6;
        locals.var_q_edge_errq_dn7 = assign48100_e61700_d_n7;
        locals.var_q_edge_errq_dn8 = assign48100_e61700_d_n8;
        locals.var_q_edge_errq_dn9 = assign48100_e61700_d_n9;
        locals.var_q_edge_errq_rv = 0.0;

        let (assign48110_e61711, assign48110_e61711_d_n4, assign48110_e61711_d_n6, assign48110_e61711_d_n7, assign48110_e61711_d_n8, assign48110_e61711_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48110_e61709: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign48110_e61709, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48110_e61711;
        locals.var_qdeffedge_dn4 = assign48110_e61711_d_n4;
        locals.var_qdeffedge_dn6 = assign48110_e61711_d_n6;
        locals.var_qdeffedge_dn7 = assign48110_e61711_d_n7;
        locals.var_qdeffedge_dn8 = assign48110_e61711_d_n8;
        locals.var_qdeffedge_dn9 = assign48110_e61711_d_n9;
        locals.var_qdeffedge_rv = 0.0;

        let assign48120_e61715: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48120_e61716: f64 = (locals.var_q_edge_n_inv * assign48120_e61715);
        let assign48120_e61718: f64 = (-230.25850929940458);
        let assign48120_e61719: f64 = if assign48120_e61716 > assign48120_e61718 { 1.0 } else { 0.0 };
        locals.var_guard1274 = assign48120_e61719;
        locals.var_guard1274_rv = 0.0;

        let (assign48130_e61736, assign48130_e61736_d_n4, assign48130_e61736_d_n6, assign48130_e61736_d_n7, assign48130_e61736_d_n8, assign48130_e61736_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 != 0.0)) {
        let assign48130_e61732: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48130_e61733: f64 = (locals.var_q_edge_n_inv * assign48130_e61732);
        let assign48130_e61734: f64 = (assign48130_e61733).exp();
        (assign48130_e61734, (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn4 * assign48130_e61732) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn6 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn7 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn8 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn9 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48130_e61736;
        locals.var_qdeffedge_dn4 = assign48130_e61736_d_n4;
        locals.var_qdeffedge_dn6 = assign48130_e61736_d_n6;
        locals.var_qdeffedge_dn7 = assign48130_e61736_d_n7;
        locals.var_qdeffedge_dn8 = assign48130_e61736_d_n8;
        locals.var_qdeffedge_dn9 = assign48130_e61736_d_n9;
        locals.var_qdeffedge_rv = 0.0;

        let (assign48140_e61786, assign48140_e61786_d_n4, assign48140_e61786_d_n6, assign48140_e61786_d_n7, assign48140_e61786_d_n8, assign48140_e61786_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 == 0.0)) {
        let assign48140_e61750: f64 = (-230.25850929940458);
        let assign48140_e61754: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61755: f64 = (locals.var_q_edge_n_inv * assign48140_e61754);
        let assign48140_e61756: f64 = (assign48140_e61750 - assign48140_e61755);
        let assign48140_e61760: f64 = (-230.25850929940458);
        let assign48140_e61764: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61765: f64 = (locals.var_q_edge_n_inv * assign48140_e61764);
        let assign48140_e61766: f64 = (assign48140_e61760 - assign48140_e61765);
        let assign48140_e61769: f64 = (-230.25850929940458);
        let assign48140_e61773: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61774: f64 = (locals.var_q_edge_n_inv * assign48140_e61773);
        let assign48140_e61775: f64 = (assign48140_e61769 - assign48140_e61774);
        let assign48140_e61777: f64 = (assign48140_e61775 * 0.3333333333333333);
        let assign48140_e61778: f64 = (1.0 + assign48140_e61777);
        let assign48140_e61779: f64 = (assign48140_e61766 * assign48140_e61778);
        let assign48140_e61780: f64 = (0.5 * assign48140_e61779);
        let assign48140_e61781: f64 = (1.0 + assign48140_e61780);
        let assign48140_e61782: f64 = (assign48140_e61756 * assign48140_e61781);
        let assign48140_e61783: f64 = (1.0 + assign48140_e61782);
        let assign48140_e61784: f64 = (1e-100 / assign48140_e61783);
        (assign48140_e61784, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61754) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61764) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61773) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48140_e61786;
        locals.var_qdeffedge_dn4 = assign48140_e61786_d_n4;
        locals.var_qdeffedge_dn6 = assign48140_e61786_d_n6;
        locals.var_qdeffedge_dn7 = assign48140_e61786_d_n7;
        locals.var_qdeffedge_dn8 = assign48140_e61786_d_n8;
        locals.var_qdeffedge_dn9 = assign48140_e61786_d_n9;
        locals.var_qdeffedge_rv = 0.0;

        let (assign48150_e61795, assign48150_e61795_d_n4, assign48150_e61795_d_n6, assign48150_e61795_d_n7, assign48150_e61795_d_n8, assign48150_e61795_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign48150_e61793: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
        (assign48150_e61793, (locals.var_qdeffedge_dn4 - locals.var_qseffedge_dn4), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8), (locals.var_qdeffedge_dn9 - locals.var_qseffedge_dn9),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9,)
    }
};
        locals.var_qdseffedge = assign48150_e61795;
        locals.var_qdseffedge_dn4 = assign48150_e61795_d_n4;
        locals.var_qdseffedge_dn6 = assign48150_e61795_d_n6;
        locals.var_qdseffedge_dn7 = assign48150_e61795_d_n7;
        locals.var_qdseffedge_dn8 = assign48150_e61795_d_n8;
        locals.var_qdseffedge_dn9 = assign48150_e61795_d_n9;
        locals.var_qdseffedge_rv = 0.0;

        let (assign48160_e61803, assign48160_e61803_d_n4, assign48160_e61803_d_n6, assign48160_e61803_d_n7, assign48160_e61803_d_n8, assign48160_e61803_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48160_e61800: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
        let assign48160_e61801: f64 = (0.5 * assign48160_e61800);
        (assign48160_e61801, (0.5 * (locals.var_qdeffedge_dn4 + locals.var_qseffedge_dn4)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)), (0.5 * (locals.var_qdeffedge_dn9 + locals.var_qseffedge_dn9)),)
    } else {
        (locals.var_qmeffedge, locals.var_qmeffedge_dn4, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, locals.var_qmeffedge_dn9,)
    }
};
        locals.var_qmeffedge = assign48160_e61803;
        locals.var_qmeffedge_dn4 = assign48160_e61803_d_n4;
        locals.var_qmeffedge_dn6 = assign48160_e61803_d_n6;
        locals.var_qmeffedge_dn7 = assign48160_e61803_d_n7;
        locals.var_qmeffedge_dn8 = assign48160_e61803_d_n8;
        locals.var_qmeffedge_dn9 = assign48160_e61803_d_n9;
        locals.var_qmeffedge_rv = 0.0;

        let (assign48170_e61816, assign48170_e61816_d_n4, assign48170_e61816_d_n6, assign48170_e61816_d_n7, assign48170_e61816_d_n8, assign48170_e61816_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48170_e61807: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        let (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9,) = {
            if (assign48170_e61807 > 1e-40) {
                let assign48170_e61812: f64 = (locals.var_xgedge - locals.var_qmeffedge);
                (assign48170_e61812, (locals.var_xgedge_dn4 - locals.var_qmeffedge_dn4), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8), (locals.var_xgedge_dn9 - locals.var_qmeffedge_dn9),)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9,)
    } else {
        (locals.var_dsqredge, locals.var_dsqredge_dn4, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, locals.var_dsqredge_dn9,)
    }
};
        locals.var_dsqredge = assign48170_e61816;
        locals.var_dsqredge_dn4 = assign48170_e61816_d_n4;
        locals.var_dsqredge_dn6 = assign48170_e61816_d_n6;
        locals.var_dsqredge_dn7 = assign48170_e61816_d_n7;
        locals.var_dsqredge_dn8 = assign48170_e61816_d_n8;
        locals.var_dsqredge_dn9 = assign48170_e61816_d_n9;
        locals.var_dsqredge_rv = 0.0;

        let (assign48180_e61831, assign48180_e61831_d_n4, assign48180_e61831_d_n6, assign48180_e61831_d_n7, assign48180_e61831_d_n8, assign48180_e61831_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48180_e61821: f64 = (0.5 * locals.var_gfedge);
        let assign48180_e61825: f64 = (0.25 * locals.var_gfedge2);
        let assign48180_e61826: f64 = (locals.var_dsqredge + assign48180_e61825);
        let assign48180_e61827: f64 = (assign48180_e61826).sqrt();
        let assign48180_e61828: f64 = (assign48180_e61821 / assign48180_e61827);
        let assign48180_e61829: f64 = (1.0 - assign48180_e61828);
        (assign48180_e61829, (-((((0.5 * locals.var_gfedge_dn4) * assign48180_e61827) - (assign48180_e61821 * ((locals.var_dsqredge_dn4 + (0.25 * locals.var_gfedge2_dn4)) / (2.0 * assign48180_e61827)))) / (assign48180_e61827 * assign48180_e61827))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn6 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn7 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn8 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn9 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))),)
    } else {
        (locals.var_alphabmedge, locals.var_alphabmedge_dn4, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, locals.var_alphabmedge_dn9,)
    }
};
        locals.var_alphabmedge = assign48180_e61831;
        locals.var_alphabmedge_dn4 = assign48180_e61831_d_n4;
        locals.var_alphabmedge_dn6 = assign48180_e61831_d_n6;
        locals.var_alphabmedge_dn7 = assign48180_e61831_d_n7;
        locals.var_alphabmedge_dn8 = assign48180_e61831_d_n8;
        locals.var_alphabmedge_dn9 = assign48180_e61831_d_n9;
        locals.var_alphabmedge_rv = 0.0;

        let (assign48190_e61850, assign48190_e61850_d_n4, assign48190_e61850_d_n6, assign48190_e61850_d_n7, assign48190_e61850_d_n8, assign48190_e61850_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48190_e61834: f64 = (-locals.var_betedge_i);
        let assign48190_e61836: f64 = (assign48190_e61834 * locals.var_phit1edge);
        let assign48190_e61838: f64 = (assign48190_e61836 * locals.var_phit1edge);
        let assign48190_e61841: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
        let assign48190_e61843: f64 = (assign48190_e61841 + 1.0);
        let assign48190_e61844: f64 = (assign48190_e61838 * assign48190_e61843);
        let assign48190_e61846: f64 = (assign48190_e61844 * locals.var_qdseffedge);
        let assign48190_e61848: f64 = (assign48190_e61846 / locals.var_gmob_dc);
        (assign48190_e61848, ((((((((((((-locals.var_betedge_i_dn4) * locals.var_phit1edge) + (assign48190_e61834 * locals.var_phit1edge_dn4)) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn4)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn4 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn4)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn4)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn6)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn7)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn8)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn9) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn9)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn9 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn9)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn9)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_i_dsedge, locals.var_i_dsedge_dn4, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, locals.var_i_dsedge_dn9,)
    }
};
        locals.var_i_dsedge = assign48190_e61850;
        locals.var_i_dsedge_dn4 = assign48190_e61850_d_n4;
        locals.var_i_dsedge_dn6 = assign48190_e61850_d_n6;
        locals.var_i_dsedge_dn7 = assign48190_e61850_d_n7;
        locals.var_i_dsedge_dn8 = assign48190_e61850_d_n8;
        locals.var_i_dsedge_dn9 = assign48190_e61850_d_n9;
        locals.var_i_dsedge_rv = 0.0;

        locals.var_mavl = 0.0;
        locals.var_mavl_dn4 = 0.0;
        locals.var_mavl_dn6 = 0.0;
        locals.var_mavl_dn7 = 0.0;
        locals.var_mavl_dn8 = 0.0;
        locals.var_mavl_dn9 = 0.0;
        locals.var_mavl_rv = 0.0;

        locals.var_iimpact = 0.0;
        locals.var_iimpact_dn4 = 0.0;
        locals.var_iimpact_dn6 = 0.0;
        locals.var_iimpact_dn7 = 0.0;
        locals.var_iimpact_dn8 = 0.0;
        locals.var_iimpact_dn9 = 0.0;
        locals.var_iimpact_rv = 0.0;

        let assign48220_e61859: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1275 = assign48220_e61859;
        locals.var_guard1275_rv = 0.0;

        let (assign48230_e61867, assign48230_e61867_d_n4, assign48230_e61867_d_n6, assign48230_e61867_d_n7, assign48230_e61867_d_n8, assign48230_e61867_d_n9,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign48230_e61864: f64 = (locals.var_a3_i * locals.var_dps_dc);
        let assign48230_e61865: f64 = (locals.var_v_ds - assign48230_e61864);
        (assign48230_e61865, (-(locals.var_a3_i * locals.var_dps_dc_dn4)), (-(locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (locals.var_v_ds_dn8 - (locals.var_a3_i * locals.var_dps_dc_dn8)), (-(locals.var_a3_i * locals.var_dps_dc_dn9)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn4, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, locals.var_delvsat_dn9,)
    }
};
        locals.var_delvsat = assign48230_e61867;
        locals.var_delvsat_dn4 = assign48230_e61867_d_n4;
        locals.var_delvsat_dn6 = assign48230_e61867_d_n6;
        locals.var_delvsat_dn7 = assign48230_e61867_d_n7;
        locals.var_delvsat_dn8 = assign48230_e61867_d_n8;
        locals.var_delvsat_dn9 = assign48230_e61867_d_n9;
        locals.var_delvsat_rv = 0.0;

        let assign48240_e61870: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1276 = assign48240_e61870;
        locals.var_guard1276_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48250_e61891, assign48250_e61891_d_n4, assign48250_e61891_d_n6, assign48250_e61891_d_n7, assign48250_e61891_d_n8, assign48250_e61891_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48250_e61879: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
        let assign48250_e61880: f64 = (assign48250_e61879).sqrt();
        let assign48250_e61882: f64 = (assign48250_e61880 - locals.var_sqrt_phib_dc);
        let assign48250_e61883: f64 = (locals.var_a4_i * assign48250_e61882);
        let assign48250_e61884: f64 = (1.0 + assign48250_e61883);
        let assign48250_e61887: f64 = (locals.var_delvsat + 1e-30);
        let assign48250_e61888: f64 = (assign48250_e61884 / assign48250_e61887);
        let assign48250_e61889: f64 = (locals.var_a2_t * assign48250_e61888);
        (assign48250_e61889, ((locals.var_a2_t_dn4 * assign48250_e61888) + (locals.var_a2_t * ((((locals.var_a4_i * (((locals.var_phib_dc_dn4 + locals.var_vsbstar_dc_dn4) / (2.0 * assign48250_e61880)) - locals.var_sqrt_phib_dc_dn4)) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn4)) / (assign48250_e61887 * assign48250_e61887)))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn6)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn7)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn8)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn9 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn9)) / (assign48250_e61887 * assign48250_e61887))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48250_e61891;
        locals.var_temp2_dn4 = assign48250_e61891_d_n4;
        locals.var_temp2_dn6 = assign48250_e61891_d_n6;
        locals.var_temp2_dn7 = assign48250_e61891_d_n7;
        locals.var_temp2_dn8 = assign48250_e61891_d_n8;
        locals.var_temp2_dn9 = assign48250_e61891_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign48260_e61893: f64 = (-locals.var_temp2);
        let assign48260_e61894: f64 = (assign48260_e61893).abs();
        let assign48260_e61896: f64 = if assign48260_e61894 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1277 = assign48260_e61896;
        locals.var_guard1277_rv = 0.0;

        let (assign48270_e61906, assign48270_e61906_d_n4, assign48270_e61906_d_n6, assign48270_e61906_d_n7, assign48270_e61906_d_n8, assign48270_e61906_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 != 0.0)) {
        let assign48270_e61903: f64 = (-locals.var_temp2);
        let assign48270_e61904: f64 = (assign48270_e61903).exp();
        (assign48270_e61904, (assign48270_e61904 * (-locals.var_temp2_dn4)), (assign48270_e61904 * (-locals.var_temp2_dn6)), (assign48270_e61904 * (-locals.var_temp2_dn7)), (assign48270_e61904 * (-locals.var_temp2_dn8)), (assign48270_e61904 * (-locals.var_temp2_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48270_e61906;
        locals.var_temp__blk949_dn4 = assign48270_e61906_d_n4;
        locals.var_temp__blk949_dn6 = assign48270_e61906_d_n6;
        locals.var_temp__blk949_dn7 = assign48270_e61906_d_n7;
        locals.var_temp__blk949_dn8 = assign48270_e61906_d_n8;
        locals.var_temp__blk949_dn9 = assign48270_e61906_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign48280_e61908: f64 = (-locals.var_temp2);
        let assign48280_e61910: f64 = if assign48280_e61908 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign48280_e61910;
        locals.var_guard1278_rv = 0.0;

        let (assign48290_e61949, assign48290_e61949_d_n4, assign48290_e61949_d_n6, assign48290_e61949_d_n7, assign48290_e61949_d_n8, assign48290_e61949_d_n9,) = {
    if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign48290_e61922: f64 = (-230.25850929940458);
        let assign48290_e61924: f64 = (-locals.var_temp2);
        let assign48290_e61925: f64 = (assign48290_e61922 - assign48290_e61924);
        let assign48290_e61929: f64 = (-230.25850929940458);
        let assign48290_e61931: f64 = (-locals.var_temp2);
        let assign48290_e61932: f64 = (assign48290_e61929 - assign48290_e61931);
        let assign48290_e61935: f64 = (-230.25850929940458);
        let assign48290_e61937: f64 = (-locals.var_temp2);
        let assign48290_e61938: f64 = (assign48290_e61935 - assign48290_e61937);
        let assign48290_e61940: f64 = (assign48290_e61938 * 0.3333333333333333);
        let assign48290_e61941: f64 = (1.0 + assign48290_e61940);
        let assign48290_e61942: f64 = (assign48290_e61932 * assign48290_e61941);
        let assign48290_e61943: f64 = (0.5 * assign48290_e61942);
        let assign48290_e61944: f64 = (1.0 + assign48290_e61943);
        let assign48290_e61945: f64 = (assign48290_e61925 * assign48290_e61944);
        let assign48290_e61946: f64 = (1.0 + assign48290_e61945);
        let assign48290_e61947: f64 = (1e-100 / assign48290_e61946);
        (assign48290_e61947, (-((1e-100 * (((-(-locals.var_temp2_dn4)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn4)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn4)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn9)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn9)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn9)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48290_e61949;
        locals.var_temp__blk949_dn4 = assign48290_e61949_d_n4;
        locals.var_temp__blk949_dn6 = assign48290_e61949_d_n6;
        locals.var_temp__blk949_dn7 = assign48290_e61949_d_n7;
        locals.var_temp__blk949_dn8 = assign48290_e61949_d_n8;
        locals.var_temp__blk949_dn9 = assign48290_e61949_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign48300_e61986, assign48300_e61986_d_n4, assign48300_e61986_d_n6, assign48300_e61986_d_n7, assign48300_e61986_d_n8, assign48300_e61986_d_n9,) = {
    if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign48300_e61962: f64 = (-locals.var_temp2);
        let assign48300_e61964: f64 = (assign48300_e61962 - 230.25850929940458);
        let assign48300_e61968: f64 = (-locals.var_temp2);
        let assign48300_e61970: f64 = (assign48300_e61968 - 230.25850929940458);
        let assign48300_e61973: f64 = (-locals.var_temp2);
        let assign48300_e61975: f64 = (assign48300_e61973 - 230.25850929940458);
        let assign48300_e61977: f64 = (assign48300_e61975 * 0.3333333333333333);
        let assign48300_e61978: f64 = (1.0 + assign48300_e61977);
        let assign48300_e61979: f64 = (assign48300_e61970 * assign48300_e61978);
        let assign48300_e61980: f64 = (0.5 * assign48300_e61979);
        let assign48300_e61981: f64 = (1.0 + assign48300_e61980);
        let assign48300_e61982: f64 = (assign48300_e61964 * assign48300_e61981);
        let assign48300_e61983: f64 = (1.0 + assign48300_e61982);
        let assign48300_e61984: f64 = (1e100 * assign48300_e61983);
        (assign48300_e61984, (1e100 * (((-locals.var_temp2_dn4) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn4) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn6) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn7) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn8) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn9) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn9) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48300_e61986;
        locals.var_temp__blk949_dn4 = assign48300_e61986_d_n4;
        locals.var_temp__blk949_dn6 = assign48300_e61986_d_n6;
        locals.var_temp__blk949_dn7 = assign48300_e61986_d_n7;
        locals.var_temp__blk949_dn8 = assign48300_e61986_d_n8;
        locals.var_temp__blk949_dn9 = assign48300_e61986_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign48310_e61996, assign48310_e61996_d_n4, assign48310_e61996_d_n6, assign48310_e61996_d_n7, assign48310_e61996_d_n8, assign48310_e61996_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48310_e61993: f64 = (locals.var_delvsat * locals.var_temp__blk949);
        let assign48310_e61994: f64 = (locals.var_a1_i * assign48310_e61993);
        (assign48310_e61994, (locals.var_a1_i * ((locals.var_delvsat_dn4 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn4))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn8))), (locals.var_a1_i * ((locals.var_delvsat_dn9 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9,)
    }
};
        locals.var_mavl = assign48310_e61996;
        locals.var_mavl_dn4 = assign48310_e61996_d_n4;
        locals.var_mavl_dn6 = assign48310_e61996_d_n6;
        locals.var_mavl_dn7 = assign48310_e61996_d_n7;
        locals.var_mavl_dn8 = assign48310_e61996_d_n8;
        locals.var_mavl_dn9 = assign48310_e61996_d_n9;
        locals.var_mavl_rv = 0.0;

        let (assign48320_e62006, assign48320_e62006_d_n4, assign48320_e62006_d_n6, assign48320_e62006_d_n7, assign48320_e62006_d_n8, assign48320_e62006_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48320_e62003: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let assign48320_e62004: f64 = (locals.var_mavl * assign48320_e62003);
        (assign48320_e62004, ((locals.var_mavl_dn4 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4))), ((locals.var_mavl_dn6 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))), ((locals.var_mavl_dn9 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign48320_e62006;
        locals.var_iimpact_dn4 = assign48320_e62006_d_n4;
        locals.var_iimpact_dn6 = assign48320_e62006_d_n6;
        locals.var_iimpact_dn7 = assign48320_e62006_d_n7;
        locals.var_iimpact_dn8 = assign48320_e62006_d_n8;
        locals.var_iimpact_dn9 = assign48320_e62006_d_n9;
        locals.var_iimpact_rv = 0.0;

        let assign48330_e62010: f64 = (0.5 * locals.var_imaxii_i);
        let assign48330_e62011: f64 = if locals.var_iimpact > assign48330_e62010 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign48330_e62011;
        locals.var_guard1279_rv = 0.0;

        let (assign48340_e62025, assign48340_e62025_d_n4, assign48340_e62025_d_n6, assign48340_e62025_d_n7, assign48340_e62025_d_n8, assign48340_e62025_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign48340_e62019: f64 = (2.0 * locals.var_iimpact);
        let assign48340_e62021: f64 = (assign48340_e62019 / locals.var_imaxii_i);
        let assign48340_e62023: f64 = (assign48340_e62021 - 1.0);
        (assign48340_e62023, ((2.0 * locals.var_iimpact_dn4) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn9) / locals.var_imaxii_i),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48340_e62025;
        locals.var_temp__blk949_dn4 = assign48340_e62025_d_n4;
        locals.var_temp__blk949_dn6 = assign48340_e62025_d_n6;
        locals.var_temp__blk949_dn7 = assign48340_e62025_d_n7;
        locals.var_temp__blk949_dn8 = assign48340_e62025_d_n8;
        locals.var_temp__blk949_dn9 = assign48340_e62025_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign48350_e62046, assign48350_e62046_d_n4, assign48350_e62046_d_n6, assign48350_e62046_d_n7, assign48350_e62046_d_n8, assign48350_e62046_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign48350_e62033: f64 = (0.5 * locals.var_imaxii_i);
        let assign48350_e62039: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign48350_e62040: f64 = (1.0 + assign48350_e62039);
        let assign48350_e62041: f64 = (assign48350_e62040).sqrt();
        let assign48350_e62042: f64 = (locals.var_temp__blk949 / assign48350_e62041);
        let assign48350_e62043: f64 = (1.0 + assign48350_e62042);
        let assign48350_e62044: f64 = (assign48350_e62033 * assign48350_e62043);
        (assign48350_e62044, (assign48350_e62033 * (((locals.var_temp__blk949_dn4 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn6 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn7 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn8 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn9 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign48350_e62046;
        locals.var_iimpact_dn4 = assign48350_e62046_d_n4;
        locals.var_iimpact_dn6 = assign48350_e62046_d_n6;
        locals.var_iimpact_dn7 = assign48350_e62046_d_n7;
        locals.var_iimpact_dn8 = assign48350_e62046_d_n8;
        locals.var_iimpact_dn9 = assign48350_e62046_d_n9;
        locals.var_iimpact_rv = 0.0;

        let assign48360_e62057: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign48360_e62057;
        locals.var_guard1473_rv = 0.0;

        let assign48370_e62064: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign48370_e62064;
        locals.var_guard1474_rv = 0.0;

        let (assign48380_e62070, assign48380_e62070_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4,)
    }
};
        locals.var_phib__blk1314 = assign48380_e62070;
        locals.var_phib__blk1314_dn4 = assign48380_e62070_d_n4;
        locals.var_phib__blk1314_rv = 0.0;

        let (assign48390_e62076, assign48390_e62076_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_aphi_dc, locals.var_aphi_dc_dn4,)
    } else {
        (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4,)
    }
};
        locals.var_aphi__blk1315 = assign48390_e62076;
        locals.var_aphi__blk1315_dn4 = assign48390_e62076_d_n4;
        locals.var_aphi__blk1315_rv = 0.0;

        let (assign48400_e62082, assign48400_e62082_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_g_0_dc, locals.var_g_0_dc_dn4,)
    } else {
        (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4,)
    }
};
        locals.var_g_0__blk1316 = assign48400_e62082;
        locals.var_g_0__blk1316_dn4 = assign48400_e62082_d_n4;
        locals.var_g_0__blk1316_rv = 0.0;

        let (assign48410_e62088, assign48410_e62088_d_n4, assign48410_e62088_d_n7, assign48410_e62088_d_n8, assign48410_e62088_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn4, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8, locals.var_v_xb_dc_tmp_dn9,)
    } else {
        (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9,)
    }
};
        locals.var_v_xb__blk1317 = assign48410_e62088;
        locals.var_v_xb__blk1317_dn4 = assign48410_e62088_d_n4;
        locals.var_v_xb__blk1317_dn7 = assign48410_e62088_d_n7;
        locals.var_v_xb__blk1317_dn8 = assign48410_e62088_d_n8;
        locals.var_v_xb__blk1317_dn9 = assign48410_e62088_d_n9;
        locals.var_v_xb__blk1317_rv = 0.0;

        let (assign48420_e62094, assign48420_e62094_d_n4, assign48420_e62094_d_n6, assign48420_e62094_d_n7, assign48420_e62094_d_n8, assign48420_e62094_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn4, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8, locals.var_vsbstar_dc_tmp_dn9,)
    } else {
        (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9,)
    }
};
        locals.var_vsbstar__blk1318 = assign48420_e62094;
        locals.var_vsbstar__blk1318_dn4 = assign48420_e62094_d_n4;
        locals.var_vsbstar__blk1318_dn6 = assign48420_e62094_d_n6;
        locals.var_vsbstar__blk1318_dn7 = assign48420_e62094_d_n7;
        locals.var_vsbstar__blk1318_dn8 = assign48420_e62094_d_n8;
        locals.var_vsbstar__blk1318_dn9 = assign48420_e62094_d_n9;
        locals.var_vsbstar__blk1318_rv = 0.0;

        let (assign48430_e62100,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_dvbstar__blk1322,)
    }
};
        locals.var_dvbstar__blk1322 = assign48430_e62100;
        locals.var_dvbstar__blk1322_rv = 0.0;

        let assign48440_e62103: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign48440_e62103;
        locals.var_guard1475_rv = 0.0;

        let (assign48450_e62128, assign48450_e62128_d_n4, assign48450_e62128_d_n7, assign48450_e62128_d_n8, assign48450_e62128_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign48450_e62112: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign48450_e62115: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48450_e62118: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48450_e62119: f64 = (assign48450_e62115 * assign48450_e62118);
        let assign48450_e62121: f64 = (assign48450_e62119 + locals.var_bphi_ac);
        let assign48450_e62122: f64 = (assign48450_e62121).sqrt();
        let assign48450_e62123: f64 = (assign48450_e62112 - assign48450_e62122);
        let assign48450_e62124: f64 = (0.5 * assign48450_e62123);
        let assign48450_e62126: f64 = (assign48450_e62124 + locals.var_phix_ac);
        (assign48450_e62126, ((0.5 * (-(locals.var_bphi_ac_dn4 / (2.0 * assign48450_e62122)))) + locals.var_phix_ac_dn4), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign48450_e62122)))),)
    } else {
        (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9,)
    }
};
        locals.var_v_xb__blk1317 = assign48450_e62128;
        locals.var_v_xb__blk1317_dn4 = assign48450_e62128_d_n4;
        locals.var_v_xb__blk1317_dn7 = assign48450_e62128_d_n7;
        locals.var_v_xb__blk1317_dn8 = assign48450_e62128_d_n8;
        locals.var_v_xb__blk1317_dn9 = assign48450_e62128_d_n9;
        locals.var_v_xb__blk1317_rv = 0.0;

        let (assign48460_e62155, assign48460_e62155_d_n4, assign48460_e62155_d_n7, assign48460_e62155_d_n8, assign48460_e62155_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign48460_e62138: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62141: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62144: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62145: f64 = (assign48460_e62141 * assign48460_e62144);
        let assign48460_e62147: f64 = (assign48460_e62145 + locals.var_aphi_ac);
        let assign48460_e62148: f64 = (assign48460_e62147).sqrt();
        let assign48460_e62149: f64 = (assign48460_e62138 - assign48460_e62148);
        let assign48460_e62150: f64 = (0.5 * assign48460_e62149);
        let assign48460_e62151: f64 = (locals.var_v_sb - assign48460_e62150);
        let assign48460_e62153: f64 = (assign48460_e62151 + locals.var_phix1_ac);
        (assign48460_e62153, ((-(0.5 * (locals.var_v_xb__blk1317_dn4 - ((((locals.var_v_xb__blk1317_dn4 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi_ac_dn4) / (2.0 * assign48460_e62148))))) + locals.var_phix1_ac_dn4), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1317_dn7 - (((locals.var_v_xb__blk1317_dn7 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1317_dn8 - (((locals.var_v_xb__blk1317_dn8 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_v_xb__blk1317_dn9 - (((locals.var_v_xb__blk1317_dn9 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48460_e62148))))),)
    } else {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9,)
    }
};
        locals.var_vsbstar_ac = assign48460_e62155;
        locals.var_vsbstar_ac_dn4 = assign48460_e62155_d_n4;
        locals.var_vsbstar_ac_dn7 = assign48460_e62155_d_n7;
        locals.var_vsbstar_ac_dn8 = assign48460_e62155_d_n8;
        locals.var_vsbstar_ac_dn9 = assign48460_e62155_d_n9;
        locals.var_vsbstar_ac_rv = 0.0;

        let (assign48470_e62163, assign48470_e62163_d_n4, assign48470_e62163_d_n6, assign48470_e62163_d_n7, assign48470_e62163_d_n8, assign48470_e62163_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, 0.0, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9,)
    } else {
        (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9,)
    }
};
        locals.var_vsbstar__blk1318 = assign48470_e62163;
        locals.var_vsbstar__blk1318_dn4 = assign48470_e62163_d_n4;
        locals.var_vsbstar__blk1318_dn6 = assign48470_e62163_d_n6;
        locals.var_vsbstar__blk1318_dn7 = assign48470_e62163_d_n7;
        locals.var_vsbstar__blk1318_dn8 = assign48470_e62163_d_n8;
        locals.var_vsbstar__blk1318_dn9 = assign48470_e62163_d_n9;
        locals.var_vsbstar__blk1318_rv = 0.0;

        let (assign48480_e62171, assign48480_e62171_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    } else {
        (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4,)
    }
};
        locals.var_phib__blk1314 = assign48480_e62171;
        locals.var_phib__blk1314_dn4 = assign48480_e62171_d_n4;
        locals.var_phib__blk1314_rv = 0.0;

        let (assign48490_e62179, assign48490_e62179_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_aphi_ac, locals.var_aphi_ac_dn4,)
    } else {
        (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4,)
    }
};
        locals.var_aphi__blk1315 = assign48490_e62179;
        locals.var_aphi__blk1315_dn4 = assign48490_e62179_d_n4;
        locals.var_aphi__blk1315_rv = 0.0;

        let (assign48500_e62187, assign48500_e62187_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_g_0_ac, locals.var_g_0_ac_dn4,)
    } else {
        (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4,)
    }
};
        locals.var_g_0__blk1316 = assign48500_e62187;
        locals.var_g_0__blk1316_dn4 = assign48500_e62187_d_n4;
        locals.var_g_0__blk1316_rv = 0.0;

        let (assign48510_e62197, assign48510_e62197_d_n4, assign48510_e62197_d_n6, assign48510_e62197_d_n7, assign48510_e62197_d_n8, assign48510_e62197_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48510_e62193: f64 = (locals.var_vgb - locals.var_dvbstar__blk1322);
        let assign48510_e62195: f64 = (assign48510_e62193 - locals.var_vfb_t);
        (assign48510_e62195, (-locals.var_vfb_t_dn4), locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9,)
    } else {
        (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9,)
    }
};
        locals.var_vgb1__blk1321 = assign48510_e62197;
        locals.var_vgb1__blk1321_dn4 = assign48510_e62197_d_n4;
        locals.var_vgb1__blk1321_dn6 = assign48510_e62197_d_n6;
        locals.var_vgb1__blk1321_dn7 = assign48510_e62197_d_n7;
        locals.var_vgb1__blk1321_dn8 = assign48510_e62197_d_n8;
        locals.var_vgb1__blk1321_dn9 = assign48510_e62197_d_n9;
        locals.var_vgb1__blk1321_rv = 0.0;

        let (assign48520_e62209, assign48520_e62209_d_n4, assign48520_e62209_d_n6, assign48520_e62209_d_n7, assign48520_e62209_d_n8, assign48520_e62209_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48520_e62205: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign48520_e62206: f64 = (0.5 * assign48520_e62205);
        let assign48520_e62207: f64 = (locals.var_vsbstar__blk1318 + assign48520_e62206);
        (assign48520_e62207, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, (locals.var_vsbstar__blk1318_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstar__blk1318_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstar__blk1318_dn9,)
    } else {
        (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9,)
    }
};
        locals.var_vsbx__blk1323 = assign48520_e62209;
        locals.var_vsbx__blk1323_dn4 = assign48520_e62209_d_n4;
        locals.var_vsbx__blk1323_dn6 = assign48520_e62209_d_n6;
        locals.var_vsbx__blk1323_dn7 = assign48520_e62209_d_n7;
        locals.var_vsbx__blk1323_dn8 = assign48520_e62209_d_n8;
        locals.var_vsbx__blk1323_dn9 = assign48520_e62209_d_n9;
        locals.var_vsbx__blk1323_rv = 0.0;

        let (assign48530_e62215, assign48530_e62215_d_n4, assign48530_e62215_d_n6, assign48530_e62215_d_n7, assign48530_e62215_d_n8, assign48530_e62215_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48530_e62215;
        locals.var_dctg__blk1335_dn4 = assign48530_e62215_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48530_e62215_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48530_e62215_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48530_e62215_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48530_e62215_d_n9;
        locals.var_dctg__blk1335_rv = 0.0;

        let assign48540_e62218: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign48540_e62218;
        locals.var_guard1476_rv = 0.0;

        let (assign48550_e62228, assign48550_e62228_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48550_e62226: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit);
        (assign48550_e62226, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit) + (locals.var_phib__blk1314 * locals.var_inv_phit_dn4)),)
    } else {
        (locals.var_xbct__blk1326, locals.var_xbct__blk1326_dn4,)
    }
};
        locals.var_xbct__blk1326 = assign48550_e62228;
        locals.var_xbct__blk1326_dn4 = assign48550_e62228_d_n4;
        locals.var_xbct__blk1326_rv = 0.0;

        let (assign48560_e62238, assign48560_e62238_d_n4, assign48560_e62238_d_n6, assign48560_e62238_d_n7, assign48560_e62238_d_n8, assign48560_e62238_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48560_e62236: f64 = (locals.var_vsbx__blk1323 * locals.var_inv_phit);
        (assign48560_e62236, ((locals.var_vsbx__blk1323_dn4 * locals.var_inv_phit) + (locals.var_vsbx__blk1323 * locals.var_inv_phit_dn4)), (locals.var_vsbx__blk1323_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn8 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar__blk1327, locals.var_xsbstar__blk1327_dn4, locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9,)
    }
};
        locals.var_xsbstar__blk1327 = assign48560_e62238;
        locals.var_xsbstar__blk1327_dn4 = assign48560_e62238_d_n4;
        locals.var_xsbstar__blk1327_dn6 = assign48560_e62238_d_n6;
        locals.var_xsbstar__blk1327_dn7 = assign48560_e62238_d_n7;
        locals.var_xsbstar__blk1327_dn8 = assign48560_e62238_d_n8;
        locals.var_xsbstar__blk1327_dn9 = assign48560_e62238_d_n9;
        locals.var_xsbstar__blk1327_rv = 0.0;

        let (assign48570_e62248, assign48570_e62248_d_n4, assign48570_e62248_d_n6, assign48570_e62248_d_n7, assign48570_e62248_d_n8, assign48570_e62248_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48570_e62246: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit);
        (assign48570_e62246, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit) + (locals.var_vgb1__blk1321 * locals.var_inv_phit_dn4)), (locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct__blk1328, locals.var_xgct__blk1328_dn4, locals.var_xgct__blk1328_dn6, locals.var_xgct__blk1328_dn7, locals.var_xgct__blk1328_dn8, locals.var_xgct__blk1328_dn9,)
    }
};
        locals.var_xgct__blk1328 = assign48570_e62248;
        locals.var_xgct__blk1328_dn4 = assign48570_e62248_d_n4;
        locals.var_xgct__blk1328_dn6 = assign48570_e62248_d_n6;
        locals.var_xgct__blk1328_dn7 = assign48570_e62248_d_n7;
        locals.var_xgct__blk1328_dn8 = assign48570_e62248_d_n8;
        locals.var_xgct__blk1328_dn9 = assign48570_e62248_d_n9;
        locals.var_xgct__blk1328_rv = 0.0;

        let (assign48580_e62263, assign48580_e62263_d_n4, assign48580_e62263_d_n6, assign48580_e62263_d_n7, assign48580_e62263_d_n8, assign48580_e62263_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48580_e62257: f64 = (0.5 * locals.var_g_0__blk1316);
        let assign48580_e62259: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48580_e62260: f64 = (assign48580_e62257 / assign48580_e62259);
        let assign48580_e62261: f64 = (1.0 + assign48580_e62260);
        (assign48580_e62261, ((((0.5 * locals.var_g_0__blk1316_dn4) * assign48580_e62259) - (assign48580_e62257 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48580_e62259)))) / (assign48580_e62259 * assign48580_e62259)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48580_e62263;
        locals.var_temp1_dn4 = assign48580_e62263_d_n4;
        locals.var_temp1_dn6 = assign48580_e62263_d_n6;
        locals.var_temp1_dn7 = assign48580_e62263_d_n7;
        locals.var_temp1_dn8 = assign48580_e62263_d_n8;
        locals.var_temp1_dn9 = assign48580_e62263_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign48590_e62276, assign48590_e62276_d_n4, assign48590_e62276_d_n6, assign48590_e62276_d_n7, assign48590_e62276_d_n8, assign48590_e62276_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48590_e62272: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48590_e62273: f64 = (locals.var_g_0__blk1316 * assign48590_e62272);
        let assign48590_e62274: f64 = (locals.var_xbct__blk1326 + assign48590_e62273);
        (assign48590_e62274, (locals.var_xbct__blk1326_dn4 + ((locals.var_g_0__blk1316_dn4 * assign48590_e62272) + (locals.var_g_0__blk1316 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48590_e62272))))), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48590_e62276;
        locals.var_temp2_dn4 = assign48590_e62276_d_n4;
        locals.var_temp2_dn6 = assign48590_e62276_d_n6;
        locals.var_temp2_dn7 = assign48590_e62276_d_n7;
        locals.var_temp2_dn8 = assign48590_e62276_d_n8;
        locals.var_temp2_dn9 = assign48590_e62276_d_n9;
        locals.var_temp2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        locals: &mut StampLocals,
    ) {
        let (assign48600_e62298, assign48600_e62298_d_n4, assign48600_e62298_d_n6, assign48600_e62298_d_n7, assign48600_e62298_d_n8, assign48600_e62298_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48600_e62284: f64 = (locals.var_xgct__blk1328 - locals.var_temp2);
        let assign48600_e62286: f64 = (assign48600_e62284 / locals.var_temp1);
        let assign48600_e62289: f64 = (0.5 * locals.var_xbct__blk1326);
        let assign48600_e62290: f64 = (assign48600_e62286 + assign48600_e62289);
        let assign48600_e62293: f64 = (1.0 + locals.var_ctb_i);
        let assign48600_e62295: f64 = (assign48600_e62293 * locals.var_xsbstar__blk1327);
        let assign48600_e62296: f64 = (assign48600_e62290 - assign48600_e62295);
        (assign48600_e62296, ((((((locals.var_xgct__blk1328_dn4 - locals.var_temp2_dn4) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + (0.5 * locals.var_xbct__blk1326_dn4)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn4)), (((((locals.var_xgct__blk1328_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn6)), (((((locals.var_xgct__blk1328_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn7)), (((((locals.var_xgct__blk1328_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn8)), (((((locals.var_xgct__blk1328_dn9 - locals.var_temp2_dn9) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn9)),)
    } else {
        (locals.var_xwict__blk1329, locals.var_xwict__blk1329_dn4, locals.var_xwict__blk1329_dn6, locals.var_xwict__blk1329_dn7, locals.var_xwict__blk1329_dn8, locals.var_xwict__blk1329_dn9,)
    }
};
        locals.var_xwict__blk1329 = assign48600_e62298;
        locals.var_xwict__blk1329_dn4 = assign48600_e62298_d_n4;
        locals.var_xwict__blk1329_dn6 = assign48600_e62298_d_n6;
        locals.var_xwict__blk1329_dn7 = assign48600_e62298_d_n7;
        locals.var_xwict__blk1329_dn8 = assign48600_e62298_d_n8;
        locals.var_xwict__blk1329_dn9 = assign48600_e62298_d_n9;
        locals.var_xwict__blk1329_rv = 0.0;

        let (assign48610_e62310, assign48610_e62310_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48610_e62306: f64 = (0.5 * locals.var_xbct__blk1326);
        let assign48610_e62308: f64 = (assign48610_e62306 + 2.0);
        (assign48610_e62308, (0.5 * locals.var_xbct__blk1326_dn4),)
    } else {
        (locals.var_xctmax__blk1330, locals.var_xctmax__blk1330_dn4,)
    }
};
        locals.var_xctmax__blk1330 = assign48610_e62310;
        locals.var_xctmax__blk1330_dn4 = assign48610_e62310_d_n4;
        locals.var_xctmax__blk1330_rv = 0.0;

        let (assign48620_e62320, assign48620_e62320_d_n4, assign48620_e62320_d_n6, assign48620_e62320_d_n7, assign48620_e62320_d_n8, assign48620_e62320_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48620_e62318: f64 = (locals.var_xbct__blk1326 + locals.var_xsbstar__blk1327);
        (assign48620_e62318, (locals.var_xbct__blk1326_dn4 + locals.var_xsbstar__blk1327_dn4), locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9,)
    } else {
        (locals.var_xnct__blk1331, locals.var_xnct__blk1331_dn4, locals.var_xnct__blk1331_dn6, locals.var_xnct__blk1331_dn7, locals.var_xnct__blk1331_dn8, locals.var_xnct__blk1331_dn9,)
    }
};
        locals.var_xnct__blk1331 = assign48620_e62320;
        locals.var_xnct__blk1331_dn4 = assign48620_e62320_d_n4;
        locals.var_xnct__blk1331_dn6 = assign48620_e62320_d_n6;
        locals.var_xnct__blk1331_dn7 = assign48620_e62320_d_n7;
        locals.var_xnct__blk1331_dn8 = assign48620_e62320_d_n8;
        locals.var_xnct__blk1331_dn9 = assign48620_e62320_d_n9;
        locals.var_xnct__blk1331_rv = 0.0;

        let (assign48630_e62345, assign48630_e62345_d_n4, assign48630_e62345_d_n6, assign48630_e62345_d_n7, assign48630_e62345_d_n8, assign48630_e62345_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48630_e62328: f64 = (locals.var_xgct__blk1328 - locals.var_xnct__blk1331);
        let assign48630_e62331: f64 = (locals.var_xnct__blk1331).sqrt();
        let assign48630_e62332: f64 = (locals.var_g_0__blk1316 * assign48630_e62331);
        let assign48630_e62333: f64 = (assign48630_e62328 - assign48630_e62332);
        let assign48630_e62337: f64 = (locals.var_xbct__blk1326 / locals.var_g_0__blk1316);
        let assign48630_e62339: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48630_e62340: f64 = (assign48630_e62337 + assign48630_e62339);
        let assign48630_e62341: f64 = (assign48630_e62340).ln();
        let assign48630_e62342: f64 = (2.0 * assign48630_e62341);
        let assign48630_e62343: f64 = (assign48630_e62333 - assign48630_e62342);
        (assign48630_e62343, (((locals.var_xgct__blk1328_dn4 - locals.var_xnct__blk1331_dn4) - ((locals.var_g_0__blk1316_dn4 * assign48630_e62331) + (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn4 / (2.0 * assign48630_e62331))))) - (2.0 * (((((locals.var_xbct__blk1326_dn4 * locals.var_g_0__blk1316) - (locals.var_xbct__blk1326 * locals.var_g_0__blk1316_dn4)) / (locals.var_g_0__blk1316 * locals.var_g_0__blk1316)) + (locals.var_xbct__blk1326_dn4 / (2.0 * assign48630_e62339))) / assign48630_e62340))), ((locals.var_xgct__blk1328_dn6 - locals.var_xnct__blk1331_dn6) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn6 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn7 - locals.var_xnct__blk1331_dn7) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn7 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn8 - locals.var_xnct__blk1331_dn8) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn8 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn9 - locals.var_xnct__blk1331_dn9) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn9 / (2.0 * assign48630_e62331)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48630_e62345;
        locals.var_temp1_dn4 = assign48630_e62345_d_n4;
        locals.var_temp1_dn6 = assign48630_e62345_d_n6;
        locals.var_temp1_dn7 = assign48630_e62345_d_n7;
        locals.var_temp1_dn8 = assign48630_e62345_d_n8;
        locals.var_temp1_dn9 = assign48630_e62345_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign48640_e62357, assign48640_e62357_d_n4, assign48640_e62357_d_n6, assign48640_e62357_d_n7, assign48640_e62357_d_n8, assign48640_e62357_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48640_e62353: f64 = (2.0 * locals.var_temp1);
        let assign48640_e62355: f64 = (assign48640_e62353 + locals.var_xctmax__blk1330);
        (assign48640_e62355, ((2.0 * locals.var_temp1_dn4) + locals.var_xctmax__blk1330_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_xmict__blk1332, locals.var_xmict__blk1332_dn4, locals.var_xmict__blk1332_dn6, locals.var_xmict__blk1332_dn7, locals.var_xmict__blk1332_dn8, locals.var_xmict__blk1332_dn9,)
    }
};
        locals.var_xmict__blk1332 = assign48640_e62357;
        locals.var_xmict__blk1332_dn4 = assign48640_e62357_d_n4;
        locals.var_xmict__blk1332_dn6 = assign48640_e62357_d_n6;
        locals.var_xmict__blk1332_dn7 = assign48640_e62357_d_n7;
        locals.var_xmict__blk1332_dn8 = assign48640_e62357_d_n8;
        locals.var_xmict__blk1332_dn9 = assign48640_e62357_d_n9;
        locals.var_xmict__blk1332_rv = 0.0;

        let (assign48650_e62380, assign48650_e62380_d_n4, assign48650_e62380_d_n6, assign48650_e62380_d_n7, assign48650_e62380_d_n8, assign48650_e62380_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48650_e62366: f64 = (locals.var_xwict__blk1329 + locals.var_xmict__blk1332);
        let assign48650_e62369: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
        let assign48650_e62372: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
        let assign48650_e62373: f64 = (assign48650_e62369 * assign48650_e62372);
        let assign48650_e62375: f64 = (assign48650_e62373 + 20.0);
        let assign48650_e62376: f64 = (assign48650_e62375).sqrt();
        let assign48650_e62377: f64 = (assign48650_e62366 + assign48650_e62376);
        let assign48650_e62378: f64 = (0.5 * assign48650_e62377);
        (assign48650_e62378, (0.5 * ((locals.var_xwict__blk1329_dn4 + locals.var_xmict__blk1332_dn4) + ((((locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn6 + locals.var_xmict__blk1332_dn6) + ((((locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn7 + locals.var_xmict__blk1332_dn7) + ((((locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn8 + locals.var_xmict__blk1332_dn8) + ((((locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn9 + locals.var_xmict__blk1332_dn9) + ((((locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9))) / (2.0 * assign48650_e62376)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48650_e62380;
        locals.var_temp1_dn4 = assign48650_e62380_d_n4;
        locals.var_temp1_dn6 = assign48650_e62380_d_n6;
        locals.var_temp1_dn7 = assign48650_e62380_d_n7;
        locals.var_temp1_dn8 = assign48650_e62380_d_n8;
        locals.var_temp1_dn9 = assign48650_e62380_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign48660_e62394, assign48660_e62394_d_n4, assign48660_e62394_d_n6, assign48660_e62394_d_n7, assign48660_e62394_d_n8, assign48660_e62394_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48660_e62389: f64 = (locals.var_xgct__blk1328 - locals.var_xsbstar__blk1327);
        let assign48660_e62390: f64 = (2.0 * assign48660_e62389);
        let assign48660_e62392: f64 = (assign48660_e62390 - locals.var_xctmax__blk1330);
        (assign48660_e62392, ((2.0 * (locals.var_xgct__blk1328_dn4 - locals.var_xsbstar__blk1327_dn4)) - locals.var_xctmax__blk1330_dn4), (2.0 * (locals.var_xgct__blk1328_dn6 - locals.var_xsbstar__blk1327_dn6)), (2.0 * (locals.var_xgct__blk1328_dn7 - locals.var_xsbstar__blk1327_dn7)), (2.0 * (locals.var_xgct__blk1328_dn8 - locals.var_xsbstar__blk1327_dn8)), (2.0 * (locals.var_xgct__blk1328_dn9 - locals.var_xsbstar__blk1327_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48660_e62394;
        locals.var_temp2_dn4 = assign48660_e62394_d_n4;
        locals.var_temp2_dn6 = assign48660_e62394_d_n6;
        locals.var_temp2_dn7 = assign48660_e62394_d_n7;
        locals.var_temp2_dn8 = assign48660_e62394_d_n8;
        locals.var_temp2_dn9 = assign48660_e62394_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign48670_e62417, assign48670_e62417_d_n4, assign48670_e62417_d_n6, assign48670_e62417_d_n7, assign48670_e62417_d_n8, assign48670_e62417_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48670_e62403: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign48670_e62406: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48670_e62409: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48670_e62410: f64 = (assign48670_e62406 * assign48670_e62409);
        let assign48670_e62412: f64 = (assign48670_e62410 + 20.0);
        let assign48670_e62413: f64 = (assign48670_e62412).sqrt();
        let assign48670_e62414: f64 = (assign48670_e62403 - assign48670_e62413);
        let assign48670_e62415: f64 = (0.5 * assign48670_e62414);
        (assign48670_e62415, (0.5 * ((locals.var_temp1_dn4 + locals.var_temp2_dn4) - ((((locals.var_temp1_dn4 - locals.var_temp2_dn4) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn9 + locals.var_temp2_dn9) - ((((locals.var_temp1_dn9 - locals.var_temp2_dn9) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) / (2.0 * assign48670_e62413)))),)
    } else {
        (locals.var_xsubct__blk1333, locals.var_xsubct__blk1333_dn4, locals.var_xsubct__blk1333_dn6, locals.var_xsubct__blk1333_dn7, locals.var_xsubct__blk1333_dn8, locals.var_xsubct__blk1333_dn9,)
    }
};
        locals.var_xsubct__blk1333 = assign48670_e62417;
        locals.var_xsubct__blk1333_dn4 = assign48670_e62417_d_n4;
        locals.var_xsubct__blk1333_dn6 = assign48670_e62417_d_n6;
        locals.var_xsubct__blk1333_dn7 = assign48670_e62417_d_n7;
        locals.var_xsubct__blk1333_dn8 = assign48670_e62417_d_n8;
        locals.var_xsubct__blk1333_dn9 = assign48670_e62417_d_n9;
        locals.var_xsubct__blk1333_rv = 0.0;

        let (assign48680_e62440, assign48680_e62440_d_n4, assign48680_e62440_d_n6, assign48680_e62440_d_n7, assign48680_e62440_d_n8, assign48680_e62440_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48680_e62426: f64 = (locals.var_xsubct__blk1333 + locals.var_xctmax__blk1330);
        let assign48680_e62429: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
        let assign48680_e62432: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
        let assign48680_e62433: f64 = (assign48680_e62429 * assign48680_e62432);
        let assign48680_e62435: f64 = (assign48680_e62433 + 5.0);
        let assign48680_e62436: f64 = (assign48680_e62435).sqrt();
        let assign48680_e62437: f64 = (assign48680_e62426 - assign48680_e62436);
        let assign48680_e62438: f64 = (0.5 * assign48680_e62437);
        (assign48680_e62438, (0.5 * ((locals.var_xsubct__blk1333_dn4 + locals.var_xctmax__blk1330_dn4) - ((((locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4) * assign48680_e62432) + (assign48680_e62429 * (locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4))) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn6 - (((locals.var_xsubct__blk1333_dn6 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn6)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn7 - (((locals.var_xsubct__blk1333_dn7 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn7)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn8 - (((locals.var_xsubct__blk1333_dn8 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn8)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn9 - (((locals.var_xsubct__blk1333_dn9 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn9)) / (2.0 * assign48680_e62436)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48680_e62440;
        locals.var_temp1_dn4 = assign48680_e62440_d_n4;
        locals.var_temp1_dn6 = assign48680_e62440_d_n6;
        locals.var_temp1_dn7 = assign48680_e62440_d_n7;
        locals.var_temp1_dn8 = assign48680_e62440_d_n8;
        locals.var_temp1_dn9 = assign48680_e62440_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign48690_e62466, assign48690_e62466_d_n4, assign48690_e62466_d_n6, assign48690_e62466_d_n7, assign48690_e62466_d_n8, assign48690_e62466_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48690_e62449: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62450: f64 = (locals.var_temp1 + assign48690_e62449);
        let assign48690_e62453: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62454: f64 = (locals.var_temp1 - assign48690_e62453);
        let assign48690_e62457: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62458: f64 = (locals.var_temp1 - assign48690_e62457);
        let assign48690_e62459: f64 = (assign48690_e62454 * assign48690_e62458);
        let assign48690_e62461: f64 = (assign48690_e62459 + 20.0);
        let assign48690_e62462: f64 = (assign48690_e62461).sqrt();
        let assign48690_e62463: f64 = (assign48690_e62450 + assign48690_e62462);
        let assign48690_e62464: f64 = (0.5 * assign48690_e62463);
        (assign48690_e62464, (0.5 * ((locals.var_temp1_dn4 + (-locals.var_xctmax__blk1330_dn4)) + ((((locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)) * assign48690_e62458) + (assign48690_e62454 * (locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)))) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn6)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn7)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn8)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn9)) / (2.0 * assign48690_e62462)))),)
    } else {
        (locals.var_xct__blk1334, locals.var_xct__blk1334_dn4, locals.var_xct__blk1334_dn6, locals.var_xct__blk1334_dn7, locals.var_xct__blk1334_dn8, locals.var_xct__blk1334_dn9,)
    }
};
        locals.var_xct__blk1334 = assign48690_e62466;
        locals.var_xct__blk1334_dn4 = assign48690_e62466_d_n4;
        locals.var_xct__blk1334_dn6 = assign48690_e62466_d_n6;
        locals.var_xct__blk1334_dn7 = assign48690_e62466_d_n7;
        locals.var_xct__blk1334_dn8 = assign48690_e62466_d_n8;
        locals.var_xct__blk1334_dn9 = assign48690_e62466_d_n9;
        locals.var_xct__blk1334_rv = 0.0;

        let (assign48700_e62480, assign48700_e62480_d_n4, assign48700_e62480_d_n6, assign48700_e62480_d_n7, assign48700_e62480_d_n8, assign48700_e62480_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48700_e62475: f64 = (locals.var_xct__blk1334 / locals.var_xctmax__blk1330);
        let assign48700_e62477: f64 = (assign48700_e62475 + 1.0);
        let assign48700_e62478: f64 = (locals.var_ctg_t * assign48700_e62477);
        (assign48700_e62478, ((locals.var_ctg_t_dn4 * assign48700_e62477) + (locals.var_ctg_t * (((locals.var_xct__blk1334_dn4 * locals.var_xctmax__blk1330) - (locals.var_xct__blk1334 * locals.var_xctmax__blk1330_dn4)) / (locals.var_xctmax__blk1330 * locals.var_xctmax__blk1330)))), (locals.var_ctg_t * (locals.var_xct__blk1334_dn6 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn7 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn8 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn9 / locals.var_xctmax__blk1330)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48700_e62480;
        locals.var_temp2_dn4 = assign48700_e62480_d_n4;
        locals.var_temp2_dn6 = assign48700_e62480_d_n6;
        locals.var_temp2_dn7 = assign48700_e62480_d_n7;
        locals.var_temp2_dn8 = assign48700_e62480_d_n8;
        locals.var_temp2_dn9 = assign48700_e62480_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign48710_e62483: f64 = (-230.25850929940458);
        let assign48710_e62484: f64 = if locals.var_temp2 > assign48710_e62483 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign48710_e62484;
        locals.var_guard1477_rv = 0.0;

        let (assign48720_e62495, assign48720_e62495_d_n4, assign48720_e62495_d_n6, assign48720_e62495_d_n7, assign48720_e62495_d_n8, assign48720_e62495_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign48720_e62493: f64 = (locals.var_temp2).exp();
        (assign48720_e62493, (assign48720_e62493 * locals.var_temp2_dn4), (assign48720_e62493 * locals.var_temp2_dn6), (assign48720_e62493 * locals.var_temp2_dn7), (assign48720_e62493 * locals.var_temp2_dn8), (assign48720_e62493 * locals.var_temp2_dn9),)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48720_e62495;
        locals.var_dctg__blk1335_dn4 = assign48720_e62495_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48720_e62495_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48720_e62495_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48720_e62495_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48720_e62495_d_n9;
        locals.var_dctg__blk1335_rv = 0.0;

        let (assign48730_e62531, assign48730_e62531_d_n4, assign48730_e62531_d_n6, assign48730_e62531_d_n7, assign48730_e62531_d_n8, assign48730_e62531_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign48730_e62507: f64 = (-230.25850929940458);
        let assign48730_e62509: f64 = (assign48730_e62507 - locals.var_temp2);
        let assign48730_e62513: f64 = (-230.25850929940458);
        let assign48730_e62515: f64 = (assign48730_e62513 - locals.var_temp2);
        let assign48730_e62518: f64 = (-230.25850929940458);
        let assign48730_e62520: f64 = (assign48730_e62518 - locals.var_temp2);
        let assign48730_e62522: f64 = (assign48730_e62520 * 0.3333333333333333);
        let assign48730_e62523: f64 = (1.0 + assign48730_e62522);
        let assign48730_e62524: f64 = (assign48730_e62515 * assign48730_e62523);
        let assign48730_e62525: f64 = (0.5 * assign48730_e62524);
        let assign48730_e62526: f64 = (1.0 + assign48730_e62525);
        let assign48730_e62527: f64 = (assign48730_e62509 * assign48730_e62526);
        let assign48730_e62528: f64 = (1.0 + assign48730_e62527);
        let assign48730_e62529: f64 = (1e-100 / assign48730_e62528);
        (assign48730_e62529, (-((1e-100 * (((-locals.var_temp2_dn4) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn4) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn6) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn7) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn8) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn9) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn9) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))),)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48730_e62531;
        locals.var_dctg__blk1335_dn4 = assign48730_e62531_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48730_e62531_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48730_e62531_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48730_e62531_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48730_e62531_d_n9;
        locals.var_dctg__blk1335_rv = 0.0;

        let (assign48740_e62541, assign48740_e62541_d_n4, assign48740_e62541_d_n6, assign48740_e62541_d_n7, assign48740_e62541_d_n8, assign48740_e62541_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48740_e62538: f64 = (locals.var_ct_t * locals.var_dctg__blk1335);
        let assign48740_e62539: f64 = (1.0 + assign48740_e62538);
        (assign48740_e62539, ((locals.var_ct_t_dn4 * locals.var_dctg__blk1335) + (locals.var_ct_t * locals.var_dctg__blk1335_dn4)), (locals.var_ct_t * locals.var_dctg__blk1335_dn6), (locals.var_ct_t * locals.var_dctg__blk1335_dn7), (locals.var_ct_t * locals.var_dctg__blk1335_dn8), (locals.var_ct_t * locals.var_dctg__blk1335_dn9),)
    } else {
        (locals.var_ct_fact__blk1336, locals.var_ct_fact__blk1336_dn4, locals.var_ct_fact__blk1336_dn6, locals.var_ct_fact__blk1336_dn7, locals.var_ct_fact__blk1336_dn8, locals.var_ct_fact__blk1336_dn9,)
    }
};
        locals.var_ct_fact__blk1336 = assign48740_e62541;
        locals.var_ct_fact__blk1336_dn4 = assign48740_e62541_d_n4;
        locals.var_ct_fact__blk1336_dn6 = assign48740_e62541_d_n6;
        locals.var_ct_fact__blk1336_dn7 = assign48740_e62541_d_n7;
        locals.var_ct_fact__blk1336_dn8 = assign48740_e62541_d_n8;
        locals.var_ct_fact__blk1336_dn9 = assign48740_e62541_d_n9;
        locals.var_ct_fact__blk1336_rv = 0.0;

        let (assign48750_e62549, assign48750_e62549_d_n4, assign48750_e62549_d_n6, assign48750_e62549_d_n7, assign48750_e62549_d_n8, assign48750_e62549_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48750_e62547: f64 = (locals.var_phit * locals.var_ct_fact__blk1336);
        (assign48750_e62547, ((locals.var_phit_dn4 * locals.var_ct_fact__blk1336) + (locals.var_phit * locals.var_ct_fact__blk1336_dn4)), (locals.var_phit * locals.var_ct_fact__blk1336_dn6), (locals.var_phit * locals.var_ct_fact__blk1336_dn7), (locals.var_phit * locals.var_ct_fact__blk1336_dn8), (locals.var_phit * locals.var_ct_fact__blk1336_dn9),)
    } else {
        (locals.var_phitct__blk1337, locals.var_phitct__blk1337_dn4, locals.var_phitct__blk1337_dn6, locals.var_phitct__blk1337_dn7, locals.var_phitct__blk1337_dn8, locals.var_phitct__blk1337_dn9,)
    }
};
        locals.var_phitct__blk1337 = assign48750_e62549;
        locals.var_phitct__blk1337_dn4 = assign48750_e62549_d_n4;
        locals.var_phitct__blk1337_dn6 = assign48750_e62549_d_n6;
        locals.var_phitct__blk1337_dn7 = assign48750_e62549_d_n7;
        locals.var_phitct__blk1337_dn8 = assign48750_e62549_d_n8;
        locals.var_phitct__blk1337_dn9 = assign48750_e62549_d_n9;
        locals.var_phitct__blk1337_rv = 0.0;

        let (assign48760_e62567, assign48760_e62567_d_n4, assign48760_e62567_d_n6, assign48760_e62567_d_n7, assign48760_e62567_d_n8, assign48760_e62567_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48760_e62557: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign48760_e62558: f64 = (1.0 + assign48760_e62557);
        let assign48760_e62559: f64 = (locals.var_psce_i * assign48760_e62558);
        let assign48760_e62563: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1323);
        let assign48760_e62564: f64 = (1.0 + assign48760_e62563);
        let assign48760_e62565: f64 = (assign48760_e62559 * assign48760_e62564);
        (assign48760_e62565, (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn4)), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn8)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn8))), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn9)),)
    } else {
        (locals.var_dphit1__blk1338, locals.var_dphit1__blk1338_dn4, locals.var_dphit1__blk1338_dn6, locals.var_dphit1__blk1338_dn7, locals.var_dphit1__blk1338_dn8, locals.var_dphit1__blk1338_dn9,)
    }
};
        locals.var_dphit1__blk1338 = assign48760_e62567;
        locals.var_dphit1__blk1338_dn4 = assign48760_e62567_d_n4;
        locals.var_dphit1__blk1338_dn6 = assign48760_e62567_d_n6;
        locals.var_dphit1__blk1338_dn7 = assign48760_e62567_d_n7;
        locals.var_dphit1__blk1338_dn8 = assign48760_e62567_d_n8;
        locals.var_dphit1__blk1338_dn9 = assign48760_e62567_d_n9;
        locals.var_dphit1__blk1338_rv = 0.0;

        let (assign48770_e62577, assign48770_e62577_d_n4, assign48770_e62577_d_n6, assign48770_e62577_d_n7, assign48770_e62577_d_n8, assign48770_e62577_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48770_e62574: f64 = (1.0 + locals.var_dphit1__blk1338);
        let assign48770_e62575: f64 = (locals.var_phitct__blk1337 * assign48770_e62574);
        (assign48770_e62575, ((locals.var_phitct__blk1337_dn4 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn4)), ((locals.var_phitct__blk1337_dn6 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn6)), ((locals.var_phitct__blk1337_dn7 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn7)), ((locals.var_phitct__blk1337_dn8 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn8)), ((locals.var_phitct__blk1337_dn9 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn9)),)
    } else {
        (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9,)
    }
};
        locals.var_phit1__blk1339 = assign48770_e62577;
        locals.var_phit1__blk1339_dn4 = assign48770_e62577_d_n4;
        locals.var_phit1__blk1339_dn6 = assign48770_e62577_d_n6;
        locals.var_phit1__blk1339_dn7 = assign48770_e62577_d_n7;
        locals.var_phit1__blk1339_dn8 = assign48770_e62577_d_n8;
        locals.var_phit1__blk1339_dn9 = assign48770_e62577_d_n9;
        locals.var_phit1__blk1339_rv = 0.0;

        let (assign48780_e62585, assign48780_e62585_d_n4, assign48780_e62585_d_n6, assign48780_e62585_d_n7, assign48780_e62585_d_n8, assign48780_e62585_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48780_e62583: f64 = (1.0 / locals.var_phit1__blk1339);
        (assign48780_e62583, (-(locals.var_phit1__blk1339_dn4 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn6 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn7 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn8 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn9 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))),)
    } else {
        (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9,)
    }
};
        locals.var_inv_phit1__blk1340 = assign48780_e62585;
        locals.var_inv_phit1__blk1340_dn4 = assign48780_e62585_d_n4;
        locals.var_inv_phit1__blk1340_dn6 = assign48780_e62585_d_n6;
        locals.var_inv_phit1__blk1340_dn7 = assign48780_e62585_d_n7;
        locals.var_inv_phit1__blk1340_dn8 = assign48780_e62585_d_n8;
        locals.var_inv_phit1__blk1340_dn9 = assign48780_e62585_d_n9;
        locals.var_inv_phit1__blk1340_rv = 0.0;

        let (assign48790_e62596, assign48790_e62596_d_n4, assign48790_e62596_d_n6, assign48790_e62596_d_n7, assign48790_e62596_d_n8, assign48790_e62596_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48790_e62592: f64 = (locals.var_phit * locals.var_inv_phit1__blk1340);
        let assign48790_e62593: f64 = (assign48790_e62592).sqrt();
        let assign48790_e62594: f64 = (locals.var_g_0__blk1316 * assign48790_e62593);
        (assign48790_e62594, ((locals.var_g_0__blk1316_dn4 * assign48790_e62593) + (locals.var_g_0__blk1316 * (((locals.var_phit_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phit * locals.var_inv_phit1__blk1340_dn4)) / (2.0 * assign48790_e62593)))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn6) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn7) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn8) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn9) / (2.0 * assign48790_e62593))),)
    } else {
        (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9,)
    }
};
        locals.var_gf__blk1324 = assign48790_e62596;
        locals.var_gf__blk1324_dn4 = assign48790_e62596_d_n4;
        locals.var_gf__blk1324_dn6 = assign48790_e62596_d_n6;
        locals.var_gf__blk1324_dn7 = assign48790_e62596_d_n7;
        locals.var_gf__blk1324_dn8 = assign48790_e62596_d_n8;
        locals.var_gf__blk1324_dn9 = assign48790_e62596_d_n9;
        locals.var_gf__blk1324_rv = 0.0;

        let (assign48800_e62604, assign48800_e62604_d_n4, assign48800_e62604_d_n6, assign48800_e62604_d_n7, assign48800_e62604_d_n8, assign48800_e62604_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48800_e62602: f64 = (locals.var_gf__blk1324 * locals.var_gf__blk1324);
        (assign48800_e62602, ((locals.var_gf__blk1324_dn4 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn4)), ((locals.var_gf__blk1324_dn6 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn6)), ((locals.var_gf__blk1324_dn7 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn7)), ((locals.var_gf__blk1324_dn8 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn8)), ((locals.var_gf__blk1324_dn9 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn9)),)
    } else {
        (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9,)
    }
};
        locals.var_gf2__blk1325 = assign48800_e62604;
        locals.var_gf2__blk1325_dn4 = assign48800_e62604_d_n4;
        locals.var_gf2__blk1325_dn6 = assign48800_e62604_d_n6;
        locals.var_gf2__blk1325_dn7 = assign48800_e62604_d_n7;
        locals.var_gf2__blk1325_dn8 = assign48800_e62604_d_n8;
        locals.var_gf2__blk1325_dn9 = assign48800_e62604_d_n9;
        locals.var_gf2__blk1325_rv = 0.0;

        let (assign48810_e62612, assign48810_e62612_d_n4, assign48810_e62612_d_n6, assign48810_e62612_d_n7, assign48810_e62612_d_n8, assign48810_e62612_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48810_e62610: f64 = (1.0 / locals.var_gf2__blk1325);
        (assign48810_e62610, (-(locals.var_gf2__blk1325_dn4 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn6 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn7 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn8 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn9 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))),)
    } else {
        (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9,)
    }
};
        locals.var_inv_gf2__blk1341 = assign48810_e62612;
        locals.var_inv_gf2__blk1341_dn4 = assign48810_e62612_d_n4;
        locals.var_inv_gf2__blk1341_dn6 = assign48810_e62612_d_n6;
        locals.var_inv_gf2__blk1341_dn7 = assign48810_e62612_d_n7;
        locals.var_inv_gf2__blk1341_dn8 = assign48810_e62612_d_n8;
        locals.var_inv_gf2__blk1341_dn9 = assign48810_e62612_d_n9;
        locals.var_inv_gf2__blk1341_rv = 0.0;

        let (assign48820_e62620, assign48820_e62620_d_n4, assign48820_e62620_d_n6, assign48820_e62620_d_n7, assign48820_e62620_d_n8, assign48820_e62620_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48820_e62618: f64 = (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340);
        (assign48820_e62618, ((locals.var_vsbstar__blk1318_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vsbstar__blk1318_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vsbstar__blk1318_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vsbstar__blk1318_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vsbstar__blk1318_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_ux__blk1342, locals.var_ux__blk1342_dn4, locals.var_ux__blk1342_dn6, locals.var_ux__blk1342_dn7, locals.var_ux__blk1342_dn8, locals.var_ux__blk1342_dn9,)
    }
};
        locals.var_ux__blk1342 = assign48820_e62620;
        locals.var_ux__blk1342_dn4 = assign48820_e62620_d_n4;
        locals.var_ux__blk1342_dn6 = assign48820_e62620_d_n6;
        locals.var_ux__blk1342_dn7 = assign48820_e62620_d_n7;
        locals.var_ux__blk1342_dn8 = assign48820_e62620_d_n8;
        locals.var_ux__blk1342_dn9 = assign48820_e62620_d_n9;
        locals.var_ux__blk1342_rv = 0.0;

        let (assign48830_e62628, assign48830_e62628_d_n4, assign48830_e62628_d_n6, assign48830_e62628_d_n7, assign48830_e62628_d_n8, assign48830_e62628_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48830_e62626: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340);
        (assign48830_e62626, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9,)
    }
};
        locals.var_xg__blk1343 = assign48830_e62628;
        locals.var_xg__blk1343_dn4 = assign48830_e62628_d_n4;
        locals.var_xg__blk1343_dn6 = assign48830_e62628_d_n6;
        locals.var_xg__blk1343_dn7 = assign48830_e62628_d_n7;
        locals.var_xg__blk1343_dn8 = assign48830_e62628_d_n8;
        locals.var_xg__blk1343_dn9 = assign48830_e62628_d_n9;
        locals.var_xg__blk1343_rv = 0.0;

        let (assign48840_e62645, assign48840_e62645_d_n7, assign48840_e62645_d_n8,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48840_e62634: f64 = (2.0 * locals.var_vdsx);
        let assign48840_e62639: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign48840_e62640: f64 = (1.0 + assign48840_e62639);
        let assign48840_e62641: f64 = (assign48840_e62640).sqrt();
        let assign48840_e62642: f64 = (1.0 + assign48840_e62641);
        let assign48840_e62643: f64 = (assign48840_e62634 / assign48840_e62642);
        (assign48840_e62643, ((((2.0 * locals.var_vdsx_dn7) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)), ((((2.0 * locals.var_vdsx_dn8) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn8) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)),)
    } else {
        (locals.var_vdsp__blk1344, locals.var_vdsp__blk1344_dn7, locals.var_vdsp__blk1344_dn8,)
    }
};
        locals.var_vdsp__blk1344 = assign48840_e62645;
        locals.var_vdsp__blk1344_dn7 = assign48840_e62645_d_n7;
        locals.var_vdsp__blk1344_dn8 = assign48840_e62645_d_n8;
        locals.var_vdsp__blk1344_rv = 0.0;

        let (assign48850_e62659, assign48850_e62659_d_n4, assign48850_e62659_d_n6, assign48850_e62659_d_n7, assign48850_e62659_d_n8, assign48850_e62659_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48850_e62651: f64 = (locals.var_cf_i * locals.var_vdsp__blk1344);
        let assign48850_e62655: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1323);
        let assign48850_e62656: f64 = (1.0 + assign48850_e62655);
        let assign48850_e62657: f64 = (assign48850_e62651 * assign48850_e62656);
        (assign48850_e62657, (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn4)), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn7) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn8) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn8))), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn9)),)
    } else {
        (locals.var_delphib__blk1345, locals.var_delphib__blk1345_dn4, locals.var_delphib__blk1345_dn6, locals.var_delphib__blk1345_dn7, locals.var_delphib__blk1345_dn8, locals.var_delphib__blk1345_dn9,)
    }
};
        locals.var_delphib__blk1345 = assign48850_e62659;
        locals.var_delphib__blk1345_dn4 = assign48850_e62659_d_n4;
        locals.var_delphib__blk1345_dn6 = assign48850_e62659_d_n6;
        locals.var_delphib__blk1345_dn7 = assign48850_e62659_d_n7;
        locals.var_delphib__blk1345_dn8 = assign48850_e62659_d_n8;
        locals.var_delphib__blk1345_dn9 = assign48850_e62659_d_n9;
        locals.var_delphib__blk1345_rv = 0.0;

        let (assign48860_e62667, assign48860_e62667_d_n4, assign48860_e62667_d_n6, assign48860_e62667_d_n7, assign48860_e62667_d_n8, assign48860_e62667_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48860_e62665: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340);
        (assign48860_e62665, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn4)), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn6), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn7), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn8), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn9),)
    } else {
        (locals.var_xb__blk1346, locals.var_xb__blk1346_dn4, locals.var_xb__blk1346_dn6, locals.var_xb__blk1346_dn7, locals.var_xb__blk1346_dn8, locals.var_xb__blk1346_dn9,)
    }
};
        locals.var_xb__blk1346 = assign48860_e62667;
        locals.var_xb__blk1346_dn4 = assign48860_e62667_d_n4;
        locals.var_xb__blk1346_dn6 = assign48860_e62667_d_n6;
        locals.var_xb__blk1346_dn7 = assign48860_e62667_d_n7;
        locals.var_xb__blk1346_dn8 = assign48860_e62667_d_n8;
        locals.var_xb__blk1346_dn9 = assign48860_e62667_d_n9;
        locals.var_xb__blk1346_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48870_e62678, assign48870_e62678_d_n4, assign48870_e62678_d_n6, assign48870_e62678_d_n7, assign48870_e62678_d_n8, assign48870_e62678_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48870_e62673: f64 = (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317);
        let assign48870_e62675: f64 = (assign48870_e62673 + locals.var_aphi__blk1315);
        let assign48870_e62676: f64 = (assign48870_e62675).sqrt();
        (assign48870_e62676, ((((locals.var_v_xb__blk1317_dn4 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48870_e62676)), 0.0, (((locals.var_v_xb__blk1317_dn7 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn8 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn9 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48870_e62676)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48870_e62678;
        locals.var_temp1_dn4 = assign48870_e62678_d_n4;
        locals.var_temp1_dn6 = assign48870_e62678_d_n6;
        locals.var_temp1_dn7 = assign48870_e62678_d_n7;
        locals.var_temp1_dn8 = assign48870_e62678_d_n8;
        locals.var_temp1_dn9 = assign48870_e62678_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign48880_e62693, assign48880_e62693_d_n4, assign48880_e62693_d_n6, assign48880_e62693_d_n7, assign48880_e62693_d_n8, assign48880_e62693_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48880_e62684: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
        let assign48880_e62687: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
        let assign48880_e62688: f64 = (assign48880_e62684 * assign48880_e62687);
        let assign48880_e62690: f64 = (assign48880_e62688 + locals.var_aphi__blk1315);
        let assign48880_e62691: f64 = (assign48880_e62690).sqrt();
        (assign48880_e62691, (((((locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4))) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48880_e62691)), ((((-locals.var_delphib__blk1345_dn6) * assign48880_e62687) + (assign48880_e62684 * (-locals.var_delphib__blk1345_dn6))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9))) / (2.0 * assign48880_e62691)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48880_e62693;
        locals.var_temp2_dn4 = assign48880_e62693_d_n4;
        locals.var_temp2_dn6 = assign48880_e62693_d_n6;
        locals.var_temp2_dn7 = assign48880_e62693_d_n7;
        locals.var_temp2_dn8 = assign48880_e62693_d_n8;
        locals.var_temp2_dn9 = assign48880_e62693_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign48890_e62707, assign48890_e62707_d_n4, assign48890_e62707_d_n6, assign48890_e62707_d_n7, assign48890_e62707_d_n8, assign48890_e62707_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48890_e62699: f64 = (0.5 * locals.var_inv_phit1__blk1340);
        let assign48890_e62702: f64 = (locals.var_delphib__blk1345 + locals.var_temp1);
        let assign48890_e62704: f64 = (assign48890_e62702 - locals.var_temp2);
        let assign48890_e62705: f64 = (assign48890_e62699 * assign48890_e62704);
        (assign48890_e62705, (((0.5 * locals.var_inv_phit1__blk1340_dn4) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn4 + locals.var_temp1_dn4) - locals.var_temp2_dn4))), (((0.5 * locals.var_inv_phit1__blk1340_dn6) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1340_dn7) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1340_dn8) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))), (((0.5 * locals.var_inv_phit1__blk1340_dn9) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn9 + locals.var_temp1_dn9) - locals.var_temp2_dn9))),)
    } else {
        (locals.var_delxb__blk1347, locals.var_delxb__blk1347_dn4, locals.var_delxb__blk1347_dn6, locals.var_delxb__blk1347_dn7, locals.var_delxb__blk1347_dn8, locals.var_delxb__blk1347_dn9,)
    }
};
        locals.var_delxb__blk1347 = assign48890_e62707;
        locals.var_delxb__blk1347_dn4 = assign48890_e62707_d_n4;
        locals.var_delxb__blk1347_dn6 = assign48890_e62707_d_n6;
        locals.var_delxb__blk1347_dn7 = assign48890_e62707_d_n7;
        locals.var_delxb__blk1347_dn8 = assign48890_e62707_d_n8;
        locals.var_delxb__blk1347_dn9 = assign48890_e62707_d_n9;
        locals.var_delxb__blk1347_rv = 0.0;

        let (assign48900_e62715, assign48900_e62715_d_n4, assign48900_e62715_d_n6, assign48900_e62715_d_n7, assign48900_e62715_d_n8, assign48900_e62715_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48900_e62713: f64 = (locals.var_xb__blk1346 + locals.var_ux__blk1342);
        (assign48900_e62713, (locals.var_xb__blk1346_dn4 + locals.var_ux__blk1342_dn4), (locals.var_xb__blk1346_dn6 + locals.var_ux__blk1342_dn6), (locals.var_xb__blk1346_dn7 + locals.var_ux__blk1342_dn7), (locals.var_xb__blk1346_dn8 + locals.var_ux__blk1342_dn8), (locals.var_xb__blk1346_dn9 + locals.var_ux__blk1342_dn9),)
    } else {
        (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9,)
    }
};
        locals.var_xno_s__blk1348 = assign48900_e62715;
        locals.var_xno_s__blk1348_dn4 = assign48900_e62715_d_n4;
        locals.var_xno_s__blk1348_dn6 = assign48900_e62715_d_n6;
        locals.var_xno_s__blk1348_dn7 = assign48900_e62715_d_n7;
        locals.var_xno_s__blk1348_dn8 = assign48900_e62715_d_n8;
        locals.var_xno_s__blk1348_dn9 = assign48900_e62715_d_n9;
        locals.var_xno_s__blk1348_rv = 0.0;

        let (assign48910_e62723, assign48910_e62723_d_n4, assign48910_e62723_d_n6, assign48910_e62723_d_n7, assign48910_e62723_d_n8, assign48910_e62723_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48910_e62721: f64 = (locals.var_xno_s__blk1348 - locals.var_delxb__blk1347);
        (assign48910_e62721, (locals.var_xno_s__blk1348_dn4 - locals.var_delxb__blk1347_dn4), (locals.var_xno_s__blk1348_dn6 - locals.var_delxb__blk1347_dn6), (locals.var_xno_s__blk1348_dn7 - locals.var_delxb__blk1347_dn7), (locals.var_xno_s__blk1348_dn8 - locals.var_delxb__blk1347_dn8), (locals.var_xno_s__blk1348_dn9 - locals.var_delxb__blk1347_dn9),)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign48910_e62723;
        locals.var_xn_s__blk1349_dn4 = assign48910_e62723_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign48910_e62723_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign48910_e62723_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign48910_e62723_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign48910_e62723_d_n9;
        locals.var_xn_s__blk1349_rv = 0.0;

        let assign48920_e62726: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign48920_e62726;
        locals.var_guard1478_rv = 0.0;

        let assign48930_e62728: f64 = (locals.var_xn_s__blk1349).abs();
        let assign48930_e62730: f64 = if assign48930_e62728 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign48930_e62730;
        locals.var_guard1479_rv = 0.0;

        let (assign48940_e62754, assign48940_e62754_d_n4, assign48940_e62754_d_n6, assign48940_e62754_d_n7, assign48940_e62754_d_n8, assign48940_e62754_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign48940_e62743: f64 = (0.5 * locals.var_xn_s__blk1349);
        let assign48940_e62747: f64 = (0.3125 * locals.var_xn_s__blk1349);
        let assign48940_e62748: f64 = (1.0 - assign48940_e62747);
        let assign48940_e62749: f64 = (assign48940_e62743 * assign48940_e62748);
        let assign48940_e62750: f64 = (1.0 - assign48940_e62749);
        let assign48940_e62751: f64 = (locals.var_gf__blk1324 * assign48940_e62750);
        let assign48940_e62752: f64 = (1.0 + assign48940_e62751);
        (assign48940_e62752, ((locals.var_gf__blk1324_dn4 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn4) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn4))))))), ((locals.var_gf__blk1324_dn6 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn6) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn6))))))), ((locals.var_gf__blk1324_dn7 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn7) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn7))))))), ((locals.var_gf__blk1324_dn8 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn8) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn8))))))), ((locals.var_gf__blk1324_dn9 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn9) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn9))))))),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign48940_e62754;
        locals.var_nscr__blk1350_dn4 = assign48940_e62754_d_n4;
        locals.var_nscr__blk1350_dn6 = assign48940_e62754_d_n6;
        locals.var_nscr__blk1350_dn7 = assign48940_e62754_d_n7;
        locals.var_nscr__blk1350_dn8 = assign48940_e62754_d_n8;
        locals.var_nscr__blk1350_dn9 = assign48940_e62754_d_n9;
        locals.var_nscr__blk1350_rv = 0.0;

        let assign48950_e62757: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign48950_e62757;
        locals.var_guard1480_rv = 0.0;

        let (assign48960_e62772, assign48960_e62772_d_n4, assign48960_e62772_d_n6, assign48960_e62772_d_n7, assign48960_e62772_d_n8, assign48960_e62772_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 != 0.0)) {
        let assign48960_e62769: f64 = (-locals.var_xn_s__blk1349);
        let assign48960_e62770: f64 = (assign48960_e62769).exp();
        (assign48960_e62770, (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn4)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn6)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn7)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn8)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign48960_e62772;
        locals.var_delta_ns__blk1364_dn4 = assign48960_e62772_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign48960_e62772_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign48960_e62772_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign48960_e62772_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign48960_e62772_d_n9;
        locals.var_delta_ns__blk1364_rv = 0.0;

        let (assign48970_e62808, assign48970_e62808_d_n4, assign48970_e62808_d_n6, assign48970_e62808_d_n7, assign48970_e62808_d_n8, assign48970_e62808_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 == 0.0)) {
        let assign48970_e62788: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62793: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62797: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62799: f64 = (assign48970_e62797 * 0.3333333333333333);
        let assign48970_e62800: f64 = (1.0 + assign48970_e62799);
        let assign48970_e62801: f64 = (assign48970_e62793 * assign48970_e62800);
        let assign48970_e62802: f64 = (0.5 * assign48970_e62801);
        let assign48970_e62803: f64 = (1.0 + assign48970_e62802);
        let assign48970_e62804: f64 = (assign48970_e62788 * assign48970_e62803);
        let assign48970_e62805: f64 = (1.0 + assign48970_e62804);
        let assign48970_e62806: f64 = (1e-200 / assign48970_e62805);
        (assign48970_e62806, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign48970_e62808;
        locals.var_delta_ns__blk1364_dn4 = assign48970_e62808_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign48970_e62808_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign48970_e62808_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign48970_e62808_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign48970_e62808_d_n9;
        locals.var_delta_ns__blk1364_rv = 0.0;

        let (assign48980_e62825, assign48980_e62825_d_n4, assign48980_e62825_d_n6, assign48980_e62825_d_n7, assign48980_e62825_d_n8, assign48980_e62825_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let (assign48980_e62823,) = {
            if (locals.var_xn_s__blk1349 > 0.0) {
                (1.0,)
            } else {
                let assign48980_e62822: f64 = (-1.0);
                (assign48980_e62822,)
            }
        };
        (assign48980_e62823, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48980_e62825;
        locals.var_temp__blk949_dn4 = assign48980_e62825_d_n4;
        locals.var_temp__blk949_dn6 = assign48980_e62825_d_n6;
        locals.var_temp__blk949_dn7 = assign48980_e62825_d_n7;
        locals.var_temp__blk949_dn8 = assign48980_e62825_d_n8;
        locals.var_temp__blk949_dn9 = assign48980_e62825_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign48990_e62857, assign48990_e62857_d_n4, assign48990_e62857_d_n6, assign48990_e62857_d_n7, assign48990_e62857_d_n8, assign48990_e62857_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let assign48990_e62837: f64 = (locals.var_temp__blk949 * locals.var_gf__blk1324);
        let assign48990_e62842: f64 = (1.0 - locals.var_xn_s__blk1349);
        let assign48990_e62843: f64 = (locals.var_delta_ns__blk1364 * assign48990_e62842);
        let assign48990_e62844: f64 = (1.0 - assign48990_e62843);
        let assign48990_e62845: f64 = (assign48990_e62837 * assign48990_e62844);
        let assign48990_e62850: f64 = (1.0 - locals.var_delta_ns__blk1364);
        let assign48990_e62851: f64 = (locals.var_xn_s__blk1349 * assign48990_e62850);
        let assign48990_e62852: f64 = (assign48990_e62851).sqrt();
        let assign48990_e62853: f64 = (2.0 * assign48990_e62852);
        let assign48990_e62854: f64 = (assign48990_e62845 / assign48990_e62853);
        let assign48990_e62855: f64 = (1.0 + assign48990_e62854);
        (assign48990_e62855, (((((((locals.var_temp__blk949_dn4 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn4)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn4 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn4)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn4 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn4))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn6 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn6)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn6 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn6)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn6 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn6))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn7 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn7)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn7 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn7)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn7 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn7))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn8 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn8)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn8 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn8)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn8 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn8))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn9 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn9)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn9 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn9)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn9 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn9))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign48990_e62857;
        locals.var_nscr__blk1350_dn4 = assign48990_e62857_d_n4;
        locals.var_nscr__blk1350_dn6 = assign48990_e62857_d_n6;
        locals.var_nscr__blk1350_dn7 = assign48990_e62857_d_n7;
        locals.var_nscr__blk1350_dn8 = assign48990_e62857_d_n8;
        locals.var_nscr__blk1350_dn9 = assign48990_e62857_d_n9;
        locals.var_nscr__blk1350_rv = 0.0;

        let (assign49000_e62873, assign49000_e62873_d_n4, assign49000_e62873_d_n6, assign49000_e62873_d_n7, assign49000_e62873_d_n8, assign49000_e62873_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 == 0.0)) {
        let assign49000_e62867: f64 = (0.5 * locals.var_gf__blk1324);
        let assign49000_e62869: f64 = (locals.var_xn_s__blk1349).sqrt();
        let assign49000_e62870: f64 = (assign49000_e62867 / assign49000_e62869);
        let assign49000_e62871: f64 = (1.0 + assign49000_e62870);
        (assign49000_e62871, ((((0.5 * locals.var_gf__blk1324_dn4) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn6) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn7) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn8) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn9) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign49000_e62873;
        locals.var_nscr__blk1350_dn4 = assign49000_e62873_d_n4;
        locals.var_nscr__blk1350_dn6 = assign49000_e62873_d_n6;
        locals.var_nscr__blk1350_dn7 = assign49000_e62873_d_n7;
        locals.var_nscr__blk1350_dn8 = assign49000_e62873_d_n8;
        locals.var_nscr__blk1350_dn9 = assign49000_e62873_d_n9;
        locals.var_nscr__blk1350_rv = 0.0;

        let (assign49010_e62891, assign49010_e62891_d_n4, assign49010_e62891_d_n6, assign49010_e62891_d_n7, assign49010_e62891_d_n8, assign49010_e62891_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49010_e62880: f64 = (locals.var_xn_s__blk1349).sqrt();
        let assign49010_e62881: f64 = (locals.var_gf__blk1324 * assign49010_e62880);
        let assign49010_e62882: f64 = (locals.var_xn_s__blk1349 + assign49010_e62881);
        let assign49010_e62886: f64 = (locals.var_nscr__blk1350 - 1.0);
        let assign49010_e62887: f64 = (assign49010_e62886).ln();
        let assign49010_e62888: f64 = (locals.var_nscr__blk1350 * assign49010_e62887);
        let assign49010_e62889: f64 = (assign49010_e62882 - assign49010_e62888);
        (assign49010_e62889, ((locals.var_xn_s__blk1349_dn4 + ((locals.var_gf__blk1324_dn4 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn4 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn4 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn6 + ((locals.var_gf__blk1324_dn6 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn6 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn6 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn7 + ((locals.var_gf__blk1324_dn7 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn7 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn7 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn8 + ((locals.var_gf__blk1324_dn8 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn8 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn8 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn9 + ((locals.var_gf__blk1324_dn9 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn9 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn9 / assign49010_e62886)))),)
    } else {
        (locals.var_xthscr__blk1351, locals.var_xthscr__blk1351_dn4, locals.var_xthscr__blk1351_dn6, locals.var_xthscr__blk1351_dn7, locals.var_xthscr__blk1351_dn8, locals.var_xthscr__blk1351_dn9,)
    }
};
        locals.var_xthscr__blk1351 = assign49010_e62891;
        locals.var_xthscr__blk1351_dn4 = assign49010_e62891_d_n4;
        locals.var_xthscr__blk1351_dn6 = assign49010_e62891_d_n6;
        locals.var_xthscr__blk1351_dn7 = assign49010_e62891_d_n7;
        locals.var_xthscr__blk1351_dn8 = assign49010_e62891_d_n8;
        locals.var_xthscr__blk1351_dn9 = assign49010_e62891_d_n9;
        locals.var_xthscr__blk1351_rv = 0.0;

        let (assign49020_e62901, assign49020_e62901_d_n4, assign49020_e62901_d_n6, assign49020_e62901_d_n7, assign49020_e62901_d_n8, assign49020_e62901_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49020_e62897: f64 = (locals.var_xg__blk1343 - locals.var_xthscr__blk1351);
        let assign49020_e62899: f64 = (assign49020_e62897 / locals.var_nscr__blk1350);
        (assign49020_e62899, ((((locals.var_xg__blk1343_dn4 - locals.var_xthscr__blk1351_dn4) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn6 - locals.var_xthscr__blk1351_dn6) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn7 - locals.var_xthscr__blk1351_dn7) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn8 - locals.var_xthscr__blk1351_dn8) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn9 - locals.var_xthscr__blk1351_dn9) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)),)
    } else {
        (locals.var_xgtscr__blk1352, locals.var_xgtscr__blk1352_dn4, locals.var_xgtscr__blk1352_dn6, locals.var_xgtscr__blk1352_dn7, locals.var_xgtscr__blk1352_dn8, locals.var_xgtscr__blk1352_dn9,)
    }
};
        locals.var_xgtscr__blk1352 = assign49020_e62901;
        locals.var_xgtscr__blk1352_dn4 = assign49020_e62901_d_n4;
        locals.var_xgtscr__blk1352_dn6 = assign49020_e62901_d_n6;
        locals.var_xgtscr__blk1352_dn7 = assign49020_e62901_d_n7;
        locals.var_xgtscr__blk1352_dn8 = assign49020_e62901_d_n8;
        locals.var_xgtscr__blk1352_dn9 = assign49020_e62901_d_n9;
        locals.var_xgtscr__blk1352_rv = 0.0;

        let (assign49030_e62918, assign49030_e62918_d_n4, assign49030_e62918_d_n6, assign49030_e62918_d_n7, assign49030_e62918_d_n8, assign49030_e62918_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49030_e62907: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign49030_e62911: f64 = (8.0 / locals.var_gf2__blk1325);
        let assign49030_e62912: f64 = (1.0 + assign49030_e62911);
        let assign49030_e62913: f64 = (assign49030_e62912).sqrt();
        let assign49030_e62915: f64 = (assign49030_e62913 - 1.0);
        let assign49030_e62916: f64 = (assign49030_e62907 * assign49030_e62915);
        (assign49030_e62916, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))),)
    } else {
        (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9,)
    }
};
        locals.var_qbscr__blk1358 = assign49030_e62918;
        locals.var_qbscr__blk1358_dn4 = assign49030_e62918_d_n4;
        locals.var_qbscr__blk1358_dn6 = assign49030_e62918_d_n6;
        locals.var_qbscr__blk1358_dn7 = assign49030_e62918_d_n7;
        locals.var_qbscr__blk1358_dn8 = assign49030_e62918_d_n8;
        locals.var_qbscr__blk1358_dn9 = assign49030_e62918_d_n9;
        locals.var_qbscr__blk1358_rv = 0.0;

        let (assign49040_e62924, assign49040_e62924_d_n4, assign49040_e62924_d_n6, assign49040_e62924_d_n7, assign49040_e62924_d_n8, assign49040_e62924_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49040_e62924;
        locals.var_qiscr__blk1357_dn4 = assign49040_e62924_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49040_e62924_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49040_e62924_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49040_e62924_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49040_e62924_d_n9;
        locals.var_qiscr__blk1357_rv = 0.0;

        let (assign49050_e62930, assign49050_e62930_d_n4, assign49050_e62930_d_n6, assign49050_e62930_d_n7, assign49050_e62930_d_n8, assign49050_e62930_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9,)
    }
};
        locals.var_fscr__blk1359 = assign49050_e62930;
        locals.var_fscr__blk1359_dn4 = assign49050_e62930_d_n4;
        locals.var_fscr__blk1359_dn6 = assign49050_e62930_d_n6;
        locals.var_fscr__blk1359_dn7 = assign49050_e62930_d_n7;
        locals.var_fscr__blk1359_dn8 = assign49050_e62930_d_n8;
        locals.var_fscr__blk1359_dn9 = assign49050_e62930_d_n9;
        locals.var_fscr__blk1359_rv = 0.0;

        let assign49060_e62933: f64 = (-30.0);
        let assign49060_e62934: f64 = if locals.var_xgtscr__blk1352 > assign49060_e62933 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign49060_e62934;
        locals.var_guard1481_rv = 0.0;

        let (assign49070_e62946, assign49070_e62946_d_n4, assign49070_e62946_d_n6, assign49070_e62946_d_n7, assign49070_e62946_d_n8, assign49070_e62946_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49070_e62942: f64 = (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352);
        let assign49070_e62944: f64 = (assign49070_e62942 - 1.0);
        (assign49070_e62944, ((locals.var_nscr__blk1350_dn4 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn4)), ((locals.var_nscr__blk1350_dn6 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn6)), ((locals.var_nscr__blk1350_dn7 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn7)), ((locals.var_nscr__blk1350_dn8 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn8)), ((locals.var_nscr__blk1350_dn9 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn9)),)
    } else {
        (locals.var_xgtscr0__blk1353, locals.var_xgtscr0__blk1353_dn4, locals.var_xgtscr0__blk1353_dn6, locals.var_xgtscr0__blk1353_dn7, locals.var_xgtscr0__blk1353_dn8, locals.var_xgtscr0__blk1353_dn9,)
    }
};
        locals.var_xgtscr0__blk1353 = assign49070_e62946;
        locals.var_xgtscr0__blk1353_dn4 = assign49070_e62946_d_n4;
        locals.var_xgtscr0__blk1353_dn6 = assign49070_e62946_d_n6;
        locals.var_xgtscr0__blk1353_dn7 = assign49070_e62946_d_n7;
        locals.var_xgtscr0__blk1353_dn8 = assign49070_e62946_d_n8;
        locals.var_xgtscr0__blk1353_dn9 = assign49070_e62946_d_n9;
        locals.var_xgtscr0__blk1353_rv = 0.0;

        let (assign49080_e62963, assign49080_e62963_d_n4, assign49080_e62963_d_n6, assign49080_e62963_d_n7, assign49080_e62963_d_n8, assign49080_e62963_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49080_e62956: f64 = (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353);
        let assign49080_e62958: f64 = (assign49080_e62956 + 10.0);
        let assign49080_e62959: f64 = (assign49080_e62958).sqrt();
        let assign49080_e62960: f64 = (locals.var_xgtscr0__blk1353 + assign49080_e62959);
        let assign49080_e62961: f64 = (0.5 * assign49080_e62960);
        (assign49080_e62961, (0.5 * (locals.var_xgtscr0__blk1353_dn4 + (((locals.var_xgtscr0__blk1353_dn4 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn4)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn6 + (((locals.var_xgtscr0__blk1353_dn6 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn6)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn7 + (((locals.var_xgtscr0__blk1353_dn7 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn7)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn8 + (((locals.var_xgtscr0__blk1353_dn8 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn8)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn9 + (((locals.var_xgtscr0__blk1353_dn9 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn9)) / (2.0 * assign49080_e62959)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49080_e62963;
        locals.var_temp__blk949_dn4 = assign49080_e62963_d_n4;
        locals.var_temp__blk949_dn6 = assign49080_e62963_d_n6;
        locals.var_temp__blk949_dn7 = assign49080_e62963_d_n7;
        locals.var_temp__blk949_dn8 = assign49080_e62963_d_n8;
        locals.var_temp__blk949_dn9 = assign49080_e62963_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign49090_e62974, assign49090_e62974_d_n4, assign49090_e62974_d_n6, assign49090_e62974_d_n7, assign49090_e62974_d_n8, assign49090_e62974_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49090_e62971: f64 = (locals.var_temp__blk949).ln();
        let assign49090_e62972: f64 = (locals.var_xgtscr__blk1352 - assign49090_e62971);
        (assign49090_e62972, (locals.var_xgtscr__blk1352_dn4 - (locals.var_temp__blk949_dn4 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn6 - (locals.var_temp__blk949_dn6 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn7 - (locals.var_temp__blk949_dn7 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn8 - (locals.var_temp__blk949_dn8 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn9 - (locals.var_temp__blk949_dn9 / locals.var_temp__blk949)),)
    } else {
        (locals.var_qiscr0si__blk1354, locals.var_qiscr0si__blk1354_dn4, locals.var_qiscr0si__blk1354_dn6, locals.var_qiscr0si__blk1354_dn7, locals.var_qiscr0si__blk1354_dn8, locals.var_qiscr0si__blk1354_dn9,)
    }
};
        locals.var_qiscr0si__blk1354 = assign49090_e62974;
        locals.var_qiscr0si__blk1354_dn4 = assign49090_e62974_d_n4;
        locals.var_qiscr0si__blk1354_dn6 = assign49090_e62974_d_n6;
        locals.var_qiscr0si__blk1354_dn7 = assign49090_e62974_d_n7;
        locals.var_qiscr0si__blk1354_dn8 = assign49090_e62974_d_n8;
        locals.var_qiscr0si__blk1354_dn9 = assign49090_e62974_d_n9;
        locals.var_qiscr0si__blk1354_rv = 0.0;

        let (assign49100_e62991, assign49100_e62991_d_n4, assign49100_e62991_d_n6, assign49100_e62991_d_n7, assign49100_e62991_d_n8, assign49100_e62991_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49100_e62984: f64 = (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354);
        let assign49100_e62986: f64 = (assign49100_e62984 + 2.0);
        let assign49100_e62987: f64 = (assign49100_e62986).sqrt();
        let assign49100_e62988: f64 = (locals.var_qiscr0si__blk1354 + assign49100_e62987);
        let assign49100_e62989: f64 = (0.5 * assign49100_e62988);
        (assign49100_e62989, (0.5 * (locals.var_qiscr0si__blk1354_dn4 + (((locals.var_qiscr0si__blk1354_dn4 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn4)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn6 + (((locals.var_qiscr0si__blk1354_dn6 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn6)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn7 + (((locals.var_qiscr0si__blk1354_dn7 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn7)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn8 + (((locals.var_qiscr0si__blk1354_dn8 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn8)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn9 + (((locals.var_qiscr0si__blk1354_dn9 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn9)) / (2.0 * assign49100_e62987)))),)
    } else {
        (locals.var_qiscr0__blk1355, locals.var_qiscr0__blk1355_dn4, locals.var_qiscr0__blk1355_dn6, locals.var_qiscr0__blk1355_dn7, locals.var_qiscr0__blk1355_dn8, locals.var_qiscr0__blk1355_dn9,)
    }
};
        locals.var_qiscr0__blk1355 = assign49100_e62991;
        locals.var_qiscr0__blk1355_dn4 = assign49100_e62991_d_n4;
        locals.var_qiscr0__blk1355_dn6 = assign49100_e62991_d_n6;
        locals.var_qiscr0__blk1355_dn7 = assign49100_e62991_d_n7;
        locals.var_qiscr0__blk1355_dn8 = assign49100_e62991_d_n8;
        locals.var_qiscr0__blk1355_dn9 = assign49100_e62991_d_n9;
        locals.var_qiscr0__blk1355_rv = 0.0;

        let assign49110_e62994: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49110_e62996: f64 = if assign49110_e62994 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign49110_e62996;
        locals.var_guard1482_rv = 0.0;

        let (assign49120_e63009, assign49120_e63009_d_n4, assign49120_e63009_d_n6, assign49120_e63009_d_n7, assign49120_e63009_d_n8, assign49120_e63009_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign49120_e63006: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49120_e63007: f64 = (assign49120_e63006).exp();
        (assign49120_e63007, (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49120_e63009;
        locals.var_temp__blk949_dn4 = assign49120_e63009_d_n4;
        locals.var_temp__blk949_dn6 = assign49120_e63009_d_n6;
        locals.var_temp__blk949_dn7 = assign49120_e63009_d_n7;
        locals.var_temp__blk949_dn8 = assign49120_e63009_d_n8;
        locals.var_temp__blk949_dn9 = assign49120_e63009_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign49130_e63048, assign49130_e63048_d_n4, assign49130_e63048_d_n6, assign49130_e63048_d_n7, assign49130_e63048_d_n8, assign49130_e63048_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        let assign49130_e63022: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63024: f64 = (assign49130_e63022 - 230.25850929940458);
        let assign49130_e63029: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63031: f64 = (assign49130_e63029 - 230.25850929940458);
        let assign49130_e63035: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63037: f64 = (assign49130_e63035 - 230.25850929940458);
        let assign49130_e63039: f64 = (assign49130_e63037 * 0.3333333333333333);
        let assign49130_e63040: f64 = (1.0 + assign49130_e63039);
        let assign49130_e63041: f64 = (assign49130_e63031 * assign49130_e63040);
        let assign49130_e63042: f64 = (0.5 * assign49130_e63041);
        let assign49130_e63043: f64 = (1.0 + assign49130_e63042);
        let assign49130_e63044: f64 = (assign49130_e63024 * assign49130_e63043);
        let assign49130_e63045: f64 = (1.0 + assign49130_e63044);
        let assign49130_e63046: f64 = (1e100 * assign49130_e63045);
        (assign49130_e63046, (1e100 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49130_e63048;
        locals.var_temp__blk949_dn4 = assign49130_e63048_d_n4;
        locals.var_temp__blk949_dn6 = assign49130_e63048_d_n6;
        locals.var_temp__blk949_dn7 = assign49130_e63048_d_n7;
        locals.var_temp__blk949_dn8 = assign49130_e63048_d_n8;
        locals.var_temp__blk949_dn9 = assign49130_e63048_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign49140_e63058, assign49140_e63058_d_n4, assign49140_e63058_d_n6, assign49140_e63058_d_n7, assign49140_e63058_d_n8, assign49140_e63058_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49140_e63056: f64 = (locals.var_temp__blk949 / locals.var_nscr__blk1350);
        (assign49140_e63056, (((locals.var_temp__blk949_dn4 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn6 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn7 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn8 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn9 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)),)
    } else {
        (locals.var_dscr0__blk1356, locals.var_dscr0__blk1356_dn4, locals.var_dscr0__blk1356_dn6, locals.var_dscr0__blk1356_dn7, locals.var_dscr0__blk1356_dn8, locals.var_dscr0__blk1356_dn9,)
    }
};
        locals.var_dscr0__blk1356 = assign49140_e63058;
        locals.var_dscr0__blk1356_dn4 = assign49140_e63058_d_n4;
        locals.var_dscr0__blk1356_dn6 = assign49140_e63058_d_n6;
        locals.var_dscr0__blk1356_dn7 = assign49140_e63058_d_n7;
        locals.var_dscr0__blk1356_dn8 = assign49140_e63058_d_n8;
        locals.var_dscr0__blk1356_dn9 = assign49140_e63058_d_n9;
        locals.var_dscr0__blk1356_rv = 0.0;

        let (assign49150_e63072, assign49150_e63072_d_n4, assign49150_e63072_d_n6, assign49150_e63072_d_n7, assign49150_e63072_d_n8, assign49150_e63072_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49150_e63067: f64 = (locals.var_qiscr0__blk1355 + 1.0);
        let assign49150_e63068: f64 = (2.0 * assign49150_e63067);
        let assign49150_e63070: f64 = (assign49150_e63068 - locals.var_dscr0__blk1356);
        (assign49150_e63070, ((2.0 * locals.var_qiscr0__blk1355_dn4) - locals.var_dscr0__blk1356_dn4), ((2.0 * locals.var_qiscr0__blk1355_dn6) - locals.var_dscr0__blk1356_dn6), ((2.0 * locals.var_qiscr0__blk1355_dn7) - locals.var_dscr0__blk1356_dn7), ((2.0 * locals.var_qiscr0__blk1355_dn8) - locals.var_dscr0__blk1356_dn8), ((2.0 * locals.var_qiscr0__blk1355_dn9) - locals.var_dscr0__blk1356_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49150_e63072;
        locals.var_temp__blk949_dn4 = assign49150_e63072_d_n4;
        locals.var_temp__blk949_dn6 = assign49150_e63072_d_n6;
        locals.var_temp__blk949_dn7 = assign49150_e63072_d_n7;
        locals.var_temp__blk949_dn8 = assign49150_e63072_d_n8;
        locals.var_temp__blk949_dn9 = assign49150_e63072_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign49160_e63075: f64 = if locals.var_dscr0__blk1356 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign49160_e63075;
        locals.var_guard1483_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign49170_e63100, assign49170_e63100_d_n4, assign49170_e63100_d_n6, assign49170_e63100_d_n7, assign49170_e63100_d_n8, assign49170_e63100_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 != 0.0)) {
        let assign49170_e63088: f64 = (locals.var_dscr0__blk1356 * locals.var_temp__blk949);
        let assign49170_e63089: f64 = (1.0 + assign49170_e63088);
        let assign49170_e63090: f64 = (assign49170_e63089).sqrt();
        let assign49170_e63092: f64 = (assign49170_e63090 - 1.0);
        let assign49170_e63094: f64 = (assign49170_e63092 / locals.var_dscr0__blk1356);
        let assign49170_e63095: f64 = (locals.var_qiscr0__blk1355 - assign49170_e63094);
        let assign49170_e63097: f64 = (assign49170_e63095 + 1.0);
        let assign49170_e63098: f64 = (locals.var_nscr__blk1350 * assign49170_e63097);
        (assign49170_e63098, ((locals.var_nscr__blk1350_dn4 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn4 - ((((((locals.var_dscr0__blk1356_dn4 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn4)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn4)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn6 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn6 - ((((((locals.var_dscr0__blk1356_dn6 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn6)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn6)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn7 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn7 - ((((((locals.var_dscr0__blk1356_dn7 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn7)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn7)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn8 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn8 - ((((((locals.var_dscr0__blk1356_dn8 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn8)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn8)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn9 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn9 - ((((((locals.var_dscr0__blk1356_dn9 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn9)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn9)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))),)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49170_e63100;
        locals.var_qiscr__blk1357_dn4 = assign49170_e63100_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49170_e63100_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49170_e63100_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49170_e63100_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49170_e63100_d_n9;
        locals.var_qiscr__blk1357_rv = 0.0;

        let (assign49180_e63123, assign49180_e63123_d_n4, assign49180_e63123_d_n6, assign49180_e63123_d_n7, assign49180_e63123_d_n8, assign49180_e63123_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 == 0.0)) {
        let assign49180_e63111: f64 = (locals.var_nscr__blk1350 * 0.5);
        let assign49180_e63113: f64 = (assign49180_e63111 * locals.var_dscr0__blk1356);
        let assign49180_e63117: f64 = (0.25 * locals.var_temp__blk949);
        let assign49180_e63119: f64 = (assign49180_e63117 * locals.var_temp__blk949);
        let assign49180_e63120: f64 = (1.0 + assign49180_e63119);
        let assign49180_e63121: f64 = (assign49180_e63113 * assign49180_e63120);
        (assign49180_e63121, (((((locals.var_nscr__blk1350_dn4 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn4)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn4)))), (((((locals.var_nscr__blk1350_dn6 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn6)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn6)))), (((((locals.var_nscr__blk1350_dn7 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn7)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn7)))), (((((locals.var_nscr__blk1350_dn8 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn8)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn8)))), (((((locals.var_nscr__blk1350_dn9 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn9)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn9)))),)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49180_e63123;
        locals.var_qiscr__blk1357_dn4 = assign49180_e63123_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49180_e63123_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49180_e63123_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49180_e63123_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49180_e63123_d_n9;
        locals.var_qiscr__blk1357_rv = 0.0;

        let (assign49190_e63152, assign49190_e63152_d_n4, assign49190_e63152_d_n6, assign49190_e63152_d_n7, assign49190_e63152_d_n8, assign49190_e63152_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49190_e63132: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63134: f64 = (assign49190_e63132 + 2.0);
        let assign49190_e63137: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63139: f64 = (assign49190_e63137 - 2.0);
        let assign49190_e63142: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63144: f64 = (assign49190_e63142 - 2.0);
        let assign49190_e63145: f64 = (assign49190_e63139 * assign49190_e63144);
        let assign49190_e63147: f64 = (assign49190_e63145 + 1.0);
        let assign49190_e63148: f64 = (assign49190_e63147).sqrt();
        let assign49190_e63149: f64 = (assign49190_e63134 + assign49190_e63148);
        let assign49190_e63150: f64 = (0.5 * assign49190_e63149);
        (assign49190_e63150, (0.5 * ((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) + ((((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) + ((((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) + ((((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) + ((((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) + ((((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9))) / (2.0 * assign49190_e63148)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49190_e63152;
        locals.var_temp__blk949_dn4 = assign49190_e63152_d_n4;
        locals.var_temp__blk949_dn6 = assign49190_e63152_d_n6;
        locals.var_temp__blk949_dn7 = assign49190_e63152_d_n7;
        locals.var_temp__blk949_dn8 = assign49190_e63152_d_n8;
        locals.var_temp__blk949_dn9 = assign49190_e63152_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let (assign49200_e63173, assign49200_e63173_d_n4, assign49200_e63173_d_n6, assign49200_e63173_d_n7, assign49200_e63173_d_n8, assign49200_e63173_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49200_e63160: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign49200_e63164: f64 = (4.0 / locals.var_gf2__blk1325);
        let assign49200_e63166: f64 = (assign49200_e63164 * locals.var_temp__blk949);
        let assign49200_e63167: f64 = (1.0 + assign49200_e63166);
        let assign49200_e63168: f64 = (assign49200_e63167).sqrt();
        let assign49200_e63170: f64 = (assign49200_e63168 - 1.0);
        let assign49200_e63171: f64 = (assign49200_e63160 * assign49200_e63170);
        (assign49200_e63171, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn4)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn6)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn7)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn8)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn9)) / (2.0 * assign49200_e63168)))),)
    } else {
        (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9,)
    }
};
        locals.var_qbscr__blk1358 = assign49200_e63173;
        locals.var_qbscr__blk1358_dn4 = assign49200_e63173_d_n4;
        locals.var_qbscr__blk1358_dn6 = assign49200_e63173_d_n6;
        locals.var_qbscr__blk1358_dn7 = assign49200_e63173_d_n7;
        locals.var_qbscr__blk1358_dn8 = assign49200_e63173_d_n8;
        locals.var_qbscr__blk1358_dn9 = assign49200_e63173_d_n9;
        locals.var_qbscr__blk1358_rv = 0.0;

        let (assign49210_e63185, assign49210_e63185_d_n4, assign49210_e63185_d_n6, assign49210_e63185_d_n7, assign49210_e63185_d_n8, assign49210_e63185_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49210_e63182: f64 = (locals.var_qbscr__blk1358 + locals.var_qiscr__blk1357);
        let assign49210_e63183: f64 = (locals.var_qbscr__blk1358 / assign49210_e63182);
        (assign49210_e63183, (((locals.var_qbscr__blk1358_dn4 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn4 + locals.var_qiscr__blk1357_dn4))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn6 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn6 + locals.var_qiscr__blk1357_dn6))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn7 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn7 + locals.var_qiscr__blk1357_dn7))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn8 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn8 + locals.var_qiscr__blk1357_dn8))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn9 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn9 + locals.var_qiscr__blk1357_dn9))) / (assign49210_e63182 * assign49210_e63182)),)
    } else {
        (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9,)
    }
};
        locals.var_fscr__blk1359 = assign49210_e63185;
        locals.var_fscr__blk1359_dn4 = assign49210_e63185_d_n4;
        locals.var_fscr__blk1359_dn6 = assign49210_e63185_d_n6;
        locals.var_fscr__blk1359_dn7 = assign49210_e63185_d_n7;
        locals.var_fscr__blk1359_dn8 = assign49210_e63185_d_n8;
        locals.var_fscr__blk1359_dn9 = assign49210_e63185_d_n9;
        locals.var_fscr__blk1359_rv = 0.0;

        let (assign49220_e63197, assign49220_e63197_d_n4, assign49220_e63197_d_n6, assign49220_e63197_d_n7, assign49220_e63197_d_n8, assign49220_e63197_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49220_e63194: f64 = (locals.var_fscr__blk1359 * locals.var_delxb__blk1347);
        let assign49220_e63195: f64 = (locals.var_xno_s__blk1348 - assign49220_e63194);
        (assign49220_e63195, (locals.var_xno_s__blk1348_dn4 - ((locals.var_fscr__blk1359_dn4 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn4))), (locals.var_xno_s__blk1348_dn6 - ((locals.var_fscr__blk1359_dn6 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn6))), (locals.var_xno_s__blk1348_dn7 - ((locals.var_fscr__blk1359_dn7 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn7))), (locals.var_xno_s__blk1348_dn8 - ((locals.var_fscr__blk1359_dn8 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn8))), (locals.var_xno_s__blk1348_dn9 - ((locals.var_fscr__blk1359_dn9 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn9))),)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign49220_e63197;
        locals.var_xn_s__blk1349_dn4 = assign49220_e63197_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign49220_e63197_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign49220_e63197_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign49220_e63197_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign49220_e63197_d_n9;
        locals.var_xn_s__blk1349_rv = 0.0;

        let (assign49230_e63207, assign49230_e63207_d_n4, assign49230_e63207_d_n6, assign49230_e63207_d_n7, assign49230_e63207_d_n8, assign49230_e63207_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49230_e63204: f64 = (locals.var_gf__blk1324 * 0.7071067811865475);
        let assign49230_e63205: f64 = (1.0 + assign49230_e63204);
        (assign49230_e63205, (locals.var_gf__blk1324_dn4 * 0.7071067811865475), (locals.var_gf__blk1324_dn6 * 0.7071067811865475), (locals.var_gf__blk1324_dn7 * 0.7071067811865475), (locals.var_gf__blk1324_dn8 * 0.7071067811865475), (locals.var_gf__blk1324_dn9 * 0.7071067811865475),)
    } else {
        (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9,)
    }
};
        locals.var_xi__blk1360 = assign49230_e63207;
        locals.var_xi__blk1360_dn4 = assign49230_e63207_d_n4;
        locals.var_xi__blk1360_dn6 = assign49230_e63207_d_n6;
        locals.var_xi__blk1360_dn7 = assign49230_e63207_d_n7;
        locals.var_xi__blk1360_dn8 = assign49230_e63207_d_n8;
        locals.var_xi__blk1360_dn9 = assign49230_e63207_d_n9;
        locals.var_xi__blk1360_rv = 0.0;

        let (assign49240_e63215, assign49240_e63215_d_n4, assign49240_e63215_d_n6, assign49240_e63215_d_n7, assign49240_e63215_d_n8, assign49240_e63215_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49240_e63213: f64 = (1e-5 * locals.var_xi__blk1360);
        (assign49240_e63213, (1e-5 * locals.var_xi__blk1360_dn4), (1e-5 * locals.var_xi__blk1360_dn6), (1e-5 * locals.var_xi__blk1360_dn7), (1e-5 * locals.var_xi__blk1360_dn8), (1e-5 * locals.var_xi__blk1360_dn9),)
    } else {
        (locals.var_margin__blk1361, locals.var_margin__blk1361_dn4, locals.var_margin__blk1361_dn6, locals.var_margin__blk1361_dn7, locals.var_margin__blk1361_dn8, locals.var_margin__blk1361_dn9,)
    }
};
        locals.var_margin__blk1361 = assign49240_e63215;
        locals.var_margin__blk1361_dn4 = assign49240_e63215_d_n4;
        locals.var_margin__blk1361_dn6 = assign49240_e63215_d_n6;
        locals.var_margin__blk1361_dn7 = assign49240_e63215_d_n7;
        locals.var_margin__blk1361_dn8 = assign49240_e63215_d_n8;
        locals.var_margin__blk1361_dn9 = assign49240_e63215_d_n9;
        locals.var_margin__blk1361_rv = 0.0;

        let (assign49250_e63223, assign49250_e63223_d_n4, assign49250_e63223_d_n6, assign49250_e63223_d_n7, assign49250_e63223_d_n8, assign49250_e63223_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49250_e63221: f64 = (1.0 / locals.var_xi__blk1360);
        (assign49250_e63221, (-(locals.var_xi__blk1360_dn4 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn6 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn7 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn8 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn9 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))),)
    } else {
        (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9,)
    }
};
        locals.var_inv_xi__blk1362 = assign49250_e63223;
        locals.var_inv_xi__blk1362_dn4 = assign49250_e63223_d_n4;
        locals.var_inv_xi__blk1362_dn6 = assign49250_e63223_d_n6;
        locals.var_inv_xi__blk1362_dn7 = assign49250_e63223_d_n7;
        locals.var_inv_xi__blk1362_dn8 = assign49250_e63223_d_n8;
        locals.var_inv_xi__blk1362_dn9 = assign49250_e63223_d_n9;
        locals.var_inv_xi__blk1362_rv = 0.0;

        let (assign49260_e63229, assign49260_e63229_d_n4, assign49260_e63229_d_n6, assign49260_e63229_d_n7, assign49260_e63229_d_n8, assign49260_e63229_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign49260_e63229;
        locals.var_sp_s_x1__blk1469_dn4 = assign49260_e63229_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign49260_e63229_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign49260_e63229_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign49260_e63229_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign49260_e63229_d_n9;
        locals.var_sp_s_x1__blk1469_rv = 0.0;

        let (assign49270_e63235, assign49270_e63235_d_n4, assign49270_e63235_d_n6, assign49270_e63235_d_n7, assign49270_e63235_d_n8, assign49270_e63235_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49270_e63235;
        locals.var_x_s__blk1363_dn4 = assign49270_e63235_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49270_e63235_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49270_e63235_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49270_e63235_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49270_e63235_d_n9;
        locals.var_x_s__blk1363_rv = 0.0;

        let assign49280_e63238: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign49280_e63238;
        locals.var_guard1484_rv = 0.0;

        let (assign49290_e63248, assign49290_e63248_d_n4, assign49290_e63248_d_n6, assign49290_e63248_d_n7, assign49290_e63248_d_n8, assign49290_e63248_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign49290_e63245: f64 = (-locals.var_xn_s__blk1349);
        let assign49290_e63246: f64 = (assign49290_e63245).exp();
        (assign49290_e63246, (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn4)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn6)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn7)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn8)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign49290_e63248;
        locals.var_delta_ns__blk1364_dn4 = assign49290_e63248_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign49290_e63248_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign49290_e63248_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign49290_e63248_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign49290_e63248_d_n9;
        locals.var_delta_ns__blk1364_rv = 0.0;

        let (assign49300_e63279, assign49300_e63279_d_n4, assign49300_e63279_d_n6, assign49300_e63279_d_n7, assign49300_e63279_d_n8, assign49300_e63279_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 == 0.0)) {
        let assign49300_e63259: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63264: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63268: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63270: f64 = (assign49300_e63268 * 0.3333333333333333);
        let assign49300_e63271: f64 = (1.0 + assign49300_e63270);
        let assign49300_e63272: f64 = (assign49300_e63264 * assign49300_e63271);
        let assign49300_e63273: f64 = (0.5 * assign49300_e63272);
        let assign49300_e63274: f64 = (1.0 + assign49300_e63273);
        let assign49300_e63275: f64 = (assign49300_e63259 * assign49300_e63274);
        let assign49300_e63276: f64 = (1.0 + assign49300_e63275);
        let assign49300_e63277: f64 = (1e-200 / assign49300_e63276);
        (assign49300_e63277, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign49300_e63279;
        locals.var_delta_ns__blk1364_dn4 = assign49300_e63279_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign49300_e63279_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign49300_e63279_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign49300_e63279_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign49300_e63279_d_n9;
        locals.var_delta_ns__blk1364_rv = 0.0;

        let assign49310_e63281: f64 = (locals.var_xg__blk1343).abs();
        let assign49310_e63283: f64 = if assign49310_e63281 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign49310_e63283;
        locals.var_guard1485_rv = 0.0;

        let (assign49320_e63297, assign49320_e63297_d_n4, assign49320_e63297_d_n6, assign49320_e63297_d_n7, assign49320_e63297_d_n8, assign49320_e63297_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign49320_e63291: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
        let assign49320_e63293: f64 = (assign49320_e63291 * 0.16666666666666666);
        let assign49320_e63295: f64 = (assign49320_e63293 * 0.7071067811865475);
        (assign49320_e63295, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49320_e63297;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49320_e63297_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49320_e63297_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49320_e63297_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49320_e63297_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49320_e63297_d_n9;
        locals.var_sp_s_temp1__blk1449_rv = 0.0;

        let (assign49330_e63319, assign49330_e63319_d_n4, assign49330_e63319_d_n6, assign49330_e63319_d_n7, assign49330_e63319_d_n8, assign49330_e63319_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign49330_e63305: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign49330_e63310: f64 = (1.0 - locals.var_delta_ns__blk1364);
        let assign49330_e63311: f64 = (locals.var_xg__blk1343 * assign49330_e63310);
        let assign49330_e63313: f64 = (assign49330_e63311 * locals.var_gf__blk1324);
        let assign49330_e63315: f64 = (assign49330_e63313 * locals.var_sp_s_temp1__blk1449);
        let assign49330_e63316: f64 = (1.0 + assign49330_e63315);
        let assign49330_e63317: f64 = (assign49330_e63305 * assign49330_e63316);
        (assign49330_e63317, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn4 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn4))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn6 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn6))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn7 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn7))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn8 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn8))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn9 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn9))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn9)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49330_e63319;
        locals.var_x_s__blk1363_dn4 = assign49330_e63319_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49330_e63319_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49330_e63319_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49330_e63319_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49330_e63319_d_n9;
        locals.var_x_s__blk1363_rv = 0.0;

        let assign49340_e63322: f64 = (-locals.var_margin__blk1361);
        let assign49340_e63323: f64 = if locals.var_xg__blk1343 < assign49340_e63322 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign49340_e63323;
        locals.var_guard1486_rv = 0.0;

        let (assign49350_e63335, assign49350_e63335_d_n4, assign49350_e63335_d_n6, assign49350_e63335_d_n7, assign49350_e63335_d_n8, assign49350_e63335_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49350_e63333: f64 = (-locals.var_xg__blk1343);
        (assign49350_e63333, (-locals.var_xg__blk1343_dn4), (-locals.var_xg__blk1343_dn6), (-locals.var_xg__blk1343_dn7), (-locals.var_xg__blk1343_dn8), (-locals.var_xg__blk1343_dn9),)
    } else {
        (locals.var_sp_s_yg__blk1451, locals.var_sp_s_yg__blk1451_dn4, locals.var_sp_s_yg__blk1451_dn6, locals.var_sp_s_yg__blk1451_dn7, locals.var_sp_s_yg__blk1451_dn8, locals.var_sp_s_yg__blk1451_dn9,)
    }
};
        locals.var_sp_s_yg__blk1451 = assign49350_e63335;
        locals.var_sp_s_yg__blk1451_dn4 = assign49350_e63335_d_n4;
        locals.var_sp_s_yg__blk1451_dn6 = assign49350_e63335_d_n6;
        locals.var_sp_s_yg__blk1451_dn7 = assign49350_e63335_d_n7;
        locals.var_sp_s_yg__blk1451_dn8 = assign49350_e63335_d_n8;
        locals.var_sp_s_yg__blk1451_dn9 = assign49350_e63335_d_n9;
        locals.var_sp_s_yg__blk1451_rv = 0.0;

        let (assign49360_e63350, assign49360_e63350_d_n4, assign49360_e63350_d_n6, assign49360_e63350_d_n7, assign49360_e63350_d_n8, assign49360_e63350_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49360_e63347: f64 = (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362);
        let assign49360_e63348: f64 = (1.25 * assign49360_e63347);
        (assign49360_e63348, (1.25 * ((locals.var_sp_s_yg__blk1451_dn4 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn4))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn6 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn7 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn8 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn8))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn9 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn9))),)
    } else {
        (locals.var_sp_s_ysub__blk1452, locals.var_sp_s_ysub__blk1452_dn4, locals.var_sp_s_ysub__blk1452_dn6, locals.var_sp_s_ysub__blk1452_dn7, locals.var_sp_s_ysub__blk1452_dn8, locals.var_sp_s_ysub__blk1452_dn9,)
    }
};
        locals.var_sp_s_ysub__blk1452 = assign49360_e63350;
        locals.var_sp_s_ysub__blk1452_dn4 = assign49360_e63350_d_n4;
        locals.var_sp_s_ysub__blk1452_dn6 = assign49360_e63350_d_n6;
        locals.var_sp_s_ysub__blk1452_dn7 = assign49360_e63350_d_n7;
        locals.var_sp_s_ysub__blk1452_dn8 = assign49360_e63350_d_n8;
        locals.var_sp_s_ysub__blk1452_dn9 = assign49360_e63350_d_n9;
        locals.var_sp_s_ysub__blk1452_rv = 0.0;

        let (assign49370_e63376, assign49370_e63376_d_n4, assign49370_e63376_d_n6, assign49370_e63376_d_n7, assign49370_e63376_d_n8, assign49370_e63376_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49370_e63362: f64 = (locals.var_sp_s_ysub__blk1452 + 10.0);
        let assign49370_e63365: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
        let assign49370_e63368: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
        let assign49370_e63369: f64 = (assign49370_e63365 * assign49370_e63368);
        let assign49370_e63371: f64 = (assign49370_e63369 + 64.0);
        let assign49370_e63372: f64 = (assign49370_e63371).sqrt();
        let assign49370_e63373: f64 = (assign49370_e63362 - assign49370_e63372);
        let assign49370_e63374: f64 = (0.5 * assign49370_e63373);
        (assign49370_e63374, (0.5 * (locals.var_sp_s_ysub__blk1452_dn4 - (((locals.var_sp_s_ysub__blk1452_dn4 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn4)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn6 - (((locals.var_sp_s_ysub__blk1452_dn6 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn6)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn7 - (((locals.var_sp_s_ysub__blk1452_dn7 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn7)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn8 - (((locals.var_sp_s_ysub__blk1452_dn8 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn8)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn9 - (((locals.var_sp_s_ysub__blk1452_dn9 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn9)) / (2.0 * assign49370_e63372)))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign49370_e63376;
        locals.var_sp_s_eta__blk1453_dn4 = assign49370_e63376_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign49370_e63376_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign49370_e63376_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign49370_e63376_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign49370_e63376_d_n9;
        locals.var_sp_s_eta__blk1453_rv = 0.0;

        let (assign49380_e63389, assign49380_e63389_d_n4, assign49380_e63389_d_n6, assign49380_e63389_d_n7, assign49380_e63389_d_n8, assign49380_e63389_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49380_e63387: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_eta__blk1453);
        (assign49380_e63387, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49380_e63389;
        locals.var_sp_s_temp__blk1448_dn4 = assign49380_e63389_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49380_e63389_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49380_e63389_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49380_e63389_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49380_e63389_d_n9;
        locals.var_sp_s_temp__blk1448_rv = 0.0;

        let (assign49390_e63408, assign49390_e63408_d_n4, assign49390_e63408_d_n6, assign49390_e63408_d_n7, assign49390_e63408_d_n8, assign49390_e63408_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49390_e63400: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49390_e63404: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign49390_e63405: f64 = (locals.var_gf2__blk1325 * assign49390_e63404);
        let assign49390_e63406: f64 = (assign49390_e63400 + assign49390_e63405);
        (assign49390_e63406, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn4))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn6))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn7))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn8))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn9))),)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign49390_e63408;
        locals.var_sp_s_a__blk1454_dn4 = assign49390_e63408_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign49390_e63408_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign49390_e63408_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign49390_e63408_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign49390_e63408_d_n9;
        locals.var_sp_s_a__blk1454_rv = 0.0;

        let (assign49400_e63423, assign49400_e63423_d_n4, assign49400_e63423_d_n6, assign49400_e63423_d_n7, assign49400_e63423_d_n8, assign49400_e63423_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49400_e63419: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49400_e63421: f64 = (assign49400_e63419 - locals.var_gf2__blk1325);
        (assign49400_e63421, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) - locals.var_gf2__blk1325_dn4), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) - locals.var_gf2__blk1325_dn6), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) - locals.var_gf2__blk1325_dn7), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) - locals.var_gf2__blk1325_dn8), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) - locals.var_gf2__blk1325_dn9),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign49400_e63423;
        locals.var_sp_s_c__blk1455_dn4 = assign49400_e63423_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign49400_e63423_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign49400_e63423_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign49400_e63423_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign49400_e63423_d_n9;
        locals.var_sp_s_c__blk1455_rv = 0.0;

        let (assign49410_e63440, assign49410_e63440_d_n4, assign49410_e63440_d_n6, assign49410_e63440_d_n7, assign49410_e63440_d_n8, assign49410_e63440_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49410_e63433: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign49410_e63436: f64 = (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341);
        let assign49410_e63437: f64 = (assign49410_e63436).ln();
        let assign49410_e63438: f64 = (assign49410_e63433 + assign49410_e63437);
        (assign49410_e63438, ((-locals.var_sp_s_eta__blk1453_dn4) + (((locals.var_sp_s_a__blk1454_dn4 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn4)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn6) + (((locals.var_sp_s_a__blk1454_dn6 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn6)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn7) + (((locals.var_sp_s_a__blk1454_dn7 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn7)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn8) + (((locals.var_sp_s_a__blk1454_dn8 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn8)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn9) + (((locals.var_sp_s_a__blk1454_dn9 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn9)) / assign49410_e63436)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign49410_e63440;
        locals.var_sp_s_tau__blk1456_dn4 = assign49410_e63440_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign49410_e63440_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign49410_e63440_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign49410_e63440_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign49410_e63440_d_n9;
        locals.var_sp_s_tau__blk1456_rv = 0.0;

        let (assign49420_e63453, assign49420_e63453_d_n4, assign49420_e63453_d_n6, assign49420_e63453_d_n7, assign49420_e63453_d_n8, assign49420_e63453_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49420_e63451: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign49420_e63451, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign49420_e63453;
        locals.var_nu_dn4 = assign49420_e63453_d_n4;
        locals.var_nu_dn6 = assign49420_e63453_d_n6;
        locals.var_nu_dn7 = assign49420_e63453_d_n7;
        locals.var_nu_dn8 = assign49420_e63453_d_n8;
        locals.var_nu_dn9 = assign49420_e63453_d_n9;
        locals.var_nu_rv = 0.0;

        let (assign49430_e63476, assign49430_e63476_d_n4, assign49430_e63476_d_n6, assign49430_e63476_d_n7, assign49430_e63476_d_n8, assign49430_e63476_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49430_e63464: f64 = (locals.var_nu * locals.var_nu);
        let assign49430_e63469: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49430_e63470: f64 = (0.5 * assign49430_e63469);
        let assign49430_e63472: f64 = (assign49430_e63470 - locals.var_sp_s_a__blk1454);
        let assign49430_e63473: f64 = (locals.var_sp_s_tau__blk1456 * assign49430_e63472);
        let assign49430_e63474: f64 = (assign49430_e63464 + assign49430_e63473);
        (assign49430_e63474, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - locals.var_sp_s_a__blk1454_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - locals.var_sp_s_a__blk1454_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - locals.var_sp_s_a__blk1454_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - locals.var_sp_s_a__blk1454_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - locals.var_sp_s_a__blk1454_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign49430_e63476;
        locals.var_mutau_dn4 = assign49430_e63476_d_n4;
        locals.var_mutau_dn6 = assign49430_e63476_d_n6;
        locals.var_mutau_dn7 = assign49430_e63476_d_n7;
        locals.var_mutau_dn8 = assign49430_e63476_d_n8;
        locals.var_mutau_dn9 = assign49430_e63476_d_n9;
        locals.var_mutau_rv = 0.0;

        let (assign49440_e63513, assign49440_e63513_d_n4, assign49440_e63513_d_n6, assign49440_e63513_d_n7, assign49440_e63513_d_n8, assign49440_e63513_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49440_e63488: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign49440_e63490: f64 = (assign49440_e63488 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63494: f64 = (locals.var_nu / locals.var_mutau);
        let assign49440_e63496: f64 = (assign49440_e63494 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63498: f64 = (assign49440_e63496 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63500: f64 = (assign49440_e63498 * locals.var_sp_s_c__blk1455);
        let assign49440_e63503: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49440_e63505: f64 = (assign49440_e63503 * 0.3333333333333333);
        let assign49440_e63507: f64 = (assign49440_e63505 - locals.var_sp_s_a__blk1454);
        let assign49440_e63508: f64 = (assign49440_e63500 * assign49440_e63507);
        let assign49440_e63509: f64 = (locals.var_mutau + assign49440_e63508);
        let assign49440_e63510: f64 = (assign49440_e63490 / assign49440_e63509);
        let assign49440_e63511: f64 = (locals.var_sp_s_eta__blk1453 + assign49440_e63510);
        (assign49440_e63511, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn4)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn4)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn4)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn6)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn6)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn6)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn7)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn7)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn7)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn8)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn8)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn8)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn9)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn9)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn9)))))) / (assign49440_e63509 * assign49440_e63509))),)
    } else {
        (locals.var_sp_s_y0__blk1457, locals.var_sp_s_y0__blk1457_dn4, locals.var_sp_s_y0__blk1457_dn6, locals.var_sp_s_y0__blk1457_dn7, locals.var_sp_s_y0__blk1457_dn8, locals.var_sp_s_y0__blk1457_dn9,)
    }
};
        locals.var_sp_s_y0__blk1457 = assign49440_e63513;
        locals.var_sp_s_y0__blk1457_dn4 = assign49440_e63513_d_n4;
        locals.var_sp_s_y0__blk1457_dn6 = assign49440_e63513_d_n6;
        locals.var_sp_s_y0__blk1457_dn7 = assign49440_e63513_d_n7;
        locals.var_sp_s_y0__blk1457_dn8 = assign49440_e63513_d_n8;
        locals.var_sp_s_y0__blk1457_dn9 = assign49440_e63513_d_n9;
        locals.var_sp_s_y0__blk1457_rv = 0.0;

        let assign49450_e63516: f64 = if locals.var_sp_s_y0__blk1457 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign49450_e63516;
        locals.var_guard1487_rv = 0.0;

    }
}
