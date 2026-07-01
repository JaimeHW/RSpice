#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47760_e61244, assign47760_e61244_d_n5, assign47760_e61244_d_n6, assign47760_e61244_d_n7, assign47760_e61244_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47760_e61240: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47760_e61241: f64 = (locals.var_gfedge * assign47760_e61240);
        let assign47760_e61242: f64 = (locals.var_q_edge_xsth + assign47760_e61241);
        (assign47760_e61242, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47760_e61240)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8,)
    }
};
        locals.var_q_edge_xth0 = assign47760_e61244;
        locals.var_q_edge_xth0_dn5 = assign47760_e61244_d_n5;
        locals.var_q_edge_xth0_dn6 = assign47760_e61244_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47760_e61244_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47760_e61244_d_n8;

        let (assign47770_e61253, assign47770_e61253_d_n5, assign47770_e61253_d_n6, assign47770_e61253_d_n7, assign47770_e61253_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47770_e61251: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47770_e61251, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8,)
    }
};
        locals.var_q_edge_xth = assign47770_e61253;
        locals.var_q_edge_xth_dn5 = assign47770_e61253_d_n5;
        locals.var_q_edge_xth_dn6 = assign47770_e61253_d_n6;
        locals.var_q_edge_xth_dn7 = assign47770_e61253_d_n7;
        locals.var_q_edge_xth_dn8 = assign47770_e61253_d_n8;

        let (assign47780_e61267, assign47780_e61267_d_n5, assign47780_e61267_d_n6, assign47780_e61267_d_n7, assign47780_e61267_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47780_e61262: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47780_e61263: f64 = (2.0 * assign47780_e61262);
        let assign47780_e61264: f64 = (locals.var_gfedge / assign47780_e61263);
        let assign47780_e61265: f64 = (1.0 + assign47780_e61264);
        (assign47780_e61265, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8,)
    }
};
        locals.var_q_edge_n = assign47780_e61267;
        locals.var_q_edge_n_dn5 = assign47780_e61267_d_n5;
        locals.var_q_edge_n_dn6 = assign47780_e61267_d_n6;
        locals.var_q_edge_n_dn7 = assign47780_e61267_d_n7;
        locals.var_q_edge_n_dn8 = assign47780_e61267_d_n8;

        let (assign47790_e61276, assign47790_e61276_d_n5, assign47790_e61276_d_n6, assign47790_e61276_d_n7, assign47790_e61276_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47790_e61274: f64 = (1.0 / locals.var_q_edge_n);
        (assign47790_e61274, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8,)
    }
};
        locals.var_q_edge_n_inv = assign47790_e61276;
        locals.var_q_edge_n_inv_dn5 = assign47790_e61276_d_n5;
        locals.var_q_edge_n_inv_dn6 = assign47790_e61276_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47790_e61276_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47790_e61276_d_n8;

        let (assign47800_e61285, assign47800_e61285_d_n5, assign47800_e61285_d_n6, assign47800_e61285_d_n7, assign47800_e61285_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47800_e61283: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47800_e61283, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    }
};
        locals.var_q_edge_xgt = assign47800_e61285;
        locals.var_q_edge_xgt_dn5 = assign47800_e61285_d_n5;
        locals.var_q_edge_xgt_dn6 = assign47800_e61285_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47800_e61285_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47800_e61285_d_n8;

        let assign47810_e61288: f64 = (-12.0);
        let assign47810_e61289: f64 = if locals.var_q_edge_xgt > assign47810_e61288 { 1.0 } else { 0.0 };
        locals.var_guard1255 = assign47810_e61289;

        let (assign47820_e61302, assign47820_e61302_d_n5, assign47820_e61302_d_n6, assign47820_e61302_d_n7, assign47820_e61302_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47820_e61298: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47820_e61300: f64 = (assign47820_e61298 - 1.0);
        (assign47820_e61300, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8,)
    }
};
        locals.var_q_edge_xgt0 = assign47820_e61302;
        locals.var_q_edge_xgt0_dn5 = assign47820_e61302_d_n5;
        locals.var_q_edge_xgt0_dn6 = assign47820_e61302_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47820_e61302_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47820_e61302_d_n8;

        let (assign47830_e61320, assign47830_e61320_d_n5, assign47830_e61320_d_n6, assign47830_e61320_d_n7, assign47830_e61320_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47830_e61313: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47830_e61315: f64 = (assign47830_e61313 + 10.0);
        let assign47830_e61316: f64 = (assign47830_e61315).sqrt();
        let assign47830_e61317: f64 = (locals.var_q_edge_xgt0 + assign47830_e61316);
        let assign47830_e61318: f64 = (0.5 * assign47830_e61317);
        (assign47830_e61318, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47830_e61316)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8,)
    }
};
        locals.var_q_edge_xgt0e = assign47830_e61320;
        locals.var_q_edge_xgt0e_dn5 = assign47830_e61320_d_n5;
        locals.var_q_edge_xgt0e_dn6 = assign47830_e61320_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47830_e61320_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47830_e61320_d_n8;

        let (assign47840_e61336, assign47840_e61336_d_n5, assign47840_e61336_d_n6, assign47840_e61336_d_n7, assign47840_e61336_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47840_e61330: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47840_e61331: f64 = (locals.var_q_edge_n * assign47840_e61330);
        let assign47840_e61332: f64 = (locals.var_q_edge_xgt - assign47840_e61331);
        let assign47840_e61334: f64 = (assign47840_e61332 + locals.var_lngfedge2);
        (assign47840_e61334, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8,)
    }
};
        locals.var_q_edge_qi0si = assign47840_e61336;
        locals.var_q_edge_qi0si_dn5 = assign47840_e61336_d_n5;
        locals.var_q_edge_qi0si_dn6 = assign47840_e61336_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47840_e61336_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47840_e61336_d_n8;

        let (assign47850_e61354, assign47850_e61354_d_n5, assign47850_e61354_d_n6, assign47850_e61354_d_n7, assign47850_e61354_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47850_e61347: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47850_e61349: f64 = (assign47850_e61347 + 2.0);
        let assign47850_e61350: f64 = (assign47850_e61349).sqrt();
        let assign47850_e61351: f64 = (locals.var_q_edge_qi0si + assign47850_e61350);
        let assign47850_e61352: f64 = (0.5 * assign47850_e61351);
        (assign47850_e61352, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47850_e61350)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8,)
    }
};
        locals.var_q_edge_qi0 = assign47850_e61354;
        locals.var_q_edge_qi0_dn5 = assign47850_e61354_d_n5;
        locals.var_q_edge_qi0_dn6 = assign47850_e61354_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47850_e61354_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47850_e61354_d_n8;

        let assign47860_e61357: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47860_e61359: f64 = if assign47860_e61357 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1256 = assign47860_e61359;

        let (assign47870_e61373, assign47870_e61373_d_n5, assign47870_e61373_d_n6, assign47870_e61373_d_n7, assign47870_e61373_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 != 0.0)) {
        let assign47870_e61370: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47870_e61371: f64 = (assign47870_e61370).exp();
        (assign47870_e61371, (assign47870_e61371 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47870_e61373;
        locals.var_q_edge_exp_x_dn5 = assign47870_e61373_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47870_e61373_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47870_e61373_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47870_e61373_d_n8;

        let (assign47880_e61413, assign47880_e61413_d_n5, assign47880_e61413_d_n6, assign47880_e61413_d_n7, assign47880_e61413_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 == 0.0)) {
        let assign47880_e61387: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61389: f64 = (assign47880_e61387 - 230.25850929940458);
        let assign47880_e61394: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61396: f64 = (assign47880_e61394 - 230.25850929940458);
        let assign47880_e61400: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47880_e61402: f64 = (assign47880_e61400 - 230.25850929940458);
        let assign47880_e61404: f64 = (assign47880_e61402 * 0.3333333333333333);
        let assign47880_e61405: f64 = (1.0 + assign47880_e61404);
        let assign47880_e61406: f64 = (assign47880_e61396 * assign47880_e61405);
        let assign47880_e61407: f64 = (0.5 * assign47880_e61406);
        let assign47880_e61408: f64 = (1.0 + assign47880_e61407);
        let assign47880_e61409: f64 = (assign47880_e61389 * assign47880_e61408);
        let assign47880_e61410: f64 = (1.0 + assign47880_e61409);
        let assign47880_e61411: f64 = (1e100 * assign47880_e61410);
        (assign47880_e61411, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8,)
    }
};
        locals.var_q_edge_exp_x = assign47880_e61413;
        locals.var_q_edge_exp_x_dn5 = assign47880_e61413_d_n5;
        locals.var_q_edge_exp_x_dn6 = assign47880_e61413_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47880_e61413_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47880_e61413_d_n8;

        let (assign47890_e61424, assign47890_e61424_d_n5, assign47890_e61424_d_n6, assign47890_e61424_d_n7, assign47890_e61424_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47890_e61422: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47890_e61422, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8,)
    }
};
        locals.var_q_edge_d0 = assign47890_e61424;
        locals.var_q_edge_d0_dn5 = assign47890_e61424_d_n5;
        locals.var_q_edge_d0_dn6 = assign47890_e61424_d_n6;
        locals.var_q_edge_d0_dn7 = assign47890_e61424_d_n7;
        locals.var_q_edge_d0_dn8 = assign47890_e61424_d_n8;

        let (assign47900_e61435, assign47900_e61435_d_n5, assign47900_e61435_d_n6, assign47900_e61435_d_n7, assign47900_e61435_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47900_e61433: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47900_e61433, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8,)
    }
};
        locals.var_q_edge_d0p = assign47900_e61435;
        locals.var_q_edge_d0p_dn5 = assign47900_e61435_d_n5;
        locals.var_q_edge_d0p_dn6 = assign47900_e61435_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47900_e61435_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47900_e61435_d_n8;

        let (assign47910_e61456, assign47910_e61456_d_n5, assign47910_e61456_d_n6, assign47910_e61456_d_n7, assign47910_e61456_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47910_e61444: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47910_e61448: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47910_e61449: f64 = (2.0 * assign47910_e61448);
        let assign47910_e61451: f64 = (assign47910_e61449 - locals.var_q_edge_d0p);
        let assign47910_e61453: f64 = (assign47910_e61451 * locals.var_q_edge_d0p);
        let assign47910_e61454: f64 = (assign47910_e61444 + assign47910_e61453);
        (assign47910_e61454, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn8))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8,)
    }
};
        locals.var_q_edge_sqerr = assign47910_e61456;
        locals.var_q_edge_sqerr_dn5 = assign47910_e61456_d_n5;
        locals.var_q_edge_sqerr_dn6 = assign47910_e61456_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47910_e61456_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47910_e61456_d_n8;

        let (assign47920_e61474, assign47920_e61474_d_n5, assign47920_e61474_d_n6, assign47920_e61474_d_n7, assign47920_e61474_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47920_e61465: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47920_e61467: f64 = (assign47920_e61465 - locals.var_q_edge_n);
        let assign47920_e61469: f64 = (assign47920_e61467 / locals.var_q_edge_d0p);
        let assign47920_e61471: f64 = (assign47920_e61469 - 1.0);
        let assign47920_e61472: f64 = (locals.var_q_edge_n * assign47920_e61471);
        (assign47920_e61472, ((locals.var_q_edge_n_dn5 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8,)
    }
};
        locals.var_q_edge_errq = assign47920_e61474;
        locals.var_q_edge_errq_dn5 = assign47920_e61474_d_n5;
        locals.var_q_edge_errq_dn6 = assign47920_e61474_d_n6;
        locals.var_q_edge_errq_dn7 = assign47920_e61474_d_n7;
        locals.var_q_edge_errq_dn8 = assign47920_e61474_d_n8;

        let (assign47930_e61485, assign47930_e61485_d_n5, assign47930_e61485_d_n6, assign47930_e61485_d_n7, assign47930_e61485_d_n8,) = {
    if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign47930_e61483: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47930_e61483, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47930_e61485;
        locals.var_qdeffedge_dn5 = assign47930_e61485_d_n5;
        locals.var_qdeffedge_dn6 = assign47930_e61485_d_n6;
        locals.var_qdeffedge_dn7 = assign47930_e61485_d_n7;
        locals.var_qdeffedge_dn8 = assign47930_e61485_d_n8;

        let assign47940_e61489: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47940_e61490: f64 = (locals.var_q_edge_n_inv * assign47940_e61489);
        let assign47940_e61492: f64 = (-230.25850929940458);
        let assign47940_e61493: f64 = if assign47940_e61490 > assign47940_e61492 { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign47940_e61493;

        let (assign47950_e61510, assign47950_e61510_d_n5, assign47950_e61510_d_n6, assign47950_e61510_d_n7, assign47950_e61510_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 != 0.0)) {
        let assign47950_e61506: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47950_e61507: f64 = (locals.var_q_edge_n_inv * assign47950_e61506);
        let assign47950_e61508: f64 = (assign47950_e61507).exp();
        (assign47950_e61508, (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn5 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn6 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn7 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn8 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47950_e61510;
        locals.var_qdeffedge_dn5 = assign47950_e61510_d_n5;
        locals.var_qdeffedge_dn6 = assign47950_e61510_d_n6;
        locals.var_qdeffedge_dn7 = assign47950_e61510_d_n7;
        locals.var_qdeffedge_dn8 = assign47950_e61510_d_n8;

        let (assign47960_e61560, assign47960_e61560_d_n5, assign47960_e61560_d_n6, assign47960_e61560_d_n7, assign47960_e61560_d_n8,) = {
    if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47960_e61524: f64 = (-230.25850929940458);
        let assign47960_e61528: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61529: f64 = (locals.var_q_edge_n_inv * assign47960_e61528);
        let assign47960_e61530: f64 = (assign47960_e61524 - assign47960_e61529);
        let assign47960_e61534: f64 = (-230.25850929940458);
        let assign47960_e61538: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61539: f64 = (locals.var_q_edge_n_inv * assign47960_e61538);
        let assign47960_e61540: f64 = (assign47960_e61534 - assign47960_e61539);
        let assign47960_e61543: f64 = (-230.25850929940458);
        let assign47960_e61547: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47960_e61548: f64 = (locals.var_q_edge_n_inv * assign47960_e61547);
        let assign47960_e61549: f64 = (assign47960_e61543 - assign47960_e61548);
        let assign47960_e61551: f64 = (assign47960_e61549 * 0.3333333333333333);
        let assign47960_e61552: f64 = (1.0 + assign47960_e61551);
        let assign47960_e61553: f64 = (assign47960_e61540 * assign47960_e61552);
        let assign47960_e61554: f64 = (0.5 * assign47960_e61553);
        let assign47960_e61555: f64 = (1.0 + assign47960_e61554);
        let assign47960_e61556: f64 = (assign47960_e61530 * assign47960_e61555);
        let assign47960_e61557: f64 = (1.0 + assign47960_e61556);
        let assign47960_e61558: f64 = (1e-100 / assign47960_e61557);
        (assign47960_e61558, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8,)
    }
};
        locals.var_qdeffedge = assign47960_e61560;
        locals.var_qdeffedge_dn5 = assign47960_e61560_d_n5;
        locals.var_qdeffedge_dn6 = assign47960_e61560_d_n6;
        locals.var_qdeffedge_dn7 = assign47960_e61560_d_n7;
        locals.var_qdeffedge_dn8 = assign47960_e61560_d_n8;

        let (assign47970_e61569, assign47970_e61569_d_n5, assign47970_e61569_d_n6, assign47970_e61569_d_n7, assign47970_e61569_d_n8,) = {
    if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
        let assign47970_e61567: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
        (assign47970_e61567, (locals.var_qdeffedge_dn5 - locals.var_qseffedge_dn5), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8,)
    }
};
        locals.var_qdseffedge = assign47970_e61569;
        locals.var_qdseffedge_dn5 = assign47970_e61569_d_n5;
        locals.var_qdseffedge_dn6 = assign47970_e61569_d_n6;
        locals.var_qdseffedge_dn7 = assign47970_e61569_d_n7;
        locals.var_qdseffedge_dn8 = assign47970_e61569_d_n8;

        let (assign47980_e61577, assign47980_e61577_d_n5, assign47980_e61577_d_n6, assign47980_e61577_d_n7, assign47980_e61577_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47980_e61574: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
        let assign47980_e61575: f64 = (0.5 * assign47980_e61574);
        (assign47980_e61575, (0.5 * (locals.var_qdeffedge_dn5 + locals.var_qseffedge_dn5)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)),)
    } else {
        (locals.var_qmeffedge, locals.var_qmeffedge_dn5, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8,)
    }
};
        locals.var_qmeffedge = assign47980_e61577;
        locals.var_qmeffedge_dn5 = assign47980_e61577_d_n5;
        locals.var_qmeffedge_dn6 = assign47980_e61577_d_n6;
        locals.var_qmeffedge_dn7 = assign47980_e61577_d_n7;
        locals.var_qmeffedge_dn8 = assign47980_e61577_d_n8;

        let (assign47990_e61590, assign47990_e61590_d_n5, assign47990_e61590_d_n6, assign47990_e61590_d_n7, assign47990_e61590_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign47990_e61581: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        let (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,) = {
            if (assign47990_e61581 > 1e-40) {
                let assign47990_e61586: f64 = (locals.var_xgedge - locals.var_qmeffedge);
                (assign47990_e61586, (locals.var_xgedge_dn5 - locals.var_qmeffedge_dn5), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8),)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,)
    } else {
        (locals.var_dsqredge, locals.var_dsqredge_dn5, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8,)
    }
};
        locals.var_dsqredge = assign47990_e61590;
        locals.var_dsqredge_dn5 = assign47990_e61590_d_n5;
        locals.var_dsqredge_dn6 = assign47990_e61590_d_n6;
        locals.var_dsqredge_dn7 = assign47990_e61590_d_n7;
        locals.var_dsqredge_dn8 = assign47990_e61590_d_n8;

        let (assign48000_e61605, assign48000_e61605_d_n5, assign48000_e61605_d_n6, assign48000_e61605_d_n7, assign48000_e61605_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign48000_e61595: f64 = (0.5 * locals.var_gfedge);
        let assign48000_e61599: f64 = (0.25 * locals.var_gfedge2);
        let assign48000_e61600: f64 = (locals.var_dsqredge + assign48000_e61599);
        let assign48000_e61601: f64 = (assign48000_e61600).sqrt();
        let assign48000_e61602: f64 = (assign48000_e61595 / assign48000_e61601);
        let assign48000_e61603: f64 = (1.0 - assign48000_e61602);
        (assign48000_e61603, (-(-((assign48000_e61595 * (locals.var_dsqredge_dn5 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn6 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn7 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn8 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))),)
    } else {
        (locals.var_alphabmedge, locals.var_alphabmedge_dn5, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8,)
    }
};
        locals.var_alphabmedge = assign48000_e61605;
        locals.var_alphabmedge_dn5 = assign48000_e61605_d_n5;
        locals.var_alphabmedge_dn6 = assign48000_e61605_d_n6;
        locals.var_alphabmedge_dn7 = assign48000_e61605_d_n7;
        locals.var_alphabmedge_dn8 = assign48000_e61605_d_n8;

        let (assign48010_e61624, assign48010_e61624_d_n5, assign48010_e61624_d_n6, assign48010_e61624_d_n7, assign48010_e61624_d_n8,) = {
    if (locals.var_guard1249 != 0.0) {
        let assign48010_e61608: f64 = (-locals.var_betedge_i);
        let assign48010_e61610: f64 = (assign48010_e61608 * locals.var_phit1edge);
        let assign48010_e61612: f64 = (assign48010_e61610 * locals.var_phit1edge);
        let assign48010_e61615: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
        let assign48010_e61617: f64 = (assign48010_e61615 + 1.0);
        let assign48010_e61618: f64 = (assign48010_e61612 * assign48010_e61617);
        let assign48010_e61620: f64 = (assign48010_e61618 * locals.var_qdseffedge);
        let assign48010_e61622: f64 = (assign48010_e61620 / locals.var_gmob_dc);
        (assign48010_e61622, ((((((((((assign48010_e61608 * locals.var_phit1edge_dn5) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn5)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn5 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn5)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn5)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn6)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn7)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn8)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_i_dsedge, locals.var_i_dsedge_dn5, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8,)
    }
};
        locals.var_i_dsedge = assign48010_e61624;
        locals.var_i_dsedge_dn5 = assign48010_e61624_d_n5;
        locals.var_i_dsedge_dn6 = assign48010_e61624_d_n6;
        locals.var_i_dsedge_dn7 = assign48010_e61624_d_n7;
        locals.var_i_dsedge_dn8 = assign48010_e61624_d_n8;

        locals.var_mavl = 0.0;
        locals.var_mavl_dn5 = 0.0;
        locals.var_mavl_dn6 = 0.0;
        locals.var_mavl_dn7 = 0.0;
        locals.var_mavl_dn8 = 0.0;

        locals.var_iimpact = 0.0;
        locals.var_iimpact_dn5 = 0.0;
        locals.var_iimpact_dn6 = 0.0;
        locals.var_iimpact_dn7 = 0.0;
        locals.var_iimpact_dn8 = 0.0;

        let assign48040_e61633: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign48040_e61633;

        let (assign48050_e61641, assign48050_e61641_d_n5, assign48050_e61641_d_n6, assign48050_e61641_d_n7, assign48050_e61641_d_n8,) = {
    if (locals.var_guard1258 != 0.0) {
        let assign48050_e61638: f64 = (locals.var_a3_i * locals.var_dps_dc);
        let assign48050_e61639: f64 = (locals.var_v_ds - assign48050_e61638);
        (assign48050_e61639, (-(locals.var_a3_i * locals.var_dps_dc_dn5)), (locals.var_v_ds_dn6 - (locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (-(locals.var_a3_i * locals.var_dps_dc_dn8)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn5, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8,)
    }
};
        locals.var_delvsat = assign48050_e61641;
        locals.var_delvsat_dn5 = assign48050_e61641_d_n5;
        locals.var_delvsat_dn6 = assign48050_e61641_d_n6;
        locals.var_delvsat_dn7 = assign48050_e61641_d_n7;
        locals.var_delvsat_dn8 = assign48050_e61641_d_n8;

        let assign48060_e61644: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign48060_e61644;

        let (assign48070_e61665, assign48070_e61665_d_n5, assign48070_e61665_d_n6, assign48070_e61665_d_n7, assign48070_e61665_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48070_e61653: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
        let assign48070_e61654: f64 = (assign48070_e61653).sqrt();
        let assign48070_e61656: f64 = (assign48070_e61654 - locals.var_sqrt_phib_dc);
        let assign48070_e61657: f64 = (locals.var_a4_i * assign48070_e61656);
        let assign48070_e61658: f64 = (1.0 + assign48070_e61657);
        let assign48070_e61661: f64 = (locals.var_delvsat + 1e-30);
        let assign48070_e61662: f64 = (assign48070_e61658 / assign48070_e61661);
        let assign48070_e61663: f64 = (locals.var_a2_t * assign48070_e61662);
        (assign48070_e61663, (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn5 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn5)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn6)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn7)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn8)) / (assign48070_e61661 * assign48070_e61661))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48070_e61665;
        locals.var_temp2_dn5 = assign48070_e61665_d_n5;
        locals.var_temp2_dn6 = assign48070_e61665_d_n6;
        locals.var_temp2_dn7 = assign48070_e61665_d_n7;
        locals.var_temp2_dn8 = assign48070_e61665_d_n8;

        let assign48080_e61667: f64 = (-locals.var_temp2);
        let assign48080_e61668: f64 = (assign48080_e61667).abs();
        let assign48080_e61670: f64 = if assign48080_e61668 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign48080_e61670;

        let (assign48090_e61680, assign48090_e61680_d_n5, assign48090_e61680_d_n6, assign48090_e61680_d_n7, assign48090_e61680_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 != 0.0)) {
        let assign48090_e61677: f64 = (-locals.var_temp2);
        let assign48090_e61678: f64 = (assign48090_e61677).exp();
        (assign48090_e61678, (assign48090_e61678 * (-locals.var_temp2_dn5)), (assign48090_e61678 * (-locals.var_temp2_dn6)), (assign48090_e61678 * (-locals.var_temp2_dn7)), (assign48090_e61678 * (-locals.var_temp2_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48090_e61680;
        locals.var_temp__blk936_dn5 = assign48090_e61680_d_n5;
        locals.var_temp__blk936_dn6 = assign48090_e61680_d_n6;
        locals.var_temp__blk936_dn7 = assign48090_e61680_d_n7;
        locals.var_temp__blk936_dn8 = assign48090_e61680_d_n8;

        let assign48100_e61682: f64 = (-locals.var_temp2);
        let assign48100_e61684: f64 = if assign48100_e61682 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign48100_e61684;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48110_e61723, assign48110_e61723_d_n5, assign48110_e61723_d_n6, assign48110_e61723_d_n7, assign48110_e61723_d_n8,) = {
    if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 != 0.0)) {
        let assign48110_e61696: f64 = (-230.25850929940458);
        let assign48110_e61698: f64 = (-locals.var_temp2);
        let assign48110_e61699: f64 = (assign48110_e61696 - assign48110_e61698);
        let assign48110_e61703: f64 = (-230.25850929940458);
        let assign48110_e61705: f64 = (-locals.var_temp2);
        let assign48110_e61706: f64 = (assign48110_e61703 - assign48110_e61705);
        let assign48110_e61709: f64 = (-230.25850929940458);
        let assign48110_e61711: f64 = (-locals.var_temp2);
        let assign48110_e61712: f64 = (assign48110_e61709 - assign48110_e61711);
        let assign48110_e61714: f64 = (assign48110_e61712 * 0.3333333333333333);
        let assign48110_e61715: f64 = (1.0 + assign48110_e61714);
        let assign48110_e61716: f64 = (assign48110_e61706 * assign48110_e61715);
        let assign48110_e61717: f64 = (0.5 * assign48110_e61716);
        let assign48110_e61718: f64 = (1.0 + assign48110_e61717);
        let assign48110_e61719: f64 = (assign48110_e61699 * assign48110_e61718);
        let assign48110_e61720: f64 = (1.0 + assign48110_e61719);
        let assign48110_e61721: f64 = (1e-100 / assign48110_e61720);
        (assign48110_e61721, (-((1e-100 * (((-(-locals.var_temp2_dn5)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn5)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn5)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48110_e61723;
        locals.var_temp__blk936_dn5 = assign48110_e61723_d_n5;
        locals.var_temp__blk936_dn6 = assign48110_e61723_d_n6;
        locals.var_temp__blk936_dn7 = assign48110_e61723_d_n7;
        locals.var_temp__blk936_dn8 = assign48110_e61723_d_n8;

        let (assign48120_e61760, assign48120_e61760_d_n5, assign48120_e61760_d_n6, assign48120_e61760_d_n7, assign48120_e61760_d_n8,) = {
    if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 == 0.0)) {
        let assign48120_e61736: f64 = (-locals.var_temp2);
        let assign48120_e61738: f64 = (assign48120_e61736 - 230.25850929940458);
        let assign48120_e61742: f64 = (-locals.var_temp2);
        let assign48120_e61744: f64 = (assign48120_e61742 - 230.25850929940458);
        let assign48120_e61747: f64 = (-locals.var_temp2);
        let assign48120_e61749: f64 = (assign48120_e61747 - 230.25850929940458);
        let assign48120_e61751: f64 = (assign48120_e61749 * 0.3333333333333333);
        let assign48120_e61752: f64 = (1.0 + assign48120_e61751);
        let assign48120_e61753: f64 = (assign48120_e61744 * assign48120_e61752);
        let assign48120_e61754: f64 = (0.5 * assign48120_e61753);
        let assign48120_e61755: f64 = (1.0 + assign48120_e61754);
        let assign48120_e61756: f64 = (assign48120_e61738 * assign48120_e61755);
        let assign48120_e61757: f64 = (1.0 + assign48120_e61756);
        let assign48120_e61758: f64 = (1e100 * assign48120_e61757);
        (assign48120_e61758, (1e100 * (((-locals.var_temp2_dn5) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn5) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn6) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn7) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn8) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48120_e61760;
        locals.var_temp__blk936_dn5 = assign48120_e61760_d_n5;
        locals.var_temp__blk936_dn6 = assign48120_e61760_d_n6;
        locals.var_temp__blk936_dn7 = assign48120_e61760_d_n7;
        locals.var_temp__blk936_dn8 = assign48120_e61760_d_n8;

        let (assign48130_e61770, assign48130_e61770_d_n5, assign48130_e61770_d_n6, assign48130_e61770_d_n7, assign48130_e61770_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48130_e61767: f64 = (locals.var_delvsat * locals.var_temp__blk936);
        let assign48130_e61768: f64 = (locals.var_a1_i * assign48130_e61767);
        (assign48130_e61768, (locals.var_a1_i * ((locals.var_delvsat_dn5 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn5))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn5, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8,)
    }
};
        locals.var_mavl = assign48130_e61770;
        locals.var_mavl_dn5 = assign48130_e61770_d_n5;
        locals.var_mavl_dn6 = assign48130_e61770_d_n6;
        locals.var_mavl_dn7 = assign48130_e61770_d_n7;
        locals.var_mavl_dn8 = assign48130_e61770_d_n8;

        let (assign48140_e61780, assign48140_e61780_d_n5, assign48140_e61780_d_n6, assign48140_e61780_d_n7, assign48140_e61780_d_n8,) = {
    if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
        let assign48140_e61777: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let assign48140_e61778: f64 = (locals.var_mavl * assign48140_e61777);
        (assign48140_e61778, ((locals.var_mavl_dn5 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5))), ((locals.var_mavl_dn6 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8,)
    }
};
        locals.var_iimpact = assign48140_e61780;
        locals.var_iimpact_dn5 = assign48140_e61780_d_n5;
        locals.var_iimpact_dn6 = assign48140_e61780_d_n6;
        locals.var_iimpact_dn7 = assign48140_e61780_d_n7;
        locals.var_iimpact_dn8 = assign48140_e61780_d_n8;

        let assign48150_e61784: f64 = (0.5 * locals.var_imaxii_i);
        let assign48150_e61785: f64 = if locals.var_iimpact > assign48150_e61784 { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign48150_e61785;

        let (assign48160_e61799, assign48160_e61799_d_n5, assign48160_e61799_d_n6, assign48160_e61799_d_n7, assign48160_e61799_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
        let assign48160_e61793: f64 = (2.0 * locals.var_iimpact);
        let assign48160_e61795: f64 = (assign48160_e61793 / locals.var_imaxii_i);
        let assign48160_e61797: f64 = (assign48160_e61795 - 1.0);
        (assign48160_e61797, ((2.0 * locals.var_iimpact_dn5) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48160_e61799;
        locals.var_temp__blk936_dn5 = assign48160_e61799_d_n5;
        locals.var_temp__blk936_dn6 = assign48160_e61799_d_n6;
        locals.var_temp__blk936_dn7 = assign48160_e61799_d_n7;
        locals.var_temp__blk936_dn8 = assign48160_e61799_d_n8;

        let (assign48170_e61820, assign48170_e61820_d_n5, assign48170_e61820_d_n6, assign48170_e61820_d_n7, assign48170_e61820_d_n8,) = {
    if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
        let assign48170_e61807: f64 = (0.5 * locals.var_imaxii_i);
        let assign48170_e61813: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign48170_e61814: f64 = (1.0 + assign48170_e61813);
        let assign48170_e61815: f64 = (assign48170_e61814).sqrt();
        let assign48170_e61816: f64 = (locals.var_temp__blk936 / assign48170_e61815);
        let assign48170_e61817: f64 = (1.0 + assign48170_e61816);
        let assign48170_e61818: f64 = (assign48170_e61807 * assign48170_e61817);
        (assign48170_e61818, (assign48170_e61807 * (((locals.var_temp__blk936_dn5 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn6 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn7 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn8 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8,)
    }
};
        locals.var_iimpact = assign48170_e61820;
        locals.var_iimpact_dn5 = assign48170_e61820_d_n5;
        locals.var_iimpact_dn6 = assign48170_e61820_d_n6;
        locals.var_iimpact_dn7 = assign48170_e61820_d_n7;
        locals.var_iimpact_dn8 = assign48170_e61820_d_n8;

        let assign48180_e61831: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1456 = assign48180_e61831;

        let assign48190_e61838: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1457 = assign48190_e61838;

        let (assign48200_e61844,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_phib_dc,)
    } else {
        (locals.var_phib__blk1297,)
    }
};
        locals.var_phib__blk1297 = assign48200_e61844;

        let (assign48210_e61850,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_aphi_dc,)
    } else {
        (locals.var_aphi__blk1298,)
    }
};
        locals.var_aphi__blk1298 = assign48210_e61850;

        let (assign48220_e61856,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_g_0_dc,)
    } else {
        (locals.var_g_0__blk1299,)
    }
};
        locals.var_g_0__blk1299 = assign48220_e61856;

        let (assign48230_e61862, assign48230_e61862_d_n6, assign48230_e61862_d_n7, assign48230_e61862_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn6, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8,)
    } else {
        (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8,)
    }
};
        locals.var_v_xb__blk1300 = assign48230_e61862;
        locals.var_v_xb__blk1300_dn6 = assign48230_e61862_d_n6;
        locals.var_v_xb__blk1300_dn7 = assign48230_e61862_d_n7;
        locals.var_v_xb__blk1300_dn8 = assign48230_e61862_d_n8;

        let (assign48240_e61868, assign48240_e61868_d_n5, assign48240_e61868_d_n6, assign48240_e61868_d_n7, assign48240_e61868_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn5, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8,)
    } else {
        (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8,)
    }
};
        locals.var_vsbstar__blk1301 = assign48240_e61868;
        locals.var_vsbstar__blk1301_dn5 = assign48240_e61868_d_n5;
        locals.var_vsbstar__blk1301_dn6 = assign48240_e61868_d_n6;
        locals.var_vsbstar__blk1301_dn7 = assign48240_e61868_d_n7;
        locals.var_vsbstar__blk1301_dn8 = assign48240_e61868_d_n8;

        let (assign48250_e61874,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_dvbstar__blk1305,)
    }
};
        locals.var_dvbstar__blk1305 = assign48250_e61874;

        let assign48260_e61877: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1458 = assign48260_e61877;

        let (assign48270_e61902, assign48270_e61902_d_n6, assign48270_e61902_d_n7, assign48270_e61902_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        let assign48270_e61886: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign48270_e61889: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48270_e61892: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48270_e61893: f64 = (assign48270_e61889 * assign48270_e61892);
        let assign48270_e61895: f64 = (assign48270_e61893 + locals.var_bphi_ac);
        let assign48270_e61896: f64 = (assign48270_e61895).sqrt();
        let assign48270_e61897: f64 = (assign48270_e61886 - assign48270_e61896);
        let assign48270_e61898: f64 = (0.5 * assign48270_e61897);
        let assign48270_e61900: f64 = (assign48270_e61898 + locals.var_phix_ac);
        (assign48270_e61900, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48270_e61896)))),)
    } else {
        (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8,)
    }
};
        locals.var_v_xb__blk1300 = assign48270_e61902;
        locals.var_v_xb__blk1300_dn6 = assign48270_e61902_d_n6;
        locals.var_v_xb__blk1300_dn7 = assign48270_e61902_d_n7;
        locals.var_v_xb__blk1300_dn8 = assign48270_e61902_d_n8;

        let (assign48280_e61929, assign48280_e61929_d_n6, assign48280_e61929_d_n7, assign48280_e61929_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        let assign48280_e61912: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61915: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61918: f64 = locals.var_v_xb__blk1300;
        let assign48280_e61919: f64 = (assign48280_e61915 * assign48280_e61918);
        let assign48280_e61921: f64 = (assign48280_e61919 + locals.var_aphi_ac);
        let assign48280_e61922: f64 = (assign48280_e61921).sqrt();
        let assign48280_e61923: f64 = (assign48280_e61912 - assign48280_e61922);
        let assign48280_e61924: f64 = (0.5 * assign48280_e61923);
        let assign48280_e61925: f64 = (locals.var_v_sb - assign48280_e61924);
        let assign48280_e61927: f64 = (assign48280_e61925 + locals.var_phix1_ac);
        (assign48280_e61927, (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb__blk1300_dn6 - (((locals.var_v_xb__blk1300_dn6 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1300_dn7 - (((locals.var_v_xb__blk1300_dn7 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1300_dn8 - (((locals.var_v_xb__blk1300_dn8 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48280_e61922))))),)
    } else {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8,)
    }
};
        locals.var_vsbstar_ac = assign48280_e61929;
        locals.var_vsbstar_ac_dn6 = assign48280_e61929_d_n6;
        locals.var_vsbstar_ac_dn7 = assign48280_e61929_d_n7;
        locals.var_vsbstar_ac_dn8 = assign48280_e61929_d_n8;

        let (assign48290_e61937, assign48290_e61937_d_n5, assign48290_e61937_d_n6, assign48290_e61937_d_n7, assign48290_e61937_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_vsbstar_ac, 0.0, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8,)
    } else {
        (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8,)
    }
};
        locals.var_vsbstar__blk1301 = assign48290_e61937;
        locals.var_vsbstar__blk1301_dn5 = assign48290_e61937_d_n5;
        locals.var_vsbstar__blk1301_dn6 = assign48290_e61937_d_n6;
        locals.var_vsbstar__blk1301_dn7 = assign48290_e61937_d_n7;
        locals.var_vsbstar__blk1301_dn8 = assign48290_e61937_d_n8;

        let (assign48300_e61945,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_phib_ac,)
    } else {
        (locals.var_phib__blk1297,)
    }
};
        locals.var_phib__blk1297 = assign48300_e61945;

        let (assign48310_e61953,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_aphi_ac,)
    } else {
        (locals.var_aphi__blk1298,)
    }
};
        locals.var_aphi__blk1298 = assign48310_e61953;

        let (assign48320_e61961,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (locals.var_g_0_ac,)
    } else {
        (locals.var_g_0__blk1299,)
    }
};
        locals.var_g_0__blk1299 = assign48320_e61961;

        let (assign48330_e61971, assign48330_e61971_d_n5, assign48330_e61971_d_n6, assign48330_e61971_d_n7, assign48330_e61971_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48330_e61967: f64 = (locals.var_vgb - locals.var_dvbstar__blk1305);
        let assign48330_e61969: f64 = (assign48330_e61967 - locals.var_vfb_t);
        (assign48330_e61969, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8,)
    } else {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    }
};
        locals.var_vgb1__blk1304 = assign48330_e61971;
        locals.var_vgb1__blk1304_dn5 = assign48330_e61971_d_n5;
        locals.var_vgb1__blk1304_dn6 = assign48330_e61971_d_n6;
        locals.var_vgb1__blk1304_dn7 = assign48330_e61971_d_n7;
        locals.var_vgb1__blk1304_dn8 = assign48330_e61971_d_n8;

        let (assign48340_e61983, assign48340_e61983_d_n5, assign48340_e61983_d_n6, assign48340_e61983_d_n7, assign48340_e61983_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48340_e61979: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign48340_e61980: f64 = (0.5 * assign48340_e61979);
        let assign48340_e61981: f64 = (locals.var_vsbstar__blk1301 + assign48340_e61980);
        (assign48340_e61981, locals.var_vsbstar__blk1301_dn5, (locals.var_vsbstar__blk1301_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar__blk1301_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar__blk1301_dn8,)
    } else {
        (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8,)
    }
};
        locals.var_vsbx__blk1306 = assign48340_e61983;
        locals.var_vsbx__blk1306_dn5 = assign48340_e61983_d_n5;
        locals.var_vsbx__blk1306_dn6 = assign48340_e61983_d_n6;
        locals.var_vsbx__blk1306_dn7 = assign48340_e61983_d_n7;
        locals.var_vsbx__blk1306_dn8 = assign48340_e61983_d_n8;

        let (assign48350_e61989, assign48350_e61989_d_n5, assign48350_e61989_d_n6, assign48350_e61989_d_n7, assign48350_e61989_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48350_e61989;
        locals.var_dctg__blk1318_dn5 = assign48350_e61989_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48350_e61989_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48350_e61989_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48350_e61989_d_n8;

        let assign48360_e61992: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1459 = assign48360_e61992;

        let (assign48370_e62002,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48370_e62000: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit);
        (assign48370_e62000,)
    } else {
        (locals.var_xbct__blk1309,)
    }
};
        locals.var_xbct__blk1309 = assign48370_e62002;

        let (assign48380_e62012, assign48380_e62012_d_n5, assign48380_e62012_d_n6, assign48380_e62012_d_n7, assign48380_e62012_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48380_e62010: f64 = (locals.var_vsbx__blk1306 * locals.var_inv_phit);
        (assign48380_e62010, (locals.var_vsbx__blk1306_dn5 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar__blk1310, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8,)
    }
};
        locals.var_xsbstar__blk1310 = assign48380_e62012;
        locals.var_xsbstar__blk1310_dn5 = assign48380_e62012_d_n5;
        locals.var_xsbstar__blk1310_dn6 = assign48380_e62012_d_n6;
        locals.var_xsbstar__blk1310_dn7 = assign48380_e62012_d_n7;
        locals.var_xsbstar__blk1310_dn8 = assign48380_e62012_d_n8;

        let (assign48390_e62022, assign48390_e62022_d_n5, assign48390_e62022_d_n6, assign48390_e62022_d_n7, assign48390_e62022_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48390_e62020: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit);
        (assign48390_e62020, (locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct__blk1311, locals.var_xgct__blk1311_dn5, locals.var_xgct__blk1311_dn6, locals.var_xgct__blk1311_dn7, locals.var_xgct__blk1311_dn8,)
    }
};
        locals.var_xgct__blk1311 = assign48390_e62022;
        locals.var_xgct__blk1311_dn5 = assign48390_e62022_d_n5;
        locals.var_xgct__blk1311_dn6 = assign48390_e62022_d_n6;
        locals.var_xgct__blk1311_dn7 = assign48390_e62022_d_n7;
        locals.var_xgct__blk1311_dn8 = assign48390_e62022_d_n8;

        let (assign48400_e62037, assign48400_e62037_d_n5, assign48400_e62037_d_n6, assign48400_e62037_d_n7, assign48400_e62037_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48400_e62031: f64 = (0.5 * locals.var_g_0__blk1299);
        let assign48400_e62033: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48400_e62034: f64 = (assign48400_e62031 / assign48400_e62033);
        let assign48400_e62035: f64 = (1.0 + assign48400_e62034);
        (assign48400_e62035, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48400_e62037;
        locals.var_temp1_dn5 = assign48400_e62037_d_n5;
        locals.var_temp1_dn6 = assign48400_e62037_d_n6;
        locals.var_temp1_dn7 = assign48400_e62037_d_n7;
        locals.var_temp1_dn8 = assign48400_e62037_d_n8;

        let (assign48410_e62050, assign48410_e62050_d_n5, assign48410_e62050_d_n6, assign48410_e62050_d_n7, assign48410_e62050_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48410_e62046: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48410_e62047: f64 = (locals.var_g_0__blk1299 * assign48410_e62046);
        let assign48410_e62048: f64 = (locals.var_xbct__blk1309 + assign48410_e62047);
        (assign48410_e62048, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48410_e62050;
        locals.var_temp2_dn5 = assign48410_e62050_d_n5;
        locals.var_temp2_dn6 = assign48410_e62050_d_n6;
        locals.var_temp2_dn7 = assign48410_e62050_d_n7;
        locals.var_temp2_dn8 = assign48410_e62050_d_n8;

        let (assign48420_e62072, assign48420_e62072_d_n5, assign48420_e62072_d_n6, assign48420_e62072_d_n7, assign48420_e62072_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48420_e62058: f64 = (locals.var_xgct__blk1311 - locals.var_temp2);
        let assign48420_e62060: f64 = (assign48420_e62058 / locals.var_temp1);
        let assign48420_e62063: f64 = (0.5 * locals.var_xbct__blk1309);
        let assign48420_e62064: f64 = (assign48420_e62060 + assign48420_e62063);
        let assign48420_e62067: f64 = (1.0 + locals.var_ctb_i);
        let assign48420_e62069: f64 = (assign48420_e62067 * locals.var_xsbstar__blk1310);
        let assign48420_e62070: f64 = (assign48420_e62064 - assign48420_e62069);
        (assign48420_e62070, (((((locals.var_xgct__blk1311_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn5)), (((((locals.var_xgct__blk1311_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn6)), (((((locals.var_xgct__blk1311_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn7)), (((((locals.var_xgct__blk1311_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn8)),)
    } else {
        (locals.var_xwict__blk1312, locals.var_xwict__blk1312_dn5, locals.var_xwict__blk1312_dn6, locals.var_xwict__blk1312_dn7, locals.var_xwict__blk1312_dn8,)
    }
};
        locals.var_xwict__blk1312 = assign48420_e62072;
        locals.var_xwict__blk1312_dn5 = assign48420_e62072_d_n5;
        locals.var_xwict__blk1312_dn6 = assign48420_e62072_d_n6;
        locals.var_xwict__blk1312_dn7 = assign48420_e62072_d_n7;
        locals.var_xwict__blk1312_dn8 = assign48420_e62072_d_n8;

        let (assign48430_e62084,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48430_e62080: f64 = (0.5 * locals.var_xbct__blk1309);
        let assign48430_e62082: f64 = (assign48430_e62080 + 2.0);
        (assign48430_e62082,)
    } else {
        (locals.var_xctmax__blk1313,)
    }
};
        locals.var_xctmax__blk1313 = assign48430_e62084;

        let (assign48440_e62094, assign48440_e62094_d_n5, assign48440_e62094_d_n6, assign48440_e62094_d_n7, assign48440_e62094_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48440_e62092: f64 = (locals.var_xbct__blk1309 + locals.var_xsbstar__blk1310);
        (assign48440_e62092, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8,)
    } else {
        (locals.var_xnct__blk1314, locals.var_xnct__blk1314_dn5, locals.var_xnct__blk1314_dn6, locals.var_xnct__blk1314_dn7, locals.var_xnct__blk1314_dn8,)
    }
};
        locals.var_xnct__blk1314 = assign48440_e62094;
        locals.var_xnct__blk1314_dn5 = assign48440_e62094_d_n5;
        locals.var_xnct__blk1314_dn6 = assign48440_e62094_d_n6;
        locals.var_xnct__blk1314_dn7 = assign48440_e62094_d_n7;
        locals.var_xnct__blk1314_dn8 = assign48440_e62094_d_n8;

        let (assign48450_e62119, assign48450_e62119_d_n5, assign48450_e62119_d_n6, assign48450_e62119_d_n7, assign48450_e62119_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48450_e62102: f64 = (locals.var_xgct__blk1311 - locals.var_xnct__blk1314);
        let assign48450_e62105: f64 = (locals.var_xnct__blk1314).sqrt();
        let assign48450_e62106: f64 = (locals.var_g_0__blk1299 * assign48450_e62105);
        let assign48450_e62107: f64 = (assign48450_e62102 - assign48450_e62106);
        let assign48450_e62111: f64 = (locals.var_xbct__blk1309 / locals.var_g_0__blk1299);
        let assign48450_e62113: f64 = (locals.var_xbct__blk1309).sqrt();
        let assign48450_e62114: f64 = (assign48450_e62111 + assign48450_e62113);
        let assign48450_e62115: f64 = (assign48450_e62114).ln();
        let assign48450_e62116: f64 = (2.0 * assign48450_e62115);
        let assign48450_e62117: f64 = (assign48450_e62107 - assign48450_e62116);
        (assign48450_e62117, ((locals.var_xgct__blk1311_dn5 - locals.var_xnct__blk1314_dn5) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn5 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn6 - locals.var_xnct__blk1314_dn6) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn6 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn7 - locals.var_xnct__blk1314_dn7) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn7 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn8 - locals.var_xnct__blk1314_dn8) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn8 / (2.0 * assign48450_e62105)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48450_e62119;
        locals.var_temp1_dn5 = assign48450_e62119_d_n5;
        locals.var_temp1_dn6 = assign48450_e62119_d_n6;
        locals.var_temp1_dn7 = assign48450_e62119_d_n7;
        locals.var_temp1_dn8 = assign48450_e62119_d_n8;

        let (assign48460_e62131, assign48460_e62131_d_n5, assign48460_e62131_d_n6, assign48460_e62131_d_n7, assign48460_e62131_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48460_e62127: f64 = (2.0 * locals.var_temp1);
        let assign48460_e62129: f64 = (assign48460_e62127 + locals.var_xctmax__blk1313);
        (assign48460_e62129, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_xmict__blk1315, locals.var_xmict__blk1315_dn5, locals.var_xmict__blk1315_dn6, locals.var_xmict__blk1315_dn7, locals.var_xmict__blk1315_dn8,)
    }
};
        locals.var_xmict__blk1315 = assign48460_e62131;
        locals.var_xmict__blk1315_dn5 = assign48460_e62131_d_n5;
        locals.var_xmict__blk1315_dn6 = assign48460_e62131_d_n6;
        locals.var_xmict__blk1315_dn7 = assign48460_e62131_d_n7;
        locals.var_xmict__blk1315_dn8 = assign48460_e62131_d_n8;

        let (assign48470_e62154, assign48470_e62154_d_n5, assign48470_e62154_d_n6, assign48470_e62154_d_n7, assign48470_e62154_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48470_e62140: f64 = (locals.var_xwict__blk1312 + locals.var_xmict__blk1315);
        let assign48470_e62143: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
        let assign48470_e62146: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
        let assign48470_e62147: f64 = (assign48470_e62143 * assign48470_e62146);
        let assign48470_e62149: f64 = (assign48470_e62147 + 20.0);
        let assign48470_e62150: f64 = (assign48470_e62149).sqrt();
        let assign48470_e62151: f64 = (assign48470_e62140 + assign48470_e62150);
        let assign48470_e62152: f64 = (0.5 * assign48470_e62151);
        (assign48470_e62152, (0.5 * ((locals.var_xwict__blk1312_dn5 + locals.var_xmict__blk1315_dn5) + ((((locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn6 + locals.var_xmict__blk1315_dn6) + ((((locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn7 + locals.var_xmict__blk1315_dn7) + ((((locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn8 + locals.var_xmict__blk1315_dn8) + ((((locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8))) / (2.0 * assign48470_e62150)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48470_e62154;
        locals.var_temp1_dn5 = assign48470_e62154_d_n5;
        locals.var_temp1_dn6 = assign48470_e62154_d_n6;
        locals.var_temp1_dn7 = assign48470_e62154_d_n7;
        locals.var_temp1_dn8 = assign48470_e62154_d_n8;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48480_e62168, assign48480_e62168_d_n5, assign48480_e62168_d_n6, assign48480_e62168_d_n7, assign48480_e62168_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48480_e62163: f64 = (locals.var_xgct__blk1311 - locals.var_xsbstar__blk1310);
        let assign48480_e62164: f64 = (2.0 * assign48480_e62163);
        let assign48480_e62166: f64 = (assign48480_e62164 - locals.var_xctmax__blk1313);
        (assign48480_e62166, (2.0 * (locals.var_xgct__blk1311_dn5 - locals.var_xsbstar__blk1310_dn5)), (2.0 * (locals.var_xgct__blk1311_dn6 - locals.var_xsbstar__blk1310_dn6)), (2.0 * (locals.var_xgct__blk1311_dn7 - locals.var_xsbstar__blk1310_dn7)), (2.0 * (locals.var_xgct__blk1311_dn8 - locals.var_xsbstar__blk1310_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48480_e62168;
        locals.var_temp2_dn5 = assign48480_e62168_d_n5;
        locals.var_temp2_dn6 = assign48480_e62168_d_n6;
        locals.var_temp2_dn7 = assign48480_e62168_d_n7;
        locals.var_temp2_dn8 = assign48480_e62168_d_n8;

        let (assign48490_e62191, assign48490_e62191_d_n5, assign48490_e62191_d_n6, assign48490_e62191_d_n7, assign48490_e62191_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48490_e62177: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign48490_e62180: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48490_e62183: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48490_e62184: f64 = (assign48490_e62180 * assign48490_e62183);
        let assign48490_e62186: f64 = (assign48490_e62184 + 20.0);
        let assign48490_e62187: f64 = (assign48490_e62186).sqrt();
        let assign48490_e62188: f64 = (assign48490_e62177 - assign48490_e62187);
        let assign48490_e62189: f64 = (0.5 * assign48490_e62188);
        (assign48490_e62189, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48490_e62187)))),)
    } else {
        (locals.var_xsubct__blk1316, locals.var_xsubct__blk1316_dn5, locals.var_xsubct__blk1316_dn6, locals.var_xsubct__blk1316_dn7, locals.var_xsubct__blk1316_dn8,)
    }
};
        locals.var_xsubct__blk1316 = assign48490_e62191;
        locals.var_xsubct__blk1316_dn5 = assign48490_e62191_d_n5;
        locals.var_xsubct__blk1316_dn6 = assign48490_e62191_d_n6;
        locals.var_xsubct__blk1316_dn7 = assign48490_e62191_d_n7;
        locals.var_xsubct__blk1316_dn8 = assign48490_e62191_d_n8;

        let (assign48500_e62214, assign48500_e62214_d_n5, assign48500_e62214_d_n6, assign48500_e62214_d_n7, assign48500_e62214_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48500_e62200: f64 = (locals.var_xsubct__blk1316 + locals.var_xctmax__blk1313);
        let assign48500_e62203: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
        let assign48500_e62206: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
        let assign48500_e62207: f64 = (assign48500_e62203 * assign48500_e62206);
        let assign48500_e62209: f64 = (assign48500_e62207 + 5.0);
        let assign48500_e62210: f64 = (assign48500_e62209).sqrt();
        let assign48500_e62211: f64 = (assign48500_e62200 - assign48500_e62210);
        let assign48500_e62212: f64 = (0.5 * assign48500_e62211);
        (assign48500_e62212, (0.5 * (locals.var_xsubct__blk1316_dn5 - (((locals.var_xsubct__blk1316_dn5 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn5)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn6 - (((locals.var_xsubct__blk1316_dn6 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn6)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn7 - (((locals.var_xsubct__blk1316_dn7 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn7)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn8 - (((locals.var_xsubct__blk1316_dn8 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn8)) / (2.0 * assign48500_e62210)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48500_e62214;
        locals.var_temp1_dn5 = assign48500_e62214_d_n5;
        locals.var_temp1_dn6 = assign48500_e62214_d_n6;
        locals.var_temp1_dn7 = assign48500_e62214_d_n7;
        locals.var_temp1_dn8 = assign48500_e62214_d_n8;

        let (assign48510_e62240, assign48510_e62240_d_n5, assign48510_e62240_d_n6, assign48510_e62240_d_n7, assign48510_e62240_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48510_e62223: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62224: f64 = (locals.var_temp1 + assign48510_e62223);
        let assign48510_e62227: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62228: f64 = (locals.var_temp1 - assign48510_e62227);
        let assign48510_e62231: f64 = (-locals.var_xctmax__blk1313);
        let assign48510_e62232: f64 = (locals.var_temp1 - assign48510_e62231);
        let assign48510_e62233: f64 = (assign48510_e62228 * assign48510_e62232);
        let assign48510_e62235: f64 = (assign48510_e62233 + 20.0);
        let assign48510_e62236: f64 = (assign48510_e62235).sqrt();
        let assign48510_e62237: f64 = (assign48510_e62224 + assign48510_e62236);
        let assign48510_e62238: f64 = (0.5 * assign48510_e62237);
        (assign48510_e62238, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn5)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn6)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn7)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn8)) / (2.0 * assign48510_e62236)))),)
    } else {
        (locals.var_xct__blk1317, locals.var_xct__blk1317_dn5, locals.var_xct__blk1317_dn6, locals.var_xct__blk1317_dn7, locals.var_xct__blk1317_dn8,)
    }
};
        locals.var_xct__blk1317 = assign48510_e62240;
        locals.var_xct__blk1317_dn5 = assign48510_e62240_d_n5;
        locals.var_xct__blk1317_dn6 = assign48510_e62240_d_n6;
        locals.var_xct__blk1317_dn7 = assign48510_e62240_d_n7;
        locals.var_xct__blk1317_dn8 = assign48510_e62240_d_n8;

        let (assign48520_e62254, assign48520_e62254_d_n5, assign48520_e62254_d_n6, assign48520_e62254_d_n7, assign48520_e62254_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign48520_e62249: f64 = (locals.var_xct__blk1317 / locals.var_xctmax__blk1313);
        let assign48520_e62251: f64 = (assign48520_e62249 + 1.0);
        let assign48520_e62252: f64 = (locals.var_ctg_t * assign48520_e62251);
        (assign48520_e62252, (locals.var_ctg_t * (locals.var_xct__blk1317_dn5 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn6 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn7 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn8 / locals.var_xctmax__blk1313)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48520_e62254;
        locals.var_temp2_dn5 = assign48520_e62254_d_n5;
        locals.var_temp2_dn6 = assign48520_e62254_d_n6;
        locals.var_temp2_dn7 = assign48520_e62254_d_n7;
        locals.var_temp2_dn8 = assign48520_e62254_d_n8;

        let assign48530_e62257: f64 = (-230.25850929940458);
        let assign48530_e62258: f64 = if locals.var_temp2 > assign48530_e62257 { 1.0 } else { 0.0 };
        locals.var_guard1460 = assign48530_e62258;

        let (assign48540_e62269, assign48540_e62269_d_n5, assign48540_e62269_d_n6, assign48540_e62269_d_n7, assign48540_e62269_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 != 0.0)) {
        let assign48540_e62267: f64 = (locals.var_temp2).exp();
        (assign48540_e62267, (assign48540_e62267 * locals.var_temp2_dn5), (assign48540_e62267 * locals.var_temp2_dn6), (assign48540_e62267 * locals.var_temp2_dn7), (assign48540_e62267 * locals.var_temp2_dn8),)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48540_e62269;
        locals.var_dctg__blk1318_dn5 = assign48540_e62269_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48540_e62269_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48540_e62269_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48540_e62269_d_n8;

        let (assign48550_e62305, assign48550_e62305_d_n5, assign48550_e62305_d_n6, assign48550_e62305_d_n7, assign48550_e62305_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 == 0.0)) {
        let assign48550_e62281: f64 = (-230.25850929940458);
        let assign48550_e62283: f64 = (assign48550_e62281 - locals.var_temp2);
        let assign48550_e62287: f64 = (-230.25850929940458);
        let assign48550_e62289: f64 = (assign48550_e62287 - locals.var_temp2);
        let assign48550_e62292: f64 = (-230.25850929940458);
        let assign48550_e62294: f64 = (assign48550_e62292 - locals.var_temp2);
        let assign48550_e62296: f64 = (assign48550_e62294 * 0.3333333333333333);
        let assign48550_e62297: f64 = (1.0 + assign48550_e62296);
        let assign48550_e62298: f64 = (assign48550_e62289 * assign48550_e62297);
        let assign48550_e62299: f64 = (0.5 * assign48550_e62298);
        let assign48550_e62300: f64 = (1.0 + assign48550_e62299);
        let assign48550_e62301: f64 = (assign48550_e62283 * assign48550_e62300);
        let assign48550_e62302: f64 = (1.0 + assign48550_e62301);
        let assign48550_e62303: f64 = (1e-100 / assign48550_e62302);
        (assign48550_e62303, (-((1e-100 * (((-locals.var_temp2_dn5) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn5) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn6) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn7) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn8) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))),)
    } else {
        (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8,)
    }
};
        locals.var_dctg__blk1318 = assign48550_e62305;
        locals.var_dctg__blk1318_dn5 = assign48550_e62305_d_n5;
        locals.var_dctg__blk1318_dn6 = assign48550_e62305_d_n6;
        locals.var_dctg__blk1318_dn7 = assign48550_e62305_d_n7;
        locals.var_dctg__blk1318_dn8 = assign48550_e62305_d_n8;

        let (assign48560_e62315, assign48560_e62315_d_n5, assign48560_e62315_d_n6, assign48560_e62315_d_n7, assign48560_e62315_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48560_e62312: f64 = (locals.var_ct_t * locals.var_dctg__blk1318);
        let assign48560_e62313: f64 = (1.0 + assign48560_e62312);
        (assign48560_e62313, (locals.var_ct_t * locals.var_dctg__blk1318_dn5), (locals.var_ct_t * locals.var_dctg__blk1318_dn6), (locals.var_ct_t * locals.var_dctg__blk1318_dn7), (locals.var_ct_t * locals.var_dctg__blk1318_dn8),)
    } else {
        (locals.var_ct_fact__blk1319, locals.var_ct_fact__blk1319_dn5, locals.var_ct_fact__blk1319_dn6, locals.var_ct_fact__blk1319_dn7, locals.var_ct_fact__blk1319_dn8,)
    }
};
        locals.var_ct_fact__blk1319 = assign48560_e62315;
        locals.var_ct_fact__blk1319_dn5 = assign48560_e62315_d_n5;
        locals.var_ct_fact__blk1319_dn6 = assign48560_e62315_d_n6;
        locals.var_ct_fact__blk1319_dn7 = assign48560_e62315_d_n7;
        locals.var_ct_fact__blk1319_dn8 = assign48560_e62315_d_n8;

        let (assign48570_e62323, assign48570_e62323_d_n5, assign48570_e62323_d_n6, assign48570_e62323_d_n7, assign48570_e62323_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48570_e62321: f64 = (locals.var_phit * locals.var_ct_fact__blk1319);
        (assign48570_e62321, (locals.var_phit * locals.var_ct_fact__blk1319_dn5), (locals.var_phit * locals.var_ct_fact__blk1319_dn6), (locals.var_phit * locals.var_ct_fact__blk1319_dn7), (locals.var_phit * locals.var_ct_fact__blk1319_dn8),)
    } else {
        (locals.var_phitct__blk1320, locals.var_phitct__blk1320_dn5, locals.var_phitct__blk1320_dn6, locals.var_phitct__blk1320_dn7, locals.var_phitct__blk1320_dn8,)
    }
};
        locals.var_phitct__blk1320 = assign48570_e62323;
        locals.var_phitct__blk1320_dn5 = assign48570_e62323_d_n5;
        locals.var_phitct__blk1320_dn6 = assign48570_e62323_d_n6;
        locals.var_phitct__blk1320_dn7 = assign48570_e62323_d_n7;
        locals.var_phitct__blk1320_dn8 = assign48570_e62323_d_n8;

        let (assign48580_e62341, assign48580_e62341_d_n5, assign48580_e62341_d_n6, assign48580_e62341_d_n7, assign48580_e62341_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48580_e62331: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign48580_e62332: f64 = (1.0 + assign48580_e62331);
        let assign48580_e62333: f64 = (locals.var_psce_i * assign48580_e62332);
        let assign48580_e62337: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1306);
        let assign48580_e62338: f64 = (1.0 + assign48580_e62337);
        let assign48580_e62339: f64 = (assign48580_e62333 * assign48580_e62338);
        (assign48580_e62339, (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn7))), (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn8)),)
    } else {
        (locals.var_dphit1__blk1321, locals.var_dphit1__blk1321_dn5, locals.var_dphit1__blk1321_dn6, locals.var_dphit1__blk1321_dn7, locals.var_dphit1__blk1321_dn8,)
    }
};
        locals.var_dphit1__blk1321 = assign48580_e62341;
        locals.var_dphit1__blk1321_dn5 = assign48580_e62341_d_n5;
        locals.var_dphit1__blk1321_dn6 = assign48580_e62341_d_n6;
        locals.var_dphit1__blk1321_dn7 = assign48580_e62341_d_n7;
        locals.var_dphit1__blk1321_dn8 = assign48580_e62341_d_n8;

        let (assign48590_e62351, assign48590_e62351_d_n5, assign48590_e62351_d_n6, assign48590_e62351_d_n7, assign48590_e62351_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48590_e62348: f64 = (1.0 + locals.var_dphit1__blk1321);
        let assign48590_e62349: f64 = (locals.var_phitct__blk1320 * assign48590_e62348);
        (assign48590_e62349, ((locals.var_phitct__blk1320_dn5 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn5)), ((locals.var_phitct__blk1320_dn6 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn6)), ((locals.var_phitct__blk1320_dn7 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn7)), ((locals.var_phitct__blk1320_dn8 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn8)),)
    } else {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    }
};
        locals.var_phit1__blk1322 = assign48590_e62351;
        locals.var_phit1__blk1322_dn5 = assign48590_e62351_d_n5;
        locals.var_phit1__blk1322_dn6 = assign48590_e62351_d_n6;
        locals.var_phit1__blk1322_dn7 = assign48590_e62351_d_n7;
        locals.var_phit1__blk1322_dn8 = assign48590_e62351_d_n8;

        let (assign48600_e62359, assign48600_e62359_d_n5, assign48600_e62359_d_n6, assign48600_e62359_d_n7, assign48600_e62359_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48600_e62357: f64 = (1.0 / locals.var_phit1__blk1322);
        (assign48600_e62357, (-(locals.var_phit1__blk1322_dn5 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn6 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn7 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn8 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))),)
    } else {
        (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8,)
    }
};
        locals.var_inv_phit1__blk1323 = assign48600_e62359;
        locals.var_inv_phit1__blk1323_dn5 = assign48600_e62359_d_n5;
        locals.var_inv_phit1__blk1323_dn6 = assign48600_e62359_d_n6;
        locals.var_inv_phit1__blk1323_dn7 = assign48600_e62359_d_n7;
        locals.var_inv_phit1__blk1323_dn8 = assign48600_e62359_d_n8;

        let (assign48610_e62370, assign48610_e62370_d_n5, assign48610_e62370_d_n6, assign48610_e62370_d_n7, assign48610_e62370_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48610_e62366: f64 = (locals.var_phit * locals.var_inv_phit1__blk1323);
        let assign48610_e62367: f64 = (assign48610_e62366).sqrt();
        let assign48610_e62368: f64 = (locals.var_g_0__blk1299 * assign48610_e62367);
        (assign48610_e62368, (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn5) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn6) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn7) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn8) / (2.0 * assign48610_e62367))),)
    } else {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    }
};
        locals.var_gf__blk1307 = assign48610_e62370;
        locals.var_gf__blk1307_dn5 = assign48610_e62370_d_n5;
        locals.var_gf__blk1307_dn6 = assign48610_e62370_d_n6;
        locals.var_gf__blk1307_dn7 = assign48610_e62370_d_n7;
        locals.var_gf__blk1307_dn8 = assign48610_e62370_d_n8;

        let (assign48620_e62378, assign48620_e62378_d_n5, assign48620_e62378_d_n6, assign48620_e62378_d_n7, assign48620_e62378_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48620_e62376: f64 = (locals.var_gf__blk1307 * locals.var_gf__blk1307);
        (assign48620_e62376, ((locals.var_gf__blk1307_dn5 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn5)), ((locals.var_gf__blk1307_dn6 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn6)), ((locals.var_gf__blk1307_dn7 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn7)), ((locals.var_gf__blk1307_dn8 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn8)),)
    } else {
        (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8,)
    }
};
        locals.var_gf2__blk1308 = assign48620_e62378;
        locals.var_gf2__blk1308_dn5 = assign48620_e62378_d_n5;
        locals.var_gf2__blk1308_dn6 = assign48620_e62378_d_n6;
        locals.var_gf2__blk1308_dn7 = assign48620_e62378_d_n7;
        locals.var_gf2__blk1308_dn8 = assign48620_e62378_d_n8;

        let (assign48630_e62386, assign48630_e62386_d_n5, assign48630_e62386_d_n6, assign48630_e62386_d_n7, assign48630_e62386_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48630_e62384: f64 = (1.0 / locals.var_gf2__blk1308);
        (assign48630_e62384, (-(locals.var_gf2__blk1308_dn5 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn6 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn7 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn8 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))),)
    } else {
        (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8,)
    }
};
        locals.var_inv_gf2__blk1324 = assign48630_e62386;
        locals.var_inv_gf2__blk1324_dn5 = assign48630_e62386_d_n5;
        locals.var_inv_gf2__blk1324_dn6 = assign48630_e62386_d_n6;
        locals.var_inv_gf2__blk1324_dn7 = assign48630_e62386_d_n7;
        locals.var_inv_gf2__blk1324_dn8 = assign48630_e62386_d_n8;

        let (assign48640_e62394, assign48640_e62394_d_n5, assign48640_e62394_d_n6, assign48640_e62394_d_n7, assign48640_e62394_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48640_e62392: f64 = (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323);
        (assign48640_e62392, ((locals.var_vsbstar__blk1301_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vsbstar__blk1301_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vsbstar__blk1301_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vsbstar__blk1301_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_ux__blk1325, locals.var_ux__blk1325_dn5, locals.var_ux__blk1325_dn6, locals.var_ux__blk1325_dn7, locals.var_ux__blk1325_dn8,)
    }
};
        locals.var_ux__blk1325 = assign48640_e62394;
        locals.var_ux__blk1325_dn5 = assign48640_e62394_d_n5;
        locals.var_ux__blk1325_dn6 = assign48640_e62394_d_n6;
        locals.var_ux__blk1325_dn7 = assign48640_e62394_d_n7;
        locals.var_ux__blk1325_dn8 = assign48640_e62394_d_n8;

        let (assign48650_e62402, assign48650_e62402_d_n5, assign48650_e62402_d_n6, assign48650_e62402_d_n7, assign48650_e62402_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48650_e62400: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323);
        (assign48650_e62400, ((locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8,)
    }
};
        locals.var_xg__blk1326 = assign48650_e62402;
        locals.var_xg__blk1326_dn5 = assign48650_e62402_d_n5;
        locals.var_xg__blk1326_dn6 = assign48650_e62402_d_n6;
        locals.var_xg__blk1326_dn7 = assign48650_e62402_d_n7;
        locals.var_xg__blk1326_dn8 = assign48650_e62402_d_n8;

        let (assign48660_e62419, assign48660_e62419_d_n6, assign48660_e62419_d_n7,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48660_e62408: f64 = (2.0 * locals.var_vdsx);
        let assign48660_e62413: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign48660_e62414: f64 = (1.0 + assign48660_e62413);
        let assign48660_e62415: f64 = (assign48660_e62414).sqrt();
        let assign48660_e62416: f64 = (1.0 + assign48660_e62415);
        let assign48660_e62417: f64 = (assign48660_e62408 / assign48660_e62416);
        (assign48660_e62417, ((((2.0 * locals.var_vdsx_dn6) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)), ((((2.0 * locals.var_vdsx_dn7) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)),)
    } else {
        (locals.var_vdsp__blk1327, locals.var_vdsp__blk1327_dn6, locals.var_vdsp__blk1327_dn7,)
    }
};
        locals.var_vdsp__blk1327 = assign48660_e62419;
        locals.var_vdsp__blk1327_dn6 = assign48660_e62419_d_n6;
        locals.var_vdsp__blk1327_dn7 = assign48660_e62419_d_n7;

        let (assign48670_e62433, assign48670_e62433_d_n5, assign48670_e62433_d_n6, assign48670_e62433_d_n7, assign48670_e62433_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48670_e62425: f64 = (locals.var_cf_i * locals.var_vdsp__blk1327);
        let assign48670_e62429: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1306);
        let assign48670_e62430: f64 = (1.0 + assign48670_e62429);
        let assign48670_e62431: f64 = (assign48670_e62425 * assign48670_e62430);
        (assign48670_e62431, (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn6) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn7) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn7))), (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn8)),)
    } else {
        (locals.var_delphib__blk1328, locals.var_delphib__blk1328_dn5, locals.var_delphib__blk1328_dn6, locals.var_delphib__blk1328_dn7, locals.var_delphib__blk1328_dn8,)
    }
};
        locals.var_delphib__blk1328 = assign48670_e62433;
        locals.var_delphib__blk1328_dn5 = assign48670_e62433_d_n5;
        locals.var_delphib__blk1328_dn6 = assign48670_e62433_d_n6;
        locals.var_delphib__blk1328_dn7 = assign48670_e62433_d_n7;
        locals.var_delphib__blk1328_dn8 = assign48670_e62433_d_n8;

        let (assign48680_e62441, assign48680_e62441_d_n5, assign48680_e62441_d_n6, assign48680_e62441_d_n7, assign48680_e62441_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48680_e62439: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323);
        (assign48680_e62439, (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn5), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn6), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn7), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn8),)
    } else {
        (locals.var_xb__blk1329, locals.var_xb__blk1329_dn5, locals.var_xb__blk1329_dn6, locals.var_xb__blk1329_dn7, locals.var_xb__blk1329_dn8,)
    }
};
        locals.var_xb__blk1329 = assign48680_e62441;
        locals.var_xb__blk1329_dn5 = assign48680_e62441_d_n5;
        locals.var_xb__blk1329_dn6 = assign48680_e62441_d_n6;
        locals.var_xb__blk1329_dn7 = assign48680_e62441_d_n7;
        locals.var_xb__blk1329_dn8 = assign48680_e62441_d_n8;

        let (assign48690_e62452, assign48690_e62452_d_n5, assign48690_e62452_d_n6, assign48690_e62452_d_n7, assign48690_e62452_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48690_e62447: f64 = (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300);
        let assign48690_e62449: f64 = (assign48690_e62447 + locals.var_aphi__blk1298);
        let assign48690_e62450: f64 = (assign48690_e62449).sqrt();
        (assign48690_e62450, 0.0, (((locals.var_v_xb__blk1300_dn6 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn7 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn8 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48690_e62450)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign48690_e62452;
        locals.var_temp1_dn5 = assign48690_e62452_d_n5;
        locals.var_temp1_dn6 = assign48690_e62452_d_n6;
        locals.var_temp1_dn7 = assign48690_e62452_d_n7;
        locals.var_temp1_dn8 = assign48690_e62452_d_n8;

        let (assign48700_e62467, assign48700_e62467_d_n5, assign48700_e62467_d_n6, assign48700_e62467_d_n7, assign48700_e62467_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48700_e62458: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
        let assign48700_e62461: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
        let assign48700_e62462: f64 = (assign48700_e62458 * assign48700_e62461);
        let assign48700_e62464: f64 = (assign48700_e62462 + locals.var_aphi__blk1298);
        let assign48700_e62465: f64 = (assign48700_e62464).sqrt();
        (assign48700_e62465, ((((-locals.var_delphib__blk1328_dn5) * assign48700_e62461) + (assign48700_e62458 * (-locals.var_delphib__blk1328_dn5))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8))) / (2.0 * assign48700_e62465)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign48700_e62467;
        locals.var_temp2_dn5 = assign48700_e62467_d_n5;
        locals.var_temp2_dn6 = assign48700_e62467_d_n6;
        locals.var_temp2_dn7 = assign48700_e62467_d_n7;
        locals.var_temp2_dn8 = assign48700_e62467_d_n8;

        let (assign48710_e62481, assign48710_e62481_d_n5, assign48710_e62481_d_n6, assign48710_e62481_d_n7, assign48710_e62481_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48710_e62473: f64 = (0.5 * locals.var_inv_phit1__blk1323);
        let assign48710_e62476: f64 = (locals.var_delphib__blk1328 + locals.var_temp1);
        let assign48710_e62478: f64 = (assign48710_e62476 - locals.var_temp2);
        let assign48710_e62479: f64 = (assign48710_e62473 * assign48710_e62478);
        (assign48710_e62479, (((0.5 * locals.var_inv_phit1__blk1323_dn5) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5))), (((0.5 * locals.var_inv_phit1__blk1323_dn6) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1323_dn7) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1323_dn8) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))),)
    } else {
        (locals.var_delxb__blk1330, locals.var_delxb__blk1330_dn5, locals.var_delxb__blk1330_dn6, locals.var_delxb__blk1330_dn7, locals.var_delxb__blk1330_dn8,)
    }
};
        locals.var_delxb__blk1330 = assign48710_e62481;
        locals.var_delxb__blk1330_dn5 = assign48710_e62481_d_n5;
        locals.var_delxb__blk1330_dn6 = assign48710_e62481_d_n6;
        locals.var_delxb__blk1330_dn7 = assign48710_e62481_d_n7;
        locals.var_delxb__blk1330_dn8 = assign48710_e62481_d_n8;

        let (assign48720_e62489, assign48720_e62489_d_n5, assign48720_e62489_d_n6, assign48720_e62489_d_n7, assign48720_e62489_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48720_e62487: f64 = (locals.var_xb__blk1329 + locals.var_ux__blk1325);
        (assign48720_e62487, (locals.var_xb__blk1329_dn5 + locals.var_ux__blk1325_dn5), (locals.var_xb__blk1329_dn6 + locals.var_ux__blk1325_dn6), (locals.var_xb__blk1329_dn7 + locals.var_ux__blk1325_dn7), (locals.var_xb__blk1329_dn8 + locals.var_ux__blk1325_dn8),)
    } else {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    }
};
        locals.var_xno_s__blk1331 = assign48720_e62489;
        locals.var_xno_s__blk1331_dn5 = assign48720_e62489_d_n5;
        locals.var_xno_s__blk1331_dn6 = assign48720_e62489_d_n6;
        locals.var_xno_s__blk1331_dn7 = assign48720_e62489_d_n7;
        locals.var_xno_s__blk1331_dn8 = assign48720_e62489_d_n8;

        let (assign48730_e62497, assign48730_e62497_d_n5, assign48730_e62497_d_n6, assign48730_e62497_d_n7, assign48730_e62497_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48730_e62495: f64 = (locals.var_xno_s__blk1331 - locals.var_delxb__blk1330);
        (assign48730_e62495, (locals.var_xno_s__blk1331_dn5 - locals.var_delxb__blk1330_dn5), (locals.var_xno_s__blk1331_dn6 - locals.var_delxb__blk1330_dn6), (locals.var_xno_s__blk1331_dn7 - locals.var_delxb__blk1330_dn7), (locals.var_xno_s__blk1331_dn8 - locals.var_delxb__blk1330_dn8),)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign48730_e62497;
        locals.var_xn_s__blk1332_dn5 = assign48730_e62497_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign48730_e62497_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign48730_e62497_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign48730_e62497_d_n8;

        let assign48740_e62500: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign48740_e62500;

        let assign48750_e62502: f64 = (locals.var_xn_s__blk1332).abs();
        let assign48750_e62504: f64 = if assign48750_e62502 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign48750_e62504;

        let (assign48760_e62528, assign48760_e62528_d_n5, assign48760_e62528_d_n6, assign48760_e62528_d_n7, assign48760_e62528_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) {
        let assign48760_e62517: f64 = (0.5 * locals.var_xn_s__blk1332);
        let assign48760_e62521: f64 = (0.3125 * locals.var_xn_s__blk1332);
        let assign48760_e62522: f64 = (1.0 - assign48760_e62521);
        let assign48760_e62523: f64 = (assign48760_e62517 * assign48760_e62522);
        let assign48760_e62524: f64 = (1.0 - assign48760_e62523);
        let assign48760_e62525: f64 = (locals.var_gf__blk1307 * assign48760_e62524);
        let assign48760_e62526: f64 = (1.0 + assign48760_e62525);
        (assign48760_e62526, ((locals.var_gf__blk1307_dn5 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn5) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn5))))))), ((locals.var_gf__blk1307_dn6 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn6) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn6))))))), ((locals.var_gf__blk1307_dn7 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn7) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn7))))))), ((locals.var_gf__blk1307_dn8 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn8) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn8))))))),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48760_e62528;
        locals.var_nscr__blk1333_dn5 = assign48760_e62528_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48760_e62528_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48760_e62528_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48760_e62528_d_n8;

        let assign48770_e62531: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign48770_e62531;

        let (assign48780_e62546, assign48780_e62546_d_n5, assign48780_e62546_d_n6, assign48780_e62546_d_n7, assign48780_e62546_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign48780_e62543: f64 = (-locals.var_xn_s__blk1332);
        let assign48780_e62544: f64 = (assign48780_e62543).exp();
        (assign48780_e62544, (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn5)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn6)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn7)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign48780_e62546;
        locals.var_delta_ns__blk1347_dn5 = assign48780_e62546_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign48780_e62546_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign48780_e62546_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign48780_e62546_d_n8;

        let (assign48790_e62582, assign48790_e62582_d_n5, assign48790_e62582_d_n6, assign48790_e62582_d_n7, assign48790_e62582_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 == 0.0)) {
        let assign48790_e62562: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62567: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62571: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62573: f64 = (assign48790_e62571 * 0.3333333333333333);
        let assign48790_e62574: f64 = (1.0 + assign48790_e62573);
        let assign48790_e62575: f64 = (assign48790_e62567 * assign48790_e62574);
        let assign48790_e62576: f64 = (0.5 * assign48790_e62575);
        let assign48790_e62577: f64 = (1.0 + assign48790_e62576);
        let assign48790_e62578: f64 = (assign48790_e62562 * assign48790_e62577);
        let assign48790_e62579: f64 = (1.0 + assign48790_e62578);
        let assign48790_e62580: f64 = (1e-200 / assign48790_e62579);
        (assign48790_e62580, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign48790_e62582;
        locals.var_delta_ns__blk1347_dn5 = assign48790_e62582_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign48790_e62582_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign48790_e62582_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign48790_e62582_d_n8;

        let (assign48800_e62599, assign48800_e62599_d_n5, assign48800_e62599_d_n6, assign48800_e62599_d_n7, assign48800_e62599_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let (assign48800_e62597,) = {
            if (locals.var_xn_s__blk1332 > 0.0) {
                (1.0,)
            } else {
                let assign48800_e62596: f64 = (-1.0);
                (assign48800_e62596,)
            }
        };
        (assign48800_e62597, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48800_e62599;
        locals.var_temp__blk936_dn5 = assign48800_e62599_d_n5;
        locals.var_temp__blk936_dn6 = assign48800_e62599_d_n6;
        locals.var_temp__blk936_dn7 = assign48800_e62599_d_n7;
        locals.var_temp__blk936_dn8 = assign48800_e62599_d_n8;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign48810_e62631, assign48810_e62631_d_n5, assign48810_e62631_d_n6, assign48810_e62631_d_n7, assign48810_e62631_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign48810_e62611: f64 = (locals.var_temp__blk936 * locals.var_gf__blk1307);
        let assign48810_e62616: f64 = (1.0 - locals.var_xn_s__blk1332);
        let assign48810_e62617: f64 = (locals.var_delta_ns__blk1347 * assign48810_e62616);
        let assign48810_e62618: f64 = (1.0 - assign48810_e62617);
        let assign48810_e62619: f64 = (assign48810_e62611 * assign48810_e62618);
        let assign48810_e62624: f64 = (1.0 - locals.var_delta_ns__blk1347);
        let assign48810_e62625: f64 = (locals.var_xn_s__blk1332 * assign48810_e62624);
        let assign48810_e62626: f64 = (assign48810_e62625).sqrt();
        let assign48810_e62627: f64 = (2.0 * assign48810_e62626);
        let assign48810_e62628: f64 = (assign48810_e62619 / assign48810_e62627);
        let assign48810_e62629: f64 = (1.0 + assign48810_e62628);
        (assign48810_e62629, (((((((locals.var_temp__blk936_dn5 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn5)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn5 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn5)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn5 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn5))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn6 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn6)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn6 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn6)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn6 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn6))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn7 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn7)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn7 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn7)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn7 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn7))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn8 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn8)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn8 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn8)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn8 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn8))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48810_e62631;
        locals.var_nscr__blk1333_dn5 = assign48810_e62631_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48810_e62631_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48810_e62631_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48810_e62631_d_n8;

        let (assign48820_e62647, assign48820_e62647_d_n5, assign48820_e62647_d_n6, assign48820_e62647_d_n7, assign48820_e62647_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 == 0.0)) {
        let assign48820_e62641: f64 = (0.5 * locals.var_gf__blk1307);
        let assign48820_e62643: f64 = (locals.var_xn_s__blk1332).sqrt();
        let assign48820_e62644: f64 = (assign48820_e62641 / assign48820_e62643);
        let assign48820_e62645: f64 = (1.0 + assign48820_e62644);
        (assign48820_e62645, ((((0.5 * locals.var_gf__blk1307_dn5) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn6) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn7) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn8) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)),)
    } else {
        (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8,)
    }
};
        locals.var_nscr__blk1333 = assign48820_e62647;
        locals.var_nscr__blk1333_dn5 = assign48820_e62647_d_n5;
        locals.var_nscr__blk1333_dn6 = assign48820_e62647_d_n6;
        locals.var_nscr__blk1333_dn7 = assign48820_e62647_d_n7;
        locals.var_nscr__blk1333_dn8 = assign48820_e62647_d_n8;

        let (assign48830_e62665, assign48830_e62665_d_n5, assign48830_e62665_d_n6, assign48830_e62665_d_n7, assign48830_e62665_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48830_e62654: f64 = (locals.var_xn_s__blk1332).sqrt();
        let assign48830_e62655: f64 = (locals.var_gf__blk1307 * assign48830_e62654);
        let assign48830_e62656: f64 = (locals.var_xn_s__blk1332 + assign48830_e62655);
        let assign48830_e62660: f64 = (locals.var_nscr__blk1333 - 1.0);
        let assign48830_e62661: f64 = (assign48830_e62660).ln();
        let assign48830_e62662: f64 = (locals.var_nscr__blk1333 * assign48830_e62661);
        let assign48830_e62663: f64 = (assign48830_e62656 - assign48830_e62662);
        (assign48830_e62663, ((locals.var_xn_s__blk1332_dn5 + ((locals.var_gf__blk1307_dn5 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn5 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn5 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn6 + ((locals.var_gf__blk1307_dn6 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn6 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn6 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn7 + ((locals.var_gf__blk1307_dn7 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn7 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn7 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn8 + ((locals.var_gf__blk1307_dn8 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn8 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn8 / assign48830_e62660)))),)
    } else {
        (locals.var_xthscr__blk1334, locals.var_xthscr__blk1334_dn5, locals.var_xthscr__blk1334_dn6, locals.var_xthscr__blk1334_dn7, locals.var_xthscr__blk1334_dn8,)
    }
};
        locals.var_xthscr__blk1334 = assign48830_e62665;
        locals.var_xthscr__blk1334_dn5 = assign48830_e62665_d_n5;
        locals.var_xthscr__blk1334_dn6 = assign48830_e62665_d_n6;
        locals.var_xthscr__blk1334_dn7 = assign48830_e62665_d_n7;
        locals.var_xthscr__blk1334_dn8 = assign48830_e62665_d_n8;

        let (assign48840_e62675, assign48840_e62675_d_n5, assign48840_e62675_d_n6, assign48840_e62675_d_n7, assign48840_e62675_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48840_e62671: f64 = (locals.var_xg__blk1326 - locals.var_xthscr__blk1334);
        let assign48840_e62673: f64 = (assign48840_e62671 / locals.var_nscr__blk1333);
        (assign48840_e62673, ((((locals.var_xg__blk1326_dn5 - locals.var_xthscr__blk1334_dn5) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn6 - locals.var_xthscr__blk1334_dn6) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn7 - locals.var_xthscr__blk1334_dn7) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn8 - locals.var_xthscr__blk1334_dn8) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)),)
    } else {
        (locals.var_xgtscr__blk1335, locals.var_xgtscr__blk1335_dn5, locals.var_xgtscr__blk1335_dn6, locals.var_xgtscr__blk1335_dn7, locals.var_xgtscr__blk1335_dn8,)
    }
};
        locals.var_xgtscr__blk1335 = assign48840_e62675;
        locals.var_xgtscr__blk1335_dn5 = assign48840_e62675_d_n5;
        locals.var_xgtscr__blk1335_dn6 = assign48840_e62675_d_n6;
        locals.var_xgtscr__blk1335_dn7 = assign48840_e62675_d_n7;
        locals.var_xgtscr__blk1335_dn8 = assign48840_e62675_d_n8;

        let (assign48850_e62692, assign48850_e62692_d_n5, assign48850_e62692_d_n6, assign48850_e62692_d_n7, assign48850_e62692_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign48850_e62681: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign48850_e62685: f64 = (8.0 / locals.var_gf2__blk1308);
        let assign48850_e62686: f64 = (1.0 + assign48850_e62685);
        let assign48850_e62687: f64 = (assign48850_e62686).sqrt();
        let assign48850_e62689: f64 = (assign48850_e62687 - 1.0);
        let assign48850_e62690: f64 = (assign48850_e62681 * assign48850_e62689);
        (assign48850_e62690, (((0.5 * locals.var_gf2__blk1308_dn5) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))),)
    } else {
        (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8,)
    }
};
        locals.var_qbscr__blk1341 = assign48850_e62692;
        locals.var_qbscr__blk1341_dn5 = assign48850_e62692_d_n5;
        locals.var_qbscr__blk1341_dn6 = assign48850_e62692_d_n6;
        locals.var_qbscr__blk1341_dn7 = assign48850_e62692_d_n7;
        locals.var_qbscr__blk1341_dn8 = assign48850_e62692_d_n8;

        let (assign48860_e62698, assign48860_e62698_d_n5, assign48860_e62698_d_n6, assign48860_e62698_d_n7, assign48860_e62698_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign48860_e62698;
        locals.var_qiscr__blk1340_dn5 = assign48860_e62698_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign48860_e62698_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign48860_e62698_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign48860_e62698_d_n8;

        let (assign48870_e62704, assign48870_e62704_d_n5, assign48870_e62704_d_n6, assign48870_e62704_d_n7, assign48870_e62704_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8,)
    }
};
        locals.var_fscr__blk1342 = assign48870_e62704;
        locals.var_fscr__blk1342_dn5 = assign48870_e62704_d_n5;
        locals.var_fscr__blk1342_dn6 = assign48870_e62704_d_n6;
        locals.var_fscr__blk1342_dn7 = assign48870_e62704_d_n7;
        locals.var_fscr__blk1342_dn8 = assign48870_e62704_d_n8;

        let assign48880_e62707: f64 = (-30.0);
        let assign48880_e62708: f64 = if locals.var_xgtscr__blk1335 > assign48880_e62707 { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign48880_e62708;

        let (assign48890_e62720, assign48890_e62720_d_n5, assign48890_e62720_d_n6, assign48890_e62720_d_n7, assign48890_e62720_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48890_e62716: f64 = (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335);
        let assign48890_e62718: f64 = (assign48890_e62716 - 1.0);
        (assign48890_e62718, ((locals.var_nscr__blk1333_dn5 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn5)), ((locals.var_nscr__blk1333_dn6 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn6)), ((locals.var_nscr__blk1333_dn7 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn7)), ((locals.var_nscr__blk1333_dn8 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn8)),)
    } else {
        (locals.var_xgtscr0__blk1336, locals.var_xgtscr0__blk1336_dn5, locals.var_xgtscr0__blk1336_dn6, locals.var_xgtscr0__blk1336_dn7, locals.var_xgtscr0__blk1336_dn8,)
    }
};
        locals.var_xgtscr0__blk1336 = assign48890_e62720;
        locals.var_xgtscr0__blk1336_dn5 = assign48890_e62720_d_n5;
        locals.var_xgtscr0__blk1336_dn6 = assign48890_e62720_d_n6;
        locals.var_xgtscr0__blk1336_dn7 = assign48890_e62720_d_n7;
        locals.var_xgtscr0__blk1336_dn8 = assign48890_e62720_d_n8;

        let (assign48900_e62737, assign48900_e62737_d_n5, assign48900_e62737_d_n6, assign48900_e62737_d_n7, assign48900_e62737_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48900_e62730: f64 = (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336);
        let assign48900_e62732: f64 = (assign48900_e62730 + 10.0);
        let assign48900_e62733: f64 = (assign48900_e62732).sqrt();
        let assign48900_e62734: f64 = (locals.var_xgtscr0__blk1336 + assign48900_e62733);
        let assign48900_e62735: f64 = (0.5 * assign48900_e62734);
        (assign48900_e62735, (0.5 * (locals.var_xgtscr0__blk1336_dn5 + (((locals.var_xgtscr0__blk1336_dn5 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn5)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn6 + (((locals.var_xgtscr0__blk1336_dn6 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn6)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn7 + (((locals.var_xgtscr0__blk1336_dn7 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn7)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn8 + (((locals.var_xgtscr0__blk1336_dn8 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn8)) / (2.0 * assign48900_e62733)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48900_e62737;
        locals.var_temp__blk936_dn5 = assign48900_e62737_d_n5;
        locals.var_temp__blk936_dn6 = assign48900_e62737_d_n6;
        locals.var_temp__blk936_dn7 = assign48900_e62737_d_n7;
        locals.var_temp__blk936_dn8 = assign48900_e62737_d_n8;

        let (assign48910_e62748, assign48910_e62748_d_n5, assign48910_e62748_d_n6, assign48910_e62748_d_n7, assign48910_e62748_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48910_e62745: f64 = (locals.var_temp__blk936).ln();
        let assign48910_e62746: f64 = (locals.var_xgtscr__blk1335 - assign48910_e62745);
        (assign48910_e62746, (locals.var_xgtscr__blk1335_dn5 - (locals.var_temp__blk936_dn5 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn6 - (locals.var_temp__blk936_dn6 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn7 - (locals.var_temp__blk936_dn7 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn8 - (locals.var_temp__blk936_dn8 / locals.var_temp__blk936)),)
    } else {
        (locals.var_qiscr0si__blk1337, locals.var_qiscr0si__blk1337_dn5, locals.var_qiscr0si__blk1337_dn6, locals.var_qiscr0si__blk1337_dn7, locals.var_qiscr0si__blk1337_dn8,)
    }
};
        locals.var_qiscr0si__blk1337 = assign48910_e62748;
        locals.var_qiscr0si__blk1337_dn5 = assign48910_e62748_d_n5;
        locals.var_qiscr0si__blk1337_dn6 = assign48910_e62748_d_n6;
        locals.var_qiscr0si__blk1337_dn7 = assign48910_e62748_d_n7;
        locals.var_qiscr0si__blk1337_dn8 = assign48910_e62748_d_n8;

        let (assign48920_e62765, assign48920_e62765_d_n5, assign48920_e62765_d_n6, assign48920_e62765_d_n7, assign48920_e62765_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48920_e62758: f64 = (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337);
        let assign48920_e62760: f64 = (assign48920_e62758 + 2.0);
        let assign48920_e62761: f64 = (assign48920_e62760).sqrt();
        let assign48920_e62762: f64 = (locals.var_qiscr0si__blk1337 + assign48920_e62761);
        let assign48920_e62763: f64 = (0.5 * assign48920_e62762);
        (assign48920_e62763, (0.5 * (locals.var_qiscr0si__blk1337_dn5 + (((locals.var_qiscr0si__blk1337_dn5 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn5)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn6 + (((locals.var_qiscr0si__blk1337_dn6 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn6)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn7 + (((locals.var_qiscr0si__blk1337_dn7 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn7)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn8 + (((locals.var_qiscr0si__blk1337_dn8 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn8)) / (2.0 * assign48920_e62761)))),)
    } else {
        (locals.var_qiscr0__blk1338, locals.var_qiscr0__blk1338_dn5, locals.var_qiscr0__blk1338_dn6, locals.var_qiscr0__blk1338_dn7, locals.var_qiscr0__blk1338_dn8,)
    }
};
        locals.var_qiscr0__blk1338 = assign48920_e62765;
        locals.var_qiscr0__blk1338_dn5 = assign48920_e62765_d_n5;
        locals.var_qiscr0__blk1338_dn6 = assign48920_e62765_d_n6;
        locals.var_qiscr0__blk1338_dn7 = assign48920_e62765_d_n7;
        locals.var_qiscr0__blk1338_dn8 = assign48920_e62765_d_n8;

        let assign48930_e62768: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48930_e62770: f64 = if assign48930_e62768 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign48930_e62770;

        let (assign48940_e62783, assign48940_e62783_d_n5, assign48940_e62783_d_n6, assign48940_e62783_d_n7, assign48940_e62783_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign48940_e62780: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48940_e62781: f64 = (assign48940_e62780).exp();
        (assign48940_e62781, (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48940_e62783;
        locals.var_temp__blk936_dn5 = assign48940_e62783_d_n5;
        locals.var_temp__blk936_dn6 = assign48940_e62783_d_n6;
        locals.var_temp__blk936_dn7 = assign48940_e62783_d_n7;
        locals.var_temp__blk936_dn8 = assign48940_e62783_d_n8;

        let (assign48950_e62822, assign48950_e62822_d_n5, assign48950_e62822_d_n6, assign48950_e62822_d_n7, assign48950_e62822_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) {
        let assign48950_e62796: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62798: f64 = (assign48950_e62796 - 230.25850929940458);
        let assign48950_e62803: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62805: f64 = (assign48950_e62803 - 230.25850929940458);
        let assign48950_e62809: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48950_e62811: f64 = (assign48950_e62809 - 230.25850929940458);
        let assign48950_e62813: f64 = (assign48950_e62811 * 0.3333333333333333);
        let assign48950_e62814: f64 = (1.0 + assign48950_e62813);
        let assign48950_e62815: f64 = (assign48950_e62805 * assign48950_e62814);
        let assign48950_e62816: f64 = (0.5 * assign48950_e62815);
        let assign48950_e62817: f64 = (1.0 + assign48950_e62816);
        let assign48950_e62818: f64 = (assign48950_e62798 * assign48950_e62817);
        let assign48950_e62819: f64 = (1.0 + assign48950_e62818);
        let assign48950_e62820: f64 = (1e100 * assign48950_e62819);
        (assign48950_e62820, (1e100 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48950_e62822;
        locals.var_temp__blk936_dn5 = assign48950_e62822_d_n5;
        locals.var_temp__blk936_dn6 = assign48950_e62822_d_n6;
        locals.var_temp__blk936_dn7 = assign48950_e62822_d_n7;
        locals.var_temp__blk936_dn8 = assign48950_e62822_d_n8;

        let (assign48960_e62832, assign48960_e62832_d_n5, assign48960_e62832_d_n6, assign48960_e62832_d_n7, assign48960_e62832_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48960_e62830: f64 = (locals.var_temp__blk936 / locals.var_nscr__blk1333);
        (assign48960_e62830, (((locals.var_temp__blk936_dn5 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn6 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn7 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn8 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)),)
    } else {
        (locals.var_dscr0__blk1339, locals.var_dscr0__blk1339_dn5, locals.var_dscr0__blk1339_dn6, locals.var_dscr0__blk1339_dn7, locals.var_dscr0__blk1339_dn8,)
    }
};
        locals.var_dscr0__blk1339 = assign48960_e62832;
        locals.var_dscr0__blk1339_dn5 = assign48960_e62832_d_n5;
        locals.var_dscr0__blk1339_dn6 = assign48960_e62832_d_n6;
        locals.var_dscr0__blk1339_dn7 = assign48960_e62832_d_n7;
        locals.var_dscr0__blk1339_dn8 = assign48960_e62832_d_n8;

        let (assign48970_e62846, assign48970_e62846_d_n5, assign48970_e62846_d_n6, assign48970_e62846_d_n7, assign48970_e62846_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign48970_e62841: f64 = (locals.var_qiscr0__blk1338 + 1.0);
        let assign48970_e62842: f64 = (2.0 * assign48970_e62841);
        let assign48970_e62844: f64 = (assign48970_e62842 - locals.var_dscr0__blk1339);
        (assign48970_e62844, ((2.0 * locals.var_qiscr0__blk1338_dn5) - locals.var_dscr0__blk1339_dn5), ((2.0 * locals.var_qiscr0__blk1338_dn6) - locals.var_dscr0__blk1339_dn6), ((2.0 * locals.var_qiscr0__blk1338_dn7) - locals.var_dscr0__blk1339_dn7), ((2.0 * locals.var_qiscr0__blk1338_dn8) - locals.var_dscr0__blk1339_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign48970_e62846;
        locals.var_temp__blk936_dn5 = assign48970_e62846_d_n5;
        locals.var_temp__blk936_dn6 = assign48970_e62846_d_n6;
        locals.var_temp__blk936_dn7 = assign48970_e62846_d_n7;
        locals.var_temp__blk936_dn8 = assign48970_e62846_d_n8;

        let assign48980_e62849: f64 = if locals.var_dscr0__blk1339 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign48980_e62849;

        let (assign48990_e62874, assign48990_e62874_d_n5, assign48990_e62874_d_n6, assign48990_e62874_d_n7, assign48990_e62874_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        let assign48990_e62862: f64 = (locals.var_dscr0__blk1339 * locals.var_temp__blk936);
        let assign48990_e62863: f64 = (1.0 + assign48990_e62862);
        let assign48990_e62864: f64 = (assign48990_e62863).sqrt();
        let assign48990_e62866: f64 = (assign48990_e62864 - 1.0);
        let assign48990_e62868: f64 = (assign48990_e62866 / locals.var_dscr0__blk1339);
        let assign48990_e62869: f64 = (locals.var_qiscr0__blk1338 - assign48990_e62868);
        let assign48990_e62871: f64 = (assign48990_e62869 + 1.0);
        let assign48990_e62872: f64 = (locals.var_nscr__blk1333 * assign48990_e62871);
        (assign48990_e62872, ((locals.var_nscr__blk1333_dn5 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn5 - ((((((locals.var_dscr0__blk1339_dn5 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn5)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn5)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn6 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn6 - ((((((locals.var_dscr0__blk1339_dn6 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn6)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn6)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn7 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn7 - ((((((locals.var_dscr0__blk1339_dn7 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn7)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn7)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn8 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn8 - ((((((locals.var_dscr0__blk1339_dn8 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn8)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn8)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))),)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign48990_e62874;
        locals.var_qiscr__blk1340_dn5 = assign48990_e62874_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign48990_e62874_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign48990_e62874_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign48990_e62874_d_n8;

        let (assign49000_e62897, assign49000_e62897_d_n5, assign49000_e62897_d_n6, assign49000_e62897_d_n7, assign49000_e62897_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign49000_e62885: f64 = (locals.var_nscr__blk1333 * 0.5);
        let assign49000_e62887: f64 = (assign49000_e62885 * locals.var_dscr0__blk1339);
        let assign49000_e62891: f64 = (0.25 * locals.var_temp__blk936);
        let assign49000_e62893: f64 = (assign49000_e62891 * locals.var_temp__blk936);
        let assign49000_e62894: f64 = (1.0 + assign49000_e62893);
        let assign49000_e62895: f64 = (assign49000_e62887 * assign49000_e62894);
        (assign49000_e62895, (((((locals.var_nscr__blk1333_dn5 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn5)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn5)))), (((((locals.var_nscr__blk1333_dn6 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn6)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn6)))), (((((locals.var_nscr__blk1333_dn7 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn7)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn7)))), (((((locals.var_nscr__blk1333_dn8 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn8)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn8)))),)
    } else {
        (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8,)
    }
};
        locals.var_qiscr__blk1340 = assign49000_e62897;
        locals.var_qiscr__blk1340_dn5 = assign49000_e62897_d_n5;
        locals.var_qiscr__blk1340_dn6 = assign49000_e62897_d_n6;
        locals.var_qiscr__blk1340_dn7 = assign49000_e62897_d_n7;
        locals.var_qiscr__blk1340_dn8 = assign49000_e62897_d_n8;

        let (assign49010_e62926, assign49010_e62926_d_n5, assign49010_e62926_d_n6, assign49010_e62926_d_n7, assign49010_e62926_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49010_e62906: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62908: f64 = (assign49010_e62906 + 2.0);
        let assign49010_e62911: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62913: f64 = (assign49010_e62911 - 2.0);
        let assign49010_e62916: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
        let assign49010_e62918: f64 = (assign49010_e62916 - 2.0);
        let assign49010_e62919: f64 = (assign49010_e62913 * assign49010_e62918);
        let assign49010_e62921: f64 = (assign49010_e62919 + 1.0);
        let assign49010_e62922: f64 = (assign49010_e62921).sqrt();
        let assign49010_e62923: f64 = (assign49010_e62908 + assign49010_e62922);
        let assign49010_e62924: f64 = (0.5 * assign49010_e62923);
        (assign49010_e62924, (0.5 * ((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) + ((((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) + ((((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) + ((((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) + ((((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8))) / (2.0 * assign49010_e62922)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign49010_e62926;
        locals.var_temp__blk936_dn5 = assign49010_e62926_d_n5;
        locals.var_temp__blk936_dn6 = assign49010_e62926_d_n6;
        locals.var_temp__blk936_dn7 = assign49010_e62926_d_n7;
        locals.var_temp__blk936_dn8 = assign49010_e62926_d_n8;

        let (assign49020_e62947, assign49020_e62947_d_n5, assign49020_e62947_d_n6, assign49020_e62947_d_n7, assign49020_e62947_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49020_e62934: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign49020_e62938: f64 = (4.0 / locals.var_gf2__blk1308);
        let assign49020_e62940: f64 = (assign49020_e62938 * locals.var_temp__blk936);
        let assign49020_e62941: f64 = (1.0 + assign49020_e62940);
        let assign49020_e62942: f64 = (assign49020_e62941).sqrt();
        let assign49020_e62944: f64 = (assign49020_e62942 - 1.0);
        let assign49020_e62945: f64 = (assign49020_e62934 * assign49020_e62944);
        (assign49020_e62945, (((0.5 * locals.var_gf2__blk1308_dn5) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn5)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn6)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn7)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn8)) / (2.0 * assign49020_e62942)))),)
    } else {
        (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8,)
    }
};
        locals.var_qbscr__blk1341 = assign49020_e62947;
        locals.var_qbscr__blk1341_dn5 = assign49020_e62947_d_n5;
        locals.var_qbscr__blk1341_dn6 = assign49020_e62947_d_n6;
        locals.var_qbscr__blk1341_dn7 = assign49020_e62947_d_n7;
        locals.var_qbscr__blk1341_dn8 = assign49020_e62947_d_n8;

        let (assign49030_e62959, assign49030_e62959_d_n5, assign49030_e62959_d_n6, assign49030_e62959_d_n7, assign49030_e62959_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49030_e62956: f64 = (locals.var_qbscr__blk1341 + locals.var_qiscr__blk1340);
        let assign49030_e62957: f64 = (locals.var_qbscr__blk1341 / assign49030_e62956);
        (assign49030_e62957, (((locals.var_qbscr__blk1341_dn5 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn5 + locals.var_qiscr__blk1340_dn5))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn6 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn6 + locals.var_qiscr__blk1340_dn6))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn7 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn7 + locals.var_qiscr__blk1340_dn7))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn8 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn8 + locals.var_qiscr__blk1340_dn8))) / (assign49030_e62956 * assign49030_e62956)),)
    } else {
        (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8,)
    }
};
        locals.var_fscr__blk1342 = assign49030_e62959;
        locals.var_fscr__blk1342_dn5 = assign49030_e62959_d_n5;
        locals.var_fscr__blk1342_dn6 = assign49030_e62959_d_n6;
        locals.var_fscr__blk1342_dn7 = assign49030_e62959_d_n7;
        locals.var_fscr__blk1342_dn8 = assign49030_e62959_d_n8;

        let (assign49040_e62971, assign49040_e62971_d_n5, assign49040_e62971_d_n6, assign49040_e62971_d_n7, assign49040_e62971_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign49040_e62968: f64 = (locals.var_fscr__blk1342 * locals.var_delxb__blk1330);
        let assign49040_e62969: f64 = (locals.var_xno_s__blk1331 - assign49040_e62968);
        (assign49040_e62969, (locals.var_xno_s__blk1331_dn5 - ((locals.var_fscr__blk1342_dn5 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn5))), (locals.var_xno_s__blk1331_dn6 - ((locals.var_fscr__blk1342_dn6 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn6))), (locals.var_xno_s__blk1331_dn7 - ((locals.var_fscr__blk1342_dn7 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn7))), (locals.var_xno_s__blk1331_dn8 - ((locals.var_fscr__blk1342_dn8 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn8))),)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign49040_e62971;
        locals.var_xn_s__blk1332_dn5 = assign49040_e62971_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign49040_e62971_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign49040_e62971_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign49040_e62971_d_n8;

        let (assign49050_e62981, assign49050_e62981_d_n5, assign49050_e62981_d_n6, assign49050_e62981_d_n7, assign49050_e62981_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49050_e62978: f64 = (locals.var_gf__blk1307 * 0.7071067811865475);
        let assign49050_e62979: f64 = (1.0 + assign49050_e62978);
        (assign49050_e62979, (locals.var_gf__blk1307_dn5 * 0.7071067811865475), (locals.var_gf__blk1307_dn6 * 0.7071067811865475), (locals.var_gf__blk1307_dn7 * 0.7071067811865475), (locals.var_gf__blk1307_dn8 * 0.7071067811865475),)
    } else {
        (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8,)
    }
};
        locals.var_xi__blk1343 = assign49050_e62981;
        locals.var_xi__blk1343_dn5 = assign49050_e62981_d_n5;
        locals.var_xi__blk1343_dn6 = assign49050_e62981_d_n6;
        locals.var_xi__blk1343_dn7 = assign49050_e62981_d_n7;
        locals.var_xi__blk1343_dn8 = assign49050_e62981_d_n8;

        let (assign49060_e62989,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49060_e62987: f64 = (1e-5 * locals.var_xi__blk1343);
        (assign49060_e62987,)
    } else {
        (locals.var_margin__blk1344,)
    }
};
        locals.var_margin__blk1344 = assign49060_e62989;

        let (assign49070_e62997, assign49070_e62997_d_n5, assign49070_e62997_d_n6, assign49070_e62997_d_n7, assign49070_e62997_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49070_e62995: f64 = (1.0 / locals.var_xi__blk1343);
        (assign49070_e62995, (-(locals.var_xi__blk1343_dn5 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn6 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn7 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn8 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))),)
    } else {
        (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8,)
    }
};
        locals.var_inv_xi__blk1345 = assign49070_e62997;
        locals.var_inv_xi__blk1345_dn5 = assign49070_e62997_d_n5;
        locals.var_inv_xi__blk1345_dn6 = assign49070_e62997_d_n6;
        locals.var_inv_xi__blk1345_dn7 = assign49070_e62997_d_n7;
        locals.var_inv_xi__blk1345_dn8 = assign49070_e62997_d_n8;

        let (assign49080_e63003, assign49080_e63003_d_n5, assign49080_e63003_d_n6, assign49080_e63003_d_n7, assign49080_e63003_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign49080_e63003;
        locals.var_sp_s_x1__blk1452_dn5 = assign49080_e63003_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign49080_e63003_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign49080_e63003_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign49080_e63003_d_n8;

        let (assign49090_e63009, assign49090_e63009_d_n5, assign49090_e63009_d_n6, assign49090_e63009_d_n7, assign49090_e63009_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49090_e63009;
        locals.var_x_s__blk1346_dn5 = assign49090_e63009_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49090_e63009_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49090_e63009_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49090_e63009_d_n8;

        let assign49100_e63012: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign49100_e63012;

        let (assign49110_e63022, assign49110_e63022_d_n5, assign49110_e63022_d_n6, assign49110_e63022_d_n7, assign49110_e63022_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign49110_e63019: f64 = (-locals.var_xn_s__blk1332);
        let assign49110_e63020: f64 = (assign49110_e63019).exp();
        (assign49110_e63020, (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn5)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn6)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn7)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign49110_e63022;
        locals.var_delta_ns__blk1347_dn5 = assign49110_e63022_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign49110_e63022_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign49110_e63022_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign49110_e63022_d_n8;

        let (assign49120_e63053, assign49120_e63053_d_n5, assign49120_e63053_d_n6, assign49120_e63053_d_n7, assign49120_e63053_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 == 0.0)) {
        let assign49120_e63033: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63038: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63042: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
        let assign49120_e63044: f64 = (assign49120_e63042 * 0.3333333333333333);
        let assign49120_e63045: f64 = (1.0 + assign49120_e63044);
        let assign49120_e63046: f64 = (assign49120_e63038 * assign49120_e63045);
        let assign49120_e63047: f64 = (0.5 * assign49120_e63046);
        let assign49120_e63048: f64 = (1.0 + assign49120_e63047);
        let assign49120_e63049: f64 = (assign49120_e63033 * assign49120_e63048);
        let assign49120_e63050: f64 = (1.0 + assign49120_e63049);
        let assign49120_e63051: f64 = (1e-200 / assign49120_e63050);
        (assign49120_e63051, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))),)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign49120_e63053;
        locals.var_delta_ns__blk1347_dn5 = assign49120_e63053_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign49120_e63053_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign49120_e63053_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign49120_e63053_d_n8;

        let assign49130_e63055: f64 = (locals.var_xg__blk1326).abs();
        let assign49130_e63057: f64 = if assign49130_e63055 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign49130_e63057;

        let (assign49140_e63071, assign49140_e63071_d_n5, assign49140_e63071_d_n6, assign49140_e63071_d_n7, assign49140_e63071_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign49140_e63065: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
        let assign49140_e63067: f64 = (assign49140_e63065 * 0.16666666666666666);
        let assign49140_e63069: f64 = (assign49140_e63067 * 0.7071067811865475);
        (assign49140_e63069, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49140_e63071;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49140_e63071_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49140_e63071_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49140_e63071_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49140_e63071_d_n8;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign49150_e63093, assign49150_e63093_d_n5, assign49150_e63093_d_n6, assign49150_e63093_d_n7, assign49150_e63093_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign49150_e63079: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign49150_e63084: f64 = (1.0 - locals.var_delta_ns__blk1347);
        let assign49150_e63085: f64 = (locals.var_xg__blk1326 * assign49150_e63084);
        let assign49150_e63087: f64 = (assign49150_e63085 * locals.var_gf__blk1307);
        let assign49150_e63089: f64 = (assign49150_e63087 * locals.var_sp_s_temp1__blk1432);
        let assign49150_e63090: f64 = (1.0 + assign49150_e63089);
        let assign49150_e63091: f64 = (assign49150_e63079 * assign49150_e63090);
        (assign49150_e63091, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn5 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn5))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn6 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn6))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn7 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn7))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn8 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn8))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn8)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49150_e63093;
        locals.var_x_s__blk1346_dn5 = assign49150_e63093_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49150_e63093_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49150_e63093_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49150_e63093_d_n8;

        let assign49160_e63096: f64 = (-locals.var_margin__blk1344);
        let assign49160_e63097: f64 = if locals.var_xg__blk1326 < assign49160_e63096 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign49160_e63097;

        let (assign49170_e63109, assign49170_e63109_d_n5, assign49170_e63109_d_n6, assign49170_e63109_d_n7, assign49170_e63109_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49170_e63107: f64 = (-locals.var_xg__blk1326);
        (assign49170_e63107, (-locals.var_xg__blk1326_dn5), (-locals.var_xg__blk1326_dn6), (-locals.var_xg__blk1326_dn7), (-locals.var_xg__blk1326_dn8),)
    } else {
        (locals.var_sp_s_yg__blk1434, locals.var_sp_s_yg__blk1434_dn5, locals.var_sp_s_yg__blk1434_dn6, locals.var_sp_s_yg__blk1434_dn7, locals.var_sp_s_yg__blk1434_dn8,)
    }
};
        locals.var_sp_s_yg__blk1434 = assign49170_e63109;
        locals.var_sp_s_yg__blk1434_dn5 = assign49170_e63109_d_n5;
        locals.var_sp_s_yg__blk1434_dn6 = assign49170_e63109_d_n6;
        locals.var_sp_s_yg__blk1434_dn7 = assign49170_e63109_d_n7;
        locals.var_sp_s_yg__blk1434_dn8 = assign49170_e63109_d_n8;

        let (assign49180_e63124, assign49180_e63124_d_n5, assign49180_e63124_d_n6, assign49180_e63124_d_n7, assign49180_e63124_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49180_e63121: f64 = (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345);
        let assign49180_e63122: f64 = (1.25 * assign49180_e63121);
        (assign49180_e63122, (1.25 * ((locals.var_sp_s_yg__blk1434_dn5 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn5))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn6 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn7 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn8 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn8))),)
    } else {
        (locals.var_sp_s_ysub__blk1435, locals.var_sp_s_ysub__blk1435_dn5, locals.var_sp_s_ysub__blk1435_dn6, locals.var_sp_s_ysub__blk1435_dn7, locals.var_sp_s_ysub__blk1435_dn8,)
    }
};
        locals.var_sp_s_ysub__blk1435 = assign49180_e63124;
        locals.var_sp_s_ysub__blk1435_dn5 = assign49180_e63124_d_n5;
        locals.var_sp_s_ysub__blk1435_dn6 = assign49180_e63124_d_n6;
        locals.var_sp_s_ysub__blk1435_dn7 = assign49180_e63124_d_n7;
        locals.var_sp_s_ysub__blk1435_dn8 = assign49180_e63124_d_n8;

        let (assign49190_e63150, assign49190_e63150_d_n5, assign49190_e63150_d_n6, assign49190_e63150_d_n7, assign49190_e63150_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49190_e63136: f64 = (locals.var_sp_s_ysub__blk1435 + 10.0);
        let assign49190_e63139: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
        let assign49190_e63142: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
        let assign49190_e63143: f64 = (assign49190_e63139 * assign49190_e63142);
        let assign49190_e63145: f64 = (assign49190_e63143 + 64.0);
        let assign49190_e63146: f64 = (assign49190_e63145).sqrt();
        let assign49190_e63147: f64 = (assign49190_e63136 - assign49190_e63146);
        let assign49190_e63148: f64 = (0.5 * assign49190_e63147);
        (assign49190_e63148, (0.5 * (locals.var_sp_s_ysub__blk1435_dn5 - (((locals.var_sp_s_ysub__blk1435_dn5 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn5)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn6 - (((locals.var_sp_s_ysub__blk1435_dn6 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn6)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn7 - (((locals.var_sp_s_ysub__blk1435_dn7 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn7)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn8 - (((locals.var_sp_s_ysub__blk1435_dn8 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn8)) / (2.0 * assign49190_e63146)))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign49190_e63150;
        locals.var_sp_s_eta__blk1436_dn5 = assign49190_e63150_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign49190_e63150_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign49190_e63150_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign49190_e63150_d_n8;

        let (assign49200_e63163, assign49200_e63163_d_n5, assign49200_e63163_d_n6, assign49200_e63163_d_n7, assign49200_e63163_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49200_e63161: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_eta__blk1436);
        (assign49200_e63161, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49200_e63163;
        locals.var_sp_s_temp__blk1431_dn5 = assign49200_e63163_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49200_e63163_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49200_e63163_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49200_e63163_d_n8;

        let (assign49210_e63182, assign49210_e63182_d_n5, assign49210_e63182_d_n6, assign49210_e63182_d_n7, assign49210_e63182_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49210_e63174: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49210_e63178: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign49210_e63179: f64 = (locals.var_gf2__blk1308 * assign49210_e63178);
        let assign49210_e63180: f64 = (assign49210_e63174 + assign49210_e63179);
        (assign49210_e63180, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn5))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn6))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn7))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn8))),)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign49210_e63182;
        locals.var_sp_s_a__blk1437_dn5 = assign49210_e63182_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign49210_e63182_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign49210_e63182_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign49210_e63182_d_n8;

        let (assign49220_e63197, assign49220_e63197_d_n5, assign49220_e63197_d_n6, assign49220_e63197_d_n7, assign49220_e63197_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49220_e63193: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49220_e63195: f64 = (assign49220_e63193 - locals.var_gf2__blk1308);
        (assign49220_e63195, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) - locals.var_gf2__blk1308_dn5), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) - locals.var_gf2__blk1308_dn6), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) - locals.var_gf2__blk1308_dn7), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) - locals.var_gf2__blk1308_dn8),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign49220_e63197;
        locals.var_sp_s_c__blk1438_dn5 = assign49220_e63197_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign49220_e63197_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign49220_e63197_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign49220_e63197_d_n8;

        let (assign49230_e63214, assign49230_e63214_d_n5, assign49230_e63214_d_n6, assign49230_e63214_d_n7, assign49230_e63214_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49230_e63207: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign49230_e63210: f64 = (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324);
        let assign49230_e63211: f64 = (assign49230_e63210).ln();
        let assign49230_e63212: f64 = (assign49230_e63207 + assign49230_e63211);
        (assign49230_e63212, ((-locals.var_sp_s_eta__blk1436_dn5) + (((locals.var_sp_s_a__blk1437_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn5)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn6) + (((locals.var_sp_s_a__blk1437_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn6)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn7) + (((locals.var_sp_s_a__blk1437_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn7)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn8) + (((locals.var_sp_s_a__blk1437_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn8)) / assign49230_e63210)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign49230_e63214;
        locals.var_sp_s_tau__blk1439_dn5 = assign49230_e63214_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign49230_e63214_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign49230_e63214_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign49230_e63214_d_n8;

        let (assign49240_e63227, assign49240_e63227_d_n5, assign49240_e63227_d_n6, assign49240_e63227_d_n7, assign49240_e63227_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49240_e63225: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign49240_e63225, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign49240_e63227;
        locals.var_nu_dn5 = assign49240_e63227_d_n5;
        locals.var_nu_dn6 = assign49240_e63227_d_n6;
        locals.var_nu_dn7 = assign49240_e63227_d_n7;
        locals.var_nu_dn8 = assign49240_e63227_d_n8;

        let (assign49250_e63250, assign49250_e63250_d_n5, assign49250_e63250_d_n6, assign49250_e63250_d_n7, assign49250_e63250_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49250_e63238: f64 = (locals.var_nu * locals.var_nu);
        let assign49250_e63243: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49250_e63244: f64 = (0.5 * assign49250_e63243);
        let assign49250_e63246: f64 = (assign49250_e63244 - locals.var_sp_s_a__blk1437);
        let assign49250_e63247: f64 = (locals.var_sp_s_tau__blk1439 * assign49250_e63246);
        let assign49250_e63248: f64 = (assign49250_e63238 + assign49250_e63247);
        (assign49250_e63248, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - locals.var_sp_s_a__blk1437_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - locals.var_sp_s_a__blk1437_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - locals.var_sp_s_a__blk1437_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - locals.var_sp_s_a__blk1437_dn8)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign49250_e63250;
        locals.var_mutau_dn5 = assign49250_e63250_d_n5;
        locals.var_mutau_dn6 = assign49250_e63250_d_n6;
        locals.var_mutau_dn7 = assign49250_e63250_d_n7;
        locals.var_mutau_dn8 = assign49250_e63250_d_n8;

        let (assign49260_e63287, assign49260_e63287_d_n5, assign49260_e63287_d_n6, assign49260_e63287_d_n7, assign49260_e63287_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49260_e63262: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign49260_e63264: f64 = (assign49260_e63262 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63268: f64 = (locals.var_nu / locals.var_mutau);
        let assign49260_e63270: f64 = (assign49260_e63268 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63272: f64 = (assign49260_e63270 * locals.var_sp_s_tau__blk1439);
        let assign49260_e63274: f64 = (assign49260_e63272 * locals.var_sp_s_c__blk1438);
        let assign49260_e63277: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49260_e63279: f64 = (assign49260_e63277 * 0.3333333333333333);
        let assign49260_e63281: f64 = (assign49260_e63279 - locals.var_sp_s_a__blk1437);
        let assign49260_e63282: f64 = (assign49260_e63274 * assign49260_e63281);
        let assign49260_e63283: f64 = (locals.var_mutau + assign49260_e63282);
        let assign49260_e63284: f64 = (assign49260_e63264 / assign49260_e63283);
        let assign49260_e63285: f64 = (locals.var_sp_s_eta__blk1436 + assign49260_e63284);
        (assign49260_e63285, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn5)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn5)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn5)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn6)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn6)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn6)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn7)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn7)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn7)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn8)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn8)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn8)))))) / (assign49260_e63283 * assign49260_e63283))),)
    } else {
        (locals.var_sp_s_y0__blk1440, locals.var_sp_s_y0__blk1440_dn5, locals.var_sp_s_y0__blk1440_dn6, locals.var_sp_s_y0__blk1440_dn7, locals.var_sp_s_y0__blk1440_dn8,)
    }
};
        locals.var_sp_s_y0__blk1440 = assign49260_e63287;
        locals.var_sp_s_y0__blk1440_dn5 = assign49260_e63287_d_n5;
        locals.var_sp_s_y0__blk1440_dn6 = assign49260_e63287_d_n6;
        locals.var_sp_s_y0__blk1440_dn7 = assign49260_e63287_d_n7;
        locals.var_sp_s_y0__blk1440_dn8 = assign49260_e63287_d_n8;

        let assign49270_e63290: f64 = if locals.var_sp_s_y0__blk1440 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign49270_e63290;

        let (assign49280_e63304, assign49280_e63304_d_n5, assign49280_e63304_d_n6, assign49280_e63304_d_n7, assign49280_e63304_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
        let assign49280_e63302: f64 = (locals.var_sp_s_y0__blk1440).exp();
        (assign49280_e63302, (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn5), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn6), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn7), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49280_e63304;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49280_e63304_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49280_e63304_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49280_e63304_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49280_e63304_d_n8;

        let (assign49290_e63340, assign49290_e63340_d_n5, assign49290_e63340_d_n6, assign49290_e63340_d_n7, assign49290_e63340_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 == 0.0)) {
        let assign49290_e63320: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63325: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63329: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
        let assign49290_e63331: f64 = (assign49290_e63329 * 0.3333333333333333);
        let assign49290_e63332: f64 = (1.0 + assign49290_e63331);
        let assign49290_e63333: f64 = (assign49290_e63325 * assign49290_e63332);
        let assign49290_e63334: f64 = (0.5 * assign49290_e63333);
        let assign49290_e63335: f64 = (1.0 + assign49290_e63334);
        let assign49290_e63336: f64 = (assign49290_e63320 * assign49290_e63335);
        let assign49290_e63337: f64 = (1.0 + assign49290_e63336);
        let assign49290_e63338: f64 = (1e100 * assign49290_e63337);
        (assign49290_e63338, (1e100 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn8 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49290_e63340;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49290_e63340_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49290_e63340_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49290_e63340_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49290_e63340_d_n8;

        let (assign49300_e63353, assign49300_e63353_d_n5, assign49300_e63353_d_n6, assign49300_e63353_d_n7, assign49300_e63353_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49300_e63351: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign49300_e63351, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49300_e63353;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49300_e63353_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49300_e63353_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49300_e63353_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49300_e63353_d_n8;

        let (assign49310_e63370, assign49310_e63370_d_n5, assign49310_e63370_d_n6, assign49310_e63370_d_n7, assign49310_e63370_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49310_e63366: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
        let assign49310_e63367: f64 = (2.0 + assign49310_e63366);
        let assign49310_e63368: f64 = (1.0 / assign49310_e63367);
        (assign49310_e63368, (-(((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) / (assign49310_e63367 * assign49310_e63367))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49310_e63370;
        locals.var_sp_s_temp__blk1431_dn5 = assign49310_e63370_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49310_e63370_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49310_e63370_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49310_e63370_d_n8;

        let (assign49320_e63385, assign49320_e63385_d_n5, assign49320_e63385_d_n6, assign49320_e63385_d_n7, assign49320_e63385_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49320_e63381: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
        let assign49320_e63383: f64 = (assign49320_e63381 * locals.var_sp_s_temp__blk1431);
        (assign49320_e63383, ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49320_e63385;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49320_e63385_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49320_e63385_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49320_e63385_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49320_e63385_d_n8;

        let (assign49330_e63402, assign49330_e63402_d_n5, assign49330_e63402_d_n6, assign49330_e63402_d_n7, assign49330_e63402_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49330_e63397: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431);
        let assign49330_e63399: f64 = (assign49330_e63397 * locals.var_sp_s_temp__blk1431);
        let assign49330_e63400: f64 = (4.0 * assign49330_e63399);
        (assign49330_e63400, (4.0 * ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49330_e63402;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49330_e63402_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49330_e63402_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49330_e63402_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49330_e63402_d_n8;

        let (assign49340_e63423, assign49340_e63423_d_n5, assign49340_e63423_d_n6, assign49340_e63423_d_n7, assign49340_e63423_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49340_e63413: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign49340_e63416: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49340_e63417: f64 = (assign49340_e63413 - assign49340_e63416);
        let assign49340_e63419: f64 = (assign49340_e63417 * locals.var_sp_s_temp__blk1431);
        let assign49340_e63421: f64 = (assign49340_e63419 * locals.var_sp_s_temp__blk1431);
        (assign49340_e63421, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49340_e63423;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49340_e63423_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49340_e63423_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49340_e63423_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49340_e63423_d_n8;

        let (assign49350_e63436, assign49350_e63436_d_n5, assign49350_e63436_d_n6, assign49350_e63436_d_n7, assign49350_e63436_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49350_e63434: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_y0__blk1440);
        (assign49350_e63434, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_y0__blk1440_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_y0__blk1440_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_y0__blk1440_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_y0__blk1440_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49350_e63436;
        locals.var_sp_s_temp__blk1431_dn5 = assign49350_e63436_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49350_e63436_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49350_e63436_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49350_e63436_d_n8;

        let (assign49360_e63449, assign49360_e63449_d_n5, assign49360_e63449_d_n6, assign49360_e63449_d_n7, assign49360_e63449_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49360_e63447: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442);
        (assign49360_e63447, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49360_e63449;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49360_e63449_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49360_e63449_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49360_e63449_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49360_e63449_d_n8;

        let (assign49370_e63476, assign49370_e63476_d_n5, assign49370_e63476_d_n6, assign49370_e63476_d_n7, assign49370_e63476_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49370_e63460: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49370_e63464: f64 = (locals.var_sp_s_delta0__blk1441 - 1.0);
        let assign49370_e63466: f64 = (assign49370_e63464 - locals.var_sp_s_temp1__blk1432);
        let assign49370_e63470: f64 = (1.0 - locals.var_sp_s_xi1__blk1444);
        let assign49370_e63471: f64 = (locals.var_delta_ns__blk1347 * assign49370_e63470);
        let assign49370_e63472: f64 = (assign49370_e63466 + assign49370_e63471);
        let assign49370_e63473: f64 = (locals.var_gf2__blk1308 * assign49370_e63472);
        let assign49370_e63474: f64 = (assign49370_e63460 + assign49370_e63473);
        (assign49370_e63474, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn5))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn8))))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign49370_e63476;
        locals.var_sp_s_pc__blk1446_dn5 = assign49370_e63476_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign49370_e63476_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign49370_e63476_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign49370_e63476_d_n8;

        let (assign49380_e63507, assign49380_e63507_d_n5, assign49380_e63507_d_n6, assign49380_e63507_d_n7, assign49380_e63507_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49380_e63487: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49380_e63491: f64 = (locals.var_sp_s_delta0__blk1441 - locals.var_sp_s_y0__blk1440);
        let assign49380_e63493: f64 = (assign49380_e63491 - 1.0);
        let assign49380_e63495: f64 = (assign49380_e63493 + locals.var_sp_s_temp1__blk1432);
        let assign49380_e63499: f64 = (locals.var_sp_s_y0__blk1440 - 1.0);
        let assign49380_e63501: f64 = (assign49380_e63499 - locals.var_sp_s_xi0__blk1443);
        let assign49380_e63502: f64 = (locals.var_delta_ns__blk1347 * assign49380_e63501);
        let assign49380_e63503: f64 = (assign49380_e63495 + assign49380_e63502);
        let assign49380_e63504: f64 = (locals.var_gf2__blk1308 * assign49380_e63503);
        let assign49380_e63505: f64 = (assign49380_e63487 - assign49380_e63504);
        (assign49380_e63505, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_y0__blk1440_dn5) + locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn5 - locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_y0__blk1440_dn6) + locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn6 - locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_y0__blk1440_dn7) + locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn7 - locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_y0__blk1440_dn8) + locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn8 - locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign49380_e63507;
        locals.var_sp_s_qc__blk1447_dn5 = assign49380_e63507_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign49380_e63507_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign49380_e63507_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign49380_e63507_d_n8;

        let (assign49390_e63528, assign49390_e63528_d_n5, assign49390_e63528_d_n6, assign49390_e63528_d_n7, assign49390_e63528_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49390_e63520: f64 = (locals.var_sp_s_delta0__blk1441 + locals.var_sp_s_temp1__blk1432);
        let assign49390_e63523: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49390_e63524: f64 = (assign49390_e63520 - assign49390_e63523);
        let assign49390_e63525: f64 = (locals.var_gf2__blk1308 * assign49390_e63524);
        let assign49390_e63526: f64 = (2.0 - assign49390_e63525);
        (assign49390_e63526, (-((locals.var_gf2__blk1308_dn5 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 + locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 + locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 + locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 + locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49390_e63528;
        locals.var_sp_s_temp__blk1431_dn5 = assign49390_e63528_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49390_e63528_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49390_e63528_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49390_e63528_d_n8;

        let (assign49400_e63547, assign49400_e63547_d_n5, assign49400_e63547_d_n6, assign49400_e63547_d_n7, assign49400_e63547_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49400_e63539: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign49400_e63543: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign49400_e63544: f64 = (2.0 * assign49400_e63543);
        let assign49400_e63545: f64 = (assign49400_e63539 - assign49400_e63544);
        (assign49400_e63545, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49400_e63547;
        locals.var_sp_s_temp__blk1431_dn5 = assign49400_e63547_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49400_e63547_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49400_e63547_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49400_e63547_d_n8;

        let (assign49410_e63568, assign49410_e63568_d_n5, assign49410_e63568_d_n6, assign49410_e63568_d_n7, assign49410_e63568_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        let assign49410_e63557: f64 = (-locals.var_sp_s_y0__blk1440);
        let assign49410_e63562: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign49410_e63563: f64 = (locals.var_sp_s_pc__blk1446 + assign49410_e63562);
        let assign49410_e63564: f64 = (locals.var_sp_s_qc__blk1447 / assign49410_e63563);
        let assign49410_e63565: f64 = (2.0 * assign49410_e63564);
        let assign49410_e63566: f64 = (assign49410_e63557 - assign49410_e63565);
        (assign49410_e63566, ((-locals.var_sp_s_y0__blk1440_dn5) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49410_e63568;
        locals.var_x_s__blk1346_dn5 = assign49410_e63568_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49410_e63568_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49410_e63568_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49410_e63568_d_n8;

        let (assign49420_e63586, assign49420_e63586_d_n5, assign49420_e63586_d_n6, assign49420_e63586_d_n7, assign49420_e63586_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49420_e63582: f64 = (locals.var_gf__blk1307 * 0.7324648775608221);
        let assign49420_e63583: f64 = (1.25 + assign49420_e63582);
        let assign49420_e63584: f64 = (1.0 / assign49420_e63583);
        (assign49420_e63584, (-((locals.var_gf__blk1307_dn5 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn6 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn7 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn8 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))),)
    } else {
        (locals.var_sp_xg1__blk1448, locals.var_sp_xg1__blk1448_dn5, locals.var_sp_xg1__blk1448_dn6, locals.var_sp_xg1__blk1448_dn7, locals.var_sp_xg1__blk1448_dn8,)
    }
};
        locals.var_sp_xg1__blk1448 = assign49420_e63586;
        locals.var_sp_xg1__blk1448_dn5 = assign49420_e63586_d_n5;
        locals.var_sp_xg1__blk1448_dn6 = assign49420_e63586_d_n6;
        locals.var_sp_xg1__blk1448_dn7 = assign49420_e63586_d_n7;
        locals.var_sp_xg1__blk1448_dn8 = assign49420_e63586_d_n8;

        let (assign49430_e63606, assign49430_e63606_d_n5, assign49430_e63606_d_n6, assign49430_e63606_d_n7, assign49430_e63606_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49430_e63598: f64 = (locals.var_xi__blk1343 * 1.25);
        let assign49430_e63600: f64 = (assign49430_e63598 * locals.var_sp_xg1__blk1448);
        let assign49430_e63602: f64 = (assign49430_e63600 - 1.0);
        let assign49430_e63604: f64 = (assign49430_e63602 * locals.var_sp_xg1__blk1448);
        (assign49430_e63604, (((((locals.var_xi__blk1343_dn5 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn5)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn5)), (((((locals.var_xi__blk1343_dn6 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn6)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn6)), (((((locals.var_xi__blk1343_dn7 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn7)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn7)), (((((locals.var_xi__blk1343_dn8 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn8)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn8)),)
    } else {
        (locals.var_sp_s_a_fac__blk1449, locals.var_sp_s_a_fac__blk1449_dn5, locals.var_sp_s_a_fac__blk1449_dn6, locals.var_sp_s_a_fac__blk1449_dn7, locals.var_sp_s_a_fac__blk1449_dn8,)
    }
};
        locals.var_sp_s_a_fac__blk1449 = assign49430_e63606;
        locals.var_sp_s_a_fac__blk1449_dn5 = assign49430_e63606_d_n5;
        locals.var_sp_s_a_fac__blk1449_dn6 = assign49430_e63606_d_n6;
        locals.var_sp_s_a_fac__blk1449_dn7 = assign49430_e63606_d_n7;
        locals.var_sp_s_a_fac__blk1449_dn8 = assign49430_e63606_d_n8;

        let (assign49440_e63626, assign49440_e63626_d_n5, assign49440_e63626_d_n6, assign49440_e63626_d_n7, assign49440_e63626_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49440_e63618: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign49440_e63622: f64 = (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326);
        let assign49440_e63623: f64 = (1.0 + assign49440_e63622);
        let assign49440_e63624: f64 = (assign49440_e63618 * assign49440_e63623);
        (assign49440_e63624, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn5 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn6 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn7 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn8 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn8)))),)
    } else {
        (locals.var_sp_s_xbar__blk1450, locals.var_sp_s_xbar__blk1450_dn5, locals.var_sp_s_xbar__blk1450_dn6, locals.var_sp_s_xbar__blk1450_dn7, locals.var_sp_s_xbar__blk1450_dn8,)
    }
};
        locals.var_sp_s_xbar__blk1450 = assign49440_e63626;
        locals.var_sp_s_xbar__blk1450_dn5 = assign49440_e63626_d_n5;
        locals.var_sp_s_xbar__blk1450_dn6 = assign49440_e63626_d_n6;
        locals.var_sp_s_xbar__blk1450_dn7 = assign49440_e63626_d_n7;
        locals.var_sp_s_xbar__blk1450_dn8 = assign49440_e63626_d_n8;

        let assign49450_e63628: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49450_e63630: f64 = (-230.25850929940458);
        let assign49450_e63631: f64 = if assign49450_e63628 > assign49450_e63630 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign49450_e63631;

        let (assign49460_e63647, assign49460_e63647_d_n5, assign49460_e63647_d_n6, assign49460_e63647_d_n7, assign49460_e63647_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign49460_e63644: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49460_e63645: f64 = (assign49460_e63644).exp();
        (assign49460_e63645, (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn5)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn6)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn7)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn8)),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49460_e63647;
        locals.var_sp_s_temp__blk1431_dn5 = assign49460_e63647_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49460_e63647_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49460_e63647_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49460_e63647_d_n8;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let (assign49470_e63690, assign49470_e63690_d_n5, assign49470_e63690_d_n6, assign49470_e63690_d_n7, assign49470_e63690_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 == 0.0)) {
        let assign49470_e63663: f64 = (-230.25850929940458);
        let assign49470_e63665: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63666: f64 = (assign49470_e63663 - assign49470_e63665);
        let assign49470_e63670: f64 = (-230.25850929940458);
        let assign49470_e63672: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63673: f64 = (assign49470_e63670 - assign49470_e63672);
        let assign49470_e63676: f64 = (-230.25850929940458);
        let assign49470_e63678: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49470_e63679: f64 = (assign49470_e63676 - assign49470_e63678);
        let assign49470_e63681: f64 = (assign49470_e63679 * 0.3333333333333333);
        let assign49470_e63682: f64 = (1.0 + assign49470_e63681);
        let assign49470_e63683: f64 = (assign49470_e63673 * assign49470_e63682);
        let assign49470_e63684: f64 = (0.5 * assign49470_e63683);
        let assign49470_e63685: f64 = (1.0 + assign49470_e63684);
        let assign49470_e63686: f64 = (assign49470_e63666 * assign49470_e63685);
        let assign49470_e63687: f64 = (1.0 + assign49470_e63686);
        let assign49470_e63688: f64 = (1e-100 / assign49470_e63687);
        (assign49470_e63688, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn5)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn6)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn7)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn8)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49470_e63690;
        locals.var_sp_s_temp__blk1431_dn5 = assign49470_e63690_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49470_e63690_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49470_e63690_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49470_e63690_d_n8;

        let (assign49480_e63704, assign49480_e63704_d_n5, assign49480_e63704_d_n6, assign49480_e63704_d_n7, assign49480_e63704_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49480_e63702: f64 = (1.0 - locals.var_sp_s_temp__blk1431);
        (assign49480_e63702, (-locals.var_sp_s_temp__blk1431_dn5), (-locals.var_sp_s_temp__blk1431_dn6), (-locals.var_sp_s_temp__blk1431_dn7), (-locals.var_sp_s_temp__blk1431_dn8),)
    } else {
        (locals.var_sp_s_w__blk1451, locals.var_sp_s_w__blk1451_dn5, locals.var_sp_s_w__blk1451_dn6, locals.var_sp_s_w__blk1451_dn7, locals.var_sp_s_w__blk1451_dn8,)
    }
};
        locals.var_sp_s_w__blk1451 = assign49480_e63704;
        locals.var_sp_s_w__blk1451_dn5 = assign49480_e63704_d_n5;
        locals.var_sp_s_w__blk1451_dn6 = assign49480_e63704_d_n6;
        locals.var_sp_s_w__blk1451_dn7 = assign49480_e63704_d_n7;
        locals.var_sp_s_w__blk1451_dn8 = assign49480_e63704_d_n8;

        let (assign49490_e63731, assign49490_e63731_d_n5, assign49490_e63731_d_n6, assign49490_e63731_d_n7, assign49490_e63731_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49490_e63717: f64 = (locals.var_gf2__blk1308 * 0.5);
        let assign49490_e63718: f64 = (locals.var_xg__blk1326 + assign49490_e63717);
        let assign49490_e63723: f64 = (locals.var_gf2__blk1308 * 0.25);
        let assign49490_e63724: f64 = (locals.var_xg__blk1326 + assign49490_e63723);
        let assign49490_e63726: f64 = (assign49490_e63724 - locals.var_sp_s_w__blk1451);
        let assign49490_e63727: f64 = (assign49490_e63726).sqrt();
        let assign49490_e63728: f64 = (locals.var_gf__blk1307 * assign49490_e63727);
        let assign49490_e63729: f64 = (assign49490_e63718 - assign49490_e63728);
        (assign49490_e63729, ((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.5)) - ((locals.var_gf__blk1307_dn5 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.25)) - locals.var_sp_s_w__blk1451_dn5) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.5)) - ((locals.var_gf__blk1307_dn6 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.25)) - locals.var_sp_s_w__blk1451_dn6) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.5)) - ((locals.var_gf__blk1307_dn7 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.25)) - locals.var_sp_s_w__blk1451_dn7) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.5)) - ((locals.var_gf__blk1307_dn8 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.25)) - locals.var_sp_s_w__blk1451_dn8) / (2.0 * assign49490_e63727))))),)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign49490_e63731;
        locals.var_sp_s_x1__blk1452_dn5 = assign49490_e63731_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign49490_e63731_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign49490_e63731_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign49490_e63731_d_n8;

        let (assign49500_e63745, assign49500_e63745_d_n5, assign49500_e63745_d_n6, assign49500_e63745_d_n7, assign49500_e63745_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49500_e63743: f64 = (locals.var_xn_s__blk1332 + 3.0);
        (assign49500_e63743, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    } else {
        (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8,)
    }
};
        locals.var_sp_s_bx__blk1453 = assign49500_e63745;
        locals.var_sp_s_bx__blk1453_dn5 = assign49500_e63745_d_n5;
        locals.var_sp_s_bx__blk1453_dn6 = assign49500_e63745_d_n6;
        locals.var_sp_s_bx__blk1453_dn7 = assign49500_e63745_d_n7;
        locals.var_sp_s_bx__blk1453_dn8 = assign49500_e63745_d_n8;

        let (assign49510_e63783, assign49510_e63783_d_n5, assign49510_e63783_d_n6, assign49510_e63783_d_n7, assign49510_e63783_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49510_e63758: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
        let assign49510_e63761: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign49510_e63764: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign49510_e63765: f64 = (assign49510_e63761 * assign49510_e63764);
        let assign49510_e63767: f64 = (assign49510_e63765 + 5.0);
        let assign49510_e63768: f64 = (assign49510_e63767).sqrt();
        let assign49510_e63769: f64 = (assign49510_e63758 - assign49510_e63768);
        let assign49510_e63770: f64 = (0.5 * assign49510_e63769);
        let assign49510_e63775: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
        let assign49510_e63777: f64 = (assign49510_e63775 + 5.0);
        let assign49510_e63778: f64 = (assign49510_e63777).sqrt();
        let assign49510_e63779: f64 = (locals.var_sp_s_bx__blk1453 - assign49510_e63778);
        let assign49510_e63780: f64 = (0.5 * assign49510_e63779);
        let assign49510_e63781: f64 = (assign49510_e63770 - assign49510_e63780);
        (assign49510_e63781, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign49510_e63778))))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign49510_e63783;
        locals.var_sp_s_eta__blk1436_dn5 = assign49510_e63783_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign49510_e63783_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign49510_e63783_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign49510_e63783_d_n8;

        let (assign49520_e63797, assign49520_e63797_d_n5, assign49520_e63797_d_n6, assign49520_e63797_d_n7, assign49520_e63797_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49520_e63795: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
        (assign49520_e63795, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49520_e63797;
        locals.var_sp_s_temp__blk1431_dn5 = assign49520_e63797_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49520_e63797_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49520_e63797_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49520_e63797_d_n8;

        let (assign49530_e63811, assign49530_e63811_d_n5, assign49530_e63811_d_n6, assign49530_e63811_d_n7, assign49530_e63811_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49530_e63808: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign49530_e63809: f64 = (assign49530_e63808).exp();
        (assign49530_e63809, (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign49530_e63811;
        locals.var_sp_s_temp1__blk1432_dn5 = assign49530_e63811_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign49530_e63811_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign49530_e63811_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign49530_e63811_d_n8;

        let (assign49540_e63829, assign49540_e63829_d_n5, assign49540_e63829_d_n6, assign49540_e63829_d_n7, assign49540_e63829_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49540_e63825: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign49540_e63826: f64 = (2.0 + assign49540_e63825);
        let assign49540_e63827: f64 = (1.0 / assign49540_e63826);
        (assign49540_e63827, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign49540_e63826 * assign49540_e63826))),)
    } else {
        (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8,)
    }
};
        locals.var_sp_s_temp2__blk1433 = assign49540_e63829;
        locals.var_sp_s_temp2__blk1433_dn5 = assign49540_e63829_d_n5;
        locals.var_sp_s_temp2__blk1433_dn6 = assign49540_e63829_d_n6;
        locals.var_sp_s_temp2__blk1433_dn7 = assign49540_e63829_d_n7;
        locals.var_sp_s_temp2__blk1433_dn8 = assign49540_e63829_d_n8;

        let (assign49550_e63845, assign49550_e63845_d_n5, assign49550_e63845_d_n6, assign49550_e63845_d_n7, assign49550_e63845_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49550_e63841: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign49550_e63843: f64 = (assign49550_e63841 * locals.var_sp_s_temp2__blk1433);
        (assign49550_e63843, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49550_e63845;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49550_e63845_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49550_e63845_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49550_e63845_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49550_e63845_d_n8;

        let (assign49560_e63863, assign49560_e63863_d_n5, assign49560_e63863_d_n6, assign49560_e63863_d_n7, assign49560_e63863_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49560_e63858: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
        let assign49560_e63860: f64 = (assign49560_e63858 * locals.var_sp_s_temp2__blk1433);
        let assign49560_e63861: f64 = (4.0 * assign49560_e63860);
        (assign49560_e63861, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49560_e63863;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49560_e63863_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49560_e63863_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49560_e63863_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49560_e63863_d_n8;

        let (assign49570_e63885, assign49570_e63885_d_n5, assign49570_e63885_d_n6, assign49570_e63885_d_n7, assign49570_e63885_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49570_e63875: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
        let assign49570_e63878: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49570_e63879: f64 = (assign49570_e63875 - assign49570_e63878);
        let assign49570_e63881: f64 = (assign49570_e63879 * locals.var_sp_s_temp2__blk1433);
        let assign49570_e63883: f64 = (assign49570_e63881 * locals.var_sp_s_temp2__blk1433);
        (assign49570_e63883, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49570_e63885;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49570_e63885_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49570_e63885_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49570_e63885_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49570_e63885_d_n8;

        let (assign49580_e63938, assign49580_e63938_d_n5, assign49580_e63938_d_n6, assign49580_e63938_d_n7, assign49580_e63938_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49580_e63898: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49580_e63902: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign49580_e63904: f64 = (assign49580_e63902 - 1.0);
        let assign49580_e63908: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign49580_e63910: f64 = (assign49580_e63908 + locals.var_sp_s_xi0__blk1443);
        let assign49580_e63911: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63910);
        let assign49580_e63912: f64 = (assign49580_e63904 - assign49580_e63911);
        let assign49580_e63913: f64 = (locals.var_gf2__blk1308 * assign49580_e63912);
        let assign49580_e63914: f64 = (assign49580_e63898 - assign49580_e63913);
        let (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8,) = {
            if (1e-40 > assign49580_e63914) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49580_e63919: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
                let assign49580_e63923: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
                let assign49580_e63925: f64 = (assign49580_e63923 - 1.0);
                let assign49580_e63929: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
                let assign49580_e63931: f64 = (assign49580_e63929 + locals.var_sp_s_xi0__blk1443);
                let assign49580_e63932: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63931);
                let assign49580_e63933: f64 = (assign49580_e63925 - assign49580_e63932);
                let assign49580_e63934: f64 = (locals.var_gf2__blk1308 * assign49580_e63933);
                let assign49580_e63935: f64 = (assign49580_e63919 - assign49580_e63934);
                (assign49580_e63935, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
            }
        };
        (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8,)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign49580_e63938;
        locals.var_sp_s_a__blk1437_dn5 = assign49580_e63938_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign49580_e63938_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign49580_e63938_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign49580_e63938_d_n8;

        let (assign49590_e63960, assign49590_e63960_d_n5, assign49590_e63960_d_n6, assign49590_e63960_d_n7, assign49590_e63960_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49590_e63954: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49590_e63955: f64 = (locals.var_sp_s_temp1__blk1432 - assign49590_e63954);
        let assign49590_e63956: f64 = (locals.var_gf2__blk1308 * assign49590_e63955);
        let assign49590_e63957: f64 = (0.5 * assign49590_e63956);
        let assign49590_e63958: f64 = (1.0 - assign49590_e63957);
        (assign49590_e63958, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8))))))),)
    } else {
        (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8,)
    }
};
        locals.var_sp_s_b__blk1454 = assign49590_e63960;
        locals.var_sp_s_b__blk1454_dn5 = assign49590_e63960_d_n5;
        locals.var_sp_s_b__blk1454_dn6 = assign49590_e63960_d_n6;
        locals.var_sp_s_b__blk1454_dn7 = assign49590_e63960_d_n7;
        locals.var_sp_s_b__blk1454_dn8 = assign49590_e63960_d_n8;

        let (assign49600_e63986, assign49600_e63986_d_n5, assign49600_e63986_d_n6, assign49600_e63986_d_n7, assign49600_e63986_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49600_e63972: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49600_e63976: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
        let assign49600_e63980: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign49600_e63981: f64 = (locals.var_delta_ns__blk1347 * assign49600_e63980);
        let assign49600_e63982: f64 = (assign49600_e63976 - assign49600_e63981);
        let assign49600_e63983: f64 = (locals.var_gf2__blk1308 * assign49600_e63982);
        let assign49600_e63984: f64 = (assign49600_e63972 + assign49600_e63983);
        (assign49600_e63984, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign49600_e63986;
        locals.var_sp_s_c__blk1438_dn5 = assign49600_e63986_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign49600_e63986_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign49600_e63986_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign49600_e63986_d_n8;

        let (assign49610_e64005, assign49610_e64005_d_n5, assign49610_e64005_d_n6, assign49610_e64005_d_n7, assign49610_e64005_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49610_e63998: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_eta__blk1436);
        let assign49610_e64001: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
        let assign49610_e64002: f64 = (assign49610_e64001).ln();
        let assign49610_e64003: f64 = (assign49610_e63998 + assign49610_e64002);
        (assign49610_e64003, ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign49610_e64005;
        locals.var_sp_s_tau__blk1439_dn5 = assign49610_e64005_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign49610_e64005_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign49610_e64005_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign49610_e64005_d_n8;

        let (assign49620_e64019, assign49620_e64019_d_n5, assign49620_e64019_d_n6, assign49620_e64019_d_n7, assign49620_e64019_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49620_e64017: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign49620_e64017, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign49620_e64019;
        locals.var_nu_dn5 = assign49620_e64019_d_n5;
        locals.var_nu_dn6 = assign49620_e64019_d_n6;
        locals.var_nu_dn7 = assign49620_e64019_d_n7;
        locals.var_nu_dn8 = assign49620_e64019_d_n8;

        let (assign49630_e64045, assign49630_e64045_d_n5, assign49630_e64045_d_n6, assign49630_e64045_d_n7, assign49630_e64045_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49630_e64031: f64 = (locals.var_nu * locals.var_nu);
        let assign49630_e64036: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49630_e64037: f64 = (0.5 * assign49630_e64036);
        let assign49630_e64040: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign49630_e64041: f64 = (assign49630_e64037 - assign49630_e64040);
        let assign49630_e64042: f64 = (locals.var_sp_s_tau__blk1439 * assign49630_e64041);
        let assign49630_e64043: f64 = (assign49630_e64031 + assign49630_e64042);
        (assign49630_e64043, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign49630_e64045;
        locals.var_mutau_dn5 = assign49630_e64045_d_n5;
        locals.var_mutau_dn6 = assign49630_e64045_d_n6;
        locals.var_mutau_dn7 = assign49630_e64045_d_n7;
        locals.var_mutau_dn8 = assign49630_e64045_d_n8;

        let (assign49640_e64085, assign49640_e64085_d_n5, assign49640_e64085_d_n6, assign49640_e64085_d_n7, assign49640_e64085_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49640_e64058: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign49640_e64060: f64 = (assign49640_e64058 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64064: f64 = (locals.var_nu / locals.var_mutau);
        let assign49640_e64066: f64 = (assign49640_e64064 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64068: f64 = (assign49640_e64066 * locals.var_sp_s_tau__blk1439);
        let assign49640_e64070: f64 = (assign49640_e64068 * locals.var_sp_s_c__blk1438);
        let assign49640_e64073: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign49640_e64075: f64 = (assign49640_e64073 * 0.3333333333333333);
        let assign49640_e64078: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign49640_e64079: f64 = (assign49640_e64075 - assign49640_e64078);
        let assign49640_e64080: f64 = (assign49640_e64070 * assign49640_e64079);
        let assign49640_e64081: f64 = (locals.var_mutau + assign49640_e64080);
        let assign49640_e64082: f64 = (assign49640_e64060 / assign49640_e64081);
        let assign49640_e64083: f64 = (locals.var_sp_s_eta__blk1436 + assign49640_e64082);
        (assign49640_e64083, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn5)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn5)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn6)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn6)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn7)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn7)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn8)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn8)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign49640_e64081 * assign49640_e64081))),)
    } else {
        (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8,)
    }
};
        locals.var_sp_s_x0__blk1455 = assign49640_e64085;
        locals.var_sp_s_x0__blk1455_dn5 = assign49640_e64085_d_n5;
        locals.var_sp_s_x0__blk1455_dn6 = assign49640_e64085_d_n6;
        locals.var_sp_s_x0__blk1455_dn7 = assign49640_e64085_d_n7;
        locals.var_sp_s_x0__blk1455_dn8 = assign49640_e64085_d_n8;

        let assign49650_e64088: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign49650_e64088;

        let (assign49660_e64103, assign49660_e64103_d_n5, assign49660_e64103_d_n6, assign49660_e64103_d_n7, assign49660_e64103_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49660_e64101: f64 = (locals.var_sp_s_x0__blk1455).exp();
        (assign49660_e64101, (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn5), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn6), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn7), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49660_e64103;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49660_e64103_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49660_e64103_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49660_e64103_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49660_e64103_d_n8;

        let (assign49670_e64119, assign49670_e64119_d_n5, assign49670_e64119_d_n6, assign49670_e64119_d_n7, assign49670_e64119_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49670_e64117: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign49670_e64117, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49670_e64119;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49670_e64119_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49670_e64119_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49670_e64119_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49670_e64119_d_n8;

        let (assign49680_e64135, assign49680_e64135_d_n5, assign49680_e64135_d_n6, assign49680_e64135_d_n7, assign49680_e64135_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign49680_e64133: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441);
        (assign49680_e64133, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49680_e64135;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49680_e64135_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49680_e64135_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49680_e64135_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49680_e64135_d_n8;

        let assign49690_e64139: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign49690_e64140: f64 = if locals.var_sp_s_x0__blk1455 > assign49690_e64139 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign49690_e64140;

        let (assign49700_e64160, assign49700_e64160_d_n5, assign49700_e64160_d_n6, assign49700_e64160_d_n7, assign49700_e64160_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign49700_e64157: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_s__blk1332);
        let assign49700_e64158: f64 = (assign49700_e64157).exp();
        (assign49700_e64158, (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_s__blk1332_dn5)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_s__blk1332_dn6)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_s__blk1332_dn7)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49700_e64160;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49700_e64160_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49700_e64160_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49700_e64160_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49700_e64160_d_n8;

        let (assign49710_e64179, assign49710_e64179_d_n5, assign49710_e64179_d_n6, assign49710_e64179_d_n7, assign49710_e64179_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign49710_e64177: f64 = (locals.var_delta_ns__blk1347 / locals.var_sp_s_delta0__blk1441);
        (assign49710_e64177, (((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49710_e64179;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49710_e64179_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49710_e64179_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49710_e64179_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49710_e64179_d_n8;

        let (assign49720_e64225, assign49720_e64225_d_n5, assign49720_e64225_d_n6, assign49720_e64225_d_n7, assign49720_e64225_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        let assign49720_e64199: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64201: f64 = (assign49720_e64199 - 230.25850929940458);
        let assign49720_e64206: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64208: f64 = (assign49720_e64206 - 230.25850929940458);
        let assign49720_e64212: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
        let assign49720_e64214: f64 = (assign49720_e64212 - 230.25850929940458);
        let assign49720_e64216: f64 = (assign49720_e64214 * 0.3333333333333333);
        let assign49720_e64217: f64 = (1.0 + assign49720_e64216);
        let assign49720_e64218: f64 = (assign49720_e64208 * assign49720_e64217);
        let assign49720_e64219: f64 = (0.5 * assign49720_e64218);
        let assign49720_e64220: f64 = (1.0 + assign49720_e64219);
        let assign49720_e64221: f64 = (assign49720_e64201 * assign49720_e64220);
        let assign49720_e64222: f64 = (1.0 + assign49720_e64221);
        let assign49720_e64223: f64 = (1e-100 / assign49720_e64222);
        (assign49720_e64223, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign49720_e64225;
        locals.var_sp_s_delta0__blk1441_dn5 = assign49720_e64225_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign49720_e64225_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign49720_e64225_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign49720_e64225_d_n8;

        let (assign49730_e64265, assign49730_e64265_d_n5, assign49730_e64265_d_n6, assign49730_e64265_d_n7, assign49730_e64265_d_n8,) = {
    if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        let assign49730_e64245: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64250: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64254: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign49730_e64256: f64 = (assign49730_e64254 * 0.3333333333333333);
        let assign49730_e64257: f64 = (1.0 + assign49730_e64256);
        let assign49730_e64258: f64 = (assign49730_e64250 * assign49730_e64257);
        let assign49730_e64259: f64 = (0.5 * assign49730_e64258);
        let assign49730_e64260: f64 = (1.0 + assign49730_e64259);
        let assign49730_e64261: f64 = (assign49730_e64245 * assign49730_e64260);
        let assign49730_e64262: f64 = (1.0 + assign49730_e64261);
        let assign49730_e64263: f64 = (1e-100 / assign49730_e64262);
        (assign49730_e64263, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign49730_e64265;
        locals.var_sp_s_delta1__blk1442_dn5 = assign49730_e64265_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign49730_e64265_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign49730_e64265_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign49730_e64265_d_n8;

        let (assign49740_e64283, assign49740_e64283_d_n5, assign49740_e64283_d_n6, assign49740_e64283_d_n7, assign49740_e64283_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49740_e64279: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign49740_e64280: f64 = (2.0 + assign49740_e64279);
        let assign49740_e64281: f64 = (1.0 / assign49740_e64280);
        (assign49740_e64281, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign49740_e64280 * assign49740_e64280))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49740_e64283;
        locals.var_sp_s_temp__blk1431_dn5 = assign49740_e64283_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49740_e64283_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49740_e64283_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49740_e64283_d_n8;

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign49750_e64299, assign49750_e64299_d_n5, assign49750_e64299_d_n6, assign49750_e64299_d_n7, assign49750_e64299_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49750_e64295: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign49750_e64297: f64 = (assign49750_e64295 * locals.var_sp_s_temp__blk1431);
        (assign49750_e64297, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign49750_e64299;
        locals.var_sp_s_xi0__blk1443_dn5 = assign49750_e64299_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign49750_e64299_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign49750_e64299_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign49750_e64299_d_n8;

        let (assign49760_e64317, assign49760_e64317_d_n5, assign49760_e64317_d_n6, assign49760_e64317_d_n7, assign49760_e64317_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49760_e64312: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
        let assign49760_e64314: f64 = (assign49760_e64312 * locals.var_sp_s_temp__blk1431);
        let assign49760_e64315: f64 = (4.0 * assign49760_e64314);
        (assign49760_e64315, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign49760_e64317;
        locals.var_sp_s_xi1__blk1444_dn5 = assign49760_e64317_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign49760_e64317_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign49760_e64317_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign49760_e64317_d_n8;

        let (assign49770_e64339, assign49770_e64339_d_n5, assign49770_e64339_d_n6, assign49770_e64339_d_n7, assign49770_e64339_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49770_e64329: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign49770_e64332: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign49770_e64333: f64 = (assign49770_e64329 - assign49770_e64332);
        let assign49770_e64335: f64 = (assign49770_e64333 * locals.var_sp_s_temp__blk1431);
        let assign49770_e64337: f64 = (assign49770_e64335 * locals.var_sp_s_temp__blk1431);
        (assign49770_e64337, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign49770_e64339;
        locals.var_sp_s_xi2__blk1445_dn5 = assign49770_e64339_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign49770_e64339_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign49770_e64339_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign49770_e64339_d_n8;

        let (assign49780_e64353, assign49780_e64353_d_n5, assign49780_e64353_d_n6, assign49780_e64353_d_n7, assign49780_e64353_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49780_e64351: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
        (assign49780_e64351, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49780_e64353;
        locals.var_sp_s_temp__blk1431_dn5 = assign49780_e64353_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49780_e64353_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49780_e64353_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49780_e64353_d_n8;

        let (assign49790_e64381, assign49790_e64381_d_n5, assign49790_e64381_d_n6, assign49790_e64381_d_n7, assign49790_e64381_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49790_e64365: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign49790_e64369: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
        let assign49790_e64371: f64 = (assign49790_e64369 + locals.var_sp_s_delta0__blk1441);
        let assign49790_e64375: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign49790_e64376: f64 = (locals.var_delta_ns__blk1347 * assign49790_e64375);
        let assign49790_e64377: f64 = (assign49790_e64371 - assign49790_e64376);
        let assign49790_e64378: f64 = (locals.var_gf2__blk1308 * assign49790_e64377);
        let assign49790_e64379: f64 = (assign49790_e64365 + assign49790_e64378);
        (assign49790_e64379, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign49790_e64381;
        locals.var_sp_s_pc__blk1446_dn5 = assign49790_e64381_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign49790_e64381_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign49790_e64381_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign49790_e64381_d_n8;

        let (assign49800_e64413, assign49800_e64413_d_n5, assign49800_e64413_d_n6, assign49800_e64413_d_n7, assign49800_e64413_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49800_e64393: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49800_e64397: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
        let assign49800_e64399: f64 = (assign49800_e64397 - 1.0);
        let assign49800_e64401: f64 = (assign49800_e64399 + locals.var_sp_s_delta0__blk1441);
        let assign49800_e64405: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
        let assign49800_e64407: f64 = (assign49800_e64405 + locals.var_sp_s_xi0__blk1443);
        let assign49800_e64408: f64 = (locals.var_delta_ns__blk1347 * assign49800_e64407);
        let assign49800_e64409: f64 = (assign49800_e64401 - assign49800_e64408);
        let assign49800_e64410: f64 = (locals.var_gf2__blk1308 * assign49800_e64409);
        let assign49800_e64411: f64 = (assign49800_e64393 - assign49800_e64410);
        (assign49800_e64411, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign49800_e64413;
        locals.var_sp_s_qc__blk1447_dn5 = assign49800_e64413_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign49800_e64413_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign49800_e64413_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign49800_e64413_d_n8;

        let (assign49810_e64435, assign49810_e64435_d_n5, assign49810_e64435_d_n6, assign49810_e64435_d_n7, assign49810_e64435_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49810_e64427: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
        let assign49810_e64430: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
        let assign49810_e64431: f64 = (assign49810_e64427 - assign49810_e64430);
        let assign49810_e64432: f64 = (locals.var_gf2__blk1308 * assign49810_e64431);
        let assign49810_e64433: f64 = (2.0 - assign49810_e64432);
        (assign49810_e64433, (-((locals.var_gf2__blk1308_dn5 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49810_e64435;
        locals.var_sp_s_temp__blk1431_dn5 = assign49810_e64435_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49810_e64435_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49810_e64435_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49810_e64435_d_n8;

        let (assign49820_e64455, assign49820_e64455_d_n5, assign49820_e64455_d_n6, assign49820_e64455_d_n7, assign49820_e64455_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49820_e64447: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign49820_e64451: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign49820_e64452: f64 = (2.0 * assign49820_e64451);
        let assign49820_e64453: f64 = (assign49820_e64447 - assign49820_e64452);
        (assign49820_e64453, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign49820_e64455;
        locals.var_sp_s_temp__blk1431_dn5 = assign49820_e64455_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign49820_e64455_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign49820_e64455_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign49820_e64455_d_n8;

        let (assign49830_e64476, assign49830_e64476_d_n5, assign49830_e64476_d_n6, assign49830_e64476_d_n7, assign49830_e64476_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign49830_e64470: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign49830_e64471: f64 = (locals.var_sp_s_pc__blk1446 + assign49830_e64470);
        let assign49830_e64472: f64 = (locals.var_sp_s_qc__blk1447 / assign49830_e64471);
        let assign49830_e64473: f64 = (2.0 * assign49830_e64472);
        let assign49830_e64474: f64 = (locals.var_sp_s_x0__blk1455 + assign49830_e64473);
        (assign49830_e64474, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))),)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign49830_e64476;
        locals.var_x_s__blk1346_dn5 = assign49830_e64476_d_n5;
        locals.var_x_s__blk1346_dn6 = assign49830_e64476_d_n6;
        locals.var_x_s__blk1346_dn7 = assign49830_e64476_d_n7;
        locals.var_x_s__blk1346_dn8 = assign49830_e64476_d_n8;

        let (assign49840_e64482, assign49840_e64482_d_n5, assign49840_e64482_d_n6, assign49840_e64482_d_n7, assign49840_e64482_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign49840_e64482;
        locals.var_xi1s__blk1349_dn5 = assign49840_e64482_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign49840_e64482_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign49840_e64482_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign49840_e64482_d_n8;

        let (assign49850_e64488, assign49850_e64488_d_n5, assign49850_e64488_d_n6, assign49850_e64488_d_n7, assign49850_e64488_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign49850_e64488;
        locals.var_xi2s__blk1350_dn5 = assign49850_e64488_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign49850_e64488_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign49850_e64488_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign49850_e64488_d_n8;

        let (assign49860_e64494, assign49860_e64494_d_n5, assign49860_e64494_d_n6, assign49860_e64494_d_n7, assign49860_e64494_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign49860_e64494;
        locals.var_delta_1s__blk1351_dn5 = assign49860_e64494_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign49860_e64494_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign49860_e64494_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign49860_e64494_d_n8;

        let (assign49870_e64500, assign49870_e64500_d_n5, assign49870_e64500_d_n6, assign49870_e64500_d_n7, assign49870_e64500_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign49870_e64500;
        locals.var_es__blk1352_dn5 = assign49870_e64500_d_n5;
        locals.var_es__blk1352_dn6 = assign49870_e64500_d_n6;
        locals.var_es__blk1352_dn7 = assign49870_e64500_d_n7;
        locals.var_es__blk1352_dn8 = assign49870_e64500_d_n8;

        let (assign49880_e64506, assign49880_e64506_d_n5, assign49880_e64506_d_n6, assign49880_e64506_d_n7, assign49880_e64506_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign49880_e64506;
        locals.var_ds__blk1353_dn5 = assign49880_e64506_d_n5;
        locals.var_ds__blk1353_dn6 = assign49880_e64506_d_n6;
        locals.var_ds__blk1353_dn7 = assign49880_e64506_d_n7;
        locals.var_ds__blk1353_dn8 = assign49880_e64506_d_n8;

        let (assign49890_e64512, assign49890_e64512_d_n5, assign49890_e64512_d_n6, assign49890_e64512_d_n7, assign49890_e64512_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign49890_e64512;
        locals.var_ps__blk1354_dn5 = assign49890_e64512_d_n5;
        locals.var_ps__blk1354_dn6 = assign49890_e64512_d_n6;
        locals.var_ps__blk1354_dn7 = assign49890_e64512_d_n7;
        locals.var_ps__blk1354_dn8 = assign49890_e64512_d_n8;

        let (assign49900_e64518, assign49900_e64518_d_n5, assign49900_e64518_d_n6, assign49900_e64518_d_n7, assign49900_e64518_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign49900_e64518;
        locals.var_sqs__blk1355_dn5 = assign49900_e64518_d_n5;
        locals.var_sqs__blk1355_dn6 = assign49900_e64518_d_n6;
        locals.var_sqs__blk1355_dn7 = assign49900_e64518_d_n7;
        locals.var_sqs__blk1355_dn8 = assign49900_e64518_d_n8;

        let (assign49910_e64524, assign49910_e64524_d_n5, assign49910_e64524_d_n6, assign49910_e64524_d_n7, assign49910_e64524_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign49910_e64524;
        locals.var_alphas__blk1356_dn5 = assign49910_e64524_d_n5;
        locals.var_alphas__blk1356_dn6 = assign49910_e64524_d_n6;
        locals.var_alphas__blk1356_dn7 = assign49910_e64524_d_n7;
        locals.var_alphas__blk1356_dn8 = assign49910_e64524_d_n8;

        let (assign49920_e64530, assign49920_e64530_d_n5, assign49920_e64530_d_n6, assign49920_e64530_d_n7, assign49920_e64530_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign49920_e64530;
        locals.var_rxcor__blk1357_dn5 = assign49920_e64530_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign49920_e64530_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign49920_e64530_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign49920_e64530_d_n8;

        let (assign49930_e64538, assign49930_e64538_d_n5, assign49930_e64538_d_n6, assign49930_e64538_d_n7, assign49930_e64538_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49930_e64536: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        (assign49930_e64536, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign49930_e64538;
        locals.var_xgs__blk1358_dn5 = assign49930_e64538_d_n5;
        locals.var_xgs__blk1358_dn6 = assign49930_e64538_d_n6;
        locals.var_xgs__blk1358_dn7 = assign49930_e64538_d_n7;
        locals.var_xgs__blk1358_dn8 = assign49930_e64538_d_n8;

        let (assign49940_e64544, assign49940_e64544_d_n5, assign49940_e64544_d_n6, assign49940_e64544_d_n7, assign49940_e64544_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign49940_e64544;
        locals.var_qis__blk1359_dn5 = assign49940_e64544_d_n5;
        locals.var_qis__blk1359_dn6 = assign49940_e64544_d_n6;
        locals.var_qis__blk1359_dn7 = assign49940_e64544_d_n7;
        locals.var_qis__blk1359_dn8 = assign49940_e64544_d_n8;

        let (assign49950_e64552, assign49950_e64552_d_n5, assign49950_e64552_d_n6, assign49950_e64552_d_n7, assign49950_e64552_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        let assign49950_e64550: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
        (assign49950_e64550, ((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)), ((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)), ((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)), ((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)),)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign49950_e64552;
        locals.var_qbs__blk1360_dn5 = assign49950_e64552_d_n5;
        locals.var_qbs__blk1360_dn6 = assign49950_e64552_d_n6;
        locals.var_qbs__blk1360_dn7 = assign49950_e64552_d_n7;
        locals.var_qbs__blk1360_dn8 = assign49950_e64552_d_n8;

        let (assign49960_e64558, assign49960_e64558_d_n5, assign49960_e64558_d_n6, assign49960_e64558_d_n7, assign49960_e64558_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign49960_e64558;
        locals.var_rhob__blk1361_dn5 = assign49960_e64558_d_n5;
        locals.var_rhob__blk1361_dn6 = assign49960_e64558_d_n6;
        locals.var_rhob__blk1361_dn7 = assign49960_e64558_d_n7;
        locals.var_rhob__blk1361_dn8 = assign49960_e64558_d_n8;

        let (assign49970_e64564, assign49970_e64564_d_n5, assign49970_e64564_d_n6, assign49970_e64564_d_n7, assign49970_e64564_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign49970_e64564;
        locals.var_rhog__blk1362_dn5 = assign49970_e64564_d_n5;
        locals.var_rhog__blk1362_dn6 = assign49970_e64564_d_n6;
        locals.var_rhog__blk1362_dn7 = assign49970_e64564_d_n7;
        locals.var_rhog__blk1362_dn8 = assign49970_e64564_d_n8;

        let (assign49980_e64570, assign49980_e64570_d_n5, assign49980_e64570_d_n6, assign49980_e64570_d_n7, assign49980_e64570_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign49980_e64570;
        locals.var_gmobs__blk1366_dn5 = assign49980_e64570_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign49980_e64570_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign49980_e64570_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign49980_e64570_d_n8;

        let (assign49990_e64576, assign49990_e64576_d_n5, assign49990_e64576_d_n6, assign49990_e64576_d_n7, assign49990_e64576_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign49990_e64576;
        locals.var_xitsb__blk1367_dn5 = assign49990_e64576_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign49990_e64576_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign49990_e64576_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign49990_e64576_d_n8;

        let (assign50000_e64582, assign50000_e64582_d_n5, assign50000_e64582_d_n6, assign50000_e64582_d_n7, assign50000_e64582_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50000_e64582;
        locals.var_factheta__blk1369_dn5 = assign50000_e64582_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50000_e64582_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50000_e64582_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50000_e64582_d_n8;

        let assign50010_e64585: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign50010_e64585;

        let (assign50020_e64599, assign50020_e64599_d_n5, assign50020_e64599_d_n6, assign50020_e64599_d_n7, assign50020_e64599_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50020_e64595: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50020_e64596: f64 = (2.0 + assign50020_e64595);
        let assign50020_e64597: f64 = (1.0 / assign50020_e64596);
        (assign50020_e64597, (-(((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) / (assign50020_e64596 * assign50020_e64596))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign50020_e64599;
        locals.var_temp__blk936_dn5 = assign50020_e64599_d_n5;
        locals.var_temp__blk936_dn6 = assign50020_e64599_d_n6;
        locals.var_temp__blk936_dn7 = assign50020_e64599_d_n7;
        locals.var_temp__blk936_dn8 = assign50020_e64599_d_n8;

        let (assign50030_e64611, assign50030_e64611_d_n5, assign50030_e64611_d_n6, assign50030_e64611_d_n7, assign50030_e64611_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50030_e64607: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50030_e64609: f64 = (assign50030_e64607 * locals.var_temp__blk936);
        (assign50030_e64609, ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn5)), ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn6)), ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn7)), ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi0s__blk1348, locals.var_xi0s__blk1348_dn5, locals.var_xi0s__blk1348_dn6, locals.var_xi0s__blk1348_dn7, locals.var_xi0s__blk1348_dn8,)
    }
};
        locals.var_xi0s__blk1348 = assign50030_e64611;
        locals.var_xi0s__blk1348_dn5 = assign50030_e64611_d_n5;
        locals.var_xi0s__blk1348_dn6 = assign50030_e64611_d_n6;
        locals.var_xi0s__blk1348_dn7 = assign50030_e64611_d_n7;
        locals.var_xi0s__blk1348_dn8 = assign50030_e64611_d_n8;

        let (assign50040_e64625, assign50040_e64625_d_n5, assign50040_e64625_d_n6, assign50040_e64625_d_n7, assign50040_e64625_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50040_e64620: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
        let assign50040_e64622: f64 = (assign50040_e64620 * locals.var_temp__blk936);
        let assign50040_e64623: f64 = (4.0 * assign50040_e64622);
        (assign50040_e64623, (4.0 * ((((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn5))), (4.0 * ((((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn6))), (4.0 * ((((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn7))), (4.0 * ((((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign50040_e64625;
        locals.var_xi1s__blk1349_dn5 = assign50040_e64625_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign50040_e64625_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign50040_e64625_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign50040_e64625_d_n8;

        let (assign50050_e64643, assign50050_e64643_d_n5, assign50050_e64643_d_n6, assign50050_e64643_d_n7, assign50050_e64643_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50050_e64633: f64 = (8.0 * locals.var_temp__blk936);
        let assign50050_e64636: f64 = (12.0 * locals.var_xi0s__blk1348);
        let assign50050_e64637: f64 = (assign50050_e64633 - assign50050_e64636);
        let assign50050_e64639: f64 = (assign50050_e64637 * locals.var_temp__blk936);
        let assign50050_e64641: f64 = (assign50050_e64639 * locals.var_temp__blk936);
        (assign50050_e64641, ((((((8.0 * locals.var_temp__blk936_dn5) - (12.0 * locals.var_xi0s__blk1348_dn5)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn5)), ((((((8.0 * locals.var_temp__blk936_dn6) - (12.0 * locals.var_xi0s__blk1348_dn6)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn6)), ((((((8.0 * locals.var_temp__blk936_dn7) - (12.0 * locals.var_xi0s__blk1348_dn7)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn7)), ((((((8.0 * locals.var_temp__blk936_dn8) - (12.0 * locals.var_xi0s__blk1348_dn8)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn8)),)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign50050_e64643;
        locals.var_xi2s__blk1350_dn5 = assign50050_e64643_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign50050_e64643_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign50050_e64643_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign50050_e64643_d_n8;

        let (assign50060_e64651, assign50060_e64651_d_n5, assign50060_e64651_d_n6, assign50060_e64651_d_n7, assign50060_e64651_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50060_e64651;
        locals.var_delta_1s__blk1351_dn5 = assign50060_e64651_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50060_e64651_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50060_e64651_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50060_e64651_d_n8;

        let assign50070_e64654: f64 = if locals.var_x_s__blk1346 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign50070_e64654;

        let (assign50080_e64665, assign50080_e64665_d_n5, assign50080_e64665_d_n6, assign50080_e64665_d_n7, assign50080_e64665_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50080_e64663: f64 = (locals.var_x_s__blk1346).exp();
        (assign50080_e64663, (assign50080_e64663 * locals.var_x_s__blk1346_dn5), (assign50080_e64663 * locals.var_x_s__blk1346_dn6), (assign50080_e64663 * locals.var_x_s__blk1346_dn7), (assign50080_e64663 * locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50080_e64665;
        locals.var_delta_1s__blk1351_dn5 = assign50080_e64665_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50080_e64665_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50080_e64665_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50080_e64665_d_n8;

        let (assign50090_e64677, assign50090_e64677_d_n5, assign50090_e64677_d_n6, assign50090_e64677_d_n7, assign50090_e64677_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50090_e64675: f64 = (1.0 / locals.var_delta_1s__blk1351);
        (assign50090_e64675, (-(locals.var_delta_1s__blk1351_dn5 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn6 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn7 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn8 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50090_e64677;
        locals.var_es__blk1352_dn5 = assign50090_e64677_d_n5;
        locals.var_es__blk1352_dn6 = assign50090_e64677_d_n6;
        locals.var_es__blk1352_dn7 = assign50090_e64677_d_n7;
        locals.var_es__blk1352_dn8 = assign50090_e64677_d_n8;

        let (assign50100_e64689, assign50100_e64689_d_n5, assign50100_e64689_d_n6, assign50100_e64689_d_n7, assign50100_e64689_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign50100_e64687: f64 = (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351);
        (assign50100_e64687, ((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50100_e64689;
        locals.var_delta_1s__blk1351_dn5 = assign50100_e64689_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50100_e64689_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50100_e64689_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50100_e64689_d_n8;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let assign50110_e64693: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign50110_e64694: f64 = if locals.var_x_s__blk1346 > assign50110_e64693 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign50110_e64694;

        let (assign50120_e64710, assign50120_e64710_d_n5, assign50120_e64710_d_n6, assign50120_e64710_d_n7, assign50120_e64710_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign50120_e64707: f64 = (locals.var_x_s__blk1346 - locals.var_xn_s__blk1332);
        let assign50120_e64708: f64 = (assign50120_e64707).exp();
        (assign50120_e64708, (assign50120_e64708 * (locals.var_x_s__blk1346_dn5 - locals.var_xn_s__blk1332_dn5)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn6 - locals.var_xn_s__blk1332_dn6)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn7 - locals.var_xn_s__blk1332_dn7)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn8 - locals.var_xn_s__blk1332_dn8)),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50120_e64710;
        locals.var_delta_1s__blk1351_dn5 = assign50120_e64710_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50120_e64710_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50120_e64710_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50120_e64710_d_n8;

        let (assign50130_e64725, assign50130_e64725_d_n5, assign50130_e64725_d_n6, assign50130_e64725_d_n7, assign50130_e64725_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign50130_e64723: f64 = (locals.var_delta_ns__blk1347 / locals.var_delta_1s__blk1351);
        (assign50130_e64723, (((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50130_e64725;
        locals.var_es__blk1352_dn5 = assign50130_e64725_d_n5;
        locals.var_es__blk1352_dn6 = assign50130_e64725_d_n6;
        locals.var_es__blk1352_dn7 = assign50130_e64725_d_n7;
        locals.var_es__blk1352_dn8 = assign50130_e64725_d_n8;

        let (assign50140_e64767, assign50140_e64767_d_n5, assign50140_e64767_d_n6, assign50140_e64767_d_n7, assign50140_e64767_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
        let assign50140_e64741: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64743: f64 = (assign50140_e64741 - 230.25850929940458);
        let assign50140_e64748: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64750: f64 = (assign50140_e64748 - 230.25850929940458);
        let assign50140_e64754: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
        let assign50140_e64756: f64 = (assign50140_e64754 - 230.25850929940458);
        let assign50140_e64758: f64 = (assign50140_e64756 * 0.3333333333333333);
        let assign50140_e64759: f64 = (1.0 + assign50140_e64758);
        let assign50140_e64760: f64 = (assign50140_e64750 * assign50140_e64759);
        let assign50140_e64761: f64 = (0.5 * assign50140_e64760);
        let assign50140_e64762: f64 = (1.0 + assign50140_e64761);
        let assign50140_e64763: f64 = (assign50140_e64743 * assign50140_e64762);
        let assign50140_e64764: f64 = (1.0 + assign50140_e64763);
        let assign50140_e64765: f64 = (1e-100 / assign50140_e64764);
        (assign50140_e64765, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))),)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50140_e64767;
        locals.var_delta_1s__blk1351_dn5 = assign50140_e64767_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50140_e64767_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50140_e64767_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50140_e64767_d_n8;

        let (assign50150_e64803, assign50150_e64803_d_n5, assign50150_e64803_d_n6, assign50150_e64803_d_n7, assign50150_e64803_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
        let assign50150_e64783: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64788: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64792: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
        let assign50150_e64794: f64 = (assign50150_e64792 * 0.3333333333333333);
        let assign50150_e64795: f64 = (1.0 + assign50150_e64794);
        let assign50150_e64796: f64 = (assign50150_e64788 * assign50150_e64795);
        let assign50150_e64797: f64 = (0.5 * assign50150_e64796);
        let assign50150_e64798: f64 = (1.0 + assign50150_e64797);
        let assign50150_e64799: f64 = (assign50150_e64783 * assign50150_e64798);
        let assign50150_e64800: f64 = (1.0 + assign50150_e64799);
        let assign50150_e64801: f64 = (1e-100 / assign50150_e64800);
        (assign50150_e64801, (-((1e-100 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn5 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn6 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn7 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn8 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))),)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50150_e64803;
        locals.var_es__blk1352_dn5 = assign50150_e64803_d_n5;
        locals.var_es__blk1352_dn6 = assign50150_e64803_d_n6;
        locals.var_es__blk1352_dn7 = assign50150_e64803_d_n7;
        locals.var_es__blk1352_dn8 = assign50150_e64803_d_n8;

        let (assign50160_e64819, assign50160_e64819_d_n5, assign50160_e64819_d_n6, assign50160_e64819_d_n7, assign50160_e64819_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50160_e64813: f64 = (locals.var_x_s__blk1346 + 1.0);
        let assign50160_e64815: f64 = (assign50160_e64813 + locals.var_xi0s__blk1348);
        let assign50160_e64816: f64 = (locals.var_delta_ns__blk1347 * assign50160_e64815);
        let assign50160_e64817: f64 = (locals.var_delta_1s__blk1351 - assign50160_e64816);
        (assign50160_e64817, (locals.var_delta_1s__blk1351_dn5 - ((locals.var_delta_ns__blk1347_dn5 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn5 + locals.var_xi0s__blk1348_dn5)))), (locals.var_delta_1s__blk1351_dn6 - ((locals.var_delta_ns__blk1347_dn6 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn6 + locals.var_xi0s__blk1348_dn6)))), (locals.var_delta_1s__blk1351_dn7 - ((locals.var_delta_ns__blk1347_dn7 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn7 + locals.var_xi0s__blk1348_dn7)))), (locals.var_delta_1s__blk1351_dn8 - ((locals.var_delta_ns__blk1347_dn8 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn8 + locals.var_xi0s__blk1348_dn8)))),)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50160_e64819;
        locals.var_ds__blk1353_dn5 = assign50160_e64819_d_n5;
        locals.var_ds__blk1353_dn6 = assign50160_e64819_d_n6;
        locals.var_ds__blk1353_dn7 = assign50160_e64819_d_n7;
        locals.var_ds__blk1353_dn8 = assign50160_e64819_d_n8;

        let assign50170_e64822: f64 = if locals.var_x_s__blk1346 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign50170_e64822;

        let (assign50180_e64848, assign50180_e64848_d_n5, assign50180_e64848_d_n6, assign50180_e64848_d_n7, assign50180_e64848_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50180_e64833: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50180_e64840: f64 = (0.25 * locals.var_x_s__blk1346);
        let assign50180_e64841: f64 = (1.0 - assign50180_e64840);
        let assign50180_e64842: f64 = (locals.var_x_s__blk1346 * assign50180_e64841);
        let assign50180_e64843: f64 = (0.3333333333333333 * assign50180_e64842);
        let assign50180_e64844: f64 = (1.0 - assign50180_e64843);
        let assign50180_e64845: f64 = (assign50180_e64833 * assign50180_e64844);
        let assign50180_e64846: f64 = (0.5 * assign50180_e64845);
        (assign50180_e64846, (0.5 * ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8))))))))),)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50180_e64848;
        locals.var_ps__blk1354_dn5 = assign50180_e64848_d_n5;
        locals.var_ps__blk1354_dn6 = assign50180_e64848_d_n6;
        locals.var_ps__blk1354_dn7 = assign50180_e64848_d_n7;
        locals.var_ps__blk1354_dn8 = assign50180_e64848_d_n8;

        let (assign50190_e64872, assign50190_e64872_d_n5, assign50190_e64872_d_n6, assign50190_e64872_d_n7, assign50190_e64872_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50190_e64859: f64 = (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346);
        let assign50190_e64861: f64 = (assign50190_e64859 * locals.var_x_s__blk1346);
        let assign50190_e64863: f64 = (assign50190_e64861 * locals.var_x_s__blk1346);
        let assign50190_e64867: f64 = (1.75 * locals.var_x_s__blk1346);
        let assign50190_e64868: f64 = (1.0 + assign50190_e64867);
        let assign50190_e64869: f64 = (assign50190_e64863 * assign50190_e64868);
        let assign50190_e64870: f64 = (0.16666666666666666 * assign50190_e64869);
        (assign50190_e64870, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn5 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn5)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn5)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn6 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn6)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn7 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn7)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn8 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn8)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn8)))),)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50190_e64872;
        locals.var_ds__blk1353_dn5 = assign50190_e64872_d_n5;
        locals.var_ds__blk1353_dn6 = assign50190_e64872_d_n6;
        locals.var_ds__blk1353_dn7 = assign50190_e64872_d_n7;
        locals.var_ds__blk1353_dn8 = assign50190_e64872_d_n8;

        let (assign50200_e64893, assign50200_e64893_d_n5, assign50200_e64893_d_n6, assign50200_e64893_d_n7, assign50200_e64893_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50200_e64886: f64 = (0.25 * locals.var_x_s__blk1346);
        let assign50200_e64887: f64 = (1.0 - assign50200_e64886);
        let assign50200_e64888: f64 = (locals.var_x_s__blk1346 * assign50200_e64887);
        let assign50200_e64889: f64 = (0.3333333333333333 * assign50200_e64888);
        let assign50200_e64890: f64 = (1.0 - assign50200_e64889);
        let assign50200_e64891: f64 = (assign50200_e64890).sqrt();
        (assign50200_e64891, ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8)))))) / (2.0 * assign50200_e64891)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign50200_e64893;
        locals.var_temp__blk936_dn5 = assign50200_e64893_d_n5;
        locals.var_temp__blk936_dn6 = assign50200_e64893_d_n6;
        locals.var_temp__blk936_dn7 = assign50200_e64893_d_n7;
        locals.var_temp__blk936_dn8 = assign50200_e64893_d_n8;

        let (assign50210_e64907, assign50210_e64907_d_n5, assign50210_e64907_d_n6, assign50210_e64907_d_n7, assign50210_e64907_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50210_e64904: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
        let assign50210_e64905: f64 = (0.7071067811865475 * assign50210_e64904);
        (assign50210_e64905, (0.7071067811865475 * ((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50210_e64907;
        locals.var_sqs__blk1355_dn5 = assign50210_e64907_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50210_e64907_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50210_e64907_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50210_e64907_d_n8;

        let (assign50220_e64935, assign50220_e64935_d_n5, assign50220_e64935_d_n6, assign50220_e64935_d_n7, assign50220_e64935_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign50220_e64921: f64 = (0.5 * locals.var_x_s__blk1346);
        let assign50220_e64922: f64 = (1.0 - assign50220_e64921);
        let assign50220_e64926: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
        let assign50220_e64927: f64 = (0.16666666666666666 * assign50220_e64926);
        let assign50220_e64928: f64 = (assign50220_e64922 + assign50220_e64927);
        let assign50220_e64929: f64 = (locals.var_gf__blk1307 * assign50220_e64928);
        let assign50220_e64931: f64 = (assign50220_e64929 / locals.var_temp__blk936);
        let assign50220_e64932: f64 = (0.7071067811865475 * assign50220_e64931);
        let assign50220_e64933: f64 = (1.0 + assign50220_e64932);
        (assign50220_e64933, (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn5)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936))),)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50220_e64935;
        locals.var_alphas__blk1356_dn5 = assign50220_e64935_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50220_e64935_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50220_e64935_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50220_e64935_d_n8;

        let (assign50230_e64950, assign50230_e64950_d_n5, assign50230_e64950_d_n6, assign50230_e64950_d_n7, assign50230_e64950_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50230_e64946: f64 = (locals.var_x_s__blk1346 - 1.0);
        let assign50230_e64948: f64 = (assign50230_e64946 + locals.var_es__blk1352);
        (assign50230_e64948, (locals.var_x_s__blk1346_dn5 + locals.var_es__blk1352_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_es__blk1352_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_es__blk1352_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_es__blk1352_dn8),)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50230_e64950;
        locals.var_ps__blk1354_dn5 = assign50230_e64950_d_n5;
        locals.var_ps__blk1354_dn6 = assign50230_e64950_d_n6;
        locals.var_ps__blk1354_dn7 = assign50230_e64950_d_n7;
        locals.var_ps__blk1354_dn8 = assign50230_e64950_d_n8;

        let (assign50240_e64962, assign50240_e64962_d_n5, assign50240_e64962_d_n6, assign50240_e64962_d_n7, assign50240_e64962_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50240_e64960: f64 = (locals.var_ps__blk1354).sqrt();
        (assign50240_e64960, (locals.var_ps__blk1354_dn5 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn6 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn7 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn8 / (2.0 * assign50240_e64960)),)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50240_e64962;
        locals.var_sqs__blk1355_dn5 = assign50240_e64962_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50240_e64962_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50240_e64962_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50240_e64962_d_n8;

        let (assign50250_e64983, assign50250_e64983_d_n5, assign50250_e64983_d_n6, assign50250_e64983_d_n7, assign50250_e64983_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign50250_e64976: f64 = (1.0 - locals.var_es__blk1352);
        let assign50250_e64977: f64 = (locals.var_gf__blk1307 * assign50250_e64976);
        let assign50250_e64979: f64 = (assign50250_e64977 / locals.var_sqs__blk1355);
        let assign50250_e64980: f64 = (0.5 * assign50250_e64979);
        let assign50250_e64981: f64 = (1.0 + assign50250_e64980);
        (assign50250_e64981, (0.5 * (((((locals.var_gf__blk1307_dn5 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn5))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn5)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn6 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn6))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn6)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn7 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn7))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn7)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn8 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn8))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn8)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))),)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50250_e64983;
        locals.var_alphas__blk1356_dn5 = assign50250_e64983_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50250_e64983_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50250_e64983_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50250_e64983_d_n8;

        let (assign50260_e65003, assign50260_e65003_d_n5, assign50260_e65003_d_n6, assign50260_e65003_d_n7, assign50260_e65003_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign50260_e64992: f64 = (0.2 * locals.var_xcor_t);
        let assign50260_e64994: f64 = (assign50260_e64992 * locals.var_vsbx__blk1306);
        let assign50260_e64995: f64 = (1.0 + assign50260_e64994);
        let assign50260_e64999: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1306);
        let assign50260_e65000: f64 = (1.0 + assign50260_e64999);
        let assign50260_e65001: f64 = (assign50260_e64995 / assign50260_e65000);
        (assign50260_e65001, ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn5) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn5))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn6) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn6))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn7) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn7))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn8) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn8))) / (assign50260_e65000 * assign50260_e65000)),)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign50260_e65003;
        locals.var_rxcor__blk1357_dn5 = assign50260_e65003_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign50260_e65003_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign50260_e65003_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign50260_e65003_d_n8;

        let assign50270_e65006: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign50270_e65006;

        let (assign50280_e65021, assign50280_e65021_d_n5, assign50280_e65021_d_n6, assign50280_e65021_d_n7, assign50280_e65021_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50280_e65017: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
        let assign50280_e65018: f64 = (assign50280_e65017).sqrt();
        let assign50280_e65019: f64 = (locals.var_gf__blk1307 * assign50280_e65018);
        (assign50280_e65019, ((locals.var_gf__blk1307_dn5 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn6 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn7 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn8 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8) / (2.0 * assign50280_e65018)))),)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign50280_e65021;
        locals.var_xgs__blk1358_dn5 = assign50280_e65021_d_n5;
        locals.var_xgs__blk1358_dn6 = assign50280_e65021_d_n6;
        locals.var_xgs__blk1358_dn7 = assign50280_e65021_d_n7;
        locals.var_xgs__blk1358_dn8 = assign50280_e65021_d_n8;

        let (assign50290_e65041, assign50290_e65041_d_n5, assign50290_e65041_d_n6, assign50290_e65041_d_n7, assign50290_e65041_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50290_e65031: f64 = (locals.var_gf2__blk1308 * locals.var_ds__blk1353);
        let assign50290_e65033: f64 = (assign50290_e65031 * locals.var_phit1__blk1322);
        let assign50290_e65037: f64 = (locals.var_gf__blk1307 * locals.var_sqs__blk1355);
        let assign50290_e65038: f64 = (locals.var_xgs__blk1358 + assign50290_e65037);
        let assign50290_e65039: f64 = (assign50290_e65033 / assign50290_e65038);
        (assign50290_e65039, (((((((locals.var_gf2__blk1308_dn5 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn5)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn5)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn5))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn6 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn6)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn6)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn6))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn7 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn7)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn7)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn7))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn8 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn8)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn8)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn8))))) / (assign50290_e65038 * assign50290_e65038)),)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign50290_e65041;
        locals.var_qis__blk1359_dn5 = assign50290_e65041_d_n5;
        locals.var_qis__blk1359_dn6 = assign50290_e65041_d_n6;
        locals.var_qis__blk1359_dn7 = assign50290_e65041_d_n7;
        locals.var_qis__blk1359_dn8 = assign50290_e65041_d_n8;

        let (assign50300_e65055, assign50300_e65055_d_n5, assign50300_e65055_d_n6, assign50300_e65055_d_n7, assign50300_e65055_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50300_e65051: f64 = (locals.var_sqs__blk1355 * locals.var_gf__blk1307);
        let assign50300_e65053: f64 = (assign50300_e65051 * locals.var_phit1__blk1322);
        (assign50300_e65053, ((((locals.var_sqs__blk1355_dn5 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqs__blk1355_dn6 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqs__blk1355_dn7 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqs__blk1355_dn8 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign50300_e65055;
        locals.var_qbs__blk1360_dn5 = assign50300_e65055_d_n5;
        locals.var_qbs__blk1360_dn6 = assign50300_e65055_d_n6;
        locals.var_qbs__blk1360_dn7 = assign50300_e65055_d_n7;
        locals.var_qbs__blk1360_dn8 = assign50300_e65055_d_n8;

        let assign50310_e65058: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign50310_e65058;

        let (assign50320_e65076, assign50320_e65076_d_n5, assign50320_e65076_d_n6, assign50320_e65076_d_n7, assign50320_e65076_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign50320_e65072: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
        let assign50320_e65073: f64 = (1.0 - assign50320_e65072);
        let assign50320_e65074: f64 = (1.0 / assign50320_e65073);
        (assign50320_e65074, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn5)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn6)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn7)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn8)) / (assign50320_e65073 * assign50320_e65073))),)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50320_e65076;
        locals.var_rhob__blk1361_dn5 = assign50320_e65076_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50320_e65076_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50320_e65076_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50320_e65076_d_n8;

        let (assign50330_e65093, assign50330_e65093_d_n5, assign50330_e65093_d_n6, assign50330_e65093_d_n7, assign50330_e65093_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let assign50330_e65090: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
        let assign50330_e65091: f64 = (1.0 + assign50330_e65090);
        (assign50330_e65091, (locals.var_rsb_i * locals.var_vsbx__blk1306_dn5), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn8),)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50330_e65093;
        locals.var_rhob__blk1361_dn5 = assign50330_e65093_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50330_e65093_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50330_e65093_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50330_e65093_d_n8;

        let assign50340_e65096: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign50340_e65096;

        let (assign50350_e65112, assign50350_e65112_d_n5, assign50350_e65112_d_n6, assign50350_e65112_d_n7, assign50350_e65112_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 != 0.0)) {
        let assign50350_e65109: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
        let assign50350_e65110: f64 = (1.0 - assign50350_e65109);
        (assign50350_e65110, (-(locals.var_rsg_i * locals.var_qis__blk1359_dn5)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn8)),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50350_e65112;
        locals.var_rhog__blk1362_dn5 = assign50350_e65112_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50350_e65112_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50350_e65112_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50350_e65112_d_n8;

        let (assign50360_e65131, assign50360_e65131_d_n5, assign50360_e65131_d_n6, assign50360_e65131_d_n7, assign50360_e65131_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 == 0.0)) {
        let assign50360_e65127: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
        let assign50360_e65128: f64 = (1.0 + assign50360_e65127);
        let assign50360_e65129: f64 = (1.0 / assign50360_e65128);
        (assign50360_e65129, (-((locals.var_rsg_i * locals.var_qis__blk1359_dn5) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn6) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn7) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn8) / (assign50360_e65128 * assign50360_e65128))),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50360_e65131;
        locals.var_rhog__blk1362_dn5 = assign50360_e65131_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50360_e65131_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50360_e65131_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50360_e65131_d_n8;

        let (assign50370_e65147, assign50370_e65147_d_n5, assign50370_e65147_d_n6, assign50370_e65147_d_n7, assign50370_e65147_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50370_e65141: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign50370_e65143: f64 = (assign50370_e65141 * locals.var_rhog__blk1362);
        let assign50370_e65145: f64 = (assign50370_e65143 * locals.var_qis__blk1359);
        (assign50370_e65145, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn5)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn6)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn7)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn8)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn8)),)
    } else {
        (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8,)
    }
};
        locals.var_gr__blk1363 = assign50370_e65147;
        locals.var_gr__blk1363_dn5 = assign50370_e65147_d_n5;
        locals.var_gr__blk1363_dn6 = assign50370_e65147_d_n6;
        locals.var_gr__blk1363_dn7 = assign50370_e65147_d_n7;
        locals.var_gr__blk1363_dn8 = assign50370_e65147_d_n8;

        let (assign50380_e65163, assign50380_e65163_d_n5, assign50380_e65163_d_n6, assign50380_e65163_d_n7, assign50380_e65163_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50380_e65159: f64 = (locals.var_eta_mu * locals.var_qis__blk1359);
        let assign50380_e65160: f64 = (locals.var_qbs__blk1360 + assign50380_e65159);
        let assign50380_e65161: f64 = (locals.var_e_eff0 * assign50380_e65160);
        (assign50380_e65161, (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn5 + (locals.var_eta_mu * locals.var_qis__blk1359_dn5))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn6 + (locals.var_eta_mu * locals.var_qis__blk1359_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn7 + (locals.var_eta_mu * locals.var_qis__blk1359_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn8 + (locals.var_eta_mu * locals.var_qis__blk1359_dn8))),)
    } else {
        (locals.var_eeffs__blk1364, locals.var_eeffs__blk1364_dn5, locals.var_eeffs__blk1364_dn6, locals.var_eeffs__blk1364_dn7, locals.var_eeffs__blk1364_dn8,)
    }
};
        locals.var_eeffs__blk1364 = assign50380_e65163;
        locals.var_eeffs__blk1364_dn5 = assign50380_e65163_d_n5;
        locals.var_eeffs__blk1364_dn6 = assign50380_e65163_d_n6;
        locals.var_eeffs__blk1364_dn7 = assign50380_e65163_d_n7;
        locals.var_eeffs__blk1364_dn8 = assign50380_e65163_d_n8;

        let (assign50390_e65180, assign50390_e65180_d_n5, assign50390_e65180_d_n6, assign50390_e65180_d_n7, assign50390_e65180_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50390_e65174: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
        let assign50390_e65176: f64 = (assign50390_e65174 + 1e-14);
        let assign50390_e65177: f64 = (locals.var_ps__blk1354 / assign50390_e65176);
        let assign50390_e65178: f64 = (assign50390_e65177).ln();
        (assign50390_e65178, ((((locals.var_ps__blk1354_dn5 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn6 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn7 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn8 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign50390_e65180;
        locals.var_temp1_dn5 = assign50390_e65180_d_n5;
        locals.var_temp1_dn6 = assign50390_e65180_d_n6;
        locals.var_temp1_dn7 = assign50390_e65180_d_n7;
        locals.var_temp1_dn8 = assign50390_e65180_d_n8;

        let (assign50400_e65203, assign50400_e65203_d_n5, assign50400_e65203_d_n6, assign50400_e65203_d_n7, assign50400_e65203_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50400_e65190: f64 = (locals.var_eeffs__blk1364 * locals.var_mue_t);
        let assign50400_e65192: f64 = (assign50400_e65190).powf(locals.var_themu_t);
        let assign50400_e65196: f64 = (0.5 * locals.var_thecs_t);
        let assign50400_e65198: f64 = (assign50400_e65196 * locals.var_temp1);
        let assign50400_e65199: f64 = (assign50400_e65198).exp();
        let assign50400_e65200: f64 = (locals.var_cs_t * assign50400_e65199);
        let assign50400_e65201: f64 = (assign50400_e65192 + assign50400_e65200);
        (assign50400_e65201, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn5 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn5 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn6 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn6 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn7 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn7 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn8 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn8 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8,)
    }
};
        locals.var_mutmp__blk1365 = assign50400_e65203;
        locals.var_mutmp__blk1365_dn5 = assign50400_e65203_d_n5;
        locals.var_mutmp__blk1365_dn6 = assign50400_e65203_d_n6;
        locals.var_mutmp__blk1365_dn7 = assign50400_e65203_d_n7;
        locals.var_mutmp__blk1365_dn8 = assign50400_e65203_d_n8;

        let (assign50410_e65219, assign50410_e65219_d_n5, assign50410_e65219_d_n6, assign50410_e65219_d_n7, assign50410_e65219_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50410_e65213: f64 = (1.0 + locals.var_mutmp__blk1365);
        let assign50410_e65215: f64 = (assign50410_e65213 + locals.var_gr__blk1363);
        let assign50410_e65217: f64 = (assign50410_e65215 * locals.var_rxcor__blk1357);
        (assign50410_e65217, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn8)),)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign50410_e65219;
        locals.var_gmobs__blk1366_dn5 = assign50410_e65219_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign50410_e65219_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign50410_e65219_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign50410_e65219_d_n8;

        let assign50420_e65222: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign50420_e65222;

        let (assign50430_e65240, assign50430_e65240_d_n5, assign50430_e65240_d_n6, assign50430_e65240_d_n7, assign50430_e65240_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign50430_e65236: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
        let assign50430_e65237: f64 = (1.0 - assign50430_e65236);
        let assign50430_e65238: f64 = (1.0 / assign50430_e65237);
        (assign50430_e65238, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8)) / (assign50430_e65237 * assign50430_e65237))),)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50430_e65240;
        locals.var_xitsb__blk1367_dn5 = assign50430_e65240_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50430_e65240_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50430_e65240_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50430_e65240_d_n8;

        let (assign50440_e65257, assign50440_e65257_d_n5, assign50440_e65257_d_n6, assign50440_e65257_d_n7, assign50440_e65257_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 == 0.0)) {
        let assign50440_e65254: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
        let assign50440_e65255: f64 = (1.0 + assign50440_e65254);
        (assign50440_e65255, (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8),)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50440_e65257;
        locals.var_xitsb__blk1367_dn5 = assign50440_e65257_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50440_e65257_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50440_e65257_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50440_e65257_d_n8;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50450_e65269, assign50450_e65269_d_n5, assign50450_e65269_d_n6, assign50450_e65269_d_n7, assign50450_e65269_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50450_e65267: f64 = (locals.var_qis__blk1359 * locals.var_xitsb__blk1367);
        (assign50450_e65267, ((locals.var_qis__blk1359_dn5 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qis__blk1359_dn6 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qis__blk1359_dn7 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qis__blk1359_dn8 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign50450_e65269;
        locals.var_temp2_dn5 = assign50450_e65269_d_n5;
        locals.var_temp2_dn6 = assign50450_e65269_d_n6;
        locals.var_temp2_dn7 = assign50450_e65269_d_n7;
        locals.var_temp2_dn8 = assign50450_e65269_d_n8;

        let (assign50460_e65283, assign50460_e65283_d_n5, assign50460_e65283_d_n6, assign50460_e65283_d_n7, assign50460_e65283_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        let assign50460_e65280: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign50460_e65281: f64 = (locals.var_temp2 / assign50460_e65280);
        (assign50460_e65281, (((locals.var_temp2_dn5 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn6 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn7 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn8 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50460_e65280 * assign50460_e65280)),)
    } else {
        (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8,)
    }
};
        locals.var_wsat__blk1368 = assign50460_e65283;
        locals.var_wsat__blk1368_dn5 = assign50460_e65283_d_n5;
        locals.var_wsat__blk1368_dn6 = assign50460_e65283_d_n6;
        locals.var_wsat__blk1368_dn7 = assign50460_e65283_d_n7;
        locals.var_wsat__blk1368_dn8 = assign50460_e65283_d_n8;

        let assign50470_e65286: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign50470_e65286;

        let (assign50480_e65304, assign50480_e65304_d_n5, assign50480_e65304_d_n6, assign50480_e65304_d_n7, assign50480_e65304_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign50480_e65300: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign50480_e65301: f64 = (1.0 - assign50480_e65300);
        let assign50480_e65302: f64 = (1.0 / assign50480_e65301);
        (assign50480_e65302, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign50480_e65301 * assign50480_e65301))),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50480_e65304;
        locals.var_factheta__blk1369_dn5 = assign50480_e65304_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50480_e65304_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50480_e65304_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50480_e65304_d_n8;

        let (assign50490_e65321, assign50490_e65321_d_n5, assign50490_e65321_d_n6, assign50490_e65321_d_n7, assign50490_e65321_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        let assign50490_e65318: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign50490_e65319: f64 = (1.0 + assign50490_e65318);
        (assign50490_e65319, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50490_e65321;
        locals.var_factheta__blk1369_dn5 = assign50490_e65321_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50490_e65321_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50490_e65321_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50490_e65321_d_n8;

        let (assign50590_e65420, assign50590_e65420_d_n5, assign50590_e65420_d_n6, assign50590_e65420_d_n7, assign50590_e65420_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8,)
    } else {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    }
};
        locals.var_vgb1__blk1304 = assign50590_e65420;
        locals.var_vgb1__blk1304_dn5 = assign50590_e65420_d_n5;
        locals.var_vgb1__blk1304_dn6 = assign50590_e65420_d_n6;
        locals.var_vgb1__blk1304_dn7 = assign50590_e65420_d_n7;
        locals.var_vgb1__blk1304_dn8 = assign50590_e65420_d_n8;

        let (assign50600_e65427, assign50600_e65427_d_n5, assign50600_e65427_d_n6, assign50600_e65427_d_n7, assign50600_e65427_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_vsbx_dc, locals.var_vsbx_dc_dn5, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8,)
    } else {
        (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8,)
    }
};
        locals.var_vsbx__blk1306 = assign50600_e65427;
        locals.var_vsbx__blk1306_dn5 = assign50600_e65427_d_n5;
        locals.var_vsbx__blk1306_dn6 = assign50600_e65427_d_n6;
        locals.var_vsbx__blk1306_dn7 = assign50600_e65427_d_n7;
        locals.var_vsbx__blk1306_dn8 = assign50600_e65427_d_n8;

        let (assign50610_e65434, assign50610_e65434_d_n5, assign50610_e65434_d_n6, assign50610_e65434_d_n7, assign50610_e65434_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8,)
    } else {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    }
};
        locals.var_phit1__blk1322 = assign50610_e65434;
        locals.var_phit1__blk1322_dn5 = assign50610_e65434_d_n5;
        locals.var_phit1__blk1322_dn6 = assign50610_e65434_d_n6;
        locals.var_phit1__blk1322_dn7 = assign50610_e65434_d_n7;
        locals.var_phit1__blk1322_dn8 = assign50610_e65434_d_n8;

        let (assign50620_e65441, assign50620_e65441_d_n5, assign50620_e65441_d_n6, assign50620_e65441_d_n7, assign50620_e65441_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn5, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8,)
    } else {
        (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8,)
    }
};
        locals.var_inv_phit1__blk1323 = assign50620_e65441;
        locals.var_inv_phit1__blk1323_dn5 = assign50620_e65441_d_n5;
        locals.var_inv_phit1__blk1323_dn6 = assign50620_e65441_d_n6;
        locals.var_inv_phit1__blk1323_dn7 = assign50620_e65441_d_n7;
        locals.var_inv_phit1__blk1323_dn8 = assign50620_e65441_d_n8;

        let (assign50630_e65448, assign50630_e65448_d_n5, assign50630_e65448_d_n6, assign50630_e65448_d_n7, assign50630_e65448_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8,)
    } else {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    }
};
        locals.var_gf__blk1307 = assign50630_e65448;
        locals.var_gf__blk1307_dn5 = assign50630_e65448_d_n5;
        locals.var_gf__blk1307_dn6 = assign50630_e65448_d_n6;
        locals.var_gf__blk1307_dn7 = assign50630_e65448_d_n7;
        locals.var_gf__blk1307_dn8 = assign50630_e65448_d_n8;

        let (assign50640_e65455, assign50640_e65455_d_n5, assign50640_e65455_d_n6, assign50640_e65455_d_n7, assign50640_e65455_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gf2_dc, locals.var_gf2_dc_dn5, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8,)
    } else {
        (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8,)
    }
};
        locals.var_gf2__blk1308 = assign50640_e65455;
        locals.var_gf2__blk1308_dn5 = assign50640_e65455_d_n5;
        locals.var_gf2__blk1308_dn6 = assign50640_e65455_d_n6;
        locals.var_gf2__blk1308_dn7 = assign50640_e65455_d_n7;
        locals.var_gf2__blk1308_dn8 = assign50640_e65455_d_n8;

        let (assign50650_e65462, assign50650_e65462_d_n5, assign50650_e65462_d_n6, assign50650_e65462_d_n7, assign50650_e65462_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn5, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8,)
    } else {
        (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8,)
    }
};
        locals.var_inv_gf2__blk1324 = assign50650_e65462;
        locals.var_inv_gf2__blk1324_dn5 = assign50650_e65462_d_n5;
        locals.var_inv_gf2__blk1324_dn6 = assign50650_e65462_d_n6;
        locals.var_inv_gf2__blk1324_dn7 = assign50650_e65462_d_n7;
        locals.var_inv_gf2__blk1324_dn8 = assign50650_e65462_d_n8;

        let (assign50660_e65469, assign50660_e65469_d_n5, assign50660_e65469_d_n6, assign50660_e65469_d_n7, assign50660_e65469_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xg_dc, locals.var_xg_dc_dn5, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8,)
    } else {
        (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8,)
    }
};
        locals.var_xg__blk1326 = assign50660_e65469;
        locals.var_xg__blk1326_dn5 = assign50660_e65469_d_n5;
        locals.var_xg__blk1326_dn6 = assign50660_e65469_d_n6;
        locals.var_xg__blk1326_dn7 = assign50660_e65469_d_n7;
        locals.var_xg__blk1326_dn8 = assign50660_e65469_d_n8;

        let (assign50670_e65476, assign50670_e65476_d_n5, assign50670_e65476_d_n6, assign50670_e65476_d_n7, assign50670_e65476_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8,)
    } else {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    }
};
        locals.var_xno_s__blk1331 = assign50670_e65476;
        locals.var_xno_s__blk1331_dn5 = assign50670_e65476_d_n5;
        locals.var_xno_s__blk1331_dn6 = assign50670_e65476_d_n6;
        locals.var_xno_s__blk1331_dn7 = assign50670_e65476_d_n7;
        locals.var_xno_s__blk1331_dn8 = assign50670_e65476_d_n8;

        let (assign50680_e65483, assign50680_e65483_d_n5, assign50680_e65483_d_n6, assign50680_e65483_d_n7, assign50680_e65483_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xn_s_dc, locals.var_xn_s_dc_dn5, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8,)
    } else {
        (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8,)
    }
};
        locals.var_xn_s__blk1332 = assign50680_e65483;
        locals.var_xn_s__blk1332_dn5 = assign50680_e65483_d_n5;
        locals.var_xn_s__blk1332_dn6 = assign50680_e65483_d_n6;
        locals.var_xn_s__blk1332_dn7 = assign50680_e65483_d_n7;
        locals.var_xn_s__blk1332_dn8 = assign50680_e65483_d_n8;

        let (assign50690_e65490, assign50690_e65490_d_n5, assign50690_e65490_d_n6, assign50690_e65490_d_n7, assign50690_e65490_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi_dc, locals.var_xi_dc_dn5, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8,)
    } else {
        (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8,)
    }
};
        locals.var_xi__blk1343 = assign50690_e65490;
        locals.var_xi__blk1343_dn5 = assign50690_e65490_d_n5;
        locals.var_xi__blk1343_dn6 = assign50690_e65490_d_n6;
        locals.var_xi__blk1343_dn7 = assign50690_e65490_d_n7;
        locals.var_xi__blk1343_dn8 = assign50690_e65490_d_n8;

        let (assign50700_e65497,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_margin_dc,)
    } else {
        (locals.var_margin__blk1344,)
    }
};
        locals.var_margin__blk1344 = assign50700_e65497;

        let (assign50710_e65504, assign50710_e65504_d_n5, assign50710_e65504_d_n6, assign50710_e65504_d_n7, assign50710_e65504_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn5, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8,)
    } else {
        (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8,)
    }
};
        locals.var_inv_xi__blk1345 = assign50710_e65504;
        locals.var_inv_xi__blk1345_dn5 = assign50710_e65504_d_n5;
        locals.var_inv_xi__blk1345_dn6 = assign50710_e65504_d_n6;
        locals.var_inv_xi__blk1345_dn7 = assign50710_e65504_d_n7;
        locals.var_inv_xi__blk1345_dn8 = assign50710_e65504_d_n8;

        let (assign50720_e65511, assign50720_e65511_d_n5, assign50720_e65511_d_n6, assign50720_e65511_d_n7, assign50720_e65511_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn5, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8,)
    } else {
        (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8,)
    }
};
        locals.var_sp_s_x1__blk1452 = assign50720_e65511;
        locals.var_sp_s_x1__blk1452_dn5 = assign50720_e65511_d_n5;
        locals.var_sp_s_x1__blk1452_dn6 = assign50720_e65511_d_n6;
        locals.var_sp_s_x1__blk1452_dn7 = assign50720_e65511_d_n7;
        locals.var_sp_s_x1__blk1452_dn8 = assign50720_e65511_d_n8;

        let (assign50730_e65518, assign50730_e65518_d_n5, assign50730_e65518_d_n6, assign50730_e65518_d_n7, assign50730_e65518_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn5, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8,)
    } else {
        (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8,)
    }
};
        locals.var_delta_ns__blk1347 = assign50730_e65518;
        locals.var_delta_ns__blk1347_dn5 = assign50730_e65518_d_n5;
        locals.var_delta_ns__blk1347_dn6 = assign50730_e65518_d_n6;
        locals.var_delta_ns__blk1347_dn7 = assign50730_e65518_d_n7;
        locals.var_delta_ns__blk1347_dn8 = assign50730_e65518_d_n8;

        let (assign50740_e65525, assign50740_e65525_d_n5, assign50740_e65525_d_n6, assign50740_e65525_d_n7, assign50740_e65525_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_x_s_dc, locals.var_x_s_dc_dn5, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8,)
    } else {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    }
};
        locals.var_x_s__blk1346 = assign50740_e65525;
        locals.var_x_s__blk1346_dn5 = assign50740_e65525_d_n5;
        locals.var_x_s__blk1346_dn6 = assign50740_e65525_d_n6;
        locals.var_x_s__blk1346_dn7 = assign50740_e65525_d_n7;
        locals.var_x_s__blk1346_dn8 = assign50740_e65525_d_n8;

        let (assign50750_e65532, assign50750_e65532_d_n5, assign50750_e65532_d_n6, assign50750_e65532_d_n7, assign50750_e65532_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi1s_dc, locals.var_xi1s_dc_dn5, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8,)
    } else {
        (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8,)
    }
};
        locals.var_xi1s__blk1349 = assign50750_e65532;
        locals.var_xi1s__blk1349_dn5 = assign50750_e65532_d_n5;
        locals.var_xi1s__blk1349_dn6 = assign50750_e65532_d_n6;
        locals.var_xi1s__blk1349_dn7 = assign50750_e65532_d_n7;
        locals.var_xi1s__blk1349_dn8 = assign50750_e65532_d_n8;

        let (assign50760_e65539, assign50760_e65539_d_n5, assign50760_e65539_d_n6, assign50760_e65539_d_n7, assign50760_e65539_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xi2s_dc, locals.var_xi2s_dc_dn5, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8,)
    } else {
        (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8,)
    }
};
        locals.var_xi2s__blk1350 = assign50760_e65539;
        locals.var_xi2s__blk1350_dn5 = assign50760_e65539_d_n5;
        locals.var_xi2s__blk1350_dn6 = assign50760_e65539_d_n6;
        locals.var_xi2s__blk1350_dn7 = assign50760_e65539_d_n7;
        locals.var_xi2s__blk1350_dn8 = assign50760_e65539_d_n8;

        let (assign50770_e65546, assign50770_e65546_d_n5, assign50770_e65546_d_n6, assign50770_e65546_d_n7, assign50770_e65546_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn5, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8,)
    } else {
        (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8,)
    }
};
        locals.var_delta_1s__blk1351 = assign50770_e65546;
        locals.var_delta_1s__blk1351_dn5 = assign50770_e65546_d_n5;
        locals.var_delta_1s__blk1351_dn6 = assign50770_e65546_d_n6;
        locals.var_delta_1s__blk1351_dn7 = assign50770_e65546_d_n7;
        locals.var_delta_1s__blk1351_dn8 = assign50770_e65546_d_n8;

        let (assign50780_e65553, assign50780_e65553_d_n5, assign50780_e65553_d_n6, assign50780_e65553_d_n7, assign50780_e65553_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_es_dc, locals.var_es_dc_dn5, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8,)
    } else {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    }
};
        locals.var_es__blk1352 = assign50780_e65553;
        locals.var_es__blk1352_dn5 = assign50780_e65553_d_n5;
        locals.var_es__blk1352_dn6 = assign50780_e65553_d_n6;
        locals.var_es__blk1352_dn7 = assign50780_e65553_d_n7;
        locals.var_es__blk1352_dn8 = assign50780_e65553_d_n8;

        let (assign50790_e65560, assign50790_e65560_d_n5, assign50790_e65560_d_n6, assign50790_e65560_d_n7, assign50790_e65560_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_ps_dc, locals.var_ps_dc_dn5, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8,)
    } else {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    }
};
        locals.var_ps__blk1354 = assign50790_e65560;
        locals.var_ps__blk1354_dn5 = assign50790_e65560_d_n5;
        locals.var_ps__blk1354_dn6 = assign50790_e65560_d_n6;
        locals.var_ps__blk1354_dn7 = assign50790_e65560_d_n7;
        locals.var_ps__blk1354_dn8 = assign50790_e65560_d_n8;

        let (assign50800_e65567, assign50800_e65567_d_n5, assign50800_e65567_d_n6, assign50800_e65567_d_n7, assign50800_e65567_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_ds_dc, locals.var_ds_dc_dn5, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8,)
    } else {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    }
};
        locals.var_ds__blk1353 = assign50800_e65567;
        locals.var_ds__blk1353_dn5 = assign50800_e65567_d_n5;
        locals.var_ds__blk1353_dn6 = assign50800_e65567_d_n6;
        locals.var_ds__blk1353_dn7 = assign50800_e65567_d_n7;
        locals.var_ds__blk1353_dn8 = assign50800_e65567_d_n8;

        let (assign50810_e65574, assign50810_e65574_d_n5, assign50810_e65574_d_n6, assign50810_e65574_d_n7, assign50810_e65574_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_sqs_dc, locals.var_sqs_dc_dn5, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8,)
    } else {
        (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8,)
    }
};
        locals.var_sqs__blk1355 = assign50810_e65574;
        locals.var_sqs__blk1355_dn5 = assign50810_e65574_d_n5;
        locals.var_sqs__blk1355_dn6 = assign50810_e65574_d_n6;
        locals.var_sqs__blk1355_dn7 = assign50810_e65574_d_n7;
        locals.var_sqs__blk1355_dn8 = assign50810_e65574_d_n8;

        let (assign50820_e65581, assign50820_e65581_d_n5, assign50820_e65581_d_n6, assign50820_e65581_d_n7, assign50820_e65581_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_alphas_dc, locals.var_alphas_dc_dn5, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8,)
    } else {
        (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8,)
    }
};
        locals.var_alphas__blk1356 = assign50820_e65581;
        locals.var_alphas__blk1356_dn5 = assign50820_e65581_d_n5;
        locals.var_alphas__blk1356_dn6 = assign50820_e65581_d_n6;
        locals.var_alphas__blk1356_dn7 = assign50820_e65581_d_n7;
        locals.var_alphas__blk1356_dn8 = assign50820_e65581_d_n8;

        let (assign50830_e65588, assign50830_e65588_d_n5, assign50830_e65588_d_n6, assign50830_e65588_d_n7, assign50830_e65588_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rxcor_dc, locals.var_rxcor_dc_dn5, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8,)
    } else {
        (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8,)
    }
};
        locals.var_rxcor__blk1357 = assign50830_e65588;
        locals.var_rxcor__blk1357_dn5 = assign50830_e65588_d_n5;
        locals.var_rxcor__blk1357_dn6 = assign50830_e65588_d_n6;
        locals.var_rxcor__blk1357_dn7 = assign50830_e65588_d_n7;
        locals.var_rxcor__blk1357_dn8 = assign50830_e65588_d_n8;

        let (assign50840_e65595, assign50840_e65595_d_n5, assign50840_e65595_d_n6, assign50840_e65595_d_n7, assign50840_e65595_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xgs_dc, locals.var_xgs_dc_dn5, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8,)
    } else {
        (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8,)
    }
};
        locals.var_xgs__blk1358 = assign50840_e65595;
        locals.var_xgs__blk1358_dn5 = assign50840_e65595_d_n5;
        locals.var_xgs__blk1358_dn6 = assign50840_e65595_d_n6;
        locals.var_xgs__blk1358_dn7 = assign50840_e65595_d_n7;
        locals.var_xgs__blk1358_dn8 = assign50840_e65595_d_n8;

        let (assign50850_e65602, assign50850_e65602_d_n5, assign50850_e65602_d_n6, assign50850_e65602_d_n7, assign50850_e65602_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_qis_dc, locals.var_qis_dc_dn5, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8,)
    } else {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    }
};
        locals.var_qis__blk1359 = assign50850_e65602;
        locals.var_qis__blk1359_dn5 = assign50850_e65602_d_n5;
        locals.var_qis__blk1359_dn6 = assign50850_e65602_d_n6;
        locals.var_qis__blk1359_dn7 = assign50850_e65602_d_n7;
        locals.var_qis__blk1359_dn8 = assign50850_e65602_d_n8;

        let (assign50860_e65609, assign50860_e65609_d_n5, assign50860_e65609_d_n6, assign50860_e65609_d_n7, assign50860_e65609_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8,)
    } else {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    }
};
        locals.var_qbs__blk1360 = assign50860_e65609;
        locals.var_qbs__blk1360_dn5 = assign50860_e65609_d_n5;
        locals.var_qbs__blk1360_dn6 = assign50860_e65609_d_n6;
        locals.var_qbs__blk1360_dn7 = assign50860_e65609_d_n7;
        locals.var_qbs__blk1360_dn8 = assign50860_e65609_d_n8;

        let (assign50870_e65616, assign50870_e65616_d_n5, assign50870_e65616_d_n6, assign50870_e65616_d_n7, assign50870_e65616_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rhob_dc, locals.var_rhob_dc_dn5, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8,)
    } else {
        (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8,)
    }
};
        locals.var_rhob__blk1361 = assign50870_e65616;
        locals.var_rhob__blk1361_dn5 = assign50870_e65616_d_n5;
        locals.var_rhob__blk1361_dn6 = assign50870_e65616_d_n6;
        locals.var_rhob__blk1361_dn7 = assign50870_e65616_d_n7;
        locals.var_rhob__blk1361_dn8 = assign50870_e65616_d_n8;

        let (assign50880_e65623, assign50880_e65623_d_n5, assign50880_e65623_d_n6, assign50880_e65623_d_n7, assign50880_e65623_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_rhog_dc, locals.var_rhog_dc_dn5, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8,)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign50880_e65623;
        locals.var_rhog__blk1362_dn5 = assign50880_e65623_d_n5;
        locals.var_rhog__blk1362_dn6 = assign50880_e65623_d_n6;
        locals.var_rhog__blk1362_dn7 = assign50880_e65623_d_n7;
        locals.var_rhog__blk1362_dn8 = assign50880_e65623_d_n8;

        let (assign50890_e65630, assign50890_e65630_d_n5, assign50890_e65630_d_n6, assign50890_e65630_d_n7, assign50890_e65630_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_gmobs_dc, locals.var_gmobs_dc_dn5, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8,)
    } else {
        (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8,)
    }
};
        locals.var_gmobs__blk1366 = assign50890_e65630;
        locals.var_gmobs__blk1366_dn5 = assign50890_e65630_d_n5;
        locals.var_gmobs__blk1366_dn6 = assign50890_e65630_d_n6;
        locals.var_gmobs__blk1366_dn7 = assign50890_e65630_d_n7;
        locals.var_gmobs__blk1366_dn8 = assign50890_e65630_d_n8;

        let (assign50900_e65637, assign50900_e65637_d_n5, assign50900_e65637_d_n6, assign50900_e65637_d_n7, assign50900_e65637_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_xitsb_dc, locals.var_xitsb_dc_dn5, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8,)
    } else {
        (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8,)
    }
};
        locals.var_xitsb__blk1367 = assign50900_e65637;
        locals.var_xitsb__blk1367_dn5 = assign50900_e65637_d_n5;
        locals.var_xitsb__blk1367_dn6 = assign50900_e65637_d_n6;
        locals.var_xitsb__blk1367_dn7 = assign50900_e65637_d_n7;
        locals.var_xitsb__blk1367_dn8 = assign50900_e65637_d_n8;

        let (assign50910_e65644, assign50910_e65644_d_n5, assign50910_e65644_d_n6, assign50910_e65644_d_n7, assign50910_e65644_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
        (locals.var_factheta_dc, locals.var_factheta_dc_dn5, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8,)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign50910_e65644;
        locals.var_factheta__blk1369_dn5 = assign50910_e65644_d_n5;
        locals.var_factheta__blk1369_dn6 = assign50910_e65644_d_n6;
        locals.var_factheta__blk1369_dn7 = assign50910_e65644_d_n7;
        locals.var_factheta__blk1369_dn8 = assign50910_e65644_d_n8;

        let (assign50930_e65655,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesat_t,)
    } else {
        (locals.var_thesatloc__blk1302,)
    }
};
        locals.var_thesatloc__blk1302 = assign50930_e65655;

        let (assign50940_e65659,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ar,)
    } else {
        (locals.var_arloc__blk1303,)
    }
};
        locals.var_arloc__blk1303 = assign50940_e65659;

        let assign50950_e65662: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign50950_e65662;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign50960_e65668,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1483 != 0.0)) {
        (locals.var_thesatac_t,)
    } else {
        (locals.var_thesatloc__blk1302,)
    }
};
        locals.var_thesatloc__blk1302 = assign50960_e65668;

        let (assign50970_e65674,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1483 != 0.0)) {
        (locals.var_arac,)
    } else {
        (locals.var_arloc__blk1303,)
    }
};
        locals.var_arloc__blk1303 = assign50970_e65674;

        let (assign50980_e65678, assign50980_e65678_d_n5, assign50980_e65678_d_n6, assign50980_e65678_d_n7, assign50980_e65678_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8,)
    }
};
        locals.var_thesat1__blk1371 = assign50980_e65678;
        locals.var_thesat1__blk1371_dn5 = assign50980_e65678_d_n5;
        locals.var_thesat1__blk1371_dn6 = assign50980_e65678_d_n6;
        locals.var_thesat1__blk1371_dn7 = assign50980_e65678_d_n7;
        locals.var_thesat1__blk1371_dn8 = assign50980_e65678_d_n8;

        let (assign50990_e65684, assign50990_e65684_d_n5, assign50990_e65684_d_n6, assign50990_e65684_d_n7, assign50990_e65684_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign50990_e65682: f64 = (locals.var_phit1__blk1322 * 4.60517018598809);
        (assign50990_e65682, (locals.var_phit1__blk1322_dn5 * 4.60517018598809), (locals.var_phit1__blk1322_dn6 * 4.60517018598809), (locals.var_phit1__blk1322_dn7 * 4.60517018598809), (locals.var_phit1__blk1322_dn8 * 4.60517018598809),)
    } else {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    }
};
        locals.var_vdsat_lim__blk1370 = assign50990_e65684;
        locals.var_vdsat_lim__blk1370_dn5 = assign50990_e65684_d_n5;
        locals.var_vdsat_lim__blk1370_dn6 = assign50990_e65684_d_n6;
        locals.var_vdsat_lim__blk1370_dn7 = assign50990_e65684_d_n7;
        locals.var_vdsat_lim__blk1370_dn8 = assign50990_e65684_d_n8;

        let (assign51000_e65688, assign51000_e65688_d_n5, assign51000_e65688_d_n6, assign51000_e65688_d_n7, assign51000_e65688_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51000_e65688;
        locals.var_v_dsat__blk1387_dn5 = assign51000_e65688_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51000_e65688_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51000_e65688_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51000_e65688_d_n8;

        let (assign51010_e65692, assign51010_e65692_d_n5, assign51010_e65692_d_n6, assign51010_e65692_d_n7, assign51010_e65692_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_v_ds, 0.0, locals.var_v_ds_dn6, locals.var_v_ds_dn7, 0.0,)
    } else {
        (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8,)
    }
};
        locals.var_vdse__blk1388 = assign51010_e65692;
        locals.var_vdse__blk1388_dn5 = assign51010_e65692_d_n5;
        locals.var_vdse__blk1388_dn6 = assign51010_e65692_d_n6;
        locals.var_vdse__blk1388_dn7 = assign51010_e65692_d_n7;
        locals.var_vdse__blk1388_dn8 = assign51010_e65692_d_n8;

        let (assign51020_e65698, assign51020_e65698_d_n5, assign51020_e65698_d_n6, assign51020_e65698_d_n7, assign51020_e65698_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51020_e65696: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1323);
        (assign51020_e65696, (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn5), ((locals.var_v_ds_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn7)), (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn8),)
    } else {
        (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8,)
    }
};
        locals.var_udse__blk1389 = assign51020_e65698;
        locals.var_udse__blk1389_dn5 = assign51020_e65698_d_n5;
        locals.var_udse__blk1389_dn6 = assign51020_e65698_d_n6;
        locals.var_udse__blk1389_dn7 = assign51020_e65698_d_n7;
        locals.var_udse__blk1389_dn8 = assign51020_e65698_d_n8;

        let (assign51030_e65702, assign51030_e65702_d_n5, assign51030_e65702_d_n6, assign51030_e65702_d_n7, assign51030_e65702_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign51030_e65702;
        locals.var_x_d__blk1393_dn5 = assign51030_e65702_d_n5;
        locals.var_x_d__blk1393_dn6 = assign51030_e65702_d_n6;
        locals.var_x_d__blk1393_dn7 = assign51030_e65702_d_n7;
        locals.var_x_d__blk1393_dn8 = assign51030_e65702_d_n8;

        let (assign51040_e65706, assign51040_e65706_d_n5, assign51040_e65706_d_n6, assign51040_e65706_d_n7, assign51040_e65706_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign51040_e65706;
        locals.var_x_ds__blk1394_dn5 = assign51040_e65706_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign51040_e65706_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign51040_e65706_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign51040_e65706_d_n8;

        let (assign51050_e65710, assign51050_e65710_d_n5, assign51050_e65710_d_n6, assign51050_e65710_d_n7, assign51050_e65710_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign51050_e65710;
        locals.var_dps__blk1397_dn5 = assign51050_e65710_d_n5;
        locals.var_dps__blk1397_dn6 = assign51050_e65710_d_n6;
        locals.var_dps__blk1397_dn7 = assign51050_e65710_d_n7;
        locals.var_dps__blk1397_dn8 = assign51050_e65710_d_n8;

        let (assign51060_e65714, assign51060_e65714_d_n5, assign51060_e65714_d_n6, assign51060_e65714_d_n7, assign51060_e65714_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign51060_e65714;
        locals.var_ed__blk1399_dn5 = assign51060_e65714_d_n5;
        locals.var_ed__blk1399_dn6 = assign51060_e65714_d_n6;
        locals.var_ed__blk1399_dn7 = assign51060_e65714_d_n7;
        locals.var_ed__blk1399_dn8 = assign51060_e65714_d_n8;

        let (assign51070_e65718, assign51070_e65718_d_n5, assign51070_e65718_d_n6, assign51070_e65718_d_n7, assign51070_e65718_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign51070_e65718;
        locals.var_pd__blk1400_dn5 = assign51070_e65718_d_n5;
        locals.var_pd__blk1400_dn6 = assign51070_e65718_d_n6;
        locals.var_pd__blk1400_dn7 = assign51070_e65718_d_n7;
        locals.var_pd__blk1400_dn8 = assign51070_e65718_d_n8;

        let (assign51080_e65722, assign51080_e65722_d_n5, assign51080_e65722_d_n6, assign51080_e65722_d_n7, assign51080_e65722_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign51080_e65722;
        locals.var_dd__blk1402_dn5 = assign51080_e65722_d_n5;
        locals.var_dd__blk1402_dn6 = assign51080_e65722_d_n6;
        locals.var_dd__blk1402_dn7 = assign51080_e65722_d_n7;
        locals.var_dd__blk1402_dn8 = assign51080_e65722_d_n8;

        let (assign51090_e65726, assign51090_e65726_d_n5, assign51090_e65726_d_n6, assign51090_e65726_d_n7, assign51090_e65726_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    }
};
        locals.var_qbd__blk1403 = assign51090_e65726;
        locals.var_qbd__blk1403_dn5 = assign51090_e65726_d_n5;
        locals.var_qbd__blk1403_dn6 = assign51090_e65726_d_n6;
        locals.var_qbd__blk1403_dn7 = assign51090_e65726_d_n7;
        locals.var_qbd__blk1403_dn8 = assign51090_e65726_d_n8;

        let (assign51100_e65730, assign51100_e65730_d_n5, assign51100_e65730_d_n6, assign51100_e65730_d_n7, assign51100_e65730_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8,)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign51100_e65730;
        locals.var_x_m__blk1404_dn5 = assign51100_e65730_d_n5;
        locals.var_x_m__blk1404_dn6 = assign51100_e65730_d_n6;
        locals.var_x_m__blk1404_dn7 = assign51100_e65730_d_n7;
        locals.var_x_m__blk1404_dn8 = assign51100_e65730_d_n8;

        let (assign51110_e65734, assign51110_e65734_d_n5, assign51110_e65734_d_n6, assign51110_e65734_d_n7, assign51110_e65734_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8,)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign51110_e65734;
        locals.var_em__blk1405_dn5 = assign51110_e65734_d_n5;
        locals.var_em__blk1405_dn6 = assign51110_e65734_d_n6;
        locals.var_em__blk1405_dn7 = assign51110_e65734_d_n7;
        locals.var_em__blk1405_dn8 = assign51110_e65734_d_n8;

        let (assign51120_e65738, assign51120_e65738_d_n5, assign51120_e65738_d_n6, assign51120_e65738_d_n7, assign51120_e65738_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8,)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign51120_e65738;
        locals.var_dm__blk1407_dn5 = assign51120_e65738_d_n5;
        locals.var_dm__blk1407_dn6 = assign51120_e65738_d_n6;
        locals.var_dm__blk1407_dn7 = assign51120_e65738_d_n7;
        locals.var_dm__blk1407_dn8 = assign51120_e65738_d_n8;

        let (assign51130_e65742, assign51130_e65742_d_n5, assign51130_e65742_d_n6, assign51130_e65742_d_n7, assign51130_e65742_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8,)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign51130_e65742;
        locals.var_pm__blk1408_dn5 = assign51130_e65742_d_n5;
        locals.var_pm__blk1408_dn6 = assign51130_e65742_d_n6;
        locals.var_pm__blk1408_dn7 = assign51130_e65742_d_n7;
        locals.var_pm__blk1408_dn8 = assign51130_e65742_d_n8;

        let (assign51140_e65748, assign51140_e65748_d_n5, assign51140_e65748_d_n6, assign51140_e65748_d_n7, assign51140_e65748_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51140_e65746: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        (assign51140_e65746, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign51140_e65748;
        locals.var_xgm__blk1409_dn5 = assign51140_e65748_d_n5;
        locals.var_xgm__blk1409_dn6 = assign51140_e65748_d_n6;
        locals.var_xgm__blk1409_dn7 = assign51140_e65748_d_n7;
        locals.var_xgm__blk1409_dn8 = assign51140_e65748_d_n8;

        let (assign51150_e65752, assign51150_e65752_d_n5, assign51150_e65752_d_n6, assign51150_e65752_d_n7, assign51150_e65752_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign51150_e65752;
        locals.var_eta_p__blk1410_dn5 = assign51150_e65752_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign51150_e65752_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign51150_e65752_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign51150_e65752_d_n8;

        let (assign51160_e65756, assign51160_e65756_d_n5, assign51160_e65756_d_n6, assign51160_e65756_d_n7, assign51160_e65756_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign51160_e65756;
        locals.var_alpha__blk1412_dn5 = assign51160_e65756_d_n5;
        locals.var_alpha__blk1412_dn6 = assign51160_e65756_d_n6;
        locals.var_alpha__blk1412_dn7 = assign51160_e65756_d_n7;
        locals.var_alpha__blk1412_dn8 = assign51160_e65756_d_n8;

        let (assign51170_e65760, assign51170_e65760_d_n5, assign51170_e65760_d_n6, assign51170_e65760_d_n7, assign51170_e65760_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign51170_e65760;
        locals.var_sqm__blk1411_dn5 = assign51170_e65760_d_n5;
        locals.var_sqm__blk1411_dn6 = assign51170_e65760_d_n6;
        locals.var_sqm__blk1411_dn7 = assign51170_e65760_d_n7;
        locals.var_sqm__blk1411_dn8 = assign51170_e65760_d_n8;

        let (assign51180_e65764, assign51180_e65764_d_n5, assign51180_e65764_d_n6, assign51180_e65764_d_n7, assign51180_e65764_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8,)
    } else {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    }
};
        locals.var_qim__blk1421 = assign51180_e65764;
        locals.var_qim__blk1421_dn5 = assign51180_e65764_d_n5;
        locals.var_qim__blk1421_dn6 = assign51180_e65764_d_n6;
        locals.var_qim__blk1421_dn7 = assign51180_e65764_d_n7;
        locals.var_qim__blk1421_dn8 = assign51180_e65764_d_n8;

        let (assign51190_e65770, assign51190_e65770_d_n5, assign51190_e65770_d_n6, assign51190_e65770_d_n7, assign51190_e65770_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        let assign51190_e65768: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
        (assign51190_e65768, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    }
};
        locals.var_qeff1__blk1425 = assign51190_e65770;
        locals.var_qeff1__blk1425_dn5 = assign51190_e65770_d_n5;
        locals.var_qeff1__blk1425_dn6 = assign51190_e65770_d_n6;
        locals.var_qeff1__blk1425_dn7 = assign51190_e65770_d_n7;
        locals.var_qeff1__blk1425_dn8 = assign51190_e65770_d_n8;

        let (assign51200_e65774, assign51200_e65774_d_n5, assign51200_e65774_d_n6, assign51200_e65774_d_n7, assign51200_e65774_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    }
};
        locals.var_qim1__blk1422 = assign51200_e65774;
        locals.var_qim1__blk1422_dn5 = assign51200_e65774_d_n5;
        locals.var_qim1__blk1422_dn6 = assign51200_e65774_d_n6;
        locals.var_qim1__blk1422_dn7 = assign51200_e65774_d_n7;
        locals.var_qim1__blk1422_dn8 = assign51200_e65774_d_n8;

        let (assign51210_e65778, assign51210_e65778_d_n5, assign51210_e65778_d_n6, assign51210_e65778_d_n7, assign51210_e65778_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8,)
    }
};
        locals.var_qbm__blk1423 = assign51210_e65778;
        locals.var_qbm__blk1423_dn5 = assign51210_e65778_d_n5;
        locals.var_qbm__blk1423_dn6 = assign51210_e65778_d_n6;
        locals.var_qbm__blk1423_dn7 = assign51210_e65778_d_n7;
        locals.var_qbm__blk1423_dn8 = assign51210_e65778_d_n8;

        let (assign51220_e65782, assign51220_e65782_d_n5, assign51220_e65782_d_n6, assign51220_e65782_d_n7, assign51220_e65782_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    }
};
        locals.var_s1__blk1428 = assign51220_e65782;
        locals.var_s1__blk1428_dn5 = assign51220_e65782_d_n5;
        locals.var_s1__blk1428_dn6 = assign51220_e65782_d_n6;
        locals.var_s1__blk1428_dn7 = assign51220_e65782_d_n7;
        locals.var_s1__blk1428_dn8 = assign51220_e65782_d_n8;

        let (assign51230_e65786, assign51230_e65786_d_n5, assign51230_e65786_d_n6, assign51230_e65786_d_n7, assign51230_e65786_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    }
};
        locals.var_gmob__blk1427 = assign51230_e65786;
        locals.var_gmob__blk1427_dn5 = assign51230_e65786_d_n5;
        locals.var_gmob__blk1427_dn6 = assign51230_e65786_d_n6;
        locals.var_gmob__blk1427_dn7 = assign51230_e65786_d_n7;
        locals.var_gmob__blk1427_dn8 = assign51230_e65786_d_n8;

        let (assign51240_e65790, assign51240_e65790_d_n5, assign51240_e65790_d_n6, assign51240_e65790_d_n7, assign51240_e65790_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesatloc__blk1302, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign51240_e65790;
        locals.var_thesateff__blk1430_dn5 = assign51240_e65790_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign51240_e65790_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign51240_e65790_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign51240_e65790_d_n8;

        let (assign51250_e65794, assign51250_e65794_d_n5, assign51250_e65794_d_n6, assign51250_e65794_d_n7, assign51250_e65794_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    } else {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    }
};
        locals.var_voxm__blk1429 = assign51250_e65794;
        locals.var_voxm__blk1429_dn5 = assign51250_e65794_d_n5;
        locals.var_voxm__blk1429_dn6 = assign51250_e65794_d_n6;
        locals.var_voxm__blk1429_dn7 = assign51250_e65794_d_n7;
        locals.var_voxm__blk1429_dn8 = assign51250_e65794_d_n8;

        let assign51260_e65797: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign51260_e65797;

        let assign51270_e65800: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign51270_e65800;

        let (assign51280_e65810, assign51280_e65810_d_n5, assign51280_e65810_d_n6, assign51280_e65810_d_n7, assign51280_e65810_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51280_e65808: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
        (assign51280_e65808, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8),)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign51280_e65810;
        locals.var_thesateff__blk1430_dn5 = assign51280_e65810_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign51280_e65810_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign51280_e65810_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign51280_e65810_d_n8;

        let (assign51290_e65820, assign51290_e65820_d_n5, assign51290_e65820_d_n6, assign51290_e65820_d_n7, assign51290_e65820_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51290_e65818: f64 = (locals.var_thesateff__blk1430 / locals.var_gmobs__blk1366);
        (assign51290_e65818, (((locals.var_thesateff__blk1430_dn5 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn5)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn6 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn6)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn7 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn7)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn8 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn8)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)),)
    } else {
        (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8,)
    }
};
        locals.var_thesat1__blk1371 = assign51290_e65820;
        locals.var_thesat1__blk1371_dn5 = assign51290_e65820_d_n5;
        locals.var_thesat1__blk1371_dn6 = assign51290_e65820_d_n6;
        locals.var_thesat1__blk1371_dn7 = assign51290_e65820_d_n7;
        locals.var_thesat1__blk1371_dn8 = assign51290_e65820_d_n8;

        let (assign51300_e65832, assign51300_e65832_d_n5, assign51300_e65832_d_n6, assign51300_e65832_d_n7, assign51300_e65832_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51300_e65829: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign51300_e65830: f64 = (locals.var_xgs__blk1358 + assign51300_e65829);
        (assign51300_e65830, (locals.var_xgs__blk1358_dn5 + (0.5 * locals.var_gf2__blk1308_dn5)), (locals.var_xgs__blk1358_dn6 + (0.5 * locals.var_gf2__blk1308_dn6)), (locals.var_xgs__blk1358_dn7 + (0.5 * locals.var_gf2__blk1308_dn7)), (locals.var_xgs__blk1358_dn8 + (0.5 * locals.var_gf2__blk1308_dn8)),)
    } else {
        (locals.var_asat__blk1372, locals.var_asat__blk1372_dn5, locals.var_asat__blk1372_dn6, locals.var_asat__blk1372_dn7, locals.var_asat__blk1372_dn8,)
    }
};
        locals.var_asat__blk1372 = assign51300_e65832;
        locals.var_asat__blk1372_dn5 = assign51300_e65832_d_n5;
        locals.var_asat__blk1372_dn6 = assign51300_e65832_d_n6;
        locals.var_asat__blk1372_dn7 = assign51300_e65832_d_n7;
        locals.var_asat__blk1372_dn8 = assign51300_e65832_d_n8;

        let (assign51310_e65846, assign51310_e65846_d_n5, assign51310_e65846_d_n6, assign51310_e65846_d_n7, assign51310_e65846_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51310_e65840: f64 = (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1372;
        let assign51310_e65842: f64 = (assign51310_e65840 * __rspice_inv_cse_0);
        let assign51310_e65844: f64 = (assign51310_e65842 * __rspice_inv_cse_0);
        (assign51310_e65844, ((((((((locals.var_gf2__blk1308_dn5 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn5)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn6 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn6)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn7 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn7)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn8 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn8)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51310_e65846;
        locals.var_temp__blk936_dn5 = assign51310_e65846_d_n5;
        locals.var_temp__blk936_dn6 = assign51310_e65846_d_n6;
        locals.var_temp__blk936_dn7 = assign51310_e65846_d_n7;
        locals.var_temp__blk936_dn8 = assign51310_e65846_d_n8;

        let assign51320_e65849: f64 = if locals.var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign51320_e65849;

        let (assign51330_e65861, assign51330_e65861_d_n5, assign51330_e65861_d_n6, assign51330_e65861_d_n7, assign51330_e65861_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign51330_e65859: f64 = (1.0 - locals.var_temp__blk936);
        (assign51330_e65859, (-locals.var_temp__blk936_dn5), (-locals.var_temp__blk936_dn6), (-locals.var_temp__blk936_dn7), (-locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51330_e65861;
        locals.var_temp1_dn5 = assign51330_e65861_d_n5;
        locals.var_temp1_dn6 = assign51330_e65861_d_n6;
        locals.var_temp1_dn7 = assign51330_e65861_d_n7;
        locals.var_temp1_dn8 = assign51330_e65861_d_n8;

        let assign51340_e65864: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign51340_e65864;

        let (assign51350_e65876, assign51350_e65876_d_n5, assign51350_e65876_d_n6, assign51350_e65876_d_n7, assign51350_e65876_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51350_e65876;
        locals.var_temp2_dn5 = assign51350_e65876_d_n5;
        locals.var_temp2_dn6 = assign51350_e65876_d_n6;
        locals.var_temp2_dn7 = assign51350_e65876_d_n7;
        locals.var_temp2_dn8 = assign51350_e65876_d_n8;

        let (assign51360_e65892, assign51360_e65892_d_n5, assign51360_e65892_d_n6, assign51360_e65892_d_n7, assign51360_e65892_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
        let assign51360_e65889: f64 = (locals.var_temp1).sqrt();
        let assign51360_e65890: f64 = (1.0 - assign51360_e65889);
        (assign51360_e65890, (-(locals.var_temp1_dn5 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn6 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn7 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn8 / (2.0 * assign51360_e65889))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51360_e65892;
        locals.var_temp2_dn5 = assign51360_e65892_d_n5;
        locals.var_temp2_dn6 = assign51360_e65892_d_n6;
        locals.var_temp2_dn7 = assign51360_e65892_d_n7;
        locals.var_temp2_dn8 = assign51360_e65892_d_n8;

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign51370_e65905, assign51370_e65905_d_n5, assign51370_e65905_d_n6, assign51370_e65905_d_n7, assign51370_e65905_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign51370_e65903: f64 = (0.5 * locals.var_temp__blk936);
        (assign51370_e65903, (0.5 * locals.var_temp__blk936_dn5), (0.5 * locals.var_temp__blk936_dn6), (0.5 * locals.var_temp__blk936_dn7), (0.5 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51370_e65905;
        locals.var_temp2_dn5 = assign51370_e65905_d_n5;
        locals.var_temp2_dn6 = assign51370_e65905_d_n6;
        locals.var_temp2_dn7 = assign51370_e65905_d_n7;
        locals.var_temp2_dn8 = assign51370_e65905_d_n8;

        let (assign51380_e65915, assign51380_e65915_d_n5, assign51380_e65915_d_n6, assign51380_e65915_d_n7, assign51380_e65915_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51380_e65913: f64 = (locals.var_temp2 * locals.var_asat__blk1372);
        (assign51380_e65913, ((locals.var_temp2_dn5 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn5)), ((locals.var_temp2_dn6 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn8)),)
    } else {
        (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8,)
    }
};
        locals.var_x_inf0__blk1373 = assign51380_e65915;
        locals.var_x_inf0__blk1373_dn5 = assign51380_e65915_d_n5;
        locals.var_x_inf0__blk1373_dn6 = assign51380_e65915_d_n6;
        locals.var_x_inf0__blk1373_dn7 = assign51380_e65915_d_n7;
        locals.var_x_inf0__blk1373_dn8 = assign51380_e65915_d_n8;

        let assign51390_e65922: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign51390_e65922;

        let (assign51400_e65936, assign51400_e65936_d_n5, assign51400_e65936_d_n6, assign51400_e65936_d_n7, assign51400_e65936_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51400_e65932: f64 = (0.475 * locals.var_phit1__blk1322);
        let assign51400_e65934: f64 = (assign51400_e65932 * locals.var_x_inf0__blk1373);
        (assign51400_e65934, (((0.475 * locals.var_phit1__blk1322_dn5) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn5)), (((0.475 * locals.var_phit1__blk1322_dn6) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn6)), (((0.475 * locals.var_phit1__blk1322_dn7) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn7)), (((0.475 * locals.var_phit1__blk1322_dn8) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn8)),)
    } else {
        (locals.var_midphi0__blk1374, locals.var_midphi0__blk1374_dn5, locals.var_midphi0__blk1374_dn6, locals.var_midphi0__blk1374_dn7, locals.var_midphi0__blk1374_dn8,)
    }
};
        locals.var_midphi0__blk1374 = assign51400_e65936;
        locals.var_midphi0__blk1374_dn5 = assign51400_e65936_d_n5;
        locals.var_midphi0__blk1374_dn6 = assign51400_e65936_d_n6;
        locals.var_midphi0__blk1374_dn7 = assign51400_e65936_d_n7;
        locals.var_midphi0__blk1374_dn8 = assign51400_e65936_d_n8;

        let (assign51410_e65950, assign51410_e65950_d_n5, assign51410_e65950_d_n6, assign51410_e65950_d_n7, assign51410_e65950_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51410_e65947: f64 = (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374);
        let assign51410_e65948: f64 = (locals.var_qis__blk1359 - assign51410_e65947);
        (assign51410_e65948, (locals.var_qis__blk1359_dn5 - ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn5))), (locals.var_qis__blk1359_dn6 - ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn6))), (locals.var_qis__blk1359_dn7 - ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn7))), (locals.var_qis__blk1359_dn8 - ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn8))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51410_e65950;
        locals.var_temp__blk936_dn5 = assign51410_e65950_d_n5;
        locals.var_temp__blk936_dn6 = assign51410_e65950_d_n6;
        locals.var_temp__blk936_dn7 = assign51410_e65950_d_n7;
        locals.var_temp__blk936_dn8 = assign51410_e65950_d_n8;

        let (assign51420_e65969, assign51420_e65969_d_n5, assign51420_e65969_d_n6, assign51420_e65969_d_n7, assign51420_e65969_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51420_e65962: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign51420_e65964: f64 = (assign51420_e65962 + 1e-12);
        let assign51420_e65965: f64 = (assign51420_e65964).sqrt();
        let assign51420_e65966: f64 = (locals.var_temp__blk936 + assign51420_e65965);
        let assign51420_e65967: f64 = (0.5 * assign51420_e65966);
        (assign51420_e65967, (0.5 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign51420_e65965)))),)
    } else {
        (locals.var_qisat__blk1375, locals.var_qisat__blk1375_dn5, locals.var_qisat__blk1375_dn6, locals.var_qisat__blk1375_dn7, locals.var_qisat__blk1375_dn8,)
    }
};
        locals.var_qisat__blk1375 = assign51420_e65969;
        locals.var_qisat__blk1375_dn5 = assign51420_e65969_d_n5;
        locals.var_qisat__blk1375_dn6 = assign51420_e65969_d_n6;
        locals.var_qisat__blk1375_dn7 = assign51420_e65969_d_n7;
        locals.var_qisat__blk1375_dn8 = assign51420_e65969_d_n8;

        let (assign51430_e65989, assign51430_e65989_d_n5, assign51430_e65989_d_n6, assign51430_e65989_d_n7, assign51430_e65989_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51430_e65979: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
        let assign51430_e65981: f64 = (assign51430_e65979 - locals.var_qis__blk1359);
        let assign51430_e65984: f64 = (locals.var_alphas__blk1356 - 1.0);
        let assign51430_e65986: f64 = (assign51430_e65984 * locals.var_midphi0__blk1374);
        let assign51430_e65987: f64 = (assign51430_e65981 + assign51430_e65986);
        (assign51430_e65987, ((((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)) - locals.var_qis__blk1359_dn5) + ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn5))), ((((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)) - locals.var_qis__blk1359_dn6) + ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn6))), ((((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)) - locals.var_qis__blk1359_dn7) + ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn7))), ((((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)) - locals.var_qis__blk1359_dn8) + ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn8))),)
    } else {
        (locals.var_qbsat__blk1376, locals.var_qbsat__blk1376_dn5, locals.var_qbsat__blk1376_dn6, locals.var_qbsat__blk1376_dn7, locals.var_qbsat__blk1376_dn8,)
    }
};
        locals.var_qbsat__blk1376 = assign51430_e65989;
        locals.var_qbsat__blk1376_dn5 = assign51430_e65989_d_n5;
        locals.var_qbsat__blk1376_dn6 = assign51430_e65989_d_n6;
        locals.var_qbsat__blk1376_dn7 = assign51430_e65989_d_n7;
        locals.var_qbsat__blk1376_dn8 = assign51430_e65989_d_n8;

        let (assign51440_e66007, assign51440_e66007_d_n5, assign51440_e66007_d_n6, assign51440_e66007_d_n7, assign51440_e66007_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51440_e66000: f64 = (0.5 * locals.var_gf2__blk1308);
        let assign51440_e66002: f64 = (assign51440_e66000 * locals.var_phit1__blk1322);
        let assign51440_e66004: f64 = (assign51440_e66002 / locals.var_qbsat__blk1376);
        let assign51440_e66005: f64 = (1.0 + assign51440_e66004);
        (assign51440_e66005, ((((((0.5 * locals.var_gf2__blk1308_dn5) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn5)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn6) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn6)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn7) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn7)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn8) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn8)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)),)
    } else {
        (locals.var_alphasat__blk1377, locals.var_alphasat__blk1377_dn5, locals.var_alphasat__blk1377_dn6, locals.var_alphasat__blk1377_dn7, locals.var_alphasat__blk1377_dn8,)
    }
};
        locals.var_alphasat__blk1377 = assign51440_e66007;
        locals.var_alphasat__blk1377_dn5 = assign51440_e66007_d_n5;
        locals.var_alphasat__blk1377_dn6 = assign51440_e66007_d_n6;
        locals.var_alphasat__blk1377_dn7 = assign51440_e66007_d_n7;
        locals.var_alphasat__blk1377_dn8 = assign51440_e66007_d_n8;

        let (assign51450_e66021, assign51450_e66021_d_n5, assign51450_e66021_d_n6, assign51450_e66021_d_n7, assign51450_e66021_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51450_e66018: f64 = (locals.var_eta_mu * locals.var_qisat__blk1375);
        let assign51450_e66019: f64 = (locals.var_qbsat__blk1376 + assign51450_e66018);
        (assign51450_e66019, (locals.var_qbsat__blk1376_dn5 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn5)), (locals.var_qbsat__blk1376_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn6)), (locals.var_qbsat__blk1376_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn7)), (locals.var_qbsat__blk1376_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51450_e66021;
        locals.var_temp__blk936_dn5 = assign51450_e66021_d_n5;
        locals.var_temp__blk936_dn6 = assign51450_e66021_d_n6;
        locals.var_temp__blk936_dn7 = assign51450_e66021_d_n7;
        locals.var_temp__blk936_dn8 = assign51450_e66021_d_n8;

        let (assign51460_e66037, assign51460_e66037_d_n5, assign51460_e66037_d_n6, assign51460_e66037_d_n7, assign51460_e66037_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51460_e66031: f64 = (locals.var_e_eff0 * locals.var_temp__blk936);
        let assign51460_e66033: f64 = (assign51460_e66031 * locals.var_mue_t);
        let assign51460_e66035: f64 = (assign51460_e66033).powf(locals.var_themu_t);
        (assign51460_e66035, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t) / assign51460_e66033))) },)
    } else {
        (locals.var_gmobmusat__blk1378, locals.var_gmobmusat__blk1378_dn5, locals.var_gmobmusat__blk1378_dn6, locals.var_gmobmusat__blk1378_dn7, locals.var_gmobmusat__blk1378_dn8,)
    }
};
        locals.var_gmobmusat__blk1378 = assign51460_e66037;
        locals.var_gmobmusat__blk1378_dn5 = assign51460_e66037_d_n5;
        locals.var_gmobmusat__blk1378_dn6 = assign51460_e66037_d_n6;
        locals.var_gmobmusat__blk1378_dn7 = assign51460_e66037_d_n7;
        locals.var_gmobmusat__blk1378_dn8 = assign51460_e66037_d_n8;

        let (assign51470_e66059, assign51470_e66059_d_n5, assign51470_e66059_d_n6, assign51470_e66059_d_n7, assign51470_e66059_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51470_e66049: f64 = (1.0 - locals.var_eta_mu);
        let assign51470_e66050: f64 = (locals.var_alphasat__blk1377 * assign51470_e66049);
        let assign51470_e66052: f64 = (assign51470_e66050 - 1.0);
        let assign51470_e66053: f64 = (locals.var_themu_t * assign51470_e66052);
        let assign51470_e66055: f64 = (assign51470_e66053 / locals.var_temp__blk936);
        let assign51470_e66057: f64 = (assign51470_e66055 * locals.var_gmobmusat__blk1378);
        (assign51470_e66057, ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn5 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn5)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn6 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn7 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn8 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn8)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51470_e66059;
        locals.var_temp1_dn5 = assign51470_e66059_d_n5;
        locals.var_temp1_dn6 = assign51470_e66059_d_n6;
        locals.var_temp1_dn7 = assign51470_e66059_d_n7;
        locals.var_temp1_dn8 = assign51470_e66059_d_n8;

        let (assign51480_e66071, assign51480_e66071_d_n5, assign51480_e66071_d_n6, assign51480_e66071_d_n7, assign51480_e66071_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51480_e66069: f64 = (locals.var_qisat__blk1375 / locals.var_qbsat__blk1376);
        (assign51480_e66069, (((locals.var_qisat__blk1375_dn5 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn6 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn7 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn8 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51480_e66071;
        locals.var_temp__blk936_dn5 = assign51480_e66071_d_n5;
        locals.var_temp__blk936_dn6 = assign51480_e66071_d_n6;
        locals.var_temp__blk936_dn7 = assign51480_e66071_d_n7;
        locals.var_temp__blk936_dn8 = assign51480_e66071_d_n8;

        let (assign51490_e66088, assign51490_e66088_d_n5, assign51490_e66088_d_n6, assign51490_e66088_d_n7, assign51490_e66088_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51490_e66082: f64 = (1.0 + locals.var_temp__blk936);
        let assign51490_e66084: f64 = (-locals.var_thecs_t);
        let assign51490_e66085: f64 = (assign51490_e66082).powf(assign51490_e66084);
        let assign51490_e66086: f64 = (locals.var_cs_t * assign51490_e66085);
        (assign51490_e66086, (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn5)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn5 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn6)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn6 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn7)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn7 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn8)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn8 / assign51490_e66082))) }),)
    } else {
        (locals.var_gmobcssat__blk1379, locals.var_gmobcssat__blk1379_dn5, locals.var_gmobcssat__blk1379_dn6, locals.var_gmobcssat__blk1379_dn7, locals.var_gmobcssat__blk1379_dn8,)
    }
};
        locals.var_gmobcssat__blk1379 = assign51490_e66088;
        locals.var_gmobcssat__blk1379_dn5 = assign51490_e66088_d_n5;
        locals.var_gmobcssat__blk1379_dn6 = assign51490_e66088_d_n6;
        locals.var_gmobcssat__blk1379_dn7 = assign51490_e66088_d_n7;
        locals.var_gmobcssat__blk1379_dn8 = assign51490_e66088_d_n8;

        let (assign51500_e66112, assign51500_e66112_d_n5, assign51500_e66112_d_n6, assign51500_e66112_d_n7, assign51500_e66112_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51500_e66099: f64 = (locals.var_alphasat__blk1377 - 1.0);
        let assign51500_e66103: f64 = (locals.var_temp__blk936 + 1.0);
        let assign51500_e66104: f64 = (1.0 / assign51500_e66103);
        let assign51500_e66105: f64 = (assign51500_e66099 + assign51500_e66104);
        let assign51500_e66106: f64 = (locals.var_thecs_t * assign51500_e66105);
        let assign51500_e66108: f64 = (assign51500_e66106 / locals.var_qbsat__blk1376);
        let assign51500_e66110: f64 = (assign51500_e66108 * locals.var_gmobcssat__blk1379);
        (assign51500_e66110, ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn5 + (-(locals.var_temp__blk936_dn5 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn5)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn6 + (-(locals.var_temp__blk936_dn6 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn7 + (-(locals.var_temp__blk936_dn7 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn8 + (-(locals.var_temp__blk936_dn8 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51500_e66112;
        locals.var_temp2_dn5 = assign51500_e66112_d_n5;
        locals.var_temp2_dn6 = assign51500_e66112_d_n6;
        locals.var_temp2_dn7 = assign51500_e66112_d_n7;
        locals.var_temp2_dn8 = assign51500_e66112_d_n8;

        let (assign51510_e66128, assign51510_e66128_d_n5, assign51510_e66128_d_n6, assign51510_e66128_d_n7, assign51510_e66128_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51510_e66122: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign51510_e66124: f64 = (assign51510_e66122 * locals.var_rhog__blk1362);
        let assign51510_e66126: f64 = (assign51510_e66124 * locals.var_qisat__blk1375);
        (assign51510_e66126, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn5)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn6)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn7)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn8)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn8)),)
    } else {
        (locals.var_grsat__blk1380, locals.var_grsat__blk1380_dn5, locals.var_grsat__blk1380_dn6, locals.var_grsat__blk1380_dn7, locals.var_grsat__blk1380_dn8,)
    }
};
        locals.var_grsat__blk1380 = assign51510_e66128;
        locals.var_grsat__blk1380_dn5 = assign51510_e66128_d_n5;
        locals.var_grsat__blk1380_dn6 = assign51510_e66128_d_n6;
        locals.var_grsat__blk1380_dn7 = assign51510_e66128_d_n7;
        locals.var_grsat__blk1380_dn8 = assign51510_e66128_d_n8;

        let (assign51520_e66150, assign51520_e66150_d_n5, assign51520_e66150_d_n6, assign51520_e66150_d_n7, assign51520_e66150_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51520_e66140: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign51520_e66142: f64 = (assign51520_e66140 * locals.var_rhog__blk1362);
        let assign51520_e66144: f64 = (assign51520_e66142 * locals.var_alphasat__blk1377);
        let assign51520_e66145: f64 = (locals.var_temp1 - assign51520_e66144);
        let assign51520_e66147: f64 = (assign51520_e66145 / locals.var_temp2);
        let assign51520_e66148: f64 = (1.0 + assign51520_e66147);
        (assign51520_e66148, ((((locals.var_temp1_dn5 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn5)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn5))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn5)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn6)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn6))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn7)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn7))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn8)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn8))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51520_e66150;
        locals.var_temp__blk936_dn5 = assign51520_e66150_d_n5;
        locals.var_temp__blk936_dn6 = assign51520_e66150_d_n6;
        locals.var_temp__blk936_dn7 = assign51520_e66150_d_n7;
        locals.var_temp__blk936_dn8 = assign51520_e66150_d_n8;

        let assign51530_e66153: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign51530_e66153;

        let (assign51540_e66173, assign51540_e66173_d_n5, assign51540_e66173_d_n6, assign51540_e66173_d_n7, assign51540_e66173_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign51540_e66167: f64 = (2.0 * locals.var_temp__blk936);
        let assign51540_e66168: f64 = (assign51540_e66167).exp();
        let assign51540_e66169: f64 = (1.0 + assign51540_e66168);
        let assign51540_e66170: f64 = (assign51540_e66169).ln();
        let assign51540_e66171: f64 = (0.5 * assign51540_e66170);
        (assign51540_e66171, (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn5)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn6)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn7)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn8)) / assign51540_e66169)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51540_e66173;
        locals.var_temp1_dn5 = assign51540_e66173_d_n5;
        locals.var_temp1_dn6 = assign51540_e66173_d_n6;
        locals.var_temp1_dn7 = assign51540_e66173_d_n7;
        locals.var_temp1_dn8 = assign51540_e66173_d_n8;

        let (assign51550_e66186, assign51550_e66186_d_n5, assign51550_e66186_d_n6, assign51550_e66186_d_n7, assign51550_e66186_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51550_e66186;
        locals.var_temp1_dn5 = assign51550_e66186_d_n5;
        locals.var_temp1_dn6 = assign51550_e66186_d_n6;
        locals.var_temp1_dn7 = assign51550_e66186_d_n7;
        locals.var_temp1_dn8 = assign51550_e66186_d_n8;

        let (assign51560_e66209, assign51560_e66209_d_n5, assign51560_e66209_d_n6, assign51560_e66209_d_n7, assign51560_e66209_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51560_e66195: f64 = (-locals.var_midphi0__blk1374);
        let assign51560_e66197: f64 = (assign51560_e66195 * locals.var_temp2);
        let assign51560_e66199: f64 = (assign51560_e66197 * locals.var_temp1);
        let assign51560_e66202: f64 = (1.0 + locals.var_gmobmusat__blk1378);
        let assign51560_e66204: f64 = (assign51560_e66202 + locals.var_gmobcssat__blk1379);
        let assign51560_e66206: f64 = (assign51560_e66204 + locals.var_grsat__blk1380);
        let assign51560_e66207: f64 = (assign51560_e66199 / assign51560_e66206);
        (assign51560_e66207, ((((((((-locals.var_midphi0__blk1374_dn5) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn5)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn5)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn5 + locals.var_gmobcssat__blk1379_dn5) + locals.var_grsat__blk1380_dn5))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn6) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn6)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn6 + locals.var_gmobcssat__blk1379_dn6) + locals.var_grsat__blk1380_dn6))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn7) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn7)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn7 + locals.var_gmobcssat__blk1379_dn7) + locals.var_grsat__blk1380_dn7))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn8) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn8)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn8 + locals.var_gmobcssat__blk1379_dn8) + locals.var_grsat__blk1380_dn8))) / (assign51560_e66206 * assign51560_e66206)),)
    } else {
        (locals.var_delta_gmob__blk1381, locals.var_delta_gmob__blk1381_dn5, locals.var_delta_gmob__blk1381_dn6, locals.var_delta_gmob__blk1381_dn7, locals.var_delta_gmob__blk1381_dn8,)
    }
};
        locals.var_delta_gmob__blk1381 = assign51560_e66209;
        locals.var_delta_gmob__blk1381_dn5 = assign51560_e66209_d_n5;
        locals.var_delta_gmob__blk1381_dn6 = assign51560_e66209_d_n6;
        locals.var_delta_gmob__blk1381_dn7 = assign51560_e66209_d_n7;
        locals.var_delta_gmob__blk1381_dn8 = assign51560_e66209_d_n8;

        let (assign51570_e66232, assign51570_e66232_d_n5, assign51570_e66232_d_n6, assign51570_e66232_d_n7, assign51570_e66232_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign51570_e66224: f64 = (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381);
        let assign51570_e66225: f64 = (1.0 + assign51570_e66224);
        let assign51570_e66226: f64 = (assign51570_e66225).sqrt();
        let assign51570_e66227: f64 = (1.0 + assign51570_e66226);
        let assign51570_e66228: f64 = (locals.var_delta_gmob__blk1381 / assign51570_e66227);
        let assign51570_e66229: f64 = (1.0 + assign51570_e66228);
        let assign51570_e66230: f64 = (locals.var_x_inf0__blk1373 * assign51570_e66229);
        (assign51570_e66230, ((locals.var_x_inf0__blk1373_dn5 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn5 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn5 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn5)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn6 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn6 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn6 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn6)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn7 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn7 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn7 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn7)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn8 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn8 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn8 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn8)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))),)
    } else {
        (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8,)
    }
};
        locals.var_x_inf__blk1382 = assign51570_e66232;
        locals.var_x_inf__blk1382_dn5 = assign51570_e66232_d_n5;
        locals.var_x_inf__blk1382_dn6 = assign51570_e66232_d_n6;
        locals.var_x_inf__blk1382_dn7 = assign51570_e66232_d_n7;
        locals.var_x_inf__blk1382_dn8 = assign51570_e66232_d_n8;

        let (assign51580_e66243, assign51580_e66243_d_n5, assign51580_e66243_d_n6, assign51580_e66243_d_n7, assign51580_e66243_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 == 0.0)) {
        (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8,)
    } else {
        (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8,)
    }
};
        locals.var_x_inf__blk1382 = assign51580_e66243;
        locals.var_x_inf__blk1382_dn5 = assign51580_e66243_d_n5;
        locals.var_x_inf__blk1382_dn6 = assign51580_e66243_d_n6;
        locals.var_x_inf__blk1382_dn7 = assign51580_e66243_d_n7;
        locals.var_x_inf__blk1382_dn8 = assign51580_e66243_d_n8;

        let (assign51590_e66257, assign51590_e66257_d_n5, assign51590_e66257_d_n6, assign51590_e66257_d_n7, assign51590_e66257_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51590_e66251: f64 = (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371);
        let assign51590_e66253: f64 = (assign51590_e66251 * locals.var_x_inf__blk1382);
        let assign51590_e66255: f64 = (assign51590_e66253 * 0.7071067811865475);
        (assign51590_e66255, (((((locals.var_phit1__blk1322_dn5 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn5)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn5)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn6 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn6)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn7 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn7)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn8 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn8)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn8)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8,)
    }
};
        locals.var_ysat__blk1383 = assign51590_e66257;
        locals.var_ysat__blk1383_dn5 = assign51590_e66257_d_n5;
        locals.var_ysat__blk1383_dn6 = assign51590_e66257_d_n6;
        locals.var_ysat__blk1383_dn7 = assign51590_e66257_d_n7;
        locals.var_ysat__blk1383_dn8 = assign51590_e66257_d_n8;

        let assign51600_e66260: f64 = (-1.0);
        let assign51600_e66261: f64 = if locals.var_chnl_type == assign51600_e66260 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign51600_e66261;

        let (assign51610_e66276, assign51610_e66276_d_n5, assign51610_e66276_d_n6, assign51610_e66276_d_n7, assign51610_e66276_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign51610_e66272: f64 = (1.0 + locals.var_ysat__blk1383);
        let assign51610_e66273: f64 = (assign51610_e66272).sqrt();
        let assign51610_e66274: f64 = (locals.var_ysat__blk1383 / assign51610_e66273);
        (assign51610_e66274, (((locals.var_ysat__blk1383_dn5 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn5 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn6 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn6 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn7 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn7 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn8 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn8 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)),)
    } else {
        (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8,)
    }
};
        locals.var_ysat__blk1383 = assign51610_e66276;
        locals.var_ysat__blk1383_dn5 = assign51610_e66276_d_n5;
        locals.var_ysat__blk1383_dn6 = assign51610_e66276_d_n6;
        locals.var_ysat__blk1383_dn7 = assign51610_e66276_d_n7;
        locals.var_ysat__blk1383_dn8 = assign51610_e66276_d_n8;

        let (assign51620_e66293, assign51620_e66293_d_n5, assign51620_e66293_d_n6, assign51620_e66293_d_n7, assign51620_e66293_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51620_e66287: f64 = (4.0 * locals.var_ysat__blk1383);
        let assign51620_e66288: f64 = (1.0 + assign51620_e66287);
        let assign51620_e66289: f64 = (assign51620_e66288).sqrt();
        let assign51620_e66290: f64 = (1.0 + assign51620_e66289);
        let assign51620_e66291: f64 = (2.0 / assign51620_e66290);
        (assign51620_e66291, (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn5) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn6) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn7) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn8) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))),)
    } else {
        (locals.var_za__blk1384, locals.var_za__blk1384_dn5, locals.var_za__blk1384_dn6, locals.var_za__blk1384_dn7, locals.var_za__blk1384_dn8,)
    }
};
        locals.var_za__blk1384 = assign51620_e66293;
        locals.var_za__blk1384_dn5 = assign51620_e66293_d_n5;
        locals.var_za__blk1384_dn6 = assign51620_e66293_d_n6;
        locals.var_za__blk1384_dn7 = assign51620_e66293_d_n7;
        locals.var_za__blk1384_dn8 = assign51620_e66293_d_n8;

        let (assign51630_e66303, assign51630_e66303_d_n5, assign51630_e66303_d_n6, assign51630_e66303_d_n7, assign51630_e66303_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51630_e66301: f64 = (locals.var_za__blk1384 * locals.var_ysat__blk1383);
        (assign51630_e66301, ((locals.var_za__blk1384_dn5 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn5)), ((locals.var_za__blk1384_dn6 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn6)), ((locals.var_za__blk1384_dn7 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn7)), ((locals.var_za__blk1384_dn8 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51630_e66303;
        locals.var_temp__blk936_dn5 = assign51630_e66303_d_n5;
        locals.var_temp__blk936_dn6 = assign51630_e66303_d_n6;
        locals.var_temp__blk936_dn7 = assign51630_e66303_d_n7;
        locals.var_temp__blk936_dn8 = assign51630_e66303_d_n8;

        let (assign51640_e66335, assign51640_e66335_d_n5, assign51640_e66335_d_n6, assign51640_e66335_d_n7, assign51640_e66335_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51640_e66311: f64 = (locals.var_x_inf__blk1382 * locals.var_za__blk1384);
        let assign51640_e66315: f64 = (0.86 * locals.var_temp__blk936);
        let assign51640_e66319: f64 = (locals.var_temp__blk936 * locals.var_za__blk1384);
        let assign51640_e66320: f64 = (1.0 - assign51640_e66319);
        let assign51640_e66321: f64 = (assign51640_e66315 * assign51640_e66320);
        let assign51640_e66325: f64 = (4.0 * locals.var_temp__blk936);
        let assign51640_e66327: f64 = (assign51640_e66325 * locals.var_temp__blk936);
        let assign51640_e66329: f64 = (assign51640_e66327 * locals.var_za__blk1384);
        let assign51640_e66330: f64 = (1.0 + assign51640_e66329);
        let assign51640_e66331: f64 = (assign51640_e66321 / assign51640_e66330);
        let assign51640_e66332: f64 = (1.0 + assign51640_e66331);
        let assign51640_e66333: f64 = (assign51640_e66311 * assign51640_e66332);
        (assign51640_e66333, ((((locals.var_x_inf__blk1382_dn5 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn5)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn5) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn5 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn5))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn5)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn5)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn6 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn6)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn6) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn6 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn6))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn6)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn6)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn7 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn7)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn7) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn7 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn7))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn7)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn7)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn8 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn8)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn8) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn8 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn8))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn8)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn8)))) / (assign51640_e66330 * assign51640_e66330)))),)
    } else {
        (locals.var_x_0__blk1385, locals.var_x_0__blk1385_dn5, locals.var_x_0__blk1385_dn6, locals.var_x_0__blk1385_dn7, locals.var_x_0__blk1385_dn8,)
    }
};
        locals.var_x_0__blk1385 = assign51640_e66335;
        locals.var_x_0__blk1385_dn5 = assign51640_e66335_d_n5;
        locals.var_x_0__blk1385_dn6 = assign51640_e66335_d_n6;
        locals.var_x_0__blk1385_dn7 = assign51640_e66335_d_n7;
        locals.var_x_0__blk1385_dn8 = assign51640_e66335_d_n8;

        let (assign51650_e66345, assign51650_e66345_d_n5, assign51650_e66345_d_n6, assign51650_e66345_d_n7, assign51650_e66345_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51650_e66343: f64 = (0.99 * locals.var_x_0__blk1385);
        (assign51650_e66343, (0.99 * locals.var_x_0__blk1385_dn5), (0.99 * locals.var_x_0__blk1385_dn6), (0.99 * locals.var_x_0__blk1385_dn7), (0.99 * locals.var_x_0__blk1385_dn8),)
    } else {
        (locals.var_x_sat__blk1386, locals.var_x_sat__blk1386_dn5, locals.var_x_sat__blk1386_dn6, locals.var_x_sat__blk1386_dn7, locals.var_x_sat__blk1386_dn8,)
    }
};
        locals.var_x_sat__blk1386 = assign51650_e66345;
        locals.var_x_sat__blk1386_dn5 = assign51650_e66345_d_n5;
        locals.var_x_sat__blk1386_dn6 = assign51650_e66345_d_n6;
        locals.var_x_sat__blk1386_dn7 = assign51650_e66345_d_n7;
        locals.var_x_sat__blk1386_dn8 = assign51650_e66345_d_n8;

        let (assign51660_e66363, assign51660_e66363_d_n5, assign51660_e66363_d_n6, assign51660_e66363_d_n7, assign51660_e66363_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51660_e66355: f64 = (2.0 * locals.var_asat__blk1372);
        let assign51660_e66356: f64 = (locals.var_x_sat__blk1386 - assign51660_e66355);
        let assign51660_e66357: f64 = (locals.var_x_sat__blk1386 * assign51660_e66356);
        let assign51660_e66359: f64 = (assign51660_e66357 * locals.var_inv_gf2__blk1324);
        let assign51660_e66361: f64 = (assign51660_e66359 / locals.var_ds__blk1353);
        (assign51660_e66361, (((((((locals.var_x_sat__blk1386_dn5 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn5 - (2.0 * locals.var_asat__blk1372_dn5)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn5)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn5)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn6 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn6 - (2.0 * locals.var_asat__blk1372_dn6)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn6)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn6)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn7 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn7 - (2.0 * locals.var_asat__blk1372_dn7)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn7)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn7)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn8 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn8 - (2.0 * locals.var_asat__blk1372_dn8)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn8)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn8)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51660_e66363;
        locals.var_temp__blk936_dn5 = assign51660_e66363_d_n5;
        locals.var_temp__blk936_dn6 = assign51660_e66363_d_n6;
        locals.var_temp__blk936_dn7 = assign51660_e66363_d_n7;
        locals.var_temp__blk936_dn8 = assign51660_e66363_d_n8;

        let (assign51670_e66385, assign51670_e66385_d_n5, assign51670_e66385_d_n6, assign51670_e66385_d_n7, assign51670_e66385_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign51670_e66374: f64 = (-0.99);
        let (assign51670_e66379, assign51670_e66379_d_n5, assign51670_e66379_d_n6, assign51670_e66379_d_n7, assign51670_e66379_d_n8,) = {
            if (locals.var_temp__blk936 > assign51670_e66374) {
                (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
            } else {
                let assign51670_e66378: f64 = (-0.99);
                (assign51670_e66378, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign51670_e66380: f64 = (1.0 + assign51670_e66379);
        let assign51670_e66381: f64 = (assign51670_e66380).ln();
        let assign51670_e66382: f64 = (locals.var_x_sat__blk1386 - assign51670_e66381);
        let assign51670_e66383: f64 = (locals.var_phit1__blk1322 * assign51670_e66382);
        (assign51670_e66383, ((locals.var_phit1__blk1322_dn5 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn5 - (assign51670_e66379_d_n5 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn6 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn6 - (assign51670_e66379_d_n6 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn7 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn7 - (assign51670_e66379_d_n7 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn8 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn8 - (assign51670_e66379_d_n8 / assign51670_e66380)))),)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51670_e66385;
        locals.var_v_dsat__blk1387_dn5 = assign51670_e66385_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51670_e66385_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51670_e66385_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51670_e66385_d_n8;

        let (assign51680_e66394, assign51680_e66394_d_n5, assign51680_e66394_d_n6, assign51680_e66394_d_n7, assign51680_e66394_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 == 0.0)) {
        (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8,)
    } else {
        (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8,)
    }
};
        locals.var_v_dsat__blk1387 = assign51680_e66394;
        locals.var_v_dsat__blk1387_dn5 = assign51680_e66394_d_n5;
        locals.var_v_dsat__blk1387_dn6 = assign51680_e66394_d_n6;
        locals.var_v_dsat__blk1387_dn7 = assign51680_e66394_d_n7;
        locals.var_v_dsat__blk1387_dn8 = assign51680_e66394_d_n8;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign51690_e66402, assign51690_e66402_d_n5, assign51690_e66402_d_n6, assign51690_e66402_d_n7, assign51690_e66402_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51690_e66400: f64 = (1.0 + locals.var_arloc__blk1303);
        (assign51690_e66400, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51690_e66402;
        locals.var_temp__blk936_dn5 = assign51690_e66402_d_n5;
        locals.var_temp__blk936_dn6 = assign51690_e66402_d_n6;
        locals.var_temp__blk936_dn7 = assign51690_e66402_d_n7;
        locals.var_temp__blk936_dn8 = assign51690_e66402_d_n8;

        let (assign51700_e66413, assign51700_e66413_d_n5, assign51700_e66413_d_n6, assign51700_e66413_d_n7, assign51700_e66413_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51700_e66407: f64 = (locals.var_temp__blk936).sqrt();
        let assign51700_e66409: f64 = (assign51700_e66407 * locals.var_v_ds);
        let assign51700_e66411: f64 = (assign51700_e66409 / locals.var_v_dsat__blk1387);
        (assign51700_e66411, (((((locals.var_temp__blk936_dn5 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn5)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn6)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn6)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn7)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn8)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign51700_e66413;
        locals.var_temp1_dn5 = assign51700_e66413_d_n5;
        locals.var_temp1_dn6 = assign51700_e66413_d_n6;
        locals.var_temp1_dn7 = assign51700_e66413_d_n7;
        locals.var_temp1_dn8 = assign51700_e66413_d_n8;

        let (assign51710_e66423, assign51710_e66423_d_n5, assign51710_e66423_d_n6, assign51710_e66423_d_n7, assign51710_e66423_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51710_e66419: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign51710_e66421: f64 = (assign51710_e66419 + locals.var_temp__blk936);
        (assign51710_e66421, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign51710_e66423;
        locals.var_temp2_dn5 = assign51710_e66423_d_n5;
        locals.var_temp2_dn6 = assign51710_e66423_d_n6;
        locals.var_temp2_dn7 = assign51710_e66423_d_n7;
        locals.var_temp2_dn8 = assign51710_e66423_d_n8;

        let (assign51720_e66431, assign51720_e66431_d_n5, assign51720_e66431_d_n6, assign51720_e66431_d_n7, assign51720_e66431_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51720_e66429: f64 = (2.0 * locals.var_temp1);
        (assign51720_e66429, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign51720_e66431;
        locals.var_temp__blk936_dn5 = assign51720_e66431_d_n5;
        locals.var_temp__blk936_dn6 = assign51720_e66431_d_n6;
        locals.var_temp__blk936_dn7 = assign51720_e66431_d_n7;
        locals.var_temp__blk936_dn8 = assign51720_e66431_d_n8;

        let (assign51730_e66449, assign51730_e66449_d_n5, assign51730_e66449_d_n6, assign51730_e66449_d_n7, assign51730_e66449_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51730_e66437: f64 = (locals.var_v_dsat__blk1387 * locals.var_temp__blk936);
        let assign51730_e66440: f64 = (locals.var_temp2 - locals.var_temp__blk936);
        let assign51730_e66441: f64 = (assign51730_e66440).sqrt();
        let assign51730_e66444: f64 = (locals.var_temp2 + locals.var_temp__blk936);
        let assign51730_e66445: f64 = (assign51730_e66444).sqrt();
        let assign51730_e66446: f64 = (assign51730_e66441 + assign51730_e66445);
        let assign51730_e66447: f64 = (assign51730_e66437 / assign51730_e66446);
        (assign51730_e66447, (((((locals.var_v_dsat__blk1387_dn5 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn5)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn6 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn6)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn7 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn7)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn8 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn8)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)),)
    } else {
        (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8,)
    }
};
        locals.var_vdse__blk1388 = assign51730_e66449;
        locals.var_vdse__blk1388_dn5 = assign51730_e66449_d_n5;
        locals.var_vdse__blk1388_dn6 = assign51730_e66449_d_n6;
        locals.var_vdse__blk1388_dn7 = assign51730_e66449_d_n7;
        locals.var_vdse__blk1388_dn8 = assign51730_e66449_d_n8;

        let (assign51740_e66457, assign51740_e66457_d_n5, assign51740_e66457_d_n6, assign51740_e66457_d_n7, assign51740_e66457_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51740_e66455: f64 = (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323);
        (assign51740_e66455, ((locals.var_vdse__blk1388_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vdse__blk1388_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vdse__blk1388_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vdse__blk1388_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn8)),)
    } else {
        (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8,)
    }
};
        locals.var_udse__blk1389 = assign51740_e66457;
        locals.var_udse__blk1389_dn5 = assign51740_e66457_d_n5;
        locals.var_udse__blk1389_dn6 = assign51740_e66457_d_n6;
        locals.var_udse__blk1389_dn7 = assign51740_e66457_d_n7;
        locals.var_udse__blk1389_dn8 = assign51740_e66457_d_n8;

        let (assign51750_e66465, assign51750_e66465_d_n5, assign51750_e66465_d_n6, assign51750_e66465_d_n7, assign51750_e66465_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51750_e66463: f64 = (locals.var_xn_s__blk1332 + locals.var_udse__blk1389);
        (assign51750_e66463, (locals.var_xn_s__blk1332_dn5 + locals.var_udse__blk1389_dn5), (locals.var_xn_s__blk1332_dn6 + locals.var_udse__blk1389_dn6), (locals.var_xn_s__blk1332_dn7 + locals.var_udse__blk1389_dn7), (locals.var_xn_s__blk1332_dn8 + locals.var_udse__blk1389_dn8),)
    } else {
        (locals.var_xn_d__blk1390, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8,)
    }
};
        locals.var_xn_d__blk1390 = assign51750_e66465;
        locals.var_xn_d__blk1390_dn5 = assign51750_e66465_d_n5;
        locals.var_xn_d__blk1390_dn6 = assign51750_e66465_d_n6;
        locals.var_xn_d__blk1390_dn7 = assign51750_e66465_d_n7;
        locals.var_xn_d__blk1390_dn8 = assign51750_e66465_d_n8;

        let assign51760_e66468: f64 = if locals.var_udse__blk1389 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign51760_e66468;

        let (assign51770_e66478, assign51770_e66478_d_n5, assign51770_e66478_d_n6, assign51770_e66478_d_n7, assign51770_e66478_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign51770_e66475: f64 = (-locals.var_udse__blk1389);
        let assign51770_e66476: f64 = (assign51770_e66475).exp();
        (assign51770_e66476, (assign51770_e66476 * (-locals.var_udse__blk1389_dn5)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn6)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn7)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn8)),)
    } else {
        (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8,)
    }
};
        locals.var_k_ds__blk1391 = assign51770_e66478;
        locals.var_k_ds__blk1391_dn5 = assign51770_e66478_d_n5;
        locals.var_k_ds__blk1391_dn6 = assign51770_e66478_d_n6;
        locals.var_k_ds__blk1391_dn7 = assign51770_e66478_d_n7;
        locals.var_k_ds__blk1391_dn8 = assign51770_e66478_d_n8;

        let (assign51780_e66509, assign51780_e66509_d_n5, assign51780_e66509_d_n6, assign51780_e66509_d_n7, assign51780_e66509_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 == 0.0)) {
        let assign51780_e66489: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66494: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66498: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
        let assign51780_e66500: f64 = (assign51780_e66498 * 0.3333333333333333);
        let assign51780_e66501: f64 = (1.0 + assign51780_e66500);
        let assign51780_e66502: f64 = (assign51780_e66494 * assign51780_e66501);
        let assign51780_e66503: f64 = (0.5 * assign51780_e66502);
        let assign51780_e66504: f64 = (1.0 + assign51780_e66503);
        let assign51780_e66505: f64 = (assign51780_e66489 * assign51780_e66504);
        let assign51780_e66506: f64 = (1.0 + assign51780_e66505);
        let assign51780_e66507: f64 = (1e-200 / assign51780_e66506);
        (assign51780_e66507, (-((1e-200 * ((locals.var_udse__blk1389_dn5 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn5 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn5 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn6 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn6 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn6 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn7 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn7 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn7 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn8 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn8 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn8 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))),)
    } else {
        (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8,)
    }
};
        locals.var_k_ds__blk1391 = assign51780_e66509;
        locals.var_k_ds__blk1391_dn5 = assign51780_e66509_d_n5;
        locals.var_k_ds__blk1391_dn6 = assign51780_e66509_d_n6;
        locals.var_k_ds__blk1391_dn7 = assign51780_e66509_d_n7;
        locals.var_k_ds__blk1391_dn8 = assign51780_e66509_d_n8;

        let (assign51790_e66517, assign51790_e66517_d_n5, assign51790_e66517_d_n6, assign51790_e66517_d_n7, assign51790_e66517_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign51790_e66515: f64 = (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391);
        (assign51790_e66515, ((locals.var_delta_ns__blk1347_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn8)),)
    } else {
        (locals.var_delta_nd__blk1392, locals.var_delta_nd__blk1392_dn5, locals.var_delta_nd__blk1392_dn6, locals.var_delta_nd__blk1392_dn7, locals.var_delta_nd__blk1392_dn8,)
    }
};
        locals.var_delta_nd__blk1392 = assign51790_e66517;
        locals.var_delta_nd__blk1392_dn5 = assign51790_e66517_d_n5;
        locals.var_delta_nd__blk1392_dn6 = assign51790_e66517_d_n6;
        locals.var_delta_nd__blk1392_dn7 = assign51790_e66517_d_n7;
        locals.var_delta_nd__blk1392_dn8 = assign51790_e66517_d_n8;

        let assign51800_e66519: f64 = (locals.var_xg__blk1326).abs();
        let assign51800_e66521: f64 = if assign51800_e66519 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign51800_e66521;

        let (assign51810_e66535, assign51810_e66535_d_n5, assign51810_e66535_d_n6, assign51810_e66535_d_n7, assign51810_e66535_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign51810_e66529: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
        let assign51810_e66531: f64 = (assign51810_e66529 * 0.16666666666666666);
        let assign51810_e66533: f64 = (assign51810_e66531 * 0.7071067811865475);
        (assign51810_e66533, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign51810_e66535;
        locals.var_sp_s_temp1__blk1432_dn5 = assign51810_e66535_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign51810_e66535_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign51810_e66535_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign51810_e66535_d_n8;

        let (assign51820_e66557, assign51820_e66557_d_n5, assign51820_e66557_d_n6, assign51820_e66557_d_n7, assign51820_e66557_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign51820_e66543: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
        let assign51820_e66548: f64 = (1.0 - locals.var_delta_nd__blk1392);
        let assign51820_e66549: f64 = (locals.var_xg__blk1326 * assign51820_e66548);
        let assign51820_e66551: f64 = (assign51820_e66549 * locals.var_gf__blk1307);
        let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_sp_s_temp1__blk1432);
        let assign51820_e66554: f64 = (1.0 + assign51820_e66553);
        let assign51820_e66555: f64 = (assign51820_e66543 * assign51820_e66554);
        (assign51820_e66555, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn5 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn5))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn6 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn6))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn7 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn7))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn8 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn8))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn8)))),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign51820_e66557;
        locals.var_x_d__blk1393_dn5 = assign51820_e66557_d_n5;
        locals.var_x_d__blk1393_dn6 = assign51820_e66557_d_n6;
        locals.var_x_d__blk1393_dn7 = assign51820_e66557_d_n7;
        locals.var_x_d__blk1393_dn8 = assign51820_e66557_d_n8;

        let (assign51830_e66568, assign51830_e66568_d_n5, assign51830_e66568_d_n6, assign51830_e66568_d_n7, assign51830_e66568_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51830_e66566: f64 = (locals.var_xn_d__blk1390 + 3.0);
        (assign51830_e66566, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8,)
    } else {
        (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8,)
    }
};
        locals.var_sp_s_bx__blk1453 = assign51830_e66568;
        locals.var_sp_s_bx__blk1453_dn5 = assign51830_e66568_d_n5;
        locals.var_sp_s_bx__blk1453_dn6 = assign51830_e66568_d_n6;
        locals.var_sp_s_bx__blk1453_dn7 = assign51830_e66568_d_n7;
        locals.var_sp_s_bx__blk1453_dn8 = assign51830_e66568_d_n8;

        let (assign51840_e66603, assign51840_e66603_d_n5, assign51840_e66603_d_n6, assign51840_e66603_d_n7, assign51840_e66603_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51840_e66578: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
        let assign51840_e66581: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign51840_e66584: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
        let assign51840_e66585: f64 = (assign51840_e66581 * assign51840_e66584);
        let assign51840_e66587: f64 = (assign51840_e66585 + 5.0);
        let assign51840_e66588: f64 = (assign51840_e66587).sqrt();
        let assign51840_e66589: f64 = (assign51840_e66578 - assign51840_e66588);
        let assign51840_e66590: f64 = (0.5 * assign51840_e66589);
        let assign51840_e66595: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
        let assign51840_e66597: f64 = (assign51840_e66595 + 5.0);
        let assign51840_e66598: f64 = (assign51840_e66597).sqrt();
        let assign51840_e66599: f64 = (locals.var_sp_s_bx__blk1453 - assign51840_e66598);
        let assign51840_e66600: f64 = (0.5 * assign51840_e66599);
        let assign51840_e66601: f64 = (assign51840_e66590 - assign51840_e66600);
        (assign51840_e66601, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign51840_e66598))))),)
    } else {
        (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8,)
    }
};
        locals.var_sp_s_eta__blk1436 = assign51840_e66603;
        locals.var_sp_s_eta__blk1436_dn5 = assign51840_e66603_d_n5;
        locals.var_sp_s_eta__blk1436_dn6 = assign51840_e66603_d_n6;
        locals.var_sp_s_eta__blk1436_dn7 = assign51840_e66603_d_n7;
        locals.var_sp_s_eta__blk1436_dn8 = assign51840_e66603_d_n8;

        let (assign51850_e66614, assign51850_e66614_d_n5, assign51850_e66614_d_n6, assign51850_e66614_d_n7, assign51850_e66614_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51850_e66612: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
        (assign51850_e66612, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign51850_e66614;
        locals.var_sp_s_temp__blk1431_dn5 = assign51850_e66614_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign51850_e66614_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign51850_e66614_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign51850_e66614_d_n8;

        let (assign51860_e66625, assign51860_e66625_d_n5, assign51860_e66625_d_n6, assign51860_e66625_d_n7, assign51860_e66625_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51860_e66622: f64 = (-locals.var_sp_s_eta__blk1436);
        let assign51860_e66623: f64 = (assign51860_e66622).exp();
        (assign51860_e66623, (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn8)),)
    } else {
        (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8,)
    }
};
        locals.var_sp_s_temp1__blk1432 = assign51860_e66625;
        locals.var_sp_s_temp1__blk1432_dn5 = assign51860_e66625_d_n5;
        locals.var_sp_s_temp1__blk1432_dn6 = assign51860_e66625_d_n6;
        locals.var_sp_s_temp1__blk1432_dn7 = assign51860_e66625_d_n7;
        locals.var_sp_s_temp1__blk1432_dn8 = assign51860_e66625_d_n8;

        let (assign51870_e66640, assign51870_e66640_d_n5, assign51870_e66640_d_n6, assign51870_e66640_d_n7, assign51870_e66640_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51870_e66636: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign51870_e66637: f64 = (2.0 + assign51870_e66636);
        let assign51870_e66638: f64 = (1.0 / assign51870_e66637);
        (assign51870_e66638, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign51870_e66637 * assign51870_e66637))),)
    } else {
        (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8,)
    }
};
        locals.var_sp_s_temp2__blk1433 = assign51870_e66640;
        locals.var_sp_s_temp2__blk1433_dn5 = assign51870_e66640_d_n5;
        locals.var_sp_s_temp2__blk1433_dn6 = assign51870_e66640_d_n6;
        locals.var_sp_s_temp2__blk1433_dn7 = assign51870_e66640_d_n7;
        locals.var_sp_s_temp2__blk1433_dn8 = assign51870_e66640_d_n8;

        let (assign51880_e66653, assign51880_e66653_d_n5, assign51880_e66653_d_n6, assign51880_e66653_d_n7, assign51880_e66653_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51880_e66649: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
        let assign51880_e66651: f64 = (assign51880_e66649 * locals.var_sp_s_temp2__blk1433);
        (assign51880_e66651, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign51880_e66653;
        locals.var_sp_s_xi0__blk1443_dn5 = assign51880_e66653_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign51880_e66653_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign51880_e66653_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign51880_e66653_d_n8;

        let (assign51890_e66668, assign51890_e66668_d_n5, assign51890_e66668_d_n6, assign51890_e66668_d_n7, assign51890_e66668_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51890_e66663: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
        let assign51890_e66665: f64 = (assign51890_e66663 * locals.var_sp_s_temp2__blk1433);
        let assign51890_e66666: f64 = (4.0 * assign51890_e66665);
        (assign51890_e66666, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign51890_e66668;
        locals.var_sp_s_xi1__blk1444_dn5 = assign51890_e66668_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign51890_e66668_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign51890_e66668_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign51890_e66668_d_n8;

        let (assign51900_e66687, assign51900_e66687_d_n5, assign51900_e66687_d_n6, assign51900_e66687_d_n7, assign51900_e66687_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51900_e66677: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
        let assign51900_e66680: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign51900_e66681: f64 = (assign51900_e66677 - assign51900_e66680);
        let assign51900_e66683: f64 = (assign51900_e66681 * locals.var_sp_s_temp2__blk1433);
        let assign51900_e66685: f64 = (assign51900_e66683 * locals.var_sp_s_temp2__blk1433);
        (assign51900_e66685, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign51900_e66687;
        locals.var_sp_s_xi2__blk1445_dn5 = assign51900_e66687_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign51900_e66687_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign51900_e66687_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign51900_e66687_d_n8;

        let (assign51910_e66737, assign51910_e66737_d_n5, assign51910_e66737_d_n6, assign51910_e66737_d_n7, assign51910_e66737_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51910_e66697: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign51910_e66701: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign51910_e66703: f64 = (assign51910_e66701 - 1.0);
        let assign51910_e66707: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign51910_e66709: f64 = (assign51910_e66707 + locals.var_sp_s_xi0__blk1443);
        let assign51910_e66710: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66709);
        let assign51910_e66711: f64 = (assign51910_e66703 - assign51910_e66710);
        let assign51910_e66712: f64 = (locals.var_gf2__blk1308 * assign51910_e66711);
        let assign51910_e66713: f64 = (assign51910_e66697 - assign51910_e66712);
        let (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8,) = {
            if (1e-40 > assign51910_e66713) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51910_e66718: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
                let assign51910_e66722: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
                let assign51910_e66724: f64 = (assign51910_e66722 - 1.0);
                let assign51910_e66728: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
                let assign51910_e66730: f64 = (assign51910_e66728 + locals.var_sp_s_xi0__blk1443);
                let assign51910_e66731: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66730);
                let assign51910_e66732: f64 = (assign51910_e66724 - assign51910_e66731);
                let assign51910_e66733: f64 = (locals.var_gf2__blk1308 * assign51910_e66732);
                let assign51910_e66734: f64 = (assign51910_e66718 - assign51910_e66733);
                (assign51910_e66734, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
            }
        };
        (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8,)
    } else {
        (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8,)
    }
};
        locals.var_sp_s_a__blk1437 = assign51910_e66737;
        locals.var_sp_s_a__blk1437_dn5 = assign51910_e66737_d_n5;
        locals.var_sp_s_a__blk1437_dn6 = assign51910_e66737_d_n6;
        locals.var_sp_s_a__blk1437_dn7 = assign51910_e66737_d_n7;
        locals.var_sp_s_a__blk1437_dn8 = assign51910_e66737_d_n8;

        let (assign51920_e66756, assign51920_e66756_d_n5, assign51920_e66756_d_n6, assign51920_e66756_d_n7, assign51920_e66756_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51920_e66750: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
        let assign51920_e66751: f64 = (locals.var_sp_s_temp1__blk1432 - assign51920_e66750);
        let assign51920_e66752: f64 = (locals.var_gf2__blk1308 * assign51920_e66751);
        let assign51920_e66753: f64 = (0.5 * assign51920_e66752);
        let assign51920_e66754: f64 = (1.0 - assign51920_e66753);
        (assign51920_e66754, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8))))))),)
    } else {
        (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8,)
    }
};
        locals.var_sp_s_b__blk1454 = assign51920_e66756;
        locals.var_sp_s_b__blk1454_dn5 = assign51920_e66756_d_n5;
        locals.var_sp_s_b__blk1454_dn6 = assign51920_e66756_d_n6;
        locals.var_sp_s_b__blk1454_dn7 = assign51920_e66756_d_n7;
        locals.var_sp_s_b__blk1454_dn8 = assign51920_e66756_d_n8;

        let (assign51930_e66779, assign51930_e66779_d_n5, assign51930_e66779_d_n6, assign51930_e66779_d_n7, assign51930_e66779_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51930_e66765: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign51930_e66769: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
        let assign51930_e66773: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign51930_e66774: f64 = (locals.var_delta_nd__blk1392 * assign51930_e66773);
        let assign51930_e66775: f64 = (assign51930_e66769 - assign51930_e66774);
        let assign51930_e66776: f64 = (locals.var_gf2__blk1308 * assign51930_e66775);
        let assign51930_e66777: f64 = (assign51930_e66765 + assign51930_e66776);
        (assign51930_e66777, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8,)
    }
};
        locals.var_sp_s_c__blk1438 = assign51930_e66779;
        locals.var_sp_s_c__blk1438_dn5 = assign51930_e66779_d_n5;
        locals.var_sp_s_c__blk1438_dn6 = assign51930_e66779_d_n6;
        locals.var_sp_s_c__blk1438_dn7 = assign51930_e66779_d_n7;
        locals.var_sp_s_c__blk1438_dn8 = assign51930_e66779_d_n8;

        let (assign51940_e66795, assign51940_e66795_d_n5, assign51940_e66795_d_n6, assign51940_e66795_d_n7, assign51940_e66795_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51940_e66788: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_eta__blk1436);
        let assign51940_e66791: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
        let assign51940_e66792: f64 = (assign51940_e66791).ln();
        let assign51940_e66793: f64 = (assign51940_e66788 + assign51940_e66792);
        (assign51940_e66793, ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)),)
    } else {
        (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8,)
    }
};
        locals.var_sp_s_tau__blk1439 = assign51940_e66795;
        locals.var_sp_s_tau__blk1439_dn5 = assign51940_e66795_d_n5;
        locals.var_sp_s_tau__blk1439_dn6 = assign51940_e66795_d_n6;
        locals.var_sp_s_tau__blk1439_dn7 = assign51940_e66795_d_n7;
        locals.var_sp_s_tau__blk1439_dn8 = assign51940_e66795_d_n8;

        let (assign51950_e66806, assign51950_e66806_d_n5, assign51950_e66806_d_n6, assign51950_e66806_d_n7, assign51950_e66806_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51950_e66804: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
        (assign51950_e66804, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8,)
    }
};
        locals.var_nu = assign51950_e66806;
        locals.var_nu_dn5 = assign51950_e66806_d_n5;
        locals.var_nu_dn6 = assign51950_e66806_d_n6;
        locals.var_nu_dn7 = assign51950_e66806_d_n7;
        locals.var_nu_dn8 = assign51950_e66806_d_n8;

        let (assign51960_e66829, assign51960_e66829_d_n5, assign51960_e66829_d_n6, assign51960_e66829_d_n7, assign51960_e66829_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51960_e66815: f64 = (locals.var_nu * locals.var_nu);
        let assign51960_e66820: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign51960_e66821: f64 = (0.5 * assign51960_e66820);
        let assign51960_e66824: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign51960_e66825: f64 = (assign51960_e66821 - assign51960_e66824);
        let assign51960_e66826: f64 = (locals.var_sp_s_tau__blk1439 * assign51960_e66825);
        let assign51960_e66827: f64 = (assign51960_e66815 + assign51960_e66826);
        (assign51960_e66827, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8,)
    }
};
        locals.var_mutau = assign51960_e66829;
        locals.var_mutau_dn5 = assign51960_e66829_d_n5;
        locals.var_mutau_dn6 = assign51960_e66829_d_n6;
        locals.var_mutau_dn7 = assign51960_e66829_d_n7;
        locals.var_mutau_dn8 = assign51960_e66829_d_n8;

        let (assign51970_e66866, assign51970_e66866_d_n5, assign51970_e66866_d_n6, assign51970_e66866_d_n7, assign51970_e66866_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign51970_e66839: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
        let assign51970_e66841: f64 = (assign51970_e66839 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66845: f64 = (locals.var_nu / locals.var_mutau);
        let assign51970_e66847: f64 = (assign51970_e66845 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66849: f64 = (assign51970_e66847 * locals.var_sp_s_tau__blk1439);
        let assign51970_e66851: f64 = (assign51970_e66849 * locals.var_sp_s_c__blk1438);
        let assign51970_e66854: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
        let assign51970_e66856: f64 = (assign51970_e66854 * 0.3333333333333333);
        let assign51970_e66859: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
        let assign51970_e66860: f64 = (assign51970_e66856 - assign51970_e66859);
        let assign51970_e66861: f64 = (assign51970_e66851 * assign51970_e66860);
        let assign51970_e66862: f64 = (locals.var_mutau + assign51970_e66861);
        let assign51970_e66863: f64 = (assign51970_e66841 / assign51970_e66862);
        let assign51970_e66864: f64 = (locals.var_sp_s_eta__blk1436 + assign51970_e66863);
        (assign51970_e66864, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn5)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn5)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn6)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn6)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn7)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn7)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn8)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn8)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign51970_e66862 * assign51970_e66862))),)
    } else {
        (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8,)
    }
};
        locals.var_sp_s_x0__blk1455 = assign51970_e66866;
        locals.var_sp_s_x0__blk1455_dn5 = assign51970_e66866_d_n5;
        locals.var_sp_s_x0__blk1455_dn6 = assign51970_e66866_d_n6;
        locals.var_sp_s_x0__blk1455_dn7 = assign51970_e66866_d_n7;
        locals.var_sp_s_x0__blk1455_dn8 = assign51970_e66866_d_n8;

        let assign51980_e66869: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign51980_e66869;

        let (assign51990_e66881, assign51990_e66881_d_n5, assign51990_e66881_d_n6, assign51990_e66881_d_n7, assign51990_e66881_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign51990_e66879: f64 = (locals.var_sp_s_x0__blk1455).exp();
        (assign51990_e66879, (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn5), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn6), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn7), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign51990_e66881;
        locals.var_sp_s_delta0__blk1441_dn5 = assign51990_e66881_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign51990_e66881_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign51990_e66881_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign51990_e66881_d_n8;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign52000_e66894, assign52000_e66894_d_n5, assign52000_e66894_d_n6, assign52000_e66894_d_n7, assign52000_e66894_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign52000_e66892: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
        (assign52000_e66892, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52000_e66894;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52000_e66894_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52000_e66894_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52000_e66894_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52000_e66894_d_n8;

        let (assign52010_e66907, assign52010_e66907_d_n5, assign52010_e66907_d_n6, assign52010_e66907_d_n7, assign52010_e66907_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign52010_e66905: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441);
        (assign52010_e66905, ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52010_e66907;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52010_e66907_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52010_e66907_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52010_e66907_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52010_e66907_d_n8;

        let assign52020_e66911: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52020_e66912: f64 = if locals.var_sp_s_x0__blk1455 > assign52020_e66911 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign52020_e66912;

        let (assign52030_e66929, assign52030_e66929_d_n5, assign52030_e66929_d_n6, assign52030_e66929_d_n7, assign52030_e66929_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign52030_e66926: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_d__blk1390);
        let assign52030_e66927: f64 = (assign52030_e66926).exp();
        (assign52030_e66927, (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_d__blk1390_dn8)),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52030_e66929;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52030_e66929_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52030_e66929_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52030_e66929_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52030_e66929_d_n8;

        let (assign52040_e66945, assign52040_e66945_d_n5, assign52040_e66945_d_n6, assign52040_e66945_d_n7, assign52040_e66945_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign52040_e66943: f64 = (locals.var_delta_nd__blk1392 / locals.var_sp_s_delta0__blk1441);
        (assign52040_e66943, (((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52040_e66945;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52040_e66945_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52040_e66945_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52040_e66945_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52040_e66945_d_n8;

        let (assign52050_e66988, assign52050_e66988_d_n5, assign52050_e66988_d_n6, assign52050_e66988_d_n7, assign52050_e66988_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign52050_e66962: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66964: f64 = (assign52050_e66962 - 230.25850929940458);
        let assign52050_e66969: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66971: f64 = (assign52050_e66969 - 230.25850929940458);
        let assign52050_e66975: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
        let assign52050_e66977: f64 = (assign52050_e66975 - 230.25850929940458);
        let assign52050_e66979: f64 = (assign52050_e66977 * 0.3333333333333333);
        let assign52050_e66980: f64 = (1.0 + assign52050_e66979);
        let assign52050_e66981: f64 = (assign52050_e66971 * assign52050_e66980);
        let assign52050_e66982: f64 = (0.5 * assign52050_e66981);
        let assign52050_e66983: f64 = (1.0 + assign52050_e66982);
        let assign52050_e66984: f64 = (assign52050_e66964 * assign52050_e66983);
        let assign52050_e66985: f64 = (1.0 + assign52050_e66984);
        let assign52050_e66986: f64 = (1e-100 / assign52050_e66985);
        (assign52050_e66986, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))),)
    } else {
        (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8,)
    }
};
        locals.var_sp_s_delta0__blk1441 = assign52050_e66988;
        locals.var_sp_s_delta0__blk1441_dn5 = assign52050_e66988_d_n5;
        locals.var_sp_s_delta0__blk1441_dn6 = assign52050_e66988_d_n6;
        locals.var_sp_s_delta0__blk1441_dn7 = assign52050_e66988_d_n7;
        locals.var_sp_s_delta0__blk1441_dn8 = assign52050_e66988_d_n8;

        let (assign52060_e67025, assign52060_e67025_d_n5, assign52060_e67025_d_n6, assign52060_e67025_d_n7, assign52060_e67025_d_n8,) = {
    if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign52060_e67005: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67010: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67014: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
        let assign52060_e67016: f64 = (assign52060_e67014 * 0.3333333333333333);
        let assign52060_e67017: f64 = (1.0 + assign52060_e67016);
        let assign52060_e67018: f64 = (assign52060_e67010 * assign52060_e67017);
        let assign52060_e67019: f64 = (0.5 * assign52060_e67018);
        let assign52060_e67020: f64 = (1.0 + assign52060_e67019);
        let assign52060_e67021: f64 = (assign52060_e67005 * assign52060_e67020);
        let assign52060_e67022: f64 = (1.0 + assign52060_e67021);
        let assign52060_e67023: f64 = (1e-100 / assign52060_e67022);
        (assign52060_e67023, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))),)
    } else {
        (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8,)
    }
};
        locals.var_sp_s_delta1__blk1442 = assign52060_e67025;
        locals.var_sp_s_delta1__blk1442_dn5 = assign52060_e67025_d_n5;
        locals.var_sp_s_delta1__blk1442_dn6 = assign52060_e67025_d_n6;
        locals.var_sp_s_delta1__blk1442_dn7 = assign52060_e67025_d_n7;
        locals.var_sp_s_delta1__blk1442_dn8 = assign52060_e67025_d_n8;

        let (assign52070_e67040, assign52070_e67040_d_n5, assign52070_e67040_d_n6, assign52070_e67040_d_n7, assign52070_e67040_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52070_e67036: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign52070_e67037: f64 = (2.0 + assign52070_e67036);
        let assign52070_e67038: f64 = (1.0 / assign52070_e67037);
        (assign52070_e67038, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign52070_e67037 * assign52070_e67037))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52070_e67040;
        locals.var_sp_s_temp__blk1431_dn5 = assign52070_e67040_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52070_e67040_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52070_e67040_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52070_e67040_d_n8;

        let (assign52080_e67053, assign52080_e67053_d_n5, assign52080_e67053_d_n6, assign52080_e67053_d_n7, assign52080_e67053_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52080_e67049: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
        let assign52080_e67051: f64 = (assign52080_e67049 * locals.var_sp_s_temp__blk1431);
        (assign52080_e67051, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8,)
    }
};
        locals.var_sp_s_xi0__blk1443 = assign52080_e67053;
        locals.var_sp_s_xi0__blk1443_dn5 = assign52080_e67053_d_n5;
        locals.var_sp_s_xi0__blk1443_dn6 = assign52080_e67053_d_n6;
        locals.var_sp_s_xi0__blk1443_dn7 = assign52080_e67053_d_n7;
        locals.var_sp_s_xi0__blk1443_dn8 = assign52080_e67053_d_n8;

        let (assign52090_e67068, assign52090_e67068_d_n5, assign52090_e67068_d_n6, assign52090_e67068_d_n7, assign52090_e67068_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52090_e67063: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
        let assign52090_e67065: f64 = (assign52090_e67063 * locals.var_sp_s_temp__blk1431);
        let assign52090_e67066: f64 = (4.0 * assign52090_e67065);
        (assign52090_e67066, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn8))),)
    } else {
        (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8,)
    }
};
        locals.var_sp_s_xi1__blk1444 = assign52090_e67068;
        locals.var_sp_s_xi1__blk1444_dn5 = assign52090_e67068_d_n5;
        locals.var_sp_s_xi1__blk1444_dn6 = assign52090_e67068_d_n6;
        locals.var_sp_s_xi1__blk1444_dn7 = assign52090_e67068_d_n7;
        locals.var_sp_s_xi1__blk1444_dn8 = assign52090_e67068_d_n8;

        let (assign52100_e67087, assign52100_e67087_d_n5, assign52100_e67087_d_n6, assign52100_e67087_d_n7, assign52100_e67087_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52100_e67077: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
        let assign52100_e67080: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
        let assign52100_e67081: f64 = (assign52100_e67077 - assign52100_e67080);
        let assign52100_e67083: f64 = (assign52100_e67081 * locals.var_sp_s_temp__blk1431);
        let assign52100_e67085: f64 = (assign52100_e67083 * locals.var_sp_s_temp__blk1431);
        (assign52100_e67085, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn8)),)
    } else {
        (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8,)
    }
};
        locals.var_sp_s_xi2__blk1445 = assign52100_e67087;
        locals.var_sp_s_xi2__blk1445_dn5 = assign52100_e67087_d_n5;
        locals.var_sp_s_xi2__blk1445_dn6 = assign52100_e67087_d_n6;
        locals.var_sp_s_xi2__blk1445_dn7 = assign52100_e67087_d_n7;
        locals.var_sp_s_xi2__blk1445_dn8 = assign52100_e67087_d_n8;

        let (assign52110_e67098, assign52110_e67098_d_n5, assign52110_e67098_d_n6, assign52110_e67098_d_n7, assign52110_e67098_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52110_e67096: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
        (assign52110_e67096, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52110_e67098;
        locals.var_sp_s_temp__blk1431_dn5 = assign52110_e67098_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52110_e67098_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52110_e67098_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52110_e67098_d_n8;

        let (assign52120_e67123, assign52120_e67123_d_n5, assign52120_e67123_d_n6, assign52120_e67123_d_n7, assign52120_e67123_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52120_e67107: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
        let assign52120_e67111: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
        let assign52120_e67113: f64 = (assign52120_e67111 + locals.var_sp_s_delta0__blk1441);
        let assign52120_e67117: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
        let assign52120_e67118: f64 = (locals.var_delta_nd__blk1392 * assign52120_e67117);
        let assign52120_e67119: f64 = (assign52120_e67113 - assign52120_e67118);
        let assign52120_e67120: f64 = (locals.var_gf2__blk1308 * assign52120_e67119);
        let assign52120_e67121: f64 = (assign52120_e67107 + assign52120_e67120);
        (assign52120_e67121, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))),)
    } else {
        (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8,)
    }
};
        locals.var_sp_s_pc__blk1446 = assign52120_e67123;
        locals.var_sp_s_pc__blk1446_dn5 = assign52120_e67123_d_n5;
        locals.var_sp_s_pc__blk1446_dn6 = assign52120_e67123_d_n6;
        locals.var_sp_s_pc__blk1446_dn7 = assign52120_e67123_d_n7;
        locals.var_sp_s_pc__blk1446_dn8 = assign52120_e67123_d_n8;

        let (assign52130_e67152, assign52130_e67152_d_n5, assign52130_e67152_d_n6, assign52130_e67152_d_n7, assign52130_e67152_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52130_e67132: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign52130_e67136: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
        let assign52130_e67138: f64 = (assign52130_e67136 - 1.0);
        let assign52130_e67140: f64 = (assign52130_e67138 + locals.var_sp_s_delta0__blk1441);
        let assign52130_e67144: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
        let assign52130_e67146: f64 = (assign52130_e67144 + locals.var_sp_s_xi0__blk1443);
        let assign52130_e67147: f64 = (locals.var_delta_nd__blk1392 * assign52130_e67146);
        let assign52130_e67148: f64 = (assign52130_e67140 - assign52130_e67147);
        let assign52130_e67149: f64 = (locals.var_gf2__blk1308 * assign52130_e67148);
        let assign52130_e67150: f64 = (assign52130_e67132 - assign52130_e67149);
        (assign52130_e67150, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    } else {
        (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8,)
    }
};
        locals.var_sp_s_qc__blk1447 = assign52130_e67152;
        locals.var_sp_s_qc__blk1447_dn5 = assign52130_e67152_d_n5;
        locals.var_sp_s_qc__blk1447_dn6 = assign52130_e67152_d_n6;
        locals.var_sp_s_qc__blk1447_dn7 = assign52130_e67152_d_n7;
        locals.var_sp_s_qc__blk1447_dn8 = assign52130_e67152_d_n8;

        let (assign52140_e67171, assign52140_e67171_d_n5, assign52140_e67171_d_n6, assign52140_e67171_d_n7, assign52140_e67171_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52140_e67163: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
        let assign52140_e67166: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
        let assign52140_e67167: f64 = (assign52140_e67163 - assign52140_e67166);
        let assign52140_e67168: f64 = (locals.var_gf2__blk1308 * assign52140_e67167);
        let assign52140_e67169: f64 = (2.0 - assign52140_e67168);
        (assign52140_e67169, (-((locals.var_gf2__blk1308_dn5 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8)))))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52140_e67171;
        locals.var_sp_s_temp__blk1431_dn5 = assign52140_e67171_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52140_e67171_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52140_e67171_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52140_e67171_d_n8;

        let (assign52150_e67188, assign52150_e67188_d_n5, assign52150_e67188_d_n6, assign52150_e67188_d_n7, assign52150_e67188_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52150_e67180: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
        let assign52150_e67184: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
        let assign52150_e67185: f64 = (2.0 * assign52150_e67184);
        let assign52150_e67186: f64 = (assign52150_e67180 - assign52150_e67185);
        (assign52150_e67186, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))),)
    } else {
        (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8,)
    }
};
        locals.var_sp_s_temp__blk1431 = assign52150_e67188;
        locals.var_sp_s_temp__blk1431_dn5 = assign52150_e67188_d_n5;
        locals.var_sp_s_temp__blk1431_dn6 = assign52150_e67188_d_n6;
        locals.var_sp_s_temp__blk1431_dn7 = assign52150_e67188_d_n7;
        locals.var_sp_s_temp__blk1431_dn8 = assign52150_e67188_d_n8;

        let (assign52160_e67206, assign52160_e67206_d_n5, assign52160_e67206_d_n6, assign52160_e67206_d_n7, assign52160_e67206_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
        let assign52160_e67200: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
        let assign52160_e67201: f64 = (locals.var_sp_s_pc__blk1446 + assign52160_e67200);
        let assign52160_e67202: f64 = (locals.var_sp_s_qc__blk1447 / assign52160_e67201);
        let assign52160_e67203: f64 = (2.0 * assign52160_e67202);
        let assign52160_e67204: f64 = (locals.var_sp_s_x0__blk1455 + assign52160_e67203);
        (assign52160_e67204, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign52160_e67206;
        locals.var_x_d__blk1393_dn5 = assign52160_e67206_d_n5;
        locals.var_x_d__blk1393_dn6 = assign52160_e67206_d_n6;
        locals.var_x_d__blk1393_dn7 = assign52160_e67206_d_n7;
        locals.var_x_d__blk1393_dn8 = assign52160_e67206_d_n8;

        let (assign52170_e67214, assign52170_e67214_d_n5, assign52170_e67214_d_n6, assign52170_e67214_d_n7, assign52170_e67214_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52170_e67212: f64 = (locals.var_x_d__blk1393 - locals.var_x_s__blk1346);
        (assign52170_e67212, (locals.var_x_d__blk1393_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_x_d__blk1393_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_x_d__blk1393_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_x_d__blk1393_dn8 - locals.var_x_s__blk1346_dn8),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52170_e67214;
        locals.var_x_ds__blk1394_dn5 = assign52170_e67214_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52170_e67214_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52170_e67214_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52170_e67214_d_n8;

        let assign52180_e67217: f64 = if locals.var_x_ds__blk1394 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign52180_e67217;

        let (assign52190_e67245, assign52190_e67245_d_n5, assign52190_e67245_d_n6, assign52190_e67245_d_n7, assign52190_e67245_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52190_e67226: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
        let assign52190_e67227: f64 = (2.0 * assign52190_e67226);
        let assign52190_e67231: f64 = (1.0 - locals.var_es__blk1352);
        let assign52190_e67234: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
        let assign52190_e67235: f64 = (assign52190_e67231 + assign52190_e67234);
        let assign52190_e67239: f64 = (1.0 + locals.var_xi1s__blk1349);
        let assign52190_e67240: f64 = (locals.var_delta_nd__blk1392 * assign52190_e67239);
        let assign52190_e67241: f64 = (assign52190_e67235 - assign52190_e67240);
        let assign52190_e67242: f64 = (locals.var_gf2__blk1308 * assign52190_e67241);
        let assign52190_e67243: f64 = (assign52190_e67227 + assign52190_e67242);
        (assign52190_e67243, ((2.0 * (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn5) + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn5)))))), ((2.0 * (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn6) + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn6)))))), ((2.0 * (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn7) + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn7)))))), ((2.0 * (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn8) + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn8)))))),)
    } else {
        (locals.var_pc__blk1395, locals.var_pc__blk1395_dn5, locals.var_pc__blk1395_dn6, locals.var_pc__blk1395_dn7, locals.var_pc__blk1395_dn8,)
    }
};
        locals.var_pc__blk1395 = assign52190_e67245;
        locals.var_pc__blk1395_dn5 = assign52190_e67245_d_n5;
        locals.var_pc__blk1395_dn6 = assign52190_e67245_d_n6;
        locals.var_pc__blk1395_dn7 = assign52190_e67245_d_n7;
        locals.var_pc__blk1395_dn8 = assign52190_e67245_d_n8;

        let (assign52200_e67259, assign52200_e67259_d_n5, assign52200_e67259_d_n6, assign52200_e67259_d_n7, assign52200_e67259_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52200_e67254: f64 = (1.0 - locals.var_k_ds__blk1391);
        let assign52200_e67255: f64 = (locals.var_gf2__blk1308 * assign52200_e67254);
        let assign52200_e67257: f64 = (assign52200_e67255 * locals.var_ds__blk1353);
        (assign52200_e67257, ((((locals.var_gf2__blk1308_dn5 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn5))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn5)), ((((locals.var_gf2__blk1308_dn6 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn6))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn6)), ((((locals.var_gf2__blk1308_dn7 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn7))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn7)), ((((locals.var_gf2__blk1308_dn8 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn8))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn8)),)
    } else {
        (locals.var_qc__blk1396, locals.var_qc__blk1396_dn5, locals.var_qc__blk1396_dn6, locals.var_qc__blk1396_dn7, locals.var_qc__blk1396_dn8,)
    }
};
        locals.var_qc__blk1396 = assign52200_e67259;
        locals.var_qc__blk1396_dn5 = assign52200_e67259_d_n5;
        locals.var_qc__blk1396_dn6 = assign52200_e67259_d_n6;
        locals.var_qc__blk1396_dn7 = assign52200_e67259_d_n7;
        locals.var_qc__blk1396_dn8 = assign52200_e67259_d_n8;

        let (assign52210_e67279, assign52210_e67279_d_n5, assign52210_e67279_d_n6, assign52210_e67279_d_n7, assign52210_e67279_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52210_e67270: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
        let assign52210_e67271: f64 = (locals.var_es__blk1352 + assign52210_e67270);
        let assign52210_e67274: f64 = (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350);
        let assign52210_e67275: f64 = (assign52210_e67271 - assign52210_e67274);
        let assign52210_e67276: f64 = (locals.var_gf2__blk1308 * assign52210_e67275);
        let assign52210_e67277: f64 = (2.0 - assign52210_e67276);
        (assign52210_e67277, (-((locals.var_gf2__blk1308_dn5 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn5 + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn6 + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn7 + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn8 + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn8)))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52210_e67279;
        locals.var_temp__blk936_dn5 = assign52210_e67279_d_n5;
        locals.var_temp__blk936_dn6 = assign52210_e67279_d_n6;
        locals.var_temp__blk936_dn7 = assign52210_e67279_d_n7;
        locals.var_temp__blk936_dn8 = assign52210_e67279_d_n8;

        let (assign52220_e67295, assign52220_e67295_d_n5, assign52220_e67295_d_n6, assign52220_e67295_d_n7, assign52220_e67295_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52220_e67287: f64 = (locals.var_pc__blk1395 * locals.var_pc__blk1395);
        let assign52220_e67291: f64 = (locals.var_temp__blk936 * locals.var_qc__blk1396);
        let assign52220_e67292: f64 = (2.0 * assign52220_e67291);
        let assign52220_e67293: f64 = (assign52220_e67287 - assign52220_e67292);
        (assign52220_e67293, (((locals.var_pc__blk1395_dn5 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn5)) - (2.0 * ((locals.var_temp__blk936_dn5 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn5)))), (((locals.var_pc__blk1395_dn6 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn6)) - (2.0 * ((locals.var_temp__blk936_dn6 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn6)))), (((locals.var_pc__blk1395_dn7 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn7)) - (2.0 * ((locals.var_temp__blk936_dn7 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn7)))), (((locals.var_pc__blk1395_dn8 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn8)) - (2.0 * ((locals.var_temp__blk936_dn8 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn8)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52220_e67295;
        locals.var_temp__blk936_dn5 = assign52220_e67295_d_n5;
        locals.var_temp__blk936_dn6 = assign52220_e67295_d_n6;
        locals.var_temp__blk936_dn7 = assign52220_e67295_d_n7;
        locals.var_temp__blk936_dn8 = assign52220_e67295_d_n8;

        let (assign52230_e67310, assign52230_e67310_d_n5, assign52230_e67310_d_n6, assign52230_e67310_d_n7, assign52230_e67310_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52230_e67305: f64 = (locals.var_temp__blk936).sqrt();
        let assign52230_e67306: f64 = (locals.var_pc__blk1395 + assign52230_e67305);
        let assign52230_e67307: f64 = (locals.var_qc__blk1396 / assign52230_e67306);
        let assign52230_e67308: f64 = (2.0 * assign52230_e67307);
        (assign52230_e67308, (2.0 * (((locals.var_qc__blk1396_dn5 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn5 + (locals.var_temp__blk936_dn5 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn6 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn6 + (locals.var_temp__blk936_dn6 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn7 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn7 + (locals.var_temp__blk936_dn7 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn8 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn8 + (locals.var_temp__blk936_dn8 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52230_e67310;
        locals.var_x_ds__blk1394_dn5 = assign52230_e67310_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52230_e67310_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52230_e67310_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52230_e67310_d_n8;

        let (assign52240_e67320, assign52240_e67320_d_n5, assign52240_e67320_d_n6, assign52240_e67320_d_n7, assign52240_e67320_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign52240_e67318: f64 = (locals.var_x_s__blk1346 + locals.var_x_ds__blk1394);
        (assign52240_e67318, (locals.var_x_s__blk1346_dn5 + locals.var_x_ds__blk1394_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_x_ds__blk1394_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_x_ds__blk1394_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_x_ds__blk1394_dn8),)
    } else {
        (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8,)
    }
};
        locals.var_x_d__blk1393 = assign52240_e67320;
        locals.var_x_d__blk1393_dn5 = assign52240_e67320_d_n5;
        locals.var_x_d__blk1393_dn6 = assign52240_e67320_d_n6;
        locals.var_x_d__blk1393_dn7 = assign52240_e67320_d_n7;
        locals.var_x_d__blk1393_dn8 = assign52240_e67320_d_n8;

        let (assign52250_e67328, assign52250_e67328_d_n5, assign52250_e67328_d_n6, assign52250_e67328_d_n7, assign52250_e67328_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52250_e67326: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
        (assign52250_e67326, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign52250_e67328;
        locals.var_dps__blk1397_dn5 = assign52250_e67328_d_n5;
        locals.var_dps__blk1397_dn6 = assign52250_e67328_d_n6;
        locals.var_dps__blk1397_dn7 = assign52250_e67328_d_n7;
        locals.var_dps__blk1397_dn8 = assign52250_e67328_d_n8;

        let (assign52260_e67342, assign52260_e67342_d_n5, assign52260_e67342_d_n6, assign52260_e67342_d_n7, assign52260_e67342_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52260_e67334: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52260_e67338: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52260_e67339: f64 = (2.0 + assign52260_e67338);
        let assign52260_e67340: f64 = (assign52260_e67334 / assign52260_e67339);
        (assign52260_e67340, (((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)))) / (assign52260_e67339 * assign52260_e67339)),)
    } else {
        (locals.var_xi0d__blk1398, locals.var_xi0d__blk1398_dn5, locals.var_xi0d__blk1398_dn6, locals.var_xi0d__blk1398_dn7, locals.var_xi0d__blk1398_dn8,)
    }
};
        locals.var_xi0d__blk1398 = assign52260_e67342;
        locals.var_xi0d__blk1398_dn5 = assign52260_e67342_d_n5;
        locals.var_xi0d__blk1398_dn6 = assign52260_e67342_d_n6;
        locals.var_xi0d__blk1398_dn7 = assign52260_e67342_d_n7;
        locals.var_xi0d__blk1398_dn8 = assign52260_e67342_d_n8;

        let assign52270_e67345: f64 = if locals.var_x_d__blk1393 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign52270_e67345;

        let (assign52280_e67355, assign52280_e67355_d_n5, assign52280_e67355_d_n6, assign52280_e67355_d_n7, assign52280_e67355_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        let assign52280_e67352: f64 = (-locals.var_x_d__blk1393);
        let assign52280_e67353: f64 = (assign52280_e67352).exp();
        (assign52280_e67353, (assign52280_e67353 * (-locals.var_x_d__blk1393_dn5)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn6)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn7)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn8)),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52280_e67355;
        locals.var_ed__blk1399_dn5 = assign52280_e67355_d_n5;
        locals.var_ed__blk1399_dn6 = assign52280_e67355_d_n6;
        locals.var_ed__blk1399_dn7 = assign52280_e67355_d_n7;
        locals.var_ed__blk1399_dn8 = assign52280_e67355_d_n8;

        let assign52290_e67358: f64 = if locals.var_x_d__blk1393 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign52290_e67358;

        let (assign52300_e67384, assign52300_e67384_d_n5, assign52300_e67384_d_n6, assign52300_e67384_d_n7, assign52300_e67384_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52300_e67369: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
        let assign52300_e67376: f64 = (0.25 * locals.var_x_d__blk1393);
        let assign52300_e67377: f64 = (1.0 - assign52300_e67376);
        let assign52300_e67378: f64 = (locals.var_x_d__blk1393 * assign52300_e67377);
        let assign52300_e67379: f64 = (0.3333333333333333 * assign52300_e67378);
        let assign52300_e67380: f64 = (1.0 - assign52300_e67379);
        let assign52300_e67381: f64 = (assign52300_e67369 * assign52300_e67380);
        let assign52300_e67382: f64 = (0.5 * assign52300_e67381);
        (assign52300_e67382, (0.5 * ((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8))))))))),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52300_e67384;
        locals.var_pd__blk1400_dn5 = assign52300_e67384_d_n5;
        locals.var_pd__blk1400_dn6 = assign52300_e67384_d_n6;
        locals.var_pd__blk1400_dn7 = assign52300_e67384_d_n7;
        locals.var_pd__blk1400_dn8 = assign52300_e67384_d_n8;

        let (assign52310_e67405, assign52310_e67405_d_n5, assign52310_e67405_d_n6, assign52310_e67405_d_n7, assign52310_e67405_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52310_e67398: f64 = (0.25 * locals.var_x_d__blk1393);
        let assign52310_e67399: f64 = (1.0 - assign52310_e67398);
        let assign52310_e67400: f64 = (locals.var_x_d__blk1393 * assign52310_e67399);
        let assign52310_e67401: f64 = (0.3333333333333333 * assign52310_e67400);
        let assign52310_e67402: f64 = (1.0 - assign52310_e67401);
        let assign52310_e67403: f64 = (assign52310_e67402).sqrt();
        (assign52310_e67403, ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8)))))) / (2.0 * assign52310_e67403)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52310_e67405;
        locals.var_temp__blk936_dn5 = assign52310_e67405_d_n5;
        locals.var_temp__blk936_dn6 = assign52310_e67405_d_n6;
        locals.var_temp__blk936_dn7 = assign52310_e67405_d_n7;
        locals.var_temp__blk936_dn8 = assign52310_e67405_d_n8;

    }

    pub(super) fn stamp_transient_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign52320_e67419, assign52320_e67419_d_n5, assign52320_e67419_d_n6, assign52320_e67419_d_n7, assign52320_e67419_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52320_e67416: f64 = (locals.var_x_d__blk1393 * locals.var_temp__blk936);
        let assign52320_e67417: f64 = (0.7071067811865475 * assign52320_e67416);
        (assign52320_e67417, (0.7071067811865475 * ((locals.var_x_d__blk1393_dn5 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn6 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn7 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn8 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52320_e67419;
        locals.var_sqd__blk1401_dn5 = assign52320_e67419_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52320_e67419_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52320_e67419_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52320_e67419_d_n8;

        let (assign52330_e67443, assign52330_e67443_d_n5, assign52330_e67443_d_n6, assign52330_e67443_d_n7, assign52330_e67443_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign52330_e67429: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1392);
        let assign52330_e67431: f64 = (assign52330_e67429 * locals.var_x_d__blk1393);
        let assign52330_e67433: f64 = (assign52330_e67431 * locals.var_x_d__blk1393);
        let assign52330_e67435: f64 = (assign52330_e67433 * locals.var_x_d__blk1393);
        let assign52330_e67439: f64 = (1.75 * locals.var_x_d__blk1393);
        let assign52330_e67440: f64 = (1.0 + assign52330_e67439);
        let assign52330_e67441: f64 = (assign52330_e67435 * assign52330_e67440);
        (assign52330_e67441, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn5) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn5)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn5))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn6) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn6)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn7) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn7)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn8) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn8)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn8))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52330_e67443;
        locals.var_dd__blk1402_dn5 = assign52330_e67443_d_n5;
        locals.var_dd__blk1402_dn6 = assign52330_e67443_d_n6;
        locals.var_dd__blk1402_dn7 = assign52330_e67443_d_n7;
        locals.var_dd__blk1402_dn8 = assign52330_e67443_d_n8;

        let (assign52340_e67458, assign52340_e67458_d_n5, assign52340_e67458_d_n6, assign52340_e67458_d_n7, assign52340_e67458_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52340_e67454: f64 = (locals.var_x_d__blk1393 - 1.0);
        let assign52340_e67456: f64 = (assign52340_e67454 + locals.var_ed__blk1399);
        (assign52340_e67456, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52340_e67458;
        locals.var_pd__blk1400_dn5 = assign52340_e67458_d_n5;
        locals.var_pd__blk1400_dn6 = assign52340_e67458_d_n6;
        locals.var_pd__blk1400_dn7 = assign52340_e67458_d_n7;
        locals.var_pd__blk1400_dn8 = assign52340_e67458_d_n8;

        let (assign52350_e67470, assign52350_e67470_d_n5, assign52350_e67470_d_n6, assign52350_e67470_d_n7, assign52350_e67470_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52350_e67468: f64 = (locals.var_pd__blk1400).sqrt();
        (assign52350_e67468, (locals.var_pd__blk1400_dn5 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52350_e67468)),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52350_e67470;
        locals.var_sqd__blk1401_dn5 = assign52350_e67470_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52350_e67470_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52350_e67470_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52350_e67470_d_n8;

        let (assign52360_e67491, assign52360_e67491_d_n5, assign52360_e67491_d_n6, assign52360_e67491_d_n7, assign52360_e67491_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign52360_e67482: f64 = (1.0 / locals.var_ed__blk1399);
        let assign52360_e67484: f64 = (assign52360_e67482 - locals.var_x_d__blk1393);
        let assign52360_e67486: f64 = (assign52360_e67484 - 1.0);
        let assign52360_e67488: f64 = (assign52360_e67486 - locals.var_xi0d__blk1398);
        let assign52360_e67489: f64 = (locals.var_delta_nd__blk1392 * assign52360_e67488);
        (assign52360_e67489, ((locals.var_delta_nd__blk1392_dn5 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn5 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn5) - locals.var_xi0d__blk1398_dn5))), ((locals.var_delta_nd__blk1392_dn6 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn6 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn6) - locals.var_xi0d__blk1398_dn6))), ((locals.var_delta_nd__blk1392_dn7 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn7 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn7) - locals.var_xi0d__blk1398_dn7))), ((locals.var_delta_nd__blk1392_dn8 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn8 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn8) - locals.var_xi0d__blk1398_dn8))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52360_e67491;
        locals.var_dd__blk1402_dn5 = assign52360_e67491_d_n5;
        locals.var_dd__blk1402_dn6 = assign52360_e67491_d_n6;
        locals.var_dd__blk1402_dn7 = assign52360_e67491_d_n7;
        locals.var_dd__blk1402_dn8 = assign52360_e67491_d_n8;

        let assign52370_e67495: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52370_e67496: f64 = if locals.var_x_d__blk1393 > assign52370_e67495 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign52370_e67496;

        let (assign52380_e67510, assign52380_e67510_d_n5, assign52380_e67510_d_n6, assign52380_e67510_d_n7, assign52380_e67510_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52380_e67507: f64 = (locals.var_x_d__blk1393 - locals.var_xn_d__blk1390);
        let assign52380_e67508: f64 = (assign52380_e67507).exp();
        (assign52380_e67508, (assign52380_e67508 * (locals.var_x_d__blk1393_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn8 - locals.var_xn_d__blk1390_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52380_e67510;
        locals.var_temp__blk936_dn5 = assign52380_e67510_d_n5;
        locals.var_temp__blk936_dn6 = assign52380_e67510_d_n6;
        locals.var_temp__blk936_dn7 = assign52380_e67510_d_n7;
        locals.var_temp__blk936_dn8 = assign52380_e67510_d_n8;

        let (assign52390_e67523, assign52390_e67523_d_n5, assign52390_e67523_d_n6, assign52390_e67523_d_n7, assign52390_e67523_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52390_e67521: f64 = (locals.var_delta_nd__blk1392 / locals.var_temp__blk936);
        (assign52390_e67521, (((locals.var_delta_nd__blk1392_dn5 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52390_e67523;
        locals.var_ed__blk1399_dn5 = assign52390_e67523_d_n5;
        locals.var_ed__blk1399_dn6 = assign52390_e67523_d_n6;
        locals.var_ed__blk1399_dn7 = assign52390_e67523_d_n7;
        locals.var_ed__blk1399_dn8 = assign52390_e67523_d_n8;

        let (assign52400_e67542, assign52400_e67542_d_n5, assign52400_e67542_d_n6, assign52400_e67542_d_n7, assign52400_e67542_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign52400_e67536: f64 = (locals.var_x_d__blk1393 + 1.0);
        let assign52400_e67538: f64 = (assign52400_e67536 + locals.var_xi0d__blk1398);
        let assign52400_e67539: f64 = (locals.var_delta_nd__blk1392 * assign52400_e67538);
        let assign52400_e67540: f64 = (locals.var_temp__blk936 - assign52400_e67539);
        (assign52400_e67540, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52400_e67542;
        locals.var_dd__blk1402_dn5 = assign52400_e67542_d_n5;
        locals.var_dd__blk1402_dn6 = assign52400_e67542_d_n6;
        locals.var_dd__blk1402_dn7 = assign52400_e67542_d_n7;
        locals.var_dd__blk1402_dn8 = assign52400_e67542_d_n8;

        let (assign52410_e67576, assign52410_e67576_d_n5, assign52410_e67576_d_n6, assign52410_e67576_d_n7, assign52410_e67576_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52410_e67556: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67561: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67565: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
        let assign52410_e67567: f64 = (assign52410_e67565 * 0.3333333333333333);
        let assign52410_e67568: f64 = (1.0 + assign52410_e67567);
        let assign52410_e67569: f64 = (assign52410_e67561 * assign52410_e67568);
        let assign52410_e67570: f64 = (0.5 * assign52410_e67569);
        let assign52410_e67571: f64 = (1.0 + assign52410_e67570);
        let assign52410_e67572: f64 = (assign52410_e67556 * assign52410_e67571);
        let assign52410_e67573: f64 = (1.0 + assign52410_e67572);
        let assign52410_e67574: f64 = (1e-100 / assign52410_e67573);
        (assign52410_e67574, (-((1e-100 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn5 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn6 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn7 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn8 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))),)
    } else {
        (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8,)
    }
};
        locals.var_ed__blk1399 = assign52410_e67576;
        locals.var_ed__blk1399_dn5 = assign52410_e67576_d_n5;
        locals.var_ed__blk1399_dn6 = assign52410_e67576_d_n6;
        locals.var_ed__blk1399_dn7 = assign52410_e67576_d_n7;
        locals.var_ed__blk1399_dn8 = assign52410_e67576_d_n8;

        let (assign52420_e67616, assign52420_e67616_d_n5, assign52420_e67616_d_n6, assign52420_e67616_d_n7, assign52420_e67616_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52420_e67590: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67592: f64 = (assign52420_e67590 - 230.25850929940458);
        let assign52420_e67597: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67599: f64 = (assign52420_e67597 - 230.25850929940458);
        let assign52420_e67603: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
        let assign52420_e67605: f64 = (assign52420_e67603 - 230.25850929940458);
        let assign52420_e67607: f64 = (assign52420_e67605 * 0.3333333333333333);
        let assign52420_e67608: f64 = (1.0 + assign52420_e67607);
        let assign52420_e67609: f64 = (assign52420_e67599 * assign52420_e67608);
        let assign52420_e67610: f64 = (0.5 * assign52420_e67609);
        let assign52420_e67611: f64 = (1.0 + assign52420_e67610);
        let assign52420_e67612: f64 = (assign52420_e67592 * assign52420_e67611);
        let assign52420_e67613: f64 = (1.0 + assign52420_e67612);
        let assign52420_e67614: f64 = (1e-100 / assign52420_e67613);
        (assign52420_e67614, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52420_e67616;
        locals.var_temp__blk936_dn5 = assign52420_e67616_d_n5;
        locals.var_temp__blk936_dn6 = assign52420_e67616_d_n6;
        locals.var_temp__blk936_dn7 = assign52420_e67616_d_n7;
        locals.var_temp__blk936_dn8 = assign52420_e67616_d_n8;

        let (assign52430_e67636, assign52430_e67636_d_n5, assign52430_e67636_d_n6, assign52430_e67636_d_n7, assign52430_e67636_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign52430_e67630: f64 = (locals.var_x_d__blk1393 + 1.0);
        let assign52430_e67632: f64 = (assign52430_e67630 + locals.var_xi0d__blk1398);
        let assign52430_e67633: f64 = (locals.var_delta_nd__blk1392 * assign52430_e67632);
        let assign52430_e67634: f64 = (locals.var_temp__blk936 - assign52430_e67633);
        (assign52430_e67634, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))),)
    } else {
        (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8,)
    }
};
        locals.var_dd__blk1402 = assign52430_e67636;
        locals.var_dd__blk1402_dn5 = assign52430_e67636_d_n5;
        locals.var_dd__blk1402_dn6 = assign52430_e67636_d_n6;
        locals.var_dd__blk1402_dn7 = assign52430_e67636_d_n7;
        locals.var_dd__blk1402_dn8 = assign52430_e67636_d_n8;

        let (assign52440_e67649, assign52440_e67649_d_n5, assign52440_e67649_d_n6, assign52440_e67649_d_n7, assign52440_e67649_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign52440_e67645: f64 = (locals.var_x_d__blk1393 - 1.0);
        let assign52440_e67647: f64 = (assign52440_e67645 + locals.var_ed__blk1399);
        (assign52440_e67647, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8),)
    } else {
        (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8,)
    }
};
        locals.var_pd__blk1400 = assign52440_e67649;
        locals.var_pd__blk1400_dn5 = assign52440_e67649_d_n5;
        locals.var_pd__blk1400_dn6 = assign52440_e67649_d_n6;
        locals.var_pd__blk1400_dn7 = assign52440_e67649_d_n7;
        locals.var_pd__blk1400_dn8 = assign52440_e67649_d_n8;

        let (assign52450_e67659, assign52450_e67659_d_n5, assign52450_e67659_d_n6, assign52450_e67659_d_n7, assign52450_e67659_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign52450_e67657: f64 = (locals.var_pd__blk1400).sqrt();
        (assign52450_e67657, (locals.var_pd__blk1400_dn5 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52450_e67657)),)
    } else {
        (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8,)
    }
};
        locals.var_sqd__blk1401 = assign52450_e67659;
        locals.var_sqd__blk1401_dn5 = assign52450_e67659_d_n5;
        locals.var_sqd__blk1401_dn6 = assign52450_e67659_d_n6;
        locals.var_sqd__blk1401_dn7 = assign52450_e67659_d_n7;
        locals.var_sqd__blk1401_dn8 = assign52450_e67659_d_n8;

        let (assign52460_e67669, assign52460_e67669_d_n5, assign52460_e67669_d_n6, assign52460_e67669_d_n7, assign52460_e67669_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52460_e67665: f64 = (locals.var_sqd__blk1401 * locals.var_gf__blk1307);
        let assign52460_e67667: f64 = (assign52460_e67665 * locals.var_phit1__blk1322);
        (assign52460_e67667, ((((locals.var_sqd__blk1401_dn5 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqd__blk1401_dn6 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqd__blk1401_dn7 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqd__blk1401_dn8 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    }
};
        locals.var_qbd__blk1403 = assign52460_e67669;
        locals.var_qbd__blk1403_dn5 = assign52460_e67669_d_n5;
        locals.var_qbd__blk1403_dn6 = assign52460_e67669_d_n6;
        locals.var_qbd__blk1403_dn7 = assign52460_e67669_d_n7;
        locals.var_qbd__blk1403_dn8 = assign52460_e67669_d_n8;

        let (assign52470_e67679, assign52470_e67679_d_n5, assign52470_e67679_d_n6, assign52470_e67679_d_n7, assign52470_e67679_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52470_e67676: f64 = (locals.var_x_s__blk1346 + locals.var_x_d__blk1393);
        let assign52470_e67677: f64 = (0.5 * assign52470_e67676);
        (assign52470_e67677, (0.5 * (locals.var_x_s__blk1346_dn5 + locals.var_x_d__blk1393_dn5)), (0.5 * (locals.var_x_s__blk1346_dn6 + locals.var_x_d__blk1393_dn6)), (0.5 * (locals.var_x_s__blk1346_dn7 + locals.var_x_d__blk1393_dn7)), (0.5 * (locals.var_x_s__blk1346_dn8 + locals.var_x_d__blk1393_dn8)),)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign52470_e67679;
        locals.var_x_m__blk1404_dn5 = assign52470_e67679_d_n5;
        locals.var_x_m__blk1404_dn6 = assign52470_e67679_d_n6;
        locals.var_x_m__blk1404_dn7 = assign52470_e67679_d_n7;
        locals.var_x_m__blk1404_dn8 = assign52470_e67679_d_n8;

        let (assign52480_e67685, assign52480_e67685_d_n5, assign52480_e67685_d_n6, assign52480_e67685_d_n7, assign52480_e67685_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52480_e67685;
        locals.var_em__blk1405_dn5 = assign52480_e67685_d_n5;
        locals.var_em__blk1405_dn6 = assign52480_e67685_d_n6;
        locals.var_em__blk1405_dn7 = assign52480_e67685_d_n7;
        locals.var_em__blk1405_dn8 = assign52480_e67685_d_n8;

        let (assign52490_e67693, assign52490_e67693_d_n5, assign52490_e67693_d_n6, assign52490_e67693_d_n7, assign52490_e67693_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52490_e67691: f64 = (locals.var_ed__blk1399 * locals.var_es__blk1352);
        (assign52490_e67691, ((locals.var_ed__blk1399_dn5 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn5)), ((locals.var_ed__blk1399_dn6 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn6)), ((locals.var_ed__blk1399_dn7 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn7)), ((locals.var_ed__blk1399_dn8 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52490_e67693;
        locals.var_temp__blk936_dn5 = assign52490_e67693_d_n5;
        locals.var_temp__blk936_dn6 = assign52490_e67693_d_n6;
        locals.var_temp__blk936_dn7 = assign52490_e67693_d_n7;
        locals.var_temp__blk936_dn8 = assign52490_e67693_d_n8;

        let assign52500_e67696: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign52500_e67696;

        let (assign52510_e67705, assign52510_e67705_d_n5, assign52510_e67705_d_n6, assign52510_e67705_d_n7, assign52510_e67705_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign52510_e67703: f64 = (locals.var_temp__blk936).sqrt();
        (assign52510_e67703, (locals.var_temp__blk936_dn5 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn6 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn7 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn8 / (2.0 * assign52510_e67703)),)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52510_e67705;
        locals.var_em__blk1405_dn5 = assign52510_e67705_d_n5;
        locals.var_em__blk1405_dn6 = assign52510_e67705_d_n6;
        locals.var_em__blk1405_dn7 = assign52510_e67705_d_n7;
        locals.var_em__blk1405_dn8 = assign52510_e67705_d_n8;

        let (assign52520_e67715, assign52520_e67715_d_n5, assign52520_e67715_d_n6, assign52520_e67715_d_n7, assign52520_e67715_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52520_e67712: f64 = (locals.var_ds__blk1353 + locals.var_dd__blk1402);
        let assign52520_e67713: f64 = (0.5 * assign52520_e67712);
        (assign52520_e67713, (0.5 * (locals.var_ds__blk1353_dn5 + locals.var_dd__blk1402_dn5)), (0.5 * (locals.var_ds__blk1353_dn6 + locals.var_dd__blk1402_dn6)), (0.5 * (locals.var_ds__blk1353_dn7 + locals.var_dd__blk1402_dn7)), (0.5 * (locals.var_ds__blk1353_dn8 + locals.var_dd__blk1402_dn8)),)
    } else {
        (locals.var_d_bar__blk1406, locals.var_d_bar__blk1406_dn5, locals.var_d_bar__blk1406_dn6, locals.var_d_bar__blk1406_dn7, locals.var_d_bar__blk1406_dn8,)
    }
};
        locals.var_d_bar__blk1406 = assign52520_e67715;
        locals.var_d_bar__blk1406_dn5 = assign52520_e67715_d_n5;
        locals.var_d_bar__blk1406_dn6 = assign52520_e67715_d_n6;
        locals.var_d_bar__blk1406_dn7 = assign52520_e67715_d_n7;
        locals.var_d_bar__blk1406_dn8 = assign52520_e67715_d_n8;

        let (assign52530_e67733, assign52530_e67733_d_n5, assign52530_e67733_d_n6, assign52530_e67733_d_n7, assign52530_e67733_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52530_e67723: f64 = (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394);
        let assign52530_e67727: f64 = (2.0 * locals.var_inv_gf2__blk1324);
        let assign52530_e67728: f64 = (locals.var_em__blk1405 - assign52530_e67727);
        let assign52530_e67729: f64 = (assign52530_e67723 * assign52530_e67728);
        let assign52530_e67730: f64 = (0.125 * assign52530_e67729);
        let assign52530_e67731: f64 = (locals.var_d_bar__blk1406 + assign52530_e67730);
        (assign52530_e67731, (locals.var_d_bar__blk1406_dn5 + (0.125 * ((((locals.var_x_ds__blk1394_dn5 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn5)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn5 - (2.0 * locals.var_inv_gf2__blk1324_dn5)))))), (locals.var_d_bar__blk1406_dn6 + (0.125 * ((((locals.var_x_ds__blk1394_dn6 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn6)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn6 - (2.0 * locals.var_inv_gf2__blk1324_dn6)))))), (locals.var_d_bar__blk1406_dn7 + (0.125 * ((((locals.var_x_ds__blk1394_dn7 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn7)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn7 - (2.0 * locals.var_inv_gf2__blk1324_dn7)))))), (locals.var_d_bar__blk1406_dn8 + (0.125 * ((((locals.var_x_ds__blk1394_dn8 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn8)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn8 - (2.0 * locals.var_inv_gf2__blk1324_dn8)))))),)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign52530_e67733;
        locals.var_dm__blk1407_dn5 = assign52530_e67733_d_n5;
        locals.var_dm__blk1407_dn6 = assign52530_e67733_d_n6;
        locals.var_dm__blk1407_dn7 = assign52530_e67733_d_n7;
        locals.var_dm__blk1407_dn8 = assign52530_e67733_d_n8;

        let assign52540_e67736: f64 = if locals.var_x_m__blk1404 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign52540_e67736;

        let (assign52550_e67760, assign52550_e67760_d_n5, assign52550_e67760_d_n6, assign52550_e67760_d_n7, assign52550_e67760_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52550_e67745: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
        let assign52550_e67752: f64 = (0.25 * locals.var_x_m__blk1404);
        let assign52550_e67753: f64 = (1.0 - assign52550_e67752);
        let assign52550_e67754: f64 = (locals.var_x_m__blk1404 * assign52550_e67753);
        let assign52550_e67755: f64 = (0.3333333333333333 * assign52550_e67754);
        let assign52550_e67756: f64 = (1.0 - assign52550_e67755);
        let assign52550_e67757: f64 = (assign52550_e67745 * assign52550_e67756);
        let assign52550_e67758: f64 = (0.5 * assign52550_e67757);
        (assign52550_e67758, (0.5 * ((((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8))))))))),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52550_e67760;
        locals.var_pm__blk1408_dn5 = assign52550_e67760_d_n5;
        locals.var_pm__blk1408_dn6 = assign52550_e67760_d_n6;
        locals.var_pm__blk1408_dn7 = assign52550_e67760_d_n7;
        locals.var_pm__blk1408_dn8 = assign52550_e67760_d_n8;

        let (assign52560_e67773, assign52560_e67773_d_n5, assign52560_e67773_d_n6, assign52560_e67773_d_n7, assign52560_e67773_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52560_e67769: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52560_e67770: f64 = (assign52560_e67769).sqrt();
        let assign52560_e67771: f64 = (locals.var_gf__blk1307 * assign52560_e67770);
        (assign52560_e67771, ((locals.var_gf__blk1307_dn5 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn6 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn7 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn8 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52560_e67770)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52560_e67773;
        locals.var_xgm__blk1409_dn5 = assign52560_e67773_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52560_e67773_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52560_e67773_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52560_e67773_d_n8;

        let assign52570_e67776: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign52570_e67776;

        let (assign52580_e67793, assign52580_e67793_d_n5, assign52580_e67793_d_n6, assign52580_e67793_d_n7, assign52580_e67793_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign52580_e67788: f64 = (locals.var_kp * locals.var_xgm__blk1409);
        let assign52580_e67789: f64 = (1.0 + assign52580_e67788);
        let assign52580_e67790: f64 = (assign52580_e67789).sqrt();
        let assign52580_e67791: f64 = (1.0 / assign52580_e67790);
        (assign52580_e67791, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))),)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign52580_e67793;
        locals.var_eta_p__blk1410_dn5 = assign52580_e67793_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign52580_e67793_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign52580_e67793_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign52580_e67793_d_n8;

        let (assign52590_e67812, assign52590_e67812_d_n5, assign52590_e67812_d_n6, assign52590_e67812_d_n7, assign52590_e67812_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52590_e67805: f64 = (0.25 * locals.var_x_m__blk1404);
        let assign52590_e67806: f64 = (1.0 - assign52590_e67805);
        let assign52590_e67807: f64 = (locals.var_x_m__blk1404 * assign52590_e67806);
        let assign52590_e67808: f64 = (0.3333333333333333 * assign52590_e67807);
        let assign52590_e67809: f64 = (1.0 - assign52590_e67808);
        let assign52590_e67810: f64 = (assign52590_e67809).sqrt();
        (assign52590_e67810, ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8)))))) / (2.0 * assign52590_e67810)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52590_e67812;
        locals.var_temp__blk936_dn5 = assign52590_e67812_d_n5;
        locals.var_temp__blk936_dn6 = assign52590_e67812_d_n6;
        locals.var_temp__blk936_dn7 = assign52590_e67812_d_n7;
        locals.var_temp__blk936_dn8 = assign52590_e67812_d_n8;

        let (assign52600_e67824, assign52600_e67824_d_n5, assign52600_e67824_d_n6, assign52600_e67824_d_n7, assign52600_e67824_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52600_e67821: f64 = (locals.var_x_m__blk1404 * locals.var_temp__blk936);
        let assign52600_e67822: f64 = (0.7071067811865475 * assign52600_e67821);
        (assign52600_e67822, (0.7071067811865475 * ((locals.var_x_m__blk1404_dn5 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn6 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn7 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn8 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn8))),)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign52600_e67824;
        locals.var_sqm__blk1411_dn5 = assign52600_e67824_d_n5;
        locals.var_sqm__blk1411_dn6 = assign52600_e67824_d_n6;
        locals.var_sqm__blk1411_dn7 = assign52600_e67824_d_n7;
        locals.var_sqm__blk1411_dn8 = assign52600_e67824_d_n8;

        let (assign52610_e67850, assign52610_e67850_d_n5, assign52610_e67850_d_n6, assign52610_e67850_d_n7, assign52610_e67850_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign52610_e67836: f64 = (0.5 * locals.var_x_m__blk1404);
        let assign52610_e67837: f64 = (1.0 - assign52610_e67836);
        let assign52610_e67841: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
        let assign52610_e67842: f64 = (0.16666666666666666 * assign52610_e67841);
        let assign52610_e67843: f64 = (assign52610_e67837 + assign52610_e67842);
        let assign52610_e67844: f64 = (locals.var_gf__blk1307 * assign52610_e67843);
        let assign52610_e67846: f64 = (assign52610_e67844 / locals.var_temp__blk936);
        let assign52610_e67847: f64 = (0.7071067811865475 * assign52610_e67846);
        let assign52610_e67848: f64 = (locals.var_eta_p__blk1410 + assign52610_e67847);
        (assign52610_e67848, (locals.var_eta_p__blk1410_dn5 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn5)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn6 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn6)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn7 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn7)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn8 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn8)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))),)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign52610_e67850;
        locals.var_alpha__blk1412_dn5 = assign52610_e67850_d_n5;
        locals.var_alpha__blk1412_dn6 = assign52610_e67850_d_n6;
        locals.var_alpha__blk1412_dn7 = assign52610_e67850_d_n7;
        locals.var_alpha__blk1412_dn8 = assign52610_e67850_d_n8;

        let (assign52620_e67863, assign52620_e67863_d_n5, assign52620_e67863_d_n6, assign52620_e67863_d_n7, assign52620_e67863_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52620_e67859: f64 = (locals.var_x_m__blk1404 - 1.0);
        let assign52620_e67861: f64 = (assign52620_e67859 + locals.var_em__blk1405);
        (assign52620_e67861, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52620_e67863;
        locals.var_pm__blk1408_dn5 = assign52620_e67863_d_n5;
        locals.var_pm__blk1408_dn6 = assign52620_e67863_d_n6;
        locals.var_pm__blk1408_dn7 = assign52620_e67863_d_n7;
        locals.var_pm__blk1408_dn8 = assign52620_e67863_d_n8;

        let (assign52630_e67877, assign52630_e67877_d_n5, assign52630_e67877_d_n6, assign52630_e67877_d_n7, assign52630_e67877_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52630_e67873: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52630_e67874: f64 = (assign52630_e67873).sqrt();
        let assign52630_e67875: f64 = (locals.var_gf__blk1307 * assign52630_e67874);
        (assign52630_e67875, ((locals.var_gf__blk1307_dn5 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn6 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn7 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn8 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52630_e67874)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52630_e67877;
        locals.var_xgm__blk1409_dn5 = assign52630_e67877_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52630_e67877_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52630_e67877_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52630_e67877_d_n8;

        let assign52640_e67880: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign52640_e67880;

        let (assign52650_e67899, assign52650_e67899_d_n5, assign52650_e67899_d_n6, assign52650_e67899_d_n7, assign52650_e67899_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52650_e67891: f64 = (1.0 - locals.var_em__blk1405);
        let assign52650_e67895: f64 = (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324);
        let assign52650_e67896: f64 = (2.0 * assign52650_e67895);
        let assign52650_e67897: f64 = (assign52650_e67891 + assign52650_e67896);
        (assign52650_e67897, ((-locals.var_em__blk1405_dn5) + (2.0 * ((locals.var_xgm__blk1409_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((locals.var_xgm__blk1409_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((locals.var_xgm__blk1409_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((locals.var_xgm__blk1409_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn8)))),)
    } else {
        (locals.var_d0__blk1413, locals.var_d0__blk1413_dn5, locals.var_d0__blk1413_dn6, locals.var_d0__blk1413_dn7, locals.var_d0__blk1413_dn8,)
    }
};
        locals.var_d0__blk1413 = assign52650_e67899;
        locals.var_d0__blk1413_dn5 = assign52650_e67899_d_n5;
        locals.var_d0__blk1413_dn6 = assign52650_e67899_d_n6;
        locals.var_d0__blk1413_dn7 = assign52650_e67899_d_n7;
        locals.var_d0__blk1413_dn8 = assign52650_e67899_d_n8;

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign52660_e67917, assign52660_e67917_d_n5, assign52660_e67917_d_n6, assign52660_e67917_d_n7, assign52660_e67917_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52660_e67912: f64 = (locals.var_kp * locals.var_xgm__blk1409);
        let assign52660_e67913: f64 = (1.0 + assign52660_e67912);
        let assign52660_e67914: f64 = (assign52660_e67913).sqrt();
        let assign52660_e67915: f64 = (1.0 / assign52660_e67914);
        (assign52660_e67915, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))),)
    } else {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    }
};
        locals.var_eta_p__blk1410 = assign52660_e67917;
        locals.var_eta_p__blk1410_dn5 = assign52660_e67917_d_n5;
        locals.var_eta_p__blk1410_dn6 = assign52660_e67917_d_n6;
        locals.var_eta_p__blk1410_dn7 = assign52660_e67917_d_n7;
        locals.var_eta_p__blk1410_dn8 = assign52660_e67917_d_n8;

        let (assign52670_e67932, assign52670_e67932_d_n5, assign52670_e67932_d_n6, assign52670_e67932_d_n7, assign52670_e67932_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52670_e67929: f64 = (locals.var_eta_p__blk1410 + 1.0);
        let assign52670_e67930: f64 = (locals.var_eta_p__blk1410 / assign52670_e67929);
        (assign52670_e67930, (((locals.var_eta_p__blk1410_dn5 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn5)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn6 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn6)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn7 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn7)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn8 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn8)) / (assign52670_e67929 * assign52670_e67929)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign52670_e67932;
        locals.var_temp__blk936_dn5 = assign52670_e67932_d_n5;
        locals.var_temp__blk936_dn6 = assign52670_e67932_d_n6;
        locals.var_temp__blk936_dn7 = assign52670_e67932_d_n7;
        locals.var_temp__blk936_dn8 = assign52670_e67932_d_n8;

        let (assign52680_e67951, assign52680_e67951_d_n5, assign52680_e67951_d_n6, assign52680_e67951_d_n7, assign52680_e67951_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52680_e67944: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign52680_e67946: f64 = (assign52680_e67944 * locals.var_gf2__blk1308);
        let assign52680_e67948: f64 = (assign52680_e67946 * locals.var_dm__blk1407);
        let assign52680_e67949: f64 = (locals.var_kp * assign52680_e67948);
        (assign52680_e67949, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn5)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn6)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn7)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn8)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn8))),)
    } else {
        (locals.var_x_pm__blk1414, locals.var_x_pm__blk1414_dn5, locals.var_x_pm__blk1414_dn6, locals.var_x_pm__blk1414_dn7, locals.var_x_pm__blk1414_dn8,)
    }
};
        locals.var_x_pm__blk1414 = assign52680_e67951;
        locals.var_x_pm__blk1414_dn5 = assign52680_e67951_d_n5;
        locals.var_x_pm__blk1414_dn6 = assign52680_e67951_d_n6;
        locals.var_x_pm__blk1414_dn7 = assign52680_e67951_d_n7;
        locals.var_x_pm__blk1414_dn8 = assign52680_e67951_d_n8;

        let (assign52690_e67974, assign52690_e67974_d_n5, assign52690_e67974_d_n6, assign52690_e67974_d_n7, assign52690_e67974_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52690_e67963: f64 = (locals.var_xgm__blk1409 - locals.var_x_pm__blk1414);
        let assign52690_e67964: f64 = (2.0 * assign52690_e67963);
        let assign52690_e67968: f64 = (1.0 - locals.var_em__blk1405);
        let assign52690_e67970: f64 = (assign52690_e67968 + locals.var_dm__blk1407);
        let assign52690_e67971: f64 = (locals.var_gf2__blk1308 * assign52690_e67970);
        let assign52690_e67972: f64 = (assign52690_e67964 + assign52690_e67971);
        (assign52690_e67972, ((2.0 * (locals.var_xgm__blk1409_dn5 - locals.var_x_pm__blk1414_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn5) + locals.var_dm__blk1407_dn5)))), ((2.0 * (locals.var_xgm__blk1409_dn6 - locals.var_x_pm__blk1414_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn6) + locals.var_dm__blk1407_dn6)))), ((2.0 * (locals.var_xgm__blk1409_dn7 - locals.var_x_pm__blk1414_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn7) + locals.var_dm__blk1407_dn7)))), ((2.0 * (locals.var_xgm__blk1409_dn8 - locals.var_x_pm__blk1414_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn8) + locals.var_dm__blk1407_dn8)))),)
    } else {
        (locals.var_p_pd__blk1415, locals.var_p_pd__blk1415_dn5, locals.var_p_pd__blk1415_dn6, locals.var_p_pd__blk1415_dn7, locals.var_p_pd__blk1415_dn8,)
    }
};
        locals.var_p_pd__blk1415 = assign52690_e67974;
        locals.var_p_pd__blk1415_dn5 = assign52690_e67974_d_n5;
        locals.var_p_pd__blk1415_dn6 = assign52690_e67974_d_n6;
        locals.var_p_pd__blk1415_dn7 = assign52690_e67974_d_n7;
        locals.var_p_pd__blk1415_dn8 = assign52690_e67974_d_n8;

        let (assign52700_e67991, assign52700_e67991_d_n5, assign52700_e67991_d_n6, assign52700_e67991_d_n7, assign52700_e67991_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52700_e67987: f64 = (2.0 * locals.var_xgm__blk1409);
        let assign52700_e67988: f64 = (locals.var_x_pm__blk1414 - assign52700_e67987);
        let assign52700_e67989: f64 = (locals.var_x_pm__blk1414 * assign52700_e67988);
        (assign52700_e67989, ((locals.var_x_pm__blk1414_dn5 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn5 - (2.0 * locals.var_xgm__blk1409_dn5)))), ((locals.var_x_pm__blk1414_dn6 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn6 - (2.0 * locals.var_xgm__blk1409_dn6)))), ((locals.var_x_pm__blk1414_dn7 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn7 - (2.0 * locals.var_xgm__blk1409_dn7)))), ((locals.var_x_pm__blk1414_dn8 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn8 - (2.0 * locals.var_xgm__blk1409_dn8)))),)
    } else {
        (locals.var_q_pd__blk1416, locals.var_q_pd__blk1416_dn5, locals.var_q_pd__blk1416_dn6, locals.var_q_pd__blk1416_dn7, locals.var_q_pd__blk1416_dn8,)
    }
};
        locals.var_q_pd__blk1416 = assign52700_e67991;
        locals.var_q_pd__blk1416_dn5 = assign52700_e67991_d_n5;
        locals.var_q_pd__blk1416_dn6 = assign52700_e67991_d_n6;
        locals.var_q_pd__blk1416_dn7 = assign52700_e67991_d_n7;
        locals.var_q_pd__blk1416_dn8 = assign52700_e67991_d_n8;

        let (assign52710_e68010, assign52710_e68010_d_n5, assign52710_e68010_d_n6, assign52710_e68010_d_n7, assign52710_e68010_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52710_e68005: f64 = (locals.var_em__blk1405 + locals.var_dm__blk1407);
        let assign52710_e68006: f64 = (locals.var_gf2__blk1308 * assign52710_e68005);
        let assign52710_e68007: f64 = (0.5 * assign52710_e68006);
        let assign52710_e68008: f64 = (1.0 - assign52710_e68007);
        (assign52710_e68008, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn5 + locals.var_dm__blk1407_dn5))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn6 + locals.var_dm__blk1407_dn6))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn7 + locals.var_dm__blk1407_dn7))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn8 + locals.var_dm__blk1407_dn8))))),)
    } else {
        (locals.var_xi_pd__blk1417, locals.var_xi_pd__blk1417_dn5, locals.var_xi_pd__blk1417_dn6, locals.var_xi_pd__blk1417_dn7, locals.var_xi_pd__blk1417_dn8,)
    }
};
        locals.var_xi_pd__blk1417 = assign52710_e68010;
        locals.var_xi_pd__blk1417_dn5 = assign52710_e68010_d_n5;
        locals.var_xi_pd__blk1417_dn6 = assign52710_e68010_d_n6;
        locals.var_xi_pd__blk1417_dn7 = assign52710_e68010_d_n7;
        locals.var_xi_pd__blk1417_dn8 = assign52710_e68010_d_n8;

        let (assign52720_e68031, assign52720_e68031_d_n5, assign52720_e68031_d_n6, assign52720_e68031_d_n7, assign52720_e68031_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52720_e68021: f64 = (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415);
        let assign52720_e68024: f64 = (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415);
        let assign52720_e68027: f64 = (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416);
        let assign52720_e68028: f64 = (assign52720_e68024 - assign52720_e68027);
        let assign52720_e68029: f64 = (assign52720_e68021 / assign52720_e68028);
        (assign52720_e68029, (((((locals.var_q_pd__blk1416_dn5 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn5)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn5 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn5)) - ((locals.var_xi_pd__blk1417_dn5 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn5))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn6 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn6)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn6 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn6)) - ((locals.var_xi_pd__blk1417_dn6 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn6))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn7 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn7)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn7 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn7)) - ((locals.var_xi_pd__blk1417_dn7 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn7))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn8 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn8)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn8 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn8)) - ((locals.var_xi_pd__blk1417_dn8 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn8))))) / (assign52720_e68028 * assign52720_e68028)),)
    } else {
        (locals.var_u_pd__blk1418, locals.var_u_pd__blk1418_dn5, locals.var_u_pd__blk1418_dn6, locals.var_u_pd__blk1418_dn7, locals.var_u_pd__blk1418_dn8,)
    }
};
        locals.var_u_pd__blk1418 = assign52720_e68031;
        locals.var_u_pd__blk1418_dn5 = assign52720_e68031_d_n5;
        locals.var_u_pd__blk1418_dn6 = assign52720_e68031_d_n6;
        locals.var_u_pd__blk1418_dn7 = assign52720_e68031_d_n7;
        locals.var_u_pd__blk1418_dn8 = assign52720_e68031_d_n8;

        let (assign52730_e68044, assign52730_e68044_d_n5, assign52730_e68044_d_n6, assign52730_e68044_d_n7, assign52730_e68044_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52730_e68042: f64 = (locals.var_x_m__blk1404 + locals.var_u_pd__blk1418);
        (assign52730_e68042, (locals.var_x_m__blk1404_dn5 + locals.var_u_pd__blk1418_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_u_pd__blk1418_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_u_pd__blk1418_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_u_pd__blk1418_dn8),)
    } else {
        (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8,)
    }
};
        locals.var_x_m__blk1404 = assign52730_e68044;
        locals.var_x_m__blk1404_dn5 = assign52730_e68044_d_n5;
        locals.var_x_m__blk1404_dn6 = assign52730_e68044_d_n6;
        locals.var_x_m__blk1404_dn7 = assign52730_e68044_d_n7;
        locals.var_x_m__blk1404_dn8 = assign52730_e68044_d_n8;

        let (assign52740_e68056, assign52740_e68056_d_n5, assign52740_e68056_d_n6, assign52740_e68056_d_n7, assign52740_e68056_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52740_e68054: f64 = (locals.var_u_pd__blk1418).exp();
        (assign52740_e68054, (assign52740_e68054 * locals.var_u_pd__blk1418_dn5), (assign52740_e68054 * locals.var_u_pd__blk1418_dn6), (assign52740_e68054 * locals.var_u_pd__blk1418_dn7), (assign52740_e68054 * locals.var_u_pd__blk1418_dn8),)
    } else {
        (locals.var_km__blk1419, locals.var_km__blk1419_dn5, locals.var_km__blk1419_dn6, locals.var_km__blk1419_dn7, locals.var_km__blk1419_dn8,)
    }
};
        locals.var_km__blk1419 = assign52740_e68056;
        locals.var_km__blk1419_dn5 = assign52740_e68056_d_n5;
        locals.var_km__blk1419_dn6 = assign52740_e68056_d_n6;
        locals.var_km__blk1419_dn7 = assign52740_e68056_d_n7;
        locals.var_km__blk1419_dn8 = assign52740_e68056_d_n8;

        let (assign52750_e68069, assign52750_e68069_d_n5, assign52750_e68069_d_n6, assign52750_e68069_d_n7, assign52750_e68069_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52750_e68067: f64 = (locals.var_em__blk1405 / locals.var_km__blk1419);
        (assign52750_e68067, (((locals.var_em__blk1405_dn5 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn5)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn6 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn6)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn7 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn7)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn8 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn8)) / (locals.var_km__blk1419 * locals.var_km__blk1419)),)
    } else {
        (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8,)
    }
};
        locals.var_em__blk1405 = assign52750_e68069;
        locals.var_em__blk1405_dn5 = assign52750_e68069_d_n5;
        locals.var_em__blk1405_dn6 = assign52750_e68069_d_n6;
        locals.var_em__blk1405_dn7 = assign52750_e68069_d_n7;
        locals.var_em__blk1405_dn8 = assign52750_e68069_d_n8;

        let (assign52760_e68082, assign52760_e68082_d_n5, assign52760_e68082_d_n6, assign52760_e68082_d_n7, assign52760_e68082_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52760_e68080: f64 = (locals.var_dm__blk1407 * locals.var_km__blk1419);
        (assign52760_e68080, ((locals.var_dm__blk1407_dn5 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn5)), ((locals.var_dm__blk1407_dn6 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn6)), ((locals.var_dm__blk1407_dn7 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn7)), ((locals.var_dm__blk1407_dn8 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn8)),)
    } else {
        (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8,)
    }
};
        locals.var_dm__blk1407 = assign52760_e68082;
        locals.var_dm__blk1407_dn5 = assign52760_e68082_d_n5;
        locals.var_dm__blk1407_dn6 = assign52760_e68082_d_n6;
        locals.var_dm__blk1407_dn7 = assign52760_e68082_d_n7;
        locals.var_dm__blk1407_dn8 = assign52760_e68082_d_n8;

        let (assign52770_e68097, assign52770_e68097_d_n5, assign52770_e68097_d_n6, assign52770_e68097_d_n7, assign52770_e68097_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52770_e68093: f64 = (locals.var_x_m__blk1404 - 1.0);
        let assign52770_e68095: f64 = (assign52770_e68093 + locals.var_em__blk1405);
        (assign52770_e68095, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8),)
    } else {
        (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8,)
    }
};
        locals.var_pm__blk1408 = assign52770_e68097;
        locals.var_pm__blk1408_dn5 = assign52770_e68097_d_n5;
        locals.var_pm__blk1408_dn6 = assign52770_e68097_d_n6;
        locals.var_pm__blk1408_dn7 = assign52770_e68097_d_n7;
        locals.var_pm__blk1408_dn8 = assign52770_e68097_d_n8;

        let (assign52780_e68113, assign52780_e68113_d_n5, assign52780_e68113_d_n6, assign52780_e68113_d_n7, assign52780_e68113_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52780_e68109: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
        let assign52780_e68110: f64 = (assign52780_e68109).sqrt();
        let assign52780_e68111: f64 = (locals.var_gf__blk1307 * assign52780_e68110);
        (assign52780_e68111, ((locals.var_gf__blk1307_dn5 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn6 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn7 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn8 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52780_e68110)))),)
    } else {
        (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8,)
    }
};
        locals.var_xgm__blk1409 = assign52780_e68113;
        locals.var_xgm__blk1409_dn5 = assign52780_e68113_d_n5;
        locals.var_xgm__blk1409_dn6 = assign52780_e68113_d_n6;
        locals.var_xgm__blk1409_dn7 = assign52780_e68113_d_n7;
        locals.var_xgm__blk1409_dn8 = assign52780_e68113_d_n8;

        let (assign52790_e68134, assign52790_e68134_d_n5, assign52790_e68134_d_n6, assign52790_e68134_d_n7, assign52790_e68134_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52790_e68124: f64 = (1.0 - locals.var_em__blk1405);
        let assign52790_e68128: f64 = (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410);
        let assign52790_e68130: f64 = (assign52790_e68128 * locals.var_inv_gf2__blk1324);
        let assign52790_e68131: f64 = (2.0 * assign52790_e68130);
        let assign52790_e68132: f64 = (assign52790_e68124 + assign52790_e68131);
        (assign52790_e68132, ((-locals.var_em__blk1405_dn5) + (2.0 * ((((locals.var_xgm__blk1409_dn5 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn5)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((((locals.var_xgm__blk1409_dn6 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn6)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((((locals.var_xgm__blk1409_dn7 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn7)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((((locals.var_xgm__blk1409_dn8 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn8)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn8)))),)
    } else {
        (locals.var_km0__blk1420, locals.var_km0__blk1420_dn5, locals.var_km0__blk1420_dn6, locals.var_km0__blk1420_dn7, locals.var_km0__blk1420_dn8,)
    }
};
        locals.var_km0__blk1420 = assign52790_e68134;
        locals.var_km0__blk1420_dn5 = assign52790_e68134_d_n5;
        locals.var_km0__blk1420_dn6 = assign52790_e68134_d_n6;
        locals.var_km0__blk1420_dn7 = assign52790_e68134_d_n7;
        locals.var_km0__blk1420_dn8 = assign52790_e68134_d_n8;

        let (assign52800_e68157, assign52800_e68157_d_n5, assign52800_e68157_d_n6, assign52800_e68157_d_n7, assign52800_e68157_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52800_e68145: f64 = (locals.var_x_ds__blk1394 * locals.var_km__blk1419);
        let assign52800_e68148: f64 = (locals.var_d0__blk1413 + locals.var_d_bar__blk1406);
        let assign52800_e68149: f64 = (assign52800_e68145 * assign52800_e68148);
        let assign52800_e68153: f64 = (locals.var_km__blk1419 * locals.var_d_bar__blk1406);
        let assign52800_e68154: f64 = (locals.var_km0__blk1420 + assign52800_e68153);
        let assign52800_e68155: f64 = (assign52800_e68149 / assign52800_e68154);
        (assign52800_e68155, (((((((locals.var_x_ds__blk1394_dn5 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn5)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn5 + locals.var_d_bar__blk1406_dn5))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn5 + ((locals.var_km__blk1419_dn5 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn5))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn6 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn6)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn6 + locals.var_d_bar__blk1406_dn6))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn6 + ((locals.var_km__blk1419_dn6 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn6))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn7 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn7)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn7 + locals.var_d_bar__blk1406_dn7))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn7 + ((locals.var_km__blk1419_dn7 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn7))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn8 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn8)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn8 + locals.var_d_bar__blk1406_dn8))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn8 + ((locals.var_km__blk1419_dn8 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn8))))) / (assign52800_e68154 * assign52800_e68154)),)
    } else {
        (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8,)
    }
};
        locals.var_x_ds__blk1394 = assign52800_e68157;
        locals.var_x_ds__blk1394_dn5 = assign52800_e68157_d_n5;
        locals.var_x_ds__blk1394_dn6 = assign52800_e68157_d_n6;
        locals.var_x_ds__blk1394_dn7 = assign52800_e68157_d_n7;
        locals.var_x_ds__blk1394_dn8 = assign52800_e68157_d_n8;

        let (assign52810_e68170, assign52810_e68170_d_n5, assign52810_e68170_d_n6, assign52810_e68170_d_n7, assign52810_e68170_d_n8,) = {
    if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign52810_e68168: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
        (assign52810_e68168, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    }
};
        locals.var_dps__blk1397 = assign52810_e68170;
        locals.var_dps__blk1397_dn5 = assign52810_e68170_d_n5;
        locals.var_dps__blk1397_dn6 = assign52810_e68170_d_n6;
        locals.var_dps__blk1397_dn7 = assign52810_e68170_d_n7;
        locals.var_dps__blk1397_dn8 = assign52810_e68170_d_n8;

        let (assign52820_e68180, assign52820_e68180_d_n5, assign52820_e68180_d_n6, assign52820_e68180_d_n7, assign52820_e68180_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52820_e68178: f64 = (locals.var_pm__blk1408).sqrt();
        (assign52820_e68178, (locals.var_pm__blk1408_dn5 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn6 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn7 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn8 / (2.0 * assign52820_e68178)),)
    } else {
        (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8,)
    }
};
        locals.var_sqm__blk1411 = assign52820_e68180;
        locals.var_sqm__blk1411_dn5 = assign52820_e68180_d_n5;
        locals.var_sqm__blk1411_dn6 = assign52820_e68180_d_n6;
        locals.var_sqm__blk1411_dn7 = assign52820_e68180_d_n7;
        locals.var_sqm__blk1411_dn8 = assign52820_e68180_d_n8;

        let (assign52830_e68199, assign52830_e68199_d_n5, assign52830_e68199_d_n6, assign52830_e68199_d_n7, assign52830_e68199_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        let assign52830_e68192: f64 = (1.0 - locals.var_em__blk1405);
        let assign52830_e68193: f64 = (locals.var_gf__blk1307 * assign52830_e68192);
        let assign52830_e68195: f64 = (assign52830_e68193 / locals.var_sqm__blk1411);
        let assign52830_e68196: f64 = (0.5 * assign52830_e68195);
        let assign52830_e68197: f64 = (locals.var_eta_p__blk1410 + assign52830_e68196);
        (assign52830_e68197, (locals.var_eta_p__blk1410_dn5 + (0.5 * (((((locals.var_gf__blk1307_dn5 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn5))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn5)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn6 + (0.5 * (((((locals.var_gf__blk1307_dn6 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn6))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn6)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn7 + (0.5 * (((((locals.var_gf__blk1307_dn7 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn7))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn7)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn8 + (0.5 * (((((locals.var_gf__blk1307_dn8 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn8))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn8)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))),)
    } else {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    }
};
        locals.var_alpha__blk1412 = assign52830_e68199;
        locals.var_alpha__blk1412_dn5 = assign52830_e68199_d_n5;
        locals.var_alpha__blk1412_dn6 = assign52830_e68199_d_n6;
        locals.var_alpha__blk1412_dn7 = assign52830_e68199_d_n7;
        locals.var_alpha__blk1412_dn8 = assign52830_e68199_d_n8;

        let (assign52840_e68215, assign52840_e68215_d_n5, assign52840_e68215_d_n6, assign52840_e68215_d_n7, assign52840_e68215_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52840_e68206: f64 = (locals.var_gf2__blk1308 * locals.var_dm__blk1407);
        let assign52840_e68210: f64 = (locals.var_gf__blk1307 * locals.var_sqm__blk1411);
        let assign52840_e68211: f64 = (locals.var_xgm__blk1409 + assign52840_e68210);
        let assign52840_e68212: f64 = (assign52840_e68206 / assign52840_e68211);
        let assign52840_e68213: f64 = (locals.var_phit1__blk1322 * assign52840_e68212);
        (assign52840_e68213, ((locals.var_phit1__blk1322_dn5 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn5 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn5)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn5))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn6 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn6 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn6)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn6))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn7 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn7 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn7)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn7))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn8 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn8 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn8)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn8))))) / (assign52840_e68211 * assign52840_e68211)))),)
    } else {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    }
};
        locals.var_qim__blk1421 = assign52840_e68215;
        locals.var_qim__blk1421_dn5 = assign52840_e68215_d_n5;
        locals.var_qim__blk1421_dn6 = assign52840_e68215_d_n6;
        locals.var_qim__blk1421_dn7 = assign52840_e68215_d_n7;
        locals.var_qim__blk1421_dn8 = assign52840_e68215_d_n8;

        let (assign52850_e68225, assign52850_e68225_d_n5, assign52850_e68225_d_n6, assign52850_e68225_d_n7, assign52850_e68225_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52850_e68222: f64 = (locals.var_phit1__blk1322 * locals.var_alpha__blk1412);
        let assign52850_e68223: f64 = (locals.var_qim__blk1421 + assign52850_e68222);
        (assign52850_e68223, (locals.var_qim__blk1421_dn5 + ((locals.var_phit1__blk1322_dn5 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn5))), (locals.var_qim__blk1421_dn6 + ((locals.var_phit1__blk1322_dn6 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn6))), (locals.var_qim__blk1421_dn7 + ((locals.var_phit1__blk1322_dn7 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn7))), (locals.var_qim__blk1421_dn8 + ((locals.var_phit1__blk1322_dn8 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn8))),)
    } else {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    }
};
        locals.var_qim1__blk1422 = assign52850_e68225;
        locals.var_qim1__blk1422_dn5 = assign52850_e68225_d_n5;
        locals.var_qim1__blk1422_dn6 = assign52850_e68225_d_n6;
        locals.var_qim1__blk1422_dn7 = assign52850_e68225_d_n7;
        locals.var_qim1__blk1422_dn8 = assign52850_e68225_d_n8;

        let (assign52860_e68235, assign52860_e68235_d_n5, assign52860_e68235_d_n6, assign52860_e68235_d_n7, assign52860_e68235_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52860_e68231: f64 = (locals.var_sqm__blk1411 * locals.var_gf__blk1307);
        let assign52860_e68233: f64 = (assign52860_e68231 * locals.var_phit1__blk1322);
        (assign52860_e68233, ((((locals.var_sqm__blk1411_dn5 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqm__blk1411_dn6 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqm__blk1411_dn7 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqm__blk1411_dn8 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8,)
    }
};
        locals.var_qbm__blk1423 = assign52860_e68235;
        locals.var_qbm__blk1423_dn5 = assign52860_e68235_d_n5;
        locals.var_qbm__blk1423_dn6 = assign52860_e68235_d_n6;
        locals.var_qbm__blk1423_dn7 = assign52860_e68235_d_n7;
        locals.var_qbm__blk1423_dn8 = assign52860_e68235_d_n8;

        let assign52870_e68238: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign52870_e68238;

        let (assign52880_e68250, assign52880_e68250_d_n5, assign52880_e68250_d_n6, assign52880_e68250_d_n7, assign52880_e68250_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign52880_e68247: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
        let assign52880_e68248: f64 = (1.0 - assign52880_e68247);
        (assign52880_e68248, (-(locals.var_rsg_i * locals.var_qim__blk1421_dn5)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn6)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn7)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign52880_e68250;
        locals.var_rhog__blk1362_dn5 = assign52880_e68250_d_n5;
        locals.var_rhog__blk1362_dn6 = assign52880_e68250_d_n6;
        locals.var_rhog__blk1362_dn7 = assign52880_e68250_d_n7;
        locals.var_rhog__blk1362_dn8 = assign52880_e68250_d_n8;

        let (assign52890_e68265, assign52890_e68265_d_n5, assign52890_e68265_d_n6, assign52890_e68265_d_n7, assign52890_e68265_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        let assign52890_e68261: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
        let assign52890_e68262: f64 = (1.0 + assign52890_e68261);
        let assign52890_e68263: f64 = (1.0 / assign52890_e68262);
        (assign52890_e68263, (-((locals.var_rsg_i * locals.var_qim__blk1421_dn5) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn6) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn7) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn8) / (assign52890_e68262 * assign52890_e68262))),)
    } else {
        (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8,)
    }
};
        locals.var_rhog__blk1362 = assign52890_e68265;
        locals.var_rhog__blk1362_dn5 = assign52890_e68265_d_n5;
        locals.var_rhog__blk1362_dn6 = assign52890_e68265_d_n6;
        locals.var_rhog__blk1362_dn7 = assign52890_e68265_d_n7;
        locals.var_rhog__blk1362_dn8 = assign52890_e68265_d_n8;

        let (assign52900_e68277, assign52900_e68277_d_n5, assign52900_e68277_d_n6, assign52900_e68277_d_n7, assign52900_e68277_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52900_e68271: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
        let assign52900_e68273: f64 = (assign52900_e68271 * locals.var_rhog__blk1362);
        let assign52900_e68275: f64 = (assign52900_e68273 * locals.var_qim__blk1421);
        (assign52900_e68275, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn5)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn6)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn7)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn8)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8,)
    }
};
        locals.var_gr__blk1363 = assign52900_e68277;
        locals.var_gr__blk1363_dn5 = assign52900_e68277_d_n5;
        locals.var_gr__blk1363_dn6 = assign52900_e68277_d_n6;
        locals.var_gr__blk1363_dn7 = assign52900_e68277_d_n7;
        locals.var_gr__blk1363_dn8 = assign52900_e68277_d_n8;

        let (assign52910_e68287, assign52910_e68287_d_n5, assign52910_e68287_d_n6, assign52910_e68287_d_n7, assign52910_e68287_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52910_e68284: f64 = (locals.var_eta_mu * locals.var_qim__blk1421);
        let assign52910_e68285: f64 = (locals.var_qbm__blk1423 + assign52910_e68284);
        (assign52910_e68285, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_qeff__blk1424, locals.var_qeff__blk1424_dn5, locals.var_qeff__blk1424_dn6, locals.var_qeff__blk1424_dn7, locals.var_qeff__blk1424_dn8,)
    }
};
        locals.var_qeff__blk1424 = assign52910_e68287;
        locals.var_qeff__blk1424_dn5 = assign52910_e68287_d_n5;
        locals.var_qeff__blk1424_dn6 = assign52910_e68287_d_n6;
        locals.var_qeff__blk1424_dn7 = assign52910_e68287_d_n7;
        locals.var_qeff__blk1424_dn8 = assign52910_e68287_d_n8;

        let (assign52920_e68297, assign52920_e68297_d_n5, assign52920_e68297_d_n6, assign52920_e68297_d_n7, assign52920_e68297_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52920_e68294: f64 = (locals.var_eta_mu1 * locals.var_qim__blk1421);
        let assign52920_e68295: f64 = (locals.var_qbm__blk1423 + assign52920_e68294);
        (assign52920_e68295, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn8)),)
    } else {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    }
};
        locals.var_qeff1__blk1425 = assign52920_e68297;
        locals.var_qeff1__blk1425_dn5 = assign52920_e68297_d_n5;
        locals.var_qeff1__blk1425_dn6 = assign52920_e68297_d_n6;
        locals.var_qeff1__blk1425_dn7 = assign52920_e68297_d_n7;
        locals.var_qeff1__blk1425_dn8 = assign52920_e68297_d_n8;

        let (assign52930_e68305, assign52930_e68305_d_n5, assign52930_e68305_d_n6, assign52930_e68305_d_n7, assign52930_e68305_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52930_e68303: f64 = (locals.var_e_eff0 * locals.var_qeff__blk1424);
        (assign52930_e68303, (locals.var_e_eff0 * locals.var_qeff__blk1424_dn5), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn6), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn7), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn8),)
    } else {
        (locals.var_eeffm__blk1426, locals.var_eeffm__blk1426_dn5, locals.var_eeffm__blk1426_dn6, locals.var_eeffm__blk1426_dn7, locals.var_eeffm__blk1426_dn8,)
    }
};
        locals.var_eeffm__blk1426 = assign52930_e68305;
        locals.var_eeffm__blk1426_dn5 = assign52930_e68305_d_n5;
        locals.var_eeffm__blk1426_dn6 = assign52930_e68305_d_n6;
        locals.var_eeffm__blk1426_dn7 = assign52930_e68305_d_n7;
        locals.var_eeffm__blk1426_dn8 = assign52930_e68305_d_n8;

        let (assign52940_e68318, assign52940_e68318_d_n5, assign52940_e68318_d_n6, assign52940_e68318_d_n7, assign52940_e68318_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52940_e68312: f64 = (locals.var_pm__blk1408 + locals.var_dm__blk1407);
        let assign52940_e68314: f64 = (assign52940_e68312 + 1e-14);
        let assign52940_e68315: f64 = (locals.var_pm__blk1408 / assign52940_e68314);
        let assign52940_e68316: f64 = (assign52940_e68315).ln();
        (assign52940_e68316, ((((locals.var_pm__blk1408_dn5 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn5 + locals.var_dm__blk1407_dn5))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn6 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn6 + locals.var_dm__blk1407_dn6))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn7 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn7 + locals.var_dm__blk1407_dn7))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn8 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn8 + locals.var_dm__blk1407_dn8))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign52940_e68318;
        locals.var_temp1_dn5 = assign52940_e68318_d_n5;
        locals.var_temp1_dn6 = assign52940_e68318_d_n6;
        locals.var_temp1_dn7 = assign52940_e68318_d_n7;
        locals.var_temp1_dn8 = assign52940_e68318_d_n8;

        let (assign52950_e68337, assign52950_e68337_d_n5, assign52950_e68337_d_n6, assign52950_e68337_d_n7, assign52950_e68337_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52950_e68324: f64 = (locals.var_eeffm__blk1426 * locals.var_mue_t);
        let assign52950_e68326: f64 = (assign52950_e68324).powf(locals.var_themu_t);
        let assign52950_e68330: f64 = (0.5 * locals.var_thecs_t);
        let assign52950_e68332: f64 = (assign52950_e68330 * locals.var_temp1);
        let assign52950_e68333: f64 = (assign52950_e68332).exp();
        let assign52950_e68334: f64 = (locals.var_cs_t * assign52950_e68333);
        let assign52950_e68335: f64 = (assign52950_e68326 + assign52950_e68334);
        (assign52950_e68335, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn5 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn5 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn6 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn6 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn7 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn7 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn8 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn8 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn8)))),)
    } else {
        (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8,)
    }
};
        locals.var_mutmp__blk1365 = assign52950_e68337;
        locals.var_mutmp__blk1365_dn5 = assign52950_e68337_d_n5;
        locals.var_mutmp__blk1365_dn6 = assign52950_e68337_d_n6;
        locals.var_mutmp__blk1365_dn7 = assign52950_e68337_d_n7;
        locals.var_mutmp__blk1365_dn8 = assign52950_e68337_d_n8;

        let (assign52960_e68349, assign52960_e68349_d_n5, assign52960_e68349_d_n6, assign52960_e68349_d_n7, assign52960_e68349_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52960_e68343: f64 = (1.0 + locals.var_mutmp__blk1365);
        let assign52960_e68345: f64 = (assign52960_e68343 + locals.var_gr__blk1363);
        let assign52960_e68347: f64 = (assign52960_e68345 * locals.var_rxcor__blk1357);
        (assign52960_e68347, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn8)),)
    } else {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    }
};
        locals.var_gmob__blk1427 = assign52960_e68349;
        locals.var_gmob__blk1427_dn5 = assign52960_e68349_d_n5;
        locals.var_gmob__blk1427_dn6 = assign52960_e68349_d_n6;
        locals.var_gmob__blk1427_dn7 = assign52960_e68349_d_n7;
        locals.var_gmob__blk1427_dn8 = assign52960_e68349_d_n8;

        let (assign52970_e68370, assign52970_e68370_d_n5, assign52970_e68370_d_n6, assign52970_e68370_d_n7, assign52970_e68370_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52970_e68356: f64 = (locals.var_v_ds - locals.var_dps__blk1397);
        let assign52970_e68358: f64 = (assign52970_e68356 * locals.var_inv_vp);
        let assign52970_e68359: f64 = (1.0 + assign52970_e68358);
        let assign52970_e68363: f64 = (locals.var_vdse__blk1388 - locals.var_dps__blk1397);
        let assign52970_e68365: f64 = (assign52970_e68363 * locals.var_inv_vp);
        let assign52970_e68366: f64 = (1.0 + assign52970_e68365);
        let assign52970_e68367: f64 = (assign52970_e68359 / assign52970_e68366);
        let assign52970_e68368: f64 = (assign52970_e68367).ln();
        (assign52970_e68368, ((((((-locals.var_dps__blk1397_dn5) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn5 - locals.var_dps__blk1397_dn5) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((-locals.var_dps__blk1397_dn8) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn8 - locals.var_dps__blk1397_dn8) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367),)
    } else {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    }
};
        locals.var_s1__blk1428 = assign52970_e68370;
        locals.var_s1__blk1428_dn5 = assign52970_e68370_d_n5;
        locals.var_s1__blk1428_dn6 = assign52970_e68370_d_n6;
        locals.var_s1__blk1428_dn7 = assign52970_e68370_d_n7;
        locals.var_s1__blk1428_dn8 = assign52970_e68370_d_n8;

    }

    pub(super) fn stamp_transient_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign52980_e68378, assign52980_e68378_d_n5, assign52980_e68378_d_n6, assign52980_e68378_d_n7, assign52980_e68378_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52980_e68376: f64 = (locals.var_qim__blk1421 * locals.var_xitsb__blk1367);
        (assign52980_e68376, ((locals.var_qim__blk1421_dn5 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qim__blk1421_dn6 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qim__blk1421_dn7 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qim__blk1421_dn8 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign52980_e68378;
        locals.var_temp2_dn5 = assign52980_e68378_d_n5;
        locals.var_temp2_dn6 = assign52980_e68378_d_n6;
        locals.var_temp2_dn7 = assign52980_e68378_d_n7;
        locals.var_temp2_dn8 = assign52980_e68378_d_n8;

        let (assign52990_e68388, assign52990_e68388_d_n5, assign52990_e68388_d_n6, assign52990_e68388_d_n7, assign52990_e68388_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign52990_e68385: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign52990_e68386: f64 = (locals.var_temp2 / assign52990_e68385);
        (assign52990_e68386, (((locals.var_temp2_dn5 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn6 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn7 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn8 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign52990_e68385 * assign52990_e68385)),)
    } else {
        (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8,)
    }
};
        locals.var_wsat__blk1368 = assign52990_e68388;
        locals.var_wsat__blk1368_dn5 = assign52990_e68388_d_n5;
        locals.var_wsat__blk1368_dn6 = assign52990_e68388_d_n6;
        locals.var_wsat__blk1368_dn7 = assign52990_e68388_d_n7;
        locals.var_wsat__blk1368_dn8 = assign52990_e68388_d_n8;

        let assign53000_e68391: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign53000_e68391;

        let (assign53010_e68405, assign53010_e68405_d_n5, assign53010_e68405_d_n6, assign53010_e68405_d_n7, assign53010_e68405_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign53010_e68401: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign53010_e68402: f64 = (1.0 - assign53010_e68401);
        let assign53010_e68403: f64 = (1.0 / assign53010_e68402);
        (assign53010_e68403, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign53010_e68402 * assign53010_e68402))),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign53010_e68405;
        locals.var_factheta__blk1369_dn5 = assign53010_e68405_d_n5;
        locals.var_factheta__blk1369_dn6 = assign53010_e68405_d_n6;
        locals.var_factheta__blk1369_dn7 = assign53010_e68405_d_n7;
        locals.var_factheta__blk1369_dn8 = assign53010_e68405_d_n8;

        let (assign53020_e68418, assign53020_e68418_d_n5, assign53020_e68418_d_n6, assign53020_e68418_d_n7, assign53020_e68418_d_n8,) = {
    if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign53020_e68415: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
        let assign53020_e68416: f64 = (1.0 + assign53020_e68415);
        (assign53020_e68416, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8),)
    } else {
        (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8,)
    }
};
        locals.var_factheta__blk1369 = assign53020_e68418;
        locals.var_factheta__blk1369_dn5 = assign53020_e68418_d_n5;
        locals.var_factheta__blk1369_dn6 = assign53020_e68418_d_n6;
        locals.var_factheta__blk1369_dn7 = assign53020_e68418_d_n7;
        locals.var_factheta__blk1369_dn8 = assign53020_e68418_d_n8;

        let (assign53030_e68426, assign53030_e68426_d_n5, assign53030_e68426_d_n6, assign53030_e68426_d_n7, assign53030_e68426_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign53030_e68424: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
        (assign53030_e68424, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8),)
    } else {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    }
};
        locals.var_thesateff__blk1430 = assign53030_e68426;
        locals.var_thesateff__blk1430_dn5 = assign53030_e68426_d_n5;
        locals.var_thesateff__blk1430_dn6 = assign53030_e68426_d_n6;
        locals.var_thesateff__blk1430_dn7 = assign53030_e68426_d_n7;
        locals.var_thesateff__blk1430_dn8 = assign53030_e68426_d_n8;

        let (assign53040_e68434, assign53040_e68434_d_n5, assign53040_e68434_d_n6, assign53040_e68434_d_n7, assign53040_e68434_d_n8,) = {
    if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
        let assign53040_e68432: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
        (assign53040_e68432, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)),)
    } else {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    }
};
        locals.var_voxm__blk1429 = assign53040_e68434;
        locals.var_voxm__blk1429_dn5 = assign53040_e68434_d_n5;
        locals.var_voxm__blk1429_dn6 = assign53040_e68434_d_n6;
        locals.var_voxm__blk1429_dn7 = assign53040_e68434_d_n7;
        locals.var_voxm__blk1429_dn8 = assign53040_e68434_d_n8;

        let (assign53050_e68438, assign53050_e68438_d_n5, assign53050_e68438_d_n6, assign53050_e68438_d_n7, assign53050_e68438_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    }
};
        locals.var_vgb1_ac = assign53050_e68438;
        locals.var_vgb1_ac_dn5 = assign53050_e68438_d_n5;
        locals.var_vgb1_ac_dn6 = assign53050_e68438_d_n6;
        locals.var_vgb1_ac_dn7 = assign53050_e68438_d_n7;
        locals.var_vgb1_ac_dn8 = assign53050_e68438_d_n8;

        let (assign53060_e68442, assign53060_e68442_d_n5, assign53060_e68442_d_n6, assign53060_e68442_d_n7, assign53060_e68442_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8,)
    }
};
        locals.var_phit1_ac = assign53060_e68442;
        locals.var_phit1_ac_dn5 = assign53060_e68442_d_n5;
        locals.var_phit1_ac_dn6 = assign53060_e68442_d_n6;
        locals.var_phit1_ac_dn7 = assign53060_e68442_d_n7;
        locals.var_phit1_ac_dn8 = assign53060_e68442_d_n8;

        let (assign53070_e68446, assign53070_e68446_d_n5, assign53070_e68446_d_n6, assign53070_e68446_d_n7, assign53070_e68446_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8,)
    }
};
        locals.var_gf_ac = assign53070_e68446;
        locals.var_gf_ac_dn5 = assign53070_e68446_d_n5;
        locals.var_gf_ac_dn6 = assign53070_e68446_d_n6;
        locals.var_gf_ac_dn7 = assign53070_e68446_d_n7;
        locals.var_gf_ac_dn8 = assign53070_e68446_d_n8;

        let (assign53080_e68450,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_xg__blk1326,)
    } else {
        (locals.var_xg_ac,)
    }
};
        locals.var_xg_ac = assign53080_e68450;

        let (assign53090_e68454, assign53090_e68454_d_n5, assign53090_e68454_d_n6, assign53090_e68454_d_n7, assign53090_e68454_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8,)
    }
};
        locals.var_xno_s_ac = assign53090_e68454;
        locals.var_xno_s_ac_dn5 = assign53090_e68454_d_n5;
        locals.var_xno_s_ac_dn6 = assign53090_e68454_d_n6;
        locals.var_xno_s_ac_dn7 = assign53090_e68454_d_n7;
        locals.var_xno_s_ac_dn8 = assign53090_e68454_d_n8;

        let (assign53100_e68458, assign53100_e68458_d_n5, assign53100_e68458_d_n6, assign53100_e68458_d_n7, assign53100_e68458_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8,)
    }
};
        locals.var_qbs_ac = assign53100_e68458;
        locals.var_qbs_ac_dn5 = assign53100_e68458_d_n5;
        locals.var_qbs_ac_dn6 = assign53100_e68458_d_n6;
        locals.var_qbs_ac_dn7 = assign53100_e68458_d_n7;
        locals.var_qbs_ac_dn8 = assign53100_e68458_d_n8;

        let (assign53110_e68462, assign53110_e68462_d_n5, assign53110_e68462_d_n6, assign53110_e68462_d_n7, assign53110_e68462_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8,)
    }
};
        locals.var_dps_ac = assign53110_e68462;
        locals.var_dps_ac_dn5 = assign53110_e68462_d_n5;
        locals.var_dps_ac_dn6 = assign53110_e68462_d_n6;
        locals.var_dps_ac_dn7 = assign53110_e68462_d_n7;
        locals.var_dps_ac_dn8 = assign53110_e68462_d_n8;

        let (assign53120_e68466, assign53120_e68466_d_n5, assign53120_e68466_d_n6, assign53120_e68466_d_n7, assign53120_e68466_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8,)
    }
};
        locals.var_qbd_ac = assign53120_e68466;
        locals.var_qbd_ac_dn5 = assign53120_e68466_d_n5;
        locals.var_qbd_ac_dn6 = assign53120_e68466_d_n6;
        locals.var_qbd_ac_dn7 = assign53120_e68466_d_n7;
        locals.var_qbd_ac_dn8 = assign53120_e68466_d_n8;

        let (assign53130_e68470, assign53130_e68470_d_n5, assign53130_e68470_d_n6, assign53130_e68470_d_n7, assign53130_e68470_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8,)
    }
};
        locals.var_eta_p_ac = assign53130_e68470;
        locals.var_eta_p_ac_dn5 = assign53130_e68470_d_n5;
        locals.var_eta_p_ac_dn6 = assign53130_e68470_d_n6;
        locals.var_eta_p_ac_dn7 = assign53130_e68470_d_n7;
        locals.var_eta_p_ac_dn8 = assign53130_e68470_d_n8;

        let (assign53140_e68474, assign53140_e68474_d_n5, assign53140_e68474_d_n6, assign53140_e68474_d_n7, assign53140_e68474_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8,)
    }
};
        locals.var_alpha_ac = assign53140_e68474;
        locals.var_alpha_ac_dn5 = assign53140_e68474_d_n5;
        locals.var_alpha_ac_dn6 = assign53140_e68474_d_n6;
        locals.var_alpha_ac_dn7 = assign53140_e68474_d_n7;
        locals.var_alpha_ac_dn8 = assign53140_e68474_d_n8;

        let (assign53150_e68478, assign53150_e68478_d_n5, assign53150_e68478_d_n6, assign53150_e68478_d_n7, assign53150_e68478_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8,)
    }
};
        locals.var_qim_ac = assign53150_e68478;
        locals.var_qim_ac_dn5 = assign53150_e68478_d_n5;
        locals.var_qim_ac_dn6 = assign53150_e68478_d_n6;
        locals.var_qim_ac_dn7 = assign53150_e68478_d_n7;
        locals.var_qim_ac_dn8 = assign53150_e68478_d_n8;

        let (assign53160_e68482, assign53160_e68482_d_n5, assign53160_e68482_d_n6, assign53160_e68482_d_n7, assign53160_e68482_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8,)
    }
};
        locals.var_qim1_ac = assign53160_e68482;
        locals.var_qim1_ac_dn5 = assign53160_e68482_d_n5;
        locals.var_qim1_ac_dn6 = assign53160_e68482_d_n6;
        locals.var_qim1_ac_dn7 = assign53160_e68482_d_n7;
        locals.var_qim1_ac_dn8 = assign53160_e68482_d_n8;

        let (assign53170_e68486, assign53170_e68486_d_n5, assign53170_e68486_d_n6, assign53170_e68486_d_n7, assign53170_e68486_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8,)
    }
};
        locals.var_qeff1_ac = assign53170_e68486;
        locals.var_qeff1_ac_dn5 = assign53170_e68486_d_n5;
        locals.var_qeff1_ac_dn6 = assign53170_e68486_d_n6;
        locals.var_qeff1_ac_dn7 = assign53170_e68486_d_n7;
        locals.var_qeff1_ac_dn8 = assign53170_e68486_d_n8;

        let (assign53180_e68490, assign53180_e68490_d_n5, assign53180_e68490_d_n6, assign53180_e68490_d_n7, assign53180_e68490_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8,)
    }
};
        locals.var_gmob_ac = assign53180_e68490;
        locals.var_gmob_ac_dn5 = assign53180_e68490_d_n5;
        locals.var_gmob_ac_dn6 = assign53180_e68490_d_n6;
        locals.var_gmob_ac_dn7 = assign53180_e68490_d_n7;
        locals.var_gmob_ac_dn8 = assign53180_e68490_d_n8;

        let (assign53190_e68494, assign53190_e68494_d_n5, assign53190_e68494_d_n6, assign53190_e68494_d_n7, assign53190_e68494_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8,)
    }
};
        locals.var_s1_ac = assign53190_e68494;
        locals.var_s1_ac_dn5 = assign53190_e68494_d_n5;
        locals.var_s1_ac_dn6 = assign53190_e68494_d_n6;
        locals.var_s1_ac_dn7 = assign53190_e68494_d_n7;
        locals.var_s1_ac_dn8 = assign53190_e68494_d_n8;

        let (assign53200_e68498, assign53200_e68498_d_n5, assign53200_e68498_d_n6, assign53200_e68498_d_n7, assign53200_e68498_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8,)
    }
};
        locals.var_thesateff_ac = assign53200_e68498;
        locals.var_thesateff_ac_dn5 = assign53200_e68498_d_n5;
        locals.var_thesateff_ac_dn6 = assign53200_e68498_d_n6;
        locals.var_thesateff_ac_dn7 = assign53200_e68498_d_n7;
        locals.var_thesateff_ac_dn8 = assign53200_e68498_d_n8;

        let (assign53210_e68502, assign53210_e68502_d_n5, assign53210_e68502_d_n6, assign53210_e68502_d_n7, assign53210_e68502_d_n8,) = {
    if (locals.var_guard1456 != 0.0) {
        (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8,)
    }
};
        locals.var_voxm_ac = assign53210_e68502;
        locals.var_voxm_ac_dn5 = assign53210_e68502_d_n5;
        locals.var_voxm_ac_dn6 = assign53210_e68502_d_n6;
        locals.var_voxm_ac_dn7 = assign53210_e68502_d_n7;
        locals.var_voxm_ac_dn8 = assign53210_e68502_d_n8;

        let (assign53220_e68507,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_phib_dc,)
    } else {
        (locals.var_phib_ac,)
    }
};
        locals.var_phib_ac = assign53220_e68507;

        let (assign53230_e68512, assign53230_e68512_d_n5, assign53230_e68512_d_n6, assign53230_e68512_d_n7, assign53230_e68512_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8,)
    } else {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    }
};
        locals.var_vgb1_ac = assign53230_e68512;
        locals.var_vgb1_ac_dn5 = assign53230_e68512_d_n5;
        locals.var_vgb1_ac_dn6 = assign53230_e68512_d_n6;
        locals.var_vgb1_ac_dn7 = assign53230_e68512_d_n7;
        locals.var_vgb1_ac_dn8 = assign53230_e68512_d_n8;

        let (assign53240_e68517, assign53240_e68517_d_n5, assign53240_e68517_d_n6, assign53240_e68517_d_n7, assign53240_e68517_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8,)
    } else {
        (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8,)
    }
};
        locals.var_phit1_ac = assign53240_e68517;
        locals.var_phit1_ac_dn5 = assign53240_e68517_d_n5;
        locals.var_phit1_ac_dn6 = assign53240_e68517_d_n6;
        locals.var_phit1_ac_dn7 = assign53240_e68517_d_n7;
        locals.var_phit1_ac_dn8 = assign53240_e68517_d_n8;

        let (assign53250_e68522, assign53250_e68522_d_n5, assign53250_e68522_d_n6, assign53250_e68522_d_n7, assign53250_e68522_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8,)
    } else {
        (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8,)
    }
};
        locals.var_gf_ac = assign53250_e68522;
        locals.var_gf_ac_dn5 = assign53250_e68522_d_n5;
        locals.var_gf_ac_dn6 = assign53250_e68522_d_n6;
        locals.var_gf_ac_dn7 = assign53250_e68522_d_n7;
        locals.var_gf_ac_dn8 = assign53250_e68522_d_n8;

        let (assign53260_e68527,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_xg_dc,)
    } else {
        (locals.var_xg_ac,)
    }
};
        locals.var_xg_ac = assign53260_e68527;

        let (assign53270_e68532, assign53270_e68532_d_n5, assign53270_e68532_d_n6, assign53270_e68532_d_n7, assign53270_e68532_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8,)
    } else {
        (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8,)
    }
};
        locals.var_xno_s_ac = assign53270_e68532;
        locals.var_xno_s_ac_dn5 = assign53270_e68532_d_n5;
        locals.var_xno_s_ac_dn6 = assign53270_e68532_d_n6;
        locals.var_xno_s_ac_dn7 = assign53270_e68532_d_n7;
        locals.var_xno_s_ac_dn8 = assign53270_e68532_d_n8;

        let (assign53280_e68537, assign53280_e68537_d_n5, assign53280_e68537_d_n6, assign53280_e68537_d_n7, assign53280_e68537_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8,)
    } else {
        (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8,)
    }
};
        locals.var_qbs_ac = assign53280_e68537;
        locals.var_qbs_ac_dn5 = assign53280_e68537_d_n5;
        locals.var_qbs_ac_dn6 = assign53280_e68537_d_n6;
        locals.var_qbs_ac_dn7 = assign53280_e68537_d_n7;
        locals.var_qbs_ac_dn8 = assign53280_e68537_d_n8;

        let (assign53290_e68542, assign53290_e68542_d_n5, assign53290_e68542_d_n6, assign53290_e68542_d_n7, assign53290_e68542_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_dps_dc, locals.var_dps_dc_dn5, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8,)
    } else {
        (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8,)
    }
};
        locals.var_dps_ac = assign53290_e68542;
        locals.var_dps_ac_dn5 = assign53290_e68542_d_n5;
        locals.var_dps_ac_dn6 = assign53290_e68542_d_n6;
        locals.var_dps_ac_dn7 = assign53290_e68542_d_n7;
        locals.var_dps_ac_dn8 = assign53290_e68542_d_n8;

        let (assign53300_e68547, assign53300_e68547_d_n5, assign53300_e68547_d_n6, assign53300_e68547_d_n7, assign53300_e68547_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qbd_dc, locals.var_qbd_dc_dn5, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8,)
    } else {
        (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8,)
    }
};
        locals.var_qbd_ac = assign53300_e68547;
        locals.var_qbd_ac_dn5 = assign53300_e68547_d_n5;
        locals.var_qbd_ac_dn6 = assign53300_e68547_d_n6;
        locals.var_qbd_ac_dn7 = assign53300_e68547_d_n7;
        locals.var_qbd_ac_dn8 = assign53300_e68547_d_n8;

        let (assign53310_e68552, assign53310_e68552_d_n5, assign53310_e68552_d_n6, assign53310_e68552_d_n7, assign53310_e68552_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_eta_p_dc, locals.var_eta_p_dc_dn5, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8,)
    } else {
        (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8,)
    }
};
        locals.var_eta_p_ac = assign53310_e68552;
        locals.var_eta_p_ac_dn5 = assign53310_e68552_d_n5;
        locals.var_eta_p_ac_dn6 = assign53310_e68552_d_n6;
        locals.var_eta_p_ac_dn7 = assign53310_e68552_d_n7;
        locals.var_eta_p_ac_dn8 = assign53310_e68552_d_n8;

        let (assign53320_e68557, assign53320_e68557_d_n5, assign53320_e68557_d_n6, assign53320_e68557_d_n7, assign53320_e68557_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_alpha_dc, locals.var_alpha_dc_dn5, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8,)
    } else {
        (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8,)
    }
};
        locals.var_alpha_ac = assign53320_e68557;
        locals.var_alpha_ac_dn5 = assign53320_e68557_d_n5;
        locals.var_alpha_ac_dn6 = assign53320_e68557_d_n6;
        locals.var_alpha_ac_dn7 = assign53320_e68557_d_n7;
        locals.var_alpha_ac_dn8 = assign53320_e68557_d_n8;

        let (assign53330_e68562, assign53330_e68562_d_n5, assign53330_e68562_d_n6, assign53330_e68562_d_n7, assign53330_e68562_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qim_dc, locals.var_qim_dc_dn5, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8,)
    }
};
        locals.var_qim_ac = assign53330_e68562;
        locals.var_qim_ac_dn5 = assign53330_e68562_d_n5;
        locals.var_qim_ac_dn6 = assign53330_e68562_d_n6;
        locals.var_qim_ac_dn7 = assign53330_e68562_d_n7;
        locals.var_qim_ac_dn8 = assign53330_e68562_d_n8;

        let (assign53340_e68567, assign53340_e68567_d_n5, assign53340_e68567_d_n6, assign53340_e68567_d_n7, assign53340_e68567_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qim1_dc, locals.var_qim1_dc_dn5, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8,)
    } else {
        (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8,)
    }
};
        locals.var_qim1_ac = assign53340_e68567;
        locals.var_qim1_ac_dn5 = assign53340_e68567_d_n5;
        locals.var_qim1_ac_dn6 = assign53340_e68567_d_n6;
        locals.var_qim1_ac_dn7 = assign53340_e68567_d_n7;
        locals.var_qim1_ac_dn8 = assign53340_e68567_d_n8;

        let (assign53350_e68572, assign53350_e68572_d_n5, assign53350_e68572_d_n6, assign53350_e68572_d_n7, assign53350_e68572_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_qeff1_dc, locals.var_qeff1_dc_dn5, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8,)
    } else {
        (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8,)
    }
};
        locals.var_qeff1_ac = assign53350_e68572;
        locals.var_qeff1_ac_dn5 = assign53350_e68572_d_n5;
        locals.var_qeff1_ac_dn6 = assign53350_e68572_d_n6;
        locals.var_qeff1_ac_dn7 = assign53350_e68572_d_n7;
        locals.var_qeff1_ac_dn8 = assign53350_e68572_d_n8;

        let (assign53360_e68577, assign53360_e68577_d_n5, assign53360_e68577_d_n6, assign53360_e68577_d_n7, assign53360_e68577_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_gmob_dc, locals.var_gmob_dc_dn5, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8,)
    } else {
        (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8,)
    }
};
        locals.var_gmob_ac = assign53360_e68577;
        locals.var_gmob_ac_dn5 = assign53360_e68577_d_n5;
        locals.var_gmob_ac_dn6 = assign53360_e68577_d_n6;
        locals.var_gmob_ac_dn7 = assign53360_e68577_d_n7;
        locals.var_gmob_ac_dn8 = assign53360_e68577_d_n8;

        let (assign53370_e68582, assign53370_e68582_d_n5, assign53370_e68582_d_n6, assign53370_e68582_d_n7, assign53370_e68582_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_s1_dc, locals.var_s1_dc_dn5, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8,)
    } else {
        (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8,)
    }
};
        locals.var_s1_ac = assign53370_e68582;
        locals.var_s1_ac_dn5 = assign53370_e68582_d_n5;
        locals.var_s1_ac_dn6 = assign53370_e68582_d_n6;
        locals.var_s1_ac_dn7 = assign53370_e68582_d_n7;
        locals.var_s1_ac_dn8 = assign53370_e68582_d_n8;

    }
}
